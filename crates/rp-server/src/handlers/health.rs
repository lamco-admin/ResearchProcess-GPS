use axum::response::Json;
use rp_protocol::{HealthResponse, HealthStatus};
use utoipa;

/// Health check endpoint
///
/// Returns the current health status of the API server
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy")
    ),
    tag = "health"
)]
pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: HealthStatus::Healthy,
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now(),
        details: None,
    })
}