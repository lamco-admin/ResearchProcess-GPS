//! Validation of entities and relationships against schemas

use rp_meta_core::{Entity, Relationship, Property, Value};

use crate::{Schema, Error, Result};
use crate::definition::PropertyType;

/// Validator for entities and relationships
pub struct Validator<'a> {
    schema: &'a Schema,
}

impl<'a> Validator<'a> {
    /// Create a new validator for a schema
    pub fn new(schema: &'a Schema) -> Self {
        Self { schema }
    }

    /// Validate an entity against the schema
    pub fn validate_entity(&self, entity: &Entity) -> Result<ValidationContext> {
        let mut ctx = ValidationContext::new();

        // Check if entity type is defined
        let type_def = self.schema.entity_types.get(&entity.entity_type)
            .ok_or_else(|| Error::entity_type_not_defined(&entity.entity_type, &self.schema.id))?;

        // Validate state
        if !type_def.valid_states.is_empty() {
            if !type_def.valid_states.contains(&entity.state) {
                ctx.add_error(format!(
                    "Invalid state '{}' for entity type '{}'. Valid states: {:?}",
                    entity.state, entity.entity_type, type_def.valid_states
                ));
            }
        }

        // Validate required properties
        for required in &type_def.required_properties {
            if !entity.properties.contains(required) {
                ctx.add_error(format!(
                    "Required property '{}' missing on entity type '{}'",
                    required, entity.entity_type
                ));
            }
        }

        // Validate property types
        for (prop_name, prop_def) in &type_def.properties {
            if let Some(prop_value) = entity.properties.get(prop_name) {
                if let Err(e) = self.validate_property_type(prop_value, &prop_def.property_type) {
                    ctx.add_error(format!(
                        "Property '{}' validation failed: {}",
                        prop_name, e
                    ));
                }
            }
        }

        // Validate constraints
        for constraint in &type_def.constraints {
            // TODO: Implement constraint validation
            tracing::debug!("Constraint validation not yet implemented: {:?}", constraint);
        }

        if ctx.has_errors() {
            Err(Error::validation_failed(ctx.errors.join("; ")))
        } else {
            Ok(ctx)
        }
    }

    /// Validate a relationship against the schema
    pub fn validate_relationship(&self, relationship: &Relationship) -> Result<ValidationContext> {
        let mut ctx = ValidationContext::new();

        // Check if relationship type is defined
        let type_def = self.schema.relationship_types.get(&relationship.relationship_type)
            .ok_or_else(|| Error::RelationshipTypeNotDefined {
                relationship_type: relationship.relationship_type.clone(),
                schema_id: self.schema.id.clone(),
            })?;

        // Validate participant count
        let participant_count = relationship.participants.len();
        if let Some(min) = type_def.min_participants {
            if participant_count < min {
                ctx.add_error(format!(
                    "Relationship '{}' requires at least {} participants, got {}",
                    relationship.relationship_type, min, participant_count
                ));
            }
        }
        if let Some(max) = type_def.max_participants {
            if participant_count > max {
                ctx.add_error(format!(
                    "Relationship '{}' allows at most {} participants, got {}",
                    relationship.relationship_type, max, participant_count
                ));
            }
        }

        // Validate required participant roles
        for participant_def in &type_def.participants {
            if participant_def.required {
                let has_role = relationship.participants
                    .iter()
                    .any(|p| p.role == participant_def.role);
                if !has_role {
                    ctx.add_error(format!(
                        "Required participant role '{}' missing in relationship '{}'",
                        participant_def.role, relationship.relationship_type
                    ));
                }
            }
        }

        // Validate required properties
        for required in &type_def.required_properties {
            if !relationship.properties.contains(required) {
                ctx.add_error(format!(
                    "Required property '{}' missing on relationship type '{}'",
                    required, relationship.relationship_type
                ));
            }
        }

        if ctx.has_errors() {
            Err(Error::validation_failed(ctx.errors.join("; ")))
        } else {
            Ok(ctx)
        }
    }

