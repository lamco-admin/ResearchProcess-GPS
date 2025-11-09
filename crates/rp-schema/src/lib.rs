//! Schema System for ResearchProcess-GPS
//!
//! This crate provides a flexible schema system that allows defining
//! and validating different data models (GEDCOM, GRAMPS, custom, etc.)
//! on top of the universal meta-model.
//!
//! # Key Concepts
//!
//! - **Schema**: Defines entity types, relationship types, and validation rules
//! - **Adapter**: Translates between a schema and external formats
//! - **Validator**: Validates entities and relationships against a schema
//! - **Registry**: Manages multiple schemas

pub mod definition;
pub mod validation;
pub mod registry;
pub mod adapter;
pub mod error;

pub use definition::{Schema, EntityTypeDefinition, RelationshipTypeDefinition, PropertyDefinition, ValidationRule};
pub use validation::{Validator, ValidationContext};
pub use registry::{SchemaRegistry, SchemaId};
pub use adapter::{Adapter, AdapterRegistry};
pub use error::{Error, Result};

/// Prelude for common imports
pub mod prelude {
    pub use crate::definition::{Schema, EntityTypeDefinition, RelationshipTypeDefinition};
    pub use crate::validation::{Validator, ValidationContext};
    pub use crate::registry::{SchemaRegistry, SchemaId};
    pub use crate::adapter::{Adapter, AdapterRegistry};
    pub use crate::error::{Error, Result};
}
