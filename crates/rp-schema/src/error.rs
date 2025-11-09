//! Error types for schema operations

use thiserror::Error;

/// Result type for schema operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in schema operations
#[derive(Error, Debug)]
pub enum Error {
    /// Schema not found
    #[error("Schema not found: {schema_id}")]
    SchemaNotFound { schema_id: String },

    /// Invalid schema definition
    #[error("Invalid schema definition: {reason}")]
    InvalidSchema { reason: String },

    /// Schema validation failed
    #[error("Schema validation failed: {reason}")]
    ValidationFailed { reason: String },

    /// Entity type not defined in schema
    #[error("Entity type not defined: {entity_type} in schema {schema_id}")]
    EntityTypeNotDefined {
        entity_type: String,
        schema_id: String,
    },

    /// Relationship type not defined in schema
    #[error("Relationship type not defined: {relationship_type} in schema {schema_id}")]
    RelationshipTypeNotDefined {
        relationship_type: String,
        schema_id: String,
    },

    /// Required property missing
    #[error("Required property missing: {property} on {entity_type}")]
    RequiredPropertyMissing {
        property: String,
        entity_type: String,
    },

    /// Property type mismatch
    #[error("Property type mismatch: {property} expected {expected}, got {actual}")]
    PropertyTypeMismatch {
        property: String,
        expected: String,
        actual: String,
    },

    /// Invalid property value
    #[error("Invalid property value: {property} - {reason}")]
    InvalidPropertyValue { property: String, reason: String },

    /// State not valid for entity type
    #[error("Invalid state: {state} not valid for entity type {entity_type}")]
    InvalidState {
        state: String,
        entity_type: String,
    },

    /// Adapter not found
    #[error("Adapter not found: {adapter_id}")]
    AdapterNotFound { adapter_id: String },

    /// Adapter error
    #[error("Adapter error: {reason}")]
    AdapterError { reason: String },

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Meta-model error
    #[error("Meta-model error: {0}")]
    MetaModel(#[from] rp_meta_core::Error),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// YAML parsing error
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

impl Error {
    pub fn schema_not_found(schema_id: impl ToString) -> Self {
        Self::SchemaNotFound {
            schema_id: schema_id.to_string(),
        }
    }

    pub fn invalid_schema(reason: impl ToString) -> Self {
        Self::InvalidSchema {
            reason: reason.to_string(),
        }
    }

    pub fn validation_failed(reason: impl ToString) -> Self {
        Self::ValidationFailed {
            reason: reason.to_string(),
        }
    }

    pub fn entity_type_not_defined(entity_type: impl ToString, schema_id: impl ToString) -> Self {
        Self::EntityTypeNotDefined {
            entity_type: entity_type.to_string(),
            schema_id: schema_id.to_string(),
        }
    }

    pub fn adapter_error(reason: impl ToString) -> Self {
        Self::AdapterError {
            reason: reason.to_string(),
        }
    }
}
