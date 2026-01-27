use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_http::compression::CompressionLayer;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;

use std::{net::SocketAddr, sync::Arc, time::Duration};

use crate::config::AppConfig;
use crate::market_data::cache::FileCache;
use crate::market_data::polygon::PolygonProvider;
use crate::market_data::MarketDataProvider;
use crate::routes;
use crate::storage::runs::RunStore;
use crate::types::UniverseFile;

#[derive(Clone)]
pub struct AppState {
    pub bind_addr: SocketAddr,
    pub polygon_key_present: bool,
    pub universe: UniverseFile,
    pub data_provider: Arc<dyn MarketDataProvider>,
    pub run_store: RunStore,
}

pub struct BuiltApp {
    pub bind_addr: SocketAddr,
    pub router: Router,
}

pub async fn build_app(cfg: AppConfig) -> anyhow::Result<BuiltApp> {
    let universe_str = std::fs::read_to_string(&cfg.universe_path)?;
    let universe: UniverseFile = serde_json::from_str(&universe_str)?;

    let run_store = RunStore::new(cfg.data_dir.clone(), cfg.run_ttl_hours);
    run_store.init().await?;

    let cache = FileCache::new(cfg.data_dir.join("cache"), cfg.cache_ttl_days);
    cache.ensure_dir().await?;

    let polygon_key_present = cfg.polygon_api_key.is_some();
    let data_provider: Arc<dyn MarketDataProvider> = if let Some(key) = cfg.polygon_api_key.clone() {
        Arc::new(PolygonProvider::new(key, cfg.polygon_base_url.clone(), cache)?)
    } else {
        Arc::new(PolygonProvider::new("MISSING".into(), cfg.polygon_base_url.clone(), cache)?)
    };

    let state = AppState {
        bind_addr: cfg.bind,
        polygon_key_present,
        universe,
        data_provider,
        run_store: run_store.clone(),
    };

    // Background cleanup
    let cleanup_store = run_store.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60 * 30));
        loop {
            interval.tick().await;
            if let Err(e) = cleanup_store.cleanup_old_runs().await {
                tracing::warn!("run cleanup error: {}", e);
            }
        }
    });

    // Rate limiting (token bucket via tower-governor).
// We interpret cfg.rate_limit_rpm as "average requests per minute per client IP".
// tower-governor models a quota as: burst_size tokens, refilling 1 token every `period`.
let rpm: u64 = cfg.rate_limit_rpm.max(1) as u64;
let period_ms: u64 = (60_000u64 / rpm).max(1); // integer ms; good enough for demo infra
let burst_size: u32 = ((cfg.rate_limit_rpm / 10).max(10)).max(1) as u32;

let governor_conf = GovernorConfigBuilder::default()
    .per_millisecond(period_ms)
    .burst_size(burst_size)
    .key_extractor(SmartIpKeyExtractor)
    .finish()
    .unwrap();

let governor_layer = GovernorLayer { config: Arc::new(governor_conf) };


    // CORS
    let mut cors = CorsLayer::new();
    for origin in cfg.cors_allow_origins.iter() {
        cors = cors.allow_origin(origin.parse::<axum::http::HeaderValue>()?);
    }
    cors = cors
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let router = Router::new()
        .merge(routes::router())
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(governor_layer)
        .with_state(state);

    Ok(BuiltApp { bind_addr: cfg.bind, router })
}
