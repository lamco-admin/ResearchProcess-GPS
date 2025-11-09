//! Adapter framework for translating between schemas and external formats
//!
//! Adapters allow importing/exporting data from different genealogy systems
//! (GEDCOM, GRAMPS, etc.) into the universal meta-model.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use rp_meta_core::{Entity, Relationship};

use crate::{Schema, Error, Result};

/// Adapter trait for format conversion
#[async_trait]
pub trait Adapter: Send + Sync {
    /// Adapter identifier (e.g., "gedcom-7", "gramps-xml")
    fn id(&self) -> &str;

    /// Adapter name
    fn name(&self) -> &str;

    /// Description of what this adapter does
    fn description(&self) -> &str;

    /// Schema this adapter works with
    fn schema(&self) -> &Schema;

    /// Import data from external format into entities and relationships
    async fn import(&self, data: &[u8]) -> Result<ImportResult>;

    /// Export entities and relationships to external format
    async fn export(&self, entities: &[Entity], relationships: &[Relationship]) -> Result<Vec<u8>>;

    /// Validate that the external data is compatible with this adapter
    async fn validate_import(&self, data: &[u8]) -> Result<ValidationReport>;
}

/// Result of an import operation
#[derive(Debug, Clone)]
pub struct ImportResult {
    /// Imported entities
    pub entities: Vec<Entity>,

    /// Imported relationships
    pub relationships: Vec<Relationship>,

    /// Import statistics
    pub stats: ImportStats,

    /// Warnings encountered during import
    pub warnings: Vec<String>,
}

/// Import statistics
#[derive(Debug, Clone, Default)]
pub struct ImportStats {
    /// Number of entities imported
    pub entity_count: usize,

    /// Number of relationships imported
    pub relationship_count: usize,

    /// Entity counts by type
    pub entity_types: HashMap<String, usize>,

    /// Relationship counts by type
    pub relationship_types: HashMap<String, usize>,

    /// Number of warnings
    pub warning_count: usize,
}

/// Validation report for import data
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Is the data valid?
    pub valid: bool,

    /// Errors found
    pub errors: Vec<String>,

    /// Warnings found
    pub warnings: Vec<String>,

    /// Detected format information
    pub detected_format: Option<String>,

    /// Detected version
    pub detected_version: Option<String>,
}

/// Registry for managing adapters
#[derive(Clone)]
pub struct AdapterRegistry {
    adapters: Arc<RwLock<HashMap<String, Arc<dyn Adapter>>>>,
}

impl AdapterRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an adapter
    pub fn register(&self, adapter: Arc<dyn Adapter>) -> Result<()> {
        let mut adapters = self.adapters.write()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        let id = adapter.id().to_string();
        adapters.insert(id, adapter);

        Ok(())
    }

    /// Get an adapter by ID
    pub fn get(&self, adapter_id: &str) -> Result<Arc<dyn Adapter>> {
        let adapters = self.adapters.read()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        adapters.get(adapter_id)
            .cloned()
            .ok_or_else(|| Error::AdapterNotFound {
                adapter_id: adapter_id.to_string(),
            })
    }

    /// List all registered adapter IDs
    pub fn list_adapters(&self) -> Result<Vec<String>> {
        let adapters = self.adapters.read()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        Ok(adapters.keys().cloned().collect())
    }

    /// Remove an adapter
    pub fn unregister(&self, adapter_id: &str) -> Result<()> {
        let mut adapters = self.adapters.write()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        adapters.remove(adapter_id)
            .ok_or_else(|| Error::AdapterNotFound {
                adapter_id: adapter_id.to_string(),
            })?;

        Ok(())
    }

    /// Auto-detect which adapter to use for given data
    pub async fn detect_adapter(&self, data: &[u8]) -> Result<Option<Arc<dyn Adapter>>> {
        let adapters = self.adapters.read()
            .map_err(|e| Error::Generic(format!("Lock poisoned: {}", e)))?;

        for adapter in adapters.values() {
            if let Ok(report) = adapter.validate_import(data).await {
                if report.valid {
                    return Ok(Some(adapter.clone()));
                }
            }
        }

        Ok(None)
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock adapter for testing
    struct MockAdapter {
        id: String,
        schema: Schema,
    }

    #[async_trait]
    impl Adapter for MockAdapter {
        fn id(&self) -> &str {
            &self.id
        }

        fn name(&self) -> &str {
            "Mock Adapter"
        }

        fn description(&self) -> &str {
            "A mock adapter for testing"
        }

        fn schema(&self) -> &Schema {
            &self.schema
        }

        async fn import(&self, _data: &[u8]) -> Result<ImportResult> {
            Ok(ImportResult {
                entities: Vec::new(),
                relationships: Vec::new(),
                stats: ImportStats::default(),
                warnings: Vec::new(),
            })
        }

        async fn export(&self, _entities: &[Entity], _relationships: &[Relationship]) -> Result<Vec<u8>> {
            Ok(Vec::new())
        }

        async fn validate_import(&self, _data: &[u8]) -> Result<ValidationReport> {
            Ok(ValidationReport {
                valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
                detected_format: Some("mock".to_string()),
                detected_version: Some("1.0".to_string()),
            })
        }
    }

    #[test]
    fn test_adapter_registry() {
        let registry = AdapterRegistry::new();

        let adapter = Arc::new(MockAdapter {
            id: "mock".to_string(),
            schema: Schema::new("mock", "1.0", "Mock"),
        });

        assert!(registry.register(adapter).is_ok());
        assert!(registry.get("mock").is_ok());
        assert!(registry.get("nonexistent").is_err());
    }
}
