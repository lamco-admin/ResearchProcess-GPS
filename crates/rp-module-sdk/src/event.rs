//! Event handling utilities

use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Trait for event handlers
#[cfg_attr(feature = "native", async_trait::async_trait)]
pub trait EventHandler: Send + Sync {
    /// Handle an event
    #[cfg(feature = "native")]
    async fn handle(&mut self, event_data: Value) -> Result<()>;
    
    /// Handle an event (sync version for WASM)
    #[cfg(feature = "wasm")]
    fn handle(&mut self, event_data: Value) -> Result<()>;
}

/// Event router for dispatching events to handlers
pub struct EventRouter {
    handlers: HashMap<String, Vec<Box<dyn EventHandler>>>,
}

impl EventRouter {
    /// Create a new event router
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Register an event handler
    pub fn register<H: EventHandler + 'static>(&mut self, event_type: impl Into<String>, handler: H) {
        self.handlers
            .entry(event_type.into())
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
    }
    
    /// Handle an event
    #[cfg(feature = "native")]
    pub async fn handle(&mut self, event_type: &str, event_data: Value) -> Result<()> {
        if let Some(handlers) = self.handlers.get_mut(event_type) {
            for handler in handlers {
                handler.handle(event_data.clone()).await?;
            }
        }
        Ok(())
    }
    
    /// Handle an event (sync version for WASM)
    #[cfg(feature = "wasm")]
    pub fn handle(&mut self, event_type: &str, event_data: Value) -> Result<()> {
        if let Some(handlers) = self.handlers.get_mut(event_type) {
            for handler in handlers {
                handler.handle(event_data.clone())?;
            }
        }
        Ok(())
    }
    
    /// Get list of event types this router handles
    pub fn event_types(&self) -> Vec<&str> {
        self.handlers.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for EventRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event type
    pub event_type: String,
    /// Event data
    pub data: Value,
    /// Timestamp (Unix timestamp in seconds)
    pub timestamp: u64,
    /// Source module ID (if from another module)
    pub source: Option<String>,
}

/// Common event types in the ResearchProcess GPS system
pub mod event_types {
    /// Entity created event
    pub const ENTITY_CREATED: &str = "entity.created";
    /// Entity updated event
    pub const ENTITY_UPDATED: &str = "entity.updated";
    /// Entity deleted event
    pub const ENTITY_DELETED: &str = "entity.deleted";
    
    /// Research session started
    pub const SESSION_STARTED: &str = "session.started";
    /// Research session ended
    pub const SESSION_ENDED: &str = "session.ended";
    
    /// Analysis completed
    pub const ANALYSIS_COMPLETED: &str = "analysis.completed";
    
    /// Module loaded
    pub const MODULE_LOADED: &str = "module.loaded";
    /// Module unloaded
    pub const MODULE_UNLOADED: &str = "module.unloaded";
}