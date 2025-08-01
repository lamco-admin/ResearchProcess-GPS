//! Error types for the module SDK

use thiserror::Error;

/// Result type alias for module operations
pub type Result<T> = std::result::Result<T, ModuleError>;

/// Module-specific errors
#[derive(Error, Debug)]
pub enum ModuleError {
    /// Module initialization failed
    #[error("Initialization failed: {0}")]
    InitializationError(String),
    
    /// Command execution failed
    #[error("Command execution failed: {0}")]
    CommandError(String),
    
    /// Event handling failed
    #[error("Event handling failed: {0}")]
    EventError(String),
    
    /// Communication error with host
    #[error("Communication error: {0}")]
    CommunicationError(String),
    
    /// Resource limit exceeded
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),
    
    /// Permission denied for operation
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Other errors
    #[error("{0}")]
    Other(String),
}