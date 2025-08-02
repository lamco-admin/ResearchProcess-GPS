// Entity - The fundamental unit of the meta-model
// Can represent ANY genealogical concept

use super::{EntityId, RelationshipId, Context, PropertyGraph};
use crate::common::MetaInfo;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Unique identifier
    pub id: EntityId,

    /// Open-ended entity type (not an enum!)
    /// Examples: "Person", "Evidence", "Theory", "GEDCOM.INDI", "Identity.Hypothesis"
    pub entity_type: String,

    /// Open-ended state
    /// Examples: "Active", "Hypothesis", "Proven", "Disproven", "Superseded"
    pub state: String,

    /// All properties as a graph - infinitely extensible
    pub properties: PropertyGraph,

    /// Relationships this entity participates in
    pub relationships: Vec<RelationshipId>,

    /// Contexts that scope this entity
    pub contexts: Vec<Context>,

    /// Metadata
    pub meta: MetaInfo,
}

impl Entity {
    /// Create a new entity of the given type
    pub fn new(entity_type: impl Into<String>) -> Self {
        Entity {
            id: EntityId::new(),
            entity_type: entity_type.into(),
            state: "Active".to_string(),
            properties: PropertyGraph::new(),
            relationships: Vec::new(),
            contexts: Vec::new(),
            meta: MetaInfo::new(),
        }
    }

    /// Create with specific ID (for imports/migrations)
    pub fn with_id(id: EntityId, entity_type: impl Into<String>) -> Self {
        Entity {
            id,
            entity_type: entity_type.into(),
            state: "Active".to_string(),
            properties: PropertyGraph::new(),
            relationships: Vec::new(),
            contexts: Vec::new(),
            meta: MetaInfo::new(),
        }
    }

    /// Check if entity is of a specific type pattern
    pub fn is_type(&self, pattern: &str) -> bool {
        if pattern.ends_with("*") {
            self.entity_type.starts_with(&pattern[..pattern.len()-1])
        } else {
            self.entity_type == pattern
        }
    }

    /// Add a relationship reference
    pub fn add_relationship(&mut self, rel_id: RelationshipId) {
        if !self.relationships.contains(&rel_id) {
            self.relationships.push(rel_id);
        }
    }

    /// Add a context
    pub fn add_context(&mut self, context: Context) {
        self.contexts.push(context);
    }
}