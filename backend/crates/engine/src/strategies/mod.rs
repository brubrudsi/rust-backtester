pub mod donchian;
pub mod ma_crossover;
pub mod pairs_zscore;
pub mod zscore_mr;

/// Single-asset simulation runner used by MA crossover, z-score MR, and Donchian breakout.
///
/// Hiring-signal focus:
/// - deterministic
/// - no lookahead (signals on close, execution next open)
/// - costs/slippage + carry
/// - emits indicator series for chart overlays
pub mod single_asset {
    use crate::metrics;
    use crate::types::*;
    use crate::EngineError;

    use super::{donchian, ma_crossover, zscore_mr};
    use std::collections::BTreeMap;

    #[derive(Clone, Debug)]
    struct Portfolio1 {
        cash: f64,
        qty: f64,
    }

    fn slip_fill(open: f64, delta_qty: f64, slippage_bps: f64) -> (f64, f64) {
        let slip = slippage_bps / 10000.0;
        if delta_qty > 0.0 {
            let fill = open * (1.0 + slip);
            let slippage_cost = (fill - open) * delta_qty;
            (fill, slippage_cost)
        } else {
            let sell_qty = -delta_qty;
            let fill = open * (1.0 - slip);
            let slippage_cost = (open - fill) * sell_qty;
            (fill, slippage_cost)
        }
    }

