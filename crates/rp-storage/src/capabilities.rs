//! Storage capability declarations

use serde::{Serialize, Deserialize};

/// Storage capabilities declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    /// Basic transaction support
    pub transactions: bool,
    
    /// Query capabilities
    pub queries: QueryCapabilities,
    
    /// Vector search support
    pub vector_search: bool,
    
    /// Graph operations support
    pub graph_operations: bool,
    
    /// Real-time update streaming
    pub real_time_updates: bool,
    
    /// Full-text search support
    pub full_text_search: bool,
    
    /// Binary data storage support
    pub binary_storage: bool,
    
    /// Compression support
    pub compression: CompressionSupport,
    
    /// Maximum size of a single entity (in bytes)
    pub max_entity_size: Option<usize>,
    
    /// Maximum concurrent connections
    pub concurrent_connections: Option<usize>,
    
    /// Bulk operations support
    pub bulk_operations: bool,
    
    /// Change data capture support
    pub change_data_capture: bool,
    
    /// Version history support
    pub version_history: bool,
    
    /// Encryption at rest
    pub encryption_at_rest: bool,
}

impl Default for StorageCapabilities {
    fn default() -> Self {
        Self {
            transactions: false,
            queries: QueryCapabilities::default(),
            vector_search: false,
            graph_operations: false,
            real_time_updates: false,
            full_text_search: false,
            binary_storage: false,
            compression: CompressionSupport::None,
            max_entity_size: None,
            concurrent_connections: None,
            bulk_operations: false,
            change_data_capture: false,
            version_history: false,
            encryption_at_rest: false,
        }
    }
}

/// Query capability details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCapabilities {
    /// Basic equality filters
    pub basic_filters: bool,
    
    /// JSON path queries (e.g., data->>'field')
    pub json_path: bool,
    
    /// Aggregation functions (COUNT, SUM, etc.)
    pub aggregations: bool,
    
    /// Join operations
    pub joins: bool,
    
    /// Recursive CTEs
    pub recursive_ctes: bool,
    
    /// Sorting support
    pub sorting: bool,
    
    /// Pagination support
    pub pagination: bool,
    
    /// Distinct queries
    pub distinct: bool,
    
    /// Grouping support
    pub grouping: bool,
    
    /// Window functions
    pub window_functions: bool,
}

impl Default for QueryCapabilities {
    fn default() -> Self {
        Self {
            basic_filters: true,
            json_path: false,
            aggregations: false,
            joins: false,
            recursive_ctes: false,
            sorting: true,
            pagination: true,
            distinct: false,
            grouping: false,
            window_functions: false,
        }
    }
}

/// Compression support levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionSupport {
    /// No compression
    None,
    /// Basic compression (gzip)
    Basic,
    /// Advanced compression (zstd, lz4)
    Advanced,
    /// Custom compression algorithms
    Custom,
}

impl StorageCapabilities {
    /// Create capabilities for a minimal filesystem backend
    pub fn filesystem() -> Self {
        Self {
            transactions: false,
            queries: QueryCapabilities {
                basic_filters: true,
                sorting: true,
                pagination: true,
                ..QueryCapabilities::default()
            },
            binary_storage: true,
            compression: CompressionSupport::Basic,
            max_entity_size: Some(10 * 1024 * 1024), // 10MB
            concurrent_connections: Some(1),
            ..Self::default()
        }
    }
    
    /// Create capabilities for SQLite backend
    pub fn sqlite() -> Self {
        Self {
            transactions: true,
            queries: QueryCapabilities {
                basic_filters: true,
                json_path: true,
                aggregations: true,
                sorting: true,
                pagination: true,
                distinct: true,
                grouping: true,
                ..QueryCapabilities::default()
            },
            full_text_search: true,
            binary_storage: true,
            compression: CompressionSupport::None,
            max_entity_size: Some(1_000_000_000), // 1GB
            concurrent_connections: Some(1), // SQLite limitation
            ..Self::default()
        }
    }
    
    /// Create capabilities for PostgreSQL backend
    pub fn postgresql() -> Self {
        Self {
            transactions: true,
            queries: QueryCapabilities {
                basic_filters: true,
                json_path: true,
                aggregations: true,
                joins: true,
                recursive_ctes: true,
                sorting: true,
                pagination: true,
                distinct: true,
                grouping: true,
                window_functions: true,
            },
            vector_search: false, // Can be enabled with pgvector
            graph_operations: false, // Can be enabled with Apache AGE
            real_time_updates: true, // LISTEN/NOTIFY
            full_text_search: true,
            binary_storage: true,
            compression: CompressionSupport::Advanced,
            max_entity_size: Some(1_000_000_000), // 1GB
            concurrent_connections: Some(100),
            bulk_operations: true,
            change_data_capture: true,
            version_history: true,
            encryption_at_rest: false, // Depends on setup
        }
    }
    
    /// Create capabilities for Git backend
    pub fn git() -> Self {
        Self {
            transactions: false, // Git commits are atomic but different
            queries: QueryCapabilities {
                basic_filters: true,
                sorting: true,
                pagination: true,
                ..QueryCapabilities::default()
            },
            binary_storage: true,
            compression: CompressionSupport::Advanced,
            max_entity_size: Some(100 * 1024 * 1024), // 100MB (Git LFS for larger)
            concurrent_connections: Some(1),
            version_history: true, // Git's primary feature!
            ..Self::default()
        }
    }
}

/// Module can declare required capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredCapabilities {
    /// Must-have capabilities
    pub required: Vec<Capability>,
    
    /// Nice-to-have capabilities
    pub optional: Vec<Capability>,
}

/// Individual capability requirement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Capability {
    Transactions,
    BasicQueries,
    JsonPathQueries,
    Aggregations,
    Joins,
    RecursiveCtes,
    VectorSearch,
    GraphOperations,
    RealTimeUpdates,
    FullTextSearch,
    BinaryStorage,
    Compression,
    BulkOperations,
    ChangeDataCapture,
    VersionHistory,
    EncryptionAtRest,
}

impl StorageCapabilities {
    /// Check if this backend satisfies the required capabilities
    pub fn satisfies(&self, required: &RequiredCapabilities) -> Result<(), Vec<Capability>> {
        let mut missing = Vec::new();
        
        for cap in &required.required {
            if !self.has_capability(cap) {
                missing.push(cap.clone());
            }
        }
        
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
    
    /// Check if a specific capability is available
    pub fn has_capability(&self, cap: &Capability) -> bool {
        match cap {
            Capability::Transactions => self.transactions,
            Capability::BasicQueries => self.queries.basic_filters,
            Capability::JsonPathQueries => self.queries.json_path,
            Capability::Aggregations => self.queries.aggregations,
            Capability::Joins => self.queries.joins,
            Capability::RecursiveCtes => self.queries.recursive_ctes,
            Capability::VectorSearch => self.vector_search,
            Capability::GraphOperations => self.graph_operations,
            Capability::RealTimeUpdates => self.real_time_updates,
            Capability::FullTextSearch => self.full_text_search,
            Capability::BinaryStorage => self.binary_storage,
            Capability::Compression => self.compression != CompressionSupport::None,
            Capability::BulkOperations => self.bulk_operations,
            Capability::ChangeDataCapture => self.change_data_capture,
            Capability::VersionHistory => self.version_history,
            Capability::EncryptionAtRest => self.encryption_at_rest,
        }
    }
}