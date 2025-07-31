//! Filter definitions for queries and searches

use chrono::{DateTime, Utc};
use rp_core::layer3::EntityType;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use uuid::Uuid;

/// Generic filter builder
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilterBuilder {
    filters: HashMap<String, FilterValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FilterValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Uuid(Uuid),
    DateTime(DateTime<Utc>),
    Array(Vec<FilterValue>),
    Object(HashMap<String, FilterValue>),
    Json(JsonValue),
    Null,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with(mut self, key: impl Into<String>, value: impl Into<FilterValue>) -> Self {
        self.filters.insert(key.into(), value.into());
        self
    }
    
    pub fn entity_type(self, entity_type: EntityType) -> Self {
        // Serialize EntityType as JSON value
        self.with("entity_type", FilterValue::Json(serde_json::to_value(&entity_type).unwrap_or(serde_json::Value::Null)))
    }
    
    pub fn workspace_id(self, workspace_id: Uuid) -> Self {
        self.with("workspace_id", FilterValue::Uuid(workspace_id))
    }
    
    pub fn state(self, state: impl Into<String>) -> Self {
        self.with("state", state.into())
    }
    
    pub fn created_after(self, date: DateTime<Utc>) -> Self {
        self.with("created_after", FilterValue::DateTime(date))
    }
    
    pub fn created_before(self, date: DateTime<Utc>) -> Self {
        self.with("created_before", FilterValue::DateTime(date))
    }
    
    pub fn build(self) -> HashMap<String, FilterValue> {
        self.filters
    }
}

/// Common filter presets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterPreset {
    Active,
    Inactive,
    Recent,
    MyItems,
    Shared,
    Archived,
}

impl FilterPreset {
    pub fn to_filters(&self, user_id: Option<Uuid>) -> HashMap<String, FilterValue> {
        match self {
            Self::Active => {
                let mut filters = HashMap::new();
                filters.insert("state".to_string(), FilterValue::String("ACTIVE".to_string()));
                filters
            }
            Self::Inactive => {
                let mut filters = HashMap::new();
                filters.insert("state".to_string(), FilterValue::String("INACTIVE".to_string()));
                filters
            }
            Self::Recent => {
                let mut filters = HashMap::new();
                let one_week_ago = Utc::now() - chrono::Duration::days(7);
                filters.insert("created_after".to_string(), FilterValue::DateTime(one_week_ago));
                filters
            }
            Self::MyItems => {
                let mut filters = HashMap::new();
                if let Some(user_id) = user_id {
                    filters.insert("created_by".to_string(), FilterValue::Uuid(user_id));
                }
                filters
            }
            Self::Shared => {
                let mut filters = HashMap::new();
                filters.insert("shared".to_string(), FilterValue::Boolean(true));
                filters
            }
            Self::Archived => {
                let mut filters = HashMap::new();
                filters.insert("archived".to_string(), FilterValue::Boolean(true));
                filters
            }
        }
    }
}

/// Date range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
}

/// Text search options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSearchOptions {
    pub query: String,
    #[serde(default)]
    pub fields: Vec<String>,
    #[serde(default)]
    pub fuzzy: bool,
    #[serde(default)]
    pub highlight: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Complex filter expression
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum FilterExpression {
    And { conditions: Vec<FilterExpression> },
    Or { conditions: Vec<FilterExpression> },
    Not { condition: Box<FilterExpression> },
    Eq { field: String, value: JsonValue },
    Ne { field: String, value: JsonValue },
    Gt { field: String, value: JsonValue },
    Gte { field: String, value: JsonValue },
    Lt { field: String, value: JsonValue },
    Lte { field: String, value: JsonValue },
    In { field: String, values: Vec<JsonValue> },
    Contains { field: String, value: String },
    StartsWith { field: String, value: String },
    EndsWith { field: String, value: String },
    IsNull { field: String },
    IsNotNull { field: String },
}

// Conversion implementations
impl From<String> for FilterValue {
    fn from(s: String) -> Self {
        FilterValue::String(s)
    }
}

impl From<&str> for FilterValue {
    fn from(s: &str) -> Self {
        FilterValue::String(s.to_string())
    }
}

impl From<f64> for FilterValue {
    fn from(n: f64) -> Self {
        FilterValue::Number(n)
    }
}

impl From<bool> for FilterValue {
    fn from(b: bool) -> Self {
        FilterValue::Boolean(b)
    }
}

impl From<Uuid> for FilterValue {
    fn from(u: Uuid) -> Self {
        FilterValue::Uuid(u)
    }
}

impl From<DateTime<Utc>> for FilterValue {
    fn from(dt: DateTime<Utc>) -> Self {
        FilterValue::DateTime(dt)
    }
}

impl From<JsonValue> for FilterValue {
    fn from(v: JsonValue) -> Self {
        FilterValue::Json(v)
    }
}