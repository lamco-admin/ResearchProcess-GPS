//! Transaction abstraction for storage operations

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::{StorageResult, StorageEntity};

/// Transaction operations for atomic changes
#[async_trait]
pub trait Transaction: Send {
    /// Store an entity
    async fn put_entity(&mut self, entity: &StorageEntity) -> StorageResult<()>;
    
    /// Retrieve an entity by ID
    async fn get_entity(&mut self, id: Uuid) -> StorageResult<Option<StorageEntity>>;
    
    /// Update an entity (partial update)
    async fn update_entity(
        &mut self,
        id: Uuid,
        updates: HashMap<String, JsonValue>,
    ) -> StorageResult<bool>;
    
    /// Delete an entity
    async fn delete_entity(&mut self, id: Uuid) -> StorageResult<bool>;
    
    /// Check if entity exists
    async fn exists(&mut self, id: Uuid) -> StorageResult<bool>;
    
    /// Get multiple entities by IDs
    async fn get_entities(&mut self, ids: &[Uuid]) -> StorageResult<Vec<StorageEntity>>;
    
    /// List entities of a specific type
    async fn list_by_type(
        &mut self,
        entity_type: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> StorageResult<Vec<StorageEntity>>;
    
    /// Store a relationship (if graph operations supported)
    async fn put_relationship(
        &mut self,
        _from: Uuid,
        _to: Uuid,
        _rel_type: &str,
        _properties: Option<HashMap<String, JsonValue>>,
    ) -> StorageResult<Uuid> {
        Err(crate::StorageError::NotSupported(
            "Graph operations not supported by this backend".to_string()
        ))
    }
    
    /// Delete a relationship
    async fn delete_relationship(&mut self, _rel_id: Uuid) -> StorageResult<bool> {
        Err(crate::StorageError::NotSupported(
            "Graph operations not supported by this backend".to_string()
        ))
    }
    
    /// Add to entity history (if versioning supported)
    async fn add_version(
        &mut self,
        _entity_id: Uuid,
        _version_data: VersionData,
    ) -> StorageResult<()> {
        Err(crate::StorageError::NotSupported(
            "Version history not supported by this backend".to_string()
        ))
    }
    
    /// Commit the transaction
    async fn commit(self) -> StorageResult<()>;
    
    /// Rollback the transaction
    async fn rollback(self) -> StorageResult<()>;
    
    /// Get transaction ID (for tracking)
    fn transaction_id(&self) -> Uuid;
    
    /// Check if transaction is still active
    fn is_active(&self) -> bool;
}

/// Version data for entity history
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionData {
    pub version: u64,
    pub data: JsonValue,
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
    pub change_reason: Option<String>,
    pub parent_version: Option<u64>,
}

/// Transaction isolation levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

/// Transaction options
#[derive(Debug, Clone)]
pub struct TransactionOptions {
    pub isolation_level: Option<IsolationLevel>,
    pub read_only: bool,
    pub timeout: Option<std::time::Duration>,
    pub retry_on_conflict: bool,
    pub max_retries: u32,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            isolation_level: None,
            read_only: false,
            timeout: None,
            retry_on_conflict: false,
            max_retries: 3,
        }
    }
}

/// Extended transaction interface for advanced backends
#[async_trait]
pub trait ExtendedTransaction: Transaction {
    /// Set a savepoint
    async fn savepoint(&mut self, name: &str) -> StorageResult<()>;
    
    /// Rollback to a savepoint
    async fn rollback_to(&mut self, savepoint: &str) -> StorageResult<()>;
    
    /// Release a savepoint
    async fn release_savepoint(&mut self, name: &str) -> StorageResult<()>;
    
    /// Lock an entity for update
    async fn lock_entity(&mut self, id: Uuid) -> StorageResult<()>;
    
    /// Get transaction metadata
    fn metadata(&self) -> &TransactionMetadata;
}

/// Transaction metadata
#[derive(Debug, Clone)]
pub struct TransactionMetadata {
    pub id: Uuid,
    pub started_at: DateTime<Utc>,
    pub isolation_level: IsolationLevel,
    pub read_only: bool,
    pub operations_count: usize,
}