    pub fn run(symbol: &str, bars: &[Bar], cfg: &BacktestConfig) -> Result<BacktestResult, EngineError> {
        let closes: Vec<f64> = bars.iter().map(|b| b.c).collect();
        let highs: Vec<f64> = bars.iter().map(|b| b.h).collect();
        let lows: Vec<f64> = bars.iter().map(|b| b.l).collect();
        let timestamps: Vec<i64> = bars.iter().map(|b| b.t).collect();

        let diag = metrics::diagnostics(&timestamps);
        if diag.bars_per_year <= 0.0 {
            return Err(EngineError::InvalidInput("unable to infer bars_per_year".into()));
        }

        let (mut indicators, mut decision_fn): (BTreeMap<String, Vec<Option<f64>>>, Box<dyn FnMut(usize, f64, f64) -> Result<f64, EngineError>>) =
            match &cfg.strategy {
                StrategyConfig::MaCrossover(p) => {
                    let ind = ma_crossover::indicators(&closes, p);
                    let f = ma_crossover::decision_fn(bars, &closes, p, diag.bars_per_year);
                    (ind, f)
                }
                StrategyConfig::ZScoreMR(p) => {
                    let ind = zscore_mr::indicators(&closes, p);
                    let f = zscore_mr::decision_fn(bars, &closes, p);
                    (ind, f)
                }
                StrategyConfig::DonchianBreakout(p) => {
                    let ind = donchian::indicators(bars, &highs, &lows, p);
                    let f = donchian::decision_fn(bars, p);
                    (ind, f)
                }
                StrategyConfig::PairsZScore(_) => {
                    return Err(EngineError::InvalidInput(
                        "pairs strategy cannot run in single-asset runner".into(),
                    ));
                }
            };

        // Add a convenience indicator: close
        indicators.insert("close".into(), closes.iter().map(|x| Some(*x)).collect());

        let mut portfolio = Portfolio1 {
            cash: cfg.initial_capital,
            qty: 0.0,
        };

        let mut equity = vec![0.0; bars.len()];
        let mut net_exposure = vec![0.0; bars.len()];
        let mut gross_exposure = vec![0.0; bars.len()];
        let mut turnover = vec![0.0; bars.len()];
        let mut positions = vec![vec![0.0; bars.len()]];

        let mut trades: Vec<Trade> = vec![];
        let mut open_trade: Option<(u32, i32, usize, i64, f64, f64, f64, f64)> = None;
        // (id, direction_sign, entry_idx, entry_time, entry_price, entry_equity, fees_paid, slippage_cost)
        let mut trade_financing_pnl: f64 = 0.0;
        let mut next_trade_id: u32 = 1;

        for i in 0..bars.len() {
            let px = bars[i].c;
            let eq = portfolio.cash + portfolio.qty * px;
            equity[i] = eq;
            positions[0][i] = portfolio.qty;

            let notional = portfolio.qty * px;
            if eq.abs() > 1e-12 {
                net_exposure[i] = notional / eq;
                gross_exposure[i] = notional.abs() / eq;
            }

            if i + 1 >= bars.len() {
                break;
            }

            // Decision at close i -> target qty for next open.
            let target_qty = decision_fn(i, eq, portfolio.qty)?;

            // Carry accrual over dt (between bar start timestamps).
            let dt_years = (bars[i + 1].t - bars[i].t) as f64 / (1000.0 * 3600.0 * 24.0 * 365.25);

            if portfolio.qty < 0.0 && cfg.borrow_annual_pct.abs() > 1e-12 {
                let short_notional = (portfolio.qty * px).abs();
                let cost = short_notional * (cfg.borrow_annual_pct / 100.0) * dt_years;
                portfolio.cash -= cost;
                trade_financing_pnl -= cost;
            }
            if cfg.funding_annual_pct.abs() > 1e-12 {
                let cost = notional * (cfg.funding_annual_pct / 100.0) * dt_years;
                portfolio.cash -= cost;
                trade_financing_pnl -= cost;
            }

            // Execute at next open
            let next_open = bars[i + 1].o;
            let delta_qty = target_qty - portfolio.qty;
            if delta_qty.abs() > 1e-12 {
                let (fill, slip_cost) = slip_fill(next_open, delta_qty, cfg.slippage_bps);
                let traded_notional = (delta_qty * fill).abs();
                let fee = traded_notional * (cfg.fees_bps / 10000.0);

                let prev_qty = portfolio.qty;

                portfolio.cash -= delta_qty * fill;
                portfolio.cash -= fee;
                portfolio.qty = target_qty;

                if eq.abs() > 1e-12 {
                    turnover[i + 1] = traded_notional / eq;
                }

                let prev_sign: i32 = if prev_qty.abs() > 1e-12 { prev_qty.signum() as i32 } else { 0 };
                let new_sign: i32 = if portfolio.qty.abs() > 1e-12 { portfolio.qty.signum() as i32 } else { 0 };

                // Close trade if we went flat or flipped sign
                if let Some((tid, dir, entry_idx, entry_time, entry_price, entry_eq, mut fees_paid, mut slippage_cost)) = open_trade.take() {
                    if new_sign == 0 || new_sign != dir {
                        fees_paid += fee;
                        slippage_cost += slip_cost;

                        let exit_time = bars[i + 1].t;
                        let exit_price = fill;

                        let entry_qty = dir as f64 * (entry_eq / entry_price).abs();
                        let leg = TradeLeg {
                            symbol: symbol.to_string(),
                            qty: entry_qty,
                            entry_price,
                            exit_price,
                            fees_paid,
                            slippage_cost,
                        };

                        let signed_qty = dir as f64 * (entry_eq / entry_price);
                        let price_pnl = signed_qty * (exit_price - entry_price);
                        let pnl = price_pnl - fees_paid - slippage_cost + trade_financing_pnl;
                        let pnl_pct = if entry_eq.abs() > 1e-12 { pnl / entry_eq } else { 0.0 };

                        let holding = (i + 1).saturating_sub(entry_idx) as u32;

                        trades.push(Trade {
                            id: tid,
                            direction: if dir > 0 { "long".into() } else { "short".into() },
                            entry_time,
                            exit_time,
                            holding_period_bars: holding,
                            legs: vec![leg],
                            pnl,
                            pnl_pct: pnl_pct * 100.0,
                            financing_pnl: trade_financing_pnl,
                        });

                        trade_financing_pnl = 0.0;
                    } else {
                        open_trade = Some((tid, dir, entry_idx, entry_time, entry_price, entry_eq, fees_paid, slippage_cost));
                    }
                }

                // Open new trade if moved from flat to non-flat or flipped direction
                if open_trade.is_none() && new_sign != 0 {
                    open_trade = Some((
                        next_trade_id,
                        new_sign,
                        i + 1,
                        bars[i + 1].t,
                        fill,
                        eq,
                        fee,
                        slip_cost,
                    ));
                    next_trade_id += 1;
                } else if open_trade.is_none() && prev_sign != 0 && new_sign != 0 && new_sign != prev_sign {
                    open_trade = Some((
                        next_trade_id,
                        new_sign,
                        i + 1,
                        bars[i + 1].t,
                        fill,
                        eq,
                        fee,
                        slip_cost,
                    ));
                    next_trade_id += 1;
                }
            }
        }

        let returns = metrics::compute_returns(&equity);
        let drawdown = metrics::compute_drawdown(&equity);

        let window = (diag.bars_per_year / 4.0).round().max(20.0) as usize;
        let rolling_sharpe = metrics::rolling_sharpe(&returns, window, diag.bars_per_year);

        let trades_n = trades.len() as u32;
        let mut wins = 0u32;
        let mut gross_profit = 0.0;
        let mut gross_loss = 0.0;
        let mut avg_hold = 0.0;
        for t in &trades {
            if t.pnl > 0.0 {
                wins += 1;
                gross_profit += t.pnl;
            } else {
                gross_loss += t.pnl;
            }
            avg_hold += t.holding_period_bars as f64;
        }
        if trades_n > 0 {
            avg_hold /= trades_n as f64;
        }

        let years = if timestamps.len() >= 2 {
            (timestamps[timestamps.len() - 1] - timestamps[0]) as f64 / (1000.0 * 3600.0 * 24.0 * 365.25)
        } else {
            0.0
        };

        let turnover_avg = if turnover.len() > 1 {
            turnover.iter().sum::<f64>() / turnover.len() as f64
        } else {
            0.0
        };

        let metrics_out = metrics::summarize_metrics(
            equity.first().copied().unwrap_or(cfg.initial_capital),
            equity.last().copied().unwrap_or(cfg.initial_capital),
            &returns,
            &drawdown,
            diag.bars_per_year,
            years,
            trades_n,
            wins,
            gross_profit,
            gross_loss,
            avg_hold,
            turnover_avg,
        );

        Ok(BacktestResult {
            timestamps,
            equity,
            drawdown,
            returns,
            rolling_sharpe,
            net_exposure,
            gross_exposure,
            turnover,
            positions,
            indicators,
            trades,
            metrics: metrics_out,
            diagnostics: diag,
        })
    }
}
