use crate::{error::ApiResult, state::AppState, ApiError};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use rp_storage::{StorageBackend, Transaction};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WorkspaceMember {
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub invited_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WorkspaceMembers {
    pub workspace_id: Uuid,
    pub members: Vec<WorkspaceMember>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InviteMemberRequest {
    pub user_email: String,
    pub role: String,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InviteMemberResponse {
    pub invitation_id: Uuid,
    pub workspace_id: Uuid,
    pub invited_email: String,
    pub status: String,
}

/// Get members of a workspace
#[utoipa::path(
    get,
    path = "/api/v1/workspaces/{id}/members",
    params(
        ("id" = Uuid, Path, description = "Workspace ID")
    ),
    responses(
        (status = 200, description = "Workspace members retrieved successfully", body = WorkspaceMembers),
        (status = 404, description = "Workspace not found"),
        (status = 400, description = "Invalid request - can only get members for Workspace entities"),
        (status = 500, description = "Internal server error")
    ),
    tag = "workspaces"
)]
pub async fn list_workspace_members(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorkspaceMembers>> {
    // Begin transaction
    let mut tx = state.storage.begin_transaction().await.map_err(ApiError::Storage)?;
    
    // Verify workspace exists
    let workspace = tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    // Check if it's a Workspace entity
    if workspace.entity_type != "Workspace" {
        return Err(ApiError::Validation(
            "Can only get members for Workspace entities".to_string()
        ));
    }
    
    // Get members from workspace data
    let members_data = workspace.data
        .get("members")
        .and_then(|m| m.as_array())
        .cloned()
        .unwrap_or_default();
    
    let mut members = Vec::new();
    
    for member_value in members_data {
        if let Some(member_obj) = member_value.as_object() {
            let user_id = member_obj
                .get("user_id")
                .and_then(|id| id.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
                .unwrap_or(Uuid::nil());
            
            let role = member_obj
                .get("role")
                .and_then(|r| r.as_str())
                .unwrap_or("member")
                .to_string();
            
            let joined_at = member_obj
                .get("joined_at")
                .and_then(|d| d.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(chrono::Utc::now);
            
            let invited_by = member_obj
                .get("invited_by")
                .and_then(|id| id.as_str())
                .and_then(|s| Uuid::parse_str(s).ok());
            
            members.push(WorkspaceMember {
                user_id,
                role,
                joined_at,
                invited_by,
            });
        }
    }
    
    let total = members.len() as u64;
    
    Ok(Json(WorkspaceMembers {
        workspace_id: id,
        members,
        total,
    }))
}

/// Invite a member to workspace
#[utoipa::path(
    post,
    path = "/api/v1/workspaces/{id}/members/invite",
    params(
        ("id" = Uuid, Path, description = "Workspace ID")
    ),
    request_body = InviteMemberRequest,
    responses(
        (status = 201, description = "Member invited successfully", body = InviteMemberResponse),
        (status = 404, description = "Workspace not found"),
        (status = 400, description = "Invalid request - can only invite members to Workspace entities"),
        (status = 500, description = "Internal server error")
    ),
    tag = "workspaces"
)]
pub async fn invite_workspace_member(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<InviteMemberRequest>,
) -> ApiResult<(StatusCode, Json<InviteMemberResponse>)> {
    // Begin transaction
    let pool = state.storage.pool().pool().clone();
    let db_tx = pool.begin().await.map_err(|e| ApiError::Database(e))?;
    
    let mut event_tx = rp_storage_postgres::EventSourcedTransaction::new(
        pool.clone(),
        db_tx,
        Uuid::nil(), // TODO: Get actor ID from auth context
    );
    
    // Verify workspace exists
    let workspace = event_tx
        .get_entity(id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    if workspace.entity_type != "Workspace" {
        return Err(ApiError::Validation(
            "Can only invite members to Workspace entities".to_string()
        ));
    }
    
    // Create invitation entity
    let invitation_id = Uuid::now_v7();
    let invitation_data = serde_json::json!({
        "workspace_id": id,
        "invited_email": request.user_email,
        "role": request.role,
        "message": request.message,
        "status": "pending",
        "invited_by": Uuid::nil(), // TODO: Get from auth context
        "invited_at": chrono::Utc::now(),
        "expires_at": chrono::Utc::now() + chrono::Duration::days(7),
    });
    
    // Store invitation as a WorkProduct entity (or we could create a specific Invitation type)
    let invitation_entity = rp_storage::StorageEntity {
        id: invitation_id,
        entity_type: "WorkProduct".to_string(), // Using WorkProduct as a document type
        data: invitation_data,
        binary_data: None,
        created_by: Uuid::nil(), // TODO: Get from auth context
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        version: 1,
    };
    
    event_tx
        .put_entity(&invitation_entity)
        .await
        .map_err(ApiError::Storage)?;
    
    // Update workspace to track pending invitations
    let mut updates = std::collections::HashMap::new();
    
    // Get current pending invitations
    let mut pending_invitations = workspace.data
        .get("pending_invitations")
        .and_then(|i| i.as_array())
        .cloned()
        .unwrap_or_default();
    
    // Add new invitation
    pending_invitations.push(serde_json::json!({
        "invitation_id": invitation_id,
        "email": request.user_email,
        "invited_at": chrono::Utc::now(),
    }));
    
    updates.insert("pending_invitations".to_string(), serde_json::json!(pending_invitations));
    
    event_tx
        .update_entity(id, updates)
        .await
        .map_err(ApiError::Storage)?;
    
    // Commit transaction
    event_tx.commit().await.map_err(ApiError::Storage)?;
    
    // In a real implementation, send invitation email here
    
    let response = InviteMemberResponse {
        invitation_id,
        workspace_id: id,
        invited_email: request.user_email,
        status: "sent".to_string(),
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}

/// Remove a member from workspace
#[utoipa::path(
    delete,
    path = "/api/v1/workspaces/{workspace_id}/members/{member_id}",
    params(
        ("workspace_id" = Uuid, Path, description = "Workspace ID"),
        ("member_id" = Uuid, Path, description = "Member ID to remove")
    ),
    responses(
        (status = 204, description = "Member removed successfully"),
        (status = 404, description = "Workspace or member not found"),
        (status = 400, description = "Invalid request - cannot remove last owner from workspace"),
        (status = 500, description = "Internal server error")
    ),
    tag = "workspaces"
)]
pub async fn remove_workspace_member(
    State(state): State<Arc<AppState>>,
    Path((workspace_id, member_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<StatusCode> {
    // Begin transaction
    let pool = state.storage.pool().pool().clone();
    let db_tx = pool.begin().await.map_err(|e| ApiError::Database(e))?;
    
    let mut event_tx = rp_storage_postgres::EventSourcedTransaction::new(
        pool.clone(),
        db_tx,
        Uuid::nil(), // TODO: Get actor ID from auth context
    );
    
    // Get workspace
    let workspace = event_tx
        .get_entity(workspace_id)
        .await
        .map_err(ApiError::Storage)?
        .ok_or(ApiError::NotFound)?;
    
    if workspace.entity_type != "Workspace" {
        return Err(ApiError::Validation(
            "Can only remove members from Workspace entities".to_string()
        ));
    }
    
    // Get current members
    let mut members = workspace.data
        .get("members")
        .and_then(|m| m.as_array())
        .cloned()
        .unwrap_or_default();
    
    // Find and remove the member
    let original_len = members.len();
    members.retain(|member| {
        member
            .get("user_id")
            .and_then(|id| id.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            != Some(member_id)
    });
    
    if members.len() == original_len {
        return Err(ApiError::NotFound);
    }
    
    // Check if removing would leave workspace without owners
    let owner_count = members.iter().filter(|m| {
        m.get("role")
            .and_then(|r| r.as_str())
            .map(|r| r == "owner")
            .unwrap_or(false)
    }).count();
    
    if owner_count == 0 {
        return Err(ApiError::Validation(
            "Cannot remove last owner from workspace".to_string()
        ));
    }
    
    // Update workspace
    let mut updates = std::collections::HashMap::new();
    updates.insert("members".to_string(), serde_json::json!(members));
    
    event_tx
        .update_entity(workspace_id, updates)
        .await
        .map_err(ApiError::Storage)?;
    
    // Commit transaction
    event_tx.commit().await.map_err(ApiError::Storage)?;
    
    Ok(StatusCode::NO_CONTENT)
}