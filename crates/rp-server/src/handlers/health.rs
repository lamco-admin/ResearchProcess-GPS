use axum::response::Json;
use rp_protocol::{HealthResponse, HealthStatus};

pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: HealthStatus::Healthy,
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now(),
        details: None,
    })
}