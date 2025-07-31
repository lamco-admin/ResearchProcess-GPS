use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use rp_protocol::{ApiError as ProtocolApiError, ApiErrorBody};

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not found")]
    NotFound,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authorization error")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Storage error: {0}")]
    Storage(#[from] rp_storage::StorageError),

    #[error("Internal server error")]
    Internal(String),
}

impl ApiError {
    pub fn to_protocol_error(&self, request_id: Option<String>) -> ProtocolApiError {
        let (code, message, details) = match self {
            Self::NotFound => ("NOT_FOUND", "Resource not found".to_string(), None),
            Self::Validation(msg) => (
                "VALIDATION_ERROR",
                "Validation failed".to_string(),
                Some(serde_json::json!({ "error": msg })),
            ),
            Self::Unauthorized => (
                "AUTHENTICATION_ERROR",
                "Authentication required".to_string(),
                None,
            ),
            Self::Forbidden => (
                "AUTHORIZATION_ERROR",
                "Access denied".to_string(),
                None,
            ),
            Self::Conflict(msg) => (
                "CONFLICT",
                "Resource conflict".to_string(),
                Some(serde_json::json!({ "error": msg })),
            ),
            Self::Database(e) => (
                "INTERNAL_ERROR",
                "Database error".to_string(),
                Some(serde_json::json!({ "error": e.to_string() })),
            ),
            Self::Storage(e) => (
                "INTERNAL_ERROR",
                "Storage error".to_string(),
                Some(serde_json::json!({ "error": e.to_string() })),
            ),
            Self::Internal(msg) => (
                "INTERNAL_ERROR",
                "Internal server error".to_string(),
                Some(serde_json::json!({ "error": msg })),
            ),
        };

        ProtocolApiError {
            error: ApiErrorBody {
                code: code.to_string(),
                message,
                details: details.map(|d| {
                    use std::collections::HashMap;
                    let mut map = HashMap::new();
                    if let serde_json::Value::Object(obj) = d {
                        for (k, v) in obj {
                            map.insert(k, v);
                        }
                    }
                    rp_protocol::ErrorDetails::Custom(map)
                }),
                request_id,
            },
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::Database(_) | Self::Storage(_) | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(self.to_protocol_error(None));

        (status, body).into_response()
    }
}