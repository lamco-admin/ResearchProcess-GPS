//! ResearchProcess GPS Module SDK
//! 
//! This SDK provides utilities and abstractions for building modules that extend
//! the ResearchProcess GPS system. It supports both native (Rust) modules and
//! WASM modules.
//!
//! # Features
//! 
//! - **Native modules**: Full async/await support with tokio runtime
//! - **WASM modules**: Sandboxed execution with C-style exports
//! - **Type-safe communication**: Strongly typed messages and commands
//! - **Resource management**: Built-in resource limiting
//! - **Event handling**: Subscribe to and emit system events

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

pub mod error;
pub mod module;
pub mod command;
pub mod event;
pub mod messages;
pub mod ffi;

#[cfg(feature = "native")]
pub mod native;

#[cfg(feature = "wasm")]
pub mod wasm;

pub mod prelude;

// Re-export commonly used types
pub use error::{ModuleError, Result};
pub use module::{Module, ModuleContext, ModuleInfo, ModuleInfoBuilder};
pub use command::{Command, CommandHandler, CommandRouter, CommandResponse};
pub use event::{Event, EventHandler, EventRouter};

// The FFI macro is exported by the macro_rules! itself

// Re-export module types
pub use rp_modules::communication::ModuleMessage;
pub use rp_modules::capabilities::Capability as ModuleCapability;
pub use rp_modules::resource_limits::ResourceLimits;

/// Module SDK version
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the module SDK logging
pub fn init_logging() {
    // For native modules, initialize env_logger if not already initialized
    #[cfg(feature = "native")]
    {
        let _ = env_logger::try_init();
    }
}

/// Module lifecycle hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleEvent {
    /// Module is being initialized
    Initializing,
    /// Module has been initialized and is ready
    Ready,
    /// Module is processing a command
    ProcessingCommand(String),
    /// Module has completed processing a command
    CommandComplete(String),
    /// Module is shutting down
    ShuttingDown,
    /// Module has shut down
    Shutdown,
}

/// Module state that can be queried
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleState {
    /// Current lifecycle state
    pub lifecycle: LifecycleEvent,
    /// Number of commands processed
    pub commands_processed: u64,
    /// Number of events received
    pub events_received: u64,
    /// Current resource usage
    pub resource_usage: ResourceUsage,
}

/// Resource usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Memory used in bytes
    pub memory_bytes: u64,
    /// CPU time used in milliseconds
    pub cpu_time_ms: u64,
    /// Number of active file handles
    pub file_handles: u32,
    /// Number of active network connections
    pub network_connections: u32,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            memory_bytes: 0,
            cpu_time_ms: 0,
            file_handles: 0,
            network_connections: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdk_version() {
        assert!(!SDK_VERSION.is_empty());
    }
}
