//! Core storage traits that all backends must implement

use async_trait::async_trait;
use meta_model_core::layer1::{Entity, EntityId, Relationship, RelationshipId};
use meta_model_core::layer2::{Process, ProcessId, Product, ProductId};
use meta_model_core::layer3::{Workspace, WorkspaceId};
use crate::{StorageCapabilities, StorageResult, Transaction, Query, QueryResult};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Core storage operations that all backends must support
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Initialize storage (create tables, directories, etc.)
    async fn initialize(&self) -> StorageResult<()>;

    /// Check if storage is properly configured and accessible
    async fn health_check(&self) -> StorageResult<HealthStatus>;

    /// Begin a new transaction
    async fn begin_transaction(&self) -> StorageResult<Transaction>;

    /// Get storage capabilities
    fn capabilities(&self) -> StorageCapabilities;

    /// Get backend name/type
    fn backend_type(&self) -> &str;

    /// Shutdown the backend gracefully
    async fn shutdown(&self) -> StorageResult<()>;

    // Layer 1 operations
    async fn store_entity(&self, entity: &Entity) -> StorageResult<()>;
    async fn get_entity(&self, id: EntityId) -> StorageResult<Option<Entity>>;
    async fn update_entity(&self, entity: &Entity) -> StorageResult<()>;
    async fn delete_entity(&self, id: EntityId) -> StorageResult<bool>;

    async fn store_relationship(&self, relationship: &Relationship) -> StorageResult<()>;
    async fn get_relationship(&self, id: RelationshipId) -> StorageResult<Option<Relationship>>;
    async fn update_relationship(&self, relationship: &Relationship) -> StorageResult<()>;
    async fn delete_relationship(&self, id: RelationshipId) -> StorageResult<bool>;

    // Layer 2 operations
    async fn store_process(&self, process: &Process) -> StorageResult<()>;
    async fn get_process(&self, id: ProcessId) -> StorageResult<Option<Process>>;
    async fn update_process(&self, process: &Process) -> StorageResult<()>;
    async fn delete_process(&self, id: ProcessId) -> StorageResult<bool>;

    async fn store_product(&self, product: &Product) -> StorageResult<()>;
    async fn get_product(&self, id: ProductId) -> StorageResult<Option<Product>>;
    async fn update_product(&self, product: &Product) -> StorageResult<()>;
    async fn delete_product(&self, id: ProductId) -> StorageResult<bool>;

    // Layer 3 operations
    async fn store_workspace(&self, workspace: &Workspace) -> StorageResult<()>;
    async fn get_workspace(&self, id: WorkspaceId) -> StorageResult<Option<Workspace>>;
    async fn update_workspace(&self, workspace: &Workspace) -> StorageResult<()>;
    async fn delete_workspace(&self, id: WorkspaceId) -> StorageResult<bool>;
}

/// Health status of storage backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub message: String,
    pub details: HashMap<String, serde_json::Value>,
    pub checked_at: DateTime<Utc>,
}

/// Optional query capabilities for backends that support it
#[async_trait]
pub trait QueryableBackend: StorageBackend {
    /// Execute a query
    async fn query(&self, query: Query) -> StorageResult<QueryResult>;

    /// Create an index for better query performance
    async fn create_index(&self, index: IndexDefinition) -> StorageResult<()>;

    /// List existing indexes
    async fn list_indexes(&self) -> StorageResult<Vec<IndexInfo>>;

    /// Drop an index
    async fn drop_index(&self, name: &str) -> StorageResult<()>;
}

/// Index definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDefinition {
    pub name: String,
    pub object_type: String, // "entity", "relationship", etc.
    pub fields: Vec<IndexField>,
    pub unique: bool,
    pub sparse: bool,
}

/// Field in an index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexField {
    pub path: String, // JSONPath-like: "properties.name"
    pub order: IndexOrder,
}

