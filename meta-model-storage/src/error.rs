//! Storage error types

use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Storage backend error: {0}")]
    Backend(String),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Entity already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Capability not supported: {0}")]
    NotSupported(String),

    #[error("Storage is read-only")]
    ReadOnly,

    #[error("Storage is offline")]
    Offline,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}