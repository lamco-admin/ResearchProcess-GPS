// Event system for meta-model audit trails and integration

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::common::AgentId;
use crate::layer1::EntityId;
use crate::layer2::ProcessId;
use crate::layer3::WorkspaceId;
use std::collections::HashMap;

/// Event that can occur in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique event ID
    pub id: uuid::Uuid,
    /// Event type
    pub event_type: String,
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
    /// Who triggered the event
    pub triggered_by: AgentId,
    /// Target of the event
    pub target: EventTarget,
    /// Event data
    pub data: serde_json::Value,
    /// Event metadata
    pub metadata: HashMap<String, String>,
}

/// Target of an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventTarget {
    Entity(EntityId),
    Process(ProcessId),
    Workspace(WorkspaceId),
    System,
    Multiple(Vec<EventTarget>),
}

impl Event {
    /// Create a new event
    pub fn new(
        event_type: impl Into<String>,
        triggered_by: AgentId,
        target: EventTarget,
        data: serde_json::Value,
    ) -> Self {
        Event {
            id: uuid::Uuid::new_v4(),
            event_type: event_type.into(),
            timestamp: Utc::now(),
            triggered_by,
            target,
            data,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the event
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Common event types
pub mod event_types {
    // Entity events
    pub const ENTITY_CREATED: &str = "entity.created";
    pub const ENTITY_UPDATED: &str = "entity.updated";
    pub const ENTITY_STATE_CHANGED: &str = "entity.state_changed";
    pub const ENTITY_DELETED: &str = "entity.deleted";
    pub const ENTITY_PROPERTY_CHANGED: &str = "entity.property_changed";
    pub const ENTITY_RELATIONSHIP_ADDED: &str = "entity.relationship_added";
    pub const ENTITY_RELATIONSHIP_REMOVED: &str = "entity.relationship_removed";
    pub const ENTITY_CONTEXT_ADDED: &str = "entity.context_added";
    pub const ENTITY_CONTEXT_REMOVED: &str = "entity.context_removed";

    // Process events
    pub const PROCESS_CREATED: &str = "process.created";
    pub const PROCESS_STARTED: &str = "process.started";
    pub const PROCESS_STATE_CHANGED: &str = "process.state_changed";
    pub const PROCESS_COMPLETED: &str = "process.completed";
    pub const PROCESS_FAILED: &str = "process.failed";
    pub const PROCESS_CANCELLED: &str = "process.cancelled";
    pub const PROCESS_ACTIVITY_ADDED: &str = "process.activity_added";
    pub const PROCESS_ACTIVITY_COMPLETED: &str = "process.activity_completed";

    // Workspace events
    pub const WORKSPACE_CREATED: &str = "workspace.created";
    pub const WORKSPACE_ACTIVATED: &str = "workspace.activated";
    pub const WORKSPACE_SUSPENDED: &str = "workspace.suspended";
    pub const WORKSPACE_ARCHIVED: &str = "workspace.archived";
    pub const WORKSPACE_ITEM_ADDED: &str = "workspace.item_added";
    pub const WORKSPACE_ITEM_REMOVED: &str = "workspace.item_removed";
    pub const WORKSPACE_CONFIGURATION_CHANGED: &str = "workspace.configuration_changed";

    // Validation events
    pub const VALIDATION_FAILED: &str = "validation.failed";
    pub const VALIDATION_WARNING: &str = "validation.warning";

    // System events
    pub const SYSTEM_STARTUP: &str = "system.startup";
    pub const SYSTEM_SHUTDOWN: &str = "system.shutdown";
    pub const SYSTEM_ERROR: &str = "system.error";
}

/// Event handler trait
pub trait EventHandler: Send + Sync {
    /// Handle an event
    fn handle(&mut self, event: &Event) -> Result<(), String>;

    /// Check if this handler is interested in an event type
    fn handles_event_type(&self, event_type: &str) -> bool;
}

/// Event store trait for persistence
pub trait EventStore: Send + Sync {
    /// Store an event
    fn store(&mut self, event: Event) -> Result<(), String>;

    /// Query events by target
    fn query_by_target(&self, target: &EventTarget) -> Result<Vec<Event>, String>;

    /// Query events by type
    fn query_by_type(&self, event_type: &str) -> Result<Vec<Event>, String>;

    /// Query events by time range
    fn query_by_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Event>, String>;

    /// Query events by agent
    fn query_by_agent(&self, agent_id: AgentId) -> Result<Vec<Event>, String>;
}

/// Event bus for distributing events
pub struct EventBus {
    handlers: Vec<Box<dyn EventHandler>>,
    stores: Vec<Box<dyn EventStore>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            handlers: Vec::new(),
            stores: Vec::new(),
        }
    }

    /// Register an event handler
    pub fn register_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    /// Register an event store
    pub fn register_store(&mut self, store: Box<dyn EventStore>) {
        self.stores.push(store);
    }

    /// Publish an event
    pub fn publish(&mut self, event: Event) -> Result<(), String> {
        // Store the event
        for store in &mut self.stores {
            store.store(event.clone())?;
        }

        // Distribute to handlers
        for handler in &mut self.handlers {
            if handler.handles_event_type(&event.event_type) {
                handler.handle(&event)?;
            }
        }

        Ok(())
    }
}

/// In-memory event store for testing
pub struct InMemoryEventStore {
    events: Vec<Event>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        InMemoryEventStore {
            events: Vec::new(),
        }
    }
}

