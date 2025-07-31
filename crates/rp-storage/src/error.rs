//! Storage error types

use thiserror::Error;
use uuid::Uuid;

/// Storage operation errors
#[derive(Error, Debug)]
pub enum StorageError {
    /// Entity not found
    #[error("Entity not found: {0}")]
    NotFound(Uuid),
    
    /// Entity already exists
    #[error("Entity already exists: {0}")]
    AlreadyExists(Uuid),
    
    /// Version conflict during update
    #[error("Version conflict for entity {id}: expected {expected}, found {actual}")]
    VersionConflict {
        id: Uuid,
        expected: u64,
        actual: u64,
    },
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    /// Query error
    #[error("Query error: {0}")]
    QueryError(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Backend not initialized
    #[error("Storage backend not initialized")]
    NotInitialized,
    
    /// Operation not supported by backend
    #[error("Operation not supported: {0}")]
    NotSupported(String),
    
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    
    /// Capacity exceeded
    #[error("Capacity exceeded: {0}")]
    CapacityExceeded(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Timeout
    #[error("Operation timed out")]
    Timeout,
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Other backend-specific error
    #[error("Backend error: {0}")]
    BackendError(String),
    
    /// Multiple errors (for bulk operations)
    #[error("Multiple errors occurred: {0:?}")]
    MultipleErrors(Vec<StorageError>),
}

/// Result type for storage operations
pub type StorageResult<T> = Result<T, StorageError>;

impl StorageError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            StorageError::ConnectionError(_)
                | StorageError::Timeout
                | StorageError::VersionConflict { .. }
        )
    }
    
    /// Check if error indicates data not found
    pub fn is_not_found(&self) -> bool {
        matches!(self, StorageError::NotFound(_))
    }
    
    /// Check if error indicates conflict
    pub fn is_conflict(&self) -> bool {
        matches!(
            self,
            StorageError::AlreadyExists(_) | StorageError::VersionConflict { .. }
        )
    }
}

/// Convert from serde_json errors
impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}

/// Convert from bincode errors
impl From<bincode::Error> for StorageError {
    fn from(err: bincode::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}