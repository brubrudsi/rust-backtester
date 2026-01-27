use crate::indicators;
use crate::metrics;
use crate::types::*;
use crate::EngineError;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Portfolio2 {
    cash: f64,
    qty_a: f64,
    qty_b: f64,
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

pub fn run(
    symbols: &[String],
    bars_by_symbol: &[Vec<Bar>],
    cfg: &BacktestConfig,
) -> Result<BacktestResult, EngineError> {
    let a = &bars_by_symbol[0];
    let b = &bars_by_symbol[1];

    if a.len() != b.len() {
        return Err(EngineError::InvalidInput(
            "pairs bars must be aligned and same length".into(),
        ));
    }
    for i in 0..a.len() {
        if a[i].t != b[i].t {
            return Err(EngineError::InvalidInput(
                "pairs bars must have identical timestamps".into(),
            ));
        }
    }

    let ts: Vec<i64> = a.iter().map(|x| x.t).collect();
    let diag = metrics::diagnostics(&ts);
    if diag.bars_per_year <= 0.0 {
        return Err(EngineError::InvalidInput("unable to infer bars_per_year".into()));
    }

    let closes_a: Vec<f64> = a.iter().map(|x| x.c).collect();
    let closes_b: Vec<f64> = b.iter().map(|x| x.c).collect();

    let p = match &cfg.strategy {
        StrategyConfig::PairsZScore(pp) => pp.clone(),
        _ => {
            return Err(EngineError::InvalidInput(
                "pairs runner requires pairs_zscore strategy".into(),
            ))
        }
    };

    if p.lookback < 10 || p.entry_z <= 0.0 || p.exit_z < 0.0 {
        return Err(EngineError::InvalidInput("pairs_zscore invalid params".into()));
    }

    let hedge: Vec<Option<f64>> = match p.hedge_method {
        HedgeMethod::Ratio => indicators::rolling_ratio(&closes_a, &closes_b, p.lookback),
        HedgeMethod::Ols => indicators::rolling_beta_ols(&closes_a, &closes_b, p.lookback),
    };

    let mut spread = vec![None; closes_a.len()];
    for i in 0..closes_a.len() {
        if let Some(beta) = hedge[i] {
            spread[i] = Some(closes_a[i] - beta * closes_b[i]);
        }
    }

    let spread_vals: Vec<f64> = spread.iter().map(|x| x.unwrap_or(0.0)).collect();
    let (m, s) = indicators::rolling_mean_std(&spread_vals, p.lookback);

    let mut z = vec![None; spread_vals.len()];
    for i in 0..spread_vals.len() {
        if let (Some(mm), Some(ss)) = (m[i], s[i]) {
            if ss > 1e-12 {
                z[i] = Some((spread_vals[i] - mm) / ss);
            }
        }
    }

    let mut indicators_map = BTreeMap::new();
    indicators_map.insert("hedge_ratio".into(), hedge.clone());
    indicators_map.insert("spread".into(), spread.clone());
    indicators_map.insert("spread_mean".into(), m.clone());
    indicators_map.insert("spread_std".into(), s.clone());
    indicators_map.insert("zscore".into(), z.clone());
    indicators_map.insert("close_a".into(), closes_a.iter().map(|x| Some(*x)).collect());
    indicators_map.insert("close_b".into(), closes_b.iter().map(|x| Some(*x)).collect());

    let n = a.len();

    let mut portfolio = Portfolio2 {
        cash: cfg.initial_capital,
        qty_a: 0.0,
        qty_b: 0.0,
    };

    let mut equity = vec![0.0; n];
    let mut net_exposure = vec![0.0; n];
    let mut gross_exposure = vec![0.0; n];
    let mut turnover = vec![0.0; n];
    let mut positions = vec![vec![0.0; n], vec![0.0; n]];

    let mut trades: Vec<Trade> = vec![];
    let mut open_trade: Option<(u32, i32, usize, i64, f64, f64, f64, f64, f64)> = None;
    // (id, side, entry_idx, entry_time, entry_eq, entry_px_a, entry_px_b, fees, slip)
    let mut next_trade_id: u32 = 1;
    let mut financing_pnl: f64 = 0.0;

    let mut side: i32 = 0;

    for i in 0..n {
        let px_a = a[i].c;
        let px_b = b[i].c;
        let eq = portfolio.cash + portfolio.qty_a * px_a + portfolio.qty_b * px_b;
        equity[i] = eq;

        let not_a = portfolio.qty_a * px_a;
        let not_b = portfolio.qty_b * px_b;
        let net = not_a + not_b;
        let gross = not_a.abs() + not_b.abs();

        if eq.abs() > 1e-12 {
            net_exposure[i] = net / eq;
            gross_exposure[i] = gross / eq;
        }

        positions[0][i] = portfolio.qty_a;
        positions[1][i] = portfolio.qty_b;

        if i + 1 >= n {
            break;
        }

        let zi = z[i];
        let beta = hedge[i];

        if zi.is_none() || beta.is_none() {
            side = 0;
        } else {
            let zi = zi.unwrap();
            if side == 0 {
                if zi >= p.entry_z {
                    side = -1;
                } else if zi <= -p.entry_z {
                    side = 1;
                }
            } else if side == 1 {
                if zi >= p.exit_z {
                    side = 0;
                }
            } else if side == -1 {
                if zi <= -p.exit_z {
                    side = 0;
                }
            }
        }

        let dt_years = (a[i + 1].t - a[i].t) as f64 / (1000.0 * 3600.0 * 24.0 * 365.25);
        if cfg.borrow_annual_pct.abs() > 1e-12 {
            let short_notional = not_a.min(0.0).abs() + not_b.min(0.0).abs();
            let cost = short_notional * (cfg.borrow_annual_pct / 100.0) * dt_years;
            portfolio.cash -= cost;
            financing_pnl -= cost;
        }
        if cfg.funding_annual_pct.abs() > 1e-12 {
            let cost = net * (cfg.funding_annual_pct / 100.0) * dt_years;
            portfolio.cash -= cost;
            financing_pnl -= cost;
        }

        let open_a = a[i + 1].o;
        let open_b = b[i + 1].o;

        let beta_exec = beta.unwrap_or(0.0);
        let base_a = side as f64;
        let base_b = -(side as f64) * beta_exec;

        let gross_per_unit = (base_a.abs() * open_a) + (base_b.abs() * open_b);
        let scale = if gross_per_unit > 1e-12 && eq > 0.0 {
            eq / gross_per_unit
        } else {
            0.0
        };

        let target_qty_a = base_a * scale;
        let target_qty_b = base_b * scale;

        let delta_a = target_qty_a - portfolio.qty_a;
        let delta_b = target_qty_b - portfolio.qty_b;

        let mut traded_notional = 0.0;
        let mut fee_paid = 0.0;
        let mut slip_cost = 0.0;

        if delta_a.abs() > 1e-12 {
            let (fill, sc) = slip_fill(open_a, delta_a, cfg.slippage_bps);
            traded_notional += (delta_a * fill).abs();
            let fee = (delta_a * fill).abs() * (cfg.fees_bps / 10000.0);
            portfolio.cash -= delta_a * fill;
            portfolio.cash -= fee;
            fee_paid += fee;
            slip_cost += sc;
            portfolio.qty_a = target_qty_a;
        }
        if delta_b.abs() > 1e-12 {
            let (fill, sc) = slip_fill(open_b, delta_b, cfg.slippage_bps);
            traded_notional += (delta_b * fill).abs();
            let fee = (delta_b * fill).abs() * (cfg.fees_bps / 10000.0);
            portfolio.cash -= delta_b * fill;
            portfolio.cash -= fee;
            fee_paid += fee;
            slip_cost += sc;
            portfolio.qty_b = target_qty_b;
        }

        if traded_notional > 0.0 && eq.abs() > 1e-12 {
            turnover[i + 1] = traded_notional / eq;
        }

        // Close trade if now flat
        if let Some((tid, open_side, entry_idx, entry_time, entry_eq, entry_px_a, entry_px_b, mut fees, mut slip)) =
            open_trade.take()
        {
            if side == 0 || side != open_side {
                fees += fee_paid;
                slip += slip_cost;
                let exit_time = a[i + 1].t;

                let exit_px_a = open_a;
                let exit_px_b = open_b;

                // Readable tear: use entry_eq scaling
                let denom = (entry_px_a.abs() + (beta_exec.abs() * entry_px_b.abs())).max(1e-12);
                let q_a = (open_side as f64) * (entry_eq / denom);
                let q_b = -(open_side as f64) * beta_exec * (entry_eq / denom);

                let price_pnl = q_a * (exit_px_a - entry_px_a) + q_b * (exit_px_b - entry_px_b);
                let pnl = price_pnl - fees - slip + financing_pnl;
                let pnl_pct = if entry_eq.abs() > 1e-12 { pnl / entry_eq } else { 0.0 };
                let hold = (i + 1).saturating_sub(entry_idx) as u32;

                let legs = vec![
                    TradeLeg {
                        symbol: symbols[0].clone(),
                        qty: q_a,
                        entry_price: entry_px_a,
                        exit_price: exit_px_a,
                        fees_paid: fees * 0.5,
                        slippage_cost: slip * 0.5,
                    },
                    TradeLeg {
                        symbol: symbols[1].clone(),
                        qty: q_b,
                        entry_price: entry_px_b,
                        exit_price: exit_px_b,
                        fees_paid: fees * 0.5,
                        slippage_cost: slip * 0.5,
                    },
                ];

                trades.push(Trade {
                    id: tid,
                    direction: if open_side > 0 { "long_spread".into() } else { "short_spread".into() },
                    entry_time,
                    exit_time,
                    holding_period_bars: hold,
                    legs,
                    pnl,
                    pnl_pct: pnl_pct * 100.0,
                    financing_pnl,
                });

                financing_pnl = 0.0;
            } else {
                open_trade = Some((tid, open_side, entry_idx, entry_time, entry_eq, entry_px_a, entry_px_b, fees, slip));
            }
        }

        if open_trade.is_none() && side != 0 {
            open_trade = Some((
                next_trade_id,
                side,
                i + 1,
                a[i + 1].t,
                eq,
                open_a,
                open_b,
                fee_paid,
                slip_cost,
            ));
            next_trade_id += 1;
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

    let years = if ts.len() >= 2 {
        (ts[ts.len() - 1] - ts[0]) as f64 / (1000.0 * 3600.0 * 24.0 * 365.25)
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
        timestamps: ts,
        equity,
        drawdown,
        returns,
        rolling_sharpe,
        net_exposure,
        gross_exposure,
        turnover,
        positions,
        indicators: indicators_map,
        trades,
        metrics: metrics_out,
        diagnostics: diag,
    })
}
