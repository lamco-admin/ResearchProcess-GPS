use crate::{error::ApiResult, state::AppState, ApiError};
use axum::{
    extract::{Query, State},
    response::Json,
};
use rp_protocol::{
    EntityResponse, EntityMetadata,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use utoipa::{ToSchema, IntoParams};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct SearchQuery {
    // Required search query
    pub q: String,
    
    // Optional filters
    pub entity_type: Option<String>,
    pub workspace_id: Option<Uuid>,
    pub state: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    pub created_before: Option<chrono::DateTime<chrono::Utc>>,
    
    // Pagination
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
    
    // Options
    #[serde(default)]
    pub fuzzy: bool,
    #[serde(default)]
    pub highlight: bool,
}

fn default_limit() -> u32 {
    20
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
    pub has_more: bool,
    pub facets: SearchFacets,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchResult {
    #[serde(flatten)]
    pub entity: EntityResponse,
    pub score: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize, Default, ToSchema)]
pub struct SearchFacets {
    pub entity_types: HashMap<String, u32>,
    pub states: HashMap<String, u32>,
    pub workspaces: HashMap<String, u32>,
}

/// Search entities with full-text search
#[utoipa::path(
    get,
    path = "/api/v1/search",
    params(
        SearchQuery
    ),
    responses(
        (status = 200, description = "Search results retrieved successfully", body = SearchResponse),
        (status = 400, description = "Invalid request - search query cannot be empty"),
        (status = 500, description = "Internal server error")
    ),
    tag = "search"
)]
pub async fn search_entities(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<SearchResponse>> {
    // Validate search query
    if query.q.trim().is_empty() {
        return Err(ApiError::Validation("Search query cannot be empty".to_string()));
    }
    
    // Limit validation
    let limit = if query.limit > 100 { 100 } else if query.limit == 0 { 20 } else { query.limit };
    let offset = query.offset;
    
    // Get database pool
    let pool = state.storage.pool().pool();
    
    // Build search query
    let search_pattern = if query.fuzzy {
        // For fuzzy search, add wildcards
        format!("%{}%", query.q.replace(' ', "%"))
    } else {
        format!("%{}%", query.q)
    };
    
    // Build the main search query
    let mut sql = String::from(
        "SELECT id, entity_type, data, binary_data, created_by, created_at, updated_at, version,
         ts_rank(to_tsvector('english', data::text), plainto_tsquery('english', $1)) as score
         FROM entities 
         WHERE deleted_at IS NULL
         AND (data::text ILIKE $2 OR to_tsvector('english', data::text) @@ plainto_tsquery('english', $1))"
    );
    
    // Add filters
    let mut param_count = 2;
    let mut filter_params = Vec::new();
    
    if let Some(ref entity_type) = query.entity_type {
        param_count += 1;
        sql.push_str(&format!(" AND entity_type = ${}", param_count));
        filter_params.push(entity_type.clone());
    }
    
    if let Some(ref workspace_id) = query.workspace_id {
        param_count += 1;
        sql.push_str(&format!(" AND data->>'workspace_id' = ${}", param_count));
        filter_params.push(workspace_id.to_string());
    }
    
    if let Some(ref state) = query.state {
        param_count += 1;
        sql.push_str(&format!(" AND data->>'state' = ${}", param_count));
        filter_params.push(state.clone());
    }
    
    // Order by relevance score
    sql.push_str(" ORDER BY score DESC, created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    
    // Execute search query
    let mut search_query = sqlx::query_as::<_, SearchResultRow>(&sql)
        .bind(&query.q)
        .bind(&search_pattern);
    
    for param in &filter_params {
        search_query = search_query.bind(param);
    }
    
    let results = search_query
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::Database(e))?;
    
    // Get total count
    let mut count_sql = String::from(
        "SELECT COUNT(*) FROM entities 
         WHERE deleted_at IS NULL
         AND (data::text ILIKE $1 OR to_tsvector('english', data::text) @@ plainto_tsquery('english', $2))"
    );
    
    param_count = 2;
    if query.entity_type.is_some() {
        param_count += 1;
        count_sql.push_str(&format!(" AND entity_type = ${}", param_count));
    }
    if query.workspace_id.is_some() {
        param_count += 1;
        count_sql.push_str(&format!(" AND data->>'workspace_id' = ${}", param_count));
    }
    if query.state.is_some() {
        param_count += 1;
        count_sql.push_str(&format!(" AND data->>'state' = ${}", param_count));
    }
    
    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql)
        .bind(&search_pattern)
        .bind(&query.q);
    
    for param in &filter_params {
        count_query = count_query.bind(param);
    }
    
    let total_row = count_query
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::Database(e))?;
    
    let total = total_row.0 as u64;
    let has_more = (offset as u64 + limit as u64) < total;
    
    // Calculate facets
    let facets = if results.len() < 100 {
        // Only calculate facets for small result sets
        calculate_facets(&results)
    } else {
        SearchFacets::default()
    };
    
    // Convert to response format
    let search_results: Vec<SearchResult> = results
        .into_iter()
        .map(|row| {
            let entity = EntityResponse {
                id: row.id,
                entity_type: row.entity_type.clone(),
                version: row.version,
                data: row.data.clone(),
                metadata: EntityMetadata {
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    created_by: row.created_by,
                    updated_by: Some(row.created_by),
                    workspace_id: None,
                    tags: None,
                },
            };
            
            let highlights = if query.highlight {
                extract_highlights(&row.data, &query.q)
            } else {
                None
            };
            
            Ok(SearchResult {
                entity,
                score: row.score,
                highlights,
            })
        })
        .collect::<Result<Vec<_>, ApiError>>()?;
    
    Ok(Json(SearchResponse {
        results: search_results,
        total,
        limit,
        offset,
        has_more,
        facets,
    }))
}

