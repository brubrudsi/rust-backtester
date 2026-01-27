pub mod backtests;
pub mod health;
pub mod universe;

use axum::Router;

use crate::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/healthz", axum::routing::get(health::healthz))
        .route("/api/universe", axum::routing::get(universe::universe))
        .route("/api/backtests", axum::routing::post(backtests::create_backtest))
        .route("/api/backtests/:id", axum::routing::get(backtests::get_backtest))
        .route(
            "/api/backtests/:id/results",
            axum::routing::get(backtests::get_results),
        )
}
