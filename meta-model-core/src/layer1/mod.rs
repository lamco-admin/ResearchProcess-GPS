// Layer 1: Universal Genealogical Data Model
// 
// Four fundamental primitives that can express ANY genealogical concept

mod entity;
mod relationship;
mod context;
mod certainty;
mod properties;
mod temporal;

pub use entity::Entity;
pub use relationship::{Relationship, Participant};
pub use context::{Context, Scope};
pub use certainty::Certainty;
pub use properties::{PropertyGraph, Property, Value};
pub use temporal::{TemporalValue, TemporalInstant, TemporalPrecision, CalendarExpression};

// ID types
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelationshipId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContextId(pub Uuid);

// Constructors
impl EntityId {
    pub fn new() -> Self {
        EntityId(Uuid::new_v4())
    }
}

impl RelationshipId {
    pub fn new() -> Self {
        RelationshipId(Uuid::new_v4())
    }
}

impl ContextId {
    pub fn new() -> Self {
        ContextId(Uuid::new_v4())
    }
}

// Display implementations
impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity:{}", self.0)
    }
}

impl std::fmt::Display for RelationshipId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Relationship:{}", self.0)
    }
}

impl std::fmt::Display for ContextId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context:{}", self.0)
    }
}