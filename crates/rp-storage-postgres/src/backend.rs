//! PostgreSQL storage backend implementation

use async_trait::async_trait;
use tracing::{debug, info, instrument};
use chrono::Utc;
use std::collections::HashMap;

use rp_storage::{
    StorageBackend, StorageCapabilities, StorageResult, StorageError,
    QueryCapabilities, CompressionSupport, HealthStatus,
};

use crate::{
    ConnectionPool, PostgresConfig, PostgresTransaction,
    PostgresError, PostgresResult,
};

/// PostgreSQL storage backend
pub struct PostgresBackend {
    pool: ConnectionPool,
    capabilities: StorageCapabilities,
}

impl PostgresBackend {
    /// Create a new PostgreSQL backend
    pub async fn new(config: PostgresConfig) -> PostgresResult<Self> {
        info!("Initializing PostgreSQL backend");
        
        let pool = ConnectionPool::new(config.clone()).await?;
        
        // Check extensions
        let ext_status = pool.check_extensions().await?;
        
        // Build capabilities based on available extensions
        let capabilities = StorageCapabilities {
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
            vector_search: ext_status.vector && config.enable_vector,
            graph_operations: ext_status.graph && config.enable_graph,
            real_time_updates: config.enable_notifications,
            full_text_search: true,
            binary_storage: true,
            compression: CompressionSupport::Advanced,
            max_entity_size: Some(1_000_000_000), // 1GB
            concurrent_connections: Some(config.max_connections as usize),
            bulk_operations: true,
            change_data_capture: true,
            version_history: true,
            encryption_at_rest: false, // Depends on PostgreSQL setup
        };
        
        info!(
            "PostgreSQL backend initialized with capabilities: vector={}, graph={}",
            capabilities.vector_search, capabilities.graph_operations
        );
        
        Ok(Self { pool, capabilities })
    }
    
    /// Get the connection pool
    pub fn pool(&self) -> &ConnectionPool {
        &self.pool
    }
}

#[async_trait]
impl StorageBackend for PostgresBackend {
    type Transaction = PostgresTransaction;
    
    #[instrument(skip(self))]
    async fn initialize(&self) -> StorageResult<()> {
        debug!("Initializing PostgreSQL storage");
        
        // Check if tables exist
        let mut conn = self.pool.pool().acquire().await
            .map_err(PostgresError::from)?;
        
        let table_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name = 'entities')"
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(PostgresError::from)?;
        
        if !table_exists {
            return Err(StorageError::NotInitialized);
        }
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    async fn health_check(&self) -> StorageResult<HealthStatus> {
        debug!("Performing health check");
        
        let start = std::time::Instant::now();
        let mut details = HashMap::new();
        
        // Check connection pool
        match self.pool.health_check().await {
            Ok(_) => {
                details.insert("connection_pool".to_string(), serde_json::json!("healthy"));
            }
            Err(e) => {
                return Ok(HealthStatus {
                    healthy: false,
                    message: format!("Connection pool unhealthy: {}", e),
                    details,
                    checked_at: Utc::now(),
                });
            }
        }
        
        // Get pool stats
        let stats = self.pool.stats();
        details.insert("pool_stats".to_string(), serde_json::json!({
            "size": stats.size,
            "idle": stats.idle,
            "max_connections": stats.max_connections,
        }));
        
        // Check response time
        let elapsed = start.elapsed();
        details.insert("response_time_ms".to_string(), serde_json::json!(elapsed.as_millis()));
        
        Ok(HealthStatus {
            healthy: true,
            message: "PostgreSQL backend is healthy".to_string(),
            details,
            checked_at: Utc::now(),
        })
    }
    
    #[instrument(skip(self))]
    async fn begin_transaction(&self) -> StorageResult<Self::Transaction> {
        debug!("Beginning new transaction");
        
        let tx = self.pool.pool().begin().await
            .map_err(PostgresError::from)?;
        
        Ok(PostgresTransaction::new(tx))
    }
    
    fn capabilities(&self) -> StorageCapabilities {
        self.capabilities.clone()
    }
    
    fn backend_type(&self) -> &str {
        "postgresql"
    }
    
    #[instrument(skip(self))]
    async fn shutdown(&self) -> StorageResult<()> {
        info!("Shutting down PostgreSQL backend");
        self.pool.close().await;
        Ok(())
    }
}

// Implement DynStorageBackend for the factory pattern
#[async_trait]
impl rp_storage::DynStorageBackend for PostgresBackend {
    async fn initialize(&self) -> StorageResult<()> {
        <Self as StorageBackend>::initialize(self).await
    }
    
    async fn health_check(&self) -> StorageResult<()> {
        <Self as StorageBackend>::health_check(self).await.map(|_| ())
    }
    
    fn backend_type(&self) -> &str {
        <Self as StorageBackend>::backend_type(self)
    }
}

// Additional trait implementations will be in separate modules:
// - QueryableBackend in query.rs
// - VectorSearchBackend in vector.rs
// - GraphBackend in graph.rs
// - StreamingBackend in streaming.rs