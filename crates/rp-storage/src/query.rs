//! Query abstraction for storage backends

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::StorageEntity;

/// Query builder for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    /// Entity type to query
    pub entity_type: Option<String>,
    
    /// Filter conditions
    pub filters: Vec<Filter>,
    
    /// Sort order
    pub sort: Vec<SortField>,
    
    /// Pagination
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    
    /// Fields to include/exclude
    pub projection: Option<Projection>,
    
    /// Aggregation operations
    pub aggregations: Vec<Aggregation>,
    
    /// Grouping fields
    pub group_by: Vec<String>,
    
    /// Having conditions (for aggregations)
    pub having: Vec<Filter>,
}

impl Query {
    /// Create a new empty query
    pub fn new() -> Self {
        Self {
            entity_type: None,
            filters: Vec::new(),
            sort: Vec::new(),
            limit: None,
            offset: None,
            projection: None,
            aggregations: Vec::new(),
            group_by: Vec::new(),
            having: Vec::new(),
        }
    }
    
    /// Set entity type
    pub fn entity_type(mut self, entity_type: impl Into<String>) -> Self {
        self.entity_type = Some(entity_type.into());
        self
    }
    
    /// Add a filter
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filters.push(filter);
        self
    }
    
    /// Add sort field
    pub fn sort(mut self, field: impl Into<String>, order: SortOrder) -> Self {
        self.sort.push(SortField {
            field: field.into(),
            order,
        });
        self
    }
    
    /// Set limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Set offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Filter conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Filter {
    /// Equality check
    Eq { field: String, value: JsonValue },
    
    /// Not equal
    Ne { field: String, value: JsonValue },
    
    /// Greater than
    Gt { field: String, value: JsonValue },
    
    /// Greater than or equal
    Gte { field: String, value: JsonValue },
    
    /// Less than
    Lt { field: String, value: JsonValue },
    
    /// Less than or equal
    Lte { field: String, value: JsonValue },
    
    /// Value in list
    In { field: String, values: Vec<JsonValue> },
    
    /// Value not in list
    NotIn { field: String, values: Vec<JsonValue> },
    
    /// Field contains substring (for strings)
    Contains { field: String, value: String },
    
    /// Field starts with (for strings)
    StartsWith { field: String, value: String },
    
    /// Field ends with (for strings)
    EndsWith { field: String, value: String },
    
    /// Field is null
    IsNull { field: String },
    
    /// Field is not null
    IsNotNull { field: String },
    
    /// JSON path exists
    JsonPathExists { path: String },
    
    /// JSON path value equals
    JsonPathEq { path: String, value: JsonValue },
    
    /// Logical AND
    And(Vec<Filter>),
    
    /// Logical OR
    Or(Vec<Filter>),
    
    /// Logical NOT
    Not(Box<Filter>),
    
    /// Date range
    DateRange {
        field: String,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    },
    
    /// Full text search
    FullText { fields: Vec<String>, query: String },
}

/// Sort field and order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortField {
    pub field: String,
    pub order: SortOrder,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Field projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Projection {
    /// Include only these fields
    Include(Vec<String>),
    
    /// Exclude these fields
    Exclude(Vec<String>),
}

/// Aggregation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Aggregation {
    Count { field: Option<String>, alias: String },
    Sum { field: String, alias: String },
    Avg { field: String, alias: String },
    Min { field: String, alias: String },
    Max { field: String, alias: String },
    Distinct { field: String, alias: String },
}

/// Query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Matching entities
    pub entities: Vec<StorageEntity>,
    
    /// Total count (if requested)
    pub total_count: Option<usize>,
    
    /// Aggregation results
    pub aggregations: HashMap<String, JsonValue>,
    
    /// Query execution time
    pub execution_time: std::time::Duration,
    
    /// Additional metadata
    pub metadata: HashMap<String, JsonValue>,
}

/// Change stream for real-time updates
pub struct ChangeStream {
    // Implementation depends on backend
    _marker: std::marker::PhantomData<()>,
}

/// History stream for replaying changes
pub struct HistoryStream {
    // Implementation depends on backend
    _marker: std::marker::PhantomData<()>,
}

/// Convenience functions for building queries
impl Filter {
    /// Create an equality filter
    pub fn eq(field: impl Into<String>, value: impl Serialize) -> Self {
        Self::Eq {
            field: field.into(),
            value: serde_json::to_value(value)
                .expect("Filter value serialization should never fail"),
        }
    }
    
    /// Create a "not equal" filter
    pub fn ne(field: impl Into<String>, value: impl Serialize) -> Self {
        Self::Ne {
            field: field.into(),
            value: serde_json::to_value(value)
                .expect("Filter value serialization should never fail"),
        }
    }
    
    /// Create an "in" filter
    pub fn in_list<T: Serialize>(field: impl Into<String>, values: Vec<T>) -> Self {
        Self::In {
            field: field.into(),
            values: values
                .into_iter()
                .map(|v| serde_json::to_value(v)
                    .expect("Filter value serialization should never fail"))
                .collect(),
        }
    }
    
    /// Combine filters with AND
    pub fn and(filters: Vec<Filter>) -> Self {
        Self::And(filters)
    }
    
    /// Combine filters with OR
    pub fn or(filters: Vec<Filter>) -> Self {
        Self::Or(filters)
    }
}