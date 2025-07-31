use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction, Row};
use uuid::Uuid;

use crate::{DomainEvent, EventError, EventMetadata, Result};

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
        
        let result: (Option<i64>,) = sqlx::query_as(
            r#"
            SELECT append_event(
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
            ) as version
            "#,
        )
        .bind(metadata.event_id)
        .bind(aggregate_id)
        .bind(aggregate_type)
        .bind(expected_version)
        .bind(event.event_type())
        .bind(event_data)
        .bind(metadata_json)
        .bind(metadata.actor_id)
        .bind(metadata.occurred_at)
        .bind(metadata.correlation_id)
        .bind(metadata.causation_id)
        .bind(None::<Uuid>) // workspace_id
        .bind(&metadata.tags)
        .fetch_one(&mut **tx)
        .await?;
        
        Ok(result.0.unwrap_or(0))
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
        let from_version = from_version.unwrap_or(1);
        let to_version = to_version.unwrap_or(i64::MAX);
        
        let rows = sqlx::query(
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
        )
        .bind(aggregate_id)
        .bind(from_version)
        .bind(to_version)
        .fetch_all(&self.pool)
        .await?;
        
        if rows.is_empty() {
            return Err(EventError::AggregateNotFound { id: aggregate_id });
        }
        
        let events: Vec<StoredEvent> = rows.into_iter()
            .map(|row| StoredEvent {
                id: row.get("id"),
                event_id: row.get("event_id"),
                aggregate_id: row.get("aggregate_id"),
                aggregate_type: row.get("aggregate_type"),
                aggregate_version: row.get("aggregate_version"),
                event_type: row.get("event_type"),
                event_version: row.get("event_version"),
                event_data: row.get("event_data"),
                metadata: row.get("metadata"),
                correlation_id: row.get("correlation_id"),
                causation_id: row.get("causation_id"),
                actor_id: row.get("actor_id"),
                occurred_at: row.get("occurred_at"),
                recorded_at: row.get("recorded_at"),
                workspace_id: row.get("workspace_id"),
                tags: row.get("tags"),
            })
            .collect();
        
        let aggregate_type = events[0].aggregate_type.clone();
        let current_version = events.last().map(|e| e.aggregate_version).unwrap_or(0);
        
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
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        
        let rows = sqlx::query(
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
        )
        .bind(event_type)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await?;
        
        let events: Vec<StoredEvent> = rows.into_iter()
            .map(|row| StoredEvent {
                id: row.get("id"),
                event_id: row.get("event_id"),
                aggregate_id: row.get("aggregate_id"),
                aggregate_type: row.get("aggregate_type"),
                aggregate_version: row.get("aggregate_version"),
                event_type: row.get("event_type"),
                event_version: row.get("event_version"),
                event_data: row.get("event_data"),
                metadata: row.get("metadata"),
                correlation_id: row.get("correlation_id"),
                causation_id: row.get("causation_id"),
                actor_id: row.get("actor_id"),
                occurred_at: row.get("occurred_at"),
                recorded_at: row.get("recorded_at"),
                workspace_id: row.get("workspace_id"),
                tags: row.get("tags"),
            })
            .collect();
        
        Ok(events)
    }
    
    async fn get_aggregate_version(&self, aggregate_id: Uuid) -> Result<i64> {
        let result: (Option<i64>,) = sqlx::query_as(
            r#"
            SELECT get_aggregate_version($1) as version
            "#,
        )
        .bind(aggregate_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.0.unwrap_or(0))
    }
    
    async fn save_snapshot(&self, snapshot: Snapshot) -> Result<()> {
        sqlx::query(
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
        )
        .bind(snapshot.aggregate_id)
        .bind(snapshot.aggregate_type)
        .bind(snapshot.aggregate_version)
        .bind(snapshot.snapshot_data)
        .bind(snapshot.snapshot_metadata)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_snapshot(
        &self,
        aggregate_id: Uuid,
        max_version: Option<i64>,
    ) -> Result<Option<Snapshot>> {
        let max_version = max_version.unwrap_or(i64::MAX);
        
        let row = sqlx::query(
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
        )
        .bind(aggregate_id)
        .bind(max_version)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row.map(|row| Snapshot {
            aggregate_id: row.get("aggregate_id"),
            aggregate_type: row.get("aggregate_type"),
            aggregate_version: row.get("aggregate_version"),
            snapshot_data: row.get("snapshot_data"),
            snapshot_metadata: row.get("snapshot_metadata"),
            created_at: row.get("created_at"),
        }))
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