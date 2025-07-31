//! Core storage traits that all backends must implement

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::{
    StorageCapabilities, StorageResult, Transaction,
    Query, QueryResult, ChangeStream, HistoryStream,
};

/// Core storage operations that all backends must support
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Associated transaction type for this backend
    type Transaction: Transaction;
    
    /// Initialize storage (create tables, directories, etc.)
    async fn initialize(&self) -> StorageResult<()>;
    
    /// Check if storage is properly configured and accessible
    async fn health_check(&self) -> StorageResult<HealthStatus>;
    
    /// Begin a new transaction
    async fn begin_transaction(&self) -> StorageResult<Self::Transaction>;
    
    /// Get storage capabilities
    fn capabilities(&self) -> StorageCapabilities;
    
    /// Get backend name/type
    fn backend_type(&self) -> &str;
    
    /// Shutdown the backend gracefully
    async fn shutdown(&self) -> StorageResult<()>;
}

/// Health status of storage backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub message: String,
    pub details: HashMap<String, JsonValue>,
    pub checked_at: DateTime<Utc>,
}

/// Optional query capabilities for backends that support it
#[async_trait]
pub trait QueryableBackend: StorageBackend {
    /// Execute a query
    async fn query(&self, query: Query) -> StorageResult<QueryResult>;
    
    /// Create an index
    async fn create_index(&self, index: IndexDefinition) -> StorageResult<()>;
    
    /// List existing indexes
    async fn list_indexes(&self) -> StorageResult<Vec<IndexInfo>>;
    
    /// Drop an index
    async fn drop_index(&self, name: &str) -> StorageResult<()>;
}

/// Optional vector search capabilities
#[async_trait]
pub trait VectorSearchBackend: StorageBackend {
    /// Store embeddings for an entity
    async fn store_embeddings(
        &self,
        entity_id: Uuid,
        embeddings: &[f32],
        metadata: Option<HashMap<String, JsonValue>>,
    ) -> StorageResult<()>;
    
    /// Search by vector similarity
    async fn vector_search(
        &self,
        query_vector: &[f32],
        limit: usize,
        filters: Option<HashMap<String, JsonValue>>,
    ) -> StorageResult<Vec<VectorSearchResult>>;
    
    /// Update embeddings for an entity
    async fn update_embeddings(
        &self,
        entity_id: Uuid,
        embeddings: &[f32],
    ) -> StorageResult<()>;
    
    /// Delete embeddings for an entity
    async fn delete_embeddings(&self, entity_id: Uuid) -> StorageResult<bool>;
}

/// Vector search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResult {
    pub entity_id: Uuid,
    pub distance: f32,
    pub metadata: Option<HashMap<String, JsonValue>>,
}

/// Optional graph capabilities
#[async_trait]
pub trait GraphBackend: StorageBackend {
    /// Store a relationship
    async fn store_relationship(
        &self,
        relationship: &Relationship,
    ) -> StorageResult<()>;
    
    /// Delete a relationship
    async fn delete_relationship(
        &self,
        relationship_id: Uuid,
    ) -> StorageResult<bool>;
    
    /// Traverse relationships
    async fn traverse(
        &self,
        start: Uuid,
        pattern: TraversalPattern,
    ) -> StorageResult<Vec<Path>>;
    
    /// Find shortest path between entities
    async fn shortest_path(
        &self,
        from: Uuid,
        to: Uuid,
        max_depth: Option<usize>,
    ) -> StorageResult<Option<Path>>;
    
    /// Get all relationships for an entity
    async fn get_relationships(
        &self,
        entity_id: Uuid,
        direction: Option<RelationshipDirection>,
        types: Option<Vec<String>>,
    ) -> StorageResult<Vec<Relationship>>;
}

/// Relationship representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: Uuid,
    pub from_entity: Uuid,
    pub to_entity: Uuid,
    pub relationship_type: String,
    pub properties: HashMap<String, JsonValue>,
    pub created_at: DateTime<Utc>,
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
    pub filters: Option<HashMap<String, JsonValue>>,
}

/// Path in graph traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub nodes: Vec<Uuid>,
    pub relationships: Vec<Uuid>,
    pub total_cost: Option<f64>,
}

/// Optional streaming capabilities
#[async_trait]
pub trait StreamingBackend: StorageBackend {
    /// Stream changes in real-time
    async fn stream_changes(&self, filters: Option<ChangeFilters>) -> StorageResult<ChangeStream>;
    
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
    pub entity_types: Option<Vec<String>>,
    pub entity_ids: Option<Vec<Uuid>>,
    pub operation_types: Option<Vec<OperationType>>,
}

/// Operation types for change tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    Create,
    Update,
    Delete,
}

/// Index definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDefinition {
    pub name: String,
    pub entity_type: String,
    pub fields: Vec<IndexField>,
    pub unique: bool,
    pub sparse: bool,
}

/// Field in an index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexField {
    pub path: String,
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
    pub entity_type: String,
    pub fields: Vec<IndexField>,
    pub unique: bool,
    pub sparse: bool,
    pub size: Option<u64>,
    pub created_at: DateTime<Utc>,
}

/// Optional bulk operations for efficiency
#[async_trait]
pub trait BulkOperations: StorageBackend {
    /// Bulk insert entities
    async fn bulk_insert(&self, entities: Vec<StorageEntity>) -> StorageResult<BulkResult>;
    
    /// Bulk update entities
    async fn bulk_update(&self, updates: Vec<BulkUpdate>) -> StorageResult<BulkResult>;
    
    /// Bulk delete entities
    async fn bulk_delete(&self, ids: Vec<Uuid>) -> StorageResult<BulkResult>;
}

/// Entity representation for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEntity {
    pub id: Uuid,
    pub entity_type: String,
    pub data: JsonValue,
    pub binary_data: Option<Vec<u8>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
}

/// Bulk update operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdate {
    pub id: Uuid,
    pub updates: HashMap<String, JsonValue>,
    pub increment_version: bool,
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