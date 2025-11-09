//! Schema definition types
//!
//! Defines the structure for declaring data models that sit on top
//! of the universal meta-model.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{Error, Result};

/// A complete schema defining a data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Schema identifier (e.g., "gedcom-7", "gramps-5.2", "custom-research")
    pub id: String,

    /// Schema version
    pub version: String,

    /// Human-readable name
    pub name: String,

    /// Description of this schema
    pub description: Option<String>,

    /// Entity type definitions
    #[serde(default)]
    pub entity_types: IndexMap<String, EntityTypeDefinition>,

    /// Relationship type definitions
    #[serde(default)]
    pub relationship_types: IndexMap<String, RelationshipTypeDefinition>,

    /// Global property definitions (reusable across entity types)
    #[serde(default)]
    pub property_definitions: IndexMap<String, PropertyDefinition>,

    /// Valid state transitions (optional, for workflow enforcement)
    #[serde(default)]
    pub state_machines: IndexMap<String, StateMachine>,
}

impl Schema {
    /// Create a new schema
    pub fn new(id: impl Into<String>, version: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            name: name.into(),
            description: None,
            entity_types: IndexMap::new(),
            relationship_types: IndexMap::new(),
            property_definitions: IndexMap::new(),
            state_machines: IndexMap::new(),
        }
    }

    /// Add an entity type definition
    pub fn with_entity_type(mut self, name: impl Into<String>, def: EntityTypeDefinition) -> Self {
        self.entity_types.insert(name.into(), def);
        self
    }

    /// Add a relationship type definition
    pub fn with_relationship_type(mut self, name: impl Into<String>, def: RelationshipTypeDefinition) -> Self {
        self.relationship_types.insert(name.into(), def);
        self
    }

    /// Validate the schema definition itself
    pub fn validate(&self) -> Result<()> {
        if self.id.is_empty() {
            return Err(Error::invalid_schema("Schema ID cannot be empty"));
        }
        if self.version.is_empty() {
            return Err(Error::invalid_schema("Schema version cannot be empty"));
        }

        // Validate entity types
        for (name, def) in &self.entity_types {
            def.validate(name)?;
        }

        // Validate relationship types
        for (name, def) in &self.relationship_types {
            def.validate(name)?;
        }

        Ok(())
    }

    /// Load schema from YAML
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let schema: Schema = serde_yaml::from_str(yaml)?;
        schema.validate()?;
        Ok(schema)
    }

    /// Load schema from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        let schema: Schema = serde_json::from_str(json)?;
        schema.validate()?;
        Ok(schema)
    }

    /// Save schema to YAML
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .map_err(|e| Error::Serialization(e.to_string()))
    }

    /// Save schema to JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| Error::Serialization(e.to_string()))
    }
}

/// Definition of an entity type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityTypeDefinition {
    /// Description of this entity type
    pub description: Option<String>,

    /// Required properties
    #[serde(default)]
    pub required_properties: Vec<String>,

    /// Optional properties
    #[serde(default)]
    pub optional_properties: Vec<String>,

    /// Property definitions specific to this entity type
    #[serde(default)]
    pub properties: IndexMap<String, PropertyDefinition>,

    /// Valid states for this entity type
    #[serde(default)]
    pub valid_states: Vec<String>,

    /// Default state when creating entities
    pub default_state: Option<String>,

    /// State machine for this entity type (references schema.state_machines)
    pub state_machine: Option<String>,

    /// Constraints on this entity type
    #[serde(default)]
    pub constraints: Vec<Constraint>,
}

impl EntityTypeDefinition {
    pub fn new() -> Self {
        Self {
            description: None,
            required_properties: Vec::new(),
            optional_properties: Vec::new(),
            properties: IndexMap::new(),
            valid_states: Vec::new(),
            default_state: None,
            state_machine: None,
            constraints: Vec::new(),
        }
    }

    pub fn validate(&self, name: &str) -> Result<()> {
        // Check for duplicate properties
        let mut seen = HashSet::new();
        for prop in &self.required_properties {
            if !seen.insert(prop) {
                return Err(Error::invalid_schema(format!(
                    "Duplicate required property '{}' in entity type '{}'",
                    prop, name
                )));
            }
        }
        for prop in &self.optional_properties {
            if !seen.insert(prop) {
                return Err(Error::invalid_schema(format!(
                    "Duplicate optional property '{}' in entity type '{}'",
                    prop, name
                )));
            }
        }

        // Validate default state is in valid states
        if let Some(default) = &self.default_state {
            if !self.valid_states.is_empty() && !self.valid_states.contains(default) {
                return Err(Error::invalid_schema(format!(
                    "Default state '{}' not in valid states for entity type '{}'",
                    default, name
                )));
            }
        }

        Ok(())
    }
}

impl Default for EntityTypeDefinition {
    fn default() -> Self {
        Self::new()
    }
}

/// Definition of a relationship type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipTypeDefinition {
    /// Description of this relationship type
    pub description: Option<String>,

    /// Participant role definitions
    #[serde(default)]
    pub participants: Vec<ParticipantDefinition>,

    /// Minimum number of participants
    pub min_participants: Option<usize>,

    /// Maximum number of participants (None = unlimited)
    pub max_participants: Option<usize>,

    /// Required properties on the relationship
    #[serde(default)]
    pub required_properties: Vec<String>,

    /// Optional properties
    #[serde(default)]
    pub optional_properties: Vec<String>,

    /// Property definitions specific to this relationship type
    #[serde(default)]
    pub properties: IndexMap<String, PropertyDefinition>,

    /// Constraints on this relationship
    #[serde(default)]
    pub constraints: Vec<Constraint>,
}

