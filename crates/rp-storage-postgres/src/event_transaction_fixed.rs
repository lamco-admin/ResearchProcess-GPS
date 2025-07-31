//! Event-sourced PostgreSQL transaction implementation

use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;
use chrono::Utc;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tracing::{debug, instrument, warn};

use rp_storage::{
    Transaction as StorageTrait, StorageResult, StorageError,
    StorageEntity, VersionData,
};
use rp_events::{
    DomainEvent, EventStore, PostgresEventStore, EventBuilder,
    TheoryEvent, IdentityPersonaEvent, PersonEvent, SourceEvent, EvidenceEvent,
    ResearcherEvent, WorkspaceEvent,
};
use rp_core::prelude::*;

use crate::PostgresError;

/// Event-sourced PostgreSQL transaction wrapper
pub struct EventSourcedTransaction {
    pool: PgPool,
    tx: SqlxTransaction<'static, Postgres>,
    event_store: PostgresEventStore,
    id: Uuid,
    actor_id: Uuid,
    active: bool,
}

impl EventSourcedTransaction {
    /// Create a new event-sourced transaction
    pub fn new(
        pool: PgPool,
        tx: SqlxTransaction<'static, Postgres>,
        actor_id: Uuid,
    ) -> Self {
        let event_store = PostgresEventStore::new(pool.clone());
        Self {
            pool,
            tx,
            event_store,
            id: Uuid::new_v4(),
            actor_id,
            active: true,
        }
    }
    
