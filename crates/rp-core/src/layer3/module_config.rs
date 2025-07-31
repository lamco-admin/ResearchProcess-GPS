//! ModuleConfig entity - Configuration for pluggable modules
//! 
//! Modules extend ResearchProcess-GPS functionality without core changes.
//! Examples: DNA analysis, map integration, custom report generators.

use crate::{
    validation::{Validatable, ValidationResult, ValidationIssue, ValidationSeverity},
    EntityId, Error, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use validator::Validate;

use super::{
    ConfigEntity, ConfigSource, ModuleConfigId, ModuleType,
};

/// Module-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    /// Unique ID for this module configuration
    pub id: ModuleConfigId,
    
    /// Module identifier (e.g., "com.example.dna-analysis")
    pub module_key: String,
    
    /// Display name
    pub name: String,
    
    /// Module version
    pub version: String,
    
    /// Module author/vendor
    pub author: String,
    
    /// When this config was created
    pub created_at: DateTime<Utc>,
    
    /// When this config was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Source of this configuration
    pub source: ConfigSource,
}

/// Module capability declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Capability name (e.g., "evidence.analyze.dna")
    pub name: String,
    
    /// Version of the capability
    pub version: String,
    
    /// Description of what this provides
    pub description: String,
}

/// Module dependency declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// Required module key
    pub module_key: String,
    
    /// Version requirement (e.g., ">=1.0.0")
    pub version_spec: String,
    
    /// Whether this is optional
    pub optional: bool,
}

/// Settings schema using JSON Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsSchema {
    /// JSON Schema for module settings
    pub schema: serde_json::Value,
    
    /// UI hints for settings editor
    pub ui_schema: Option<serde_json::Value>,
}

/// Module resource reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRef {
    /// Resource type (e.g., "template", "validator", "script")
    pub resource_type: String,
    
    /// Resource identifier
    pub resource_id: String,
    
    /// Resource location/path
    pub location: String,
}

/// Module lifecycle hooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHooks {
    /// Called when module is loaded
    pub on_load: Option<String>,
    
    /// Called when module is enabled
    pub on_enable: Option<String>,
    
    /// Called when module is disabled
    pub on_disable: Option<String>,
    
    /// Called when module is unloaded
    pub on_unload: Option<String>,
}

/// Configuration for a pluggable module
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ModuleConfig {
    /// Module-specific metadata
    pub metadata: ModuleMetadata,
    
    /// Module type category
    pub module_type: ModuleType,
    
    /// Brief description
    pub description: String,
    
    /// Detailed documentation URL
    pub documentation_url: Option<String>,
    
    /// Module homepage
    pub homepage: Option<String>,
    
    /// License identifier (e.g., "MIT", "Apache-2.0")
    pub license: String,
    
    /// What this module provides
    pub provides: Vec<Capability>,
    
    /// What this module requires
    pub requires: Vec<Dependency>,
    
    /// Conflicts with these modules
    pub conflicts_with: Vec<String>,
    
    /// Settings schema for configuration
    pub settings_schema: SettingsSchema,
    
    /// Default settings values
    pub default_settings: HashMap<String, serde_json::Value>,
    
    /// Resources provided by this module
    pub resources: Vec<ResourceRef>,
    
    /// Supported file formats for import/export
    pub supported_formats: HashMap<String, Vec<String>>,
    
    /// Lifecycle hooks
    pub lifecycle_hooks: LifecycleHooks,
    
    /// Security permissions required
    pub required_permissions: HashSet<String>,
    
    /// Whether to auto-load on workspace activation
    pub auto_load: bool,
    
    /// Load priority (lower numbers load first)
    pub load_priority: i32,
    
    /// Feature flags this module supports
    pub feature_flags: HashSet<String>,
    
    /// Custom metadata for extensions
    pub custom_metadata: HashMap<String, serde_json::Value>,
}

