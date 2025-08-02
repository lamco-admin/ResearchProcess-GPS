//! PostgreSQL backend implementation

use async_trait::async_trait;
use meta_model_storage::{
    StorageBackend, StorageCapabilities, StorageResult, StorageError,
    HealthStatus, Transaction, QueryableBackend, GraphBackend, BulkOperations,
    Query, QueryResult, IndexDefinition, IndexInfo,
    RelationshipDirection, TraversalPattern, Path,
    BulkResult, BulkError,
};
use meta_model_core::layer1::{Entity, EntityId, Relationship, RelationshipId};
use meta_model_core::layer2::{Process, ProcessId, Product, ProductId};
use meta_model_core::layer3::{Workspace, WorkspaceId};
use sqlx::{PgPool, postgres::PgPoolOptions, Row};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use tracing::{info, error};

use crate::{PostgresError, PostgresResult};
use crate::query_builder::PostgresQueryBuilder;

/// PostgreSQL storage backend
pub struct PostgresBackend {
    pool: PgPool,
    capabilities: StorageCapabilities,
}

impl PostgresBackend {
    /// Create a new PostgreSQL backend
    pub async fn new(connection_string: &str) -> PostgresResult<Self> {
        info!("Creating PostgreSQL backend");

        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(connection_string)
            .await?;

        let capabilities = StorageCapabilities::full_featured();

        Ok(PostgresBackend {
            pool,
            capabilities,
        })
    }

    /// Get the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
impl StorageBackend for PostgresBackend {
    async fn initialize(&self) -> StorageResult<()> {
        info!("Initializing PostgreSQL backend");

        // Run migrations
        crate::migrations::run_migrations(&self.pool)
            .await
            .map_err(|e| StorageError::Backend(format!("Migration failed: {}", e)))?;

        Ok(())
    }

    async fn health_check(&self) -> StorageResult<HealthStatus> {
        let mut details = HashMap::new();

        // Check connection
        match sqlx::query("SELECT 1").fetch_one(&self.pool).await {
            Ok(_) => {
                details.insert("connection".to_string(), serde_json::json!("ok"));
            }
            Err(e) => {
                error!("Health check failed: {}", e);
                return Ok(HealthStatus {
                    healthy: false,
                    message: format!("Database connection failed: {}", e),
                    details,
                    checked_at: Utc::now(),
                });
            }
        }

        // Check tables exist
        let table_check = sqlx::query(
            "SELECT COUNT(*) FROM information_schema.tables
             WHERE table_schema = 'public'
             AND table_name IN ('entities', 'relationships', 'processes', 'products', 'workspaces')"
        )
        .fetch_one(&self.pool)
        .await;

        match table_check {
            Ok(row) => {
                let count: i64 = row.get(0);
                details.insert("tables".to_string(), serde_json::json!(count));

                Ok(HealthStatus {
                    healthy: count == 5,
                    message: if count == 5 {
                        "All tables present".to_string()
                    } else {
                        format!("Missing tables: found {}/5", count)
                    },
                    details,
                    checked_at: Utc::now(),
                })
            }
            Err(e) => {
                Ok(HealthStatus {
                    healthy: false,
                    message: format!("Table check failed: {}", e),
                    details,
                    checked_at: Utc::now(),
                })
            }
        }
    }

    async fn begin_transaction(&self) -> StorageResult<Transaction> {
        // PostgreSQL handles transactions at the connection level
        // For now, return a simple transaction handle
        Ok(Transaction::new())
    }

    fn capabilities(&self) -> StorageCapabilities {
        self.capabilities.clone()
    }

    fn backend_type(&self) -> &str {
        "PostgreSQL"
    }

    async fn shutdown(&self) -> StorageResult<()> {
        info!("Shutting down PostgreSQL backend");
        self.pool.close().await;
        Ok(())
    }

    // Layer 1: Entity operations

    async fn store_entity(&self, entity: &Entity) -> StorageResult<()> {
        let data = serde_json::to_value(entity)
            .map_err(StorageError::Serialization)?;

        sqlx::query(
            "INSERT INTO entities (id, entity_type, state, data, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET
                entity_type = EXCLUDED.entity_type,
                state = EXCLUDED.state,
                data = EXCLUDED.data,
                updated_at = EXCLUDED.updated_at"
        )
        .bind(entity.id.0)
        .bind(&entity.entity_type)
        .bind(&entity.state)
        .bind(&data)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(())
    }

