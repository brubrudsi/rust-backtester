use crate::types::{Diagnostics, Metrics};

pub fn median_dt_seconds(timestamps: &[i64]) -> f64 {
    if timestamps.len() < 2 {
        return 0.0;
    }
    let mut diffs: Vec<i64> = timestamps
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|d| *d > 0)
        .collect();
    if diffs.is_empty() {
        return 0.0;
    }
    diffs.sort_unstable();
    let mid = diffs.len() / 2;
    let med_ms = if diffs.len() % 2 == 1 {
        diffs[mid] as f64
    } else {
        (diffs[mid - 1] as f64 + diffs[mid] as f64) / 2.0
    };
    med_ms / 1000.0
}

pub fn bars_per_year_from_median_dt(median_dt_seconds: f64) -> f64 {
    if median_dt_seconds <= 0.0 {
        return 0.0;
    }
    let seconds_per_year = 365.25 * 24.0 * 3600.0;
    seconds_per_year / median_dt_seconds
}

pub fn compute_returns(equity: &[f64]) -> Vec<f64> {
    let mut r = vec![0.0; equity.len()];
    for i in 1..equity.len() {
        let prev = equity[i - 1];
        if prev.abs() > 1e-12 {
            r[i] = (equity[i] / prev) - 1.0;
        }
    }
    r
}

pub fn compute_drawdown(equity: &[f64]) -> Vec<f64> {
    let mut dd = vec![0.0; equity.len()];
    let mut peak = f64::NEG_INFINITY;
    for i in 0..equity.len() {
        peak = peak.max(equity[i]);
        dd[i] = if peak > 0.0 { (equity[i] / peak) - 1.0 } else { 0.0 };
    }
    dd
}

pub fn rolling_sharpe(returns: &[f64], window: usize, bars_per_year: f64) -> Vec<Option<f64>> {
    let n = returns.len();
    let mut out = vec![None; n];
    if window == 0 || n == 0 || bars_per_year <= 0.0 {
        return out;
    }

    let mut sum = 0.0;
    let mut sumsq = 0.0;
    for i in 0..n {
        let x = returns[i];
        sum += x;
        sumsq += x * x;

        if i >= window {
            let old = returns[i - window];
            sum -= old;
            sumsq -= old * old;
        }

        if i + 1 >= window {
            let w = window as f64;
            let mean = sum / w;
            let var = (sumsq / w) - (mean * mean);
            let std = var.max(0.0).sqrt();
            if std > 1e-12 {
                out[i] = Some((mean / std) * bars_per_year.sqrt());
            }
        }
    }
    out
}

pub fn summarize_metrics(
    start_equity: f64,
    end_equity: f64,
    returns: &[f64],
    drawdown: &[f64],
    bars_per_year: f64,
    years: f64,
    trades: u32,
    wins: u32,
    gross_profit: f64,
    gross_loss: f64,
    avg_hold_bars: f64,
    turnover_avg: f64,
) -> Metrics {
    let total_return = if start_equity.abs() > 1e-12 {
        (end_equity / start_equity) - 1.0
    } else {
        0.0
    };

    let cagr = if years > 0.0 && start_equity > 0.0 && end_equity > 0.0 {
        (end_equity / start_equity).powf(1.0 / years) - 1.0
    } else {
        0.0
    };

    let mean = returns.iter().copied().sum::<f64>() / returns.len().max(1) as f64;
    let var = returns
        .iter()
        .map(|x| (*x - mean) * (*x - mean))
        .sum::<f64>()
        / returns.len().max(1) as f64;
    let std = var.max(0.0).sqrt();

    let vol_annual = std * bars_per_year.sqrt();
    let sharpe = if std > 1e-12 {
        (mean / std) * bars_per_year.sqrt()
    } else {
        0.0
    };

    let max_dd = drawdown.iter().copied().fold(0.0f64, |acc, x| acc.min(x));

    let win_rate = if trades > 0 { (wins as f64 / trades as f64) * 100.0 } else { 0.0 };

    let profit_factor = if gross_loss.abs() > 1e-12 {
        gross_profit / gross_loss.abs()
    } else if gross_profit > 0.0 {
        f64::INFINITY
    } else {
        0.0
    };

    Metrics {
        start_equity,
        end_equity,
        total_return_pct: total_return * 100.0,
        cagr_pct: cagr * 100.0,
        vol_annual_pct: vol_annual * 100.0,
        sharpe,
        max_drawdown_pct: max_dd * 100.0,
        trades,
        win_rate_pct: win_rate,
        profit_factor,
        avg_hold_bars,
        turnover_avg,
    }
}

pub fn diagnostics(timestamps: &[i64]) -> Diagnostics {
    let med_dt = median_dt_seconds(timestamps);
    let bpy = bars_per_year_from_median_dt(med_dt);
    Diagnostics {
        bars_per_year: bpy,
        median_dt_seconds: med_dt,
    }
}