/// Index ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexOrder {
    Ascending,
    Descending,
}

/// Information about an existing index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    pub name: String,
    pub object_type: String,
    pub fields: Vec<IndexField>,
    pub unique: bool,
    pub sparse: bool,
    pub size: Option<u64>,
    pub created_at: DateTime<Utc>,
}

/// Optional graph capabilities
#[async_trait]
pub trait GraphBackend: StorageBackend {
    /// Get all relationships for an entity
    async fn get_relationships_for_entity(
        &self,
        entity_id: EntityId,
        direction: Option<RelationshipDirection>,
        types: Option<Vec<String>>,
    ) -> StorageResult<Vec<Relationship>>;

    /// Traverse relationships from a starting entity
    async fn traverse(
        &self,
        start: EntityId,
        pattern: TraversalPattern,
    ) -> StorageResult<Vec<Path>>;

    /// Find shortest path between entities
    async fn shortest_path(
        &self,
        from: EntityId,
        to: EntityId,
        max_depth: Option<usize>,
    ) -> StorageResult<Option<Path>>;
}

/// Relationship direction for queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipDirection {
    Outgoing,
    Incoming,
    Both,
}

/// Graph traversal pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalPattern {
    pub direction: RelationshipDirection,
    pub relationship_types: Option<Vec<String>>,
    pub min_depth: usize,
    pub max_depth: usize,
    pub filters: Option<HashMap<String, serde_json::Value>>,
}

/// Path in graph traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub entities: Vec<EntityId>,
    pub relationships: Vec<RelationshipId>,
    pub total_cost: Option<f64>,
}

/// Optional streaming capabilities
#[async_trait]
pub trait StreamingBackend: StorageBackend {
    /// Stream changes in real-time
    async fn stream_changes(
        &self,
        filters: Option<ChangeFilters>,
    ) -> StorageResult<ChangeStream>;

    /// Replay history from a point in time
    async fn replay_history(
        &self,
        from: DateTime<Utc>,
        to: Option<DateTime<Utc>>,
    ) -> StorageResult<HistoryStream>;
}

/// Change filters for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeFilters {
    pub object_types: Option<Vec<String>>,
    pub object_ids: Option<Vec<Uuid>>,
    pub operation_types: Option<Vec<OperationType>>,
}

/// Operation types for change tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    Create,
    Update,
    Delete,
}

/// Stream of changes
pub type ChangeStream = Box<dyn futures::Stream<Item = StorageResult<Change>> + Send + Unpin>;

/// Stream of historical events
pub type HistoryStream = Box<dyn futures::Stream<Item = StorageResult<HistoricalEvent>> + Send + Unpin>;

/// A change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub operation: OperationType,
    pub object_type: String,
    pub object_id: Uuid,
    pub data: Option<serde_json::Value>,
}

/// A historical event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub data: serde_json::Value,
}

/// Optional bulk operations for efficiency
#[async_trait]
pub trait BulkOperations: StorageBackend {
    /// Bulk insert entities
    async fn bulk_insert_entities(&self, entities: Vec<Entity>) -> StorageResult<BulkResult>;

    /// Bulk insert relationships
    async fn bulk_insert_relationships(&self, relationships: Vec<Relationship>) -> StorageResult<BulkResult>;

    /// Bulk update entities
    async fn bulk_update_entities(&self, entities: Vec<Entity>) -> StorageResult<BulkResult>;

    /// Bulk delete by IDs
    async fn bulk_delete(&self, object_type: &str, ids: Vec<Uuid>) -> StorageResult<BulkResult>;
}

/// Result of bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkResult {
    pub successful: usize,
    pub failed: usize,
    pub errors: Vec<BulkError>,
}

/// Error in bulk operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkError {
    pub index: usize,
    pub id: Option<Uuid>,
    pub error: String,
}

/// Entity representation for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEntity {
    pub id: Uuid,
    pub object_type: String,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
}