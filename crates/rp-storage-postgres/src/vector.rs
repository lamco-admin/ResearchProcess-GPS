//! pgvector support for semantic search

use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use serde_json::Value as JsonValue;
use tracing::{debug, instrument};

use rp_storage::{
    VectorSearchBackend, StorageBackend, StorageResult, StorageError,
    VectorSearchResult,
};

use crate::{PostgresBackend, PostgresError};

#[async_trait]
impl VectorSearchBackend for PostgresBackend {
    #[instrument(skip(self, embeddings))]
    async fn store_embeddings(
        &self,
        entity_id: Uuid,
        embeddings: &[f32],
        metadata: Option<HashMap<String, JsonValue>>,
    ) -> StorageResult<()> {
        if !self.capabilities().vector_search {
            return Err(StorageError::NotSupported(
                "Vector search not enabled for this backend".to_string()
            ));
        }
        
        debug!("Storing embeddings for entity: {} (dimension: {})", entity_id, embeddings.len());
        
        let mut conn = self.pool().pool().acquire().await
            .map_err(PostgresError::from)?;
        
        // Update entity with embeddings
        sqlx::query(
            "UPDATE entities SET embedding = $2::vector WHERE id = $1"
        )
        .bind(&entity_id)
        .bind(embeddings)
        .execute(&mut *conn)
        .await
        .map_err(PostgresError::from)?;
        
        // Store metadata if provided
        if let Some(meta) = metadata {
            let model_name = meta.get("model").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
            let meta_json = serde_json::to_value(&meta).unwrap();
            
            sqlx::query(
                r#"
                INSERT INTO vector_metadata (entity_id, embedding_model, metadata)
                VALUES ($1, $2, $3)
                ON CONFLICT (entity_id) DO UPDATE
                SET embedding_model = $2, metadata = $3, embedding_date = NOW()
                "#
            )
            .bind(&entity_id)
            .bind(&model_name)
            .bind(&meta_json)
            .execute(&mut *conn)
            .await
            .map_err(PostgresError::from)?;
        }
        
        Ok(())
    }
    
    #[instrument(skip(self, query_vector))]
    async fn vector_search(
        &self,
        query_vector: &[f32],
        limit: usize,
        filters: Option<HashMap<String, JsonValue>>,
    ) -> StorageResult<Vec<VectorSearchResult>> {
        if !self.capabilities().vector_search {
            return Err(StorageError::NotSupported(
                "Vector search not enabled for this backend".to_string()
            ));
        }
        
        debug!("Performing vector search (dimension: {}, limit: {})", query_vector.len(), limit);
        
        let mut conn = self.pool().pool().acquire().await
            .map_err(PostgresError::from)?;
        
        // Build query with optional filters
        let mut query = String::from(
            r#"
            SELECT id, embedding <=> $1::vector as distance, data
            FROM entities
            WHERE embedding IS NOT NULL AND deleted_at IS NULL
            "#
        );
        
        // Add filters if provided
        if let Some(filters) = &filters {
            for (key, value) in filters {
                query.push_str(&format!(" AND data->>'{}' = '{}'", key, value));
            }
        }
        
        query.push_str(" ORDER BY embedding <=> $1::vector LIMIT $2");
        
        let rows = sqlx::query_as::<_, VectorSearchRow>(&query)
            .bind(query_vector)
            .bind(limit as i64)
            .fetch_all(&mut *conn)
            .await
            .map_err(PostgresError::from)?;
        
        Ok(rows.into_iter().map(|r| VectorSearchResult {
            entity_id: r.id,
            distance: r.distance,
            metadata: Some(HashMap::from([
                ("entity_type".to_string(), r.data.get("entity_type").cloned().unwrap_or(JsonValue::Null)),
            ])),
        }).collect())
    }
    
    #[instrument(skip(self, embeddings))]
    async fn update_embeddings(
        &self,
        entity_id: Uuid,
        embeddings: &[f32],
    ) -> StorageResult<()> {
        self.store_embeddings(entity_id, embeddings, None).await
    }
    
    #[instrument(skip(self))]
    async fn delete_embeddings(&self, entity_id: Uuid) -> StorageResult<bool> {
        if !self.capabilities().vector_search {
            return Err(StorageError::NotSupported(
                "Vector search not enabled for this backend".to_string()
            ));
        }
        
        debug!("Deleting embeddings for entity: {}", entity_id);
        
        let mut conn = self.pool().pool().acquire().await
            .map_err(PostgresError::from)?;
        
        let result = sqlx::query(
            "UPDATE entities SET embedding = NULL WHERE id = $1"
        )
        .bind(&entity_id)
        .execute(&mut *conn)
        .await
        .map_err(PostgresError::from)?;
        
        // Also delete metadata
        sqlx::query("DELETE FROM vector_metadata WHERE entity_id = $1")
            .bind(&entity_id)
            .execute(&mut *conn)
            .await
            .map_err(PostgresError::from)?;
        
        Ok(result.rows_affected() > 0)
    }
}

#[derive(sqlx::FromRow)]
struct VectorSearchRow {
    id: Uuid,
    distance: f32,
    data: JsonValue,
}