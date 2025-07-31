//! Workspace entity - User's configured research environment
//! 
//! The Workspace is unique in Layer 3 as it's the environment controller
//! that governs all modules and provides configurations to them.

use crate::{
    entity::{Entity, EntityMetadata},
    validation::{Validatable, ValidationResult},
    EntityId, Error, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

use super::{
    ConfigSource, MethodologyConfig,
    StandardsConfig, ModuleConfig, ModuleConfigId,
    TemplateConfig, ValidationRule,
    WorkspaceId,
};

/// Workspace-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMetadata {
    /// Unique workspace ID
    pub id: WorkspaceId,
    
    /// Human-readable workspace identifier
    pub workspace_key: String,
    
    /// Workspace name
    pub name: String,
    
    /// Workspace description
    pub description: Option<String>,
    
    /// Owner of the workspace
    pub owner: EntityId,
    
    /// Whether the workspace is currently active
    pub active: bool,
    
    /// When the workspace was created
    pub created_at: DateTime<Utc>,
    
    /// When the workspace was last modified
    pub modified_at: DateTime<Utc>,
    
    /// Last time the workspace was accessed
    pub last_accessed: DateTime<Utc>,
}

/// Export preferences for the workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPreferences {
    pub default_format: String,
    pub include_metadata: bool,
    pub compression: Option<String>,
    pub encryption: Option<String>,
}

impl Default for ExportPreferences {
    fn default() -> Self {
        Self {
            default_format: "json".to_string(),
            include_metadata: true,
            compression: None,
            encryption: None,
        }
    }
}

/// Reference to an enabled module
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EnabledModule {
    pub module_id: ModuleConfigId,
    pub enabled: bool,
    pub settings: HashMap<String, serde_json::Value>,
}

/// User's configured research workspace
/// This is the environment controller for all research activities
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Workspace {
    /// Workspace-specific metadata
    pub metadata: WorkspaceMetadata,
    
    /// Standard entity metadata for compatibility
    #[serde(skip)]
    entity_metadata: Option<EntityMetadata>,
    
    /// Loaded methodology configurations
    pub methodologies: HashMap<String, MethodologyConfig>,
    
    /// Active methodology IDs
    pub active_methodologies: Vec<String>,
    
    /// Loaded standards configurations
    pub standards: HashMap<String, StandardsConfig>,
    
    /// Active standard IDs
    pub active_standards: Vec<String>,
    
    /// Enabled modules with their settings
    pub enabled_modules: Vec<EnabledModule>,
    
    /// Loaded module configurations
    pub module_configs: HashMap<String, ModuleConfig>,
    
    /// Loaded template configurations
    pub templates: HashMap<String, TemplateConfig>,
    
    /// Default templates for different work product types
    pub default_templates: HashMap<String, String>,
    
    /// Loaded validation rules
    pub validation_rules: HashMap<String, ValidationRule>,
    
    /// Export preferences
    pub export_preferences: ExportPreferences,
    
    /// Currently open theories (for UI state)
    pub open_theories: Vec<EntityId>,
    
    /// Active research logs (for UI state)
    pub active_research_logs: Vec<EntityId>,
    
    /// Recent work products (for quick access)
    pub recent_work_products: Vec<EntityId>,
    
    /// Collaborators with access
    pub collaborators: Vec<EntityId>,
    
    /// Parent workspace (for nested workspaces)
    pub parent_workspace: Option<WorkspaceId>,
    
    /// Child workspaces
    pub child_workspaces: Vec<WorkspaceId>,
    
    /// Custom metadata for extensions
    pub custom_metadata: HashMap<String, serde_json::Value>,
    
    /// Configuration source
    pub config_source: ConfigSource,
}

impl Workspace {
    /// Creates a new workspace
    pub fn new(workspace_key: String, name: String, owner: EntityId) -> Self {
        let now = Utc::now();
        let id = WorkspaceId::new();
        
        Self {
            metadata: WorkspaceMetadata {
                id: id.clone(),
                workspace_key,
                name,
                description: None,
                owner: owner.clone(),
                active: true,
                created_at: now,
                modified_at: now,
                last_accessed: now,
            },
            entity_metadata: Some(EntityMetadata::new(owner)),
            methodologies: HashMap::new(),
            active_methodologies: Vec::new(),
            standards: HashMap::new(),
            active_standards: Vec::new(),
            enabled_modules: Vec::new(),
            module_configs: HashMap::new(),
            templates: HashMap::new(),
            default_templates: HashMap::new(),
            validation_rules: HashMap::new(),
            export_preferences: ExportPreferences::default(),
            open_theories: Vec::new(),
            active_research_logs: Vec::new(),
            recent_work_products: Vec::new(),
            collaborators: Vec::new(),
            parent_workspace: None,
            child_workspaces: Vec::new(),
            custom_metadata: HashMap::new(),
            config_source: ConfigSource::UserDefined { created_at: now },
        }
    }
    
