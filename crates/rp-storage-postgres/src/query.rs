//! PostgreSQL query implementation

use async_trait::async_trait;
use sqlx::QueryBuilder;
use serde_json::Value as JsonValue;
use tracing::{debug, instrument};
use chrono::{DateTime, Utc};

use rp_storage::{
    QueryableBackend, StorageResult, StorageError,
    Query, QueryResult, Filter, SortOrder,
    IndexDefinition, IndexInfo, StorageEntity,
};
use std::collections::HashMap;

use crate::{PostgresBackend, PostgresError};

#[async_trait]
impl QueryableBackend for PostgresBackend {
    #[instrument(skip(self, query))]
    async fn query(&self, query: Query) -> StorageResult<QueryResult> {
        debug!("Executing query");
        let start = std::time::Instant::now();
        
        let mut builder = QueryBuilder::new(
            "SELECT id, entity_type, data, binary_data, created_by, created_at, updated_at, version FROM entities WHERE deleted_at IS NULL"
        );
        
        // Add entity type filter if specified
        if let Some(entity_type) = &query.entity_type {
            builder.push(" AND entity_type = ");
            builder.push_bind(entity_type);
        }
        
        // Add filters
        for filter in &query.filters {
            builder.push(" AND ");
            apply_filter(&mut builder, filter)?;
        }
        
        // Add sorting
        if !query.sort.is_empty() {
            builder.push(" ORDER BY ");
            for (i, sort_field) in query.sort.iter().enumerate() {
                if i > 0 {
                    builder.push(", ");
                }
                
                // Handle JSON path vs regular column
                if sort_field.field.contains("->") {
                    builder.push(format!("data{}", sort_field.field));
                } else {
                    builder.push(&sort_field.field);
                }
                
                match sort_field.order {
                    SortOrder::Asc => builder.push(" ASC"),
                    SortOrder::Desc => builder.push(" DESC"),
                };
            }
        } else {
            builder.push(" ORDER BY created_at DESC");
        }
        
        // Add limit and offset
        if let Some(limit) = query.limit {
            builder.push(" LIMIT ");
            builder.push_bind(limit as i64);
        }
        
        if let Some(offset) = query.offset {
            builder.push(" OFFSET ");
            builder.push_bind(offset as i64);
        }
        
        // Execute query
        let mut conn = self.pool().pool().acquire().await
            .map_err(PostgresError::from)?;
        
        let rows = builder
            .build_query_as::<EntityRow>()
            .fetch_all(&mut *conn)
            .await
            .map_err(PostgresError::from)?;
        
        let entities: Vec<StorageEntity> = rows.into_iter().map(|r| r.into()).collect();
        
        // Get total count if needed
        let total_count = if query.limit.is_some() || query.offset.is_some() {
            Some(self.get_total_count(&query).await?)
        } else {
            Some(entities.len())
        };
        
        let execution_time = start.elapsed();
        
        Ok(QueryResult {
            entities,
            total_count,
            aggregations: HashMap::new(), // TODO: Implement aggregations
            execution_time,
            metadata: HashMap::new(),
        })
    }
    
    #[instrument(skip(self, index))]
    async fn create_index(&self, index: IndexDefinition) -> StorageResult<()> {
        debug!("Creating index: {}", index.name);
        
        let mut conn = self.pool().pool().acquire().await
            .map_err(PostgresError::from)?;
        
        // Build index SQL
        let mut sql = format!("CREATE ");
        if index.unique {
            sql.push_str("UNIQUE ");
        }
        sql.push_str(&format!("INDEX IF NOT EXISTS {} ON entities ", index.name));
        
        // Add index type based on fields
        if index.fields.iter().any(|f| f.path.starts_with("data->")) {
            sql.push_str("USING GIN ");
        }
        
        sql.push('(');
        for (i, field) in index.fields.iter().enumerate() {
            if i > 0 {
                sql.push_str(", ");
            }
            sql.push_str(&field.path);
        }
        sql.push(')');
        
        // Add WHERE clause for partial index
        sql.push_str(&format!(" WHERE entity_type = '{}' AND deleted_at IS NULL", index.entity_type));
        
        sqlx::query(&sql)
            .execute(&mut *conn)
            .await
            .map_err(PostgresError::from)?;
        
        Ok(())
    }
    