impl RelationshipTypeDefinition {
    pub fn new() -> Self {
        Self {
            description: None,
            participants: Vec::new(),
            min_participants: None,
            max_participants: None,
            required_properties: Vec::new(),
            optional_properties: Vec::new(),
            properties: IndexMap::new(),
            constraints: Vec::new(),
        }
    }

    pub fn validate(&self, name: &str) -> Result<()> {
        // Validate participant count constraints
        if let (Some(min), Some(max)) = (self.min_participants, self.max_participants) {
            if min > max {
                return Err(Error::invalid_schema(format!(
                    "min_participants ({}) > max_participants ({}) for relationship type '{}'",
                    min, max, name
                )));
            }
        }

        Ok(())
    }
}

impl Default for RelationshipTypeDefinition {
    fn default() -> Self {
        Self::new()
    }
}

/// Definition of a participant role in a relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantDefinition {
    /// Role name (e.g., "parent", "child", "subject", "object")
    pub role: String,

    /// Allowed entity types for this role
    #[serde(default)]
    pub entity_types: Vec<String>,

    /// Is this role required?
    #[serde(default)]
    pub required: bool,

    /// Can this role appear multiple times?
    #[serde(default = "default_true")]
    pub multiple: bool,

    /// Description of this role
    pub description: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDefinition {
    /// Property type
    pub property_type: PropertyType,

    /// Description
    pub description: Option<String>,

    /// Validation rules
    #[serde(default)]
    pub validation: Vec<ValidationRule>,

    /// Default value (as JSON)
    pub default: Option<serde_json::Value>,
}

/// Property types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum PropertyType {
    /// Text/string
    Text {
        #[serde(default)]
        min_length: Option<usize>,
        #[serde(default)]
        max_length: Option<usize>,
        #[serde(default)]
        pattern: Option<String>,
    },

    /// Integer
    Integer {
        #[serde(default)]
        min: Option<i64>,
        #[serde(default)]
        max: Option<i64>,
    },

    /// Float
    Float {
        #[serde(default)]
        min: Option<f64>,
        #[serde(default)]
        max: Option<f64>,
    },

    /// Boolean
    Boolean,

    /// Temporal value
    Temporal,

    /// Spatial value
    Spatial,

    /// Entity reference
    Reference {
        #[serde(default)]
        entity_types: Vec<String>,
    },

    /// Collection of values
    Collection {
        item_type: Box<PropertyType>,
        #[serde(default)]
        min_items: Option<usize>,
        #[serde(default)]
        max_items: Option<usize>,
    },

    /// Enum (one of a set of values)
    Enum {
        values: Vec<String>,
    },

    /// Any value
    Any,
}

/// Validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "rule", content = "params")]
pub enum ValidationRule {
    /// Regex pattern match
    Pattern(String),

    /// Minimum value
    Min(f64),

    /// Maximum value
    Max(f64),

    /// Required (not null/empty)
    Required,

    /// Custom validation expression
    Custom(String),
}

/// State machine definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    /// Initial state
    pub initial_state: String,

    /// All valid states
    pub states: Vec<String>,

    /// Valid transitions (from_state -> to_states)
    pub transitions: IndexMap<String, Vec<String>>,

    /// Terminal states (no transitions out)
    #[serde(default)]
    pub terminal_states: Vec<String>,
}

/// Constraint on entity or relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec")]
pub enum Constraint {
    /// Unique property value across all entities of this type
    Unique {
        property: String,
    },

    /// Conditional requirement
    ConditionalRequired {
        property: String,
        condition: String,
    },

    /// Custom constraint expression
    Custom {
        expression: String,
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_creation() {
        let schema = Schema::new("test", "1.0", "Test Schema");
        assert_eq!(schema.id, "test");
        assert_eq!(schema.version, "1.0");
        assert_eq!(schema.name, "Test Schema");
    }

    #[test]
    fn test_schema_validation() {
        let schema = Schema::new("test", "1.0", "Test");
        assert!(schema.validate().is_ok());

        let invalid = Schema::new("", "1.0", "Test");
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_entity_type_definition() {
        let mut def = EntityTypeDefinition::new();
        def.required_properties.push("name".to_string());
        def.valid_states = vec!["Active".to_string(), "Inactive".to_string()];
        def.default_state = Some("Active".to_string());

        assert!(def.validate("Person").is_ok());
    }

    #[test]
    fn test_relationship_type_definition() {
        let mut def = RelationshipTypeDefinition::new();
        def.min_participants = Some(2);
        def.max_participants = Some(2);

        assert!(def.validate("Marriage").is_ok());

        // Invalid: min > max
        let mut invalid = RelationshipTypeDefinition::new();
        invalid.min_participants = Some(5);
        invalid.max_participants = Some(2);
        assert!(invalid.validate("Invalid").is_err());
    }

    #[test]
    fn test_property_types() {
        let text = PropertyType::Text {
            min_length: Some(1),
            max_length: Some(100),
            pattern: None,
        };
        assert!(matches!(text, PropertyType::Text { .. }));

        let int = PropertyType::Integer {
            min: Some(0),
            max: Some(150),
        };
        assert!(matches!(int, PropertyType::Integer { .. }));
    }
}
