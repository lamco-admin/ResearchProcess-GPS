//! Core entity traits and types

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

use crate::{EntityId, Error, Result, Validatable};

/// Core trait for all ResearchProcess-GPS entities
#[async_trait]
pub trait Entity: Debug + Send + Sync + Serialize + for<'de> Deserialize<'de> + Validatable {
    /// Get the entity's unique identifier
    fn id(&self) -> EntityId;
    
    /// Get the entity type name (e.g., "Theory", "Person", "Evidence")
    fn entity_type(&self) -> &'static str;
    
    /// Get the researcher who created this entity
    fn created_by(&self) -> EntityId;
    
    /// Get the creation timestamp
    fn created_at(&self) -> DateTime<Utc>;
    
    /// Get the researcher who last modified this entity
    fn modified_by(&self) -> EntityId;
    
    /// Get the last modification timestamp
    fn modified_at(&self) -> DateTime<Utc>;
    
    /// Check if this entity is active (not deleted)
    fn is_active(&self) -> bool;
    
    /// Convert to a generic entity representation
    fn as_entity(&self) -> EntityData;
}

/// Trait for entities that can contain other entities
#[async_trait]
pub trait NestableEntity: Entity {
    /// Get child entity IDs
    fn children(&self) -> Vec<EntityId>;
    
    /// Check if this entity can contain a specific entity type
    fn can_contain(&self, entity_type: &str) -> bool;
    
    /// Add a child entity
    async fn add_child(&mut self, child_id: EntityId) -> Result<()>;
    
    /// Remove a child entity
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool>;
}

/// Trait for entities that maintain version history
#[async_trait]
pub trait VersionedEntity: Entity {
    /// Get the current version number
    fn version(&self) -> u32;
    
    /// Get the parent version ID (if this is not the first version)
    fn parent_version(&self) -> Option<EntityId>;
    
    /// Check if this is the latest version
    fn is_latest(&self) -> bool;
    
    /// Create a new version of this entity
    async fn create_version(&self, modified_by: EntityId) -> Result<Self>
    where
        Self: Sized;
}

/// Generic entity data structure for protocol-level operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
    pub id: EntityId,
    pub entity_type: String,
    pub created_by: EntityId,
    pub created_at: DateTime<Utc>,
    pub modified_by: EntityId,
    pub modified_at: DateTime<Utc>,
    pub is_active: bool,
    pub version: Option<u32>,
    pub data: serde_json::Value,
}

/// Entity metadata that all entities must maintain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMetadata {
    pub id: EntityId,
    pub created_by: EntityId,
    pub created_at: DateTime<Utc>,
    pub modified_by: EntityId,
    pub modified_at: DateTime<Utc>,
    pub is_active: bool,
    pub version: u32,
    pub parent_version: Option<EntityId>,
}

impl EntityMetadata {
    /// Create new metadata for a newly created entity
    pub fn new(created_by: EntityId) -> Self {
        let now = Utc::now();
        Self {
            id: EntityId::new(),
            created_by,
            created_at: now,
            modified_by: created_by,
            modified_at: now,
            is_active: true,
            version: 1,
            parent_version: None,
        }
    }
    
    /// Update metadata for a modification
    pub fn update(&mut self, modified_by: EntityId) {
        self.modified_by = modified_by;
        self.modified_at = Utc::now();
    }
    
    /// Create metadata for a new version
    pub fn new_version(&self, modified_by: EntityId) -> Self {
        Self {
            id: EntityId::new(),
            created_by: self.created_by,
            created_at: self.created_at,
            modified_by,
            modified_at: Utc::now(),
            is_active: true,
            version: self.version + 1,
            parent_version: Some(self.id),
        }
    }
}

/// Helper macro to implement common Entity trait methods
#[macro_export]
macro_rules! impl_entity {
    ($type:ty, $entity_type:expr) => {
        impl Entity for $type {
            fn id(&self) -> EntityId {
                self.metadata.id
            }
            
            fn entity_type(&self) -> &'static str {
                $entity_type
            }
            
            fn created_by(&self) -> EntityId {
                self.metadata.created_by
            }
            
            fn created_at(&self) -> DateTime<Utc> {
                self.metadata.created_at
            }
            
            fn modified_by(&self) -> EntityId {
                self.metadata.modified_by
            }
            
            fn modified_at(&self) -> DateTime<Utc> {
                self.metadata.modified_at
            }
            
            fn is_active(&self) -> bool {
                self.metadata.is_active
            }
            
            fn as_entity(&self) -> EntityData {
                EntityData {
                    id: self.metadata.id,
                    entity_type: $entity_type.to_string(),
                    created_by: self.metadata.created_by,
                    created_at: self.metadata.created_at,
                    modified_by: self.metadata.modified_by,
                    modified_at: self.metadata.modified_at,
                    is_active: self.metadata.is_active,
                    version: Some(self.metadata.version),
                    data: serde_json::to_value(self).unwrap_or(serde_json::Value::Null),
                }
            }
        }
    };
}