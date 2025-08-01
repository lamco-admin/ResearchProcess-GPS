//! ResearchProcess-GPS Module System
//! 
//! Provides a secure, extensible framework for custom research tools.
//! Supports both native Rust modules and sandboxed WASM modules.

pub mod module;
pub mod manifest;
pub mod registry;
pub mod loader;
pub mod context;
pub mod capabilities;
pub mod resource_limits;
pub mod communication;
pub mod error;

#[cfg(test)]
mod tests;

pub use module::{ResearchModule, ModuleMetadata};
pub use manifest::ModuleManifest;
pub use registry::ModuleRegistry;
pub use loader::{ModuleLoader, ModuleInstance};
pub use context::ModuleContext;
pub use capabilities::{ModuleCapabilities, Capability};
pub use resource_limits::{ResourceLimits, ResourceLimiter};
pub use communication::{ModuleMessage, ModuleChannel};
pub use error::{ModuleError, Result};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        ResearchModule, ModuleMetadata, ModuleContext,
        ModuleCapabilities, ModuleMessage, ModuleError, Result,
    };
}