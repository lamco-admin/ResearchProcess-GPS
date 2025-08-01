//! PostgreSQL query builder for meta-model queries

use meta_model_storage::{
    Query, QueryResult, SelectClause, FilterExpression, ComparisonOp, 
    FilterValue, OrderBy, SortDirection, Include, StorageError, StorageResult,
};
use meta_model_core::layer1::{Entity, Relationship};
use meta_model_core::layer2::{Process, Product};
use meta_model_core::layer3::Workspace;
use sqlx::{PgPool, Row};
use std::fmt::Write;

pub struct PostgresQueryBuilder;

impl PostgresQueryBuilder {
    pub fn new() -> Self {
        PostgresQueryBuilder
    }
    
    pub async fn execute(&self, query: Query, pool: &PgPool) -> StorageResult<QueryResult> {
        match &query.select {
            SelectClause::Entities { types } => {
                let sql = self.build_entity_query(&query, types)?;
                let entities = self.execute_entity_query(&sql, pool).await?;
                Ok(QueryResult::Entities(entities))
            }
            SelectClause::Relationships { types } => {
                let sql = self.build_relationship_query(&query, types)?;
                let relationships = self.execute_relationship_query(&sql, pool).await?;
                Ok(QueryResult::Relationships(relationships))
            }
            SelectClause::Processes { types } => {
                let sql = self.build_process_query(&query, types)?;
                let processes = self.execute_process_query(&sql, pool).await?;
                Ok(QueryResult::Processes(processes))
            }
            SelectClause::Products { types } => {
                let sql = self.build_product_query(&query, types)?;
                let products = self.execute_product_query(&sql, pool).await?;
                Ok(QueryResult::Products(products))
            }
            SelectClause::Workspaces { types } => {
                let sql = self.build_workspace_query(&query, types)?;
                let workspaces = self.execute_workspace_query(&sql, pool).await?;
                Ok(QueryResult::Workspaces(workspaces))
            }
            SelectClause::All => {
                // Execute all queries and return mixed results
                todo!("Mixed query results")
            }
        }
    }
    
