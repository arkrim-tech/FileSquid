//! HTTP-shaped routes of the FileSquid API. Paths are relative to [`crate::API_PREFIX`].

use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use filesquid_core::{health, HealthStatus};

use crate::{ApiState, API_PREFIX, CONTRACT};

/// Build the FileSquid API router. Hosts mount it unchanged.
pub fn router(state: ApiState) -> Router {
    let api = Router::new()
        .route("/health", get(health_handler))
        .route("/openapi.json", get(contract_handler))
        .merge(crate::admin::routes())
        .with_state(state);
    Router::new().nest(API_PREFIX, api)
}

/// `GET /api/v1/health`: process health from `filesquid-core`.
async fn health_handler() -> Json<HealthStatus> {
    Json(health())
}

/// `GET /api/v1/openapi.json`: the API contract this build serves.
async fn contract_handler() -> impl IntoResponse {
    ([(CONTENT_TYPE, "application/json")], CONTRACT)
}
