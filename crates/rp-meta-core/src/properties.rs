//! Flexible property graph system
//!
//! The PropertyGraph is the core of the meta-model's flexibility.
//! It can contain any type of data, including nested entities, computed values,
//! and quantum superpositions.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{EntityId, Error, Result};

/// A flexible property graph that can contain any type of property
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyGraph {
    properties: IndexMap<String, Property>,
}

impl PropertyGraph {
    /// Create a new empty property graph
    pub fn new() -> Self {
        Self {
            properties: IndexMap::new(),
        }
    }

    /// Set a property value
    pub fn set(&mut self, key: impl Into<String>, value: Property) {
        self.properties.insert(key.into(), value);
    }

    /// Get a property value
    pub fn get(&self, key: &str) -> Option<&Property> {
        self.properties.get(key)
    }

    /// Get a mutable property value
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Property> {
        self.properties.get_mut(key)
    }

    /// Remove a property
    pub fn remove(&mut self, key: &str) -> Option<Property> {
        self.properties.shift_remove(key)
    }

    /// Check if a property exists
    pub fn contains(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }

    /// Get all property keys
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.properties.keys()
    }

    /// Get all properties
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.properties.iter()
    }

    /// Get the number of properties
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Check if the property graph is empty
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }

    /// Merge another property graph into this one
    pub fn merge(&mut self, other: PropertyGraph) {
        self.properties.extend(other.properties);
    }

    // Convenience methods for common types

    /// Set a text property
    pub fn set_text(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.set(key, Property::Value(Value::Text(value.into())));
    }

    /// Get a text property
    pub fn get_text(&self, key: &str) -> Result<&str> {
        match self.get(key) {
            Some(Property::Value(Value::Text(s))) => Ok(s),
            Some(other) => Err(Error::property_type_mismatch(
                key,
                "Text",
                format!("{:?}", other),
            )),
            None => Err(Error::PropertyNotFound {
                property: key.to_string(),
            }),
        }
    }

    /// Set an integer property
    pub fn set_integer(&mut self, key: impl Into<String>, value: i64) {
        self.set(key, Property::Value(Value::Integer(value)));
    }

    /// Get an integer property
    pub fn get_integer(&self, key: &str) -> Result<i64> {
        match self.get(key) {
            Some(Property::Value(Value::Integer(i))) => Ok(*i),
            Some(other) => Err(Error::property_type_mismatch(
                key,
                "Integer",
                format!("{:?}", other),
            )),
            None => Err(Error::PropertyNotFound {
                property: key.to_string(),
            }),
        }
    }

    /// Set a float property
    pub fn set_float(&mut self, key: impl Into<String>, value: f64) {
        self.set(key, Property::Value(Value::Float(value)));
    }

    /// Get a float property
    pub fn get_float(&self, key: &str) -> Result<f64> {
        match self.get(key) {
            Some(Property::Value(Value::Float(f))) => Ok(*f),
            Some(other) => Err(Error::property_type_mismatch(
                key,
                "Float",
                format!("{:?}", other),
            )),
            None => Err(Error::PropertyNotFound {
                property: key.to_string(),
            }),
        }
    }

    /// Set a boolean property
    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.set(key, Property::Value(Value::Boolean(value)));
    }

    /// Get a boolean property
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        match self.get(key) {
            Some(Property::Value(Value::Boolean(b))) => Ok(*b),
            Some(other) => Err(Error::property_type_mismatch(
                key,
                "Boolean",
                format!("{:?}", other),
            )),
            None => Err(Error::PropertyNotFound {
                property: key.to_string(),
            }),
        }
    }

    /// Set a reference property
    pub fn set_reference(&mut self, key: impl Into<String>, entity_id: EntityId) {
        self.set(key, Property::Reference(entity_id));
    }

    /// Get a reference property
    pub fn get_reference(&self, key: &str) -> Result<EntityId> {
        match self.get(key) {
            Some(Property::Reference(id)) => Ok(*id),
            Some(other) => Err(Error::property_type_mismatch(
                key,
                "Reference",
                format!("{:?}", other),
            )),
            None => Err(Error::PropertyNotFound {
                property: key.to_string(),
            }),
        }
    }

    /// Set a collection property
    pub fn set_collection(&mut self, key: impl Into<String>, values: Vec<Property>) {
        self.set(key, Property::Collection(values));
    }

    /// Get a collection property
    pub fn get_collection(&self, key: &str) -> Result<&Vec<Property>> {
        match self.get(key) {
            Some(Property::Collection(vec)) => Ok(vec),
            Some(other) => Err(Error::property_type_mismatch(
                key,
                "Collection",
                format!("{:?}", other),
            )),
            None => Err(Error::PropertyNotFound {
                property: key.to_string(),
            }),
        }
    }
}

