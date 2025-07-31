//! StandardsRegistry entity - Central registry of available standards and methodologies
//! 
//! This entity manages discovery, versioning, and compatibility of research standards
//! like GPS, BCG, and custom institutional methodologies.

use crate::{
    validation::{Validatable, ValidationResult},
    EntityId, Error, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use super::{
    ConfigEntity, ConfigSource, StandardsRegistryId, MethodologyConfig,
};

/// Moved duplicate definitions here temporarily
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsConfigMetadata {
    pub id: StandardsConfigId,
    pub standards_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsConfig {
    pub metadata: StandardsConfigMetadata,
}

use super::StandardsConfigId;

/// Registry-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsRegistryMetadata {
    /// Unique ID for this registry instance
    pub id: StandardsRegistryId,
    
    /// Registry key (e.g., "main", "institutional")
    pub registry_key: String,
    
    /// Display name
    pub name: String,
    
    /// Registry description
    pub description: Option<String>,
    
    /// When this registry was created
    pub created_at: DateTime<Utc>,
    
    /// When this registry was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Last time the registry was synchronized
    pub last_synced: Option<DateTime<Utc>>,
}

/// Version specification for compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionSpec {
    /// Minimum compatible version
    pub min_version: String,
    
    /// Maximum compatible version
    pub max_version: Option<String>,
    
    /// Recommended version
    pub recommended_version: String,
}

/// Migration path between versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPath {
    /// From version
    pub from_version: String,
    
    /// To version
    pub to_version: String,
    
    /// Migration type
    pub migration_type: MigrationType,
    
    /// Migration notes
    pub notes: Option<String>,
}

/// Type of migration between versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationType {
    /// Automatic migration possible
    Automatic,
    /// Manual steps required
    Manual,
    /// Breaking changes, no direct migration
    Breaking,
}

/// Remote registry source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySource {
    /// Source name
    pub name: String,
    
    /// Source URL
    pub url: String,
    
    /// Authentication required
    pub requires_auth: bool,
    
    /// Source priority (lower is higher priority)
    pub priority: u32,
    
    /// Is this source active
    pub active: bool,
}

/// Standard configuration entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardEntry {
    /// Standard key (e.g., "gps-2021")
    pub standard_key: String,
    
    /// Display name
    pub name: String,
    
    /// Standard authority
    pub authority: String,
    
    /// Current version
    pub version: String,
    
    /// Available versions
    pub available_versions: Vec<String>,
    
    /// Is this the default for its type
    pub is_default: bool,
    
    /// Tags for categorization
    pub tags: HashSet<String>,
    
    /// Source where this was loaded from
    pub source: ConfigSource,
}

/// Central registry of all available standards and methodologies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardsRegistry {
    /// Registry metadata
    pub metadata: StandardsRegistryMetadata,
    
    /// Available standards indexed by key
    pub standards: HashMap<String, StandardEntry>,
    
    /// Loaded methodology configurations
    pub methodologies: HashMap<String, MethodologyConfig>,
    
    /// Version compatibility matrix
    pub version_compatibility: HashMap<String, VersionSpec>,
    
    /// Migration paths between versions
    pub migration_paths: Vec<MigrationPath>,
    
    /// Local source directories
    pub local_sources: Vec<PathBuf>,
    
    /// Remote registry sources
    pub remote_sources: Vec<RegistrySource>,
    
    /// Currently active standard keys
    pub active_standards: HashSet<String>,
    
    /// Default methodology key
    pub default_methodology: Option<String>,
    
    /// Categories for organization
    pub categories: HashMap<String, Vec<String>>,
    
    /// Custom metadata
    pub custom_metadata: HashMap<String, serde_json::Value>,
}

