// Storage layer for the meta-model

mod error;
// mod sqlite;
mod query;

pub use error::{StorageError, StorageResult};
// pub use sqlite::SqliteStorage;
pub use query::{Query, QueryBuilder, QueryResult};

use crate::layer1::{Entity, EntityId, Relationship, RelationshipId};
use crate::layer2::{Process, ProcessId, Product, ProductId};
use crate::layer3::{Workspace, WorkspaceId};
use async_trait::async_trait;

/// Core storage trait for meta-model
#[async_trait]
pub trait MetaModelStorage: Send + Sync {
    // Layer 1 operations
    async fn store_entity(&self, entity: &Entity) -> StorageResult<()>;
    async fn get_entity(&self, id: EntityId) -> StorageResult<Option<Entity>>;
    async fn update_entity(&self, entity: &Entity) -> StorageResult<()>;
    async fn delete_entity(&self, id: EntityId) -> StorageResult<bool>;

    async fn store_relationship(&self, relationship: &Relationship) -> StorageResult<()>;
    async fn get_relationship(&self, id: RelationshipId) -> StorageResult<Option<Relationship>>;
    async fn get_relationships_for_entity(&self, entity_id: EntityId) -> StorageResult<Vec<Relationship>>;
    async fn delete_relationship(&self, id: RelationshipId) -> StorageResult<bool>;

    // Layer 2 operations
    async fn store_process(&self, process: &Process) -> StorageResult<()>;
    async fn get_process(&self, id: ProcessId) -> StorageResult<Option<Process>>;
    async fn update_process(&self, process: &Process) -> StorageResult<()>;
    async fn delete_process(&self, id: ProcessId) -> StorageResult<bool>;

    async fn store_product(&self, product: &Product) -> StorageResult<()>;
    async fn get_product(&self, id: ProductId) -> StorageResult<Option<Product>>;

    // Layer 3 operations
    async fn store_workspace(&self, workspace: &Workspace) -> StorageResult<()>;
    async fn get_workspace(&self, id: WorkspaceId) -> StorageResult<Option<Workspace>>;
    async fn update_workspace(&self, workspace: &Workspace) -> StorageResult<()>;

    // Query operations
    async fn query(&self, query: Query) -> StorageResult<QueryResult>;

    // Transaction support
    async fn begin_transaction(&self) -> StorageResult<Transaction>;
}

/// Transaction handle
pub struct Transaction {
    id: uuid::Uuid,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            id: uuid::Uuid::new_v4(),
        }
    }

    pub async fn commit(self) -> StorageResult<()> {
        // Implementation depends on storage backend
        Ok(())
    }

    pub async fn rollback(self) -> StorageResult<()> {
        // Implementation depends on storage backend
        Ok(())
    }
}