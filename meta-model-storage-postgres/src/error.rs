//! PostgreSQL-specific errors

use thiserror::Error;
use meta_model_storage::StorageError;

pub type PostgresResult<T> = Result<T, PostgresError>;

#[derive(Error, Debug)]
pub enum PostgresError {
    #[error("Database connection error: {0}")]
    Connection(#[from] sqlx::Error),
    
    #[error("Query error: {0}")]
    Query(String),
    
    #[error("Migration error: {0}")]
    Migration(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Constraint violation: {0}")]
    Constraint(String),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl From<PostgresError> for StorageError {
    fn from(err: PostgresError) -> Self {
        match err {
            PostgresError::Connection(e) => StorageError::Backend(e.to_string()),
            PostgresError::Query(e) => StorageError::InvalidQuery(e),
            PostgresError::Migration(e) => StorageError::Backend(format!("Migration failed: {}", e)),
            PostgresError::Serialization(e) => StorageError::Serialization(e),
            PostgresError::Constraint(e) => StorageError::ConstraintViolation(e),
            PostgresError::Other(e) => StorageError::Backend(e),
        }
    }
}