use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Bar {
    /// Unix ms timestamp for the bar start (as provided by aggregates endpoints).
    pub t: i64,
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
    pub v: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestConfig {
    pub initial_capital: f64,

    /// Commission/fees in basis points (bps) per side.
    pub fees_bps: f64,

    /// Slippage in basis points (bps) per side.
    pub slippage_bps: f64,

    /// Annualized borrow cost (%) applied to short notional.
    pub borrow_annual_pct: f64,

    /// Annualized funding rate (%) applied to position notional.
    /// Convention: funding pnl per dt = - position_notional * rate * dt_years.
    pub funding_annual_pct: f64,

    pub strategy: StrategyConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StrategyConfig {
    /// A) Trend: Moving Average Crossover
    MaCrossover(MaCrossoverParams),
    /// B) Mean Reversion: Z-score / Bollinger-like
    ZScoreMR(ZScoreMRParams),
    /// C) Breakout: Donchian channel breakout
    DonchianBreakout(DonchianParams),
    /// D) Pairs: Spread z-score
    PairsZScore(PairsParams),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SizingMode {
    FixedNotional,
    VolTarget,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaCrossoverParams {
    pub fast: usize,
    pub slow: usize,
    pub sizing_mode: SizingMode,

    /// Only used for vol-target sizing.
    pub vol_lookback: usize,
    /// Annual vol target (e.g., 0.20 = 20%).
    pub vol_target: f64,
    pub max_leverage: f64,

    /// Optional stop-loss (% of entry price). Evaluated on close, executed next open.
    pub stop_loss_pct: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ZScoreMRParams {
    pub lookback: usize,
    pub entry_z: f64,
    pub exit_z: f64,
    pub stop_loss_pct: Option<f64>,
    pub time_stop_bars: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DonchianParams {
    pub lookback: usize,
    pub atr_period: usize,
    pub atr_stop_mult: Option<f64>,
    pub trailing_stop: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HedgeMethod {
    Ratio,
    Ols,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PairsParams {
    pub lookback: usize,
    pub entry_z: f64,
    pub exit_z: f64,
    pub hedge_method: HedgeMethod,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeLeg {
    pub symbol: String,
    pub qty: f64,
    pub entry_price: f64,
    pub exit_price: f64,
    pub fees_paid: f64,
    pub slippage_cost: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub id: u32,
    pub direction: String,
    pub entry_time: i64,
    pub exit_time: i64,
    pub holding_period_bars: u32,
    pub legs: Vec<TradeLeg>,
    pub pnl: f64,
    pub pnl_pct: f64,
    pub financing_pnl: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub start_equity: f64,
    pub end_equity: f64,
    pub total_return_pct: f64,
    pub cagr_pct: f64,
    pub vol_annual_pct: f64,
    pub sharpe: f64,
    pub max_drawdown_pct: f64,

    pub trades: u32,
    pub win_rate_pct: f64,
    pub profit_factor: f64,
    pub avg_hold_bars: f64,
    pub turnover_avg: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Diagnostics {
    pub bars_per_year: f64,
    pub median_dt_seconds: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestResult {
    pub timestamps: Vec<i64>,
    pub equity: Vec<f64>,
    pub drawdown: Vec<f64>,
    pub returns: Vec<f64>,
    pub rolling_sharpe: Vec<Option<f64>>,
    pub net_exposure: Vec<f64>,
    pub gross_exposure: Vec<f64>,
    pub turnover: Vec<f64>,
    pub positions: Vec<Vec<f64>>,
    pub indicators: BTreeMap<String, Vec<Option<f64>>>,
    pub trades: Vec<Trade>,
    pub metrics: Metrics,
    pub diagnostics: Diagnostics,
}
