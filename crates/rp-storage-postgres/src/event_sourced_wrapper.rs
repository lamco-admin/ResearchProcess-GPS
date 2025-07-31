//! Event-sourced transaction wrapper with event publishing

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tracing::warn;

use rp_storage::{
    Transaction as StorageTrait, StorageResult,
    StorageEntity, VersionData,
};
use rp_events::{DomainEvent, PostgresEventStore, EventStore};

use crate::PostgresTransaction;

/// Event-sourced PostgreSQL transaction wrapper
pub struct EventSourcedTransaction {
    inner: PostgresTransaction,
    event_store: PostgresEventStore,
    actor_id: Uuid,
}

impl EventSourcedTransaction {
    /// Create a new event-sourced transaction
    pub fn new(
        pool: PgPool,
        tx: SqlxTransaction<'static, Postgres>,
        actor_id: Uuid,
    ) -> Self {
        let event_store = PostgresEventStore::new(pool);
        let inner = PostgresTransaction::new(tx);
        Self {
            inner,
            event_store,
            actor_id,
        }
    }
    
    /// Publish event after successful operation (best-effort)
    async fn publish_event(&self, event: DomainEvent, metadata: rp_events::EventMetadata) {
        let aggregate_id = metadata.aggregate_id;
        let aggregate_type = metadata.aggregate_type.clone();
        
        if let Err(e) = self.event_store.append_event(
            aggregate_id,
            &aggregate_type,
            -1, // Use -1 to skip version check for now
            event,
            metadata,
        ).await {
            warn!("Failed to publish event: {}", e);
        }
    }
}

#[async_trait]
impl StorageTrait for EventSourcedTransaction {
    async fn put_entity(&mut self, entity: &StorageEntity) -> StorageResult<()> {
        // First, perform the CRUD operation
        self.inner.put_entity(entity).await?;
        
        // Then publish event (best-effort)
        let (event, metadata) = DomainEvent::entity_created(
            entity.id,
            &entity.entity_type,
            entity.data.clone(),
            self.actor_id,
        );
        self.publish_event(event, metadata).await;
        
        Ok(())
    }
    
    async fn get_entity(&mut self, id: Uuid) -> StorageResult<Option<StorageEntity>> {
        self.inner.get_entity(id).await
    }
    
    async fn get_entities(&mut self, ids: &[Uuid]) -> StorageResult<Vec<StorageEntity>> {
        self.inner.get_entities(ids).await
    }
    
    async fn update_entity(&mut self, id: Uuid, updates: HashMap<String, JsonValue>) -> StorageResult<bool> {
        // Get current entity to know its type and version
        let entity = self.inner.get_entity(id).await?;
        if let Some(entity) = entity {
            // Perform the update
            let updated = self.inner.update_entity(id, updates.clone()).await?;
            
            if updated {
                // Publish event with incremented version
                let (event, metadata) = DomainEvent::entity_updated(
                    id,
                    &entity.entity_type,
                    updates,
                    self.actor_id,
                    (entity.version + 1) as i64,
                );
                self.publish_event(event, metadata).await;
            }
            
            Ok(updated)
        } else {
            Ok(false)
        }
    }
    
    async fn delete_entity(&mut self, id: Uuid) -> StorageResult<bool> {
        // Get current entity to know its type and version
        let entity = self.inner.get_entity(id).await?;
        if let Some(entity) = entity {
            // Perform the deletion
            let deleted = self.inner.delete_entity(id).await?;
            
            if deleted {
                // Publish event
                let (event, metadata) = DomainEvent::entity_deleted(
                    id,
                    &entity.entity_type,
                    self.actor_id,
                    (entity.version + 1) as i64,
                );
                self.publish_event(event, metadata).await;
            }
            
            Ok(deleted)
        } else {
            Ok(false)
        }
    }
    
    async fn exists(&mut self, id: Uuid) -> StorageResult<bool> {
        self.inner.exists(id).await
    }
    
    async fn list_by_type(
        &mut self,
        entity_type: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> StorageResult<Vec<StorageEntity>> {
        self.inner.list_by_type(entity_type, limit, offset).await
    }
    
    async fn put_relationship(
        &mut self,
        from: Uuid,
        to: Uuid,
        rel_type: &str,
        properties: Option<HashMap<String, JsonValue>>,
    ) -> StorageResult<Uuid> {
        self.inner.put_relationship(from, to, rel_type, properties).await
    }
    
    async fn delete_relationship(&mut self, rel_id: Uuid) -> StorageResult<bool> {
        self.inner.delete_relationship(rel_id).await
    }
    
    async fn add_version(&mut self, entity_id: Uuid, version_data: VersionData) -> StorageResult<()> {
        self.inner.add_version(entity_id, version_data).await
    }
    
    async fn commit(self) -> StorageResult<()> {
        self.inner.commit().await
    }
    
    async fn rollback(self) -> StorageResult<()> {
        self.inner.rollback().await
    }
    
    fn transaction_id(&self) -> Uuid {
        self.inner.transaction_id()
    }
    
    fn is_active(&self) -> bool {
        self.inner.is_active()
    }
}