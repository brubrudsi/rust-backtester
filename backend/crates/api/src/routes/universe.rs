use axum::{extract::State, Json};

use crate::app::AppState;
use crate::types::UniverseFile;

pub async fn universe(State(state): State<AppState>) -> Json<UniverseFile> {
    Json(state.universe.clone())
}
