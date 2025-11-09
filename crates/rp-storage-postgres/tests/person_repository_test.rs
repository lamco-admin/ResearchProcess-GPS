//! Integration tests for Person repository
//!
//! Tests complete CRUD operations for Person authority control system:
//! - Person (canonical authority records)
//! - VariantName (name variants)
//! - PersonRelationship (family relationships)
//! - SourcePerson (source attribution)
//! - PersonMerge (duplicate handling)

use rp_storage_postgres::{PostgresConfig, PostgresPersonRepository, PersonRepository};
use rp_core::person::{
    Person, Sex, PersonConfidence, GenealogyDate, DateCertainty,
    VariantName, VariantNameType,
    PersonRelationship, PersonRelationshipType,
    SourcePerson,
    PersonMerge,
};
use uuid::Uuid;

/// Get test database configuration
fn test_config() -> PostgresConfig {
    PostgresConfig {
        host: std::env::var("TEST_DB_HOST").unwrap_or_else(|_| "192.168.10.90".to_string()),
        port: std::env::var("TEST_DB_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(5432),
        database: std::env::var("TEST_DB_NAME")
            .unwrap_or_else(|_| "researchprocess_gps".to_string()),
        username: std::env::var("TEST_DB_USER")
            .unwrap_or_else(|_| "researchprocess_gps".to_string()),
        password: std::env::var("TEST_DB_PASSWORD")
            .unwrap_or_else(|_| "researchprocess_gps".to_string()),
        max_connections: 5,
        min_connections: 1,
        connect_timeout: std::time::Duration::from_secs(30),
        connection_timeout: std::time::Duration::from_secs(30),
        idle_timeout: Some(std::time::Duration::from_secs(600)),
        max_lifetime: Some(std::time::Duration::from_secs(1800)),
        statement_cache_capacity: 100,
        ssl_mode: rp_storage_postgres::config::SslMode::Prefer,
        application_name: "rp-person-test".to_string(),
        enable_vector: false,
        enable_graph: false,
        enable_notifications: false,
    }
}

// =============================================================================
// PERSON CRUD TESTS
// =============================================================================

#[tokio::test]
async fn test_person_create_and_get() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create test person
    let person = Person::builder()
        .given_name("John")
        .surname("Smith")
        .middle_name("William")
        .birth_date(GenealogyDate::exact(1820, Some(3), Some(15)))
        .death_date(GenealogyDate::exact(1891, Some(12), Some(10)))
        .sex(Sex::Male)
        .occupation("Farmer")
        .conclusion_confidence(PersonConfidence::Definite)
        .created_by(user_id)
        .build()?;

    // Test create
    let created = repo.create_person(&person).await?;
    assert_eq!(created.person_id, person.person_id);
    println!("✓ Person created: {}", created.canonical_name());

    // Test get
    let retrieved = repo.get_person(person.person_id).await?;
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.given_name, Some("John".to_string()));
    assert_eq!(retrieved.surname, Some("Smith".to_string()));
    assert_eq!(retrieved.sex, Sex::Male);
    println!("✓ Person retrieved: {}", retrieved.canonical_name());

    // Cleanup
    repo.archive_person(person.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_person_update() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create person
    let mut person = Person::builder()
        .given_name("Jane")
        .surname("Doe")
        .sex(Sex::Female)
        .created_by(user_id)
        .build()?;

    repo.create_person(&person).await?;
    println!("✓ Person created");

    // Update person
    person.middle_name = Some("Marie".to_string());
    person.occupation = Some("Teacher".to_string());

    let updated = repo.update_person(&person).await?;
    assert_eq!(updated.middle_name, Some("Marie".to_string()));
    assert_eq!(updated.occupation, Some("Teacher".to_string()));
    println!("✓ Person updated");

    // Verify update
    let retrieved = repo.get_person(person.person_id).await?.unwrap();
    assert_eq!(retrieved.middle_name, Some("Marie".to_string()));
    println!("✓ Update verified");

    // Cleanup
    repo.archive_person(person.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_person_archive_and_unarchive() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create person
    let person = Person::builder()
        .given_name("Test")
        .surname("Archive")
        .created_by(user_id)
        .build()?;

    repo.create_person(&person).await?;
    println!("✓ Person created");

    // Archive
    repo.archive_person(
        person.person_id,
        "Testing archive functionality".to_string(),
        user_id,
    )
    .await?;
    println!("✓ Person archived");

    // Verify archived
    let archived = repo.get_person(person.person_id).await?.unwrap();
    assert!(archived.archived);
    assert_eq!(
        archived.archived_reason,
        Some("Testing archive functionality".to_string())
    );
    println!("✓ Archive verified");

    // Unarchive
    repo.unarchive_person(person.person_id, user_id).await?;
    println!("✓ Person unarchived");

    // Verify unarchived
    let unarchived = repo.get_person(person.person_id).await?.unwrap();
    assert!(!unarchived.archived);
    assert!(unarchived.archived_reason.is_none());
    println!("✓ Unarchive verified");

    // Final cleanup
    repo.archive_person(person.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_person_list() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create test persons
    let person1 = Person::builder()
        .given_name("Alice")
        .surname("Test")
        .created_by(user_id)
        .build()?;

    let person2 = Person::builder()
        .given_name("Bob")
        .surname("Test")
        .created_by(user_id)
        .build()?;

    repo.create_person(&person1).await?;
    repo.create_person(&person2).await?;
    println!("✓ Test persons created");

    // List active persons
    let active = repo.list_active_persons(10, 0).await?;
    assert!(!active.is_empty());
    println!("✓ Listed {} active persons", active.len());

    // Archive one
    repo.archive_person(person1.person_id, "Test".to_string(), user_id)
        .await?;

    // List all (should include archived)
    let all = repo.list_persons(10, 0).await?;
    assert!(!all.is_empty());
    println!("✓ Listed {} total persons", all.len());

    // Cleanup
    repo.archive_person(person2.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

// =============================================================================
// VARIANT NAME TESTS
// =============================================================================

#[tokio::test]
async fn test_variant_name_operations() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create person
    let person = Person::builder()
        .given_name("Mary")
        .surname("Smith")
        .created_by(user_id)
        .build()?;

    repo.create_person(&person).await?;
    println!("✓ Person created");

    // Add birth name variant
    let birth_name = VariantName::builder(person.person_id)
        .given_name("Mary")
        .surname("Johnson")
        .middle_name("Elizabeth")
        .variant_type(VariantNameType::Birth)
        .use_from_year(1820)
        .use_to_year(1845)
        .notes("Maiden name before marriage")
        .build()?;

    let created_variant = repo.add_variant(&birth_name).await?;
    assert_eq!(created_variant.variant_id, birth_name.variant_id);
    println!("✓ Variant name added: {}", created_variant.full_name);

    // Get variant
    let retrieved = repo.get_variant(birth_name.variant_id).await?;
    assert!(retrieved.is_some());
    println!("✓ Variant name retrieved");

    // Get all variants for person
    let variants = repo.get_variants(person.person_id).await?;
    assert_eq!(variants.len(), 1);
    println!("✓ Found {} variants for person", variants.len());

    // Update variant
    let mut updated_variant = birth_name.clone();
    updated_variant.notes = Some("Updated notes".to_string());
    repo.update_variant(&updated_variant).await?;
    println!("✓ Variant name updated");

    // Delete variant
    repo.delete_variant(birth_name.variant_id).await?;
    println!("✓ Variant name deleted");

    // Verify deletion
    let deleted = repo.get_variant(birth_name.variant_id).await?;
    assert!(deleted.is_none());
    println!("✓ Deletion verified");

    // Cleanup
    repo.archive_person(person.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

// =============================================================================
// PERSON RELATIONSHIP TESTS
// =============================================================================

#[tokio::test]
async fn test_relationship_operations() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create father
    let father = Person::builder()
        .given_name("John")
        .surname("Smith")
        .sex(Sex::Male)
        .created_by(user_id)
        .build()?;

    // Create child
    let child = Person::builder()
        .given_name("William")
        .surname("Smith")
        .sex(Sex::Male)
        .created_by(user_id)
        .build()?;

    repo.create_person(&father).await?;
    repo.create_person(&child).await?;
    println!("✓ Father and child persons created");

    // Create parent-child relationship
    let parent_rel = PersonRelationship::builder()
        .person_id(father.person_id)
        .related_person_id(child.person_id)
        .relationship_type(PersonRelationshipType::Parent)
        .confidence(PersonConfidence::Definite)
        .build()?;

    let created_rel = repo.add_relationship(&parent_rel).await?;
    assert_eq!(created_rel.relationship_id, parent_rel.relationship_id);
    println!("✓ Relationship added: {}", created_rel.relationship_type);

    // Get relationship
    let retrieved = repo.get_relationship(parent_rel.relationship_id).await?;
    assert!(retrieved.is_some());
    println!("✓ Relationship retrieved");

    // Get all relationships for person
    let relationships = repo.get_relationships(father.person_id).await?;
    assert_eq!(relationships.len(), 1);
    println!("✓ Found {} relationships", relationships.len());

    // Create reciprocal relationship
    let child_rel = parent_rel.reciprocal();
    repo.add_relationship(&child_rel).await?;
    println!("✓ Reciprocal relationship added");

    // Verify both directions
    let father_rels = repo.get_relationships(father.person_id).await?;
    let child_rels = repo.get_relationships(child.person_id).await?;
    assert_eq!(father_rels.len(), 1);
    assert_eq!(child_rels.len(), 1);
    println!("✓ Bidirectional relationships verified");

    // Delete relationships
    repo.delete_relationship(parent_rel.relationship_id).await?;
    repo.delete_relationship(child_rel.relationship_id).await?;
    println!("✓ Relationships deleted");

    // Cleanup
    repo.archive_person(father.person_id, "Test cleanup".to_string(), user_id)
        .await?;
    repo.archive_person(child.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

// =============================================================================
// SOURCE-PERSON LINK TESTS
// =============================================================================

#[tokio::test]
async fn test_source_person_operations() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create person
    let person = Person::builder()
        .given_name("John")
        .surname("Smith")
        .created_by(user_id)
        .build()?;

    repo.create_person(&person).await?;
    println!("✓ Person created");

    // Create source link
    let source_id = Uuid::now_v7(); // Mock source ID
    let source_person = SourcePerson::builder()
        .source_id(source_id)
        .person_id(person.person_id)
        .extracted_name_full("Jno Smith")
        .extracted_role("farmer")
        .page_reference("15")
        .confidence(PersonConfidence::Probable)
        .notes("Listed as head of household")
        .build()?;

    let created_sp = repo.add_source_link(&source_person).await?;
    assert_eq!(created_sp.source_person_id, source_person.source_person_id);
    println!("✓ Source link added: {}", created_sp.get_extracted_name());

    // Get source link
    let retrieved = repo.get_source_link(source_person.source_person_id).await?;
    assert!(retrieved.is_some());
    println!("✓ Source link retrieved");

    // Get all source links for person
    let links = repo.get_source_links(person.person_id).await?;
    assert_eq!(links.len(), 1);
    println!("✓ Found {} source links for person", links.len());

    // Get source links by source
    let by_source = repo.get_source_links_by_source(source_id).await?;
    assert_eq!(by_source.len(), 1);
    println!("✓ Found {} persons in source", by_source.len());

    // Delete source link
    repo.delete_source_link(source_person.source_person_id)
        .await?;
    println!("✓ Source link deleted");

    // Cleanup
    repo.archive_person(person.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

// =============================================================================
// PERSON MERGE TESTS
// =============================================================================

#[tokio::test]
async fn test_merge_operations() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create duplicate persons
    let source_person = Person::builder()
        .given_name("John")
        .surname("Smith")
        .created_by(user_id)
        .build()?;

    let target_person = Person::builder()
        .given_name("John")
        .surname("Smith")
        .middle_name("William") // More complete
        .created_by(user_id)
        .build()?;

    repo.create_person(&source_person).await?;
    repo.create_person(&target_person).await?;
    println!("✓ Source and target persons created");

    // Create merge
    let merge = PersonMerge::builder()
        .source_person_id(source_person.person_id)
        .target_person_id(target_person.person_id)
        .merge_reason("Duplicate detected: same name and birth year")
        .merged_by(user_id)
        .build()?;

    let created_merge = repo.create_merge(&merge).await?;
    assert_eq!(created_merge.merge_id, merge.merge_id);
    println!("✓ Merge created");

    // Archive source person (as part of merge)
    repo.archive_person(
        source_person.person_id,
        format!("Merged into {}", target_person.canonical_name()),
        user_id,
    )
    .await?;
    println!("✓ Source person archived");

    // Get merge
    let retrieved = repo.get_merge(merge.merge_id).await?;
    assert!(retrieved.is_some());
    assert!(retrieved.unwrap().is_active());
    println!("✓ Merge retrieved and is active");

    // Get merge history for person
    let merges = repo.get_merges_for_person(source_person.person_id).await?;
    assert_eq!(merges.len(), 1);
    println!("✓ Found {} merges in history", merges.len());

    // Reverse merge
    repo.reverse_merge(
        merge.merge_id,
        "Actually different people - new evidence found".to_string(),
        user_id,
    )
    .await?;
    println!("✓ Merge reversed");

    // Verify reversal
    let reversed = repo.get_merge(merge.merge_id).await?.unwrap();
    assert!(reversed.is_reversed());
    assert_eq!(
        reversed.reversed_reason,
        Some("Actually different people - new evidence found".to_string())
    );
    println!("✓ Reversal verified");

    // Unarchive source person
    repo.unarchive_person(source_person.person_id, user_id)
        .await?;
    println!("✓ Source person restored");

    // Cleanup
    repo.archive_person(source_person.person_id, "Test cleanup".to_string(), user_id)
        .await?;
    repo.archive_person(target_person.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

// =============================================================================
// SEARCH AND QUERY TESTS
// =============================================================================

#[tokio::test]
async fn test_search_operations() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create test person with unique name
    let unique_surname = format!("SearchTest{}", Uuid::now_v7().to_string()[..8].to_string());
    let person = Person::builder()
        .given_name("Alice")
        .surname(&unique_surname)
        .created_by(user_id)
        .build()?;

    repo.create_person(&person).await?;
    println!("✓ Test person created with surname: {}", unique_surname);

    // Add variant name
    let variant = VariantName::builder(person.person_id)
        .given_name("Alice")
        .surname("VariantSurname")
        .variant_type(VariantNameType::Birth)
        .build()?;

    repo.add_variant(&variant).await?;
    println!("✓ Variant name added");

    // Test search by canonical name
    let results = repo.search_persons(&unique_surname, 10).await?;
    assert!(!results.is_empty());
    println!("✓ Search by canonical surname found {} results", results.len());

    // Test search by variant name
    let variant_results = repo.search_persons("VariantSurname", 10).await?;
    assert!(!variant_results.is_empty());
    println!(
        "✓ Search by variant surname found {} results",
        variant_results.len()
    );

    // Test find by name
    let by_name = repo.find_by_name(Some("Alice"), Some(&unique_surname)).await?;
    assert!(!by_name.is_empty());
    println!("✓ Find by name found {} results", by_name.len());

    // Cleanup
    repo.delete_variant(variant.variant_id).await?;
    repo.archive_person(person.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

// =============================================================================
// AGGREGATE OPERATIONS TESTS
// =============================================================================

#[tokio::test]
async fn test_aggregate_operations() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    // Create person with all data
    let person = Person::builder()
        .given_name("Complete")
        .surname("Test")
        .created_by(user_id)
        .build()?;

    repo.create_person(&person).await?;
    println!("✓ Person created");

    // Add variant
    let variant = VariantName::builder(person.person_id)
        .given_name("Complete")
        .surname("OldTest")
        .variant_type(VariantNameType::Birth)
        .build()?;
    repo.add_variant(&variant).await?;

    // Add source link
    let source_id = Uuid::now_v7();
    let source_person = SourcePerson::builder()
        .source_id(source_id)
        .person_id(person.person_id)
        .extracted_name_full("C. Test")
        .build()?;
    repo.add_source_link(&source_person).await?;

    println!("✓ Related data added");

    // Get person with all data
    let complete = repo.get_person_with_all_data(person.person_id).await?;
    assert!(complete.is_some());

    let complete = complete.unwrap();
    assert_eq!(complete.person.person_id, person.person_id);
    assert_eq!(complete.variants.len(), 1);
    assert_eq!(complete.sources.len(), 1);
    println!("✓ Retrieved complete person record with all related data");

    // Test counts
    let total_count = repo.count_persons().await?;
    let active_count = repo.count_active_persons().await?;
    println!(
        "✓ Total persons: {}, Active persons: {}",
        total_count, active_count
    );

    // Cleanup
    repo.delete_variant(variant.variant_id).await?;
    repo.delete_source_link(source_person.source_person_id)
        .await?;
    repo.archive_person(person.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    Ok(())
}

// =============================================================================
// COMPLEX WORKFLOW TESTS
// =============================================================================

#[tokio::test]
async fn test_complete_person_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let config = test_config();
    let pool = sqlx::PgPool::connect(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    ))
    .await?;

    let repo = PostgresPersonRepository::new(pool);
    let user_id = Uuid::now_v7();

    println!("\n=== Complete Person Workflow Test ===\n");

    // Step 1: Create canonical person
    let person = Person::builder()
        .given_name("Mary")
        .surname("Smith")
        .middle_name("Elizabeth")
        .birth_date(GenealogyDate::exact(1820, Some(3), Some(15)))
        .death_date(GenealogyDate::exact(1891, Some(12), Some(10)))
        .sex(Sex::Female)
        .occupation("Teacher")
        .conclusion_confidence(PersonConfidence::Definite)
        .created_by(user_id)
        .build()?;

    repo.create_person(&person).await?;
    println!("1. ✓ Created canonical person: {}", person.canonical_name());

    // Step 2: Add birth name variant
    let birth_name = VariantName::builder(person.person_id)
        .given_name("Mary")
        .surname("Johnson")
        .middle_name("Elizabeth")
        .variant_type(VariantNameType::Birth)
        .use_from_year(1820)
        .use_to_year(1845)
        .notes("Maiden name before marriage")
        .build()?;

    repo.add_variant(&birth_name).await?;
    println!("2. ✓ Added birth name variant: {}", birth_name.full_name);

    // Step 3: Create spouse
    let spouse = Person::builder()
        .given_name("John")
        .surname("Smith")
        .sex(Sex::Male)
        .created_by(user_id)
        .build()?;

    repo.create_person(&spouse).await?;
    println!("3. ✓ Created spouse: {}", spouse.canonical_name());

    // Step 4: Create marriage relationship
    let marriage = PersonRelationship::builder()
        .person_id(person.person_id)
        .related_person_id(spouse.person_id)
        .relationship_type(PersonRelationshipType::Spouse)
        .start_date(GenealogyDate::exact(1845, Some(6), Some(15)))
        .confidence(PersonConfidence::Definite)
        .build()?;

    repo.add_relationship(&marriage).await?;
    println!("4. ✓ Created marriage relationship");

    // Step 5: Add source attribution
    let census_source_id = Uuid::now_v7();
    let census_mention = SourcePerson::builder()
        .source_id(census_source_id)
        .person_id(person.person_id)
        .extracted_name_full("Mary E. Smith")
        .extracted_role("teacher")
        .page_reference("42")
        .confidence(PersonConfidence::Definite)
        .build()?;

    repo.add_source_link(&census_mention).await?;
    println!("5. ✓ Added source attribution from 1850 census");

    // Step 6: Get complete person record
    let complete = repo.get_person_with_all_data(person.person_id).await?;
    assert!(complete.is_some());

    let complete = complete.unwrap();
    println!("\n6. ✓ Retrieved complete person record:");
    println!("   - Canonical name: {}", complete.person.canonical_name());
    println!("   - Life span: {:?}", complete.person.life_span());
    println!("   - Variants: {}", complete.variants.len());
    println!("   - Relationships: {}", complete.relationships.len());
    println!("   - Source citations: {}", complete.sources.len());

    // Step 7: Cleanup
    repo.delete_variant(birth_name.variant_id).await?;
    repo.delete_relationship(marriage.relationship_id).await?;
    repo.delete_source_link(census_mention.source_person_id)
        .await?;
    repo.archive_person(person.person_id, "Test cleanup".to_string(), user_id)
        .await?;
    repo.archive_person(spouse.person_id, "Test cleanup".to_string(), user_id)
        .await?;

    println!("\n7. ✓ Cleanup completed");
    println!("\n=== Workflow Test Complete ===\n");

    Ok(())
}