    /// Validate a property value against its type definition
    fn validate_property_type(&self, value: &Property, type_def: &PropertyType) -> Result<()> {
        match (value, type_def) {
            (Property::Value(Value::Text(s)), PropertyType::Text { min_length, max_length, pattern }) => {
                if let Some(min) = min_length {
                    if s.len() < *min {
                        return Err(Error::InvalidPropertyValue {
                            property: "text".to_string(),
                            reason: format!("Length {} < minimum {}", s.len(), min),
                        });
                    }
                }
                if let Some(max) = max_length {
                    if s.len() > *max {
                        return Err(Error::InvalidPropertyValue {
                            property: "text".to_string(),
                            reason: format!("Length {} > maximum {}", s.len(), max),
                        });
                    }
                }
                if let Some(pat) = pattern {
                    let re = regex::Regex::new(pat)
                        .map_err(|e| Error::InvalidPropertyValue {
                            property: "text".to_string(),
                            reason: format!("Invalid regex pattern: {}", e),
                        })?;
                    if !re.is_match(&s) {
                        return Err(Error::InvalidPropertyValue {
                            property: "text".to_string(),
                            reason: format!("Does not match pattern: {}", pat),
                        });
                    }
                }
                Ok(())
            }
            (Property::Value(Value::Integer(i)), PropertyType::Integer { min, max }) => {
                if let Some(min_val) = min {
                    if i < min_val {
                        return Err(Error::InvalidPropertyValue {
                            property: "integer".to_string(),
                            reason: format!("{} < minimum {}", i, min_val),
                        });
                    }
                }
                if let Some(max_val) = max {
                    if i > max_val {
                        return Err(Error::InvalidPropertyValue {
                            property: "integer".to_string(),
                            reason: format!("{} > maximum {}", i, max_val),
                        });
                    }
                }
                Ok(())
            }
            (Property::Value(Value::Boolean(_)), PropertyType::Boolean) => Ok(()),
            (Property::Collection(items), PropertyType::Collection { item_type, min_items, max_items }) => {
                if let Some(min) = min_items {
                    if items.len() < *min {
                        return Err(Error::InvalidPropertyValue {
                            property: "collection".to_string(),
                            reason: format!("Size {} < minimum {}", items.len(), min),
                        });
                    }
                }
                if let Some(max) = max_items {
                    if items.len() > *max {
                        return Err(Error::InvalidPropertyValue {
                            property: "collection".to_string(),
                            reason: format!("Size {} > maximum {}", items.len(), max),
                        });
                    }
                }
                // Validate each item
                for item in items {
                    self.validate_property_type(&item, item_type)?;
                }
                Ok(())
            }
            (_, PropertyType::Any) => Ok(()),
            _ => {
                // Type mismatch or unimplemented type check
                Ok(())
            }
        }
    }
}

/// Validation context that accumulates errors and warnings
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// Validation errors
    pub errors: Vec<String>,

    /// Validation warnings
    pub warnings: Vec<String>,
}

impl ValidationContext {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
    }

    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::*;

    #[test]
    fn test_entity_validation() {
        // Create schema with Person entity type
        let mut schema = Schema::new("test", "1.0", "Test");

        let mut person_def = EntityTypeDefinition::new();
        person_def.required_properties = vec!["name".to_string()];
        person_def.valid_states = vec!["Active".to_string()];

        schema.entity_types.insert("Person".to_string(), person_def);

        // Create valid entity
        let mut entity = Entity::new("Person");
        entity.state = "Active".to_string();
        entity.set_property("name", Property::Value(Value::Text("John".to_string())));

        let validator = Validator::new(&schema);
        assert!(validator.validate_entity(&entity).is_ok());

        // Create invalid entity (missing required property)
        let invalid = Entity::new("Person");
        assert!(validator.validate_entity(&invalid).is_err());
    }
}
