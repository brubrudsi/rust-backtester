use std::{env, net::SocketAddr, path::PathBuf};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub bind: SocketAddr,
    pub polygon_api_key: Option<String>,
    pub polygon_base_url: String,
    pub universe_path: PathBuf,
    pub data_dir: PathBuf,
    pub run_ttl_hours: u64,
    pub cache_ttl_days: u64,
    pub cors_allow_origins: Vec<String>,
    pub rate_limit_rpm: u32,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind = env::var("API_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
        let bind: SocketAddr = bind.parse()?;

        let polygon_api_key = env::var("POLYGON_API_KEY").ok();
        let polygon_base_url = env::var("POLYGON_BASE_URL").unwrap_or_else(|_| "https://api.polygon.io".into());

        let universe_path = env::var("UNIVERSE_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./config/universe.json"));

        let data_dir = env::var("DATA_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/data"));

        let run_ttl_hours = env::var("RUN_TTL_HOURS").ok().and_then(|s| s.parse().ok()).unwrap_or(24);
        let cache_ttl_days = env::var("CACHE_TTL_DAYS").ok().and_then(|s| s.parse().ok()).unwrap_or(30);

        let cors_allow_origins = env::var("CORS_ALLOW_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let rate_limit_rpm = env::var("RATE_LIMIT_RPM").ok().and_then(|s| s.parse().ok()).unwrap_or(240);

        Ok(Self {
            bind,
            polygon_api_key,
            polygon_base_url,
            universe_path,
            data_dir,
            run_ttl_hours,
            cache_ttl_days,
            cors_allow_origins,
            rate_limit_rpm,
        })
    }
}
