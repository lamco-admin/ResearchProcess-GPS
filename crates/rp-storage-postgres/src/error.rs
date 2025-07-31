//! PostgreSQL-specific error types

use thiserror::Error;

/// PostgreSQL storage errors
#[derive(Error, Debug)]
pub enum PostgresError {
    /// Database connection error
    #[error("Database connection error: {0}")]
    Connection(#[from] sqlx::Error),
    
    /// Pool error
    #[error("Connection pool error: {0}")]
    Pool(String),
    
    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    /// Query error
    #[error("Query error: {0}")]
    Query(String),
    
    /// Extension not available
    #[error("Extension not available: {0}")]
    ExtensionNotAvailable(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Vector operation error
    #[error("Vector operation error: {0}")]
    VectorOperation(String),
    
    /// Graph operation error
    #[error("Graph operation error: {0}")]
    GraphOperation(String),
}

/// Result type for PostgreSQL operations
pub type PostgresResult<T> = Result<T, PostgresError>;

/// Convert PostgreSQL errors to storage errors
impl From<PostgresError> for rp_storage::StorageError {
    fn from(err: PostgresError) -> Self {
        match err {
            PostgresError::Connection(e) => {
                // Check for specific error types
                if let Some(db_err) = e.as_database_error() {
                    match db_err.code().as_ref() {
                        "23505" => { // unique_violation
                            // Try to extract entity ID from error message
                            rp_storage::StorageError::AlreadyExists(uuid::Uuid::nil())
                        }
                        "23503" => { // foreign_key_violation
                            rp_storage::StorageError::NotFound(uuid::Uuid::nil())
                        }
                        _ => rp_storage::StorageError::BackendError(e.to_string()),
                    }
                } else {
                    rp_storage::StorageError::ConnectionError(e.to_string())
                }
            }
            PostgresError::Pool(e) => rp_storage::StorageError::ConnectionError(e),
            PostgresError::Transaction(e) => rp_storage::StorageError::TransactionError(e),
            PostgresError::Query(e) => rp_storage::StorageError::QueryError(e),
            PostgresError::ExtensionNotAvailable(e) => {
                rp_storage::StorageError::NotSupported(format!("Extension not available: {}", e))
            }
            PostgresError::Config(e) => rp_storage::StorageError::ConfigError(e),
            PostgresError::Serialization(e) => rp_storage::StorageError::SerializationError(e),
            PostgresError::VectorOperation(e) => {
                rp_storage::StorageError::BackendError(format!("Vector operation failed: {}", e))
            }
            PostgresError::GraphOperation(e) => {
                rp_storage::StorageError::BackendError(format!("Graph operation failed: {}", e))
            }
        }
    }
}

impl From<serde_json::Error> for PostgresError {
    fn from(err: serde_json::Error) -> Self {
        PostgresError::Serialization(err.to_string())
    }
}

impl From<bincode::Error> for PostgresError {
    fn from(err: bincode::Error) -> Self {
        PostgresError::Serialization(err.to_string())
    }
}