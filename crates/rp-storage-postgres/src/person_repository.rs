//! Person entity repository for PostgreSQL storage
//!
//! Implements complete CRUD operations for the Person authority control system:
//! - Person (canonical authority records)
//! - VariantName (name variants - MARC 4XX)
//! - PersonRelationship (family relationships - MARC 5XX)
//! - SourcePerson (source attribution - MARC $9)
//! - PersonMerge (duplicate handling with audit trail)

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use tracing::{debug, instrument};
use uuid::Uuid;

use rp_core::person::{
    Person, Sex, PersonConfidence, GenealogyDate, DateCertainty,
    VariantName, VariantNameType,
    PersonRelationship, PersonRelationshipType,
    SourcePerson,
    PersonMerge,
};

use crate::PostgresResult;

// =============================================================================
// REPOSITORY TRAIT
// =============================================================================

/// Repository trait for Person entity operations
#[async_trait]
pub trait PersonRepository: Send + Sync {
    // --- Person CRUD ---
    async fn create_person(&self, person: &Person) -> PostgresResult<Person>;
    async fn get_person(&self, person_id: Uuid) -> PostgresResult<Option<Person>>;
    async fn update_person(&self, person: &Person) -> PostgresResult<Person>;
    async fn archive_person(&self, person_id: Uuid, reason: String, user_id: Uuid) -> PostgresResult<()>;
    async fn unarchive_person(&self, person_id: Uuid, user_id: Uuid) -> PostgresResult<()>;
    async fn list_persons(&self, limit: i64, offset: i64) -> PostgresResult<Vec<Person>>;
    async fn list_active_persons(&self, limit: i64, offset: i64) -> PostgresResult<Vec<Person>>;

    // --- VariantName CRUD ---
    async fn add_variant(&self, variant: &VariantName) -> PostgresResult<VariantName>;
    async fn get_variant(&self, variant_id: Uuid) -> PostgresResult<Option<VariantName>>;
    async fn get_variants(&self, person_id: Uuid) -> PostgresResult<Vec<VariantName>>;
    async fn update_variant(&self, variant: &VariantName) -> PostgresResult<VariantName>;
    async fn delete_variant(&self, variant_id: Uuid) -> PostgresResult<()>;

    // --- PersonRelationship CRUD ---
    async fn add_relationship(&self, rel: &PersonRelationship) -> PostgresResult<PersonRelationship>;
    async fn get_relationship(&self, relationship_id: Uuid) -> PostgresResult<Option<PersonRelationship>>;
    async fn get_relationships(&self, person_id: Uuid) -> PostgresResult<Vec<PersonRelationship>>;
    async fn update_relationship(&self, rel: &PersonRelationship) -> PostgresResult<PersonRelationship>;
    async fn delete_relationship(&self, relationship_id: Uuid) -> PostgresResult<()>;

    // --- SourcePerson CRUD ---
    async fn add_source_link(&self, sp: &SourcePerson) -> PostgresResult<SourcePerson>;
    async fn get_source_link(&self, source_person_id: Uuid) -> PostgresResult<Option<SourcePerson>>;
    async fn get_source_links(&self, person_id: Uuid) -> PostgresResult<Vec<SourcePerson>>;
    async fn get_source_links_by_source(&self, source_id: Uuid) -> PostgresResult<Vec<SourcePerson>>;
    async fn delete_source_link(&self, source_person_id: Uuid) -> PostgresResult<()>;

    // --- PersonMerge Operations ---
    async fn create_merge(&self, merge: &PersonMerge) -> PostgresResult<PersonMerge>;
    async fn get_merge(&self, merge_id: Uuid) -> PostgresResult<Option<PersonMerge>>;
    async fn get_merges_for_person(&self, person_id: Uuid) -> PostgresResult<Vec<PersonMerge>>;
    async fn reverse_merge(&self, merge_id: Uuid, reason: String, user_id: Uuid) -> PostgresResult<()>;

    // --- Search & Query ---
    async fn search_persons(&self, query: &str, limit: i64) -> PostgresResult<Vec<Person>>;
    async fn find_by_name(&self, given_name: Option<&str>, surname: Option<&str>) -> PostgresResult<Vec<Person>>;
    async fn find_potential_duplicates(&self, person: &Person) -> PostgresResult<Vec<Person>>;

    // --- Aggregate Operations ---
    async fn get_person_with_all_data(&self, person_id: Uuid) -> PostgresResult<Option<PersonWithData>>;
    async fn count_persons(&self) -> PostgresResult<i64>;
    async fn count_active_persons(&self) -> PostgresResult<i64>;
}

/// Complete person record with all related data
#[derive(Debug, Clone)]
pub struct PersonWithData {
    pub person: Person,
    pub variants: Vec<VariantName>,
    pub relationships: Vec<PersonRelationship>,
    pub sources: Vec<SourcePerson>,
    pub merges: Vec<PersonMerge>,
}

// =============================================================================
// POSTGRESQL IMPLEMENTATION
// =============================================================================

/// PostgreSQL implementation of PersonRepository
pub struct PostgresPersonRepository {
    pool: PgPool,
}

impl PostgresPersonRepository {
    /// Create a new PostgreSQL person repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
impl PersonRepository for PostgresPersonRepository {
    // --- Person CRUD ---

