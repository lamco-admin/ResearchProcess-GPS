use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction, Row};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{Result, StoredEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionState {
    pub projection_name: String,
    pub projection_type: String,
    pub last_processed_event_id: Option<Uuid>,
    pub last_processed_at: Option<DateTime<Utc>>,
    pub current_position: i64,
    pub status: String,
    pub error_count: i32,
    pub last_error: Option<String>,
    pub last_error_at: Option<DateTime<Utc>>,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectionStatus {
    Active,
    Paused,
    Error,
    Rebuilding,
}

impl From<String> for ProjectionStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ACTIVE" => ProjectionStatus::Active,
            "PAUSED" => ProjectionStatus::Paused,
            "ERROR" => ProjectionStatus::Error,
            "REBUILDING" => ProjectionStatus::Rebuilding,
            _ => ProjectionStatus::Active,
        }
    }
}

#[async_trait]
pub trait Projection: Send + Sync {
    fn name(&self) -> &str;
    
    async fn handle_event(
        &self,
        event: &StoredEvent,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()>;
    
    async fn get_state(&self, pool: &PgPool) -> Result<ProjectionState>;
    
    async fn reset(&self, pool: &PgPool) -> Result<()>;
}

pub struct ProjectionManager {
    pool: PgPool,
    projections: HashMap<String, Box<dyn Projection>>,
}

impl ProjectionManager {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            projections: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, projection: Box<dyn Projection>) {
        self.projections.insert(projection.name().to_string(), projection);
    }
    
    pub async fn process_events(&self, from_position: Option<i64>) -> Result<()> {
        let from_position = from_position.unwrap_or(0);
        
        // Get unprocessed events
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
            WHERE id > (
                SELECT COALESCE(
                    (SELECT id FROM events ORDER BY id LIMIT 1 OFFSET $1),
                    '00000000-0000-0000-0000-000000000000'::uuid
                )
            )
            ORDER BY id
            LIMIT 1000
            "#,
        )
        .bind(from_position)
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
        
        for event in events {
            for (name, projection) in &self.projections {
                let mut tx = self.pool.begin().await?;
                
                match projection.handle_event(&event, &mut tx).await {
                    Ok(_) => {
                        // Update projection position
                        sqlx::query(
                            r#"
                            SELECT update_projection_position($1, $2, $3)
                            "#,
                        )
                        .bind(name)
                        .bind(event.event_id)
                        .bind(from_position + 1)
                        .execute(&mut *tx)
                        .await?;
                        
                        tx.commit().await?;
                    }
                    Err(e) => {
                        tx.rollback().await?;
                        
                        // Record error
                        sqlx::query(
                            r#"
                            SELECT record_projection_error($1, $2)
                            "#,
                        )
                        .bind(name)
                        .bind(e.to_string())
                        .execute(&self.pool)
                        .await?;
                    }
                }
            }
        }
        
        Ok(())
    }
}

// Entity Count Projection
pub struct EntityCountProjection {
    name: String,
}

impl EntityCountProjection {
    pub fn new() -> Self {
        Self {
            name: "entity_count".to_string(),
        }
    }
}

#[async_trait]
impl Projection for EntityCountProjection {
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn handle_event(
        &self,
        event: &StoredEvent,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        // Extract entity type from event
        let entity_type = event.aggregate_type.clone();
        
        // Check if this is a creation event
        if event.event_type.contains("Created") {
            // Increment count
            sqlx::query(
                r#"
                INSERT INTO projection_entity_count (entity_type, count)
                VALUES ($1, 1)
                ON CONFLICT (entity_type) 
                DO UPDATE SET count = projection_entity_count.count + 1
                "#,
            )
            .bind(entity_type)
            .execute(&mut **tx)
            .await?;
        }
        
        Ok(())
    }
    