    async fn get_entity(&self, id: EntityId) -> StorageResult<Option<Entity>> {
        let row = sqlx::query(
            "SELECT data FROM entities WHERE id = $1"
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let entity = serde_json::from_value(data)
                    .map_err(StorageError::Serialization)?;
                Ok(Some(entity))
            }
            None => Ok(None),
        }
    }

    async fn update_entity(&self, entity: &Entity) -> StorageResult<()> {
        let data = serde_json::to_value(entity)
            .map_err(StorageError::Serialization)?;

        let result = sqlx::query(
            "UPDATE entities SET
                entity_type = $2,
                state = $3,
                data = $4,
                updated_at = $5
             WHERE id = $1"
        )
        .bind(entity.id.0)
        .bind(&entity.entity_type)
        .bind(&entity.state)
        .bind(&data)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Entity {} not found", entity.id)));
        }

        Ok(())
    }

    async fn delete_entity(&self, id: EntityId) -> StorageResult<bool> {
        let result = sqlx::query(
            "DELETE FROM entities WHERE id = $1"
        )
        .bind(id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    // Layer 1: Relationship operations

    async fn store_relationship(&self, relationship: &Relationship) -> StorageResult<()> {
        let data = serde_json::to_value(relationship)
            .map_err(StorageError::Serialization)?;

        // Extract participant entity IDs for indexing
        let participant_ids: Vec<Uuid> = relationship.participants
            .iter()
            .map(|p| p.entity.0)
            .collect();

        sqlx::query(
            "INSERT INTO relationships (id, relationship_type, participant_ids, data, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET
                relationship_type = EXCLUDED.relationship_type,
                participant_ids = EXCLUDED.participant_ids,
                data = EXCLUDED.data,
                updated_at = EXCLUDED.updated_at"
        )
        .bind(relationship.id.0)
        .bind(&relationship.relationship_type)
        .bind(&participant_ids)
        .bind(&data)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(())
    }

    async fn get_relationship(&self, id: RelationshipId) -> StorageResult<Option<Relationship>> {
        let row = sqlx::query(
            "SELECT data FROM relationships WHERE id = $1"
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let relationship = serde_json::from_value(data)
                    .map_err(StorageError::Serialization)?;
                Ok(Some(relationship))
            }
            None => Ok(None),
        }
    }

    async fn update_relationship(&self, relationship: &Relationship) -> StorageResult<()> {
        let data = serde_json::to_value(relationship)
            .map_err(StorageError::Serialization)?;

        let participant_ids: Vec<Uuid> = relationship.participants
            .iter()
            .map(|p| p.entity.0)
            .collect();

        let result = sqlx::query(
            "UPDATE relationships SET
                relationship_type = $2,
                participant_ids = $3,
                data = $4,
                updated_at = $5
             WHERE id = $1"
        )
        .bind(relationship.id.0)
        .bind(&relationship.relationship_type)
        .bind(&participant_ids)
        .bind(&data)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Relationship {} not found", relationship.id)));
        }

        Ok(())
    }

    async fn delete_relationship(&self, id: RelationshipId) -> StorageResult<bool> {
        let result = sqlx::query(
            "DELETE FROM relationships WHERE id = $1"
        )
        .bind(id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    // Layer 2: Process operations

    async fn store_process(&self, process: &Process) -> StorageResult<()> {
        let data = serde_json::to_value(process)
            .map_err(StorageError::Serialization)?;

        sqlx::query(
            "INSERT INTO processes (id, process_type, state, data, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET
                process_type = EXCLUDED.process_type,
                state = EXCLUDED.state,
                data = EXCLUDED.data,
                updated_at = EXCLUDED.updated_at"
        )
        .bind(process.id.0)
        .bind(&process.process_type)
        .bind(&process.state)
        .bind(&data)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(())
    }

    async fn get_process(&self, id: ProcessId) -> StorageResult<Option<Process>> {
        let row = sqlx::query(
            "SELECT data FROM processes WHERE id = $1"
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let process = serde_json::from_value(data)
                    .map_err(StorageError::Serialization)?;
                Ok(Some(process))
            }
            None => Ok(None),
        }
    }

    async fn update_process(&self, process: &Process) -> StorageResult<()> {
        let data = serde_json::to_value(process)
            .map_err(StorageError::Serialization)?;

        let result = sqlx::query(
            "UPDATE processes SET
                process_type = $2,
                state = $3,
                data = $4,
                updated_at = $5
             WHERE id = $1"
        )
        .bind(process.id.0)
        .bind(&process.process_type)
        .bind(&process.state)
        .bind(&data)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Process {} not found", process.id)));
        }

        Ok(())
    }

    async fn delete_process(&self, id: ProcessId) -> StorageResult<bool> {
        let result = sqlx::query(
            "DELETE FROM processes WHERE id = $1"
        )
        .bind(id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    // Layer 2: Product operations

    async fn store_product(&self, product: &Product) -> StorageResult<()> {
        let data = serde_json::to_value(product)
            .map_err(StorageError::Serialization)?;

        sqlx::query(
            "INSERT INTO products (id, product_type, state, data, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET
                product_type = EXCLUDED.product_type,
                state = EXCLUDED.state,
                data = EXCLUDED.data,
                updated_at = EXCLUDED.updated_at"
        )
        .bind(product.id.0)
        .bind(&product.product_type)
        .bind(&product.state)
        .bind(&data)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(())
    }

    async fn get_product(&self, id: ProductId) -> StorageResult<Option<Product>> {
        let row = sqlx::query(
            "SELECT data FROM products WHERE id = $1"
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let product = serde_json::from_value(data)
                    .map_err(StorageError::Serialization)?;
                Ok(Some(product))
            }
            None => Ok(None),
        }
    }

    async fn update_product(&self, product: &Product) -> StorageResult<()> {
        let data = serde_json::to_value(product)
            .map_err(StorageError::Serialization)?;

        let result = sqlx::query(
            "UPDATE products SET
                product_type = $2,
                state = $3,
                data = $4,
                updated_at = $5
             WHERE id = $1"
        )
        .bind(product.id.0)
        .bind(&product.product_type)
        .bind(&product.state)
        .bind(&data)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Product {} not found", product.id)));
        }

        Ok(())
    }

    async fn delete_product(&self, id: ProductId) -> StorageResult<bool> {
        let result = sqlx::query(
            "DELETE FROM products WHERE id = $1"
        )
        .bind(id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    // Layer 3: Workspace operations

    async fn store_workspace(&self, workspace: &Workspace) -> StorageResult<()> {
        let data = serde_json::to_value(workspace)
            .map_err(StorageError::Serialization)?;

        sqlx::query(
            "INSERT INTO workspaces (id, workspace_type, state, data, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET
                workspace_type = EXCLUDED.workspace_type,
                state = EXCLUDED.state,
                data = EXCLUDED.data,
                updated_at = EXCLUDED.updated_at"
        )
        .bind(workspace.id.0)
        .bind(&workspace.workspace_type)
        .bind(&workspace.state)
        .bind(&data)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(())
    }

    async fn get_workspace(&self, id: WorkspaceId) -> StorageResult<Option<Workspace>> {
        let row = sqlx::query(
            "SELECT data FROM workspaces WHERE id = $1"
        )
        .bind(id.0)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        match row {
            Some(row) => {
                let data: serde_json::Value = row.get("data");
                let workspace = serde_json::from_value(data)
                    .map_err(StorageError::Serialization)?;
                Ok(Some(workspace))
            }
            None => Ok(None),
        }
    }

    async fn update_workspace(&self, workspace: &Workspace) -> StorageResult<()> {
        let data = serde_json::to_value(workspace)
            .map_err(StorageError::Serialization)?;

        let result = sqlx::query(
            "UPDATE workspaces SET
                workspace_type = $2,
                state = $3,
                data = $4,
                updated_at = $5
             WHERE id = $1"
        )
        .bind(workspace.id.0)
        .bind(&workspace.workspace_type)
        .bind(&workspace.state)
        .bind(&data)
        .bind(Utc::now())
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Workspace {} not found", workspace.id)));
        }

        Ok(())
    }

    async fn delete_workspace(&self, id: WorkspaceId) -> StorageResult<bool> {
        let result = sqlx::query(
            "DELETE FROM workspaces WHERE id = $1"
        )
        .bind(id.0)
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }
}

