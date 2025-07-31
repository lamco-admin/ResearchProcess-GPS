//! Response message definitions for the ResearchProcess-GPS protocol

use chrono::{DateTime, Utc};
use rp_core::layer3::EntityType;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use uuid::Uuid;

/// Generic entity response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityResponse {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub version: i64,
    pub data: JsonValue,
    pub metadata: EntityMetadata,
}

/// Entity metadata included in responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// List response with pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// Search response with facets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<EntityResponse>,
    pub total: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facets: Option<SearchFacets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<HashMap<Uuid, Vec<Highlight>>>,
}

/// Search facets for filtering
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchFacets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<HashMap<String, u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<HashMap<Uuid, u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<HashMap<String, u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_ranges: Option<DateRangeFacets>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeFacets {
    pub last_hour: u64,
    pub last_day: u64,
    pub last_week: u64,
    pub last_month: u64,
    pub last_year: u64,
    pub older: u64,
}

/// Text highlight in search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    pub field: String,
    pub snippet: String,
    pub positions: Vec<(usize, usize)>,
}

/// Bulk operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResponse {
    pub results: Vec<BulkOperationResult>,
    pub successful: u32,
    pub failed: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResult {
    pub index: usize,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

/// Error detail for failed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<JsonValue>,
}

// Entity-specific responses

/// Theory branch response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchTheoryResponse {
    pub original_id: Uuid,
    pub branch_id: Uuid,
    pub branch_name: String,
    pub created_at: DateTime<Utc>,
}

/// Person timeline response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonTimelineResponse {
    pub person_id: Uuid,
    pub events: Vec<TimelineEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub event_type: String,
    pub date: Option<String>, // Flexible date format
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// Person relationships response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRelationshipsResponse {
    pub person_id: Uuid,
    pub relationships: Vec<RelationshipInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipInfo {
    pub relationship_id: Uuid,
    pub relationship_type: String,
    pub related_person_id: Uuid,
    pub related_person_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_ids: Option<Vec<Uuid>>,
}

/// Workspace members response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMembersResponse {
    pub workspace_id: Uuid,
    pub members: Vec<WorkspaceMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub joined_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active: Option<DateTime<Utc>>,
}

/// Analysis status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStatusResponse {
    pub analysis_id: Uuid,
    pub status: AnalysisStatus,
    pub progress: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<JsonValue>,
    pub started_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnalysisStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Compliance status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatusResponse {
    pub entity_id: Uuid,
    pub compliance_results: HashMap<String, ComplianceResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub standard: String,
    pub version: String,
    pub compliant: bool,
    pub score: f32,
    pub items: Vec<ComplianceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceItem {
    pub requirement: String,
    pub status: ComplianceItemStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComplianceItemStatus {
    Met,
    NotMet,
    Partial,
    NotApplicable,
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub version: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, JsonValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Rate limit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub limit: u32,
    pub remaining: u32,
    pub reset_at: DateTime<Utc>,
}