    async fn get_state(&self, pool: &PgPool) -> Result<ProjectionState> {
        let row = sqlx::query(
            r#"
            SELECT 
                projection_name,
                projection_type,
                last_processed_event_id,
                last_processed_at,
                current_position,
                status,
                error_count,
                last_error,
                last_error_at,
                config
            FROM projections
            WHERE projection_name = $1
            "#,
        )
        .bind(self.name.as_str())
        .fetch_one(pool)
        .await?;
        
        Ok(ProjectionState {
            projection_name: row.get("projection_name"),
            projection_type: row.get("projection_type"),
            last_processed_event_id: row.get("last_processed_event_id"),
            last_processed_at: row.get("last_processed_at"),
            current_position: row.get("current_position"),
            status: row.get("status"),
            error_count: row.get("error_count"),
            last_error: row.get("last_error"),
            last_error_at: row.get("last_error_at"),
            config: row.get("config"),
        })
    }
    
    async fn reset(&self, pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM projection_entity_count;
            UPDATE projections 
            SET last_processed_event_id = NULL,
                last_processed_at = NULL,
                current_position = 0,
                error_count = 0,
                last_error = NULL,
                last_error_at = NULL
            WHERE projection_name = $1
            "#,
        )
        .bind(self.name.as_str())
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

// State Distribution Projection
pub struct StateDistributionProjection {
    name: String,
}

impl StateDistributionProjection {
    pub fn new() -> Self {
        Self {
            name: "state_distribution".to_string(),
        }
    }
}

#[async_trait]
impl Projection for StateDistributionProjection {
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn handle_event(
        &self,
        event: &StoredEvent,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        // Check if this is a state change event
        if event.event_type.contains("StateChanged") {
            let entity_type = event.aggregate_type.clone();
            
            // Parse event data to get states
            if let Ok(state_change) = serde_json::from_value::<StateChangeData>(event.event_data.clone()) {
                // Decrement old state count
                if let Some(from_state) = state_change.from_state {
                    sqlx::query(
                        r#"
                        UPDATE projection_state_distribution 
                        SET count = GREATEST(0, count - 1)
                        WHERE entity_type = $1 AND state = $2
                        "#,
                    )
                    .bind(&entity_type)
                    .bind(&from_state)
                    .execute(&mut **tx)
                    .await?;
                }
                
                // Increment new state count
                sqlx::query(
                    r#"
                    INSERT INTO projection_state_distribution (entity_type, state, count)
                    VALUES ($1, $2, 1)
                    ON CONFLICT (entity_type, state) 
                    DO UPDATE SET count = projection_state_distribution.count + 1
                    "#,
                )
                .bind(&entity_type)
                .bind(&state_change.to_state)
                .execute(&mut **tx)
                .await?;
            }
        }
        
        Ok(())
    }
    
    async fn get_state(&self, pool: &PgPool) -> Result<ProjectionState> {
        let row = sqlx::query(
            r#"
            SELECT 
                projection_name,
                projection_type,
                last_processed_event_id,
                last_processed_at,
                current_position,
                status,
                error_count,
                last_error,
                last_error_at,
                config
            FROM projections
            WHERE projection_name = $1
            "#,
        )
        .bind(self.name.as_str())
        .fetch_one(pool)
        .await?;
        
        Ok(ProjectionState {
            projection_name: row.get("projection_name"),
            projection_type: row.get("projection_type"),
            last_processed_event_id: row.get("last_processed_event_id"),
            last_processed_at: row.get("last_processed_at"),
            current_position: row.get("current_position"),
            status: row.get("status"),
            error_count: row.get("error_count"),
            last_error: row.get("last_error"),
            last_error_at: row.get("last_error_at"),
            config: row.get("config"),
        })
    }
    