    /// Generate event from entity changes
    async fn generate_event(
        &self,
        entity: &StorageEntity,
        exists: bool,
        old_entity: Option<&StorageEntity>,
    ) -> StorageResult<Option<DomainEvent>> {
        let event = match entity.entity_type.as_str() {
            "Theory" => {
                if !exists {
                    // Theory requires at least a question
                    if let Some(question) = entity.data.get("question").and_then(|v| v.as_str()) {
                        let hypothesis = entity.data.get("hypothesis")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        
                        Some(DomainEvent::Theory(TheoryEvent::Created {
                            question: question.to_string(),
                            hypothesis: hypothesis.unwrap_or_else(|| "".to_string()), // Empty hypothesis is acceptable
                            researcher_id: entity.created_by,
                        }))
                    } else {
                        tracing::warn!("Theory created without question, skipping event");
                        None
                    }
                } else if let Some(old) = old_entity {
                    // Check for state change
                    let old_state = old.data.get("state").and_then(|v| v.as_str());
                    let new_state = entity.data.get("state").and_then(|v| v.as_str());
                    
                    if old_state != new_state {
                        if let (Some(from), Some(to)) = (old_state, new_state) {
                            // Parse states - return None if parsing fails
                            match (
                                serde_json::from_str::<TheoryState>(&format!("\"{}\"", from)),
                                serde_json::from_str::<TheoryState>(&format!("\"{}\"", to))
                            ) {
                                (Ok(from_state), Ok(to_state)) => {
                                    Some(DomainEvent::Theory(TheoryEvent::StateChanged {
                                        from_state,
                                        to_state,
                                        reason: "State transition".to_string(),
                                    }))
                                }
                                _ => {
                                    // Log error and skip event if state parsing fails
                                    tracing::error!(
                                        "Failed to parse theory state transition: {} -> {}",
                                        from, to
                                    );
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    } else {
                        // General update
                        Some(DomainEvent::Theory(TheoryEvent::Updated {
                            question: None,
                            hypothesis: None,
                            details: Some("Entity updated".to_string()),
                        }))
                    }
                } else {
                    None
                }
            }
            
            "IdentityPersona" => {
                if !exists {
                    let name = entity.data.get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    
                    Some(DomainEvent::IdentityPersona(IdentityPersonaEvent::Created {
                        name,
                        researcher_id: entity.created_by,
                    }))
                } else {
                    Some(DomainEvent::IdentityPersona(IdentityPersonaEvent::Updated {
                        name: None,
                        details: entity.data.clone(),
                    }))
                }
            }
            
            "Person" => {
                if !exists {
                    Some(DomainEvent::Person(PersonEvent::Created {
                        from_identities: vec![], // Would need to extract from entity data
                        concluded_by: entity.created_by,
                    }))
                } else {
                    Some(DomainEvent::Person(PersonEvent::Updated {
                        details: entity.data.clone(),
                    }))
                }
            }
            
            "Source" => {
                if !exists {
                    // Title is required for source creation
                    if let Some(title) = entity.data.get("title").and_then(|v| v.as_str()) {
                        Some(DomainEvent::Source(SourceEvent::Created {
                            source_type: SourceType::Item, // Would need proper parsing
                            title: title.to_string(),
                            researcher_id: entity.created_by,
                        }))
                    } else {
                        tracing::warn!("Source created without title, skipping event");
                        None
                    }
                } else {
                    Some(DomainEvent::Source(SourceEvent::Updated {
                        title: None,
                        details: entity.data.clone(),
                    }))
                }
            }
            
            "Evidence" => {
                if !exists {
                    // Both source_id and original_text are required for evidence
                    match (
                        entity.data.get("source_id")
                            .and_then(|v| v.as_str())
                            .and_then(|s| Uuid::parse_str(s).ok()),
                        entity.data.get("original_text")
                            .and_then(|v| v.as_str())
                    ) {
                        (Some(source_id), Some(original_text)) => {
                            Some(DomainEvent::Evidence(EvidenceEvent::Extracted {
                                source_id,
                                original_text: original_text.to_string(),
                                researcher_id: entity.created_by,
                            }))
                        }
                        _ => {
                            tracing::warn!("Evidence created without required fields, skipping event");
                            None
                        }
                    }
                } else {
                    Some(DomainEvent::Evidence(EvidenceEvent::Updated {
                        interpreted_text: None,
                        details: entity.data.clone(),
                    }))
                }
            }
            
            "Researcher" => {
                if !exists {
                    // Name is required for researcher
                    if let Some(name) = entity.data.get("name").and_then(|v| v.as_str()) {
                        Some(DomainEvent::Researcher(ResearcherEvent::Created {
                            name: name.to_string(),
                            researcher_type: "Individual".to_string(),
                        }))
                    } else {
                        tracing::warn!("Researcher created without name, skipping event");
                        None
                    }
                } else {
                    Some(DomainEvent::Researcher(ResearcherEvent::Updated {
                        name: None,
                        contact_info: JsonValue::Object(Default::default()),
                    }))
                }
            }
            
            "Workspace" => {
                if !exists {
                    // Name is required for workspace
                    if let Some(name) = entity.data.get("name").and_then(|v| v.as_str()) {
                        Some(DomainEvent::Workspace(WorkspaceEvent::Created {
                            name: name.to_string(),
                            owner_id: entity.created_by,
                        }))
                    } else {
                        tracing::warn!("Workspace created without name, skipping event");
                        None
                    }
                } else {
                    None // Workspace updates handled differently
                }
            }
            
            _ => {
                debug!("No event mapping for entity type: {}", entity.entity_type);
                None
            }
        };
        
        Ok(event)
    }
}

#[async_trait]
impl StorageTrait for EventSourcedTransaction {
    #[instrument(skip(self, entity), fields(entity_id = %entity.id))]
    async fn put_entity(&mut self, entity: &StorageEntity) -> StorageResult<()> {
        debug!("Storing entity with event sourcing: {} (type: {})", entity.id, entity.entity_type);
        
        // Check if entity exists
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM entities WHERE id = $1 AND deleted_at IS NULL)"
        )
        .bind(&entity.id)
        .fetch_one(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        // Get old entity if updating
        let old_entity = if exists {
            self.get_entity(entity.id).await?
        } else {
            None
        };
        
        // Perform the CRUD operation
        if exists {
            // Update existing entity
            sqlx::query(
                r#"
                UPDATE entities 
                SET data = $2,
                    binary_data = $3,
                    updated_at = $4,
                    version = version + 1
                WHERE id = $1 AND deleted_at IS NULL
                "#
            )
            .bind(&entity.id)
            .bind(&entity.data)
            .bind(&entity.binary_data)
            .bind(&entity.updated_at)
            .execute(&mut *self.tx)
            .await
            .map_err(PostgresError::from)?;
        } else {
            // Insert new entity
            sqlx::query(
                r#"
                INSERT INTO entities (
                    id, entity_type, data, binary_data,
                    created_by, created_at, updated_at, version
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#
            )
            .bind(&entity.id)
            .bind(&entity.entity_type)
            .bind(&entity.data)
            .bind(&entity.binary_data)
            .bind(&entity.created_by)
            .bind(&entity.created_at)
            .bind(&entity.updated_at)
            .bind(entity.version as i64)
            .execute(&mut *self.tx)
            .await
            .map_err(PostgresError::from)?;
        }
        
        // Generate and publish event
        if let Some(event) = self.generate_event(entity, exists, old_entity.as_ref()).await? {
            let version = match self.event_store.get_aggregate_version(entity.id).await {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("Failed to get aggregate version: {}", e);
                    return Err(PostgresError::from(e).into());
                }
            };
            
            let metadata = EventBuilder::new(
                entity.id,
                entity.entity_type.clone(),
                self.actor_id
            )
            .with_tag(if exists { "update".to_string() } else { "create".to_string() })
            .build(version + 1);
            
            // Use -1 for expected version to skip concurrency check in this context
            match self.event_store.append_event(
                entity.id,
                &entity.entity_type,
                -1, // Skip version check
                event,
                metadata
            ).await {
                Ok(_) => debug!("Event published for entity {}", entity.id),
                Err(e) => warn!("Failed to publish event for entity {}: {}", entity.id, e),
            }
        }
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    async fn get_entity(&mut self, id: Uuid) -> StorageResult<Option<StorageEntity>> {
        debug!("Retrieving entity: {}", id);
        
        let row = sqlx::query!(
            r#"
            SELECT id, entity_type, data, binary_data,
                   created_by, created_at, updated_at, version
            FROM entities
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id
        )
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(row.map(|r| StorageEntity {
            id: r.id,
            entity_type: r.entity_type,
            data: r.data,
            binary_data: r.binary_data,
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
            version: r.version as u64,
        }))
    }
    
    #[instrument(skip(self))]
    async fn get_entities(&mut self, ids: &[Uuid]) -> StorageResult<Vec<StorageEntity>> {
        debug!("Retrieving {} entities", ids.len());
        
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        
        let rows = sqlx::query!(
            r#"
            SELECT id, entity_type, data, binary_data,
                   created_by, created_at, updated_at, version
            FROM entities
            WHERE id = ANY($1) AND deleted_at IS NULL
            ORDER BY created_at
            "#,
            ids
        )
        .fetch_all(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(rows.into_iter().map(|r| StorageEntity {
            id: r.id,
            entity_type: r.entity_type,
            data: r.data,
            binary_data: r.binary_data,
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
            version: r.version as u64,
        }).collect())
    }
    
    #[instrument(skip(self, updates))]
    async fn update_entity(
        &mut self,
        id: Uuid,
        updates: HashMap<String, JsonValue>,
    ) -> StorageResult<bool> {
        debug!("Updating entity {} with {} fields", id, updates.len());
        
        // Get current entity
        let entity = self.get_entity(id).await?;
        
        if let Some(mut entity) = entity {
            // Merge updates into existing data
            if let Some(obj) = entity.data.as_object_mut() {
                for (key, value) in updates {
                    obj.insert(key, value);
                }
            }
            
            // Update timestamp
            entity.updated_at = Utc::now();
            
            // Use put_entity which handles event publishing
            self.put_entity(&entity).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    #[instrument(skip(self))]
    async fn delete_entity(&mut self, id: Uuid) -> StorageResult<bool> {
        debug!("Soft deleting entity: {}", id);
        
        let result = sqlx::query!(
            r#"
            UPDATE entities 
            SET deleted_at = $2, deleted_by = $3
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            id,
            Utc::now(),
            self.actor_id
        )
        .execute(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        if result.rows_affected() == 0 {
            return Ok(false);
        }
        
        // TODO: Publish deletion event
        
        Ok(true)
    }
    
    #[instrument(skip(self))]
    async fn list_by_type(
        &mut self,
        entity_type: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> StorageResult<Vec<StorageEntity>> {
        debug!("Listing entities of type: {}", entity_type);
        
        // Use explicit defaults for pagination
        let limit = limit.map(|l| l as i64).unwrap_or(1000);
        let offset = offset.map(|o| o as i64).unwrap_or(0);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, entity_type, data, binary_data,
                   created_by, created_at, updated_at, version
            FROM entities
            WHERE entity_type = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            entity_type,
            limit,
            offset
        )
        .fetch_all(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(rows.into_iter().map(|r| StorageEntity {
            id: r.id,
            entity_type: r.entity_type,
            data: r.data,
            binary_data: r.binary_data,
            created_by: r.created_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
            version: r.version as u64,
        }).collect())
    }
    
    #[instrument(skip(self))]
    async fn exists(&mut self, id: Uuid) -> StorageResult<bool> {
        debug!("Checking existence of entity: {}", id);
        
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM entities WHERE id = $1 AND deleted_at IS NULL)"
        )
        .bind(id)
        .fetch_one(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(exists)
    }
    
    #[instrument(skip(self, version_data))]
    async fn add_version(
        &mut self,
        entity_id: Uuid,
        version_data: VersionData,
    ) -> StorageResult<()> {
        debug!("Adding version for entity: {}", entity_id);
        
        sqlx::query!(
            r#"
            INSERT INTO entity_versions (
                entity_id, version, data, binary_data,
                changed_by, change_reason, parent_version
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            entity_id,
            version_data.version as i64,
            version_data.data,
            None::<Vec<u8>>, // binary_data not in VersionData
            version_data.changed_by,
            version_data.change_reason,
            version_data.parent_version.map(|v| v as i64)
        )
        .execute(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    async fn commit(mut self) -> StorageResult<()> {
        debug!("Committing transaction: {}", self.id);
        
        if !self.active {
            return Err(StorageError::TransactionError("Transaction already completed".to_string()));
        }
        
        self.active = false;
        self.tx.commit().await
            .map_err(PostgresError::from)?;
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    async fn rollback(mut self) -> StorageResult<()> {
        debug!("Rolling back transaction: {}", self.id);
        
        if !self.active {
            return Err(StorageError::TransactionError("Transaction already completed".to_string()));
        }
        
        self.active = false;
        self.tx.rollback().await
            .map_err(PostgresError::from)?;
        
        Ok(())
    }
    
    fn transaction_id(&self) -> Uuid {
        self.id
    }
    
    fn is_active(&self) -> bool {
        self.active
    }
}