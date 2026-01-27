export type AssetClass = "equity" | "etf" | "crypto";

export type UniverseItem = {
  id: string;
  display: string;
  name: string;
  asset_class: AssetClass;
  polygon: string;
  supports_adjusted: boolean;
};

export type PairItem = {
  id: string;
  a: string;
  b: string;
  description: string;
};

export type UniverseFile = {
  version: number;
  generated_at: string;
  intraday: { preferred: string; fallback: string };
  items: UniverseItem[];
  pairs: PairItem[];
};

export type StrategyId = "ma_crossover" | "zscore_mr" | "donchian_breakout" | "pairs_zscore";

export type CostsRequest = {
  fees_bps: number;
  slippage_bps: number;
  borrow_annual_pct: number;
  funding_annual_pct: number;
};

export type BacktestRequest = {
  strategy_id: StrategyId;
  symbols: string[];
  timeframe: string;
  start: string;
  end: string;
  adjusted?: boolean;
  initial_capital?: number;
  params: any;
  costs?: CostsRequest;
};

export type BacktestStatus = "queued" | "running" | "complete" | "failed";

export type BacktestSummary = {
  end_equity: number;
  total_return_pct: number;
  sharpe: number;
  max_drawdown_pct: number;
  trades: number;
};

export type BacktestStatusResponse = {
  id: string;
  status: BacktestStatus;
  message?: string | null;
  summary?: BacktestSummary | null;
  links: { self_url: string; results_url: string };
};

export type CreateBacktestResponse = {
  id: string;
  status_url: string;
  results_url: string;
};

export type Bar = {
  t: number;
  o: number;
  h: number;
  l: number;
  c: number;
  v: number;
};

export type TradeLeg = {
  symbol: string;
  qty: number;
  entry_price: number;
  exit_price: number;
  fees_paid: number;
  slippage_cost: number;
};

export type Trade = {
  id: number;
  direction: string;
  entry_time: number;
  exit_time: number;
  holding_period_bars: number;
  legs: TradeLeg[];
  pnl: number;
  pnl_pct: number;
  financing_pnl: number;
};

export type Metrics = {
  start_equity: number;
  end_equity: number;
  total_return_pct: number;
  cagr_pct: number;
  vol_annual_pct: number;
  sharpe: number;
  max_drawdown_pct: number;
  trades: number;
  win_rate_pct: number;
  profit_factor: number;
  avg_hold_bars: number;
  turnover_avg: number;
};

export type Diagnostics = {
  bars_per_year: number;
  median_dt_seconds: number;
};

export type EngineResult = {
  timestamps: number[];
  equity: number[];
  drawdown: number[];
  returns: number[];
  rolling_sharpe: Array<number | null>;
  net_exposure: number[];
  gross_exposure: number[];
  turnover: number[];
  positions: number[][];
  indicators: Record<string, Array<number | null>>;
  trades: Trade[];
  metrics: Metrics;
  diagnostics: Diagnostics;
};

export type Assumptions = {
  execution: string;
  fees: string;
  slippage: string;
  borrow: string;
  funding: string;
  adjusted_prices: string;
  missing_data: string;
  timezone: string;
};

export type SymbolBars = {
  symbol: string;
  polygon: string;
  bars: Bar[];
};

export type BacktestMeta = {
  id: string;
  strategy_id: StrategyId;
  symbols: string[];
  timeframe_requested: string;
  timeframe_used: string;
  start: string;
  end: string;
  adjusted: boolean;
  data_source: string;
};

export type BacktestResults = {
  meta: BacktestMeta;
  config: BacktestRequest;
  assumptions: Assumptions;
  bars: SymbolBars[];
  engine: EngineResult;
};
