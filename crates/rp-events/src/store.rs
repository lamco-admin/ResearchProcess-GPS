use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{DomainEvent, EventError, EventMetadata, Result};

// Explicit pagination constants
const DEFAULT_PAGE_SIZE: i32 = 100;
const DEFAULT_OFFSET: i32 = 0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredEvent {
    pub id: Uuid,
    pub event_id: Uuid,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_version: i64,
    pub event_type: String,
    pub event_version: i32,
    pub event_data: serde_json::Value,
    pub metadata: serde_json::Value,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub actor_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub recorded_at: DateTime<Utc>,
    pub workspace_id: Option<Uuid>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStream {
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub events: Vec<StoredEvent>,
    pub current_version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_version: i64,
    pub snapshot_data: serde_json::Value,
    pub snapshot_metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append_event(
        &self,
        aggregate_id: Uuid,
        aggregate_type: &str,
        expected_version: i64,
        event: DomainEvent,
        metadata: EventMetadata,
    ) -> Result<i64>;
    
    async fn get_events(
        &self,
        aggregate_id: Uuid,
        from_version: Option<i64>,
        to_version: Option<i64>,
    ) -> Result<EventStream>;
    
    async fn get_events_by_type(
        &self,
        event_type: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<StoredEvent>>;
    
    async fn get_aggregate_version(&self, aggregate_id: Uuid) -> Result<i64>;
    
    async fn save_snapshot(&self, snapshot: Snapshot) -> Result<()>;
    
    async fn get_snapshot(
        &self,
        aggregate_id: Uuid,
        max_version: Option<i64>,
    ) -> Result<Option<Snapshot>>;
}

pub struct PostgresEventStore {
    pool: PgPool,
}

impl PostgresEventStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    async fn append_event_in_tx(
        tx: &mut Transaction<'_, Postgres>,
        aggregate_id: Uuid,
        aggregate_type: &str,
        expected_version: i64,
        event: DomainEvent,
        metadata: EventMetadata,
    ) -> Result<i64> {
        let event_data = serde_json::to_value(&event)?;
        let metadata_json = serde_json::to_value(&metadata)?;
        
        let result = sqlx::query!(
            r#"
            SELECT append_event(
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
            ) as version
            "#,
            metadata.event_id,
            aggregate_id,
            aggregate_type,
            expected_version,
            event.event_type(),
            event_data,
            metadata_json,
            metadata.actor_id,
            metadata.occurred_at,
            metadata.correlation_id,
            metadata.causation_id,
            None::<Uuid>, // workspace_id
            &metadata.tags[..]
        )
        .fetch_one(&mut **tx)
        .await?;
        
        Ok(result.version.unwrap_or(0)) // 0 for new aggregates
    }
}

#[async_trait]
impl EventStore for PostgresEventStore {
    async fn append_event(
        &self,
        aggregate_id: Uuid,
        aggregate_type: &str,
        expected_version: i64,
        event: DomainEvent,
        metadata: EventMetadata,
    ) -> Result<i64> {
        let mut tx = self.pool.begin().await?;
        
        let version = Self::append_event_in_tx(
            &mut tx,
            aggregate_id,
            aggregate_type,
            expected_version,
            event,
            metadata,
        ).await?;
        
        tx.commit().await?;
        
        Ok(version)
    }
    
    async fn get_events(
        &self,
        aggregate_id: Uuid,
        from_version: Option<i64>,
        to_version: Option<i64>,
    ) -> Result<EventStream> {
        let from_version = from_version.unwrap_or(1); // Start from version 1
        let to_version = to_version.unwrap_or(i64::MAX); // Unbounded upper limit
        
        let events = sqlx::query_as!(
            StoredEvent,
            r#"
            SELECT 
                id,
                event_id,
                aggregate_id,
                aggregate_type,
                aggregate_version,
                event_type,
                event_version,
                event_data,
                metadata,
                correlation_id,
                causation_id,
                actor_id,
                occurred_at,
                recorded_at,
                workspace_id,
                tags
            FROM events
            WHERE aggregate_id = $1
                AND aggregate_version >= $2
                AND aggregate_version <= $3
            ORDER BY aggregate_version
            "#,
            aggregate_id,
            from_version,
            to_version
        )
        .fetch_all(&self.pool)
        .await?;
        
        if events.is_empty() {
            return Err(EventError::AggregateNotFound { id: aggregate_id });
        }
        
        let aggregate_type = events[0].aggregate_type.clone();
        let current_version = events.last()
            .map(|e| e.aggregate_version)
            .ok_or_else(|| EventError::InvalidEventData {
                message: "Event stream is empty".to_string()
            })?;
        
        Ok(EventStream {
            aggregate_id,
            aggregate_type,
            events,
            current_version,
        })
    }
    
    async fn get_events_by_type(
        &self,
        event_type: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<StoredEvent>> {
        let limit = limit.unwrap_or(DEFAULT_PAGE_SIZE); // Use default page size
        let offset = offset.unwrap_or(DEFAULT_OFFSET); // Default offset for pagination
        
        let events = sqlx::query_as!(
            StoredEvent,
            r#"
            SELECT 
                id,
                event_id,
                aggregate_id,
                aggregate_type,
                aggregate_version,
                event_type,
                event_version,
                event_data,
                metadata,
                correlation_id,
                causation_id,
                actor_id,
                occurred_at,
                recorded_at,
                workspace_id,
                tags
            FROM events
            WHERE event_type = $1
            ORDER BY occurred_at DESC
            LIMIT $2 OFFSET $3
            "#,
            event_type,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(events)
    }
    
    async fn get_aggregate_version(&self, aggregate_id: Uuid) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            SELECT get_aggregate_version($1) as version
            "#,
            aggregate_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.version.unwrap_or(0)) // 0 for new aggregates
    }
    
    async fn save_snapshot(&self, snapshot: Snapshot) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO event_snapshots (
                aggregate_id,
                aggregate_type,
                aggregate_version,
                snapshot_data,
                snapshot_metadata
            ) VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (aggregate_id, aggregate_version) 
            DO UPDATE SET
                snapshot_data = EXCLUDED.snapshot_data,
                snapshot_metadata = EXCLUDED.snapshot_metadata
            "#,
            snapshot.aggregate_id,
            snapshot.aggregate_type,
            snapshot.aggregate_version,
            snapshot.snapshot_data,
            snapshot.snapshot_metadata
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_snapshot(
        &self,
        aggregate_id: Uuid,
        max_version: Option<i64>,
    ) -> Result<Option<Snapshot>> {
        let max_version = max_version.unwrap_or(i64::MAX); // Unbounded version search
        
        let snapshot = sqlx::query_as!(
            Snapshot,
            r#"
            SELECT 
                aggregate_id,
                aggregate_type,
                aggregate_version,
                snapshot_data,
                snapshot_metadata,
                created_at
            FROM event_snapshots
            WHERE aggregate_id = $1
                AND aggregate_version <= $2
            ORDER BY aggregate_version DESC
            LIMIT 1
            "#,
            aggregate_id,
            max_version
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(snapshot)
    }
}

// Event builder for easier event creation
pub struct EventBuilder {
    event_id: Uuid,
    aggregate_id: Uuid,
    aggregate_type: String,
    actor_id: Uuid,
    correlation_id: Option<Uuid>,
    causation_id: Option<Uuid>,
    tags: Vec<String>,
}

impl EventBuilder {
    pub fn new(aggregate_id: Uuid, aggregate_type: String, actor_id: Uuid) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            aggregate_id,
            aggregate_type,
            actor_id,
            correlation_id: None,
            causation_id: None,
            tags: Vec::new(),
        }
    }
    
    pub fn with_correlation_id(mut self, id: Uuid) -> Self {
        self.correlation_id = Some(id);
        self
    }
    
    pub fn with_causation_id(mut self, id: Uuid) -> Self {
        self.causation_id = Some(id);
        self
    }
    
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags.extend(tags);
        self
    }
    
    pub fn build(self, aggregate_version: i64) -> EventMetadata {
        EventMetadata {
            event_id: self.event_id,
            aggregate_id: self.aggregate_id,
            aggregate_type: self.aggregate_type,
            aggregate_version,
            occurred_at: Utc::now(),
            actor_id: self.actor_id,
            correlation_id: self.correlation_id,
            causation_id: self.causation_id,
            tags: self.tags,
        }
    }
}