use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_level(true)
        .json()
        .init();

    let cfg = api::config::AppConfig::from_env()?;
    let built = api::app::build_app(cfg).await?;

    let addr = built.bind_addr;
    tracing::info!(%addr, "api listening");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        built.router.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
