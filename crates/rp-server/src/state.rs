use rp_storage_postgres::{PostgresBackend, PostgresConfig};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::notify::EventNotification;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<PostgresBackend>,
    pub server_id: Uuid,
    pub event_tx: broadcast::Sender<EventNotification>,
    pub pg_pool: PgPool,
}

impl AppState {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let config = PostgresConfig::from_url(database_url)?;
        
        let storage = PostgresBackend::new(config.clone()).await?;
        let server_id = Uuid::now_v7();
        
        // Create a PgPool for notifications
        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        // Create broadcast channel for events
        let (event_tx, _) = broadcast::channel(1000);

        Ok(Self {
            storage: Arc::new(storage),
            server_id,
            event_tx,
            pg_pool,
        })
    }
}