    /// Loads a methodology configuration into the workspace
    pub fn load_methodology(&mut self, config: MethodologyConfig) -> Result<()> {
        let key = config.metadata.methodology_key.clone();
        self.methodologies.insert(key.clone(), config);
        self.update_modified();
        Ok(())
    }
    
    /// Activates a methodology
    pub fn activate_methodology(&mut self, methodology_key: &str) -> Result<()> {
        if !self.methodologies.contains_key(methodology_key) {
            return Err(Error::ConfigurationError(format!("Methodology '{}' not loaded", methodology_key)));
        }
        
        if !self.active_methodologies.contains(&methodology_key.to_string()) {
            self.active_methodologies.push(methodology_key.to_string());
            self.update_modified();
        }
        
        Ok(())
    }
    
    /// Deactivates a methodology
    pub fn deactivate_methodology(&mut self, methodology_key: &str) {
        self.active_methodologies.retain(|k| k != methodology_key);
        self.update_modified();
    }
    
    /// Gets an active methodology configuration
    pub fn get_active_methodology(&self, key: &str) -> Option<&MethodologyConfig> {
        if self.active_methodologies.contains(&key.to_string()) {
            self.methodologies.get(key)
        } else {
            None
        }
    }
    
    /// Loads a standards configuration
    pub fn load_standards(&mut self, config: StandardsConfig) -> Result<()> {
        let key = config.metadata.standards_key.clone();
        self.standards.insert(key, config);
        self.update_modified();
        Ok(())
    }
    
    /// Loads a module configuration
    pub fn load_module_config(&mut self, config: ModuleConfig) -> Result<()> {
        let key = config.metadata.module_key.clone();
        self.module_configs.insert(key, config);
        self.update_modified();
        Ok(())
    }
    
    /// Enables a module
    pub fn enable_module(&mut self, module_key: &str, settings: HashMap<String, serde_json::Value>) -> Result<()> {
        let config = self.module_configs.get(module_key)
            .ok_or_else(|| Error::ConfigurationError(format!("Module '{}' not loaded", module_key)))?;
        
        let enabled_module = EnabledModule {
            module_id: config.metadata.id.clone(),
            enabled: true,
            settings,
        };
        
        // Remove any existing entry for this module
        self.enabled_modules.retain(|m| m.module_id != config.metadata.id);
        self.enabled_modules.push(enabled_module);
        
        self.update_modified();
        Ok(())
    }
    
    /// Disables a module
    pub fn disable_module(&mut self, module_id: &ModuleConfigId) {
        if let Some(module) = self.enabled_modules.iter_mut().find(|m| &m.module_id == module_id) {
            module.enabled = false;
            self.update_modified();
        }
    }
    
    /// Loads a template configuration
    pub fn load_template(&mut self, config: TemplateConfig) -> Result<()> {
        let key = config.metadata.template_key.clone();
        self.templates.insert(key, config);
        self.update_modified();
        Ok(())
    }
    
    /// Sets the default template for a work product type
    pub fn set_default_template(&mut self, work_product_type: String, template_key: String) -> Result<()> {
        if !self.templates.contains_key(&template_key) {
            return Err(Error::ConfigurationError(format!("Template '{}' not loaded", template_key)));
        }
        
        self.default_templates.insert(work_product_type, template_key);
        self.update_modified();
        Ok(())
    }
    
    /// Loads a validation rule
    pub fn load_validation_rule(&mut self, rule: ValidationRule) -> Result<()> {
        let key = rule.metadata.rule_key.clone();
        self.validation_rules.insert(key, rule);
        self.update_modified();
        Ok(())
    }
    
    /// Adds a collaborator
    pub fn add_collaborator(&mut self, collaborator: EntityId) {
        if !self.collaborators.contains(&collaborator) {
            self.collaborators.push(collaborator);
            self.update_modified();
        }
    }
    
