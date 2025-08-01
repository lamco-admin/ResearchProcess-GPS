//! Query system for meta-model storage

use meta_model_core::layer1::{Entity, EntityId, Relationship, RelationshipId};
use meta_model_core::layer2::{Process, ProcessId, Product, ProductId};
use meta_model_core::layer3::{Workspace, WorkspaceId};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Query for searching meta-model data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// What type of objects to return
    pub select: SelectClause,
    
    /// Filter conditions
    pub filter: Option<FilterExpression>,
    
    /// Sort order
    pub order_by: Vec<OrderBy>,
    
    /// Pagination
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    
    /// Include related data
    pub include: Vec<Include>,
}

/// What to select
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectClause {
    Entities {
        types: Option<Vec<String>>,
    },
    Relationships {
        types: Option<Vec<String>>,
    },
    Processes {
        types: Option<Vec<String>>,
    },
    Products {
        types: Option<Vec<String>>,
    },
    Workspaces {
        types: Option<Vec<String>>,
    },
    All,
}

/// Filter expression (can be nested)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterExpression {
    /// Simple field comparison
    Field {
        path: String,
        op: ComparisonOp,
        value: FilterValue,
    },
    
    /// Logical AND
    And(Vec<FilterExpression>),
    
    /// Logical OR
    Or(Vec<FilterExpression>),
    
    /// Logical NOT
    Not(Box<FilterExpression>),
    
    /// Full-text search
    Search {
        query: String,
        fields: Option<Vec<String>>,
    },
    
    /// Graph traversal filter
    Connected {
        to: EntityId,
        via: Option<String>, // relationship type
        depth: Option<usize>,
    },
}

/// Comparison operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOp {
    Eq,       // =
    Ne,       // !=
    Gt,       // >
    Gte,      // >=
    Lt,       // <
    Lte,      // <=
    Like,     // SQL LIKE
    ILike,    // Case-insensitive LIKE
    In,       // IN
    NotIn,    // NOT IN
    IsNull,   // IS NULL
    IsNotNull,// IS NOT NULL
    Contains, // JSON contains
    Exists,   // JSON path exists
}

/// Filter value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterValue {
    Text(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Null,
    List(Vec<FilterValue>),
    Json(serde_json::Value),
}

/// Order by clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBy {
    pub path: String,
    pub direction: SortDirection,
    pub nulls: Option<NullsOrder>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NullsOrder {
    First,
    Last,
}

/// What to include with results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Include {
    /// Include relationships for entities
    Relationships {
        types: Option<Vec<String>>,
        direction: Option<crate::traits::RelationshipDirection>,
    },
    
    /// Include contexts
    Contexts,
    
    /// Include specific properties
    Properties(Vec<String>),
    
    /// Include related entities
    RelatedEntities {
        via: Option<String>, // relationship type
        depth: usize,
    },
}

/// Query results
#[derive(Debug, Clone)]
pub enum QueryResult {
    Entities(Vec<Entity>),
    Relationships(Vec<Relationship>),
    Processes(Vec<Process>),
    Products(Vec<Product>),
    Workspaces(Vec<Workspace>),
    Mixed {
        entities: Vec<Entity>,
        relationships: Vec<Relationship>,
        processes: Vec<Process>,
        products: Vec<Product>,
        workspaces: Vec<Workspace>,
    },
}

impl QueryResult {
    pub fn is_empty(&self) -> bool {
        match self {
            QueryResult::Entities(v) => v.is_empty(),
            QueryResult::Relationships(v) => v.is_empty(),
            QueryResult::Processes(v) => v.is_empty(),
            QueryResult::Products(v) => v.is_empty(),
            QueryResult::Workspaces(v) => v.is_empty(),
            QueryResult::Mixed { entities, relationships, processes, products, workspaces } => {
                entities.is_empty() && relationships.is_empty() && 
                processes.is_empty() && products.is_empty() && workspaces.is_empty()
            }
        }
    }
    
    pub fn count(&self) -> usize {
        match self {
            QueryResult::Entities(v) => v.len(),
            QueryResult::Relationships(v) => v.len(),
            QueryResult::Processes(v) => v.len(),
            QueryResult::Products(v) => v.len(),
            QueryResult::Workspaces(v) => v.len(),
            QueryResult::Mixed { entities, relationships, processes, products, workspaces } => {
                entities.len() + relationships.len() + processes.len() + 
                products.len() + workspaces.len()
            }
        }
    }
}

