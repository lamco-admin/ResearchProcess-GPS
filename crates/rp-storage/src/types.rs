//! Common types used across storage implementations

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Workspace information for multi-tenancy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub settings: serde_json::Value,
}

/// Change event for CDC (Change Data Capture)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEvent {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: String,
    pub operation: ChangeOperation,
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
    pub workspace_id: Option<Uuid>,
    pub before: Option<serde_json::Value>,
    pub after: Option<serde_json::Value>,
    pub metadata: serde_json::Value,
}

/// Type of change operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeOperation {
    Create,
    Update,
    Delete,
    StateTransition,
}

/// Module registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub config: serde_json::Value,
    pub installed_at: DateTime<Utc>,
    pub installed_by: Uuid,
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_entities: usize,
    pub entities_by_type: std::collections::HashMap<String, usize>,
    pub total_size_bytes: u64,
    pub last_updated: DateTime<Utc>,
}

/// Backup information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub size_bytes: u64,
    pub entity_count: usize,
    pub location: String,
    pub metadata: serde_json::Value,
}

/// Migration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatus {
    pub version: String,
    pub applied_at: DateTime<Utc>,
    pub applied_by: String,
    pub success: bool,
    pub error: Option<String>,
}