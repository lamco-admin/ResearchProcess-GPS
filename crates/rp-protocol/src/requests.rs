//! Request message definitions for the ResearchProcess-GPS protocol

use chrono::{DateTime, Utc};
use rp_core::layer3::{EntityType, ModuleConfig};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use uuid::Uuid;

/// Generic entity request wrapper for CRUD operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRequest<T> {
    pub entity_type: EntityType,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Uuid>,
}

/// Create entity request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntityRequest {
    pub entity_type: EntityType,
    pub data: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Uuid>,
}

/// Update entity request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEntityRequest {
    pub entity_type: EntityType,
    pub data: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>, // For optimistic concurrency control
}

/// Query request for listing/searching entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<EntityType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, JsonValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationRequest>,
}

/// Search request with advanced filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub filters: SearchFilters,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortSpec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_facets: Option<bool>,
}

/// Filters for search operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<EntityType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>, // Generic state filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_filters: Option<HashMap<String, JsonValue>>,
}

/// Sort specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortSpec {
    pub field: String,
    #[serde(default = "default_sort_order")]
    pub order: SortOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

fn default_sort_order() -> SortOrder {
    SortOrder::Asc
}

/// Pagination request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>, // For cursor-based pagination
}

fn default_limit() -> u32 {
    20
}

/// Bulk operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationRequest {
    pub operations: Vec<BulkOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "lowercase")]
pub enum BulkOperation {
    Create {
        entity_type: EntityType,
        data: JsonValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        client_id: Option<String>, // Client-provided ID for correlation
    },
    Update {
        id: Uuid,
        data: JsonValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        version: Option<i64>,
    },
    Delete {
        id: Uuid,
        #[serde(skip_serializing_if = "Option::is_none")]
        version: Option<i64>,
    },
}

// Entity-specific requests (for the hybrid approach)

/// Theory-specific requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchTheoryRequest {
    pub branch_name: String,
    pub hypothesis: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_evidence: Option<bool>,
}

/// Person-specific requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergePersonsRequest {
    pub primary_id: Uuid,
    pub secondary_id: Uuid,
    pub merge_strategy: MergeStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeStrategy {
    PreferPrimary,
    PreferSecondary,
    Manual(HashMap<String, MergeChoice>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeChoice {
    Primary,
    Secondary,
    Both,
    Neither,
}

/// Workspace-specific requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteToWorkspaceRequest {
    pub email: String,
    pub role: WorkspaceRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkspaceRole {
    Owner,
    Admin,
    Contributor,
    Viewer,
}

/// Analysis requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartAnalysisRequest {
    pub analysis_type: AnalysisType,
    pub target_entities: Vec<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, JsonValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisType {
    RelationshipDiscovery,
    EvidenceCorrelation,
    TimelineConstruction,
    ConflictDetection,
    Custom(String),
}

/// Subscription request (for real-time updates)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRequest {
    pub subscription_type: SubscriptionType,
    pub params: SubscriptionParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionType {
    Entity,
    EntityType,
    Workspace,
    Query,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<SearchFilters>,
}

/// Module operation requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadModuleRequest {
    pub name: String,
    pub config: ModuleConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeModuleOperationRequest {
    pub module: String,
    pub operation: String,
    pub params: JsonValue,
}