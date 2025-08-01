// Storage errors

use thiserror::Error;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    #[error("Query error: {0}")]
    Query(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}