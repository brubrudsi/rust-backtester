use crate::indicators;
use crate::types::{Bar, ZScoreMRParams};
use crate::EngineError;
use std::collections::BTreeMap;

pub fn indicators(closes: &[f64], p: &ZScoreMRParams) -> BTreeMap<String, Vec<Option<f64>>> {
    let (mean, std) = indicators::rolling_mean_std(closes, p.lookback);
    let mut upper = vec![None; closes.len()];
    let mut lower = vec![None; closes.len()];
    let mut z = vec![None; closes.len()];

    for i in 0..closes.len() {
        if let (Some(m), Some(s)) = (mean[i], std[i]) {
            upper[i] = Some(m + p.entry_z * s);
            lower[i] = Some(m - p.entry_z * s);
            if s > 1e-12 {
                z[i] = Some((closes[i] - m) / s);
            }
        }
    }

    let mut out = BTreeMap::new();
    out.insert("mean".into(), mean);
    out.insert("upper".into(), upper);
    out.insert("lower".into(), lower);
    out.insert("zscore".into(), z);
    out
}

pub fn decision_fn<'a>(
    bars: &'a [Bar],
    closes: &'a [f64],
    p: &'a ZScoreMRParams,
) -> Box<dyn FnMut(usize, f64, f64) -> Result<f64, EngineError> + 'a> {
    if p.lookback < 5 || p.entry_z <= 0.0 {
        return Box::new(move |_i, _eq, _qty| {
            Err(EngineError::InvalidInput(
                "zscore_mr requires lookback>=5 and entry_z>0".into(),
            ))
        });
    }
    if p.exit_z < 0.0 {
        return Box::new(move |_i, _eq, _qty| {
            Err(EngineError::InvalidInput("exit_z must be >= 0".into()))
        });
    }
    if let Some(x) = p.stop_loss_pct {
        if x <= 0.0 || x >= 100.0 {
            return Box::new(move |_i, _eq, _qty| {
                Err(EngineError::InvalidInput(
                    "stop_loss_pct must be in (0,100)".into(),
                ))
            });
        }
    }

    let (mean, std) = indicators::rolling_mean_std(closes, p.lookback);
    let mut z = vec![None; closes.len()];
    for i in 0..closes.len() {
        if let (Some(m), Some(s)) = (mean[i], std[i]) {
            if s > 1e-12 {
                z[i] = Some((closes[i] - m) / s);
            }
        }
    }

    let mut entry_price: Option<f64> = None;
    let mut bars_held: usize = 0;
    let mut pos_sign: i32 = 0;

    Box::new(move |i: usize, equity_at_close: f64, _current_qty: f64| -> Result<f64, EngineError> {
        let zi = z[i];
        if zi.is_none() {
            return Ok(0.0);
        }
        let zi = zi.unwrap();

        // Next-bar execution (no lookahead): decision on close i, fill at open i+1.
        let next_open = bars[i + 1].o;

        if pos_sign != 0 {
            bars_held += 1;
        }

        // Risk controls evaluated on close (conservative)
        if pos_sign != 0 {
            if let Some(sl) = p.stop_loss_pct {
                if let Some(ep) = entry_price {
                    let adverse = if pos_sign > 0 {
                        (ep - closes[i]) / ep
                    } else {
                        (closes[i] - ep) / ep
                    };
                    if adverse >= sl / 100.0 {
                        pos_sign = 0;
                        entry_price = None;
                        bars_held = 0;
                        return Ok(0.0);
                    }
                }
            }
            if let Some(ts) = p.time_stop_bars {
                if bars_held >= ts {
                    pos_sign = 0;
                    entry_price = None;
                    bars_held = 0;
                    return Ok(0.0);
                }
            }
        }

        // Mean reversion exits
        if pos_sign > 0 && zi >= p.exit_z {
            pos_sign = 0;
            entry_price = None;
            bars_held = 0;
            return Ok(0.0);
        }
        if pos_sign < 0 && zi <= -p.exit_z {
            pos_sign = 0;
            entry_price = None;
            bars_held = 0;
            return Ok(0.0);
        }

        // Entries
        if pos_sign == 0 {
            if zi <= -p.entry_z {
                pos_sign = 1;
                entry_price = Some(next_open);
                bars_held = 0;
            } else if zi >= p.entry_z {
                pos_sign = -1;
                entry_price = Some(next_open);
                bars_held = 0;
            }
        }

        if next_open <= 0.0 {
            return Ok(0.0);
        }

        let target_notional = (pos_sign as f64) * equity_at_close;
        Ok(target_notional / next_open)
    })
}