impl Default for PropertyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// A property value that can be any type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Property {
    /// Simple value
    Value(Value),

    /// Reference to another entity
    Reference(EntityId),

    /// Collection of properties
    Collection(Vec<Property>),

    /// Nested property map
    Map(IndexMap<String, Property>),

    /// Computed/derived property
    Computation(Computation),

    /// Theoretical/hypothetical value
    Theoretical(TheoreticalValue),

    /// Quantum superposition of properties with probabilities
    Quantum(Vec<(Property, f64)>),
}

/// Basic value types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Value {
    /// Null/None value
    Null,

    /// Boolean value
    Boolean(bool),

    /// Integer value
    Integer(i64),

    /// Floating point value
    Float(f64),

    /// Text value
    Text(String),

    /// Binary data
    Binary(Vec<u8>),

    /// Identifier (system, id)
    Identifier(String, String),

    /// Quantity with unit (value, unit)
    Quantity(f64, String),

    /// Arbitrary JSON value
    Json(serde_json::Value),

    /// Expression that produces a value
    Expression(String),
}

/// Computed/derived property
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Computation {
    /// The expression to compute
    pub expression: String,

    /// The language/interpreter for the expression
    pub language: String,

    /// Dependencies (property IDs or paths)
    pub dependencies: Vec<String>,
}

/// Theoretical/hypothetical value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TheoreticalValue {
    /// The hypothesis
    pub hypothesis: String,

    /// Supporting evidence (entity IDs)
    pub supporting_evidence: Vec<EntityId>,

    /// Contradicting evidence (entity IDs)
    pub contradicting_evidence: Vec<EntityId>,

    /// Probability (0.0 to 1.0)
    pub probability: f64,
}

// Helper macro for creating property graphs
#[macro_export]
macro_rules! properties {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut pg = $crate::PropertyGraph::new();
            $(
                pg.set($key, $value);
            )*
            pg
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_graph_basic() {
        let mut pg = PropertyGraph::new();
        pg.set_text("name", "John");
        pg.set_integer("age", 42);
        pg.set_bool("active", true);

        assert_eq!(pg.get_text("name").unwrap(), "John");
        assert_eq!(pg.get_integer("age").unwrap(), 42);
        assert!(pg.get_bool("active").unwrap());
    }

    #[test]
    fn test_property_graph_reference() {
        let mut pg = PropertyGraph::new();
        let id = EntityId::new();
        pg.set_reference("parent", id);

        assert_eq!(pg.get_reference("parent").unwrap(), id);
    }

    #[test]
    fn test_property_graph_collection() {
        let mut pg = PropertyGraph::new();
        pg.set_collection(
            "tags",
            vec![
                Property::Value(Value::Text("tag1".to_string())),
                Property::Value(Value::Text("tag2".to_string())),
            ],
        );

        let tags = pg.get_collection("tags").unwrap();
        assert_eq!(tags.len(), 2);
    }

    #[test]
    fn test_property_graph_errors() {
        let pg = PropertyGraph::new();

        // Missing property
        assert!(pg.get_text("missing").is_err());

        // Type mismatch
        let mut pg = PropertyGraph::new();
        pg.set_text("name", "John");
        assert!(pg.get_integer("name").is_err());
    }

    #[test]
    fn test_property_graph_macro() {
        let pg = properties! {
            "name" => Property::Value(Value::Text("John".to_string())),
            "age" => Property::Value(Value::Integer(42)),
        };

        assert_eq!(pg.get_text("name").unwrap(), "John");
        assert_eq!(pg.get_integer("age").unwrap(), 42);
    }

    #[test]
    fn test_property_graph_merge() {
        let mut pg1 = PropertyGraph::new();
        pg1.set_text("name", "John");

        let mut pg2 = PropertyGraph::new();
        pg2.set_integer("age", 42);

        pg1.merge(pg2);

        assert_eq!(pg1.get_text("name").unwrap(), "John");
        assert_eq!(pg1.get_integer("age").unwrap(), 42);
    }

    #[test]
    fn test_theoretical_value() {
        let theoretical = TheoreticalValue {
            hypothesis: "John was born in Boston".to_string(),
            supporting_evidence: vec![EntityId::new()],
            contradicting_evidence: vec![],
            probability: 0.75,
        };

        let prop = Property::Theoretical(theoretical);
        assert!(matches!(prop, Property::Theoretical(_)));
    }

    #[test]
    fn test_quantum_property() {
        let quantum = Property::Quantum(vec![
            (Property::Value(Value::Text("Boston".to_string())), 0.6),
            (Property::Value(Value::Text("New York".to_string())), 0.4),
        ]);

        assert!(matches!(quantum, Property::Quantum(_)));
    }
}
