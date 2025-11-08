//! Error types for meta-model operations
//!
//! Following NO_FALLBACK_POLICY: All errors are explicit, no silent failures,
//! no default assumptions, no degraded operation.

use thiserror::Error;

/// Result type for meta-model operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in meta-model operations
#[derive(Error, Debug)]
pub enum Error {
    /// Entity not found
    #[error("Entity not found: {id}")]
    EntityNotFound { id: String },

    /// Relationship not found
    #[error("Relationship not found: {id}")]
    RelationshipNotFound { id: String },

    /// Context not found
    #[error("Context not found: {id}")]
    ContextNotFound { id: String },

    /// Invalid entity type
    #[error("Invalid entity type: {entity_type}. Reason: {reason}")]
    InvalidEntityType {
        entity_type: String,
        reason: String,
    },

    /// Invalid relationship type
    #[error("Invalid relationship type: {relationship_type}. Reason: {reason}")]
    InvalidRelationshipType {
        relationship_type: String,
        reason: String,
    },

    /// Invalid state
    #[error("Invalid state: {state}. Reason: {reason}")]
    InvalidState { state: String, reason: String },

    /// Invalid property
    #[error("Invalid property: {property}. Reason: {reason}")]
    InvalidProperty { property: String, reason: String },

    /// Missing required property
    #[error("Missing required property: {property} on entity type {entity_type}")]
    MissingRequiredProperty {
        property: String,
        entity_type: String,
    },

    /// Type mismatch
    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    /// Invalid temporal value
    #[error("Invalid temporal value: {reason}")]
    InvalidTemporalValue { reason: String },

    /// Invalid spatial value
    #[error("Invalid spatial value: {reason}")]
    InvalidSpatialValue { reason: String },

    /// Invalid certainty value
    #[error("Invalid certainty value: {reason}")]
    InvalidCertainty { reason: String },

    /// Schema violation
    #[error("Schema violation: {reason}")]
    SchemaViolation { reason: String },

    /// Validation failed
    #[error("Validation failed: {reason}")]
    ValidationFailed { reason: String },

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// UUID parse error
    #[error("UUID parse error: {0}")]
    UuidParse(#[from] uuid::Error),

    /// Property not found
    #[error("Property not found: {property}")]
    PropertyNotFound { property: String },

    /// Property type mismatch
    #[error("Property {property} has wrong type: expected {expected}, got {actual}")]
    PropertyTypeMismatch {
        property: String,
        expected: String,
        actual: String,
    },

    /// Relationship participant error
    #[error("Invalid relationship participant: {reason}")]
    InvalidParticipant { reason: String },

    /// Missing required participant role
    #[error("Missing required participant role: {role} in relationship {relationship_type}")]
    MissingParticipantRole {
        role: String,
        relationship_type: String,
    },

    /// Context scope error
    #[error("Invalid context scope: {reason}")]
    InvalidContextScope { reason: String },

    /// Computation error
    #[error("Computation failed: {reason}")]
    ComputationFailed { reason: String },

    /// Circular reference detected
    #[error("Circular reference detected: {path}")]
    CircularReference { path: String },

    /// Capacity exceeded
    #[error("Capacity exceeded: {resource}. Limit: {limit}, Attempted: {attempted}")]
    CapacityExceeded {
        resource: String,
        limit: usize,
        attempted: usize,
    },

    /// Internal error (should never happen)
    #[error("Internal error: {0}. This is a bug, please report it.")]
    Internal(String),

    /// Generic error for wrapping other error types
    #[error("{0}")]
    Generic(String),
}

impl Error {
    /// Create an EntityNotFound error
    pub fn entity_not_found(id: impl ToString) -> Self {
        Self::EntityNotFound {
            id: id.to_string(),
        }
    }

    /// Create a MissingRequiredProperty error
    pub fn missing_property(property: impl ToString, entity_type: impl ToString) -> Self {
        Self::MissingRequiredProperty {
            property: property.to_string(),
            entity_type: entity_type.to_string(),
        }
    }

    /// Create a PropertyTypeMismatch error
    pub fn property_type_mismatch(
        property: impl ToString,
        expected: impl ToString,
        actual: impl ToString,
    ) -> Self {
        Self::PropertyTypeMismatch {
            property: property.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }

    /// Create a ValidationFailed error
    pub fn validation_failed(reason: impl ToString) -> Self {
        Self::ValidationFailed {
            reason: reason.to_string(),
        }
    }

    /// Create a SchemaViolation error
    pub fn schema_violation(reason: impl ToString) -> Self {
        Self::SchemaViolation {
            reason: reason.to_string(),
        }
    }

    /// Create an Internal error
    pub fn internal(msg: impl ToString) -> Self {
        Self::Internal(msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::entity_not_found("test-id");
        assert!(err.to_string().contains("test-id"));
    }

    #[test]
    fn test_error_variants() {
        let _ = Error::missing_property("name", "Person");
        let _ = Error::validation_failed("invalid data");
        let _ = Error::schema_violation("type mismatch");
    }
}
