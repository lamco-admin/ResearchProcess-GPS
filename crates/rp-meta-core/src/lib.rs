//! ResearchProcess-GPS Universal Meta-Model Core
//!
//! This crate provides universal primitives that can express ANY research data model,
//! enabling complete flexibility and semantic preservation.
//!
//! # Core Primitives
//!
//! 1. **Entity** - Universal container for any type of thing
//! 2. **Relationship** - Universal connection between entities (N-ary)
//! 3. **Context** - Universal qualifier/scope for entities and relationships
//! 4. **Certainty** - Universal uncertainty expression
//!
//! # Design Principles
//!
//! - **Open-ended types**: No hardcoded entity types, relationship types, or states
//! - **Infinite flexibility**: Properties can contain anything, including other entities
//! - **Semantic preservation**: Never lose information through transformations
//! - **NO_FALLBACK_POLICY**: All errors explicit, no silent failures
//! - **Multi-paradigm**: Support multiple data models simultaneously

pub mod entity;
pub mod relationship;
pub mod context;
pub mod certainty;
pub mod properties;
pub mod temporal;
pub mod spatial;
pub mod metadata;
pub mod error;
pub mod id;

// Re-exports for convenience
pub use entity::Entity;
pub use relationship::{Relationship, Participant};
pub use context::{Context, Scope};
pub use certainty::Certainty;
pub use properties::{PropertyGraph, Property, Value};
pub use temporal::{TemporalValue, TemporalInstant, CalendarExpression};
pub use spatial::{SpatialValue, SpatialPoint, CoordinateExpression};
pub use metadata::MetaInfo;
pub use error::{Error, Result};
pub use id::{
    EntityId, RelationshipId, ContextId, PropertyId,
    ProcessId, ActivityId, AgentId, ProductId,
    WorkspaceId, ConfigurationId, ViewId, ToolId,
};

/// Prelude for common imports
pub mod prelude {
    pub use crate::entity::Entity;
    pub use crate::relationship::{Relationship, Participant};
    pub use crate::context::{Context, Scope};
    pub use crate::certainty::Certainty;
    pub use crate::properties::{PropertyGraph, Property, Value};
    pub use crate::temporal::{TemporalValue, TemporalInstant};
    pub use crate::spatial::{SpatialValue, SpatialPoint};
    pub use crate::metadata::MetaInfo;
    pub use crate::error::{Error, Result};
    pub use crate::id::{EntityId, RelationshipId, ContextId};
}
