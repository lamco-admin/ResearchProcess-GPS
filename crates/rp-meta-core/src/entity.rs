//! Universal Entity primitive - can represent anything

use serde::{Deserialize, Serialize};

use crate::{Context, EntityId, MetaInfo, PropertyGraph, RelationshipId, Result};

/// Universal entity that can represent anything
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    /// Unique identifier
    pub id: EntityId,

    /// Entity type (open-ended)
    /// Examples: "IdentityPersona", "Theory", "Evidence", "DNASample", "Artifact"
    pub entity_type: String,

    /// Entity state (open-ended)
    /// Examples: "Draft", "Active", "Concluded", "Verified"
    pub state: String,

    /// Flexible property graph containing any data
    pub properties: PropertyGraph,

    /// Relationships this entity participates in
    pub relationships: Vec<RelationshipId>,

    /// Contexts that qualify this entity
    pub contexts: Vec<Context>,

    /// Meta-information (provenance, versioning, etc.)
    pub meta: MetaInfo,
}

impl Entity {
    /// Create a new entity
    pub fn new(entity_type: impl Into<String>) -> Self {
        Self {
            id: EntityId::new(),
            entity_type: entity_type.into(),
            state: "Initial".to_string(),
            properties: PropertyGraph::new(),
            relationships: Vec::new(),
            contexts: Vec::new(),
            meta: MetaInfo::now(),
        }
    }

    /// Create a new entity with a specific ID (for migration/import)
    pub fn with_id(id: EntityId, entity_type: impl Into<String>) -> Self {
        Self {
            id,
            entity_type: entity_type.into(),
            state: "Initial".to_string(),
            properties: PropertyGraph::new(),
            relationships: Vec::new(),
            contexts: Vec::new(),
            meta: MetaInfo::now(),
        }
    }

    /// Create a new entity with a creator
    pub fn with_creator(entity_type: impl Into<String>, created_by: EntityId) -> Self {
        Self {
            id: EntityId::new(),
            entity_type: entity_type.into(),
            state: "Initial".to_string(),
            properties: PropertyGraph::new(),
            relationships: Vec::new(),
            contexts: Vec::new(),
            meta: MetaInfo::with_creator(created_by),
        }
    }

    /// Set the state
    pub fn with_state(mut self, state: impl Into<String>) -> Self {
        self.state = state.into();
        self
    }

    /// Add a property
    pub fn with_property(mut self, key: impl Into<String>, value: crate::Property) -> Self {
        self.properties.set(key, value);
        self
    }

    /// Add multiple properties
    pub fn with_properties(mut self, properties: PropertyGraph) -> Self {
        self.properties.merge(properties);
        self
    }

    /// Add a relationship
    pub fn with_relationship(mut self, relationship_id: RelationshipId) -> Self {
        self.relationships.push(relationship_id);
        self
    }

    /// Add a context
    pub fn with_context(mut self, context: Context) -> Self {
        self.contexts.push(context);
        self
    }

    /// Add multiple contexts
    pub fn with_contexts(mut self, contexts: Vec<Context>) -> Self {
        self.contexts.extend(contexts);
        self
    }

    /// Set meta-information
    pub fn with_meta(mut self, meta: MetaInfo) -> Self {
        self.meta = meta;
        self
    }

    /// Get a property value
    pub fn get_property(&self, key: &str) -> Option<&crate::Property> {
        self.properties.get(key)
    }

    /// Set a property value
    pub fn set_property(&mut self, key: impl Into<String>, value: crate::Property) {
        self.properties.set(key, value);
    }

    /// Update state
    pub fn set_state(&mut self, state: impl Into<String>) {
        self.state = state.into();
        self.meta.update(None);
    }

    /// Add a relationship
    pub fn add_relationship(&mut self, relationship_id: RelationshipId) {
        if !self.relationships.contains(&relationship_id) {
            self.relationships.push(relationship_id);
        }
    }

    /// Remove a relationship
    pub fn remove_relationship(&mut self, relationship_id: RelationshipId) -> bool {
        if let Some(pos) = self.relationships.iter().position(|&r| r == relationship_id) {
            self.relationships.remove(pos);
            true
        } else {
            false
        }
    }

    /// Add a context
    pub fn add_context(&mut self, context: Context) {
        self.contexts.push(context);
    }

    /// Get contexts of a specific type
    pub fn get_contexts_by_type(&self, context_type: &str) -> Vec<&Context> {
        self.contexts
            .iter()
            .filter(|c| c.context_type == context_type)
            .collect()
    }

