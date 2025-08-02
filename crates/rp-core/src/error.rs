//! Core error types for ResearchProcess-GPS

use thiserror::Error;

use meta_model_core::layer1::EntityId;

/// Core error type for ResearchProcess-GPS
#[derive(Error, Debug)]
pub enum Error {
    /// Entity not found
    #[error("Entity not found: {0}")]
    EntityNotFound(EntityId),
    
    /// Invalid entity type
    #[error("Invalid entity type: expected {expected}, got {actual}")]
    InvalidEntityType { expected: String, actual: String },
    
    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// State transition error
    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Module error
    #[error("Module error: {0}")]
    ModuleError(String),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    /// Capacity exceeded
    #[error("Capacity exceeded: {0}")]
    CapacityExceeded(String),
    
    /// Conflict error (e.g., concurrent modification)
    #[error("Conflict: {0}")]
    Conflict(String),
    
    /// Operation timeout
    #[error("Operation timeout: {0}")]
    Timeout(String),
    
    /// Not implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    /// Other error
    #[error("{0}")]
    Other(String),
}

/// Convenience Result type
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a storage error from another error
    pub fn storage<E: std::error::Error>(err: E) -> Self {
        Self::StorageError(err.to_string())
    }
    
    /// Create a serialization error from another error
    pub fn serialization<E: std::error::Error>(err: E) -> Self {
        Self::SerializationError(err.to_string())
    }
    
    /// Create a network error from another error
    pub fn network<E: std::error::Error>(err: E) -> Self {
        Self::NetworkError(err.to_string())
    }
    
    /// Create a module error from another error
    pub fn module<E: std::error::Error>(err: E) -> Self {
        Self::ModuleError(err.to_string())
    }
}

/// Error context trait for adding context to errors
pub trait ErrorContext<T> {
    /// Add context to an error
    fn context(self, msg: &str) -> Result<T>;
    
    /// Add context with a closure
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: std::error::Error,
{
    fn context(self, msg: &str) -> Result<T> {
        self.map_err(|e| Error::Other(format!("{}: {}", msg, e)))
    }
    
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| Error::Other(format!("{}: {}", f(), e)))
    }
}