use axum_test::TestServer;

#[tokio::test]
async fn healthz_works() {
    std::env::set_var("API_BIND", "127.0.0.1:0");
    std::env::set_var("UNIVERSE_PATH", "./config/universe.json");

    let cfg = api::config::AppConfig::from_env().unwrap();
    let built = api::app::build_app(cfg).await.unwrap();
    let server = TestServer::new(built.router).unwrap();

    let res = server.get("/api/healthz").await;
    res.assert_status_ok();
}
