//! Storage abstraction layer for meta-model
//!
//! This follows the same pattern as rp-storage, providing a clean abstraction
//! over different storage backends (PostgreSQL, GitHub, filesystem, etc.)

pub mod traits;
pub mod error;
pub mod capabilities;
pub mod query;
pub mod transaction;

pub use traits::{
    StorageBackend, QueryableBackend, GraphBackend, StreamingBackend,
    BulkOperations, StorageEntity, HealthStatus,
};
pub use error::{StorageError, StorageResult};
pub use capabilities::StorageCapabilities;
pub use query::{Query, QueryBuilder, QueryResult};
pub use transaction::Transaction;

/// Registry for storage backends
pub mod registry {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// Storage backend registry
    pub struct Registry {
        backends: RwLock<HashMap<String, Arc<dyn StorageBackend>>>,
    }

    impl Registry {
        pub fn new() -> Self {
            Registry {
                backends: RwLock::new(HashMap::new()),
            }
        }

        /// Register a storage backend
        pub async fn register(
            &self,
            name: String,
            backend: Arc<dyn StorageBackend>,
        ) -> StorageResult<()> {
            let mut backends = self.backends.write().await;
            if backends.contains_key(&name) {
                return Err(StorageError::AlreadyExists(format!(
                    "Backend '{}' already registered", name
                )));
            }
            backends.insert(name, backend);
            Ok(())
        }

        /// Get a storage backend
        pub async fn get(&self, name: &str) -> StorageResult<Arc<dyn StorageBackend>> {
            let backends = self.backends.read().await;
            backends.get(name)
                .cloned()
                .ok_or_else(|| StorageError::NotFound(format!(
                    "Backend '{}' not found", name
                )))
        }

        /// List registered backends
        pub async fn list(&self) -> Vec<String> {
            let backends = self.backends.read().await;
            backends.keys().cloned().collect()
        }
    }

    impl Default for Registry {
        fn default() -> Self {
            Self::new()
        }
    }
}