// Helper struct for search results
#[derive(sqlx::FromRow)]
struct SearchResultRow {
    pub id: Uuid,
    pub entity_type: String,
    pub data: serde_json::Value,
    #[allow(dead_code)]
    pub binary_data: Option<Vec<u8>>,
    pub created_by: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub version: i64,
    pub score: f32,
}

// Calculate facets from search results
fn calculate_facets(results: &[SearchResultRow]) -> SearchFacets {
    let mut facets = SearchFacets::default();
    
    for result in results {
        // Entity type facet
        *facets.entity_types.entry(result.entity_type.clone()).or_insert(0) += 1;
        
        // State facet
        if let Some(state) = result.data.get("state").and_then(|s| s.as_str()) {
            *facets.states.entry(state.to_string()).or_insert(0) += 1;
        }
        
        // Workspace facet
        if let Some(workspace_id) = result.data.get("workspace_id").and_then(|w| w.as_str()) {
            *facets.workspaces.entry(workspace_id.to_string()).or_insert(0) += 1;
        }
    }
    
    facets
}

// Extract highlighted snippets from data
fn extract_highlights(data: &serde_json::Value, query: &str) -> Option<HashMap<String, Vec<String>>> {
    let mut highlights = HashMap::new();
    let query_lower = query.to_lowercase();
    
    // Search through all string fields in the JSON
    if let serde_json::Value::Object(map) = data {
        for (key, value) in map {
            if let Some(text) = value.as_str() {
                if text.to_lowercase().contains(&query_lower) {
                    // Extract snippet around the match
                    let snippet = extract_snippet(text, &query_lower, 50);
                    highlights.entry(key.clone()).or_insert_with(Vec::new).push(snippet);
                }
            }
        }
    }
    
    if highlights.is_empty() {
        None
    } else {
        Some(highlights)
    }
}

// Extract a snippet of text around the search query
fn extract_snippet(text: &str, query: &str, context_chars: usize) -> String {
    let text_lower = text.to_lowercase();
    if let Some(pos) = text_lower.find(query) {
        let start = pos.saturating_sub(context_chars);
        let end = (pos + query.len() + context_chars).min(text.len());
        
        let mut snippet = String::new();
        if start > 0 {
            snippet.push_str("...");
        }
        snippet.push_str(&text[start..end]);
        if end < text.len() {
            snippet.push_str("...");
        }
        
        snippet
    } else {
        text.chars().take(context_chars * 2).collect()
    }
}