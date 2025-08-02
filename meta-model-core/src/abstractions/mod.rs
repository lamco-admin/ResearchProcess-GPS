// Abstraction Layers - Format conversion and data transformation
//
// These are NOT storage abstractions, but format converters that transform
// between external formats (GRAMPS, GEDCOM, etc.) and our meta-model

mod registry;
pub mod base;
pub mod error;

// Built-in abstractions
#[cfg(feature = "builtin-abstractions")]
mod builtin;

pub use registry::AbstractionRegistry;
pub use base::{AbstractionLayer, TransformContext};
pub use error::{AbstractionError, AbstractionResult};

// Re-export built-in abstractions when enabled
#[cfg(feature = "builtin-abstractions")]
pub use builtin::{
    gramps::GrampsAbstraction,
    gedcom::GedcomAbstraction,
    calendar::CalendarAbstraction,
};