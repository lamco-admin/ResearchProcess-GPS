//! Error types and error handling for the protocol

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Protocol-level errors
#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    #[error("Authorization failed: {0}")]
    AuthorizationError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Entity not found: {0}")]
    NotFound(Uuid),
    
    #[error("Version conflict: expected {expected}, got {actual}")]
    VersionConflict { expected: i64, actual: i64 },
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

/// API error response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: ApiErrorBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorDetails {
    ValidationErrors(Vec<ValidationError>),
    ConflictDetails(ConflictDetails),
    RateLimitDetails(RateLimitDetails),
    Custom(HashMap<String, JsonValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetails {
    pub entity_id: Uuid,
    pub current_version: i64,
    pub requested_version: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitDetails {
    pub limit: u32,
    pub remaining: u32,
    pub reset_at: i64, // Unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u32>, // Seconds
}

/// Convert protocol errors to API errors
impl From<ProtocolError> for ApiError {
    fn from(err: ProtocolError) -> Self {
        let (code, message, details) = match err {
            ProtocolError::AuthenticationError(msg) => {
                ("AUTHENTICATION_ERROR", msg, None)
            }
            ProtocolError::AuthorizationError(msg) => {
                ("AUTHORIZATION_ERROR", msg, None)
            }
            ProtocolError::ValidationError(msg) => {
                ("VALIDATION_ERROR", msg, None)
            }
            ProtocolError::NotFound(id) => {
                ("NOT_FOUND", format!("Entity {} not found", id), None)
            }
            ProtocolError::VersionConflict { expected, actual } => {
                let details = ConflictDetails {
                    entity_id: Uuid::nil(), // Should be provided by context
                    current_version: actual,
                    requested_version: expected,
                    conflict_type: Some("VERSION_MISMATCH".to_string()),
                };
                ("CONFLICT", "Version conflict detected".to_string(), 
                 Some(ErrorDetails::ConflictDetails(details)))
            }
            ProtocolError::RateLimitExceeded => {
                ("RATE_LIMIT", "Rate limit exceeded".to_string(), None)
            }
            ProtocolError::InternalError(msg) => {
                ("INTERNAL_ERROR", msg, None)
            }
            ProtocolError::Protocol(msg) => {
                ("PROTOCOL_ERROR", msg, None)
            }
            ProtocolError::Serialization(err) => {
                ("SERIALIZATION_ERROR", err.to_string(), None)
            }
            ProtocolError::InvalidRequest(msg) => {
                ("INVALID_REQUEST", msg, None)
            }
        };
        
        ApiError {
            error: ApiErrorBody {
                code: code.to_string(),
                message,
                details,
                request_id: None,
            },
        }
    }
}

/// Result type for protocol operations
pub type ProtocolResult<T> = Result<T, ProtocolError>;

/// HTTP status code mappings
impl ProtocolError {
    pub fn status_code(&self) -> u16 {
        match self {
            ProtocolError::AuthenticationError(_) => 401,
            ProtocolError::AuthorizationError(_) => 403,
            ProtocolError::ValidationError(_) => 400,
            ProtocolError::NotFound(_) => 404,
            ProtocolError::VersionConflict { .. } => 409,
            ProtocolError::RateLimitExceeded => 429,
            ProtocolError::InternalError(_) => 500,
            ProtocolError::Protocol(_) => 400,
            ProtocolError::Serialization(_) => 400,
            ProtocolError::InvalidRequest(_) => 400,
        }
    }
}