    #[instrument(skip(self, person))]
    async fn create_person(&self, person: &Person) -> PostgresResult<Person> {
        debug!("Creating person: {}", person.person_id);

        sqlx::query(
            r#"
            INSERT INTO persons (
                person_id, given_name, surname, middle_name, name_prefix, name_suffix,
                birth_date_year, birth_date_month, birth_date_day, birth_date_certainty, birth_date_circa,
                birth_date_end_year, birth_date_original_text,
                birth_place_id, birth_source_id,
                death_date_year, death_date_month, death_date_day, death_date_certainty, death_date_circa,
                death_date_end_year, death_date_original_text,
                death_place_id, death_source_id,
                sex, occupation, religion,
                notes, research_notes, conclusion_notes, conclusion_confidence,
                marc_binary, marc_xml, marc_updated_at,
                created_at, updated_at, created_by, last_modified_by,
                archived, archived_reason, archived_at, archived_by
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20, $21, $22, $23, $24,
                $25, $26, $27,
                $28, $29, $30, $31,
                $32, $33, $34,
                $35, $36, $37, $38,
                $39, $40, $41, $42
            )
            "#,
        )
        .bind(person.person_id)
        .bind(&person.given_name)
        .bind(&person.surname)
        .bind(&person.middle_name)
        .bind(&person.name_prefix)
        .bind(&person.name_suffix)
        // Birth date
        .bind(person.birth_date.as_ref().and_then(|d| d.year))
        .bind(person.birth_date.as_ref().and_then(|d| d.month.map(|m| m as i16)))
        .bind(person.birth_date.as_ref().and_then(|d| d.day.map(|d| d as i16)))
        .bind(person.birth_date.as_ref().map(|d| format!("{:?}", d.certainty)))
        .bind(person.birth_date.as_ref().map(|d| d.circa).unwrap_or(false))
        .bind(person.birth_date.as_ref().and_then(|d| d.end_year))
        .bind(person.birth_date.as_ref().and_then(|d| d.original_text.clone()))
        .bind(person.birth_place_id)
        .bind(person.birth_source_id)
        // Death date
        .bind(person.death_date.as_ref().and_then(|d| d.year))
        .bind(person.death_date.as_ref().and_then(|d| d.month.map(|m| m as i16)))
        .bind(person.death_date.as_ref().and_then(|d| d.day.map(|d| d as i16)))
        .bind(person.death_date.as_ref().map(|d| format!("{:?}", d.certainty)))
        .bind(person.death_date.as_ref().map(|d| d.circa).unwrap_or(false))
        .bind(person.death_date.as_ref().and_then(|d| d.end_year))
        .bind(person.death_date.as_ref().and_then(|d| d.original_text.clone()))
        .bind(person.death_place_id)
        .bind(person.death_source_id)
        // Biographical
        .bind(format!("{:?}", person.sex))
        .bind(&person.occupation)
        .bind(&person.religion)
        // Notes
        .bind(&person.notes)
        .bind(&person.research_notes)
        .bind(&person.conclusion_notes)
        .bind(person.conclusion_confidence.map(|c| c as i16))
        // MARC cache
        .bind(&person.marc_binary)
        .bind(&person.marc_xml)
        .bind(person.marc_updated_at)
        // Metadata
        .bind(person.created_at)
        .bind(person.updated_at)
        .bind(person.created_by)
        .bind(person.last_modified_by)
        // Archival
        .bind(person.archived)
        .bind(&person.archived_reason)
        .bind(person.archived_at)
        .bind(person.archived_by)
        .execute(&self.pool)
        .await?;

        Ok(person.clone())
    }

    #[instrument(skip(self))]
    async fn get_person(&self, person_id: Uuid) -> PostgresResult<Option<Person>> {
        debug!("Getting person: {}", person_id);

        let row = sqlx::query(
            r#"
            SELECT
                person_id, given_name, surname, middle_name, name_prefix, name_suffix,
                birth_date_year, birth_date_month, birth_date_day, birth_date_certainty, birth_date_circa,
                birth_date_end_year, birth_date_original_text,
                birth_place_id, birth_source_id,
                death_date_year, death_date_month, death_date_day, death_date_certainty, death_date_circa,
                death_date_end_year, death_date_original_text,
                death_place_id, death_source_id,
                sex, occupation, religion,
                notes, research_notes, conclusion_notes, conclusion_confidence,
                marc_binary, marc_xml, marc_updated_at,
                created_at, updated_at, created_by, last_modified_by,
                archived, archived_reason, archived_at, archived_by
            FROM persons
            WHERE person_id = $1
            "#,
        )
        .bind(person_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(map_row_to_person(row)?)),
            None => Ok(None),
        }
    }

    #[instrument(skip(self, person))]
    async fn update_person(&self, person: &Person) -> PostgresResult<Person> {
        debug!("Updating person: {}", person.person_id);

        sqlx::query(
            r#"
            UPDATE persons SET
                given_name = $2, surname = $3, middle_name = $4, name_prefix = $5, name_suffix = $6,
                birth_date_year = $7, birth_date_month = $8, birth_date_day = $9,
                birth_date_certainty = $10, birth_date_circa = $11,
                birth_date_end_year = $12, birth_date_original_text = $13,
                birth_place_id = $14, birth_source_id = $15,
                death_date_year = $16, death_date_month = $17, death_date_day = $18,
                death_date_certainty = $19, death_date_circa = $20,
                death_date_end_year = $21, death_date_original_text = $22,
                death_place_id = $23, death_source_id = $24,
                sex = $25, occupation = $26, religion = $27,
                notes = $28, research_notes = $29, conclusion_notes = $30, conclusion_confidence = $31,
                marc_binary = $32, marc_xml = $33, marc_updated_at = $34,
                updated_at = $35, last_modified_by = $36,
                archived = $37, archived_reason = $38, archived_at = $39, archived_by = $40
            WHERE person_id = $1
            "#,
        )
        .bind(person.person_id)
        .bind(&person.given_name)
        .bind(&person.surname)
        .bind(&person.middle_name)
        .bind(&person.name_prefix)
        .bind(&person.name_suffix)
        // Birth date
        .bind(person.birth_date.as_ref().and_then(|d| d.year))
        .bind(person.birth_date.as_ref().and_then(|d| d.month.map(|m| m as i16)))
        .bind(person.birth_date.as_ref().and_then(|d| d.day.map(|d| d as i16)))
        .bind(person.birth_date.as_ref().map(|d| format!("{:?}", d.certainty)))
        .bind(person.birth_date.as_ref().map(|d| d.circa).unwrap_or(false))
        .bind(person.birth_date.as_ref().and_then(|d| d.end_year))
        .bind(person.birth_date.as_ref().and_then(|d| d.original_text.clone()))
        .bind(person.birth_place_id)
        .bind(person.birth_source_id)
        // Death date
        .bind(person.death_date.as_ref().and_then(|d| d.year))
        .bind(person.death_date.as_ref().and_then(|d| d.month.map(|m| m as i16)))
        .bind(person.death_date.as_ref().and_then(|d| d.day.map(|d| d as i16)))
        .bind(person.death_date.as_ref().map(|d| format!("{:?}", d.certainty)))
        .bind(person.death_date.as_ref().map(|d| d.circa).unwrap_or(false))
        .bind(person.death_date.as_ref().and_then(|d| d.end_year))
        .bind(person.death_date.as_ref().and_then(|d| d.original_text.clone()))
        .bind(person.death_place_id)
        .bind(person.death_source_id)
        // Biographical
        .bind(format!("{:?}", person.sex))
        .bind(&person.occupation)
        .bind(&person.religion)
        // Notes
        .bind(&person.notes)
        .bind(&person.research_notes)
        .bind(&person.conclusion_notes)
        .bind(person.conclusion_confidence.map(|c| c as i16))
        // MARC cache
        .bind(&person.marc_binary)
        .bind(&person.marc_xml)
        .bind(person.marc_updated_at)
        // Metadata
        .bind(person.updated_at)
        .bind(person.last_modified_by)
        // Archival
        .bind(person.archived)
        .bind(&person.archived_reason)
        .bind(person.archived_at)
        .bind(person.archived_by)
        .execute(&self.pool)
        .await?;

        Ok(person.clone())
    }

