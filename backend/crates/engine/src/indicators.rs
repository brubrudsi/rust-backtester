use crate::types::Bar;

pub fn sma(values: &[f64], window: usize) -> Vec<Option<f64>> {
    if window == 0 {
        return vec![None; values.len()];
    }
    let mut out = vec![None; values.len()];
    let mut sum = 0.0;
    for i in 0..values.len() {
        sum += values[i];
        if i >= window {
            sum -= values[i - window];
        }
        if i + 1 >= window {
            out[i] = Some(sum / window as f64);
        }
    }
    out
}

pub fn rolling_mean_std(values: &[f64], window: usize) -> (Vec<Option<f64>>, Vec<Option<f64>>) {
    if window == 0 {
        return (vec![None; values.len()], vec![None; values.len()]);
    }
    let mut mean = vec![None; values.len()];
    let mut std = vec![None; values.len()];
    let mut sum = 0.0;
    let mut sumsq = 0.0;

    for i in 0..values.len() {
        let x = values[i];
        sum += x;
        sumsq += x * x;

        if i >= window {
            let old = values[i - window];
            sum -= old;
            sumsq -= old * old;
        }

        if i + 1 >= window {
            let w = window as f64;
            let m = sum / w;
            let var = (sumsq / w) - (m * m);
            mean[i] = Some(m);
            std[i] = Some(var.max(0.0).sqrt());
        }
    }
    (mean, std)
}

/// Rolling max excluding the current bar: out[i] = max(values[i-window..i])
pub fn rolling_max_exclusive(values: &[f64], window: usize) -> Vec<Option<f64>> {
    let n = values.len();
    let mut out = vec![None; n];
    if window == 0 {
        return out;
    }
    for i in 0..n {
        if i < window {
            continue;
        }
        let mut m = f64::NEG_INFINITY;
        for x in &values[i - window..i] {
            if *x > m {
                m = *x;
            }
        }
        out[i] = Some(m);
    }
    out
}

/// Rolling min excluding the current bar: out[i] = min(values[i-window..i])
pub fn rolling_min_exclusive(values: &[f64], window: usize) -> Vec<Option<f64>> {
    let n = values.len();
    let mut out = vec![None; n];
    if window == 0 {
        return out;
    }
    for i in 0..n {
        if i < window {
            continue;
        }
        let mut m = f64::INFINITY;
        for x in &values[i - window..i] {
            if *x < m {
                m = *x;
            }
        }
        out[i] = Some(m);
    }
    out
}

pub fn true_range(bars: &[Bar]) -> Vec<f64> {
    let mut tr = Vec::with_capacity(bars.len());
    for i in 0..bars.len() {
        if i == 0 {
            tr.push(bars[i].h - bars[i].l);
        } else {
            let prev_c = bars[i - 1].c;
            let a = bars[i].h - bars[i].l;
            let b = (bars[i].h - prev_c).abs();
            let c = (bars[i].l - prev_c).abs();
            tr.push(a.max(b).max(c));
        }
    }
    tr
}

pub fn atr(bars: &[Bar], window: usize) -> Vec<Option<f64>> {
    let tr = true_range(bars);
    sma(&tr, window)
}

/// Rolling OLS beta with intercept: beta = cov(y,x)/var(x).
pub fn rolling_beta_ols(y: &[f64], x: &[f64], window: usize) -> Vec<Option<f64>> {
    let n = y.len().min(x.len());
    let mut out = vec![None; n];
    if window == 0 || n == 0 {
        return out;
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_x2 = 0.0;
    let mut sum_xy = 0.0;

    for i in 0..n {
        let xi = x[i];
        let yi = y[i];
        sum_x += xi;
        sum_y += yi;
        sum_x2 += xi * xi;
        sum_xy += xi * yi;

        if i >= window {
            let xo = x[i - window];
            let yo = y[i - window];
            sum_x -= xo;
            sum_y -= yo;
            sum_x2 -= xo * xo;
            sum_xy -= xo * yo;
        }

        if i + 1 >= window {
            let w = window as f64;
            let denom = (w * sum_x2) - (sum_x * sum_x);
            if denom.abs() > 1e-12 {
                let num = (w * sum_xy) - (sum_x * sum_y);
                out[i] = Some(num / denom);
            } else {
                out[i] = None;
            }
        }
    }
    out
}

/// Rolling ratio estimate via SMA(p1/p2).
pub fn rolling_ratio(p1: &[f64], p2: &[f64], window: usize) -> Vec<Option<f64>> {
    let n = p1.len().min(p2.len());
    let mut ratio = vec![0.0; n];
    for i in 0..n {
        ratio[i] = p1[i] / p2[i];
    }
    sma(&ratio, window)
}