    /// Check if entity has a specific context type
    pub fn has_context_type(&self, context_type: &str) -> bool {
        self.contexts.iter().any(|c| c.context_type == context_type)
    }

    /// Validate required properties based on entity type
    pub fn validate(&self) -> Result<()> {
        // Basic validation - can be extended with schema validation
        if self.entity_type.is_empty() {
            return Err(crate::Error::validation_failed("Entity type cannot be empty"));
        }
        if self.state.is_empty() {
            return Err(crate::Error::validation_failed("Entity state cannot be empty"));
        }
        Ok(())
    }

    /// Create a new version of this entity
    pub fn create_version(&self, modified_by: Option<EntityId>) -> Self {
        let mut new_entity = self.clone();
        new_entity.id = EntityId::new();
        new_entity.meta = self.meta.new_version(self.id, modified_by);
        new_entity
    }

    /// Check if this entity is of a specific type
    pub fn is_type(&self, entity_type: &str) -> bool {
        self.entity_type == entity_type
    }

    /// Check if this entity is in a specific state
    pub fn is_state(&self, state: &str) -> bool {
        self.state == state
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new("Unknown")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Property, Value};

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new("Person");
        assert_eq!(entity.entity_type, "Person");
        assert_eq!(entity.state, "Initial");
        assert!(entity.properties.is_empty());
    }

    #[test]
    fn test_entity_with_properties() {
        let entity = Entity::new("Person")
            .with_property("name", Property::Value(Value::Text("John".to_string())))
            .with_property("age", Property::Value(Value::Integer(42)));

        assert_eq!(entity.properties.get_text("name").unwrap(), "John");
        assert_eq!(entity.properties.get_integer("age").unwrap(), 42);
    }

    #[test]
    fn test_entity_with_state() {
        let entity = Entity::new("Theory").with_state("Active");
        assert_eq!(entity.state, "Active");
    }

    #[test]
    fn test_entity_with_context() {
        let ctx = Context::temporal("19th century", None);
        let entity = Entity::new("Event").with_context(ctx.clone());

        assert_eq!(entity.contexts.len(), 1);
        assert_eq!(entity.contexts[0].context_type, "Temporal");
    }

    #[test]
    fn test_entity_relationships() {
        let mut entity = Entity::new("Person");
        let rel_id = RelationshipId::new();

        entity.add_relationship(rel_id);
        assert_eq!(entity.relationships.len(), 1);

        // Adding same relationship twice doesn't duplicate
        entity.add_relationship(rel_id);
        assert_eq!(entity.relationships.len(), 1);

        // Remove relationship
        assert!(entity.remove_relationship(rel_id));
        assert_eq!(entity.relationships.len(), 0);
    }

    #[test]
    fn test_entity_context_queries() {
        let mut entity = Entity::new("Event");
        entity.add_context(Context::temporal("1850s", None));
        entity.add_context(Context::spatial("Boston", ));

        assert!(entity.has_context_type("Temporal"));
        assert!(entity.has_context_type("Spatial"));
        assert!(!entity.has_context_type("Cultural"));

        let temporal_contexts = entity.get_contexts_by_type("Temporal");
        assert_eq!(temporal_contexts.len(), 1);
    }

    #[test]
    fn test_entity_validation() {
        let entity = Entity::new("Person");
        assert!(entity.validate().is_ok());

        let invalid_entity = Entity {
            id: EntityId::new(),
            entity_type: "".to_string(),
            state: "Active".to_string(),
            properties: PropertyGraph::new(),
            relationships: Vec::new(),
            contexts: Vec::new(),
            meta: MetaInfo::now(),
        };
        assert!(invalid_entity.validate().is_err());
    }

    #[test]
    fn test_entity_versioning() {
        let entity = Entity::new("Document")
            .with_property("title", Property::Value(Value::Text("Draft".to_string())));

        let modifier = EntityId::new();
        let new_version = entity.create_version(Some(modifier));

        assert_ne!(entity.id, new_version.id);
        assert_eq!(new_version.meta.version, 2);
        assert_eq!(new_version.meta.previous_version, Some(entity.id));
        assert_eq!(new_version.properties.get_text("title").unwrap(), "Draft");
    }

    #[test]
    fn test_entity_type_check() {
        let entity = Entity::new("Person");
        assert!(entity.is_type("Person"));
        assert!(!entity.is_type("Event"));
    }

    #[test]
    fn test_entity_state_check() {
        let entity = Entity::new("Theory").with_state("Active");
        assert!(entity.is_state("Active"));
        assert!(!entity.is_state("Draft"));
    }
}