    #[instrument(skip(self))]
    async fn list_indexes(&self) -> StorageResult<Vec<IndexInfo>> {
        debug!("Listing indexes");
        
        let mut conn = self.pool().pool().acquire().await
            .map_err(PostgresError::from)?;
        
        let rows = sqlx::query_as::<_, IndexRow>(
            r#"
            SELECT 
                indexname as name,
                indexdef as definition,
                pg_relation_size(indexrelid) as size_bytes
            FROM pg_indexes
            JOIN pg_class ON pg_class.relname = indexname
            WHERE tablename = 'entities'
            AND schemaname = 'public'
            ORDER BY indexname
            "#
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(PostgresError::from)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    #[instrument(skip(self))]
    async fn drop_index(&self, name: &str) -> StorageResult<()> {
        debug!("Dropping index: {}", name);
        
        let mut conn = self.pool().pool().acquire().await
            .map_err(PostgresError::from)?;
        
        sqlx::query(&format!("DROP INDEX IF EXISTS {}", name))
            .execute(&mut *conn)
            .await
            .map_err(PostgresError::from)?;
        
        Ok(())
    }
}

impl PostgresBackend {
    /// Get total count for a query (without limit/offset)
    async fn get_total_count(&self, query: &Query) -> StorageResult<usize> {
        let mut builder = QueryBuilder::new(
            "SELECT COUNT(*) FROM entities WHERE deleted_at IS NULL"
        );
        
        if let Some(entity_type) = &query.entity_type {
            builder.push(" AND entity_type = ");
            builder.push_bind(entity_type);
        }
        
        for filter in &query.filters {
            builder.push(" AND ");
            apply_filter(&mut builder, filter)?;
        }
        
        let mut conn = self.pool().pool().acquire().await
            .map_err(PostgresError::from)?;
        
        let count: i64 = builder
            .build_query_scalar()
            .fetch_one(&mut *conn)
            .await
            .map_err(PostgresError::from)?;
        
        Ok(count as usize)
    }
}

/// Apply a filter to the query builder
fn apply_filter<'q>(builder: &mut QueryBuilder<'q, sqlx::Postgres>, filter: &Filter) -> StorageResult<()> {
    match filter {
        Filter::Eq { field, value } => {
            if field.contains("->") {
                builder.push(format!("data{} = ", field));
            } else {
                builder.push(format!("{} = ", field));
            }
            builder.push_bind(value.clone());
        }
        Filter::Ne { field, value } => {
            if field.contains("->") {
                builder.push(format!("data{} != ", field));
            } else {
                builder.push(format!("{} != ", field));
            }
            builder.push_bind(value.clone());
        }
        Filter::Gt { field, value } => {
            if field.contains("->") {
                builder.push(format!("(data{})::numeric > ", field));
            } else {
                builder.push(format!("{} > ", field));
            }
            builder.push_bind(value.clone());
        }
        Filter::Gte { field, value } => {
            if field.contains("->") {
                builder.push(format!("(data{})::numeric >= ", field));
            } else {
                builder.push(format!("{} >= ", field));
            }
            builder.push_bind(value.clone());
        }
        Filter::Lt { field, value } => {
            if field.contains("->") {
                builder.push(format!("(data{})::numeric < ", field));
            } else {
                builder.push(format!("{} < ", field));
            }
            builder.push_bind(value.clone());
        }
        Filter::Lte { field, value } => {
            if field.contains("->") {
                builder.push(format!("(data{})::numeric <= ", field));
            } else {
                builder.push(format!("{} <= ", field));
            }
            builder.push_bind(value.clone());
        }
        Filter::In { field, values } => {
            // Use PostgreSQL's proper JSONB array handling
            if field.contains("->") {
                // For JSONB fields, we need to use the containment operator or array comparison
                builder.push(format!("data{} = ANY(", field));
                builder.push_bind(serde_json::to_value(values).unwrap());
                builder.push("::jsonb[])");
            } else {
                // For regular columns, use standard SQL array
                builder.push(format!("{} = ANY(", field));
                builder.push_bind(values.clone());
                builder.push(")");
            }
        }
        Filter::Contains { field, value } => {
            if field.contains("->") {
                builder.push(format!("data{} ILIKE ", field));
            } else {
                builder.push(format!("{} ILIKE ", field));
            }
            builder.push_bind(format!("%{}%", value));
        }
        Filter::IsNull { field } => {
            if field.contains("->") {
                builder.push(format!("data{} IS NULL", field));
            } else {
                builder.push(format!("{} IS NULL", field));
            }
        }
        Filter::IsNotNull { field } => {
            if field.contains("->") {
                builder.push(format!("data{} IS NOT NULL", field));
            } else {
                builder.push(format!("{} IS NOT NULL", field));
            }
        }
        Filter::And(filters) => {
            builder.push("(");
            for (i, f) in filters.iter().enumerate() {
                if i > 0 {
                    builder.push(" AND ");
                }
                apply_filter(builder, f)?;
            }
            builder.push(")");
        }
        Filter::Or(filters) => {
            builder.push("(");
            for (i, f) in filters.iter().enumerate() {
                if i > 0 {
                    builder.push(" OR ");
                }
                apply_filter(builder, f)?;
            }
            builder.push(")");
        }
        Filter::Not(f) => {
            builder.push("NOT (");
            apply_filter(builder, f)?;
            builder.push(")");
        }
        _ => {
            return Err(StorageError::NotSupported(
                format!("Filter type not yet implemented: {:?}", filter)
            ));
        }
    }
    
    Ok(())
}

// Helper structs
#[derive(sqlx::FromRow)]
struct EntityRow {
    id: uuid::Uuid,
    entity_type: String,
    data: JsonValue,
    binary_data: Option<Vec<u8>>,
    created_by: uuid::Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    version: i64,
}

impl From<EntityRow> for StorageEntity {
    fn from(row: EntityRow) -> Self {
        Self {
            id: row.id,
            entity_type: row.entity_type,
            data: row.data,
            binary_data: row.binary_data,
            created_by: row.created_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
            version: row.version as u64,
        }
    }
}

#[derive(sqlx::FromRow)]
struct IndexRow {
    name: String,
    definition: String,
    size_bytes: Option<i64>,
}

impl From<IndexRow> for IndexInfo {
    fn from(row: IndexRow) -> Self {
        // Parse definition to extract fields
        // This is a simplified version
        let fields = Vec::new(); // TODO: Parse from definition
        
        Self {
            name: row.name,
            entity_type: String::new(), // TODO: Extract from WHERE clause
            fields,
            unique: row.definition.contains("UNIQUE"),
            sparse: false,
            size: row.size_bytes.map(|s| s as u64),
            created_at: Utc::now(), // PostgreSQL doesn't track this
        }
    }
}