    /// Removes a collaborator
    pub fn remove_collaborator(&mut self, collaborator: &EntityId) {
        self.collaborators.retain(|c| c != collaborator);
        self.update_modified();
    }
    
    /// Updates the last accessed timestamp
    pub fn touch(&mut self) {
        self.metadata.last_accessed = Utc::now();
    }
    
    /// Adds a work product to recent items
    pub fn add_recent_work_product(&mut self, work_product_id: EntityId) {
        self.recent_work_products.retain(|id| id != &work_product_id);
        self.recent_work_products.insert(0, work_product_id);
        
        // Keep only the most recent 20 items
        if self.recent_work_products.len() > 20 {
            self.recent_work_products.truncate(20);
        }
        
        self.update_modified();
    }
    
    /// Gets all active configurations for modules to use
    pub fn get_active_configs(&self) -> ActiveConfigs {
        ActiveConfigs {
            methodologies: self.active_methodologies.iter()
                .filter_map(|key| self.methodologies.get(key))
                .cloned()
                .collect(),
            standards: self.active_standards.iter()
                .filter_map(|key| self.standards.get(key))
                .cloned()
                .collect(),
            validation_rules: self.validation_rules.values()
                .cloned()
                .collect(),
            templates: self.templates.clone(),
            default_templates: self.default_templates.clone(),
        }
    }
    
    /// Updates modification timestamp
    fn update_modified(&mut self) {
        self.metadata.modified_at = Utc::now();
        if let Some(ref mut em) = self.entity_metadata {
            em.update(self.metadata.owner.clone());
        }
    }
}

/// Active configurations that modules can access
#[derive(Debug, Clone)]
pub struct ActiveConfigs {
    pub methodologies: Vec<MethodologyConfig>,
    pub standards: Vec<StandardsConfig>,
    pub validation_rules: Vec<ValidationRule>,
    pub templates: HashMap<String, TemplateConfig>,
    pub default_templates: HashMap<String, String>,
}

use async_trait::async_trait;

#[async_trait]
impl Validatable for Workspace {
    async fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Validate using validator crate
        if let Err(e) = <Self as validator::Validate>::validate(self) {
            for (field, errors) in e.field_errors() {
                for error in errors {
                    result.add_error(
                        field.to_string(),
                        "validation_failed",
                        error.to_string()
                    );
                }
            }
        }
        
        // Custom validation
        if self.metadata.workspace_key.trim().is_empty() {
            result.add_error(
                "workspace_key",
                "required",
                "Workspace key cannot be empty"
            );
        }
        
        if self.metadata.name.trim().is_empty() {
            result.add_error(
                "name",
                "required",
                "Workspace name cannot be empty"
            );
        }
        
        // Validate active methodologies are loaded
        for methodology in &self.active_methodologies {
            if !self.methodologies.contains_key(methodology) {
                result.add_error(
                    "active_methodologies",
                    "invalid_reference",
                    format!("Active methodology '{}' is not loaded", methodology)
                );
            }
        }
        
        // Validate active standards are loaded
        for standard in &self.active_standards {
            if !self.standards.contains_key(standard) {
                result.add_error(
                    "active_standards",
                    "invalid_reference",
                    format!("Active standard '{}' is not loaded", standard)
                );
            }
        }
        
        // Validate default templates exist
        for (work_type, template_key) in &self.default_templates {
            if !self.templates.contains_key(template_key) {
                result.add_error(
                    "default_templates",
                    "invalid_reference",
                    format!("Default template '{}' for '{}' is not loaded", template_key, work_type)
                );
            }
        }
        
        result
    }
}

// Implement Entity trait for Workspace
impl Entity for Workspace {
    fn id(&self) -> EntityId {
        self.metadata.id.clone()
    }
    
    fn entity_type(&self) -> &'static str {
        "Workspace"
    }
    
    fn created_by(&self) -> EntityId {
        self.entity_metadata.as_ref()
            .map(|em| em.created_by.clone())
            .unwrap_or_else(|| self.metadata.owner.clone())
    }
    
    fn created_at(&self) -> DateTime<Utc> {
        self.entity_metadata.as_ref()
            .map(|em| em.created_at)
            .unwrap_or(self.metadata.created_at)
    }
    
    fn modified_by(&self) -> EntityId {
        self.entity_metadata.as_ref()
            .map(|em| em.modified_by.clone())
            .unwrap_or_else(|| self.metadata.owner.clone())
    }
    
    fn modified_at(&self) -> DateTime<Utc> {
        self.entity_metadata.as_ref()
            .map(|em| em.modified_at)
            .unwrap_or(self.metadata.modified_at)
    }
    
    fn is_active(&self) -> bool {
        self.metadata.active
    }
    
    fn as_entity(&self) -> crate::entity::EntityData {
        crate::entity::EntityData {
            id: self.id(),
            entity_type: self.entity_type().to_string(),
            created_by: self.created_by(),
            created_at: self.created_at(),
            modified_by: self.modified_by(),
            modified_at: self.modified_at(),
            is_active: self.is_active(),
            version: self.entity_metadata.as_ref().map(|em| em.version),
            data: serde_json::to_value(self).unwrap_or(serde_json::Value::Null),
        }
    }
}