impl StandardsRegistry {
    /// Creates a new standards registry
    pub fn new(registry_key: String, name: String) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: StandardsRegistryMetadata {
                id: StandardsRegistryId::new(),
                registry_key,
                name,
                description: None,
                created_at: now,
                updated_at: now,
                last_synced: None,
            },
            standards: HashMap::new(),
            methodologies: HashMap::new(),
            version_compatibility: HashMap::new(),
            migration_paths: Vec::new(),
            local_sources: Vec::new(),
            remote_sources: Vec::new(),
            active_standards: HashSet::new(),
            default_methodology: None,
            categories: HashMap::new(),
            custom_metadata: HashMap::new(),
        }
    }
    
    /// Registers a new standard
    pub fn register_standard(&mut self, standard: StandardEntry) -> Result<()> {
        let key = standard.standard_key.clone();
        
        // Check for conflicts
        if let Some(existing) = self.standards.get(&key) {
            if existing.version != standard.version {
                return Err(Error::ConfigurationError(
                    format!("Standard '{}' already registered with version '{}'", key, existing.version)
                ));
            }
        }
        
        self.standards.insert(key.clone(), standard);
        self.update_metadata();
        Ok(())
    }
    
    /// Loads a methodology configuration
    pub fn load_methodology(&mut self, methodology: MethodologyConfig) -> Result<()> {
        let key = methodology.metadata.methodology_key.clone();
        
        // Register as a standard entry
        let standard_entry = StandardEntry {
            standard_key: key.clone(),
            name: methodology.metadata.name.clone(),
            authority: methodology.metadata.authority.clone(),
            version: methodology.metadata.version.clone(),
            available_versions: vec![methodology.metadata.version.clone()],
            is_default: false,
            tags: HashSet::from_iter(["methodology".to_string()]),
            source: methodology.metadata.source.clone(),
        };
        
        self.register_standard(standard_entry)?;
        self.methodologies.insert(key, methodology);
        self.update_metadata();
        Ok(())
    }
    
    /// Gets a methodology by key
    pub fn get_methodology(&self, key: &str) -> Option<&MethodologyConfig> {
        self.methodologies.get(key)
    }
    
    /// Sets the default methodology
    pub fn set_default_methodology(&mut self, key: &str) -> Result<()> {
        if !self.methodologies.contains_key(key) {
            return Err(Error::ConfigurationError(
                format!("Methodology '{}' not found in registry", key)
            ));
        }
        
        // Clear previous default
        if let Some(old_default) = &self.default_methodology {
            if let Some(standard) = self.standards.get_mut(old_default) {
                standard.is_default = false;
            }
        }
        
        // Set new default
        if let Some(standard) = self.standards.get_mut(key) {
            standard.is_default = true;
        }
        
        self.default_methodology = Some(key.to_string());
        self.update_metadata();
        Ok(())
    }
    
    /// Adds a local source directory
    pub fn add_local_source(&mut self, path: PathBuf) {
        if !self.local_sources.contains(&path) {
            self.local_sources.push(path);
            self.update_metadata();
        }
    }
    
    /// Adds a remote registry source
    pub fn add_remote_source(&mut self, source: RegistrySource) {
        self.remote_sources.push(source);
        self.remote_sources.sort_by_key(|s| s.priority);
        self.update_metadata();
    }
    
    /// Discovers standards from local sources
    pub async fn discover_local_standards(&mut self) -> Result<Vec<String>> {
        let discovered = Vec::new();
        
        // This would scan local directories for methodology files
        // For now, return empty as this is a placeholder
        
        self.metadata.last_synced = Some(Utc::now());
        self.update_metadata();
        Ok(discovered)
    }
    
    /// Activates a standard
    pub fn activate_standard(&mut self, key: &str) -> Result<()> {
        if !self.standards.contains_key(key) {
            return Err(Error::ConfigurationError(
                format!("Standard '{}' not found in registry", key)
            ));
        }
        
        self.active_standards.insert(key.to_string());
        self.update_metadata();
        Ok(())
    }
    
    /// Deactivates a standard
    pub fn deactivate_standard(&mut self, key: &str) {
        self.active_standards.remove(key);
        self.update_metadata();
    }
    
    /// Adds a version compatibility specification
    pub fn add_version_compatibility(&mut self, standard_key: String, spec: VersionSpec) {
        self.version_compatibility.insert(standard_key, spec);
        self.update_metadata();
    }
    
    /// Adds a migration path
    pub fn add_migration_path(&mut self, path: MigrationPath) {
        self.migration_paths.push(path);
        self.update_metadata();
    }
    
    /// Checks if two versions are compatible
    pub fn check_compatibility(&self, standard_key: &str, version: &str) -> bool {
        if let Some(spec) = self.version_compatibility.get(standard_key) {
            // Simple version comparison - in reality would use semver
            version >= spec.min_version.as_str() && 
                spec.max_version.as_ref().map_or(true, |max| version <= max.as_str())
        } else {
            // No compatibility info means we assume compatible
            true
        }
    }
    
    /// Finds migration path between versions
    pub fn find_migration_path(&self, from: &str, to: &str) -> Option<&MigrationPath> {
        self.migration_paths.iter()
            .find(|p| p.from_version == from && p.to_version == to)
    }
    
    /// Organizes standards by category
    pub fn add_to_category(&mut self, category: String, standard_key: String) {
        self.categories.entry(category)
            .or_insert_with(Vec::new)
            .push(standard_key);
        self.update_metadata();
    }
    
    /// Gets standards by category
    pub fn get_by_category(&self, category: &str) -> Vec<&StandardEntry> {
        self.categories.get(category)
            .map(|keys| {
                keys.iter()
                    .filter_map(|k| self.standards.get(k))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Gets standards by tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<&StandardEntry> {
        self.standards.values()
            .filter(|s| s.tags.contains(tag))
            .collect()
    }
    
    /// Creates a summary of the registry
    pub fn summary(&self) -> RegistrySummary {
        RegistrySummary {
            total_standards: self.standards.len(),
            total_methodologies: self.methodologies.len(),
            active_standards: self.active_standards.len(),
            categories: self.categories.keys().cloned().collect(),
            default_methodology: self.default_methodology.clone(),
            last_synced: self.metadata.last_synced,
        }
    }
    
    /// Updates the registry metadata timestamp
    fn update_metadata(&mut self) {
        self.metadata.updated_at = Utc::now();
    }
}

/// Summary of registry contents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrySummary {
    pub total_standards: usize,
    pub total_methodologies: usize,
    pub active_standards: usize,
    pub categories: Vec<String>,
    pub default_methodology: Option<String>,
    pub last_synced: Option<DateTime<Utc>>,
}

impl ConfigEntity for StandardsRegistry {
    fn id(&self) -> EntityId {
        self.metadata.id.clone()
    }
    
    fn config_type(&self) -> &'static str {
        "StandardsRegistry"
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn validate(&self) -> Result<()> {
        // Validate registry consistency
        for (key, _) in &self.methodologies {
            if !self.standards.contains_key(key) {
                return Err(Error::ConfigurationError(
                    format!("Methodology '{}' not registered as standard", key)
                ));
            }
        }
        
        // Validate default methodology exists
        if let Some(ref default) = self.default_methodology {
            if !self.methodologies.contains_key(default) {
                return Err(Error::ConfigurationError(
                    format!("Default methodology '{}' not found", default)
                ));
            }
        }
        
        // Validate migration paths reference existing versions
        for _path in &self.migration_paths {
            // In a real implementation, would check versions exist
        }
        
        Ok(())
    }
}

use async_trait::async_trait;

#[async_trait]
impl Validatable for StandardsRegistry {
    async fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Basic validation
        if self.metadata.registry_key.trim().is_empty() {
            result.add_error(
                "registry_key",
                "required",
                "Registry key cannot be empty"
            );
        }
        
        if self.metadata.name.trim().is_empty() {
            result.add_error(
                "name",
                "required",
                "Registry name cannot be empty"
            );
        }
        
        // Validate methodologies are registered
        for (key, _) in &self.methodologies {
            if !self.standards.contains_key(key) {
                result.add_error(
                    "methodologies",
                    "consistency",
                    format!("Methodology '{}' not registered as standard", key)
                );
            }
        }
        
        // Validate default methodology
        if let Some(ref default) = self.default_methodology {
            if !self.methodologies.contains_key(default) {
                result.add_error(
                    "default_methodology",
                    "invalid_reference",
                    format!("Default methodology '{}' not found", default)
                );
            }
        }
        
        // Validate active standards exist
        for active in &self.active_standards {
            if !self.standards.contains_key(active) {
                result.add_error(
                    "active_standards",
                    "invalid_reference",
                    format!("Active standard '{}' not found", active)
                );
            }
        }
        
        // Validate categories reference existing standards
        for (category, standards) in &self.categories {
            for standard_key in standards {
                if !self.standards.contains_key(standard_key) {
                    result.add_error(
                        "categories",
                        "invalid_reference",
                        format!("Category '{}' references non-existent standard '{}'", category, standard_key)
                    );
                }
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registry_creation() {
        let registry = StandardsRegistry::new(
            "main".to_string(),
            "Main Standards Registry".to_string()
        );
        
        assert_eq!(registry.metadata.registry_key, "main");
        assert_eq!(registry.metadata.name, "Main Standards Registry");
        assert!(registry.standards.is_empty());
        assert!(registry.methodologies.is_empty());
    }
    
    #[test]
    fn test_register_standard() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test".to_string());
        
        let standard = StandardEntry {
            standard_key: "gps-2021".to_string(),
            name: "GPS 2021".to_string(),
            authority: "BCG".to_string(),
            version: "2021.1".to_string(),
            available_versions: vec!["2021.1".to_string()],
            is_default: false,
            tags: HashSet::from_iter(["genealogy".to_string(), "proof".to_string()]),
            source: ConfigSource::Embedded { version: "1.0".to_string() },
        };
        
        assert!(registry.register_standard(standard).is_ok());
        assert!(registry.standards.contains_key("gps-2021"));
        
        // Test duplicate with same version (should succeed)
        let duplicate = StandardEntry {
            standard_key: "gps-2021".to_string(),
            name: "GPS 2021".to_string(),
            authority: "BCG".to_string(),
            version: "2021.1".to_string(),
            available_versions: vec!["2021.1".to_string()],
            is_default: false,
            tags: HashSet::new(),
            source: ConfigSource::Embedded { version: "1.0".to_string() },
        };
        
        assert!(registry.register_standard(duplicate).is_ok());
    }
    
    #[test]
    fn test_methodology_management() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test".to_string());
        
        let methodology = MethodologyConfig::create_gps_2021();
        
        // Load methodology
        assert!(registry.load_methodology(methodology).is_ok());
        assert!(registry.methodologies.contains_key("gps-2021"));
        assert!(registry.standards.contains_key("gps-2021"));
        
        // Get methodology
        assert!(registry.get_methodology("gps-2021").is_some());
        assert!(registry.get_methodology("non-existent").is_none());
        
        // Set as default
        assert!(registry.set_default_methodology("gps-2021").is_ok());
        assert_eq!(registry.default_methodology, Some("gps-2021".to_string()));
        
        // Verify default flag
        let standard = registry.standards.get("gps-2021").unwrap();
        assert!(standard.is_default);
    }
    
    #[test]
    fn test_standard_activation() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test".to_string());
        
        // Register a standard
        let standard = StandardEntry {
            standard_key: "bcg-2023".to_string(),
            name: "BCG Standards 2023".to_string(),
            authority: "BCG".to_string(),
            version: "2023.1".to_string(),
            available_versions: vec!["2023.1".to_string()],
            is_default: false,
            tags: HashSet::new(),
            source: ConfigSource::Embedded { version: "1.0".to_string() },
        };
        
        registry.register_standard(standard).unwrap();
        
        // Activate standard
        assert!(registry.activate_standard("bcg-2023").is_ok());
        assert!(registry.active_standards.contains("bcg-2023"));
        
        // Try to activate non-existent
        assert!(registry.activate_standard("non-existent").is_err());
        
        // Deactivate
        registry.deactivate_standard("bcg-2023");
        assert!(!registry.active_standards.contains("bcg-2023"));
    }
    
    #[test]
    fn test_categories() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test".to_string());
        
        // Add standards
        for i in 1..=3 {
            let standard = StandardEntry {
                standard_key: format!("standard-{}", i),
                name: format!("Standard {}", i),
                authority: "Test".to_string(),
                version: "1.0".to_string(),
                available_versions: vec!["1.0".to_string()],
                is_default: false,
                tags: HashSet::new(),
                source: ConfigSource::Embedded { version: "1.0".to_string() },
            };
            registry.register_standard(standard).unwrap();
        }
        
        // Categorize
        registry.add_to_category("genealogy".to_string(), "standard-1".to_string());
        registry.add_to_category("genealogy".to_string(), "standard-2".to_string());
        registry.add_to_category("research".to_string(), "standard-2".to_string());
        registry.add_to_category("research".to_string(), "standard-3".to_string());
        
        // Get by category
        let genealogy = registry.get_by_category("genealogy");
        assert_eq!(genealogy.len(), 2);
        
        let research = registry.get_by_category("research");
        assert_eq!(research.len(), 2);
    }
    
    #[test]
    fn test_version_compatibility() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test".to_string());
        
        // Add compatibility spec
        let spec = VersionSpec {
            min_version: "2.0.0".to_string(),
            max_version: Some("3.0.0".to_string()),
            recommended_version: "2.5.0".to_string(),
        };
        
        registry.add_version_compatibility("my-standard".to_string(), spec);
        
        // Check compatibility
        assert!(registry.check_compatibility("my-standard", "2.5.0"));
        assert!(registry.check_compatibility("my-standard", "2.0.0"));
        assert!(registry.check_compatibility("my-standard", "3.0.0"));
        assert!(!registry.check_compatibility("my-standard", "1.9.0"));
        assert!(!registry.check_compatibility("my-standard", "3.1.0"));
        
        // Unknown standard assumes compatible
        assert!(registry.check_compatibility("unknown", "1.0.0"));
    }
    
    #[test]
    fn test_migration_paths() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test".to_string());
        
        // Add migration paths
        registry.add_migration_path(MigrationPath {
            from_version: "1.0".to_string(),
            to_version: "2.0".to_string(),
            migration_type: MigrationType::Automatic,
            notes: Some("Automatic upgrade available".to_string()),
        });
        
        registry.add_migration_path(MigrationPath {
            from_version: "2.0".to_string(),
            to_version: "3.0".to_string(),
            migration_type: MigrationType::Manual,
            notes: Some("Manual steps required".to_string()),
        });
        
        // Find paths
        let path = registry.find_migration_path("1.0", "2.0");
        assert!(path.is_some());
        assert_eq!(path.unwrap().migration_type, MigrationType::Automatic);
        
        let path = registry.find_migration_path("2.0", "3.0");
        assert!(path.is_some());
        assert_eq!(path.unwrap().migration_type, MigrationType::Manual);
        
        let path = registry.find_migration_path("1.0", "3.0");
        assert!(path.is_none());
    }
    
    #[test]
    fn test_tags() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test".to_string());
        
        // Add standards with tags
        let standard1 = StandardEntry {
            standard_key: "standard-1".to_string(),
            name: "Standard 1".to_string(),
            authority: "Test".to_string(),
            version: "1.0".to_string(),
            available_versions: vec!["1.0".to_string()],
            is_default: false,
            tags: HashSet::from_iter(["genealogy".to_string(), "proof".to_string()]),
            source: ConfigSource::Embedded { version: "1.0".to_string() },
        };
        
        let standard2 = StandardEntry {
            standard_key: "standard-2".to_string(),
            name: "Standard 2".to_string(),
            authority: "Test".to_string(),
            version: "1.0".to_string(),
            available_versions: vec!["1.0".to_string()],
            is_default: false,
            tags: HashSet::from_iter(["research".to_string(), "proof".to_string()]),
            source: ConfigSource::Embedded { version: "1.0".to_string() },
        };
        
        registry.register_standard(standard1).unwrap();
        registry.register_standard(standard2).unwrap();
        
        // Get by tag
        let proof_standards = registry.get_by_tag("proof");
        assert_eq!(proof_standards.len(), 2);
        
        let genealogy_standards = registry.get_by_tag("genealogy");
        assert_eq!(genealogy_standards.len(), 1);
    }
    
    #[tokio::test]
    async fn test_registry_validation() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test Registry".to_string());
        
        // Valid registry
        let result = <StandardsRegistry as Validatable>::validate(&registry).await;
        assert!(result.is_valid());
        
        // Empty registry key
        registry.metadata.registry_key = "".to_string();
        let result = <StandardsRegistry as Validatable>::validate(&registry).await;
        assert!(!result.is_valid());
        
        // Fix key, add invalid default methodology
        registry.metadata.registry_key = "test".to_string();
        registry.default_methodology = Some("non-existent".to_string());
        let result = <StandardsRegistry as Validatable>::validate(&registry).await;
        assert!(!result.is_valid());
        
        // Fix by loading the methodology
        let methodology = MethodologyConfig::create_gps_2021();
        registry.load_methodology(methodology).unwrap();
        registry.default_methodology = Some("gps-2021".to_string());
        let result = <StandardsRegistry as Validatable>::validate(&registry).await;
        assert!(result.is_valid());
    }
    
    #[test]
    fn test_registry_summary() {
        let mut registry = StandardsRegistry::new("test".to_string(), "Test".to_string());
        
        // Add some content
        registry.load_methodology(MethodologyConfig::create_gps_2021()).unwrap();
        registry.activate_standard("gps-2021").unwrap();
        registry.add_to_category("genealogy".to_string(), "gps-2021".to_string());
        registry.set_default_methodology("gps-2021").unwrap();
        
        let summary = registry.summary();
        assert_eq!(summary.total_standards, 1);
        assert_eq!(summary.total_methodologies, 1);
        assert_eq!(summary.active_standards, 1);
        assert_eq!(summary.categories.len(), 1);
        assert_eq!(summary.default_methodology, Some("gps-2021".to_string()));
    }
}