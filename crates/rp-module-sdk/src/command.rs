//! Command handling utilities

use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Trait for command handlers
#[cfg_attr(feature = "native", async_trait::async_trait)]
pub trait CommandHandler: Send + Sync {
    /// Handle a command
    #[cfg(feature = "native")]
    async fn handle(&mut self, args: Value) -> Result<Value>;
    
    /// Handle a command (sync version for WASM)
    #[cfg(feature = "wasm")]
    fn handle(&mut self, args: Value) -> Result<Value>;
}

/// Command router for dispatching commands to handlers
pub struct CommandRouter {
    handlers: HashMap<String, Box<dyn CommandHandler>>,
}

impl CommandRouter {
    /// Create a new command router
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Register a command handler
    pub fn register<H: CommandHandler + 'static>(&mut self, command: impl Into<String>, handler: H) {
        self.handlers.insert(command.into(), Box::new(handler));
    }
    
    /// Execute a command
    #[cfg(feature = "native")]
    pub async fn execute(&mut self, command: &str, args: Value) -> Result<Value> {
        match self.handlers.get_mut(command) {
            Some(handler) => handler.handle(args).await,
            None => Err(crate::error::ModuleError::CommandError(
                format!("Unknown command: {}", command)
            )),
        }
    }
    
    /// Execute a command (sync version for WASM)
    #[cfg(feature = "wasm")]
    pub fn execute(&mut self, command: &str, args: Value) -> Result<Value> {
        match self.handlers.get_mut(command) {
            Some(handler) => handler.handle(args),
            None => Err(crate::error::ModuleError::CommandError(
                format!("Unknown command: {}", command)
            )),
        }
    }
    
    /// Get list of registered commands
    pub fn commands(&self) -> Vec<&str> {
        self.handlers.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Command request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Command name
    pub name: String,
    /// Command arguments
    pub args: Value,
}

/// Command response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    /// Success flag
    pub success: bool,
    /// Result data (if successful)
    pub result: Option<Value>,
    /// Error message (if failed)
    pub error: Option<String>,
}

impl CommandResponse {
    /// Create a successful response
    pub fn success(result: Value) -> Self {
        Self {
            success: true,
            result: Some(result),
            error: None,
        }
    }
    
    /// Create an error response
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            result: None,
            error: Some(message.into()),
        }
    }
}