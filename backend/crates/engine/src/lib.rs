pub mod indicators;
pub mod metrics;
pub mod strategies;
pub mod types;

use crate::types::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

/// Main entry point: run a deterministic backtest.
///
/// - Signals computed on bar close.
/// - Orders execute at next bar open.
/// - Costs and carry applied in a deterministic accounting loop.
///
/// For pairs, the two bar series must be aligned (same timestamps).
pub fn run_backtest(
    symbols: Vec<String>,
    bars_by_symbol: Vec<Vec<Bar>>,
    cfg: BacktestConfig,
) -> Result<BacktestResult, EngineError> {
    if symbols.is_empty() || bars_by_symbol.is_empty() || symbols.len() != bars_by_symbol.len() {
        return Err(EngineError::InvalidInput("symbols/bars length mismatch".into()));
    }
    if cfg.initial_capital <= 0.0 {
        return Err(EngineError::InvalidInput("initial_capital must be > 0".into()));
    }
    if cfg.fees_bps < 0.0 || cfg.slippage_bps < 0.0 {
        return Err(EngineError::InvalidInput(
            "fees_bps and slippage_bps must be >= 0".into(),
        ));
    }
    for (s, bars) in symbols.iter().zip(bars_by_symbol.iter()) {
        if bars.len() < 5 {
            return Err(EngineError::InvalidInput(format!("not enough bars for {}", s)));
        }
        for w in bars.windows(2) {
            if w[1].t <= w[0].t {
                return Err(EngineError::InvalidInput(format!(
                    "bars not strictly increasing for {}",
                    s
                )));
            }
        }
        if bars
            .iter()
            .any(|b| b.o <= 0.0 || b.h <= 0.0 || b.l <= 0.0 || b.c <= 0.0)
        {
            return Err(EngineError::InvalidInput(format!(
                "non-positive prices for {}",
                s
            )));
        }
    }

    match (&cfg.strategy, bars_by_symbol.len()) {
        (StrategyConfig::PairsZScore(_), 2) => {
            strategies::pairs_zscore::run(&symbols, &bars_by_symbol, &cfg)
        }
        (StrategyConfig::PairsZScore(_), _) => Err(EngineError::InvalidInput(
            "pairs_zscore requires exactly 2 symbols".into(),
        )),
        (_, 1) => strategies::single_asset::run(&symbols[0], &bars_by_symbol[0], &cfg),
        _ => Err(EngineError::InvalidInput(
            "single-asset strategies require exactly 1 symbol".into(),
        )),
    }
}
