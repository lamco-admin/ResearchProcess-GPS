//! Core module trait and metadata

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{ModuleContext, ModuleMessage, Result};

/// Core trait that all research modules must implement
#[async_trait]
pub trait ResearchModule: Send + Sync {
    /// Get module metadata
    fn metadata(&self) -> &ModuleMetadata;
    
    /// Initialize module with host context
    async fn initialize(&mut self, context: ModuleContext) -> Result<()>;
    
    /// Handle incoming messages from the host
    async fn handle_message(&mut self, message: ModuleMessage) -> Result<()>;
    
    /// Execute module-specific commands
    async fn execute_command(
        &mut self, 
        command: &str, 
        args: serde_json::Value
    ) -> Result<serde_json::Value>;
    
    /// Cleanup before unload
    async fn shutdown(&mut self) -> Result<()>;
}

/// Module metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    /// Module unique identifier
    pub id: Uuid,
    
    /// Module name
    pub name: String,
    
    /// Module version
    pub version: String,
    
    /// Module description
    pub description: String,
    
    /// Module author
    pub author: String,
    
    /// Module license
    pub license: String,
    
    /// Module type (native or wasm)
    pub module_type: ModuleType,
    
    /// When the module was loaded
    pub loaded_at: DateTime<Utc>,
    
    /// Module status
    pub status: ModuleStatus,
}

/// Module type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleType {
    /// Native Rust module (dynamic library)
    Native,
    
    /// WebAssembly module (sandboxed)
    Wasm,
}

/// Module status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleStatus {
    /// Module is loaded but not initialized
    Loaded,
    
    /// Module is initializing
    Initializing,
    
    /// Module is ready and running
    Running,
    
    /// Module is shutting down
    ShuttingDown,
    
    /// Module has stopped
    Stopped,
    
    /// Module encountered an error
    Error,
}