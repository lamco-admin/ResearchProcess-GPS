// Property Graph - Infinitely extensible property system

use super::{EntityId, TemporalValue};
use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Property graph - can hold any data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyGraph {
    properties: IndexMap<String, Property>,
}

/// A property can be many things
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Property {
    /// Simple value
    Value(Value),
    
    /// Reference to another entity
    Reference(EntityId),
    
    /// Nested entity (fully contained)
    Entity(Box<super::Entity>),
    
    /// Collection of properties
    Collection(Vec<Property>),
    
    /// Map of properties
    Map(IndexMap<String, Property>),
    
    /// Quantum superposition (property + probability)
    Quantum(Vec<(Property, f64)>),
    
    /// Theoretical value with evidence
    Theoretical(TheoreticalValue),
}

/// Concrete value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Text(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Binary(Vec<u8>),
    Json(JsonValue),
    Temporal(TemporalValue),
    Quantity(f64, String), // value + unit
    Unknown, // Explicit unknown
}

/// Theoretical value with supporting/contradicting evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoreticalValue {
    pub hypothesis: String,
    pub supporting_evidence: Vec<EntityId>,
    pub contradicting_evidence: Vec<EntityId>,
    pub probability: f64,
}

impl PropertyGraph {
    pub fn new() -> Self {
        PropertyGraph {
            properties: IndexMap::new(),
        }
    }
    
    /// Set a property
    pub fn set(&mut self, key: impl Into<String>, value: Property) {
        self.properties.insert(key.into(), value);
    }
    
    /// Get a property
    pub fn get(&self, key: &str) -> Option<&Property> {
        self.properties.get(key)
    }
    
    /// Get mutable property
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Property> {
        self.properties.get_mut(key)
    }
    
    /// Remove a property
    pub fn remove(&mut self, key: &str) -> Option<Property> {
        self.properties.shift_remove(key)
    }
    
    /// Check if property exists
    pub fn contains(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }
    
    /// Iterate over properties
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Property)> {
        self.properties.iter()
    }
    
    /// Number of properties
    pub fn len(&self) -> usize {
        self.properties.len()
    }
    
    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
    
    // Convenience methods for common operations
    
    /// Set a text value
    pub fn set_text(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.set(key, Property::Value(Value::Text(value.into())));
    }
    
    /// Set an integer value
    pub fn set_integer(&mut self, key: impl Into<String>, value: i64) {
        self.set(key, Property::Value(Value::Integer(value)));
    }
    
    /// Set a reference
    pub fn set_reference(&mut self, key: impl Into<String>, entity_id: EntityId) {
        self.set(key, Property::Reference(entity_id));
    }
    
    /// Set a nested entity
    pub fn set_entity(&mut self, key: impl Into<String>, entity: super::Entity) {
        self.set(key, Property::Entity(Box::new(entity)));
    }
    
    /// Get as text
    pub fn get_text(&self, key: &str) -> Option<&str> {
        match self.get(key)? {
            Property::Value(Value::Text(s)) => Some(s),
            _ => None,
        }
    }
    
    /// Get as integer
    pub fn get_integer(&self, key: &str) -> Option<i64> {
        match self.get(key)? {
            Property::Value(Value::Integer(i)) => Some(*i),
            _ => None,
        }
    }
    
    /// Merge another property graph into this one
    pub fn merge(&mut self, other: PropertyGraph) {
        for (key, value) in other.properties {
            self.properties.insert(key, value);
        }
    }
}

impl Default for PropertyGraph {
    fn default() -> Self {
        Self::new()
    }
}