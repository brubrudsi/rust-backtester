use axum::{extract::{Path, State}, Json};
use chrono::Utc;
use uuid::Uuid;

use crate::app::AppState;
use crate::errors::{ApiError, ApiResult};
use crate::market_data::Timeframe;
use crate::types::*;

fn parse_timeframe(tf: &str) -> ApiResult<Timeframe> {
    match tf {
        "1D" => Ok(Timeframe { multiplier: 1, timespan: "day".into(), label: "1D".into() }),
        "1H" => Ok(Timeframe { multiplier: 1, timespan: "hour".into(), label: "1H".into() }),
        "1m" => Ok(Timeframe { multiplier: 1, timespan: "minute".into(), label: "1m".into() }),
        "5m" => Ok(Timeframe { multiplier: 5, timespan: "minute".into(), label: "5m".into() }),
        _ => Err(ApiError::bad_request("timeframe must be one of: 1D, 1H, 1m, 5m")),
    }
}

fn validate_dates(start: &str, end: &str) -> ApiResult<()> {
    let s = chrono::NaiveDate::parse_from_str(start, "%Y-%m-%d").map_err(|_| ApiError::bad_request("start must be YYYY-MM-DD"))?;
    let e = chrono::NaiveDate::parse_from_str(end, "%Y-%m-%d").map_err(|_| ApiError::bad_request("end must be YYYY-MM-DD"))?;
    if s >= e {
        return Err(ApiError::bad_request("start must be < end"));
    }
    let today = Utc::now().date_naive();
    if e > today {
        return Err(ApiError::bad_request("end must not be in the future"));
    }
    Ok(())
}

fn default_assumptions(adjusted: bool, costs: &CostsRequest) -> Assumptions {
    Assumptions {
        execution: "Signals computed on bar close; orders execute at next bar open (no lookahead).".into(),
        fees: format!("Commission/fees: {:.2} bps per side.", costs.fees_bps),
        slippage: format!("Slippage: {:.2} bps per side applied to next-open fills.", costs.slippage_bps),
        borrow: format!("Short borrow: {:.2}% annualized applied to short notional over dt.", costs.borrow_annual_pct),
        funding: format!("Funding: {:.2}% annualized; pnl per dt = -notional * rate * dt_years.", costs.funding_annual_pct),
        adjusted_prices: if adjusted {
            "Adjusted prices ON (splits/dividends where supported).".into()
        } else {
            "Adjusted prices OFF.".into()
        },
        missing_data: "No forward-fill. Gaps remain gaps; carry accrues over dt. Pairs use strict timestamp intersection only.".into(),
        timezone: "Timestamps served as Unix ms (UTC). Note: equity aggregates are session-aligned; crypto is 24/7.".into(),
    }
}

pub async fn create_backtest(State(state): State<AppState>, Json(mut req): Json<BacktestRequest>) -> ApiResult<Json<CreateBacktestResponse>> {
    if state.polygon_key_present == false {
        return Err(ApiError::internal("POLYGON_API_KEY is not set on the server"));
    }

    validate_dates(&req.start, &req.end)?;
    let tf = parse_timeframe(&req.timeframe)?;

    let need = match req.strategy_id {
        StrategyId::PairsZscore => 2,
        _ => 1,
    };
    if req.symbols.len() != need {
        return Err(ApiError::bad_request(format!("strategy requires {} symbol(s)", need)));
    }

    let mut resolved: Vec<UniverseItem> = vec![];
    for sid in &req.symbols {
        let item = state.universe.items.iter().find(|x| &x.id == sid)
            .ok_or_else(|| ApiError::bad_request(format!("unknown symbol id: {}", sid)))?;
        resolved.push(item.clone());
    }

    let mut adjusted = req.adjusted.unwrap_or(true);
    for item in &resolved {
        if !item.supports_adjusted {
            adjusted = false;
        }
    }
    req.adjusted = Some(adjusted);

    if req.initial_capital.is_none() {
        req.initial_capital = Some(10_000.0);
    }
    if req.initial_capital.unwrap_or(0.0) <= 0.0 {
        return Err(ApiError::bad_request("initial_capital must be > 0"));
    }

    if req.costs.fees_bps < 0.0 || req.costs.slippage_bps < 0.0 {
        return Err(ApiError::bad_request("fees_bps and slippage_bps must be >= 0"));
    }

    let id = state.run_store.create_run(&req).await.map_err(|e| ApiError::internal(e.to_string()))?;
    state.run_store.set_status(id, BacktestStatus::Queued, None).await.map_err(|e| ApiError::internal(e.to_string()))?;

    let job_state = state.clone();
    tokio::spawn(async move {
        if let Err(e) = run_job(job_state, id, req, resolved, tf, adjusted).await {
            tracing::error!(%id, "job failed: {}", e);
        }
    });

    Ok(Json(CreateBacktestResponse {
        id,
        status_url: format!("/api/backtests/{}", id),
        results_url: format!("/api/backtests/{}/results", id),
    }))
}

