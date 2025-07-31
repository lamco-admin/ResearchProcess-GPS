use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;

use rp_core::{Theory, TheoryState, Researcher, ResearcherType, ResearcherState};
use rp_storage::{StorageEntity, Transaction as StorageTrait};
use rp_storage_postgres::{PostgresBackend, PostgresConfig, EventSourcedTransaction};
use rp_events::{
    EventStore, PostgresEventStore, ProjectionManager, 
    EntityCountProjection, StateDistributionProjection, RecentActivityProjection,
    create_projection_tables,
};

async fn setup_test_db() -> PgPool {
    let config = PostgresConfig {
        host: "192.168.10.90".to_string(),
        port: 5432,
        database: "researchprocess_gps".to_string(),
        username: "researchprocess_gps".to_string(),
        password: "researchprocess_gps".to_string(),
        ..Default::default()
    };
    
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.connection_string())
        .await
        .expect("Failed to connect to database");
    
    // Ensure projection tables exist
    create_projection_tables(&pool).await.expect("Failed to create projection tables");
    
    pool
}

#[tokio::test]
async fn test_event_sourcing_crud_integration() {
    let pool = setup_test_db().await;
    let event_store = PostgresEventStore::new(pool.clone());
    
    // Create test entities
    let researcher_id = Uuid::new_v4();
    let theory_id = Uuid::new_v4();
    
    // Create researcher
    let researcher = Researcher {
        id: researcher_id,
        name: "Dr. Jane Smith".to_string(),
        researcher_type: ResearcherType::Individual,
        state: ResearcherState::Active,
        created_by: researcher_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ..Default::default()
    };
    
    // Convert to storage entity
    let researcher_entity = StorageEntity {
        id: researcher.id,
        entity_type: "Researcher".to_string(),
        data: serde_json::to_value(&researcher).unwrap(),
        binary_data: None,
        created_by: researcher_id,
        created_at: researcher.created_at,
        updated_at: researcher.updated_at,
        version: 1,
    };
    
    // Start transaction with event sourcing
    let tx = pool.begin().await.unwrap();
    let mut event_tx = EventSourcedTransaction::new(pool.clone(), tx, researcher_id);
    
    // Store researcher (should generate event)
    event_tx.put_entity(&researcher_entity).await.unwrap();
    
    // Create theory
    let theory = Theory {
        id: theory_id,
        question: "Who were the parents of John Smith?".to_string(),
        hypothesis: "John Smith's parents were William Smith and Mary Jones".to_string(),
        state: TheoryState::Exploring,
        created_by: researcher_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ..Default::default()
    };
    
    let theory_entity = StorageEntity {
        id: theory.id,
        entity_type: "Theory".to_string(),
        data: serde_json::to_value(&theory).unwrap(),
        binary_data: None,
        created_by: researcher_id,
        created_at: theory.created_at,
        updated_at: theory.updated_at,
        version: 1,
    };
    
    // Store theory (should generate event)
    event_tx.put_entity(&theory_entity).await.unwrap();
    
    // Commit transaction
    event_tx.commit().await.unwrap();
    
    // Verify events were created
    let researcher_events = event_store.get_events(researcher_id, None, None).await.unwrap();
    assert_eq!(researcher_events.events.len(), 1);
    assert_eq!(researcher_events.aggregate_type, "Researcher");
    
    let theory_events = event_store.get_events(theory_id, None, None).await.unwrap();
    assert_eq!(theory_events.events.len(), 1);
    assert_eq!(theory_events.aggregate_type, "Theory");
    
    // Test state change
    let tx = pool.begin().await.unwrap();
    let mut event_tx = EventSourcedTransaction::new(pool.clone(), tx, researcher_id);
    
    // Update theory state
    let mut updated_theory = theory_entity.clone();
    updated_theory.data["state"] = serde_json::json!("Testing");
    updated_theory.updated_at = Utc::now();
    
    event_tx.put_entity(&updated_theory).await.unwrap();
    event_tx.commit().await.unwrap();
    
    // Verify state change event
    let theory_events = event_store.get_events(theory_id, None, None).await.unwrap();
    assert_eq!(theory_events.events.len(), 2);
    
    // Clean up
    sqlx::query!("DELETE FROM events WHERE aggregate_id IN ($1, $2)", researcher_id, theory_id)
        .execute(&pool)
        .await
        .unwrap();
    
    sqlx::query!("DELETE FROM entities WHERE id IN ($1, $2)", researcher_id, theory_id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_projections() {
    let pool = setup_test_db().await;
    
    // Set up projection manager
    let mut projection_manager = ProjectionManager::new(pool.clone());
    projection_manager.register(Box::new(EntityCountProjection::new()));
    projection_manager.register(Box::new(StateDistributionProjection::new()));
    projection_manager.register(Box::new(RecentActivityProjection::new()));
    
    // Create test data
    let researcher_id = Uuid::new_v4();
    let theory_id = Uuid::new_v4();
    
    // Start transaction
    let tx = pool.begin().await.unwrap();
    let mut event_tx = EventSourcedTransaction::new(pool.clone(), tx, researcher_id);
    
    // Create entities that will generate events
    let researcher_entity = StorageEntity {
        id: researcher_id,
        entity_type: "Researcher".to_string(),
        data: serde_json::json!({
            "name": "Test Researcher",
            "state": "Active"
        }),
        binary_data: None,
        created_by: researcher_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: 1,
    };
    
    event_tx.put_entity(&researcher_entity).await.unwrap();
    
    let theory_entity = StorageEntity {
        id: theory_id,
        entity_type: "Theory".to_string(),
        data: serde_json::json!({
            "question": "Test question",
            "state": "Exploring"
        }),
        binary_data: None,
        created_by: researcher_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: 1,
    };
    
    event_tx.put_entity(&theory_entity).await.unwrap();
    event_tx.commit().await.unwrap();
    
    // Process projections
    projection_manager.process_events(None).await.unwrap();
    
    // Verify entity count projection
    let count_result = sqlx::query!(
        "SELECT count FROM projection_entity_count WHERE entity_type = 'Theory'"
    )
    .fetch_optional(&pool)
    .await
    .unwrap();
    
    assert!(count_result.is_some());
    assert_eq!(count_result.unwrap().count, 1);
    
    // Verify recent activity projection
    let activity_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM projection_recent_activity"
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    
    assert_eq!(activity_count.count.unwrap(), 2); // 2 create events
    
    // Clean up
    sqlx::query!("DELETE FROM projection_entity_count").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM projection_state_distribution").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM projection_recent_activity").execute(&pool).await.unwrap();
    sqlx::query!("DELETE FROM events WHERE aggregate_id IN ($1, $2)", researcher_id, theory_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM entities WHERE id IN ($1, $2)", researcher_id, theory_id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_event_versioning() {
    let pool = setup_test_db().await;
    let event_store = PostgresEventStore::new(pool.clone());
    
    let aggregate_id = Uuid::new_v4();
    let actor_id = Uuid::new_v4();
    
    // Create multiple versions
    for i in 1..=5 {
        let tx = pool.begin().await.unwrap();
        let mut event_tx = EventSourcedTransaction::new(pool.clone(), tx, actor_id);
        
        let entity = StorageEntity {
            id: aggregate_id,
            entity_type: "Theory".to_string(),
            data: serde_json::json!({
                "question": format!("Question version {}", i),
                "version": i
            }),
            binary_data: None,
            created_by: actor_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: i as u64,
        };
        
        event_tx.put_entity(&entity).await.unwrap();
        event_tx.commit().await.unwrap();
    }
    
    // Get all events
    let events = event_store.get_events(aggregate_id, None, None).await.unwrap();
    assert_eq!(events.events.len(), 5);
    assert_eq!(events.current_version, 5);
    
    // Get events from version 3 to 4
    let partial_events = event_store.get_events(aggregate_id, Some(3), Some(4)).await.unwrap();
    assert_eq!(partial_events.events.len(), 2);
    
    // Clean up
    sqlx::query!("DELETE FROM events WHERE aggregate_id = $1", aggregate_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query!("DELETE FROM entities WHERE id = $1", aggregate_id)
        .execute(&pool)
        .await
        .unwrap();
}