/// Query builder for fluent API
pub struct QueryBuilder {
    query: Query,
}

impl QueryBuilder {
    /// Start building a query
    pub fn select(select: SelectClause) -> Self {
        QueryBuilder {
            query: Query {
                select,
                filter: None,
                order_by: Vec::new(),
                limit: None,
                offset: None,
                include: Vec::new(),
            },
        }
    }
    
    /// Select all entities
    pub fn select_entities() -> Self {
        Self::select(SelectClause::Entities { types: None })
    }
    
    /// Select entities of specific types
    pub fn select_entities_of_type(types: Vec<String>) -> Self {
        Self::select(SelectClause::Entities { types: Some(types) })
    }
    
    /// Add a filter
    pub fn filter(mut self, filter: FilterExpression) -> Self {
        self.query.filter = Some(filter);
        self
    }
    
    /// Add field equals filter
    pub fn where_eq(self, path: &str, value: impl Into<FilterValue>) -> Self {
        self.filter(FilterExpression::Field {
            path: path.to_string(),
            op: ComparisonOp::Eq,
            value: value.into(),
        })
    }
    
    /// Add field like filter
    pub fn where_like(self, path: &str, pattern: &str) -> Self {
        self.filter(FilterExpression::Field {
            path: path.to_string(),
            op: ComparisonOp::Like,
            value: FilterValue::Text(pattern.to_string()),
        })
    }
    
    /// Add full-text search
    pub fn search(self, query: &str) -> Self {
        self.filter(FilterExpression::Search {
            query: query.to_string(),
            fields: None,
        })
    }
    
    /// Order by field
    pub fn order_by(mut self, path: &str, direction: SortDirection) -> Self {
        self.query.order_by.push(OrderBy {
            path: path.to_string(),
            direction,
            nulls: None,
        });
        self
    }
    
    /// Set limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.query.limit = Some(limit);
        self
    }
    
    /// Set offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.query.offset = Some(offset);
        self
    }
    
    /// Include relationships
    pub fn include_relationships(mut self) -> Self {
        self.query.include.push(Include::Relationships {
            types: None,
            direction: None,
        });
        self
    }
    
    /// Include contexts
    pub fn include_contexts(mut self) -> Self {
        self.query.include.push(Include::Contexts);
        self
    }
    
    /// Build the query
    pub fn build(self) -> Query {
        self.query
    }
}

// Implement Into<FilterValue> for common types
impl From<String> for FilterValue {
    fn from(v: String) -> Self {
        FilterValue::Text(v)
    }
}

impl From<&str> for FilterValue {
    fn from(v: &str) -> Self {
        FilterValue::Text(v.to_string())
    }
}

impl From<i32> for FilterValue {
    fn from(v: i32) -> Self {
        FilterValue::Integer(v as i64)
    }
}

impl From<i64> for FilterValue {
    fn from(v: i64) -> Self {
        FilterValue::Integer(v)
    }
}

impl From<f64> for FilterValue {
    fn from(v: f64) -> Self {
        FilterValue::Number(v)
    }
}

impl From<bool> for FilterValue {
    fn from(v: bool) -> Self {
        FilterValue::Boolean(v)
    }
}

/// Common queries
impl Query {
    /// Find all persons
    pub fn all_persons() -> Query {
        QueryBuilder::select_entities_of_type(vec!["Person".to_string()])
            .build()
    }
    
    /// Find persons by name pattern
    pub fn persons_by_name(pattern: &str) -> Query {
        QueryBuilder::select_entities_of_type(vec!["Person".to_string()])
            .search(pattern)
            .build()
    }
    
    /// Find active processes
    pub fn active_processes() -> Query {
        QueryBuilder::select(SelectClause::Processes { types: None })
            .where_eq("state", "Active")
            .build()
    }
    
    /// Find relationships for entity
    pub fn relationships_for(entity_id: EntityId) -> Query {
        QueryBuilder::select(SelectClause::Relationships { types: None })
            .filter(FilterExpression::Or(vec![
                FilterExpression::Field {
                    path: "participants[*].entity".to_string(),
                    op: ComparisonOp::Contains,
                    value: FilterValue::Text(entity_id.to_string()),
                },
            ]))
            .build()
    }
}