use crate::indicators;
use crate::types::{Bar, DonchianParams};
use crate::EngineError;
use std::collections::BTreeMap;

pub fn indicators(
    bars: &[Bar],
    highs: &[f64],
    lows: &[f64],
    p: &DonchianParams,
) -> BTreeMap<String, Vec<Option<f64>>> {
    let upper = indicators::rolling_max_exclusive(highs, p.lookback);
    let lower = indicators::rolling_min_exclusive(lows, p.lookback);
    let atr = indicators::atr(bars, p.atr_period);

    let mut out = BTreeMap::new();
    out.insert("donchian_upper".into(), upper);
    out.insert("donchian_lower".into(), lower);
    out.insert("atr".into(), atr);
    out
}

pub fn decision_fn<'a>(
    bars: &'a [Bar],
    p: &'a DonchianParams,
) -> Box<dyn FnMut(usize, f64, f64) -> Result<f64, EngineError> + 'a> {
    if p.lookback < 5 {
        return Box::new(move |_i, _eq, _qty| {
            Err(EngineError::InvalidInput(
                "donchian lookback must be >= 5".into(),
            ))
        });
    }
    if p.atr_period < 5 {
        return Box::new(move |_i, _eq, _qty| {
            Err(EngineError::InvalidInput("atr_period must be >= 5".into()))
        });
    }
    if let Some(m) = p.atr_stop_mult {
        if m <= 0.0 {
            return Box::new(move |_i, _eq, _qty| {
                Err(EngineError::InvalidInput(
                    "atr_stop_mult must be > 0".into(),
                ))
            });
        }
    }

    let highs: Vec<f64> = bars.iter().map(|b| b.h).collect();
    let lows: Vec<f64> = bars.iter().map(|b| b.l).collect();
    let closes: Vec<f64> = bars.iter().map(|b| b.c).collect();

    let upper = indicators::rolling_max_exclusive(&highs, p.lookback);
    let lower = indicators::rolling_min_exclusive(&lows, p.lookback);
    let atr = indicators::atr(bars, p.atr_period);

    let mut pos_sign: i32 = 0;
    let mut entry_price: Option<f64> = None;
    let mut trail_ref: Option<f64> = None;

    Box::new(move |i: usize, equity_at_close: f64, _current_qty: f64| -> Result<f64, EngineError> {
        // Next-bar execution (no lookahead): decision on close i, fill at open i+1.
        let next_open = bars[i + 1].o;

        if pos_sign > 0 {
            trail_ref = Some(trail_ref.unwrap_or(closes[i]).max(closes[i]));
        } else if pos_sign < 0 {
            trail_ref = Some(trail_ref.unwrap_or(closes[i]).min(closes[i]));
        }

        // Stops evaluated on close (conservative; avoids intrabar assumptions)
        if pos_sign != 0 {
            if let (Some(mult), Some(a)) = (p.atr_stop_mult, atr[i]) {
                let stop = mult * a;
                if let Some(ep) = entry_price {
                    if pos_sign > 0 && closes[i] <= ep - stop {
                        pos_sign = 0;
                        entry_price = None;
                        trail_ref = None;
                    } else if pos_sign < 0 && closes[i] >= ep + stop {
                        pos_sign = 0;
                        entry_price = None;
                        trail_ref = None;
                    }
                }
            }
            if p.trailing_stop {
                if let (Some(mult), Some(a), Some(tr)) = (p.atr_stop_mult, atr[i], trail_ref) {
                    let stop = mult * a;
                    if pos_sign > 0 && closes[i] <= tr - stop {
                        pos_sign = 0;
                        entry_price = None;
                        trail_ref = None;
                    } else if pos_sign < 0 && closes[i] >= tr + stop {
                        pos_sign = 0;
                        entry_price = None;
                        trail_ref = None;
                    }
                }
            }
        }

        // Breakout entries on close vs channel computed from prior bars (exclusive)
        if let (Some(u), Some(l)) = (upper[i], lower[i]) {
            if closes[i] > u {
                if pos_sign != 1 {
                    pos_sign = 1;
                    entry_price = Some(next_open);
                    trail_ref = Some(closes[i]);
                }
            } else if closes[i] < l {
                if pos_sign != -1 {
                    pos_sign = -1;
                    entry_price = Some(next_open);
                    trail_ref = Some(closes[i]);
                }
            }
        }

        if next_open <= 0.0 {
            return Ok(0.0);
        }

        let target_notional = (pos_sign as f64) * equity_at_close;
        Ok(target_notional / next_open)
    })
}