async fn run_job(
    state: AppState,
    id: Uuid,
    req: BacktestRequest,
    resolved: Vec<UniverseItem>,
    tf: Timeframe,
    adjusted: bool,
) -> anyhow::Result<()> {
    state.run_store.set_status(id, BacktestStatus::Running, None).await?;

    let mut bars_out: Vec<SymbolBars> = vec![];
    let mut bars_by_symbol: Vec<Vec<engine::types::Bar>> = vec![];

    for item in &resolved {
        let bars = state.data_provider.get_bars(&item.polygon, &tf, &req.start, &req.end, adjusted).await?;
        if bars.len() < 10 {
            anyhow::bail!("not enough bars returned for {}", item.id);
        }
        bars_out.push(SymbolBars {
            symbol: item.id.clone(),
            polygon: item.polygon.clone(),
            bars: bars.clone(),
        });
        bars_by_symbol.push(bars);
    }

    if matches!(req.strategy_id, StrategyId::PairsZscore) {
        let (a, b) = align_two(&bars_by_symbol[0], &bars_by_symbol[1]);
        bars_by_symbol = vec![a, b];
        bars_out[0].bars = bars_by_symbol[0].clone();
        bars_out[1].bars = bars_by_symbol[1].clone();
    }

    let engine_cfg = build_engine_config(&req)?;
    let symbols_engine: Vec<String> = resolved.iter().map(|x| x.id.clone()).collect();

    let engine_res = engine::run_backtest(symbols_engine.clone(), bars_by_symbol, engine_cfg)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let assumptions = default_assumptions(adjusted, &req.costs);

    let meta = BacktestMeta {
        id,
        strategy_id: req.strategy_id.clone(),
        symbols: symbols_engine,
        timeframe_requested: req.timeframe.clone(),
        timeframe_used: tf.label.clone(),
        start: req.start.clone(),
        end: req.end.clone(),
        adjusted,
        data_source: "Polygon/Massive Aggregates".into(),
    };

    let results = BacktestResults {
        meta,
        config: req,
        assumptions,
        bars: bars_out,
        engine: engine_res,
    };

    state.run_store.write_results(id, &results).await?;
    state.run_store.set_status(id, BacktestStatus::Complete, None).await?;
    Ok(())
}

fn align_two(a: &[engine::types::Bar], b: &[engine::types::Bar]) -> (Vec<engine::types::Bar>, Vec<engine::types::Bar>) {
    let mut i = 0usize;
    let mut j = 0usize;
    let mut out_a = vec![];
    let mut out_b = vec![];

    while i < a.len() && j < b.len() {
        if a[i].t == b[j].t {
            out_a.push(a[i]);
            out_b.push(b[j]);
            i += 1;
            j += 1;
        } else if a[i].t < b[j].t {
            i += 1;
        } else {
            j += 1;
        }
    }
    (out_a, out_b)
}

fn build_engine_config(req: &BacktestRequest) -> ApiResult<engine::types::BacktestConfig> {
    use engine::types::*;

    let strategy = match req.strategy_id {
        StrategyId::MaCrossover => {
            let p: MaCrossoverParams = serde_json::from_value(req.params.clone())
                .map_err(|_| ApiError::bad_request("invalid params for ma_crossover"))?;
            StrategyConfig::MaCrossover(p)
        }
        StrategyId::ZscoreMr => {
            let p: ZScoreMRParams = serde_json::from_value(req.params.clone())
                .map_err(|_| ApiError::bad_request("invalid params for zscore_mr"))?;
            StrategyConfig::ZScoreMR(p)
        }
        StrategyId::DonchianBreakout => {
            let p: DonchianParams = serde_json::from_value(req.params.clone())
                .map_err(|_| ApiError::bad_request("invalid params for donchian_breakout"))?;
            StrategyConfig::DonchianBreakout(p)
        }
        StrategyId::PairsZscore => {
            let p: PairsParams = serde_json::from_value(req.params.clone())
                .map_err(|_| ApiError::bad_request("invalid params for pairs_zscore"))?;
            StrategyConfig::PairsZScore(p)
        }
    };

    Ok(engine::types::BacktestConfig {
        initial_capital: req.initial_capital.unwrap_or(10_000.0),
        fees_bps: req.costs.fees_bps,
        slippage_bps: req.costs.slippage_bps,
        borrow_annual_pct: req.costs.borrow_annual_pct,
        funding_annual_pct: req.costs.funding_annual_pct,
        strategy,
    })
}

pub async fn get_backtest(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<BacktestStatusResponse>> {
    let status = state.run_store.read_status(id).await.map_err(|_| ApiError::not_found("unknown backtest id"))?;
    let summary = state.run_store.read_summary(id).await.map_err(|_| ApiError::internal("failed reading summary"))?;

    Ok(Json(BacktestStatusResponse {
        id,
        status: status.status,
        message: status.message,
        summary,
        links: BacktestLinks {
            self_url: format!("/api/backtests/{}", id),
            results_url: format!("/api/backtests/{}/results", id),
        },
    }))
}

pub async fn get_results(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<BacktestResults>> {
    let res = state.run_store.read_results(id).await.map_err(|_| ApiError::not_found("results not found"))?;
    Ok(Json(res))
}
