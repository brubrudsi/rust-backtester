use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniverseFile {
    pub version: u32,
    pub generated_at: String,
    pub intraday: IntradayPref,
    pub items: Vec<UniverseItem>,
    pub pairs: Vec<PairItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntradayPref {
    pub preferred: String,
    pub fallback: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    Equity,
    Etf,
    Crypto,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniverseItem {
    pub id: String,
    pub display: String,
    pub name: String,
    pub asset_class: AssetClass,
    pub polygon: String,
    pub supports_adjusted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PairItem {
    pub id: String,
    pub a: String,
    pub b: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrategyId {
    MaCrossover,
    ZscoreMr,
    DonchianBreakout,
    PairsZscore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CostsRequest {
    pub fees_bps: f64,
    pub slippage_bps: f64,
    pub borrow_annual_pct: f64,
    pub funding_annual_pct: f64,
}

impl Default for CostsRequest {
    fn default() -> Self {
        Self {
            fees_bps: 1.0,
            slippage_bps: 1.0,
            borrow_annual_pct: 0.0,
            funding_annual_pct: 0.0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestRequest {
    pub strategy_id: StrategyId,
    pub symbols: Vec<String>,
    pub timeframe: String,
    pub start: String,
    pub end: String,
    pub adjusted: Option<bool>,
    pub initial_capital: Option<f64>,
    pub params: serde_json::Value,
    #[serde(default)]
    pub costs: CostsRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BacktestStatus {
    Queued,
    Running,
    Complete,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestStatusFile {
    pub id: Uuid,
    pub status: BacktestStatus,
    pub created_at: String,
    pub updated_at: String,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestSummary {
    pub end_equity: f64,
    pub total_return_pct: f64,
    pub sharpe: f64,
    pub max_drawdown_pct: f64,
    pub trades: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestStatusResponse {
    pub id: Uuid,
    pub status: BacktestStatus,
    pub message: Option<String>,
    pub summary: Option<BacktestSummary>,
    pub links: BacktestLinks,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestLinks {
    pub self_url: String,
    pub results_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateBacktestResponse {
    pub id: Uuid,
    pub status_url: String,
    pub results_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Assumptions {
    pub execution: String,
    pub fees: String,
    pub slippage: String,
    pub borrow: String,
    pub funding: String,
    pub adjusted_prices: String,
    pub missing_data: String,
    pub timezone: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SymbolBars {
    pub symbol: String,
    pub polygon: String,
    pub bars: Vec<engine::types::Bar>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestMeta {
    pub id: Uuid,
    pub strategy_id: StrategyId,
    pub symbols: Vec<String>,
    pub timeframe_requested: String,
    pub timeframe_used: String,
    pub start: String,
    pub end: String,
    pub adjusted: bool,
    pub data_source: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BacktestResults {
    pub meta: BacktestMeta,
    pub config: BacktestRequest,
    pub assumptions: Assumptions,
    pub bars: Vec<SymbolBars>,
    pub engine: engine::types::BacktestResult,
}
