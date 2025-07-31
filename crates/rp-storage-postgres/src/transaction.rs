//! PostgreSQL transaction implementation

use async_trait::async_trait;
use sqlx::{Postgres, Transaction as SqlxTransaction};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tracing::{debug, instrument};

use rp_storage::{
    Transaction as StorageTrait, StorageResult, StorageError,
    StorageEntity, VersionData,
};

use crate::PostgresError;

/// PostgreSQL transaction wrapper
pub struct PostgresTransaction {
    tx: SqlxTransaction<'static, Postgres>,
    id: Uuid,
    active: bool,
}

impl PostgresTransaction {
    /// Create a new transaction wrapper
    pub fn new(tx: SqlxTransaction<'static, Postgres>) -> Self {
        Self {
            tx,
            id: Uuid::now_v7(),
            active: true,
        }
    }
}

#[async_trait]
impl StorageTrait for PostgresTransaction {
    #[instrument(skip(self, entity), fields(entity_id = %entity.id))]
    async fn put_entity(&mut self, entity: &StorageEntity) -> StorageResult<()> {
        debug!("Storing entity: {} (type: {})", entity.id, entity.entity_type);
        
        // Check if entity exists for update vs insert
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM entities WHERE id = $1 AND deleted_at IS NULL)"
        )
        .bind(&entity.id)
        .fetch_one(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
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
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    async fn get_entity(&mut self, id: Uuid) -> StorageResult<Option<StorageEntity>> {
        debug!("Retrieving entity: {}", id);
        
        let row = sqlx::query_as::<_, EntityRow>(
            r#"
            SELECT id, entity_type, data, binary_data,
                   created_by, created_at, updated_at, version
            FROM entities
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(&id)
        .fetch_optional(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(row.map(|r| r.into()))
    }
    
    #[instrument(skip(self, updates))]
    async fn update_entity(
        &mut self,
        id: Uuid,
        updates: HashMap<String, JsonValue>,
    ) -> StorageResult<bool> {
        debug!("Updating entity: {} with {} fields", id, updates.len());
        
        // Get current entity
        let current = self.get_entity(id).await?;
        let mut entity = current.ok_or(StorageError::NotFound(id))?;
        
        // Merge updates into data
        if let Some(obj) = entity.data.as_object_mut() {
            for (key, value) in updates {
                obj.insert(key, value);
            }
        }
        
        // Update entity
        entity.updated_at = Utc::now();
        self.put_entity(&entity).await?;
        
        Ok(true)
    }
    
    #[instrument(skip(self))]
    async fn delete_entity(&mut self, id: Uuid) -> StorageResult<bool> {
        debug!("Deleting entity: {}", id);
        
        let result = sqlx::query(
            r#"
            UPDATE entities 
            SET deleted_at = NOW(),
                deleted_by = $2
            WHERE id = $1 AND deleted_at IS NULL
            "#
        )
        .bind(&id)
        .bind(&self.id) // Use transaction ID as deleted_by
        .execute(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(result.rows_affected() > 0)
    }
    
    #[instrument(skip(self))]
    async fn exists(&mut self, id: Uuid) -> StorageResult<bool> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM entities WHERE id = $1 AND deleted_at IS NULL)"
        )
        .bind(&id)
        .fetch_one(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(exists)
    }
    
    #[instrument(skip(self))]
    async fn get_entities(&mut self, ids: &[Uuid]) -> StorageResult<Vec<StorageEntity>> {
        debug!("Retrieving {} entities", ids.len());
        
        let rows = sqlx::query_as::<_, EntityRow>(
            r#"
            SELECT id, entity_type, data, binary_data,
                   created_by, created_at, updated_at, version
            FROM entities
            WHERE id = ANY($1) AND deleted_at IS NULL
            "#
        )
        .bind(ids)
        .fetch_all(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    #[instrument(skip(self))]
    async fn list_by_type(
        &mut self,
        entity_type: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> StorageResult<Vec<StorageEntity>> {
        debug!("Listing entities of type: {}", entity_type);
        
        let mut query = sqlx::query_as::<_, EntityRow>(
            r#"
            SELECT id, entity_type, data, binary_data,
                   created_by, created_at, updated_at, version
            FROM entities
            WHERE entity_type = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#
        )
        .bind(entity_type);
        
        // Add limit/offset if provided
        if let Some(limit) = limit {
            query = sqlx::query_as::<_, EntityRow>(
                r#"
                SELECT id, entity_type, data, binary_data,
                       created_by, created_at, updated_at, version
                FROM entities
                WHERE entity_type = $1 AND deleted_at IS NULL
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(entity_type)
            .bind(limit as i64)
            .bind(offset.unwrap_or(0) as i64);
        }
        
        let rows = query
            .fetch_all(&mut *self.tx)
            .await
            .map_err(PostgresError::from)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    #[instrument(skip(self, version_data))]
    async fn add_version(
        &mut self,
        entity_id: Uuid,
        version_data: VersionData,
    ) -> StorageResult<()> {
        debug!("Adding version {} for entity: {}", version_data.version, entity_id);
        
        sqlx::query(
            r#"
            INSERT INTO entity_versions (
                entity_id, version, data, binary_data,
                changed_by, changed_at, change_reason, parent_version
            ) VALUES ($1, $2, $3, NULL, $4, $5, $6, $7)
            "#
        )
        .bind(&entity_id)
        .bind(version_data.version as i64)
        .bind(&version_data.data)
        .bind(&version_data.changed_by)
        .bind(&version_data.changed_at)
        .bind(&version_data.change_reason)
        .bind(version_data.parent_version.map(|v| v as i64))
        .execute(&mut *self.tx)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    async fn commit(mut self) -> StorageResult<()> {
        debug!("Committing transaction: {}", self.id);
        
        self.active = false;
        self.tx.commit().await
            .map_err(PostgresError::from)?;
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    async fn rollback(mut self) -> StorageResult<()> {
        debug!("Rolling back transaction: {}", self.id);
        
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

// Helper struct for SQL queries
#[derive(sqlx::FromRow)]
struct EntityRow {
    id: Uuid,
    entity_type: String,
    data: JsonValue,
    binary_data: Option<Vec<u8>>,
    created_by: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    version: i64,
}

impl From<EntityRow> for StorageEntity {
    fn from(row: EntityRow) -> Self {
        Self {
            id: row.id,
            entity_type: row.entity_type,
            data: row.data,
            binary_data: row.binary_data,
            created_by: row.created_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
            version: row.version as u64,
        }
    }
}