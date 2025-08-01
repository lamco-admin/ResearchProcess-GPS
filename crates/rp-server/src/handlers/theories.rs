use crate::{error::ApiResult, state::AppState, ApiError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use rp_core::layer3::EntityType;
use rp_protocol::{EntityResponse, ListResponse};
use rp_storage::{StorageBackend, Transaction};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BranchTheoryRequest {
    pub branch_name: String,
    pub hypothesis: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BranchTheoryResponse {
    pub id: Uuid,
    pub parent_theory_id: Uuid,
    pub branch_name: String,
    pub hypothesis: String,
    pub version: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ComplianceStatus {
    pub compliant: bool,
    pub issues: Vec<ComplianceIssue>,
    pub checked_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ComplianceIssue {
    pub rule_id: String,
    pub severity: String,
    pub message: String,
}

/// Create a branch from an existing theory
#[utoipa::path(
    post,
    path = "/api/v1/theories/{id}/branch",
    params(
        ("id" = Uuid, Path, description = "Theory ID to branch from")
    ),
    request_body = BranchTheoryRequest,
    responses(
        (status = 201, description = "Theory branch created successfully", body = BranchTheoryResponse),
        (status = 404, description = "Theory not found"),
        (status = 400, description = "Invalid request - can only branch from Theory entities"),
        (status = 500, description = "Internal server error")
    ),
    tag = "theories"
)]
pub async fn branch_theory(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<BranchTheoryRequest>,
) -> ApiResult<(StatusCode, Json<BranchTheoryResponse>)> {
    // Begin transaction
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    
    // Verify parent theory exists
    let parent = tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    // Verify it's actually a Theory
    if parent.entity_type != "Theory" {
        return Err(ApiError::Validation(
            "Can only branch from Theory entities".to_string()
        ));
    }
    
    // Create the branch theory with reference to parent
    let mut branch_data = serde_json::json!({
        "parent_theory_id": id,
        "branch_name": request.branch_name,
        "hypothesis": request.hypothesis,
        "state": "ACTIVE",
        "created_from_branch": true
    });
    
    // Copy relevant fields from parent
    if let Some(question) = parent.data.get("question") {
        branch_data["question"] = question.clone();
    }
    
    // Create new theory entity for the branch
    let branch_entity = rp_storage::StorageEntity {
        id: Uuid::now_v7(),
        entity_type: "Theory".to_string(),
        data: branch_data,
        binary_data: None,
        created_by: Uuid::nil(), // TODO: Get from auth context
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        version: 1,
    };
    
    // Use event-sourced transaction for proper event handling
    let pool = state.storage.pool().pool().clone();
    let db_tx = pool.begin().await.map_err(|e| ApiError::Database(e))?;
    
    let mut event_tx = rp_storage_postgres::EventSourcedTransaction::new(
        pool.clone(),
        db_tx,
        Uuid::nil(), // TODO: Get actor ID from auth context
    );
    
    // Store the branch
    event_tx
        .put_entity(&branch_entity)
        .await
        .map_err(ApiError::Storage)?;
    
    // Commit transaction
    event_tx.commit().await.map_err(ApiError::Storage)?;
    
    let response = BranchTheoryResponse {
        id: branch_entity.id,
        parent_theory_id: id,
        branch_name: request.branch_name,
        hypothesis: request.hypothesis,
        version: 1,
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

/// Get all evidence associated with a theory
#[utoipa::path(
    get,
    path = "/api/v1/theories/{id}/evidence",
    params(
        ("id" = Uuid, Path, description = "Theory ID")
    ),
    responses(
        (status = 200, description = "Evidence list retrieved successfully", body = ListResponse<EntityResponse>),
        (status = 404, description = "Theory not found"),
        (status = 400, description = "Invalid request - can only get evidence for Theory entities"),
        (status = 500, description = "Internal server error")
    ),
    tag = "theories"
)]
pub async fn get_theory_evidence(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ListResponse<EntityResponse>>> {
    // Begin transaction
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    
    // Verify theory exists
    let theory = tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    if theory.entity_type != "Theory" {
        return Err(ApiError::Validation(
            "Can only get evidence for Theory entities".to_string()
        ));
    }
    
    // Query evidence that references this theory
    let pool = state.storage.pool().pool();
    
    let evidence_rows: Vec<crate::handlers::entities::EntityRow> = sqlx::query_as(
        r#"
        SELECT id, entity_type, data, binary_data, created_by, created_at, updated_at, version
        FROM entities
        WHERE deleted_at IS NULL
          AND entity_type = 'Evidence'
          AND data->>'theory_id' = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Database(e))?;
    
    // Convert to response format
    let items: Vec<EntityResponse> = evidence_rows
        .into_iter()
        .map(|entity| EntityResponse {
            id: entity.id,
            entity_type: EntityType::Evidence,
            version: entity.version,
            data: entity.data,
            metadata: rp_protocol::EntityMetadata {
                created_at: entity.created_at,
                updated_at: entity.updated_at,
                created_by: entity.created_by,
                updated_by: Some(entity.created_by),
                workspace_id: None,
                tags: None,
            },
        })
        .collect();
    
    let total = items.len() as u64;
    
    Ok(Json(ListResponse {
        items,
        total,
        limit: 100,
        offset: 0,
        has_more: false,
        cursor: None,
    }))
}

/// Get compliance status for a theory
#[utoipa::path(
    get,
    path = "/api/v1/theories/{id}/compliance-status",
    params(
        ("id" = Uuid, Path, description = "Theory ID")
    ),
    responses(
        (status = 200, description = "Compliance status retrieved successfully", body = ComplianceStatus),
        (status = 404, description = "Theory not found"),
        (status = 400, description = "Invalid request - can only check compliance for Theory entities"),
        (status = 500, description = "Internal server error")
    ),
    tag = "theories"
)]
pub async fn get_theory_compliance_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ComplianceStatus>> {
    // Begin transaction
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    
    // Verify theory exists
    let theory = tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    if theory.entity_type != "Theory" {
        return Err(ApiError::Validation(
            "Can only check compliance for Theory entities".to_string()
        ));
    }
    
    // Check compliance rules
    let mut issues = Vec::new();
    
    // Example compliance checks
    // Check 1: Theory must have a question
    if theory.data.get("question").is_none() {
        issues.push(ComplianceIssue {
            rule_id: "THEORY_QUESTION_REQUIRED".to_string(),
            severity: "error".to_string(),
            message: "Theory must have a research question".to_string(),
        });
    }
    
    // Check 2: Theory must have a hypothesis
    if theory.data.get("hypothesis").is_none() {
        issues.push(ComplianceIssue {
            rule_id: "THEORY_HYPOTHESIS_REQUIRED".to_string(),
            severity: "error".to_string(),
            message: "Theory must have a hypothesis".to_string(),
        });
    }
    
    // Check 3: Active theories should have evidence (warning only)
    if theory.data.get("state") == Some(&serde_json::Value::String("ACTIVE".to_string())) {
        // Check if theory has any evidence
        let pool = state.storage.pool().pool();
        let evidence_count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM entities
            WHERE deleted_at IS NULL
              AND entity_type = 'Evidence'
              AND data->>'theory_id' = $1
            "#
        )
        .bind(id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::Database(e))?;
        
        if evidence_count.0 == 0 {
            issues.push(ComplianceIssue {
                rule_id: "THEORY_EVIDENCE_RECOMMENDED".to_string(),
                severity: "warning".to_string(),
                message: "Active theories should have supporting evidence".to_string(),
            });
        }
    }
    
    let compliant = issues.iter().all(|issue| issue.severity != "error");
    
    Ok(Json(ComplianceStatus {
        compliant,
        issues,
        checked_at: chrono::Utc::now(),
    }))
}