//! Module capability system

use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};

/// Module capabilities define what a module is allowed to do
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCapabilities {
    /// Entity types the module can read
    pub entity_read: HashSet<String>,
    
    /// Entity types the module can write
    pub entity_write: HashSet<String>,
    
    /// Entity types the module can create
    pub entity_create: HashSet<String>,
    
    /// Entity types the module can delete
    pub entity_delete: HashSet<String>,
    
    /// Event types the module can subscribe to
    pub event_subscribe: HashSet<String>,
    
    /// Event types the module can emit
    pub event_emit: HashSet<String>,
    
    /// Whether the module can access network
    pub network_access: bool,
    
    /// Whether the module can access file system
    pub filesystem_access: bool,
    
    /// Custom capabilities
    pub custom: HashSet<String>,
    
    /// Entity permissions map (for more fine-grained control)
    pub entity_permissions: HashMap<String, EntityPermission>,
}

impl Default for ModuleCapabilities {
    fn default() -> Self {
        Self::none()
    }
}

/// Entity-specific permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityPermission {
    pub read: bool,
    pub write: bool,
    pub create: bool,
    pub delete: bool,
}

impl EntityPermission {
    /// Create permission with all rights
    pub fn all() -> Self {
        Self {
            read: true,
            write: true,
            create: true,
            delete: true,
        }
    }
    
    /// Create permission with no rights
    pub fn none() -> Self {
        Self {
            read: false,
            write: false,
            create: false,
            delete: false,
        }
    }
    
    /// Create read-only permission
    pub fn read_only() -> Self {
        Self {
            read: true,
            write: false,
            create: false,
            delete: false,
        }
    }
}

impl ModuleCapabilities {
    /// Create empty capabilities (no permissions)
    pub fn none() -> Self {
        Self {
            entity_read: HashSet::new(),
            entity_write: HashSet::new(),
            entity_create: HashSet::new(),
            entity_delete: HashSet::new(),
            event_subscribe: HashSet::new(),
            event_emit: HashSet::new(),
            network_access: false,
            filesystem_access: false,
            custom: HashSet::new(),
            entity_permissions: HashMap::new(),
        }
    }
    
    /// Create full capabilities (all permissions) - use with caution
    pub fn full() -> Self {
        let mut caps = Self::none();
        caps.entity_read.insert("*".to_string());
        caps.entity_write.insert("*".to_string());
        caps.entity_create.insert("*".to_string());
        caps.entity_delete.insert("*".to_string());
        caps.event_subscribe.insert("*".to_string());
        caps.event_emit.insert("*".to_string());
        caps.network_access = true;
        caps.filesystem_access = true;
        caps
    }
    
    /// Check if a specific capability is granted
    pub fn has_capability(&self, capability: &Capability) -> bool {
        match capability {
            Capability::EntityRead(entity_type) => {
                self.entity_read.contains("*") || self.entity_read.contains(entity_type)
            }
            Capability::EntityWrite(entity_type) => {
                self.entity_write.contains("*") || self.entity_write.contains(entity_type)
            }
            Capability::EntityCreate(entity_type) => {
                self.entity_create.contains("*") || self.entity_create.contains(entity_type)
            }
            Capability::EntityDelete(entity_type) => {
                self.entity_delete.contains("*") || self.entity_delete.contains(entity_type)
            }
            Capability::EventSubscribe(event_type) => {
                self.event_subscribe.contains("*") || self.event_subscribe.contains(event_type)
            }
            Capability::EventEmit(event_type) => {
                self.event_emit.contains("*") || self.event_emit.contains(event_type)
            }
            Capability::NetworkAccess => self.network_access,
            Capability::FilesystemAccess => self.filesystem_access,
            Capability::Custom(name) => self.custom.contains(name),
        }
    }
    
    /// Check if module can read a specific entity type
    pub fn can_read_entity(&self, entity_type: &str) -> bool {
        self.entity_read.contains("*") || self.entity_read.contains(entity_type)
    }
    
    /// Check if module can write a specific entity type
    pub fn can_write_entity(&self, entity_type: &str) -> bool {
        self.entity_write.contains("*") || self.entity_write.contains(entity_type)
    }
    
    /// Check if module can create a specific entity type
    pub fn can_create_entity(&self, entity_type: &str) -> bool {
        self.entity_create.contains("*") || self.entity_create.contains(entity_type)
    }
    
    /// Check if module can delete a specific entity type
    pub fn can_delete_entity(&self, entity_type: &str) -> bool {
        self.entity_delete.contains("*") || self.entity_delete.contains(entity_type)
    }
    
    /// Check if module can subscribe to a specific event type
    pub fn can_subscribe_event(&self, event_type: &str) -> bool {
        self.event_subscribe.contains("*") || self.event_subscribe.contains(event_type)
    }
    
    /// Check if module can emit a specific event type
    pub fn can_emit_event(&self, event_type: &str) -> bool {
        self.event_emit.contains("*") || self.event_emit.contains(event_type)
    }
}

/// Individual capability
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Read entities of a specific type
    EntityRead(String),
    
    /// Write entities of a specific type
    EntityWrite(String),
    
    /// Create entities of a specific type
    EntityCreate(String),
    
    /// Delete entities of a specific type
    EntityDelete(String),
    
    /// Subscribe to events of a specific type
    EventSubscribe(String),
    
    /// Emit events of a specific type
    EventEmit(String),
    
    /// Access network resources
    NetworkAccess,
    
    /// Access filesystem
    FilesystemAccess,
    
    /// Custom capability
    Custom(String),
}