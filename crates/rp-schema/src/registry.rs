//! Schema registry for managing multiple schemas

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{Schema, Error, Result};

/// Unique identifier for a schema
pub type SchemaId = String;

/// Registry for managing multiple schemas
#[derive(Clone)]
pub struct SchemaRegistry {
    schemas: Arc<RwLock<HashMap<SchemaId, Schema>>>,
}

impl SchemaRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a schema
    pub fn register(&self, schema: Schema) -> Result<()> {
        schema.validate()?;

        let mut schemas = self.schemas.write()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        let id = schema.id.clone();
        schemas.insert(id, schema);

        Ok(())
    }

    /// Get a schema by ID
    pub fn get(&self, schema_id: &str) -> Result<Schema> {
        let schemas = self.schemas.read()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        schemas.get(schema_id)
            .cloned()
            .ok_or_else(|| Error::schema_not_found(schema_id))
    }

    /// Check if a schema exists
    pub fn contains(&self, schema_id: &str) -> bool {
        self.schemas.read()
            .map(|schemas| schemas.contains_key(schema_id))
            .unwrap_or(false)
    }

    /// List all registered schema IDs
    pub fn list_schemas(&self) -> Result<Vec<SchemaId>> {
        let schemas = self.schemas.read()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        Ok(schemas.keys().cloned().collect())
    }

    /// Remove a schema
    pub fn unregister(&self, schema_id: &str) -> Result<()> {
        let mut schemas = self.schemas.write()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        schemas.remove(schema_id)
            .ok_or_else(|| Error::schema_not_found(schema_id))?;

        Ok(())
    }

    /// Load and register a schema from YAML
    pub fn register_from_yaml(&self, yaml: &str) -> Result<SchemaId> {
        let schema = Schema::from_yaml(yaml)?;
        let id = schema.id.clone();
        self.register(schema)?;
        Ok(id)
    }

    /// Load and register a schema from JSON
    pub fn register_from_json(&self, json: &str) -> Result<SchemaId> {
        let schema = Schema::from_json(json)?;
        let id = schema.id.clone();
        self.register(schema)?;
        Ok(id)
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_registry() {
        let registry = SchemaRegistry::new();

        let schema = Schema::new("test", "1.0", "Test Schema");
        assert!(registry.register(schema).is_ok());

        assert!(registry.contains("test"));
        assert!(!registry.contains("nonexistent"));

        assert!(registry.get("test").is_ok());
        assert!(registry.get("nonexistent").is_err());
    }

    #[test]
    fn test_list_schemas() {
        let registry = SchemaRegistry::new();

        registry.register(Schema::new("schema1", "1.0", "Schema 1")).unwrap();
        registry.register(Schema::new("schema2", "1.0", "Schema 2")).unwrap();

        let schemas = registry.list_schemas().unwrap();
        assert_eq!(schemas.len(), 2);
        assert!(schemas.contains(&"schema1".to_string()));
        assert!(schemas.contains(&"schema2".to_string()));
    }
}
