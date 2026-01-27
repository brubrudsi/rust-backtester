use super::{MarketDataProvider, Timeframe};
use crate::market_data::cache::FileCache;
use async_trait::async_trait;
use engine::types::Bar;
use reqwest::StatusCode;
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct PolygonProvider {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    cache: FileCache,
}

impl PolygonProvider {
    pub fn new(api_key: String, base_url: String, cache: FileCache) -> anyhow::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("quant-web-backtester/0.1")
            .build()?;
        Ok(Self { client, api_key, base_url, cache })
    }

    fn cache_key(&self, ticker: &str, tf: &Timeframe, start: &str, end: &str, adjusted: bool) -> String {
        format!(
            "aggs__ticker={ticker}__m={}__span={}__from={}__to={}__adj={}",
            tf.multiplier, tf.timespan, start, end, adjusted
        )
    }

    async fn get_with_retry(&self, url: &str, query: &[(&str, String)]) -> anyhow::Result<String> {
        let mut attempt = 0u32;
        let max_attempts = 6u32;

        loop {
            let resp = self.client.get(url).query(query).send().await;

            match resp {
                Ok(r) => {
                    let status = r.status();
                    let text = r.text().await.unwrap_or_default();

                    if status.is_success() {
                        return Ok(text);
                    }

                    if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
                        let backoff = (2u64).pow(attempt.min(5)) * 250;
                        let sleep_ms = backoff.min(10_000);
                        tracing::warn!(%status, attempt, "polygon request retrying in {}ms", sleep_ms);
                        sleep(Duration::from_millis(sleep_ms)).await;
                        attempt += 1;
                        if attempt > max_attempts {
                            anyhow::bail!("polygon request failed after retries: {} {}", status, text);
                        }
                        continue;
                    }

                    anyhow::bail!("polygon request failed: {} {}", status, text);
                }
                Err(e) => {
                    if attempt >= max_attempts {
                        return Err(e.into());
                    }
                    let backoff = (2u64).pow(attempt.min(5)) * 250;
                    tracing::warn!(attempt, "polygon request error; retrying in {}ms: {}", backoff, e);
                    sleep(Duration::from_millis(backoff)).await;
                    attempt += 1;
                }
            }
        }
    }

    async fn fetch_segment(&self, ticker: &str, tf: &Timeframe, start: &str, end: &str, adjusted: bool) -> anyhow::Result<Vec<Bar>> {
        let key = self.cache_key(ticker, tf, start, end, adjusted);
        if let Some(cached) = self.cache.get_json(&key).await? {
            let parsed: AggResponse = serde_json::from_str(&cached)?;
            return Ok(parsed.into_bars());
        }

        let url = format!(
            "{}/v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            self.base_url.trim_end_matches('/'),
            urlencoding::encode(ticker),
            tf.multiplier,
            tf.timespan,
            start,
            end
        );

        let query = vec![
            ("adjusted", adjusted.to_string()),
            ("sort", "asc".to_string()),
            ("limit", "50000".to_string()),
            ("apiKey", self.api_key.clone()),
        ];

        let text = self.get_with_retry(&url, &query).await?;
        self.cache.put_json(&key, &text).await?;

        let parsed: AggResponse = serde_json::from_str(&text)?;
        Ok(parsed.into_bars())
    }
}

#[derive(Debug, Deserialize)]
struct AggResponse {
    #[serde(default)]
    status: String,
    #[serde(default)]
    results: Vec<AggBar>,
    #[serde(default)]
    next_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AggBar {
    t: i64,
    o: f64,
    h: f64,
    l: f64,
    c: f64,
    #[serde(default)]
    v: f64,
}

impl AggResponse {
    fn into_bars(self) -> Vec<Bar> {
        let mut bars: Vec<Bar> = self
            .results
            .into_iter()
            .map(|b| Bar { t: b.t, o: b.o, h: b.h, l: b.l, c: b.c, v: b.v })
            .collect();
        bars.sort_by_key(|b| b.t);
        bars
    }
}

#[async_trait]
impl MarketDataProvider for PolygonProvider {
    async fn get_bars(&self, polygon_ticker: &str, tf: &Timeframe, start: &str, end: &str, adjusted: bool) -> anyhow::Result<Vec<Bar>> {
        if self.api_key == "MISSING" {
            anyhow::bail!("POLYGON_API_KEY not set on server");
        }

        if tf.timespan == "minute" && tf.multiplier <= 5 {
            let chunk_days = if tf.multiplier == 1 { 30 } else { 120 };

            let start_date = chrono::NaiveDate::parse_from_str(start, "%Y-%m-%d")?;
            let end_date = chrono::NaiveDate::parse_from_str(end, "%Y-%m-%d")?;

            let mut cur = start_date;
            let mut out = Vec::new();

            while cur <= end_date {
                let seg_end = (cur + chrono::Duration::days(chunk_days)).min(end_date);
                let seg_start_s = cur.format("%Y-%m-%d").to_string();
                let seg_end_s = seg_end.format("%Y-%m-%d").to_string();

                let mut seg = self.fetch_segment(polygon_ticker, tf, &seg_start_s, &seg_end_s, adjusted).await?;
                out.append(&mut seg);

                cur = seg_end + chrono::Duration::days(1);
            }

            out.sort_by_key(|b| b.t);
            out.dedup_by_key(|b| b.t);
            return Ok(out);
        }

        self.fetch_segment(polygon_ticker, tf, start, end, adjusted).await
    }
}