#[async_trait]
impl QueryableBackend for PostgresBackend {
    async fn query(&self, query: Query) -> StorageResult<QueryResult> {
        let query_builder = PostgresQueryBuilder::new();
        query_builder.execute(query, &self.pool).await
    }

    async fn create_index(&self, index: IndexDefinition) -> StorageResult<()> {
        // Create PostgreSQL index based on definition
        // This would generate appropriate CREATE INDEX statements
        todo!("Index creation")
    }

    async fn list_indexes(&self) -> StorageResult<Vec<IndexInfo>> {
        // Query pg_indexes to get index information
        todo!("List indexes")
    }

    async fn drop_index(&self, name: &str) -> StorageResult<()> {
        sqlx::query(&format!("DROP INDEX IF EXISTS {}", name))
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::Backend(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl GraphBackend for PostgresBackend {
    async fn get_relationships_for_entity(
        &self,
        entity_id: EntityId,
        direction: Option<RelationshipDirection>,
        types: Option<Vec<String>>,
    ) -> StorageResult<Vec<Relationship>> {
        let mut query = String::from(
            "SELECT data FROM relationships WHERE $1 = ANY(participant_ids)"
        );

        if let Some(types) = types {
            query.push_str(" AND relationship_type = ANY($2)");
        }

        let mut q = sqlx::query(&query).bind(entity_id.0);

        if let Some(types) = types {
            q = q.bind(&types);
        }

        let rows = q.fetch_all(&self.pool)
            .await
            .map_err(|e| StorageError::Backend(e.to_string()))?;

        let mut relationships = Vec::new();
        for row in rows {
            let data: serde_json::Value = row.get("data");
            let relationship: Relationship = serde_json::from_value(data)
                .map_err(StorageError::Serialization)?;

            // Filter by direction if specified
            if let Some(dir) = direction {
                let matches = match dir {
                    RelationshipDirection::Outgoing => {
                        relationship.participants.first()
                            .map(|p| p.entity == entity_id)
                            .unwrap_or(false)
                    }
                    RelationshipDirection::Incoming => {
                        relationship.participants.last()
                            .map(|p| p.entity == entity_id)
                            .unwrap_or(false)
                    }
                    RelationshipDirection::Both => true,
                };

                if matches {
                    relationships.push(relationship);
                }
            } else {
                relationships.push(relationship);
            }
        }

        Ok(relationships)
    }

    async fn traverse(
        &self,
        _start: EntityId,
        _pattern: TraversalPattern,
    ) -> StorageResult<Vec<Path>> {
        // Graph traversal would use recursive CTEs or graph extensions
        todo!("Graph traversal")
    }

    async fn shortest_path(
        &self,
        _from: EntityId,
        _to: EntityId,
        _max_depth: Option<usize>,
    ) -> StorageResult<Option<Path>> {
        // Shortest path would use graph algorithms
        todo!("Shortest path")
    }
}

#[async_trait]
impl BulkOperations for PostgresBackend {
    async fn bulk_insert_entities(&self, entities: Vec<Entity>) -> StorageResult<BulkResult> {
        let mut successful = 0;
        let mut errors = Vec::new();

        for (index, entity) in entities.iter().enumerate() {
            match self.store_entity(entity).await {
                Ok(_) => successful += 1,
                Err(e) => errors.push(BulkError {
                    index,
                    id: Some(entity.id.0),
                    error: e.to_string(),
                }),
            }
        }

        Ok(BulkResult {
            successful,
            failed: errors.len(),
            errors,
        })
    }

    async fn bulk_insert_relationships(&self, relationships: Vec<Relationship>) -> StorageResult<BulkResult> {
        let mut successful = 0;
        let mut errors = Vec::new();

        for (index, relationship) in relationships.iter().enumerate() {
            match self.store_relationship(relationship).await {
                Ok(_) => successful += 1,
                Err(e) => errors.push(BulkError {
                    index,
                    id: Some(relationship.id.0),
                    error: e.to_string(),
                }),
            }
        }

        Ok(BulkResult {
            successful,
            failed: errors.len(),
            errors,
        })
    }

    async fn bulk_update_entities(&self, entities: Vec<Entity>) -> StorageResult<BulkResult> {
        let mut successful = 0;
        let mut errors = Vec::new();

        for (index, entity) in entities.iter().enumerate() {
            match self.update_entity(entity).await {
                Ok(_) => successful += 1,
                Err(e) => errors.push(BulkError {
                    index,
                    id: Some(entity.id.0),
                    error: e.to_string(),
                }),
            }
        }

        Ok(BulkResult {
            successful,
            failed: errors.len(),
            errors,
        })
    }

    async fn bulk_delete(&self, object_type: &str, ids: Vec<Uuid>) -> StorageResult<BulkResult> {
        let table = match object_type {
            "entity" => "entities",
            "relationship" => "relationships",
            "process" => "processes",
            "product" => "products",
            "workspace" => "workspaces",
            _ => return Err(StorageError::InvalidQuery(format!("Unknown object type: {}", object_type))),
        };

        let query = format!("DELETE FROM {} WHERE id = ANY($1)", table);
        let result = sqlx::query(&query)
            .bind(&ids)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::Backend(e.to_string()))?;

        Ok(BulkResult {
            successful: result.rows_affected() as usize,
            failed: ids.len() - result.rows_affected() as usize,
            errors: Vec::new(),
        })
    }
}