//! PostgreSQL storage backend for ResearchProcess-GPS
//! 
//! This crate provides a high-performance PostgreSQL implementation of the
//! storage abstraction layer with support for:
//! - Hybrid JSONB + binary storage
//! - Vector search (pgvector)
//! - Graph operations (Apache AGE)
//! - Change data capture
//! - Real-time notifications

pub mod backend;
pub mod connection;
pub mod transaction;
pub mod query;
pub mod vector;
pub mod error;
pub mod config;

pub use backend::PostgresBackend;
pub use connection::ConnectionPool;
pub use transaction::PostgresTransaction;
pub use error::{PostgresError, PostgresResult};
pub use config::PostgresConfig;

/// Factory for creating PostgreSQL backends
pub struct PostgresBackendFactory;

#[async_trait::async_trait]
impl rp_storage::BackendFactory for PostgresBackendFactory {
    async fn create(&self, url: &str) -> rp_storage::StorageResult<Box<dyn rp_storage::StorageBackend>> {
        let config = PostgresConfig::from_url(url)?;
        let backend = PostgresBackend::new(config).await?;
        Ok(Box::new(backend))
    }
    
    fn scheme(&self) -> &str {
        "postgres"
    }
    
    fn validate_url(&self, url: &str) -> rp_storage::StorageResult<()> {
        PostgresConfig::from_url(url)?;
        Ok(())
    }
}