impl EventStore for InMemoryEventStore {
    fn store(&mut self, event: Event) -> Result<(), String> {
        self.events.push(event);
        Ok(())
    }

    fn query_by_target(&self, target: &EventTarget) -> Result<Vec<Event>, String> {
        Ok(self.events
            .iter()
            .filter(|e| match (&e.target, target) {
                (EventTarget::Entity(id1), EventTarget::Entity(id2)) => id1 == id2,
                (EventTarget::Process(id1), EventTarget::Process(id2)) => id1 == id2,
                (EventTarget::Workspace(id1), EventTarget::Workspace(id2)) => id1 == id2,
                (EventTarget::System, EventTarget::System) => true,
                _ => false,
            })
            .cloned()
            .collect())
    }

    fn query_by_type(&self, event_type: &str) -> Result<Vec<Event>, String> {
        Ok(self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .cloned()
            .collect())
    }

    fn query_by_time_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<Event>, String> {
        Ok(self.events
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .cloned()
            .collect())
    }

    fn query_by_agent(&self, agent_id: AgentId) -> Result<Vec<Event>, String> {
        Ok(self.events
            .iter()
            .filter(|e| e.triggered_by == agent_id)
            .cloned()
            .collect())
    }
}

/// Audit trail builder for common operations
pub struct AuditTrail {
    events: Vec<Event>,
}

impl AuditTrail {
    pub fn new() -> Self {
        AuditTrail {
            events: Vec::new(),
        }
    }

    /// Record entity creation
    pub fn entity_created(&mut self, entity_id: EntityId, entity_type: &str, agent: AgentId) {
        let event = Event::new(
            event_types::ENTITY_CREATED,
            agent,
            EventTarget::Entity(entity_id),
            serde_json::json!({
                "entity_type": entity_type,
            }),
        );
        self.events.push(event);
    }

    /// Record entity state change
    pub fn entity_state_changed(
        &mut self,
        entity_id: EntityId,
        old_state: &str,
        new_state: &str,
        agent: AgentId,
        reason: Option<String>,
    ) {
        let event = Event::new(
            event_types::ENTITY_STATE_CHANGED,
            agent,
            EventTarget::Entity(entity_id),
            serde_json::json!({
                "old_state": old_state,
                "new_state": new_state,
                "reason": reason,
            }),
        );
        self.events.push(event);
    }

    /// Record process completion
    pub fn process_completed(&mut self, process_id: ProcessId, agent: AgentId, results: serde_json::Value) {
        let event = Event::new(
            event_types::PROCESS_COMPLETED,
            agent,
            EventTarget::Process(process_id),
            serde_json::json!({
                "results": results,
                "completed_at": Utc::now(),
            }),
        );
        self.events.push(event);
    }

    /// Get all events
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    /// Publish all events to an event bus
    pub fn publish_to(&mut self, bus: &mut EventBus) -> Result<(), String> {
        for event in self.events.drain(..) {
            bus.publish(event)?;
        }
        Ok(())
    }
}