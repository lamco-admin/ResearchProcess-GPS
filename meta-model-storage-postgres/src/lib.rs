//! PostgreSQL storage backend for meta-model

mod backend;
mod query_builder;
mod migrations;
mod error;

pub use backend::PostgresBackend;
pub use error::{PostgresError, PostgresResult};

/// Create a new PostgreSQL backend
pub async fn create_backend(connection_string: &str) -> PostgresResult<PostgresBackend> {
    PostgresBackend::new(connection_string).await
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_backend_creation() {
        // Test will use testcontainers for isolated PostgreSQL instance
    }
}