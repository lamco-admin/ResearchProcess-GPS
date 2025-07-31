//! CRUD operation tests for PostgreSQL storage backend

use rp_storage_postgres::{PostgresBackend, PostgresConfig};
use rp_storage::{StorageBackend, StorageEntity, Transaction};
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;

#[tokio::test]
async fn test_crud_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Setup test database connection
    let config = PostgresConfig {
        host: "192.168.10.90".to_string(),
        port: 5432,
        database: "researchprocess_gps".to_string(),
        username: "researchprocess_gps".to_string(),
        password: "researchprocess_gps".to_string(),
        max_connections: 5,
        min_connections: 1,
        connect_timeout: std::time::Duration::from_secs(30),
        connection_timeout: std::time::Duration::from_secs(30),
        idle_timeout: Some(std::time::Duration::from_secs(600)),
        max_lifetime: Some(std::time::Duration::from_secs(1800)),
        statement_cache_capacity: 100,
        ssl_mode: rp_storage_postgres::config::SslMode::Prefer,
        application_name: "rp-test".to_string(),
        enable_vector: false,
        enable_graph: false,
        enable_notifications: false,
    };
    
    // Create backend
    let backend = PostgresBackend::new(config).await?;
    backend.initialize().await?;
    
    // Test entity data
    let test_id = Uuid::now_v7();
    let researcher_id = Uuid::now_v7();
    let test_entity = StorageEntity {
        id: test_id,
        entity_type: "Theory".to_string(),
        data: json!({
            "question": "Who were the parents of John Smith?",
            "hypothesis": "John Smith was the son of William and Mary Smith",
            "state": "EXPLORING",
            "created_by": researcher_id,
        }),
        binary_data: None,
        created_by: researcher_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: 1,
    };
    
    // Test 1: Create (Put) operation
    {
        let mut tx = backend.begin_transaction().await?;
        tx.put_entity(&test_entity).await?;
        tx.commit().await?;
        println!("✓ Create operation successful");
    }
    
    // Test 2: Read (Get) operation
    {
        let mut tx = backend.begin_transaction().await?;
        let retrieved = tx.get_entity(test_id).await?;
        assert!(retrieved.is_some());
        let entity = retrieved.unwrap();
        assert_eq!(entity.id, test_id);
        assert_eq!(entity.entity_type, "Theory");
        assert_eq!(entity.data["question"], "Who were the parents of John Smith?");
        tx.commit().await?;
        println!("✓ Read operation successful");
    }
    
    // Test 3: Update operation
    {
        let mut tx = backend.begin_transaction().await?;
        let updates = vec![
            ("state".to_string(), json!("TESTING")),
            ("notes".to_string(), json!("Found birth certificate")),
        ].into_iter().collect();
        
        let updated = tx.update_entity(test_id, updates).await?;
        assert!(updated);
        
        // Verify update
        let entity = tx.get_entity(test_id).await?.unwrap();
        assert_eq!(entity.data["state"], "TESTING");
        assert_eq!(entity.data["notes"], "Found birth certificate");
        
        tx.commit().await?;
        println!("✓ Update operation successful");
    }
    
    // Test 4: List by type
    {
        let mut tx = backend.begin_transaction().await?;
        let theories = tx.list_by_type("Theory", Some(10), None).await?;
        assert!(!theories.is_empty());
        assert!(theories.iter().any(|e| e.id == test_id));
        tx.commit().await?;
        println!("✓ List by type operation successful");
    }
    
    // Test 5: Exists check
    {
        let mut tx = backend.begin_transaction().await?;
        assert!(tx.exists(test_id).await?);
        assert!(!tx.exists(Uuid::now_v7()).await?);
        tx.commit().await?;
        println!("✓ Exists check successful");
    }
    
    // Test 6: Get multiple entities
    {
        let mut tx = backend.begin_transaction().await?;
        
        // Create another entity
        let test_id2 = Uuid::now_v7();
        let test_entity2 = StorageEntity {
            id: test_id2,
            entity_type: "Theory".to_string(),
            data: json!({
                "question": "Where was John Smith born?",
                "state": "EXPLORING",
            }),
            binary_data: None,
            created_by: researcher_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
        };
        tx.put_entity(&test_entity2).await?;
        
        // Get multiple
        let entities = tx.get_entities(&[test_id, test_id2]).await?;
        assert_eq!(entities.len(), 2);
        
        tx.commit().await?;
        println!("✓ Get multiple entities successful");
    }
    
    // Test 7: Delete operation (soft delete)
    {
        let mut tx = backend.begin_transaction().await?;
        let deleted = tx.delete_entity(test_id).await?;
        assert!(deleted);
        
        // Verify deletion
        let entity = tx.get_entity(test_id).await?;
        assert!(entity.is_none());
        
        tx.commit().await?;
        println!("✓ Delete operation successful");
    }
    
    // Test 8: Transaction rollback
    {
        let rollback_id = Uuid::now_v7();
        let rollback_entity = StorageEntity {
            id: rollback_id,
            entity_type: "Theory".to_string(),
            data: json!({
                "question": "This should be rolled back",
                "state": "EXPLORING",
            }),
            binary_data: None,
            created_by: researcher_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: 1,
        };
        
        // Create in transaction then rollback
        let mut tx = backend.begin_transaction().await?;
        tx.put_entity(&rollback_entity).await?;
        tx.rollback().await?;
        
        // Verify it wasn't persisted
        let mut tx2 = backend.begin_transaction().await?;
        let entity = tx2.get_entity(rollback_id).await?;
        assert!(entity.is_none());
        tx2.commit().await?;
        
        println!("✓ Transaction rollback successful");
    }
    
    println!("\nAll CRUD tests passed! ✨");
    
    Ok(())
}

#[tokio::test]
async fn test_version_history() -> Result<(), Box<dyn std::error::Error>> {
    let config = PostgresConfig {
        host: "192.168.10.90".to_string(),
        port: 5432,
        database: "researchprocess_gps".to_string(),
        username: "researchprocess_gps".to_string(),
        password: "researchprocess_gps".to_string(),
        max_connections: 5,
        min_connections: 1,
        connect_timeout: std::time::Duration::from_secs(30),
        connection_timeout: std::time::Duration::from_secs(30),
        idle_timeout: Some(std::time::Duration::from_secs(600)),
        max_lifetime: Some(std::time::Duration::from_secs(1800)),
        statement_cache_capacity: 100,
        ssl_mode: rp_storage_postgres::config::SslMode::Prefer,
        application_name: "rp-test".to_string(),
        enable_vector: false,
        enable_graph: false,
        enable_notifications: false,
    };
    
    let backend = PostgresBackend::new(config).await?;
    let test_id = Uuid::now_v7();
    let researcher_id = Uuid::now_v7();
    
    // Create initial entity
    let mut tx = backend.begin_transaction().await?;
    let entity = StorageEntity {
        id: test_id,
        entity_type: "Evidence".to_string(),
        data: json!({
            "description": "Birth certificate found",
            "quality": "PRIMARY",
            "source_id": Uuid::now_v7(),
        }),
        binary_data: None,
        created_by: researcher_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        version: 1,
    };
    tx.put_entity(&entity).await?;
    
    // Add version history
    tx.add_version(test_id, rp_storage::VersionData {
        version: 1,
        data: entity.data.clone(),
        changed_by: researcher_id,
        changed_at: Utc::now(),
        change_reason: Some("Initial creation".to_string()),
        parent_version: None,
    }).await?;
    
    tx.commit().await?;
    
    println!("✓ Version history test successful");
    
    Ok(())
}