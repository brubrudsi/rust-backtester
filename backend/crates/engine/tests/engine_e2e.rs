use engine::run_backtest;
use engine::types::*;

#[test]
fn e2e_ma_crossover_runs_and_is_deterministic() {
    let bars: Vec<Bar> = serde_json::from_str(include_str!("fixtures/spy_15bars.json")).unwrap();

    let cfg = BacktestConfig {
        initial_capital: 10_000.0,
        fees_bps: 1.0,
        slippage_bps: 1.0,
        borrow_annual_pct: 0.0,
        funding_annual_pct: 0.0,
        strategy: StrategyConfig::MaCrossover(MaCrossoverParams {
            fast: 3,
            slow: 6,
            sizing_mode: SizingMode::FixedNotional,
            vol_lookback: 10,
            vol_target: 0.20,
            max_leverage: 1.0,
            stop_loss_pct: None,
        }),
    };

    let r1 = run_backtest(vec!["SPY".into()], vec![bars.clone()], cfg.clone()).unwrap();
    let r2 = run_backtest(vec!["SPY".into()], vec![bars.clone()], cfg).unwrap();

    assert_eq!(r1.timestamps, r2.timestamps);
    assert_eq!(r1.equity.len(), bars.len());
    assert_eq!(r1.equity, r2.equity);
    assert!(r1.equity.iter().all(|x| x.is_finite() && *x > 0.0));
}

#[test]
fn e2e_pairs_zscore_runs() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/aapl_msft_15bars.json")).unwrap();
    let a: Vec<Bar> = serde_json::from_value(fixture["a"].clone()).unwrap();
    let b: Vec<Bar> = serde_json::from_value(fixture["b"].clone()).unwrap();

    let cfg = BacktestConfig {
        initial_capital: 10_000.0,
        fees_bps: 1.0,
        slippage_bps: 1.0,
        borrow_annual_pct: 3.0,
        funding_annual_pct: 0.0,
        strategy: StrategyConfig::PairsZScore(PairsParams {
            lookback: 10,
            entry_z: 1.0,
            exit_z: 0.0,
            hedge_method: HedgeMethod::Ols,
        }),
    };

    let r = run_backtest(vec!["AAPL".into(), "MSFT".into()], vec![a, b], cfg).unwrap();
    assert_eq!(r.timestamps.len(), 15);
    assert!(r.metrics.end_equity.is_finite());
}
