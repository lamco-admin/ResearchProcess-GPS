//! Person Authority Control REST API Endpoints
//!
//! Comprehensive API for Person entity with authority control patterns:
//! - Person (canonical authority records)
//! - VariantName (name variants - MARC 4XX pattern)
//! - PersonRelationship (family relationships - MARC 5XX pattern)
//! - SourcePerson (source attribution - MARC $9 pattern)
//! - PersonMerge (duplicate handling)

use crate::{error::ApiResult, state::AppState, ApiError};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use rp_core::person::{
    Person, VariantName, VariantNameType,
    PersonRelationship, PersonRelationshipType,
    SourcePerson, PersonMerge, Sex, PersonConfidence,
    GenealogyDate, DateCertainty,
};
use rp_storage_postgres::{PersonRepository, PostgresPersonRepository};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use utoipa::ToSchema;

// =============================================================================
// REQUEST/RESPONSE TYPES
// =============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePersonRequest {
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub birth_year: Option<i32>,
    pub birth_month: Option<u8>,
    pub birth_day: Option<u8>,
    pub death_year: Option<i32>,
    pub death_month: Option<u8>,
    pub death_day: Option<u8>,
    pub sex: Option<Sex>,
    pub occupation: Option<String>,
    pub religion: Option<String>,
    pub notes: Option<String>,
    pub conclusion_confidence: Option<PersonConfidence>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePersonRequest {
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub birth_year: Option<i32>,
    pub birth_month: Option<u8>,
    pub birth_day: Option<u8>,
    pub death_year: Option<i32>,
    pub death_month: Option<u8>,
    pub death_day: Option<u8>,
    pub sex: Option<Sex>,
    pub occupation: Option<String>,
    pub religion: Option<String>,
    pub notes: Option<String>,
    pub conclusion_confidence: Option<PersonConfidence>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PersonResponse {
    pub person_id: Uuid,
    pub canonical_name: String,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub birth_date: Option<GenealogyDate>,
    pub death_date: Option<GenealogyDate>,
    pub sex: Sex,
    pub occupation: Option<String>,
    pub religion: Option<String>,
    pub notes: Option<String>,
    pub conclusion_confidence: Option<PersonConfidence>,
    pub is_archived: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Person> for PersonResponse {
    fn from(person: Person) -> Self {
        let is_archived = person.is_archived();
        Self {
            person_id: person.person_id,
            canonical_name: person.canonical_name(),
            given_name: person.given_name,
            surname: person.surname,
            middle_name: person.middle_name,
            name_prefix: person.name_prefix,
            name_suffix: person.name_suffix,
            birth_date: person.birth_date,
            death_date: person.death_date,
            sex: person.sex,
            occupation: person.occupation,
            religion: person.religion,
            notes: person.notes,
            conclusion_confidence: person.conclusion_confidence,
            is_archived,
            created_at: person.created_at,
            updated_at: person.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddVariantNameRequest {
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub variant_type: VariantNameType,
    pub use_from_year: Option<i32>,
    pub use_to_year: Option<i32>,
    pub notes: Option<String>,
    pub source_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VariantNameResponse {
    pub variant_id: Uuid,
    pub person_id: Uuid,
    pub full_name: String,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub variant_type: VariantNameType,
    pub use_from_year: Option<i32>,
    pub use_to_year: Option<i32>,
    pub notes: Option<String>,
    pub source_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<VariantName> for VariantNameResponse {
    fn from(variant: VariantName) -> Self {
        Self {
            variant_id: variant.variant_id,
            person_id: variant.person_id,
            full_name: variant.full_name,
            given_name: variant.given_name,
            surname: variant.surname,
            middle_name: variant.middle_name,
            name_prefix: variant.name_prefix,
            name_suffix: variant.name_suffix,
            variant_type: variant.variant_type,
            use_from_year: variant.use_from_year,
            use_to_year: variant.use_to_year,
            notes: variant.notes,
            source_id: variant.source_id,
            created_at: variant.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddRelationshipRequest {
    pub related_person_id: Uuid,
    pub relationship_type: PersonRelationshipType,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub notes: Option<String>,
    pub source_id: Option<Uuid>,
    pub create_reciprocal: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RelationshipResponse {
    pub relationship_id: Uuid,
    pub person_id: Uuid,
    pub related_person_id: Uuid,
    pub relationship_type: PersonRelationshipType,
    pub start_date: Option<GenealogyDate>,
    pub end_date: Option<GenealogyDate>,
    pub notes: Option<String>,
    pub source_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<PersonRelationship> for RelationshipResponse {
    fn from(rel: PersonRelationship) -> Self {
        Self {
            relationship_id: rel.relationship_id,
            person_id: rel.person_id,
            related_person_id: rel.related_person_id,
            relationship_type: rel.relationship_type,
            start_date: rel.start_date,
            end_date: rel.end_date,
            notes: rel.notes,
            source_id: rel.source_id,
            created_at: rel.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AddSourceLinkRequest {
    pub source_id: Uuid,
    pub extracted_name_full: Option<String>,
    pub extracted_name_given: Option<String>,
    pub extracted_name_surname: Option<String>,
    pub extracted_role: Option<String>,
    pub page_reference: Option<String>,
    pub notes: Option<String>,
    pub confidence: Option<PersonConfidence>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SourceLinkResponse {
    pub source_person_id: Uuid,
    pub person_id: Uuid,
    pub source_id: Uuid,
    pub extracted_name_full: Option<String>,
    pub extracted_name_given: Option<String>,
    pub extracted_name_surname: Option<String>,
    pub extracted_role: Option<String>,
    pub page_reference: Option<String>,
    pub notes: Option<String>,
    pub confidence: PersonConfidence,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<SourcePerson> for SourceLinkResponse {
    fn from(sp: SourcePerson) -> Self {
        Self {
            source_person_id: sp.source_person_id,
            person_id: sp.person_id,
            source_id: sp.source_id,
            extracted_name_full: sp.extracted_name_full,
            extracted_name_given: sp.extracted_name_given,
            extracted_name_surname: sp.extracted_name_surname,
            extracted_role: sp.extracted_role,
            page_reference: sp.page_reference,
            notes: sp.notes,
            confidence: sp.confidence,
            created_at: sp.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateMergeRequest {
    pub target_person_id: Uuid,
    pub merge_reason: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReverseMergeRequest {
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MergeResponse {
    pub merge_id: Uuid,
    pub source_person_id: Uuid,
    pub target_person_id: Uuid,
    pub merge_reason: String,
    pub merged_at: chrono::DateTime<chrono::Utc>,
    pub merged_by: Uuid,
    pub is_reversed: bool,
}

impl From<PersonMerge> for MergeResponse {
    fn from(merge: PersonMerge) -> Self {
        Self {
            merge_id: merge.merge_id,
            source_person_id: merge.source_person_id,
            target_person_id: merge.target_person_id,
            merge_reason: merge.merge_reason,
            merged_at: merge.merged_at,
            merged_by: merge.merged_by,
            is_reversed: merge.reversed_at.is_some(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PersonWithAllDataResponse {
    pub person: PersonResponse,
    pub variants: Vec<VariantNameResponse>,
    pub relationships: Vec<RelationshipResponse>,
    pub sources: Vec<SourceLinkResponse>,
    pub merges: Vec<MergeResponse>,
}

#[derive(Debug, Deserialize, ToSchema, utoipa::IntoParams)]
pub struct ListPersonsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub include_archived: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema, utoipa::IntoParams)]
pub struct SearchPersonsQuery {
    pub query: String,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PersonListResponse {
    pub persons: Vec<PersonResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ArchivePersonRequest {
    pub reason: String,
}

// =============================================================================
// PERSON CRUD ENDPOINTS
// =============================================================================

/// Create a new person
#[utoipa::path(
    post,
    path = "/api/v1/persons/authority",
    request_body = CreatePersonRequest,
    responses(
        (status = 201, description = "Person created successfully", body = PersonResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn create_person(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreatePersonRequest>,
) -> ApiResult<Json<PersonResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Get user ID from auth context (TODO: implement proper auth)
    let user_id = Uuid::now_v7();

    // Build birth date if any birth fields are present
    let birth_date = if request.birth_year.is_some() {
        Some(GenealogyDate {
            year: request.birth_year,
            month: request.birth_month,
            day: request.birth_day,
            certainty: DateCertainty::Exact,
            circa: false,
            end_year: None,
            original_text: None,
        })
    } else {
        None
    };

    // Build death date if any death fields are present
    let death_date = if request.death_year.is_some() {
        Some(GenealogyDate {
            year: request.death_year,
            month: request.death_month,
            day: request.death_day,
            certainty: DateCertainty::Exact,
            circa: false,
            end_year: None,
            original_text: None,
        })
    } else {
        None
    };

    // Build the person
    let mut builder = Person::builder().created_by(user_id);

    if let Some(given_name) = request.given_name {
        builder = builder.given_name(&given_name);
    }
    if let Some(surname) = request.surname {
        builder = builder.surname(&surname);
    }
    if let Some(middle_name) = request.middle_name {
        builder = builder.middle_name(&middle_name);
    }
    if let Some(name_prefix) = request.name_prefix {
        builder = builder.name_prefix(&name_prefix);
    }
    if let Some(name_suffix) = request.name_suffix {
        builder = builder.name_suffix(&name_suffix);
    }
    if let Some(birth_date) = birth_date {
        builder = builder.birth_date(birth_date);
    }
    if let Some(death_date) = death_date {
        builder = builder.death_date(death_date);
    }
    if let Some(sex) = request.sex {
        builder = builder.sex(sex);
    }
    if let Some(occupation) = request.occupation {
        builder = builder.occupation(&occupation);
    }
    if let Some(religion) = request.religion {
        builder = builder.religion(&religion);
    }
    if let Some(notes) = request.notes {
        builder = builder.notes(&notes);
    }
    if let Some(confidence) = request.conclusion_confidence {
        builder = builder.conclusion_confidence(confidence);
    }

    let person = builder.build()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let created = repo.create_person(&person).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(PersonResponse::from(created)))
}

/// Get a person by ID
#[utoipa::path(
    get,
    path = "/api/v1/persons/authority/{id}",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    responses(
        (status = 200, description = "Person retrieved successfully", body = PersonResponse),
        (status = 404, description = "Person not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn get_person(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PersonResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    let person = repo.get_person(id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(PersonResponse::from(person)))
}

/// Update a person
#[utoipa::path(
    put,
    path = "/api/v1/persons/authority/{id}",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    request_body = UpdatePersonRequest,
    responses(
        (status = 200, description = "Person updated successfully", body = PersonResponse),
        (status = 404, description = "Person not found"),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn update_person(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePersonRequest>,
) -> ApiResult<Json<PersonResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Get existing person
    let mut person = repo.get_person(id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    // Update fields if provided
    if let Some(given_name) = request.given_name {
        person.given_name = Some(given_name);
    }
    if let Some(surname) = request.surname {
        person.surname = Some(surname);
    }
    if let Some(middle_name) = request.middle_name {
        person.middle_name = Some(middle_name);
    }
    if let Some(name_prefix) = request.name_prefix {
        person.name_prefix = Some(name_prefix);
    }
    if let Some(name_suffix) = request.name_suffix {
        person.name_suffix = Some(name_suffix);
    }
    if let Some(sex) = request.sex {
        person.sex = sex;
    }
    if let Some(occupation) = request.occupation {
        person.occupation = Some(occupation);
    }
    if let Some(religion) = request.religion {
        person.religion = Some(religion);
    }
    if let Some(notes) = request.notes {
        person.notes = Some(notes);
    }
    if let Some(confidence) = request.conclusion_confidence {
        person.conclusion_confidence = Some(confidence);
    }

    // Update birth date if any birth fields are provided
    if request.birth_year.is_some() || request.birth_month.is_some() || request.birth_day.is_some() {
        person.birth_date = Some(GenealogyDate {
            year: request.birth_year.or_else(|| person.birth_date.as_ref().and_then(|d| d.year)),
            month: request.birth_month.or_else(|| person.birth_date.as_ref().and_then(|d| d.month)),
            day: request.birth_day.or_else(|| person.birth_date.as_ref().and_then(|d| d.day)),
            certainty: DateCertainty::Exact,
            circa: false,
            end_year: None,
            original_text: None,
        });
    }

    // Update death date if any death fields are provided
    if request.death_year.is_some() || request.death_month.is_some() || request.death_day.is_some() {
        person.death_date = Some(GenealogyDate {
            year: request.death_year.or_else(|| person.death_date.as_ref().and_then(|d| d.year)),
            month: request.death_month.or_else(|| person.death_date.as_ref().and_then(|d| d.month)),
            day: request.death_day.or_else(|| person.death_date.as_ref().and_then(|d| d.day)),
            certainty: DateCertainty::Exact,
            circa: false,
            end_year: None,
            original_text: None,
        });
    }

    let updated = repo.update_person(&person).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(PersonResponse::from(updated)))
}

/// Archive a person (soft delete)
#[utoipa::path(
    post,
    path = "/api/v1/persons/authority/{id}/archive",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    request_body = ArchivePersonRequest,
    responses(
        (status = 204, description = "Person archived successfully"),
        (status = 404, description = "Person not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn archive_person(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<ArchivePersonRequest>,
) -> ApiResult<StatusCode> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());
    let user_id = Uuid::now_v7(); // TODO: Get from auth context

    repo.archive_person(id, request.reason, user_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Unarchive a person
#[utoipa::path(
    post,
    path = "/api/v1/persons/authority/{id}/unarchive",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    responses(
        (status = 204, description = "Person unarchived successfully"),
        (status = 404, description = "Person not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn unarchive_person(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());
    let user_id = Uuid::now_v7(); // TODO: Get from auth context

    repo.unarchive_person(id, user_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// List persons with pagination
#[utoipa::path(
    get,
    path = "/api/v1/persons/authority",
    params(
        ListPersonsQuery
    ),
    responses(
        (status = 200, description = "Persons list retrieved successfully", body = PersonListResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn list_persons(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListPersonsQuery>,
) -> ApiResult<Json<PersonListResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    let limit = query.limit.unwrap_or(50).min(1000);
    let offset = query.offset.unwrap_or(0);
    let include_archived = query.include_archived.unwrap_or(false);

    let persons = if include_archived {
        repo.list_persons(limit, offset).await
    } else {
        repo.list_active_persons(limit, offset).await
    }.map_err(|e| ApiError::Internal(e.to_string()))?;

    let total = if include_archived {
        repo.count_persons().await
    } else {
        repo.count_active_persons().await
    }.map_err(|e| ApiError::Internal(e.to_string()))?;

    let person_responses: Vec<PersonResponse> = persons.into_iter()
        .map(PersonResponse::from)
        .collect();

    Ok(Json(PersonListResponse {
        persons: person_responses,
        total,
        limit,
        offset,
    }))
}

/// Search persons by name
#[utoipa::path(
    get,
    path = "/api/v1/persons/authority/search",
    params(
        SearchPersonsQuery
    ),
    responses(
        (status = 200, description = "Search results retrieved successfully", body = Vec<PersonResponse>),
        (status = 400, description = "Invalid search query"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn search_persons(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchPersonsQuery>,
) -> ApiResult<Json<Vec<PersonResponse>>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    let limit = query.limit.unwrap_or(50).min(1000);

    let persons = repo.search_persons(&query.query, limit).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let person_responses: Vec<PersonResponse> = persons.into_iter()
        .map(PersonResponse::from)
        .collect();

    Ok(Json(person_responses))
}

/// Get person with all related data
#[utoipa::path(
    get,
    path = "/api/v1/persons/authority/{id}/complete",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    responses(
        (status = 200, description = "Person with all data retrieved successfully", body = PersonWithAllDataResponse),
        (status = 404, description = "Person not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn get_person_complete(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PersonWithAllDataResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    let person_data = repo.get_person_with_all_data(id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(PersonWithAllDataResponse {
        person: PersonResponse::from(person_data.person),
        variants: person_data.variants.into_iter().map(VariantNameResponse::from).collect(),
        relationships: person_data.relationships.into_iter().map(RelationshipResponse::from).collect(),
        sources: person_data.sources.into_iter().map(SourceLinkResponse::from).collect(),
        merges: person_data.merges.into_iter().map(MergeResponse::from).collect(),
    }))
}

// =============================================================================
// VARIANT NAME ENDPOINTS
// =============================================================================

/// Add a variant name to a person
#[utoipa::path(
    post,
    path = "/api/v1/persons/authority/{id}/variants",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    request_body = AddVariantNameRequest,
    responses(
        (status = 201, description = "Variant name added successfully", body = VariantNameResponse),
        (status = 404, description = "Person not found"),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn add_variant_name(
    State(state): State<Arc<AppState>>,
    Path(person_id): Path<Uuid>,
    Json(request): Json<AddVariantNameRequest>,
) -> ApiResult<Json<VariantNameResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Verify person exists
    repo.get_person(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    // Build variant name
    let mut builder = VariantName::builder(person_id)
        .variant_type(request.variant_type);

    if let Some(given_name) = request.given_name {
        builder = builder.given_name(&given_name);
    }
    if let Some(surname) = request.surname {
        builder = builder.surname(&surname);
    }
    if let Some(middle_name) = request.middle_name {
        builder = builder.middle_name(&middle_name);
    }
    if let Some(name_prefix) = request.name_prefix {
        builder = builder.name_prefix(&name_prefix);
    }
    if let Some(name_suffix) = request.name_suffix {
        builder = builder.name_suffix(&name_suffix);
    }
    if let Some(use_from_year) = request.use_from_year {
        builder = builder.use_from_year(use_from_year);
    }
    if let Some(use_to_year) = request.use_to_year {
        builder = builder.use_to_year(use_to_year);
    }
    if let Some(notes) = request.notes {
        builder = builder.notes(&notes);
    }
    if let Some(source_id) = request.source_id {
        builder = builder.source_id(source_id);
    }

    let variant = builder.build()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let created = repo.add_variant(&variant).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(VariantNameResponse::from(created)))
}

/// Get all variant names for a person
#[utoipa::path(
    get,
    path = "/api/v1/persons/authority/{id}/variants",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    responses(
        (status = 200, description = "Variant names retrieved successfully", body = Vec<VariantNameResponse>),
        (status = 404, description = "Person not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn get_variant_names(
    State(state): State<Arc<AppState>>,
    Path(person_id): Path<Uuid>,
) -> ApiResult<Json<Vec<VariantNameResponse>>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Verify person exists
    repo.get_person(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    let variants = repo.get_variants(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let variant_responses: Vec<VariantNameResponse> = variants.into_iter()
        .map(VariantNameResponse::from)
        .collect();

    Ok(Json(variant_responses))
}

/// Delete a variant name
#[utoipa::path(
    delete,
    path = "/api/v1/persons/authority/variants/{variant_id}",
    params(
        ("variant_id" = Uuid, Path, description = "Variant name ID")
    ),
    responses(
        (status = 204, description = "Variant name deleted successfully"),
        (status = 404, description = "Variant name not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn delete_variant_name(
    State(state): State<Arc<AppState>>,
    Path(variant_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    repo.delete_variant(variant_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

// =============================================================================
// RELATIONSHIP ENDPOINTS
// =============================================================================

/// Add a relationship to a person
#[utoipa::path(
    post,
    path = "/api/v1/persons/authority/{id}/relationships",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    request_body = AddRelationshipRequest,
    responses(
        (status = 201, description = "Relationship added successfully", body = RelationshipResponse),
        (status = 404, description = "Person not found"),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn add_relationship(
    State(state): State<Arc<AppState>>,
    Path(person_id): Path<Uuid>,
    Json(request): Json<AddRelationshipRequest>,
) -> ApiResult<Json<RelationshipResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Verify both persons exist
    repo.get_person(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    repo.get_person(request.related_person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::Validation("Related person not found".to_string()))?;

    // Build start date if year is provided
    let start_date = request.start_year.map(|year| GenealogyDate {
        year: Some(year),
        month: None,
        day: None,
        certainty: DateCertainty::Exact,
        circa: false,
        end_year: None,
        original_text: None,
    });

    // Build end date if year is provided
    let end_date = request.end_year.map(|year| GenealogyDate {
        year: Some(year),
        month: None,
        day: None,
        certainty: DateCertainty::Exact,
        circa: false,
        end_year: None,
        original_text: None,
    });

    // Build relationship
    let mut builder = PersonRelationship::builder()
        .person_id(person_id)
        .related_person_id(request.related_person_id)
        .relationship_type(request.relationship_type);

    if let Some(start_date) = start_date {
        builder = builder.start_date(start_date);
    }
    if let Some(end_date) = end_date {
        builder = builder.end_date(end_date);
    }
    if let Some(notes) = request.notes {
        builder = builder.notes(&notes);
    }
    if let Some(source_id) = request.source_id {
        builder = builder.source_id(source_id);
    }

    let relationship = builder.build()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let created = repo.add_relationship(&relationship).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    // Create reciprocal relationship if requested
    if request.create_reciprocal.unwrap_or(true) {
        let reciprocal_type = request.relationship_type.reciprocal();
        let mut reciprocal_builder = PersonRelationship::builder()
            .person_id(request.related_person_id)
            .related_person_id(person_id)
            .relationship_type(reciprocal_type);

        if let Some(start_date) = relationship.start_date {
            reciprocal_builder = reciprocal_builder.start_date(start_date);
        }
        if let Some(end_date) = relationship.end_date {
            reciprocal_builder = reciprocal_builder.end_date(end_date);
        }
        if let Some(ref notes) = relationship.notes {
            reciprocal_builder = reciprocal_builder.notes(notes);
        }
        if let Some(source_id) = relationship.source_id {
            reciprocal_builder = reciprocal_builder.source_id(source_id);
        }

        let reciprocal = reciprocal_builder.build()
            .map_err(|e| ApiError::Validation(e.to_string()))?;

        repo.add_relationship(&reciprocal).await
            .map_err(|e| ApiError::Internal(e.to_string()))?;
    }

    Ok(Json(RelationshipResponse::from(created)))
}

/// Get all relationships for a person
#[utoipa::path(
    get,
    path = "/api/v1/persons/authority/{id}/relationships",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    responses(
        (status = 200, description = "Relationships retrieved successfully", body = Vec<RelationshipResponse>),
        (status = 404, description = "Person not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn get_relationships(
    State(state): State<Arc<AppState>>,
    Path(person_id): Path<Uuid>,
) -> ApiResult<Json<Vec<RelationshipResponse>>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Verify person exists
    repo.get_person(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    let relationships = repo.get_relationships(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let relationship_responses: Vec<RelationshipResponse> = relationships.into_iter()
        .map(RelationshipResponse::from)
        .collect();

    Ok(Json(relationship_responses))
}

/// Delete a relationship
#[utoipa::path(
    delete,
    path = "/api/v1/persons/authority/relationships/{relationship_id}",
    params(
        ("relationship_id" = Uuid, Path, description = "Relationship ID")
    ),
    responses(
        (status = 204, description = "Relationship deleted successfully"),
        (status = 404, description = "Relationship not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn delete_relationship(
    State(state): State<Arc<AppState>>,
    Path(relationship_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    repo.delete_relationship(relationship_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

// =============================================================================
// SOURCE LINK ENDPOINTS
// =============================================================================

/// Add a source link to a person
#[utoipa::path(
    post,
    path = "/api/v1/persons/authority/{id}/sources",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    request_body = AddSourceLinkRequest,
    responses(
        (status = 201, description = "Source link added successfully", body = SourceLinkResponse),
        (status = 404, description = "Person not found"),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn add_source_link(
    State(state): State<Arc<AppState>>,
    Path(person_id): Path<Uuid>,
    Json(request): Json<AddSourceLinkRequest>,
) -> ApiResult<Json<SourceLinkResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Verify person exists
    repo.get_person(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    // Build source link
    let mut builder = SourcePerson::builder()
        .person_id(person_id)
        .source_id(request.source_id);

    if let Some(name) = request.extracted_name_full {
        builder = builder.extracted_name_full(name);
    }
    if let Some(name) = request.extracted_name_given {
        builder = builder.extracted_name_given(name);
    }
    if let Some(name) = request.extracted_name_surname {
        builder = builder.extracted_name_surname(name);
    }
    if let Some(role) = request.extracted_role {
        builder = builder.extracted_role(role);
    }
    if let Some(page_ref) = request.page_reference {
        builder = builder.page_reference(page_ref);
    }
    if let Some(notes) = request.notes {
        builder = builder.notes(notes);
    }
    if let Some(confidence) = request.confidence {
        builder = builder.confidence(confidence);
    }

    let source_link = builder.build()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let created = repo.add_source_link(&source_link).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(SourceLinkResponse::from(created)))
}

/// Get all source links for a person
#[utoipa::path(
    get,
    path = "/api/v1/persons/authority/{id}/sources",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    responses(
        (status = 200, description = "Source links retrieved successfully", body = Vec<SourceLinkResponse>),
        (status = 404, description = "Person not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn get_source_links(
    State(state): State<Arc<AppState>>,
    Path(person_id): Path<Uuid>,
) -> ApiResult<Json<Vec<SourceLinkResponse>>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Verify person exists
    repo.get_person(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    let source_links = repo.get_source_links(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let source_responses: Vec<SourceLinkResponse> = source_links.into_iter()
        .map(SourceLinkResponse::from)
        .collect();

    Ok(Json(source_responses))
}

/// Delete a source link
#[utoipa::path(
    delete,
    path = "/api/v1/persons/authority/sources/{source_person_id}",
    params(
        ("source_person_id" = Uuid, Path, description = "Source link ID")
    ),
    responses(
        (status = 204, description = "Source link deleted successfully"),
        (status = 404, description = "Source link not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn delete_source_link(
    State(state): State<Arc<AppState>>,
    Path(source_person_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    repo.delete_source_link(source_person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

// =============================================================================
// MERGE ENDPOINTS
// =============================================================================

/// Create a merge (merge source person into target)
#[utoipa::path(
    post,
    path = "/api/v1/persons/authority/{id}/merge",
    params(
        ("id" = Uuid, Path, description = "Source person ID (will be archived)")
    ),
    request_body = CreateMergeRequest,
    responses(
        (status = 201, description = "Merge created successfully", body = MergeResponse),
        (status = 404, description = "Person not found"),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn create_merge(
    State(state): State<Arc<AppState>>,
    Path(source_person_id): Path<Uuid>,
    Json(request): Json<CreateMergeRequest>,
) -> ApiResult<Json<MergeResponse>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());
    let user_id = Uuid::now_v7(); // TODO: Get from auth context

    // Verify both persons exist
    repo.get_person(source_person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    repo.get_person(request.target_person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or_else(|| ApiError::Validation("Target person not found".to_string()))?;

    // Build merge record
    let merge = PersonMerge::builder()
        .source_person_id(source_person_id)
        .target_person_id(request.target_person_id)
        .merge_reason(&request.merge_reason)
        .merged_by(user_id)
        .build()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let created = repo.create_merge(&merge).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(MergeResponse::from(created)))
}

/// Reverse a merge
#[utoipa::path(
    post,
    path = "/api/v1/persons/authority/merges/{merge_id}/reverse",
    params(
        ("merge_id" = Uuid, Path, description = "Merge ID")
    ),
    request_body = ReverseMergeRequest,
    responses(
        (status = 204, description = "Merge reversed successfully"),
        (status = 404, description = "Merge not found"),
        (status = 400, description = "Merge already reversed"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn reverse_merge(
    State(state): State<Arc<AppState>>,
    Path(merge_id): Path<Uuid>,
    Json(request): Json<ReverseMergeRequest>,
) -> ApiResult<StatusCode> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());
    let user_id = Uuid::now_v7(); // TODO: Get from auth context

    repo.reverse_merge(merge_id, request.reason, user_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

/// Get all merges for a person
#[utoipa::path(
    get,
    path = "/api/v1/persons/authority/{id}/merges",
    params(
        ("id" = Uuid, Path, description = "Person ID")
    ),
    responses(
        (status = 200, description = "Merges retrieved successfully", body = Vec<MergeResponse>),
        (status = 404, description = "Person not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "persons-authority"
)]
pub async fn get_merges(
    State(state): State<Arc<AppState>>,
    Path(person_id): Path<Uuid>,
) -> ApiResult<Json<Vec<MergeResponse>>> {
    let repo = PostgresPersonRepository::new(state.pg_pool.clone());

    // Verify person exists
    repo.get_person(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?
        .ok_or(ApiError::NotFound)?;

    let merges = repo.get_merges_for_person(person_id).await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let merge_responses: Vec<MergeResponse> = merges.into_iter()
        .map(MergeResponse::from)
        .collect();

    Ok(Json(merge_responses))
}
