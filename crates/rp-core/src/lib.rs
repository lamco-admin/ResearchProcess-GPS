//! ResearchProcess-GPS Core Library
//! 
//! This crate defines the fundamental traits, types, and entities that form
//! the foundation of the ResearchProcess-GPS protocol and engine.

pub mod entity;
pub mod error;
pub mod id;
pub mod state;
pub mod validation;
pub mod researcher;
pub mod theory;
pub mod confidence;

pub use entity::*;
pub use error::*;
pub use id::*;
pub use state::*;
pub use validation::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::entity::{Entity, NestableEntity, VersionedEntity};
    pub use crate::error::{Error, Result};
    pub use crate::id::{EntityId, generate_id};
    pub use crate::state::{StateMachine, State, StateTransition};
    pub use crate::validation::{Validatable, ValidationError};
    pub use crate::researcher::Researcher;
    pub use crate::theory::{Theory, TheoryState};
    pub use crate::confidence::{Confidence, ConfidenceLevel};
}