// Note: Workspace implements Entity manually due to custom metadata structure
// impl_entity!(Workspace, "Workspace");
// impl_validatable!(Workspace);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workspace_creation() {
        let owner = EntityId::new();
        let workspace = Workspace::new(
            "research-2025".to_string(),
            "My Research Project".to_string(),
            owner.clone()
        );
        
        assert_eq!(workspace.metadata.workspace_key, "research-2025");
        assert_eq!(workspace.metadata.name, "My Research Project");
        assert_eq!(workspace.metadata.owner, owner);
        assert!(workspace.metadata.active);
        assert!(workspace.methodologies.is_empty());
        assert!(workspace.active_methodologies.is_empty());
    }
    
    #[test]
    fn test_methodology_management() {
        let mut workspace = Workspace::new(
            "test".to_string(),
            "Test".to_string(),
            EntityId::new()
        );
        
        // Create a dummy methodology config
        let methodology = MethodologyConfig::new(
            "gps-2021".to_string(),
            "GPS 2021".to_string(),
            "2021.1".to_string(),
            "BCG".to_string(),
        );
        
        // Load methodology
        assert!(workspace.load_methodology(methodology).is_ok());
        assert!(workspace.methodologies.contains_key("gps-2021"));
        
        // Activate methodology
        assert!(workspace.activate_methodology("gps-2021").is_ok());
        assert!(workspace.active_methodologies.contains(&"gps-2021".to_string()));
        
        // Try to activate non-existent methodology
        assert!(workspace.activate_methodology("bcg-2023").is_err());
        
        // Deactivate methodology
        workspace.deactivate_methodology("gps-2021");
        assert!(!workspace.active_methodologies.contains(&"gps-2021".to_string()));
    }
    
    #[test]
    fn test_collaborator_management() {
        let mut workspace = Workspace::new(
            "test".to_string(),
            "Test".to_string(),
            EntityId::new()
        );
        
        let collaborator1 = EntityId::new();
        let collaborator2 = EntityId::new();
        
        // Add collaborators
        workspace.add_collaborator(collaborator1.clone());
        workspace.add_collaborator(collaborator2.clone());
        assert_eq!(workspace.collaborators.len(), 2);
        
        // No duplicates
        workspace.add_collaborator(collaborator1.clone());
        assert_eq!(workspace.collaborators.len(), 2);
        
        // Remove collaborator
        workspace.remove_collaborator(&collaborator1);
        assert_eq!(workspace.collaborators.len(), 1);
    }
    
    #[test]
    fn test_recent_work_products() {
        let mut workspace = Workspace::new(
            "test".to_string(),
            "Test".to_string(),
            EntityId::new()
        );
        
        // Add work products
        for _ in 0..25 {
            workspace.add_recent_work_product(EntityId::new());
        }
        
        // Should keep only 20 most recent
        assert_eq!(workspace.recent_work_products.len(), 20);
    }
    
    #[tokio::test]
    async fn test_workspace_validation() {
        let mut workspace = Workspace::new(
            "test".to_string(),
            "Test".to_string(),
            EntityId::new()
        );
        
        // Valid workspace
        assert!(<Workspace as Validatable>::validate(&workspace).await.is_valid());
        
        // Empty workspace key
        workspace.metadata.workspace_key = "".to_string();
        let result = <Workspace as Validatable>::validate(&workspace).await;
        assert!(!result.is_valid());
        assert!(result.has_errors());
        
        // Fix key, add invalid active methodology
        workspace.metadata.workspace_key = "test".to_string();
        workspace.active_methodologies.push("non-existent".to_string());
        let result = <Workspace as Validatable>::validate(&workspace).await;
        assert!(!result.is_valid());
    }
}

