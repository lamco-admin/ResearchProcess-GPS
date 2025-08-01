//! Module registry for managing loaded modules

use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::{ModuleMetadata, ModuleManifest, Result, ModuleError};

/// Registry for managing modules
#[derive(Clone)]
pub struct ModuleRegistry {
    /// Loaded modules by ID
    modules: Arc<DashMap<Uuid, ModuleEntry>>,
    
    /// Module name to ID mapping
    name_to_id: Arc<DashMap<String, Uuid>>,
}

/// Entry in the module registry
pub struct ModuleEntry {
    pub metadata: ModuleMetadata,
    pub manifest: ModuleManifest,
    pub path: std::path::PathBuf,
}

impl ModuleRegistry {
    /// Create a new module registry
    pub fn new() -> Self {
        Self {
            modules: Arc::new(DashMap::new()),
            name_to_id: Arc::new(DashMap::new()),
        }
    }
    
    /// Register a module
    pub fn register(&self, id: Uuid, entry: ModuleEntry) -> Result<()> {
        // Check for duplicate names
        if self.name_to_id.contains_key(&entry.metadata.name) {
            return Err(ModuleError::LoadError(
                format!("Module with name '{}' already registered", entry.metadata.name)
            ));
        }
        
        let name = entry.metadata.name.clone();
        self.modules.insert(id, entry);
        self.name_to_id.insert(name, id);
        
        Ok(())
    }
    
    /// Unregister a module
    pub fn unregister(&self, id: Uuid) -> Option<ModuleEntry> {
        if let Some((_, entry)) = self.modules.remove(&id) {
            self.name_to_id.remove(&entry.metadata.name);
            Some(entry)
        } else {
            None
        }
    }
    
    /// Get module by ID
    pub fn get(&self, id: &Uuid) -> Option<dashmap::mapref::one::Ref<Uuid, ModuleEntry>> {
        self.modules.get(id)
    }
    
    /// Get module by name
    pub fn get_by_name(&self, name: &str) -> Option<dashmap::mapref::one::Ref<Uuid, ModuleEntry>> {
        self.name_to_id.get(name)
            .and_then(|id| self.modules.get(&*id))
    }
    
    /// List all modules
    pub fn list(&self) -> Vec<ModuleMetadata> {
        self.modules.iter()
            .map(|entry| entry.metadata.clone())
            .collect()
    }
    
    /// Get total module count
    pub fn count(&self) -> usize {
        self.modules.len()
    }
    
    /// Check if a module is registered
    pub fn contains(&self, id: &Uuid) -> bool {
        self.modules.contains_key(id)
    }
    
    /// Clear all modules
    pub fn clear(&self) {
        self.name_to_id.clear();
        self.modules.clear();
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}