    #[instrument(skip(self))]
    async fn archive_person(&self, person_id: Uuid, reason: String, user_id: Uuid) -> PostgresResult<()> {
        debug!("Archiving person: {}", person_id);

        sqlx::query(
            r#"
            UPDATE persons SET
                archived = true,
                archived_reason = $2,
                archived_at = $3,
                archived_by = $4,
                updated_at = $3,
                last_modified_by = $4
            WHERE person_id = $1
            "#,
        )
        .bind(person_id)
        .bind(reason)
        .bind(Utc::now())
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn unarchive_person(&self, person_id: Uuid, user_id: Uuid) -> PostgresResult<()> {
        debug!("Unarchiving person: {}", person_id);

        sqlx::query(
            r#"
            UPDATE persons SET
                archived = false,
                archived_reason = NULL,
                archived_at = NULL,
                archived_by = NULL,
                updated_at = $2,
                last_modified_by = $3
            WHERE person_id = $1
            "#,
        )
        .bind(person_id)
        .bind(Utc::now())
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn list_persons(&self, limit: i64, offset: i64) -> PostgresResult<Vec<Person>> {
        debug!("Listing persons: limit={}, offset={}", limit, offset);

        let rows = sqlx::query(
            r#"
            SELECT
                person_id, given_name, surname, middle_name, name_prefix, name_suffix,
                birth_date_year, birth_date_month, birth_date_day, birth_date_certainty, birth_date_circa,
                birth_date_end_year, birth_date_original_text,
                birth_place_id, birth_source_id,
                death_date_year, death_date_month, death_date_day, death_date_certainty, death_date_circa,
                death_date_end_year, death_date_original_text,
                death_place_id, death_source_id,
                sex, occupation, religion,
                notes, research_notes, conclusion_notes, conclusion_confidence,
                marc_binary, marc_xml, marc_updated_at,
                created_at, updated_at, created_by, last_modified_by,
                archived, archived_reason, archived_at, archived_by
            FROM persons
            ORDER BY updated_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_person)
            .collect::<Result<Vec<_>, _>>()
    }

    #[instrument(skip(self))]
    async fn list_active_persons(&self, limit: i64, offset: i64) -> PostgresResult<Vec<Person>> {
        debug!("Listing active persons: limit={}, offset={}", limit, offset);

        let rows = sqlx::query(
            r#"
            SELECT
                person_id, given_name, surname, middle_name, name_prefix, name_suffix,
                birth_date_year, birth_date_month, birth_date_day, birth_date_certainty, birth_date_circa,
                birth_date_end_year, birth_date_original_text,
                birth_place_id, birth_source_id,
                death_date_year, death_date_month, death_date_day, death_date_certainty, death_date_circa,
                death_date_end_year, death_date_original_text,
                death_place_id, death_source_id,
                sex, occupation, religion,
                notes, research_notes, conclusion_notes, conclusion_confidence,
                marc_binary, marc_xml, marc_updated_at,
                created_at, updated_at, created_by, last_modified_by,
                archived, archived_reason, archived_at, archived_by
            FROM persons
            WHERE archived = false
            ORDER BY updated_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_person)
            .collect::<Result<Vec<_>, _>>()
    }

    // --- VariantName CRUD ---

    #[instrument(skip(self, variant))]
    async fn add_variant(&self, variant: &VariantName) -> PostgresResult<VariantName> {
        debug!("Adding variant: {}", variant.variant_id);

        sqlx::query(
            r#"
            INSERT INTO person_variant_names (
                variant_id, person_id,
                given_name, surname, middle_name, name_prefix, name_suffix, full_name,
                variant_type,
                use_from_year, use_to_year,
                notes, source_id,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
            )
            "#,
        )
        .bind(variant.variant_id)
        .bind(variant.person_id)
        .bind(&variant.given_name)
        .bind(&variant.surname)
        .bind(&variant.middle_name)
        .bind(&variant.name_prefix)
        .bind(&variant.name_suffix)
        .bind(&variant.full_name)
        .bind(format!("{:?}", variant.variant_type))
        .bind(variant.use_from_year)
        .bind(variant.use_to_year)
        .bind(&variant.notes)
        .bind(variant.source_id)
        .bind(variant.created_at)
        .bind(variant.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(variant.clone())
    }

    #[instrument(skip(self))]
    async fn get_variant(&self, variant_id: Uuid) -> PostgresResult<Option<VariantName>> {
        debug!("Getting variant: {}", variant_id);

        let row = sqlx::query(
            r#"
            SELECT
                variant_id, person_id,
                given_name, surname, middle_name, name_prefix, name_suffix, full_name,
                variant_type,
                use_from_year, use_to_year,
                notes, source_id,
                created_at, updated_at
            FROM person_variant_names
            WHERE variant_id = $1
            "#,
        )
        .bind(variant_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(map_row_to_variant(row)?)),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn get_variants(&self, person_id: Uuid) -> PostgresResult<Vec<VariantName>> {
        debug!("Getting variants for person: {}", person_id);

        let rows = sqlx::query(
            r#"
            SELECT
                variant_id, person_id,
                given_name, surname, middle_name, name_prefix, name_suffix, full_name,
                variant_type,
                use_from_year, use_to_year,
                notes, source_id,
                created_at, updated_at
            FROM person_variant_names
            WHERE person_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(person_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_variant)
            .collect::<Result<Vec<_>, _>>()
    }

    #[instrument(skip(self, variant))]
    async fn update_variant(&self, variant: &VariantName) -> PostgresResult<VariantName> {
        debug!("Updating variant: {}", variant.variant_id);

        sqlx::query(
            r#"
            UPDATE person_variant_names SET
                given_name = $2, surname = $3, middle_name = $4,
                name_prefix = $5, name_suffix = $6, full_name = $7,
                variant_type = $8,
                use_from_year = $9, use_to_year = $10,
                notes = $11, source_id = $12,
                updated_at = $13
            WHERE variant_id = $1
            "#,
        )
        .bind(variant.variant_id)
        .bind(&variant.given_name)
        .bind(&variant.surname)
        .bind(&variant.middle_name)
        .bind(&variant.name_prefix)
        .bind(&variant.name_suffix)
        .bind(&variant.full_name)
        .bind(format!("{:?}", variant.variant_type))
        .bind(variant.use_from_year)
        .bind(variant.use_to_year)
        .bind(&variant.notes)
        .bind(variant.source_id)
        .bind(variant.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(variant.clone())
    }

    #[instrument(skip(self))]
    async fn delete_variant(&self, variant_id: Uuid) -> PostgresResult<()> {
        debug!("Deleting variant: {}", variant_id);

        sqlx::query("DELETE FROM person_variant_names WHERE variant_id = $1")
            .bind(variant_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // --- PersonRelationship CRUD ---

    #[instrument(skip(self, rel))]
    async fn add_relationship(&self, rel: &PersonRelationship) -> PostgresResult<PersonRelationship> {
        debug!("Adding relationship: {}", rel.relationship_id);

        sqlx::query(
            r#"
            INSERT INTO person_relationships (
                relationship_id, person_id, related_person_id, relationship_type,
                start_date_year, start_date_month, start_date_day, start_date_certainty, start_date_circa,
                start_date_end_year, start_date_original_text,
                end_date_year, end_date_month, end_date_day, end_date_certainty, end_date_circa,
                end_date_end_year, end_date_original_text,
                confidence, notes, source_id,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4,
                $5, $6, $7, $8, $9, $10, $11,
                $12, $13, $14, $15, $16, $17, $18,
                $19, $20, $21, $22, $23
            )
            "#,
        )
        .bind(rel.relationship_id)
        .bind(rel.person_id)
        .bind(rel.related_person_id)
        .bind(format!("{:?}", rel.relationship_type))
        // Start date
        .bind(rel.start_date.as_ref().and_then(|d| d.year))
        .bind(rel.start_date.as_ref().and_then(|d| d.month.map(|m| m as i16)))
        .bind(rel.start_date.as_ref().and_then(|d| d.day.map(|d| d as i16)))
        .bind(rel.start_date.as_ref().map(|d| format!("{:?}", d.certainty)))
        .bind(rel.start_date.as_ref().map(|d| d.circa).unwrap_or(false))
        .bind(rel.start_date.as_ref().and_then(|d| d.end_year))
        .bind(rel.start_date.as_ref().and_then(|d| d.original_text.clone()))
        // End date
        .bind(rel.end_date.as_ref().and_then(|d| d.year))
        .bind(rel.end_date.as_ref().and_then(|d| d.month.map(|m| m as i16)))
        .bind(rel.end_date.as_ref().and_then(|d| d.day.map(|d| d as i16)))
        .bind(rel.end_date.as_ref().map(|d| format!("{:?}", d.certainty)))
        .bind(rel.end_date.as_ref().map(|d| d.circa).unwrap_or(false))
        .bind(rel.end_date.as_ref().and_then(|d| d.end_year))
        .bind(rel.end_date.as_ref().and_then(|d| d.original_text.clone()))
        // Other fields
        .bind(rel.confidence as i16)
        .bind(&rel.notes)
        .bind(rel.source_id)
        .bind(rel.created_at)
        .bind(rel.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(rel.clone())
    }

    #[instrument(skip(self))]
    async fn get_relationship(&self, relationship_id: Uuid) -> PostgresResult<Option<PersonRelationship>> {
        debug!("Getting relationship: {}", relationship_id);

        let row = sqlx::query(
            r#"
            SELECT
                relationship_id, person_id, related_person_id, relationship_type,
                start_date_year, start_date_month, start_date_day, start_date_certainty, start_date_circa,
                start_date_end_year, start_date_original_text,
                end_date_year, end_date_month, end_date_day, end_date_certainty, end_date_circa,
                end_date_end_year, end_date_original_text,
                confidence, notes, source_id,
                created_at, updated_at
            FROM person_relationships
            WHERE relationship_id = $1
            "#,
        )
        .bind(relationship_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(map_row_to_relationship(row)?)),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn get_relationships(&self, person_id: Uuid) -> PostgresResult<Vec<PersonRelationship>> {
        debug!("Getting relationships for person: {}", person_id);

        let rows = sqlx::query(
            r#"
            SELECT
                relationship_id, person_id, related_person_id, relationship_type,
                start_date_year, start_date_month, start_date_day, start_date_certainty, start_date_circa,
                start_date_end_year, start_date_original_text,
                end_date_year, end_date_month, end_date_day, end_date_certainty, end_date_circa,
                end_date_end_year, end_date_original_text,
                confidence, notes, source_id,
                created_at, updated_at
            FROM person_relationships
            WHERE person_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(person_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_relationship)
            .collect::<Result<Vec<_>, _>>()
    }

    #[instrument(skip(self, rel))]
    async fn update_relationship(&self, rel: &PersonRelationship) -> PostgresResult<PersonRelationship> {
        debug!("Updating relationship: {}", rel.relationship_id);

        sqlx::query(
            r#"
            UPDATE person_relationships SET
                relationship_type = $2,
                start_date_year = $3, start_date_month = $4, start_date_day = $5,
                start_date_certainty = $6, start_date_circa = $7,
                start_date_end_year = $8, start_date_original_text = $9,
                end_date_year = $10, end_date_month = $11, end_date_day = $12,
                end_date_certainty = $13, end_date_circa = $14,
                end_date_end_year = $15, end_date_original_text = $16,
                confidence = $17, notes = $18, source_id = $19,
                updated_at = $20
            WHERE relationship_id = $1
            "#,
        )
        .bind(rel.relationship_id)
        .bind(format!("{:?}", rel.relationship_type))
        // Start date
        .bind(rel.start_date.as_ref().and_then(|d| d.year))
        .bind(rel.start_date.as_ref().and_then(|d| d.month.map(|m| m as i16)))
        .bind(rel.start_date.as_ref().and_then(|d| d.day.map(|d| d as i16)))
        .bind(rel.start_date.as_ref().map(|d| format!("{:?}", d.certainty)))
        .bind(rel.start_date.as_ref().map(|d| d.circa).unwrap_or(false))
        .bind(rel.start_date.as_ref().and_then(|d| d.end_year))
        .bind(rel.start_date.as_ref().and_then(|d| d.original_text.clone()))
        // End date
        .bind(rel.end_date.as_ref().and_then(|d| d.year))
        .bind(rel.end_date.as_ref().and_then(|d| d.month.map(|m| m as i16)))
        .bind(rel.end_date.as_ref().and_then(|d| d.day.map(|d| d as i16)))
        .bind(rel.end_date.as_ref().map(|d| format!("{:?}", d.certainty)))
        .bind(rel.end_date.as_ref().map(|d| d.circa).unwrap_or(false))
        .bind(rel.end_date.as_ref().and_then(|d| d.end_year))
        .bind(rel.end_date.as_ref().and_then(|d| d.original_text.clone()))
        // Other fields
        .bind(rel.confidence as i16)
        .bind(&rel.notes)
        .bind(rel.source_id)
        .bind(rel.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(rel.clone())
    }

    #[instrument(skip(self))]
    async fn delete_relationship(&self, relationship_id: Uuid) -> PostgresResult<()> {
        debug!("Deleting relationship: {}", relationship_id);

        sqlx::query("DELETE FROM person_relationships WHERE relationship_id = $1")
            .bind(relationship_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // --- SourcePerson CRUD ---

    #[instrument(skip(self, sp))]
    async fn add_source_link(&self, sp: &SourcePerson) -> PostgresResult<SourcePerson> {
        debug!("Adding source link: {}", sp.source_person_id);

        sqlx::query(
            r#"
            INSERT INTO source_persons (
                source_person_id, source_id, person_id,
                extracted_name_full, extracted_name_given, extracted_name_surname,
                extracted_role, page_reference, notes, confidence,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
            "#,
        )
        .bind(sp.source_person_id)
        .bind(sp.source_id)
        .bind(sp.person_id)
        .bind(&sp.extracted_name_full)
        .bind(&sp.extracted_name_given)
        .bind(&sp.extracted_name_surname)
        .bind(&sp.extracted_role)
        .bind(&sp.page_reference)
        .bind(&sp.notes)
        .bind(sp.confidence as i16)
        .bind(sp.created_at)
        .bind(sp.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(sp.clone())
    }

    #[instrument(skip(self))]
    async fn get_source_link(&self, source_person_id: Uuid) -> PostgresResult<Option<SourcePerson>> {
        debug!("Getting source link: {}", source_person_id);

        let row = sqlx::query(
            r#"
            SELECT
                source_person_id, source_id, person_id,
                extracted_name_full, extracted_name_given, extracted_name_surname,
                extracted_role, page_reference, notes, confidence,
                created_at, updated_at
            FROM source_persons
            WHERE source_person_id = $1
            "#,
        )
        .bind(source_person_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(map_row_to_source_person(row)?)),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn get_source_links(&self, person_id: Uuid) -> PostgresResult<Vec<SourcePerson>> {
        debug!("Getting source links for person: {}", person_id);

        let rows = sqlx::query(
            r#"
            SELECT
                source_person_id, source_id, person_id,
                extracted_name_full, extracted_name_given, extracted_name_surname,
                extracted_role, page_reference, notes, confidence,
                created_at, updated_at
            FROM source_persons
            WHERE person_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(person_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_source_person)
            .collect::<Result<Vec<_>, _>>()
    }

    #[instrument(skip(self))]
    async fn get_source_links_by_source(&self, source_id: Uuid) -> PostgresResult<Vec<SourcePerson>> {
        debug!("Getting source links for source: {}", source_id);

        let rows = sqlx::query(
            r#"
            SELECT
                source_person_id, source_id, person_id,
                extracted_name_full, extracted_name_given, extracted_name_surname,
                extracted_role, page_reference, notes, confidence,
                created_at, updated_at
            FROM source_persons
            WHERE source_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(source_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_source_person)
            .collect::<Result<Vec<_>, _>>()
    }

    #[instrument(skip(self))]
    async fn delete_source_link(&self, source_person_id: Uuid) -> PostgresResult<()> {
        debug!("Deleting source link: {}", source_person_id);

        sqlx::query("DELETE FROM source_persons WHERE source_person_id = $1")
            .bind(source_person_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // --- PersonMerge Operations ---

    #[instrument(skip(self, merge))]
    async fn create_merge(&self, merge: &PersonMerge) -> PostgresResult<PersonMerge> {
        debug!("Creating merge: {}", merge.merge_id);

        sqlx::query(
            r#"
            INSERT INTO person_merges (
                merge_id, source_person_id, target_person_id, merge_reason,
                merged_at, merged_by,
                reversed_at, reversed_by, reversed_reason
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            "#,
        )
        .bind(merge.merge_id)
        .bind(merge.source_person_id)
        .bind(merge.target_person_id)
        .bind(&merge.merge_reason)
        .bind(merge.merged_at)
        .bind(merge.merged_by)
        .bind(merge.reversed_at)
        .bind(merge.reversed_by)
        .bind(&merge.reversed_reason)
        .execute(&self.pool)
        .await?;

        Ok(merge.clone())
    }

    #[instrument(skip(self))]
    async fn get_merge(&self, merge_id: Uuid) -> PostgresResult<Option<PersonMerge>> {
        debug!("Getting merge: {}", merge_id);

        let row = sqlx::query(
            r#"
            SELECT
                merge_id, source_person_id, target_person_id, merge_reason,
                merged_at, merged_by,
                reversed_at, reversed_by, reversed_reason
            FROM person_merges
            WHERE merge_id = $1
            "#,
        )
        .bind(merge_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(map_row_to_merge(row)?)),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn get_merges_for_person(&self, person_id: Uuid) -> PostgresResult<Vec<PersonMerge>> {
        debug!("Getting merges for person: {}", person_id);

        let rows = sqlx::query(
            r#"
            SELECT
                merge_id, source_person_id, target_person_id, merge_reason,
                merged_at, merged_by,
                reversed_at, reversed_by, reversed_reason
            FROM person_merges
            WHERE source_person_id = $1 OR target_person_id = $1
            ORDER BY merged_at DESC
            "#,
        )
        .bind(person_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_merge)
            .collect::<Result<Vec<_>, _>>()
    }

    #[instrument(skip(self))]
    async fn reverse_merge(&self, merge_id: Uuid, reason: String, user_id: Uuid) -> PostgresResult<()> {
        debug!("Reversing merge: {}", merge_id);

        sqlx::query(
            r#"
            UPDATE person_merges SET
                reversed_at = $2,
                reversed_by = $3,
                reversed_reason = $4
            WHERE merge_id = $1 AND reversed_at IS NULL
            "#,
        )
        .bind(merge_id)
        .bind(Utc::now())
        .bind(user_id)
        .bind(reason)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // --- Search & Query ---

    #[instrument(skip(self))]
    async fn search_persons(&self, query: &str, limit: i64) -> PostgresResult<Vec<Person>> {
        debug!("Searching persons: query={}, limit={}", query, limit);

        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query(
            r#"
            SELECT DISTINCT
                p.person_id, p.given_name, p.surname, p.middle_name, p.name_prefix, p.name_suffix,
                p.birth_date_year, p.birth_date_month, p.birth_date_day, p.birth_date_certainty, p.birth_date_circa,
                p.birth_date_end_year, p.birth_date_original_text,
                p.birth_place_id, p.birth_source_id,
                p.death_date_year, p.death_date_month, p.death_date_day, p.death_date_certainty, p.death_date_circa,
                p.death_date_end_year, p.death_date_original_text,
                p.death_place_id, p.death_source_id,
                p.sex, p.occupation, p.religion,
                p.notes, p.research_notes, p.conclusion_notes, p.conclusion_confidence,
                p.marc_binary, p.marc_xml, p.marc_updated_at,
                p.created_at, p.updated_at, p.created_by, p.last_modified_by,
                p.archived, p.archived_reason, p.archived_at, p.archived_by
            FROM persons p
            LEFT JOIN person_variant_names v ON p.person_id = v.person_id
            WHERE p.archived = false
            AND (
                LOWER(p.given_name) LIKE LOWER($1) OR
                LOWER(p.surname) LIKE LOWER($1) OR
                LOWER(p.middle_name) LIKE LOWER($1) OR
                LOWER(v.full_name) LIKE LOWER($1)
            )
            ORDER BY p.updated_at DESC
            LIMIT $2
            "#,
        )
        .bind(&search_pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_person)
            .collect::<Result<Vec<_>, _>>()
    }

    #[instrument(skip(self))]
    async fn find_by_name(&self, given_name: Option<&str>, surname: Option<&str>) -> PostgresResult<Vec<Person>> {
        debug!("Finding persons by name: given={:?}, surname={:?}", given_name, surname);

        let rows = match (given_name, surname) {
            (Some(g), Some(s)) => {
                sqlx::query(
                    r#"
                    SELECT
                        person_id, given_name, surname, middle_name, name_prefix, name_suffix,
                        birth_date_year, birth_date_month, birth_date_day, birth_date_certainty, birth_date_circa,
                        birth_date_end_year, birth_date_original_text,
                        birth_place_id, birth_source_id,
                        death_date_year, death_date_month, death_date_day, death_date_certainty, death_date_circa,
                        death_date_end_year, death_date_original_text,
                        death_place_id, death_source_id,
                        sex, occupation, religion,
                        notes, research_notes, conclusion_notes, conclusion_confidence,
                        marc_binary, marc_xml, marc_updated_at,
                        created_at, updated_at, created_by, last_modified_by,
                        archived, archived_reason, archived_at, archived_by
                    FROM persons
                    WHERE archived = false
                    AND LOWER(given_name) = LOWER($1)
                    AND LOWER(surname) = LOWER($2)
                    ORDER BY updated_at DESC
                    "#,
                )
                .bind(g)
                .bind(s)
                .fetch_all(&self.pool)
                .await?
            }
            (Some(g), None) => {
                sqlx::query(
                    r#"
                    SELECT
                        person_id, given_name, surname, middle_name, name_prefix, name_suffix,
                        birth_date_year, birth_date_month, birth_date_day, birth_date_certainty, birth_date_circa,
                        birth_date_end_year, birth_date_original_text,
                        birth_place_id, birth_source_id,
                        death_date_year, death_date_month, death_date_day, death_date_certainty, death_date_circa,
                        death_date_end_year, death_date_original_text,
                        death_place_id, death_source_id,
                        sex, occupation, religion,
                        notes, research_notes, conclusion_notes, conclusion_confidence,
                        marc_binary, marc_xml, marc_updated_at,
                        created_at, updated_at, created_by, last_modified_by,
                        archived, archived_reason, archived_at, archived_by
                    FROM persons
                    WHERE archived = false
                    AND LOWER(given_name) = LOWER($1)
                    ORDER BY updated_at DESC
                    "#,
                )
                .bind(g)
                .fetch_all(&self.pool)
                .await?
            }
            (None, Some(s)) => {
                sqlx::query(
                    r#"
                    SELECT
                        person_id, given_name, surname, middle_name, name_prefix, name_suffix,
                        birth_date_year, birth_date_month, birth_date_day, birth_date_certainty, birth_date_circa,
                        birth_date_end_year, birth_date_original_text,
                        birth_place_id, birth_source_id,
                        death_date_year, death_date_month, death_date_day, death_date_certainty, death_date_circa,
                        death_date_end_year, death_date_original_text,
                        death_place_id, death_source_id,
                        sex, occupation, religion,
                        notes, research_notes, conclusion_notes, conclusion_confidence,
                        marc_binary, marc_xml, marc_updated_at,
                        created_at, updated_at, created_by, last_modified_by,
                        archived, archived_reason, archived_at, archived_by
                    FROM persons
                    WHERE archived = false
                    AND LOWER(surname) = LOWER($1)
                    ORDER BY updated_at DESC
                    "#,
                )
                .bind(s)
                .fetch_all(&self.pool)
                .await?
            }
            (None, None) => return Ok(Vec::new()),
        };

        rows.into_iter()
            .map(map_row_to_person)
            .collect::<Result<Vec<_>, _>>()
    }

    #[instrument(skip(self, person))]
    async fn find_potential_duplicates(&self, person: &Person) -> PostgresResult<Vec<Person>> {
        debug!("Finding potential duplicates for: {}", person.person_id);

        // Find persons with similar names and birth years
        let rows = sqlx::query(
            r#"
            SELECT
                person_id, given_name, surname, middle_name, name_prefix, name_suffix,
                birth_date_year, birth_date_month, birth_date_day, birth_date_certainty, birth_date_circa,
                birth_date_end_year, birth_date_original_text,
                birth_place_id, birth_source_id,
                death_date_year, death_date_month, death_date_day, death_date_certainty, death_date_circa,
                death_date_end_year, death_date_original_text,
                death_place_id, death_source_id,
                sex, occupation, religion,
                notes, research_notes, conclusion_notes, conclusion_confidence,
                marc_binary, marc_xml, marc_updated_at,
                created_at, updated_at, created_by, last_modified_by,
                archived, archived_reason, archived_at, archived_by
            FROM persons
            WHERE person_id != $1
            AND archived = false
            AND (
                (LOWER(given_name) = LOWER($2) AND LOWER(surname) = LOWER($3))
                OR (birth_date_year = $4)
            )
            LIMIT 10
            "#,
        )
        .bind(person.person_id)
        .bind(&person.given_name)
        .bind(&person.surname)
        .bind(person.birth_date.as_ref().and_then(|d| d.year))
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(map_row_to_person)
            .collect::<Result<Vec<_>, _>>()
    }

    // --- Aggregate Operations ---

    #[instrument(skip(self))]
    async fn get_person_with_all_data(&self, person_id: Uuid) -> PostgresResult<Option<PersonWithData>> {
        debug!("Getting person with all data: {}", person_id);

        let person = match self.get_person(person_id).await? {
            Some(p) => p,
            None => return Ok(None),
        };

        let variants = self.get_variants(person_id).await?;
        let relationships = self.get_relationships(person_id).await?;
        let sources = self.get_source_links(person_id).await?;
        let merges = self.get_merges_for_person(person_id).await?;

        Ok(Some(PersonWithData {
            person,
            variants,
            relationships,
            sources,
            merges,
        }))
    }

    #[instrument(skip(self))]
    async fn count_persons(&self) -> PostgresResult<i64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM persons")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    #[instrument(skip(self))]
    async fn count_active_persons(&self) -> PostgresResult<i64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM persons WHERE archived = false")
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }
}

// =============================================================================
// ROW MAPPING HELPERS
// =============================================================================

/// Map database row to Person
fn map_row_to_person(row: sqlx::postgres::PgRow) -> PostgresResult<Person> {
    // Helper to build GenealogyDate
    let build_date = |prefix: &str| -> Option<GenealogyDate> {
        let year_col = format!("{}_year", prefix);
        let month_col = format!("{}_month", prefix);
        let day_col = format!("{}_day", prefix);
        let certainty_col = format!("{}_certainty", prefix);
        let circa_col = format!("{}_circa", prefix);
        let end_year_col = format!("{}_end_year", prefix);
        let original_col = format!("{}_original_text", prefix);

        let year: Option<i32> = row.try_get(&*year_col).ok()?;
        if year.is_none() {
            return None;
        }

        let month: Option<i16> = row.try_get(&*month_col).ok().flatten();
        let day: Option<i16> = row.try_get(&*day_col).ok().flatten();
        let certainty_str: Option<String> = row.try_get(&*certainty_col).ok().flatten();
        let circa: bool = row.try_get(&*circa_col).ok().unwrap_or(false);
        let end_year: Option<i32> = row.try_get(&*end_year_col).ok().flatten();
        let original_text: Option<String> = row.try_get(&*original_col).ok().flatten();

        let certainty = match certainty_str.as_deref() {
            Some("Exact") => DateCertainty::Exact,
            Some("Estimated") => DateCertainty::Estimated,
            Some("Calculated") => DateCertainty::Calculated,
            Some("Before") => DateCertainty::Before,
            Some("After") => DateCertainty::After,
            Some("Between") => DateCertainty::Between,
            _ => DateCertainty::Exact,
        };

        Some(GenealogyDate {
            year,
            month: month.map(|m| m as u8),
            day: day.map(|d| d as u8),
            certainty,
            circa,
            end_year,
            original_text,
        })
    };

    let birth_date = build_date("birth_date");
    let death_date = build_date("death_date");

    let sex_str: String = row.try_get("sex")?;
    let sex = match sex_str.as_str() {
        "Male" => Sex::Male,
        "Female" => Sex::Female,
        _ => Sex::Unknown,
    };

    let confidence: Option<i16> = row.try_get("conclusion_confidence")?;
    let conclusion_confidence = confidence.and_then(|c| match c {
        1 => Some(PersonConfidence::Speculative),
        2 => Some(PersonConfidence::Uncertain),
        3 => Some(PersonConfidence::Possible),
        4 => Some(PersonConfidence::Probable),
        5 => Some(PersonConfidence::Definite),
        _ => None,
    });

    Ok(Person {
        person_id: row.try_get("person_id")?,
        given_name: row.try_get("given_name")?,
        surname: row.try_get("surname")?,
        middle_name: row.try_get("middle_name")?,
        name_prefix: row.try_get("name_prefix")?,
        name_suffix: row.try_get("name_suffix")?,
        birth_date,
        birth_place_id: row.try_get("birth_place_id")?,
        birth_source_id: row.try_get("birth_source_id")?,
        death_date,
        death_place_id: row.try_get("death_place_id")?,
        death_source_id: row.try_get("death_source_id")?,
        sex,
        occupation: row.try_get("occupation")?,
        religion: row.try_get("religion")?,
        notes: row.try_get("notes")?,
        research_notes: row.try_get("research_notes")?,
        conclusion_notes: row.try_get("conclusion_notes")?,
        conclusion_confidence,
        marc_binary: row.try_get("marc_binary")?,
        marc_xml: row.try_get("marc_xml")?,
        marc_updated_at: row.try_get("marc_updated_at")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        created_by: row.try_get("created_by")?,
        last_modified_by: row.try_get("last_modified_by")?,
        archived: row.try_get("archived")?,
        archived_reason: row.try_get("archived_reason")?,
        archived_at: row.try_get("archived_at")?,
        archived_by: row.try_get("archived_by")?,
    })
}

/// Map database row to VariantName
fn map_row_to_variant(row: sqlx::postgres::PgRow) -> PostgresResult<VariantName> {
    let variant_type_str: String = row.try_get("variant_type")?;
    let variant_type = match variant_type_str.as_str() {
        "Birth" => VariantNameType::Birth,
        "Married" => VariantNameType::Married,
        "Divorced" => VariantNameType::Divorced,
        "Nickname" => VariantNameType::Nickname,
        "Immigration" => VariantNameType::Immigration,
        "Spelling" => VariantNameType::Spelling,
        "Translation" => VariantNameType::Translation,
        "Abbreviation" => VariantNameType::Abbreviation,
        "Pseudonym" => VariantNameType::Pseudonym,
        "Legal" => VariantNameType::Legal,
        "Religious" => VariantNameType::Religious,
        "Stage" => VariantNameType::Stage,
        _ => VariantNameType::Documented,
    };

    Ok(VariantName {
        variant_id: row.try_get("variant_id")?,
        person_id: row.try_get("person_id")?,
        given_name: row.try_get("given_name")?,
        surname: row.try_get("surname")?,
        middle_name: row.try_get("middle_name")?,
        name_prefix: row.try_get("name_prefix")?,
        name_suffix: row.try_get("name_suffix")?,
        full_name: row.try_get("full_name")?,
        variant_type,
        use_from_year: row.try_get("use_from_year")?,
        use_to_year: row.try_get("use_to_year")?,
        notes: row.try_get("notes")?,
        source_id: row.try_get("source_id")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

/// Map database row to PersonRelationship
fn map_row_to_relationship(row: sqlx::postgres::PgRow) -> PostgresResult<PersonRelationship> {
    let rel_type_str: String = row.try_get("relationship_type")?;
    let relationship_type = match rel_type_str.as_str() {
        "Parent" => PersonRelationshipType::Parent,
        "Child" => PersonRelationshipType::Child,
        "Spouse" => PersonRelationshipType::Spouse,
        "Sibling" => PersonRelationshipType::Sibling,
        "Grandparent" => PersonRelationshipType::Grandparent,
        "Grandchild" => PersonRelationshipType::Grandchild,
        "AuntUncle" => PersonRelationshipType::AuntUncle,
        "NieceNephew" => PersonRelationshipType::NieceNephew,
        "Cousin" => PersonRelationshipType::Cousin,
        "StepParent" => PersonRelationshipType::StepParent,
        "StepChild" => PersonRelationshipType::StepChild,
        "AdoptiveParent" => PersonRelationshipType::AdoptiveParent,
        "AdoptiveChild" => PersonRelationshipType::AdoptiveChild,
        "FosterParent" => PersonRelationshipType::FosterParent,
        "FosterChild" => PersonRelationshipType::FosterChild,
        "Guardian" => PersonRelationshipType::Guardian,
        _ => PersonRelationshipType::Ward,
    };

    let build_date = |prefix: &str| -> Option<GenealogyDate> {
        let year_col = format!("{}_year", prefix);
        let month_col = format!("{}_month", prefix);
        let day_col = format!("{}_day", prefix);
        let certainty_col = format!("{}_certainty", prefix);
        let circa_col = format!("{}_circa", prefix);
        let end_year_col = format!("{}_end_year", prefix);
        let original_col = format!("{}_original_text", prefix);

        let year: Option<i32> = row.try_get(&*year_col).ok()?;
        if year.is_none() {
            return None;
        }

        let month: Option<i16> = row.try_get(&*month_col).ok().flatten();
        let day: Option<i16> = row.try_get(&*day_col).ok().flatten();
        let certainty_str: Option<String> = row.try_get(&*certainty_col).ok().flatten();
        let circa: bool = row.try_get(&*circa_col).ok().unwrap_or(false);
        let end_year: Option<i32> = row.try_get(&*end_year_col).ok().flatten();
        let original_text: Option<String> = row.try_get(&*original_col).ok().flatten();

        let certainty = match certainty_str.as_deref() {
            Some("Exact") => DateCertainty::Exact,
            Some("Estimated") => DateCertainty::Estimated,
            Some("Calculated") => DateCertainty::Calculated,
            Some("Before") => DateCertainty::Before,
            Some("After") => DateCertainty::After,
            Some("Between") => DateCertainty::Between,
            _ => DateCertainty::Exact,
        };

        Some(GenealogyDate {
            year,
            month: month.map(|m| m as u8),
            day: day.map(|d| d as u8),
            certainty,
            circa,
            end_year,
            original_text,
        })
    };

    let start_date = build_date("start_date");
    let end_date = build_date("end_date");

    let confidence: i16 = row.try_get("confidence")?;
    let confidence = match confidence {
        1 => PersonConfidence::Speculative,
        2 => PersonConfidence::Uncertain,
        3 => PersonConfidence::Possible,
        4 => PersonConfidence::Probable,
        _ => PersonConfidence::Definite,
    };

    Ok(PersonRelationship {
        relationship_id: row.try_get("relationship_id")?,
        person_id: row.try_get("person_id")?,
        related_person_id: row.try_get("related_person_id")?,
        relationship_type,
        start_date,
        end_date,
        confidence,
        notes: row.try_get("notes")?,
        source_id: row.try_get("source_id")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

/// Map database row to SourcePerson
fn map_row_to_source_person(row: sqlx::postgres::PgRow) -> PostgresResult<SourcePerson> {
    let confidence: i16 = row.try_get("confidence")?;
    let confidence = match confidence {
        1 => PersonConfidence::Speculative,
        2 => PersonConfidence::Uncertain,
        3 => PersonConfidence::Possible,
        4 => PersonConfidence::Probable,
        _ => PersonConfidence::Definite,
    };

    Ok(SourcePerson {
        source_person_id: row.try_get("source_person_id")?,
        source_id: row.try_get("source_id")?,
        person_id: row.try_get("person_id")?,
        extracted_name_full: row.try_get("extracted_name_full")?,
        extracted_name_given: row.try_get("extracted_name_given")?,
        extracted_name_surname: row.try_get("extracted_name_surname")?,
        extracted_role: row.try_get("extracted_role")?,
        page_reference: row.try_get("page_reference")?,
        notes: row.try_get("notes")?,
        confidence,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

/// Map database row to PersonMerge
fn map_row_to_merge(row: sqlx::postgres::PgRow) -> PostgresResult<PersonMerge> {
    Ok(PersonMerge {
        merge_id: row.try_get("merge_id")?,
        source_person_id: row.try_get("source_person_id")?,
        target_person_id: row.try_get("target_person_id")?,
        merge_reason: row.try_get("merge_reason")?,
        merged_at: row.try_get("merged_at")?,
        merged_by: row.try_get("merged_by")?,
        reversed_at: row.try_get("reversed_at")?,
        reversed_by: row.try_get("reversed_by")?,
        reversed_reason: row.try_get("reversed_reason")?,
    })
}
