//! Connection pool management

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::{PostgresConfig, PostgresResult};

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub size: u32,
    pub idle: u32,
    pub max_connections: u32,
    pub min_connections: u32,
}

/// Extension availability status
#[derive(Debug, Default)]
pub struct ExtensionStatus {
    pub vector: bool,
    pub graph: bool,
}

/// PostgreSQL connection pool wrapper
#[derive(Clone)]
pub struct ConnectionPool {
    pool: Arc<PgPool>,
    config: Arc<PostgresConfig>,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub async fn new(config: PostgresConfig) -> PostgresResult<Self> {
        info!("Creating PostgreSQL connection pool");
        
        let mut pool_options = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.connect_timeout);
            
        if let Some(idle_timeout) = config.idle_timeout {
            pool_options = pool_options.idle_timeout(idle_timeout);
        }
        
        if let Some(max_lifetime) = config.max_lifetime {
            pool_options = pool_options.max_lifetime(max_lifetime);
        }
        
        let pool = pool_options
            .connect_with(config.to_sqlx_options())
            .await?;
        
        info!(
            "Connection pool created with {} connections",
            pool.size()
        );
        
        Ok(Self {
            pool: Arc::new(pool),
            config: Arc::new(config),
        })
    }
    
    /// Get the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
    
    /// Get the configuration
    pub fn config(&self) -> &PostgresConfig {
        &self.config
    }
    
    /// Check if the pool is healthy
    pub async fn health_check(&self) -> PostgresResult<()> {
        debug!("Performing connection pool health check");
        
        // Try to acquire a connection
        let mut conn = self.pool.acquire().await?;
        
        // Run a simple query
        sqlx::query("SELECT 1")
            .execute(&mut *conn)
            .await?;
        
        debug!("Connection pool health check passed");
        Ok(())
    }
    
    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            size: self.pool.size(),
            idle: self.pool.num_idle() as u32,
            max_connections: self.config.max_connections,
            min_connections: self.config.min_connections,
        }
    }
    
    /// Close the pool gracefully
    pub async fn close(&self) {
        info!("Closing connection pool");
        self.pool.close().await;
    }
    
    /// Check if required extensions are available
    pub async fn check_extensions(&self) -> PostgresResult<ExtensionStatus> {
        let mut conn = self.pool.acquire().await?;
        
        let mut status = ExtensionStatus::default();
        
        // Check for pgvector
        if self.config.enable_vector {
            let result = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM pg_extension WHERE extname = 'vector')"
            )
            .fetch_one(&mut *conn)
            .await?;
            
            status.vector = result;
            if !result {
                warn!("pgvector extension not found but was requested");
            }
        }
        
        // Check for Apache AGE
        if self.config.enable_graph {
            let result = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM pg_extension WHERE extname = 'age')"
            )
            .fetch_one(&mut *conn)
            .await?;
            
            status.graph = result;
            if !result {
                warn!("Apache AGE extension not found but was requested");
            }
        }
        
        // Always check for required extensions
        // Check for uuid-ossp and btree_gin are not stored but they're required extensions
        let _uuid_ext = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM pg_extension WHERE extname = 'uuid-ossp')"
        )
        .fetch_one(&mut *conn)
        .await?;
        
        let _btree_gin_ext = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM pg_extension WHERE extname = 'btree_gin')"
        )
        .fetch_one(&mut *conn)
        .await?;
        
        Ok(status)
    }
}

