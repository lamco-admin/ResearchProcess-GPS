use crate::{error::ApiResult, state::AppState, ApiError};
use axum::{
    extract::{Path, State},
    response::Json,
};
use rp_storage::{StorageBackend, Transaction};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub date: chrono::NaiveDate,
    pub event_type: String,
    pub description: String,
    pub source_id: Option<Uuid>,
    pub confidence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonTimeline {
    pub person_id: Uuid,
    pub events: Vec<TimelineEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRelationship {
    pub relationship_id: Uuid,
    pub relationship_type: String,
    pub related_person_id: Uuid,
    pub related_person_name: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub confidence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonRelationships {
    pub person_id: Uuid,
    pub relationships: Vec<PersonRelationship>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergePersonsRequest {
    pub target_person_id: Uuid,
    pub merge_strategy: String, // "prefer_source", "prefer_target", "manual"
    pub field_overrides: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergePersonsResponse {
    pub merged_person_id: Uuid,
    pub version: i64,
    pub conflicts_resolved: Vec<String>,
}

/// Get timeline of events for a person
pub async fn get_person_timeline(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PersonTimeline>> {
    // Begin transaction
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    
    // Verify person exists
    let person = tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    // Check if it's a Person/IdentityPersona entity
    if person.entity_type != "IdentityPersona" && person.entity_type != "Person" {
        return Err(ApiError::Validation(
            "Can only get timeline for Person entities".to_string()
        ));
    }
    
    let mut events = Vec::new();
    
    // Extract birth event if available
    if let Some(birth_date) = person.data.get("birth_date") {
        if let Some(date_str) = birth_date.as_str() {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                events.push(TimelineEvent {
                    date,
                    event_type: "birth".to_string(),
                    description: "Birth".to_string(),
                    source_id: None,
                    confidence: "high".to_string(),
                });
            }
        }
    }
    
    // Extract death event if available
    if let Some(death_date) = person.data.get("death_date") {
        if let Some(date_str) = death_date.as_str() {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                events.push(TimelineEvent {
                    date,
                    event_type: "death".to_string(),
                    description: "Death".to_string(),
                    source_id: None,
                    confidence: "high".to_string(),
                });
            }
        }
    }
    
    // Query for fact entities related to this person
    let pool = state.storage.pool().pool();
    let fact_rows: Vec<crate::handlers::entities::EntityRow> = sqlx::query_as(
        r#"
        SELECT id, entity_type, data, binary_data, created_by, created_at, updated_at, version
        FROM entities
        WHERE deleted_at IS NULL
          AND entity_type = 'Fact'
          AND (data->>'person_id' = $1 OR data->>'subject_id' = $1)
        ORDER BY data->>'date' ASC
        "#
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Database(e))?;
    
    // Convert facts to timeline events
    for fact in fact_rows {
        if let Some(date_str) = fact.data.get("date").and_then(|d| d.as_str()) {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                let event_type = fact.data
                    .get("fact_type")
                    .and_then(|t| t.as_str())
                    .unwrap_or("event")
                    .to_string();
                
                let description = fact.data
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("Event")
                    .to_string();
                
                let source_id = fact.data
                    .get("source_id")
                    .and_then(|s| s.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok());
                
                let confidence = fact.data
                    .get("confidence")
                    .and_then(|c| c.as_str())
                    .unwrap_or("medium")
                    .to_string();
                
                events.push(TimelineEvent {
                    date,
                    event_type,
                    description,
                    source_id,
                    confidence,
                });
            }
        }
    }
    
    // Sort events by date
    events.sort_by_key(|e| e.date);
    
    Ok(Json(PersonTimeline {
        person_id: id,
        events,
    }))
}

/// Get all relationships for a person
pub async fn get_person_relationships(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PersonRelationships>> {
    // Begin transaction
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    
    // Verify person exists
    let person = tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    if person.entity_type != "IdentityPersona" && person.entity_type != "Person" {
        return Err(ApiError::Validation(
            "Can only get relationships for Person entities".to_string()
        ));
    }
    
    // Query for relationship entities involving this person
    let pool = state.storage.pool().pool();
    let relationship_rows: Vec<crate::handlers::entities::EntityRow> = sqlx::query_as(
        r#"
        SELECT id, entity_type, data, binary_data, created_by, created_at, updated_at, version
        FROM entities
        WHERE deleted_at IS NULL
          AND entity_type = 'Relationship'
          AND (data->>'person1_id' = $1 OR data->>'person2_id' = $1)
        ORDER BY created_at DESC
        "#
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Database(e))?;
    
    let mut relationships = Vec::new();
    
    for rel in relationship_rows {
        // Determine which person is the "other" in the relationship
        let person1_id = rel.data
            .get("person1_id")
            .and_then(|p| p.as_str())
            .and_then(|s| Uuid::parse_str(s).ok());
        
        let person2_id = rel.data
            .get("person2_id")
            .and_then(|p| p.as_str())
            .and_then(|s| Uuid::parse_str(s).ok());
        
        let related_person_id = if person1_id == Some(id) {
            person2_id
        } else {
            person1_id
        };
        
        if let Some(related_id) = related_person_id {
            // Try to get the related person's name
            let related_person_name = if let Ok(Some(related)) = tx.get_entity(related_id).await {
                related.data
                    .get("name")
                    .and_then(|n| n.as_str())
                    .map(|s| s.to_string())
            } else {
                None
            };
            
            let relationship_type = rel.data
                .get("relationship_type")
                .and_then(|t| t.as_str())
                .unwrap_or("unknown")
                .to_string();
            
            let start_date = rel.data
                .get("start_date")
                .and_then(|d| d.as_str())
                .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
            
            let end_date = rel.data
                .get("end_date")
                .and_then(|d| d.as_str())
                .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
            
            let confidence = rel.data
                .get("confidence")
                .and_then(|c| c.as_str())
                .unwrap_or("medium")
                .to_string();
            
            relationships.push(PersonRelationship {
                relationship_id: rel.id,
                relationship_type,
                related_person_id: related_id,
                related_person_name,
                start_date,
                end_date,
                confidence,
            });
        }
    }
    
    Ok(Json(PersonRelationships {
        person_id: id,
        relationships,
    }))
}

/// Merge two person entities
pub async fn merge_persons(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<MergePersonsRequest>,
) -> ApiResult<Json<MergePersonsResponse>> {
    // Begin transaction
    let pool = state.storage.pool().pool().clone();
    let db_tx = pool.begin().await.map_err(|e| ApiError::Database(e))?;
    
    let mut event_tx = rp_storage_postgres::EventSourcedTransaction::new(
        pool.clone(),
        db_tx,
        Uuid::nil(), // TODO: Get actor ID from auth context
    );
    
    // Get both persons
    let source_person = event_tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    let target_person = event_tx
        .get_entity(request.target_person_id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or_else(|| ApiError::NotFound)?;
    
    // Verify both are person entities
    if (source_person.entity_type != "IdentityPersona" && source_person.entity_type != "Person") ||
       (target_person.entity_type != "IdentityPersona" && target_person.entity_type != "Person") {
        return Err(ApiError::Validation(
            "Can only merge Person entities".to_string()
        ));
    }
    
    // Merge data based on strategy
    let mut merged_data = match request.merge_strategy.as_str() {
        "prefer_source" => source_person.data.clone(),
        "prefer_target" => target_person.data.clone(),
        "manual" => {
            if let Some(overrides) = request.field_overrides {
                overrides
            } else {
                return Err(ApiError::Validation(
                    "Manual merge requires field_overrides".to_string()
                ));
            }
        }
        _ => return Err(ApiError::Validation(
            "Invalid merge strategy".to_string()
        )),
    };
    
    // Track conflicts resolved
    let mut conflicts_resolved = Vec::new();
    
    // For non-manual strategies, merge non-conflicting fields
    if request.merge_strategy != "manual" {
        let _preferred = if request.merge_strategy == "prefer_source" {
            &source_person
        } else {
            &target_person
        };
        let other = if request.merge_strategy == "prefer_source" {
            &target_person
        } else {
            &source_person
        };
        
        // Merge fields from other if not present in preferred
        if let serde_json::Value::Object(preferred_obj) = &mut merged_data {
            if let serde_json::Value::Object(other_obj) = &other.data {
                for (key, value) in other_obj {
                    if !preferred_obj.contains_key(key) {
                        preferred_obj.insert(key.clone(), value.clone());
                    } else if &preferred_obj[key] != value {
                        conflicts_resolved.push(format!("Kept {} from preferred person", key));
                    }
                }
            }
        }
    }
    
    // Add merge metadata
    if let serde_json::Value::Object(obj) = &mut merged_data {
        obj.insert("merged_from".to_string(), serde_json::json!([id, request.target_person_id]));
        obj.insert("merge_date".to_string(), serde_json::json!(chrono::Utc::now()));
    }
    
    // Update the target person with merged data
    let mut updates = std::collections::HashMap::new();
    if let serde_json::Value::Object(obj) = merged_data {
        for (k, v) in obj {
            updates.insert(k, v);
        }
    }
    
    event_tx
        .update_entity(request.target_person_id, updates)
        .await
        .map_err(ApiError::Storage)?;
    
    // Mark source person as merged (soft delete with metadata)
    let mut delete_updates = std::collections::HashMap::new();
    delete_updates.insert("merged_into".to_string(), serde_json::json!(request.target_person_id));
    delete_updates.insert("state".to_string(), serde_json::json!("MERGED"));
    
    event_tx
        .update_entity(id, delete_updates)
        .await
        .map_err(ApiError::Storage)?;
    
    // Commit transaction
    event_tx.commit().await.map_err(ApiError::Storage)?;
    
    // Get the updated target person for version info
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    let merged = tx
        .get_entity(request.target_person_id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    Ok(Json(MergePersonsResponse {
        merged_person_id: request.target_person_id,
        version: merged.version as i64,
        conflicts_resolved,
    }))
}