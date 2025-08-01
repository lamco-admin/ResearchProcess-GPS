// Query system for meta-model

use crate::layer1::{Entity, Relationship};
use crate::layer2::{Process, Product};
use crate::layer3::Workspace;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Query for searching meta-model data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Target type to query
    pub target: QueryTarget,
    
    /// Filter conditions
    pub filters: Vec<Filter>,
    
    /// Sort order
    pub order_by: Vec<OrderBy>,
    
    /// Pagination
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    
    /// Include related data
    pub include: Vec<Include>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryTarget {
    Entities,
    Relationships,
    Processes,
    Products,
    Workspaces,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: FilterValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    In,
    NotIn,
    Exists,
    NotExists,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    List(Vec<String>),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBy {
    pub field: String,
    pub direction: OrderDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Include {
    Relationships,
    Contexts,
    Properties(Vec<String>),
}

/// Query result
#[derive(Debug, Clone)]
pub enum QueryResult {
    Entities(Vec<Entity>),
    Relationships(Vec<Relationship>),
    Processes(Vec<Process>),
    Products(Vec<Product>),
    Workspaces(Vec<Workspace>),
    Mixed(MixedResults),
}

#[derive(Debug, Clone)]
pub struct MixedResults {
    pub entities: Vec<Entity>,
    pub relationships: Vec<Relationship>,
    pub processes: Vec<Process>,
    pub products: Vec<Product>,
    pub workspaces: Vec<Workspace>,
}

/// Query builder for fluent API
pub struct QueryBuilder {
    query: Query,
}

impl QueryBuilder {
    pub fn new(target: QueryTarget) -> Self {
        QueryBuilder {
            query: Query {
                target,
                filters: Vec::new(),
                order_by: Vec::new(),
                limit: None,
                offset: None,
                include: Vec::new(),
            },
        }
    }
    
    /// Add a filter
    pub fn filter(mut self, field: &str, operator: FilterOperator, value: FilterValue) -> Self {
        self.query.filters.push(Filter {
            field: field.to_string(),
            operator,
            value,
        });
        self
    }
    
    /// Filter by entity type
    pub fn entity_type(self, entity_type: &str) -> Self {
        self.filter("entity_type", FilterOperator::Equals, FilterValue::Text(entity_type.to_string()))
    }
    
    /// Filter by state
    pub fn state(self, state: &str) -> Self {
        self.filter("state", FilterOperator::Equals, FilterValue::Text(state.to_string()))
    }
    
    /// Filter by property
    pub fn property(self, key: &str, value: &str) -> Self {
        self.filter(
            &format!("properties.{}", key),
            FilterOperator::Equals,
            FilterValue::Text(value.to_string())
        )
    }
    
    /// Search text in properties
    pub fn search(self, text: &str) -> Self {
        self.filter("_search", FilterOperator::Contains, FilterValue::Text(text.to_string()))
    }
    
    /// Order by field
    pub fn order_by(mut self, field: &str, direction: OrderDirection) -> Self {
        self.query.order_by.push(OrderBy {
            field: field.to_string(),
            direction,
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
        self.query.include.push(Include::Relationships);
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

/// Common queries
impl Query {
    /// Find all persons
    pub fn all_persons() -> Query {
        QueryBuilder::new(QueryTarget::Entities)
            .entity_type("Person")
            .build()
    }
    
    /// Find persons by name
    pub fn persons_by_name(name: &str) -> Query {
        QueryBuilder::new(QueryTarget::Entities)
            .entity_type("Person")
            .search(name)
            .build()
    }
    
    /// Find active processes
    pub fn active_processes() -> Query {
        QueryBuilder::new(QueryTarget::Processes)
            .state("Active")
            .build()
    }
    
    /// Find relationships for entity
    pub fn relationships_for(entity_id: crate::layer1::EntityId) -> Query {
        QueryBuilder::new(QueryTarget::Relationships)
            .filter("participants", FilterOperator::Contains, FilterValue::Text(entity_id.to_string()))
            .build()
    }
}