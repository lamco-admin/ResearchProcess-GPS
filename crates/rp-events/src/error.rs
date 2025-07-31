use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Concurrency conflict: expected version {expected} but current is {current}")]
    ConcurrencyConflict { expected: i64, current: i64 },
    
    #[error("Event not found: {id}")]
    EventNotFound { id: Uuid },
    
    #[error("Aggregate not found: {id}")]
    AggregateNotFound { id: Uuid },
    
    #[error("Invalid event data: {message}")]
    InvalidEventData { message: String },
    
    #[error("Projection error: {name} - {message}")]
    ProjectionError { name: String, message: String },
    
    #[error("Snapshot error: {message}")]
    SnapshotError { message: String },
    
    #[error("Storage error: {0}")]
    StorageError(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, EventError>;