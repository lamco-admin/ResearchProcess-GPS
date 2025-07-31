//! Storage abstraction layer for ResearchProcess-GPS
//! 
//! This crate provides storage-agnostic traits that allow the engine to work
//! with various backends (PostgreSQL, SQLite, Git, filesystem, S3, etc.)

pub mod traits;
pub mod capabilities;
pub mod query;
pub mod transaction;
pub mod error;
pub mod registry;
pub mod types;

pub use traits::*;
pub use capabilities::*;
pub use query::*;
pub use transaction::*;
pub use error::*;
pub use registry::*;
pub use types::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::traits::{StorageBackend, QueryableBackend, VectorSearchBackend, GraphBackend};
    pub use crate::capabilities::{StorageCapabilities, QueryCapabilities};
    pub use crate::transaction::Transaction;
    pub use crate::error::{StorageError, StorageResult};
    pub use crate::registry::StorageRegistry;
}