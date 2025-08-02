//! ResearchProcess-GPS Core Library
//! 
//! This crate defines the fundamental traits, types, and entities that form
//! the foundation of the ResearchProcess-GPS protocol and engine.

pub mod error;
pub mod validation;
pub mod model;

pub use error::*;
pub use validation::*;

/// Re-export the meta-model
pub use meta_model_core as meta_model;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::validation::{Validatable, ValidationError};
    pub use crate::meta_model::*;
    pub use crate::model::{TemporalConstraint, TemporalQuality};
}