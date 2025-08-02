//! Storage capabilities

use serde::{Serialize, Deserialize};

/// Capabilities of a storage backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    /// Can perform queries
    pub queryable: bool,

    /// Supports graph operations
    pub graph: bool,

    /// Supports streaming changes
    pub streaming: bool,

    /// Supports bulk operations
    pub bulk_operations: bool,

    /// Supports transactions
    pub transactions: bool,

    /// Supports full-text search
    pub full_text_search: bool,

    /// Supports vector/embedding search
    pub vector_search: bool,

    /// Maximum entity size in bytes
    pub max_entity_size: Option<usize>,

    /// Maximum query complexity
    pub max_query_complexity: Option<usize>,

    /// Is read-only
    pub read_only: bool,

    /// Supports offline mode
    pub offline_capable: bool,

    /// Custom capabilities
    pub custom: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for StorageCapabilities {
    fn default() -> Self {
        StorageCapabilities {
            queryable: true,
            graph: false,
            streaming: false,
            bulk_operations: false,
            transactions: false,
            full_text_search: false,
            vector_search: false,
            max_entity_size: None,
            max_query_complexity: None,
            read_only: false,
            offline_capable: false,
            custom: std::collections::HashMap::new(),
        }
    }
}

impl StorageCapabilities {
    /// Create capabilities for a full-featured backend (like PostgreSQL)
    pub fn full_featured() -> Self {
        StorageCapabilities {
            queryable: true,
            graph: true,
            streaming: true,
            bulk_operations: true,
            transactions: true,
            full_text_search: true,
            vector_search: false, // Can be enabled with pgvector
            max_entity_size: Some(16 * 1024 * 1024), // 16MB
            max_query_complexity: Some(1000),
            read_only: false,
            offline_capable: false,
            custom: std::collections::HashMap::new(),
        }
    }

    /// Create capabilities for a read-only backend
    pub fn read_only() -> Self {
        StorageCapabilities {
            queryable: true,
            read_only: true,
            ..Default::default()
        }
    }

    /// Create capabilities for a simple key-value backend
    pub fn key_value() -> Self {
        StorageCapabilities {
            queryable: false,
            transactions: true,
            ..Default::default()
        }
    }
}