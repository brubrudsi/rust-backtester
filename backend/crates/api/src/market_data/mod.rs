pub mod cache;
pub mod polygon;

use async_trait::async_trait;
use engine::types::Bar;

#[derive(Clone, Debug)]
pub struct Timeframe {
    pub multiplier: u32,
    pub timespan: String,
    pub label: String,
}

#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    async fn get_bars(
        &self,
        polygon_ticker: &str,
        timeframe: &Timeframe,
        start: &str,
        end: &str,
        adjusted: bool,
    ) -> anyhow::Result<Vec<Bar>>;
}
