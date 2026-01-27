use crate::indicators;
use crate::types::{Bar, MaCrossoverParams, SizingMode};
use crate::EngineError;
use std::collections::BTreeMap;

pub fn indicators(closes: &[f64], p: &MaCrossoverParams) -> BTreeMap<String, Vec<Option<f64>>> {
    let mut m = BTreeMap::new();
    m.insert("fast_ma".into(), indicators::sma(closes, p.fast));
    m.insert("slow_ma".into(), indicators::sma(closes, p.slow));
    m
}

pub fn decision_fn<'a>(
    bars: &'a [Bar],
    closes: &'a [f64],
    p: &'a MaCrossoverParams,
    bars_per_year: f64,
) -> Box<dyn FnMut(usize, f64, f64) -> Result<f64, EngineError> + 'a> {
    if p.fast == 0 || p.slow == 0 || p.fast >= p.slow {
        return Box::new(move |_i, _eq, _qty| {
            Err(EngineError::InvalidInput(
                "ma_crossover requires 0 < fast < slow".into(),
            ))
        });
    }
    if matches!(p.sizing_mode, SizingMode::VolTarget)
        && (p.vol_lookback < 2 || p.vol_target <= 0.0 || p.max_leverage <= 0.0)
    {
        return Box::new(move |_i, _eq, _qty| {
            Err(EngineError::InvalidInput("invalid vol-target params".into()))
        });
    }

    let fast = indicators::sma(closes, p.fast);
    let slow = indicators::sma(closes, p.slow);

    let mut rets = vec![0.0; closes.len()];
    for i in 1..closes.len() {
        rets[i] = (closes[i] / closes[i - 1]) - 1.0;
    }

    Box::new(move |i: usize, equity_at_close: f64, _current_qty: f64| -> Result<f64, EngineError> {
        let f = fast[i];
        let s = slow[i];
        if f.is_none() || s.is_none() {
            return Ok(0.0);
        }

        let diff = f.unwrap() - s.unwrap();
        let desired_sign = if diff > 0.0 {
            1.0
        } else if diff < 0.0 {
            -1.0
        } else {
            0.0
        };

        if desired_sign == 0.0 {
            return Ok(0.0);
        }

        // Next-bar execution (no lookahead): decision on close i, fill at open i+1.
        let next_open = bars[i + 1].o;
        if next_open <= 0.0 {
            return Ok(0.0);
        }

        let leverage = match p.sizing_mode {
            SizingMode::FixedNotional => 1.0,
            SizingMode::VolTarget => {
                let lb = p.vol_lookback;
                if i + 1 < lb {
                    return Ok(0.0);
                }
                let start = i + 1 - lb;
                let slice = &rets[start..=i];
                let mean = slice.iter().sum::<f64>() / slice.len() as f64;
                let mut var = 0.0;
                for x in slice {
                    var += (*x - mean) * (*x - mean);
                }
                var /= slice.len() as f64;
                let std = var.max(0.0).sqrt();
                let vol_annual = std * bars_per_year.sqrt();
                if vol_annual <= 1e-12 {
                    0.0
                } else {
                    (p.vol_target / vol_annual).clamp(0.0, p.max_leverage)
                }
            }
        };

        let target_notional = desired_sign * equity_at_close * leverage;
        Ok(target_notional / next_open)
    })
}
