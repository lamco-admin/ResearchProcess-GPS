//! Module system error types

use thiserror::Error;

/// Module system errors
#[derive(Debug, Error)]
pub enum ModuleError {
    /// Module not found
    #[error("Module not found: {0}")]
    NotFound(String),
    
    /// Module loading failed
    #[error("Failed to load module: {0}")]
    LoadError(String),
    
    /// Invalid module manifest
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
    
    /// Module initialization failed
    #[error("Module initialization failed: {0}")]
    InitializationError(String),
    
    /// Module execution error
    #[error("Module execution error: {0}")]
    ExecutionError(String),
    
    /// Module shutdown error
    #[error("Module shutdown failed: {0}")]
    ShutdownError(String),
    
    /// Resource limit exceeded
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    
    /// Capability not granted
    #[error("Capability not granted: {0}")]
    CapabilityDenied(String),
    
    /// Communication error
    #[error("Communication error: {0}")]
    CommunicationError(String),
    
    /// WASM runtime error
    #[error("WASM runtime error: {0}")]
    WasmError(String),
    
    /// Native module error
    #[error("Native module error: {0}")]
    NativeError(String),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// Other errors
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Module system result type
pub type Result<T> = std::result::Result<T, ModuleError>;