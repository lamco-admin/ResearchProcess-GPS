// Meta-Model Core - Universal Research Process GPS
//
// A universal meta-model that can express ANY:
// - Genealogical data model (Layer 1)
// - Research methodology (Layer 2)
// - Workflow system (Layer 3)

// Layer 1: Universal Genealogical Data Model
pub mod layer1;

// Layer 2: Universal Research Process
pub mod layer2;

// Layer 3: Universal Workflow
pub mod layer3;

// Abstraction Layers for format transformation
pub mod abstractions;

// Storage and persistence
pub mod storage;

// Common types used across layers
mod common;

// Re-export main types
pub use layer1::{Entity, Relationship, Context, Certainty};
pub use layer2::{Process, Activity, Agent, Product};
pub use layer3::{Workspace, Configuration, View, Tool};
pub use abstractions::{AbstractionLayer, AbstractionRegistry};

// Error types
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetaModelError {
    #[error("Storage error: {0}")]
    Storage(#[from] storage::StorageError),

    #[error("Abstraction error: {0}")]
    Abstraction(#[from] abstractions::AbstractionError),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid state transition: {0}")]
    InvalidTransition(String),
}

pub type Result<T> = std::result::Result<T, MetaModelError>;