    fn build_entity_query(&self, query: &Query, types: &Option<Vec<String>>) -> StorageResult<String> {
        let mut sql = String::from("SELECT data FROM entities WHERE 1=1");
        
        // Add type filter
        if let Some(types) = types {
            write!(&mut sql, " AND entity_type = ANY(ARRAY[{}])", 
                types.iter().map(|t| format!("'{}'", t)).collect::<Vec<_>>().join(",")
            ).map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        // Add filter expression
        if let Some(filter) = &query.filter {
            let filter_sql = self.build_filter_expression(filter, "data")?;
            write!(&mut sql, " AND {}", filter_sql)
                .map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        // Add order by
        if !query.order_by.is_empty() {
            let order_clauses: Result<Vec<String>, StorageError> = query.order_by
                .iter()
                .map(|order| self.build_order_clause(order, "data"))
                .collect();
            let order_clauses = order_clauses?;
            write!(&mut sql, " ORDER BY {}", order_clauses.join(", "))
                .map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        // Add limit/offset
        if let Some(limit) = query.limit {
            write!(&mut sql, " LIMIT {}", limit)
                .map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        if let Some(offset) = query.offset {
            write!(&mut sql, " OFFSET {}", offset)
                .map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        Ok(sql)
    }
    
    fn build_relationship_query(&self, query: &Query, types: &Option<Vec<String>>) -> StorageResult<String> {
        let mut sql = String::from("SELECT data FROM relationships WHERE 1=1");
        
        if let Some(types) = types {
            write!(&mut sql, " AND relationship_type = ANY(ARRAY[{}])", 
                types.iter().map(|t| format!("'{}'", t)).collect::<Vec<_>>().join(",")
            ).map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        if let Some(filter) = &query.filter {
            let filter_sql = self.build_filter_expression(filter, "data")?;
            write!(&mut sql, " AND {}", filter_sql)
                .map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        // Similar order by and limit/offset handling
        Ok(sql)
    }
    
    fn build_process_query(&self, query: &Query, types: &Option<Vec<String>>) -> StorageResult<String> {
        let mut sql = String::from("SELECT data FROM processes WHERE 1=1");
        
        if let Some(types) = types {
            write!(&mut sql, " AND process_type = ANY(ARRAY[{}])", 
                types.iter().map(|t| format!("'{}'", t)).collect::<Vec<_>>().join(",")
            ).map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        if let Some(filter) = &query.filter {
            let filter_sql = self.build_filter_expression(filter, "data")?;
            write!(&mut sql, " AND {}", filter_sql)
                .map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        Ok(sql)
    }
    
    fn build_product_query(&self, query: &Query, types: &Option<Vec<String>>) -> StorageResult<String> {
        let mut sql = String::from("SELECT data FROM products WHERE 1=1");
        
        if let Some(types) = types {
            write!(&mut sql, " AND product_type = ANY(ARRAY[{}])", 
                types.iter().map(|t| format!("'{}'", t)).collect::<Vec<_>>().join(",")
            ).map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        if let Some(filter) = &query.filter {
            let filter_sql = self.build_filter_expression(filter, "data")?;
            write!(&mut sql, " AND {}", filter_sql)
                .map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        Ok(sql)
    }
    
    fn build_workspace_query(&self, query: &Query, types: &Option<Vec<String>>) -> StorageResult<String> {
        let mut sql = String::from("SELECT data FROM workspaces WHERE 1=1");
        
        if let Some(types) = types {
            write!(&mut sql, " AND workspace_type = ANY(ARRAY[{}])", 
                types.iter().map(|t| format!("'{}'", t)).collect::<Vec<_>>().join(",")
            ).map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        if let Some(filter) = &query.filter {
            let filter_sql = self.build_filter_expression(filter, "data")?;
            write!(&mut sql, " AND {}", filter_sql)
                .map_err(|e| StorageError::InvalidQuery(e.to_string()))?;
        }
        
        Ok(sql)
    }
    
    fn build_filter_expression(&self, expr: &FilterExpression, column: &str) -> StorageResult<String> {
        match expr {
            FilterExpression::Field { path, op, value } => {
                self.build_field_filter(column, path, op, value)
            }
            FilterExpression::And(exprs) => {
                let parts: Result<Vec<String>, StorageError> = exprs
                    .iter()
                    .map(|e| self.build_filter_expression(e, column))
                    .collect();
                Ok(format!("({})", parts?.join(" AND ")))
            }
            FilterExpression::Or(exprs) => {
                let parts: Result<Vec<String>, StorageError> = exprs
                    .iter()
                    .map(|e| self.build_filter_expression(e, column))
                    .collect();
                Ok(format!("({})", parts?.join(" OR ")))
            }
            FilterExpression::Not(expr) => {
                let inner = self.build_filter_expression(expr, column)?;
                Ok(format!("NOT {}", inner))
            }
            FilterExpression::Search { query, fields } => {
                self.build_search_filter(column, query, fields)
            }
            FilterExpression::Connected { .. } => {
                // Graph queries need special handling
                Err(StorageError::NotSupported("Graph queries not yet implemented".to_string()))
            }
        }
    }
    
    fn build_field_filter(
        &self, 
        column: &str, 
        path: &str, 
        op: &ComparisonOp, 
        value: &FilterValue
    ) -> StorageResult<String> {
        let jsonb_path = if path.is_empty() {
            column.to_string()
        } else {
            format!("{}->>'{}'", column, path.replace('.', "'->'"))
        };
        
        let sql = match op {
            ComparisonOp::Eq => match value {
                FilterValue::Null => format!("{} IS NULL", jsonb_path),
                _ => format!("{} = {}", jsonb_path, self.format_value(value)?),
            },
            ComparisonOp::Ne => match value {
                FilterValue::Null => format!("{} IS NOT NULL", jsonb_path),
                _ => format!("{} != {}", jsonb_path, self.format_value(value)?),
            },
            ComparisonOp::Gt => format!("{} > {}", jsonb_path, self.format_value(value)?),
            ComparisonOp::Gte => format!("{} >= {}", jsonb_path, self.format_value(value)?),
            ComparisonOp::Lt => format!("{} < {}", jsonb_path, self.format_value(value)?),
            ComparisonOp::Lte => format!("{} <= {}", jsonb_path, self.format_value(value)?),
            ComparisonOp::Like => format!("{} LIKE {}", jsonb_path, self.format_value(value)?),
            ComparisonOp::ILike => format!("{} ILIKE {}", jsonb_path, self.format_value(value)?),
            ComparisonOp::In => match value {
                FilterValue::List(values) => {
                    let formatted: Result<Vec<String>, StorageError> = values
                        .iter()
                        .map(|v| self.format_value(v))
                        .collect();
                    format!("{} = ANY(ARRAY[{}])", jsonb_path, formatted?.join(","))
                }
                _ => return Err(StorageError::InvalidQuery("IN requires a list value".to_string())),
            },
            ComparisonOp::NotIn => match value {
                FilterValue::List(values) => {
                    let formatted: Result<Vec<String>, StorageError> = values
                        .iter()
                        .map(|v| self.format_value(v))
                        .collect();
                    format!("NOT ({} = ANY(ARRAY[{}]))", jsonb_path, formatted?.join(","))
                }
                _ => return Err(StorageError::InvalidQuery("NOT IN requires a list value".to_string())),
            },
            ComparisonOp::IsNull => format!("{} IS NULL", jsonb_path),
            ComparisonOp::IsNotNull => format!("{} IS NOT NULL", jsonb_path),
            ComparisonOp::Contains => format!("{} @> {}", column, self.format_value(value)?),
            ComparisonOp::Exists => format!("{} ? '{}'", column, path),
        };
        
        Ok(sql)
    }
    
    fn build_search_filter(
        &self,
        column: &str,
        query: &str,
        fields: &Option<Vec<String>>
    ) -> StorageResult<String> {
        if let Some(fields) = fields {
            // Search specific fields
            let conditions: Vec<String> = fields
                .iter()
                .map(|field| format!("{}->'{}' ??| '{}'", column, field, query))
                .collect();
            Ok(format!("({})", conditions.join(" OR ")))
        } else {
            // Full text search on entire JSON
            Ok(format!("to_tsvector('english', {}::text) @@ plainto_tsquery('english', '{}')", 
                column, query))
        }
    }
    
    fn build_order_clause(&self, order: &OrderBy, column: &str) -> StorageResult<String> {
        let jsonb_path = if order.path.is_empty() {
            column.to_string()
        } else {
            format!("{}->>'{}'", column, order.path.replace('.', "'->'"))
        };
        
        let direction = match order.direction {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };
        
        let nulls = match order.nulls {
            Some(meta_model_storage::query::NullsOrder::First) => " NULLS FIRST",
            Some(meta_model_storage::query::NullsOrder::Last) => " NULLS LAST",
            None => "",
        };
        
        Ok(format!("{} {}{}", jsonb_path, direction, nulls))
    }
    
    fn format_value(&self, value: &FilterValue) -> StorageResult<String> {
        match value {
            FilterValue::Text(s) => Ok(format!("'{}'", s.replace('\'', "''"))),
            FilterValue::Number(n) => Ok(n.to_string()),
            FilterValue::Integer(i) => Ok(i.to_string()),
            FilterValue::Boolean(b) => Ok(b.to_string()),
            FilterValue::Null => Ok("NULL".to_string()),
            FilterValue::Json(v) => Ok(format!("'{}'::jsonb", serde_json::to_string(v)
                .map_err(StorageError::Serialization)?)),
            FilterValue::List(_) => Err(StorageError::InvalidQuery("List values need special handling".to_string())),
        }
    }
    
    async fn execute_entity_query(&self, sql: &str, pool: &PgPool) -> StorageResult<Vec<Entity>> {
        let rows = sqlx::query(sql)
            .fetch_all(pool)
            .await
            .map_err(|e| StorageError::Backend(e.to_string()))?;
        
        let mut entities = Vec::new();
        for row in rows {
            let data: serde_json::Value = row.get("data");
            let entity = serde_json::from_value(data)
                .map_err(StorageError::Serialization)?;
            entities.push(entity);
        }
        
        Ok(entities)
    }
    
    async fn execute_relationship_query(&self, sql: &str, pool: &PgPool) -> StorageResult<Vec<Relationship>> {
        let rows = sqlx::query(sql)
            .fetch_all(pool)
            .await
            .map_err(|e| StorageError::Backend(e.to_string()))?;
        
        let mut relationships = Vec::new();
        for row in rows {
            let data: serde_json::Value = row.get("data");
            let relationship = serde_json::from_value(data)
                .map_err(StorageError::Serialization)?;
            relationships.push(relationship);
        }
        
        Ok(relationships)
    }
    
    async fn execute_process_query(&self, sql: &str, pool: &PgPool) -> StorageResult<Vec<Process>> {
        let rows = sqlx::query(sql)
            .fetch_all(pool)
            .await
            .map_err(|e| StorageError::Backend(e.to_string()))?;
        
        let mut processes = Vec::new();
        for row in rows {
            let data: serde_json::Value = row.get("data");
            let process = serde_json::from_value(data)
                .map_err(StorageError::Serialization)?;
            processes.push(process);
        }
        
        Ok(processes)
    }
    
    async fn execute_product_query(&self, sql: &str, pool: &PgPool) -> StorageResult<Vec<Product>> {
        let rows = sqlx::query(sql)
            .fetch_all(pool)
            .await
            .map_err(|e| StorageError::Backend(e.to_string()))?;
        
        let mut products = Vec::new();
        for row in rows {
            let data: serde_json::Value = row.get("data");
            let product = serde_json::from_value(data)
                .map_err(StorageError::Serialization)?;
            products.push(product);
        }
        
        Ok(products)
    }
    
    async fn execute_workspace_query(&self, sql: &str, pool: &PgPool) -> StorageResult<Vec<Workspace>> {
        let rows = sqlx::query(sql)
            .fetch_all(pool)
            .await
            .map_err(|e| StorageError::Backend(e.to_string()))?;
        
        let mut workspaces = Vec::new();
        for row in rows {
            let data: serde_json::Value = row.get("data");
            let workspace = serde_json::from_value(data)
                .map_err(StorageError::Serialization)?;
            workspaces.push(workspace);
        }
        
        Ok(workspaces)
    }
}