use crate::{error::ApiResult, state::AppState, ApiError};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use rp_core::layer3::EntityType;
use rp_protocol::{
    CreateEntityRequest, EntityResponse, ListResponse, PaginationParams, UpdateEntityRequest,
    EntityMetadata,
};
use rp_storage::{StorageBackend, Transaction, StorageEntity};
use rp_storage_postgres::EventSourcedTransaction;
use serde_json::Value;
use sqlx;
use std::sync::Arc;
use uuid::Uuid;

// Create a new entity
pub async fn create_entity(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateEntityRequest>,
) -> ApiResult<(StatusCode, Json<EntityResponse>)> {
    // Convert EntityType to string representation
    let entity_type_str = entity_type_to_string(&request.entity_type);
    
    // Create entity struct
    let entity = StorageEntity {
        id: Uuid::now_v7(),
        entity_type: entity_type_str,
        data: request.data,
        binary_data: None,
        created_by: Uuid::nil(), // TODO: Get from auth context
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        version: 1,
    };

    // Begin transaction
    let pool = state.storage.pool().pool().clone();
    let tx = pool.begin().await.map_err(|e| ApiError::Database(e))?;
    
    // Create event-sourced transaction
    let mut event_tx = EventSourcedTransaction::new(
        pool.clone(),
        tx,
        Uuid::nil(), // TODO: Get actor ID from auth context
    );
    
    // Store entity
    event_tx
        .put_entity(&entity)
        .await
        .map_err(ApiError::Storage)?;
    
    // Commit transaction
    event_tx.commit().await.map_err(ApiError::Storage)?;

    let response = EntityResponse {
        id: entity.id,
        entity_type: request.entity_type,
        version: entity.version as i64,
        data: entity.data,
        metadata: EntityMetadata {
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: Some(entity.created_by),
            workspace_id: None, // TODO: Get from request or context
            tags: None,
        },
    };

    Ok((StatusCode::CREATED, Json(response)))
}

// Get entity by ID
pub async fn get_entity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<EntityResponse>> {
    // Begin transaction
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    
    // Get entity
    let entity = tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    // Convert entity_type string to enum
    let entity_type = string_to_entity_type(&entity.entity_type)?;

    let response = EntityResponse {
        id: entity.id,
        entity_type,
        version: entity.version as i64,
        data: entity.data,
        metadata: EntityMetadata {
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            created_by: entity.created_by,
            updated_by: Some(entity.created_by),
            workspace_id: None,
            tags: None,
        },
    };

    Ok(Json(response))
}

// Update entity
pub async fn update_entity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateEntityRequest>,
) -> ApiResult<Json<EntityResponse>> {
    // Begin transaction
    let pool = state.storage.pool().pool().clone();
    let tx = pool.begin().await.map_err(|e| ApiError::Database(e))?;
    
    // Create event-sourced transaction
    let mut event_tx = EventSourcedTransaction::new(
        pool.clone(),
        tx,
        Uuid::nil(), // TODO: Get actor ID from auth context
    );
    
    // Get current entity
    let current = event_tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;

    // Check version for optimistic locking
    if let Some(expected_version) = request.version {
        if current.version as i64 != expected_version {
            return Err(ApiError::Conflict(format!(
                "Version conflict: expected {}, found {}",
                expected_version, current.version
            )));
        }
    }

    // Update entity via transaction
    let mut updates = std::collections::HashMap::new();
    if let Value::Object(obj) = request.data {
        for (k, v) in obj {
            updates.insert(k, v);
        }
    }
    
    event_tx
        .update_entity(id, updates)
        .await
        .map_err(ApiError::Storage)?;
    
    // Commit transaction
    event_tx.commit().await.map_err(ApiError::Storage)?;

    // Get updated entity
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    let updated = tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    // Convert entity_type string to enum
    let entity_type = string_to_entity_type(&updated.entity_type)?;

    let response = EntityResponse {
        id: updated.id,
        entity_type,
        version: updated.version as i64,
        data: updated.data,
        metadata: EntityMetadata {
            created_at: updated.created_at,
            updated_at: updated.updated_at,
            created_by: updated.created_by,
            updated_by: Some(updated.created_by),
            workspace_id: None,
            tags: None,
        },
    };

    Ok(Json(response))
}

// Delete entity (soft delete)
pub async fn delete_entity(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    // Begin transaction
    let pool = state.storage.pool().pool().clone();
    let tx = pool.begin().await.map_err(|e| ApiError::Database(e))?;
    
    // Create event-sourced transaction
    let mut event_tx = EventSourcedTransaction::new(
        pool.clone(),
        tx,
        Uuid::nil(), // TODO: Get actor ID from auth context
    );
    
    // Delete entity
    let deleted = event_tx
        .delete_entity(id)
        .await
        .map_err(ApiError::Storage)?;
    
    if !deleted {
        return Err(ApiError::NotFound);
    }
    
    // Commit transaction
    event_tx.commit().await.map_err(ApiError::Storage)?;

    Ok(StatusCode::NO_CONTENT)
}

// List entities with filtering
pub async fn list_entities(
    State(state): State<Arc<AppState>>,
    Query(mut pagination): Query<PaginationParams>,
) -> ApiResult<Json<ListResponse<EntityResponse>>> {
    // Validate pagination params
    pagination.validate();
    
    let limit = pagination.limit as usize;
    let offset = pagination.offset as usize;

    // Get database pool directly for raw query
    let pool = state.storage.pool().pool();
    
    // Query entities with pagination
    let entities: Vec<EntityRow> = sqlx::query_as(
        r#"
        SELECT id, entity_type, data, binary_data, created_by, created_at, updated_at, version
        FROM entities
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Database(e))?;
    
    // Get total count
    let total_row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM entities WHERE deleted_at IS NULL"
    )
    .fetch_one(pool)
    .await
    .map_err(|e| ApiError::Database(e))?;
    
    let total = total_row.0 as u64;
    let has_more = pagination.has_more(total);

    // Convert to response format
    let items: Vec<EntityResponse> = entities
        .into_iter()
        .map(|entity| {
            let entity_type = string_to_entity_type(&entity.entity_type)?;
            Ok(EntityResponse {
                id: entity.id,
                entity_type,
                version: entity.version,
                data: entity.data,
                metadata: EntityMetadata {
                    created_at: entity.created_at,
                    updated_at: entity.updated_at,
                    created_by: entity.created_by,
                    updated_by: Some(entity.created_by),
                    workspace_id: None,
                    tags: None,
                },
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;

    Ok(Json(ListResponse {
        items,
        total,
        limit: pagination.limit,
        offset: pagination.offset,
        has_more,
        cursor: None,
    }))
}

// Helper struct for SQL queries
#[derive(sqlx::FromRow)]
pub(crate) struct EntityRow {
    pub id: Uuid,
    pub entity_type: String,
    pub data: serde_json::Value,
    #[allow(dead_code)]
    pub binary_data: Option<Vec<u8>>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
}

// Helper functions to convert between EntityType and String
fn entity_type_to_string(entity_type: &EntityType) -> String {
    crate::entity_type_mapper::entity_type_to_string(entity_type).to_string()
}

fn string_to_entity_type(s: &str) -> Result<EntityType, ApiError> {
    crate::entity_type_mapper::parse_entity_type(s)
}