    async fn reset(&self, pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM projection_state_distribution;
            UPDATE projections 
            SET last_processed_event_id = NULL,
                last_processed_at = NULL,
                current_position = 0,
                error_count = 0,
                last_error = NULL,
                last_error_at = NULL
            WHERE projection_name = $1
            "#,
        )
        .bind(self.name.as_str())
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

// Recent Activity Projection
pub struct RecentActivityProjection {
    name: String,
}

impl RecentActivityProjection {
    pub fn new() -> Self {
        Self {
            name: "recent_activity".to_string(),
        }
    }
}

#[async_trait]
impl Projection for RecentActivityProjection {
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn handle_event(
        &self,
        event: &StoredEvent,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        // Store recent activity
        sqlx::query(
            r#"
            INSERT INTO projection_recent_activity (
                event_id,
                aggregate_id,
                aggregate_type,
                event_type,
                actor_id,
                occurred_at,
                summary
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(event.event_id)
        .bind(event.aggregate_id)
        .bind(&event.aggregate_type)
        .bind(&event.event_type)
        .bind(event.actor_id)
        .bind(event.occurred_at)
        .bind(format!("{} {} at {}", event.aggregate_type, event.event_type, event.occurred_at))
        .execute(&mut **tx)
        .await?;
        
        // Keep only last 1000 activities
        sqlx::query(
            r#"
            DELETE FROM projection_recent_activity
            WHERE event_id IN (
                SELECT event_id 
                FROM projection_recent_activity 
                ORDER BY occurred_at DESC 
                OFFSET 1000
            )
            "#
        )
        .execute(&mut **tx)
        .await?;
        
        Ok(())
    }
    
    async fn get_state(&self, pool: &PgPool) -> Result<ProjectionState> {
        let row = sqlx::query(
            r#"
            SELECT 
                projection_name,
                projection_type,
                last_processed_event_id,
                last_processed_at,
                current_position,
                status,
                error_count,
                last_error,
                last_error_at,
                config
            FROM projections
            WHERE projection_name = $1
            "#,
        )
        .bind(self.name.as_str())
        .fetch_one(pool)
        .await?;
        
        Ok(ProjectionState {
            projection_name: row.get("projection_name"),
            projection_type: row.get("projection_type"),
            last_processed_event_id: row.get("last_processed_event_id"),
            last_processed_at: row.get("last_processed_at"),
            current_position: row.get("current_position"),
            status: row.get("status"),
            error_count: row.get("error_count"),
            last_error: row.get("last_error"),
            last_error_at: row.get("last_error_at"),
            config: row.get("config"),
        })
    }
    
    async fn reset(&self, pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM projection_recent_activity;
            UPDATE projections 
            SET last_processed_event_id = NULL,
                last_processed_at = NULL,
                current_position = 0,
                error_count = 0,
                last_error = NULL,
                last_error_at = NULL
            WHERE projection_name = $1
            "#,
        )
        .bind(self.name.as_str())
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

// Helper structs for parsing event data
#[derive(Debug, Deserialize)]
struct StateChangeData {
    from_state: Option<String>,
    to_state: String,
}

// Create projection tables
pub async fn create_projection_tables(pool: &PgPool) -> Result<()> {
    // Entity count projection
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS projection_entity_count (
            entity_type VARCHAR(100) PRIMARY KEY,
            count BIGINT NOT NULL DEFAULT 0,
            last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await?;
    
    // State distribution projection
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS projection_state_distribution (
            entity_type VARCHAR(100) NOT NULL,
            state VARCHAR(100) NOT NULL,
            count BIGINT NOT NULL DEFAULT 0,
            last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            PRIMARY KEY (entity_type, state)
        )
        "#
    )
    .execute(pool)
    .await?;
    
    // Recent activity projection
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS projection_recent_activity (
            event_id UUID PRIMARY KEY,
            aggregate_id UUID NOT NULL,
            aggregate_type VARCHAR(100) NOT NULL,
            event_type VARCHAR(200) NOT NULL,
            actor_id UUID NOT NULL,
            occurred_at TIMESTAMPTZ NOT NULL,
            summary TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;
    
    // Create indexes
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_recent_activity_occurred 
        ON projection_recent_activity(occurred_at DESC)
        "#
    )
    .execute(pool)
    .await?;
    
    Ok(())
}