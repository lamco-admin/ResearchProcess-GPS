//! Core module trait and types

use crate::error::Result;
use crate::ModuleState;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Core trait that all modules must implement
#[cfg_attr(feature = "native", async_trait::async_trait)]
pub trait Module: Send + Sync {
    /// Get module information
    fn info(&self) -> ModuleInfo;
    
    /// Initialize the module with given context
    #[cfg(feature = "native")]
    async fn initialize(&mut self, context: ModuleContext) -> Result<()>;
    
    /// Initialize the module with given context (sync version for WASM)
    #[cfg(feature = "wasm")]
    fn initialize(&mut self, context: ModuleContext) -> Result<()>;
    
    /// Execute a command
    #[cfg(feature = "native")]
    async fn execute_command(&mut self, command: &str, args: Value) -> Result<Value>;
    
    /// Execute a command (sync version for WASM)
    #[cfg(feature = "wasm")]
    fn execute_command(&mut self, command: &str, args: Value) -> Result<Value>;
    
    /// Handle an event
    #[cfg(feature = "native")]
    async fn handle_event(&mut self, event_type: &str, event_data: Value) -> Result<()>;
    
    /// Handle an event (sync version for WASM)
    #[cfg(feature = "wasm")]
    fn handle_event(&mut self, event_type: &str, event_data: Value) -> Result<()>;
    
    /// Get current module state
    fn get_state(&self) -> ModuleState;
    
    /// Shutdown the module
    #[cfg(feature = "native")]
    async fn shutdown(&mut self) -> Result<()>;
    
    /// Shutdown the module (sync version for WASM)
    #[cfg(feature = "wasm")]
    fn shutdown(&mut self) -> Result<()>;
}

/// Module metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Module name
    pub name: String,
    /// Module version
    pub version: String,
    /// Module description
    pub description: String,
    /// Module author
    pub author: String,
    /// Module capabilities
    pub capabilities: Vec<String>,
    /// Supported commands
    pub commands: Vec<CommandInfo>,
    /// Events this module can emit
    pub emits_events: Vec<String>,
    /// Events this module listens to
    pub listens_to_events: Vec<String>,
}

/// Information about a command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    /// Command name
    pub name: String,
    /// Command description
    pub description: String,
    /// Expected argument structure (JSON schema or description)
    pub args_schema: Value,
}

/// Module execution context
#[derive(Debug, Clone)]
pub struct ModuleContext {
    /// Module instance ID
    pub instance_id: Uuid,
    /// Actor ID (who is running this module)
    pub actor_id: Uuid,
    /// Module configuration
    pub config: Value,
    /// Resource limits
    pub resource_limits: rp_modules::resource_limits::ResourceLimits,
    /// Granted capabilities
    pub capabilities: Vec<rp_modules::capabilities::Capability>,
}

/// Builder for ModuleInfo
pub struct ModuleInfoBuilder {
    name: String,
    version: String,
    description: String,
    author: String,
    capabilities: Vec<String>,
    commands: Vec<CommandInfo>,
    emits_events: Vec<String>,
    listens_to_events: Vec<String>,
}

impl ModuleInfoBuilder {
    /// Create a new builder with required fields
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: String::new(),
            author: String::new(),
            capabilities: Vec::new(),
            commands: Vec::new(),
            emits_events: Vec::new(),
            listens_to_events: Vec::new(),
        }
    }
    
    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    /// Set author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }
    
    /// Add a capability
    pub fn capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities.push(cap.into());
        self
    }
    
    /// Add a command
    pub fn command(mut self, name: impl Into<String>, desc: impl Into<String>, schema: Value) -> Self {
        self.commands.push(CommandInfo {
            name: name.into(),
            description: desc.into(),
            args_schema: schema,
        });
        self
    }
    
    /// Add an event this module emits
    pub fn emits_event(mut self, event: impl Into<String>) -> Self {
        self.emits_events.push(event.into());
        self
    }
    
    /// Add an event this module listens to
    pub fn listens_to_event(mut self, event: impl Into<String>) -> Self {
        self.listens_to_events.push(event.into());
        self
    }
    
    /// Build the ModuleInfo
    pub fn build(self) -> ModuleInfo {
        ModuleInfo {
            name: self.name,
            version: self.version,
            description: self.description,
            author: self.author,
            capabilities: self.capabilities,
            commands: self.commands,
            emits_events: self.emits_events,
            listens_to_events: self.listens_to_events,
        }
    }
}