impl ModuleConfig {
    /// Creates a new module configuration
    pub fn new(
        module_key: String,
        name: String,
        version: String,
        author: String,
        module_type: ModuleType,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: ModuleMetadata {
                id: ModuleConfigId::new(),
                module_key,
                name,
                version,
                author,
                created_at: now,
                updated_at: now,
                source: ConfigSource::UserDefined { created_at: now },
            },
            module_type,
            description: String::new(),
            documentation_url: None,
            homepage: None,
            license: "Unknown".to_string(),
            provides: Vec::new(),
            requires: Vec::new(),
            conflicts_with: Vec::new(),
            settings_schema: SettingsSchema {
                schema: serde_json::json!({}),
                ui_schema: None,
            },
            default_settings: HashMap::new(),
            resources: Vec::new(),
            supported_formats: HashMap::new(),
            lifecycle_hooks: LifecycleHooks {
                on_load: None,
                on_enable: None,
                on_disable: None,
                on_unload: None,
            },
            required_permissions: HashSet::new(),
            auto_load: false,
            load_priority: 100,
            feature_flags: HashSet::new(),
            custom_metadata: HashMap::new(),
        }
    }
    
    /// Creates an example DNA analysis module configuration
    pub fn create_dna_analysis_example() -> Self {
        let mut config = Self::new(
            "com.example.dna-analysis".to_string(),
            "DNA Analysis Module".to_string(),
            "1.0.0".to_string(),
            "Example Corp".to_string(),
            ModuleType::Analysis,
        );
        
        config.description = "Analyzes DNA test results and integrates with genealogical research".to_string();
        config.documentation_url = Some("https://example.com/docs/dna-module".to_string());
        config.license = "MIT".to_string();
        
        // Capabilities
        config.provides.push(Capability {
            name: "evidence.analyze.dna".to_string(),
            version: "1.0".to_string(),
            description: "Analyze DNA test results from major providers".to_string(),
        });
        
        config.provides.push(Capability {
            name: "evidence.import.gedmatch".to_string(),
            version: "1.0".to_string(),
            description: "Import GEDmatch analysis results".to_string(),
        });
        
        // Settings schema
        config.settings_schema.schema = serde_json::json!({
            "type": "object",
            "properties": {
                "defaultCmThreshold": {
                    "type": "number",
                    "title": "Default cM Threshold",
                    "description": "Minimum centimorgans for matches",
                    "default": 7.0,
                    "minimum": 0
                },
                "enableChromosomeBrowser": {
                    "type": "boolean",
                    "title": "Enable Chromosome Browser",
                    "default": true
                },
                "providers": {
                    "type": "array",
                    "title": "Supported DNA Providers",
                    "items": {
                        "type": "string",
                        "enum": ["AncestryDNA", "23andMe", "MyHeritage", "FamilyTreeDNA"]
                    }
                }
            }
        });
        
        // Default settings
        config.default_settings.insert(
            "defaultCmThreshold".to_string(),
            serde_json::json!(7.0)
        );
        config.default_settings.insert(
            "enableChromosomeBrowser".to_string(),
            serde_json::json!(true)
        );
        
        // Resources
        config.resources.push(ResourceRef {
            resource_type: "template".to_string(),
            resource_id: "dna-match-report".to_string(),
            location: "templates/dna-match-report.md".to_string(),
        });
        
        config.resources.push(ResourceRef {
            resource_type: "validator".to_string(),
            resource_id: "dna-evidence-validator".to_string(),
            location: "validators/dna-evidence.json".to_string(),
        });
        
        // Supported formats
        config.supported_formats.insert(
            "import".to_string(),
            vec!["csv".to_string(), "txt".to_string(), "json".to_string()]
        );
        config.supported_formats.insert(
            "export".to_string(),
            vec!["pdf".to_string(), "html".to_string(), "xlsx".to_string()]
        );
        
        // Permissions
        config.required_permissions.insert("evidence.create".to_string());
        config.required_permissions.insert("evidence.modify".to_string());
        config.required_permissions.insert("report.generate".to_string());
        
        // Feature flags
        config.feature_flags.insert("chromosome-browser".to_string());
        config.feature_flags.insert("triangulation-tools".to_string());
        config.feature_flags.insert("haplogroup-analysis".to_string());
        
        config
    }
    
    /// Checks if this module is compatible with another
    pub fn is_compatible_with(&self, other_key: &str) -> bool {
        !self.conflicts_with.contains(&other_key.to_string())
    }
    
    /// Gets all dependencies (required and optional)
    pub fn all_dependencies(&self) -> Vec<&Dependency> {
        self.requires.iter().collect()
    }
    
    /// Gets only required dependencies
    pub fn required_dependencies(&self) -> Vec<&Dependency> {
        self.requires.iter()
            .filter(|d| !d.optional)
            .collect()
    }
    
    /// Validates settings against schema
    pub fn validate_settings(&self, settings: &HashMap<String, serde_json::Value>) -> Result<()> {
        // In a real implementation, this would use a JSON Schema validator
        // For now, just check that required settings are present
        if let Some(schema) = self.settings_schema.schema.as_object() {
            if let Some(_properties) = schema.get("properties").and_then(|p| p.as_object()) {
                if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
                    for req in required {
                        if let Some(key) = req.as_str() {
                            if !settings.contains_key(key) && !self.default_settings.contains_key(key) {
                                return Err(Error::ConfigurationError(
                                    format!("Required setting '{}' is missing", key)
                                ));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Merges user settings with defaults
    pub fn merge_settings(&self, user_settings: HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let mut merged = self.default_settings.clone();
        merged.extend(user_settings);
        merged
    }
}

impl ConfigEntity for ModuleConfig {
    fn id(&self) -> EntityId {
        self.metadata.id
    }
    
    fn config_type(&self) -> &'static str {
        "ModuleConfig"
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn validate(&self) -> Result<()> {
        // Basic validation
        if self.metadata.module_key.is_empty() {
            return Err(Error::ConfigurationError("Module key cannot be empty".to_string()));
        }
        
        if self.metadata.name.is_empty() {
            return Err(Error::ConfigurationError("Module name cannot be empty".to_string()));
        }
        
        if self.metadata.version.is_empty() {
            return Err(Error::ConfigurationError("Module version cannot be empty".to_string()));
        }
        
        // Validate schema is valid JSON Schema
        if !self.settings_schema.schema.is_object() {
            return Err(Error::ConfigurationError("Settings schema must be a JSON object".to_string()));
        }
        
        // Validate default settings against schema
        self.validate_settings(&self.default_settings)?;
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl Validatable for ModuleConfig {
    async fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        
        // Module key validation
        if self.metadata.module_key.is_empty() {
            errors.push(ValidationIssue {
                field: "module_key".to_string(),
                code: "required_field".to_string(),
                message: "Module key is required".to_string(),
                severity: ValidationSeverity::Error,
            });
        } else if !self.metadata.module_key.contains('.') {
            errors.push(ValidationIssue {
                field: "module_key".to_string(),
                code: "invalid_format".to_string(),
                message: "Module key should be in reverse domain format (e.g., com.example.module)".to_string(),
                severity: ValidationSeverity::Error,
            });
        }
        
        // Name validation
        if self.metadata.name.is_empty() {
            errors.push(ValidationIssue {
                field: "name".to_string(),
                code: "required_field".to_string(),
                message: "Name is required".to_string(),
                severity: ValidationSeverity::Error,
            });
        }
        
        // Version validation
        if self.metadata.version.is_empty() {
            errors.push(ValidationIssue {
                field: "version".to_string(),
                code: "required_field".to_string(),
                message: "Version is required".to_string(),
                severity: ValidationSeverity::Error,
            });
        }
        
        // License validation
        if self.license.is_empty() || self.license == "Unknown" {
            errors.push(ValidationIssue {
                field: "license".to_string(),
                code: "required_field".to_string(),
                message: "License is required".to_string(),
                severity: ValidationSeverity::Error,
            });
        }
        
        // Validate no self-dependency
        for dep in &self.requires {
            if dep.module_key == self.metadata.module_key {
                errors.push(ValidationIssue {
                    field: "requires".to_string(),
                    code: "invalid_reference".to_string(),
                    message: format!("Module cannot depend on itself: {}", dep.module_key),
                    severity: ValidationSeverity::Error,
                });
            }
        }
        
        // Validate no self-conflict
        if self.conflicts_with.contains(&self.metadata.module_key) {
            errors.push(ValidationIssue {
                field: "conflicts_with".to_string(),
                code: "invalid_reference".to_string(),
                message: format!("Module cannot conflict with itself: {}", self.metadata.module_key),
                severity: ValidationSeverity::Error,
            });
        }
        
        // Validate resources
        for resource in &self.resources {
            if resource.resource_id.is_empty() {
                errors.push(ValidationIssue {
                    field: format!("resource[{}].resource_id", resource.resource_type),
                    code: "required_field".to_string(),
                    message: "Resource ID is required".to_string(),
                    severity: ValidationSeverity::Error,
                });
            }
            if resource.location.is_empty() {
                errors.push(ValidationIssue {
                    field: format!("resource[{}].location", resource.resource_type),
                    code: "required_field".to_string(),
                    message: "Resource location is required".to_string(),
                    severity: ValidationSeverity::Error,
                });
            }
        }
        
        // Validate capabilities
        for cap in &self.provides {
            if cap.name.is_empty() {
                errors.push(ValidationIssue {
                    field: "capability.name".to_string(),
                    code: "required_field".to_string(),
                    message: "Capability name is required".to_string(),
                    severity: ValidationSeverity::Error,
                });
            }
        }
        
        ValidationResult { issues: errors }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_config_creation() {
        let config = ModuleConfig::new(
            "com.test.module".to_string(),
            "Test Module".to_string(),
            "1.0.0".to_string(),
            "Test Author".to_string(),
            ModuleType::Analysis,
        );
        
        assert_eq!(config.metadata.module_key, "com.test.module");
        assert_eq!(config.metadata.name, "Test Module");
        assert_eq!(config.module_type, ModuleType::Analysis);
        assert!(!config.auto_load);
        assert_eq!(config.load_priority, 100);
    }
    
    #[test]
    fn test_dna_analysis_example() {
        let config = ModuleConfig::create_dna_analysis_example();
        
        assert_eq!(config.metadata.module_key, "com.example.dna-analysis");
        assert_eq!(config.module_type, ModuleType::Analysis);
        assert_eq!(config.provides.len(), 2);
        assert_eq!(config.resources.len(), 2);
        assert!(config.required_permissions.contains("evidence.create"));
        assert!(config.feature_flags.contains("chromosome-browser"));
    }
    
    #[test]
    fn test_compatibility_check() {
        let config = ModuleConfig::new(
            "com.test.module".to_string(),
            "Test Module".to_string(),
            "1.0.0".to_string(),
            "Test Author".to_string(),
            ModuleType::Analysis,
        );
        
        assert!(config.is_compatible_with("com.other.module"));
        
        let mut config_with_conflicts = config.clone();
        config_with_conflicts.conflicts_with.push("com.conflicting.module".to_string());
        
        assert!(!config_with_conflicts.is_compatible_with("com.conflicting.module"));
        assert!(config_with_conflicts.is_compatible_with("com.other.module"));
    }
    
    #[test]
    fn test_dependency_filtering() {
        let mut config = ModuleConfig::new(
            "com.test.module".to_string(),
            "Test Module".to_string(),
            "1.0.0".to_string(),
            "Test Author".to_string(),
            ModuleType::Analysis,
        );
        
        config.requires.push(Dependency {
            module_key: "com.required.module".to_string(),
            version_spec: ">=1.0.0".to_string(),
            optional: false,
        });
        
        config.requires.push(Dependency {
            module_key: "com.optional.module".to_string(),
            version_spec: ">=2.0.0".to_string(),
            optional: true,
        });
        
        assert_eq!(config.all_dependencies().len(), 2);
        assert_eq!(config.required_dependencies().len(), 1);
        assert_eq!(config.required_dependencies()[0].module_key, "com.required.module");
    }
    
    #[test]
    fn test_settings_merge() {
        let mut config = ModuleConfig::new(
            "com.test.module".to_string(),
            "Test Module".to_string(),
            "1.0.0".to_string(),
            "Test Author".to_string(),
            ModuleType::Analysis,
        );
        
        config.default_settings.insert(
            "setting1".to_string(),
            serde_json::json!("default1")
        );
        config.default_settings.insert(
            "setting2".to_string(),
            serde_json::json!("default2")
        );
        
        let mut user_settings = HashMap::new();
        user_settings.insert(
            "setting2".to_string(),
            serde_json::json!("user2")
        );
        user_settings.insert(
            "setting3".to_string(),
            serde_json::json!("user3")
        );
        
        let merged = config.merge_settings(user_settings);
        
        assert_eq!(merged.get("setting1"), Some(&serde_json::json!("default1")));
        assert_eq!(merged.get("setting2"), Some(&serde_json::json!("user2")));
        assert_eq!(merged.get("setting3"), Some(&serde_json::json!("user3")));
    }
    
    #[tokio::test]
    async fn test_module_validation() {
        let mut config = ModuleConfig::new(
            "com.test.module".to_string(),
            "Test Module".to_string(),
            "1.0.0".to_string(),
            "Test Author".to_string(),
            ModuleType::Analysis,
        );
        config.license = "MIT".to_string();
        
        let result = <ModuleConfig as Validatable>::validate(&config).await;
        assert!(result.is_valid());
        
        // Test invalid module key
        let mut invalid_config = config.clone();
        invalid_config.metadata.module_key = "invalid-key".to_string();
        
        let result = <ModuleConfig as Validatable>::validate(&invalid_config).await;
        assert!(!result.is_valid());
        assert!(result.issues.iter().any(|e| e.field == "module_key" && e.code == "invalid_format"));
        
        // Test self-dependency
        let mut self_dep_config = config.clone();
        self_dep_config.requires.push(Dependency {
            module_key: "com.test.module".to_string(),
            version_spec: ">=1.0.0".to_string(),
            optional: false,
        });
        
        let result = <ModuleConfig as Validatable>::validate(&self_dep_config).await;
        assert!(!result.is_valid());
    }
}