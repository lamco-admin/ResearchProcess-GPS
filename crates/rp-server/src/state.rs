use rp_storage_postgres::{PostgresBackend, PostgresConfig};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<PostgresBackend>,
    pub server_id: Uuid,
}

impl AppState {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let config = PostgresConfig::from_url(database_url)?;
        
        let storage = PostgresBackend::new(config).await?;
        let server_id = Uuid::now_v7();

        Ok(Self {
            storage: Arc::new(storage),
            server_id,
        })
    }
}