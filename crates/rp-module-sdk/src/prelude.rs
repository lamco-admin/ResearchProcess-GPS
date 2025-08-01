//! Module SDK prelude - commonly used types and traits

pub use crate::{
    Module, ModuleContext, ModuleInfo, ModuleInfoBuilder,
    ModuleError, Result,
    CommandHandler, CommandRouter, Command, CommandResponse,
    EventHandler, EventRouter, Event,
    LifecycleEvent, ModuleState, ResourceUsage,
    init_logging,
    // Message builders
    messages::{
        MessageType, ResponseStatus, ResponseBuilder,
        QueryBuilder, MutationBuilder, MutationOperation,
        parse_message, parse_command,
    },
    // FFI utilities
    ffi::{FfiModule, ModuleHolder, c_str_to_string, string_to_c_str},
};

pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, Value};
pub use uuid::Uuid;
pub use log::{debug, error, info, trace, warn};

// Re-export module types
pub use rp_modules::{
    communication::ModuleMessage,
    capabilities::Capability as ModuleCapability,
    resource_limits::ResourceLimits,
};

// Re-export async-trait for native modules
#[cfg(feature = "native")]
pub use async_trait::async_trait;

// Common result types
/// Result type for command execution
pub type CommandResult = Result<Value>;
/// Result type for event handling
pub type EventResult = Result<()>;