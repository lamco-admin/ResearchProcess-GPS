//! TemplateRegistry entity - Registry of templates for work products
//! 
//! Manages templates for generating research documentation, reports, and visualizations.
//! Templates support multiple formats and can be customized per methodology.

use crate::{
    validation::{Validatable, ValidationResult, ValidationIssue, ValidationSeverity},
    work_product::WorkProductType,
    EntityId, Error, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use validator::Validate;

use super::{
    ConfigEntity, ConfigSource, TemplateRegistryId, TemplateConfigId, TemplateFormat,
};

/// Template registry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateRegistryMetadata {
    /// Unique ID for this registry
    pub id: TemplateRegistryId,
    
    /// Registry identifier
    pub registry_key: String,
    
    /// Display name
    pub name: String,
    
    /// Registry version
    pub version: String,
    
    /// When this registry was created
    pub created_at: DateTime<Utc>,
    
    /// When this registry was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Last time templates were synced
    pub last_synced: Option<DateTime<Utc>>,
    
    /// Source of this registry
    pub source: ConfigSource,
}

/// Template metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfigMetadata {
    /// Unique ID for this template
    pub id: TemplateConfigId,
    
    /// Template identifier (e.g., "gps-proof-statement")
    pub template_key: String,
    
    /// Display name
    pub name: String,
    
    /// Template version
    pub version: String,
    
    /// Template author
    pub author: String,
    
    /// When this template was created
    pub created_at: DateTime<Utc>,
    
    /// When this template was last updated
    pub updated_at: DateTime<Utc>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    /// Variable name
    pub name: String,
    
    /// Data type
    pub var_type: String,
    
    /// Description
    pub description: String,
    
    /// Whether required
    pub required: bool,
    
    /// Default value if not provided
    pub default_value: Option<serde_json::Value>,
    
    /// Example value
    pub example: Option<serde_json::Value>,
}

/// Template example showing usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateExample {
    /// Example name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Sample data
    pub sample_data: HashMap<String, serde_json::Value>,
    
    /// Expected output snippet
    pub expected_output: Option<String>,
}

/// Template configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TemplateConfig {
    /// Template metadata
    pub metadata: TemplateConfigMetadata,
    
    /// Brief description
    pub description: String,
    
    /// Template category (e.g., "proof", "report", "chart")
    pub category: String,
    
    /// Output format
    pub format: TemplateFormat,
    
    /// Template content
    pub content: String,
    
    /// Variables this template expects
    pub variables: Vec<TemplateVariable>,
    
    /// Usage examples
    pub examples: Vec<TemplateExample>,
    
    /// Compatible work product types
    pub compatible_work_products: Vec<WorkProductType>,
    
    /// Compatible methodologies
    pub compatible_methodologies: Vec<String>,
    
    /// Tags for organization
    pub tags: HashSet<String>,
    
    /// Related templates
    pub related_templates: Vec<String>,
    
    /// Template inheritance (base template)
    pub extends: Option<String>,
    
    /// Custom metadata
    pub custom_metadata: HashMap<String, serde_json::Value>,
}

impl TemplateConfig {
    /// Creates a new template configuration
    pub fn new(
        template_key: String,
        name: String,
        version: String,
        author: String,
        category: String,
        format: TemplateFormat,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: TemplateConfigMetadata {
                id: TemplateConfigId::new(),
                template_key,
                name,
                version,
                author,
                created_at: now,
                updated_at: now,
            },
            description: String::new(),
            category,
            format,
            content: String::new(),
            variables: Vec::new(),
            examples: Vec::new(),
            compatible_work_products: Vec::new(),
            compatible_methodologies: Vec::new(),
            tags: HashSet::new(),
            related_templates: Vec::new(),
            extends: None,
            custom_metadata: HashMap::new(),
        }
    }
    
    /// Creates an example GPS proof statement template
    pub fn create_gps_proof_statement_example() -> Self {
        let mut template = Self::new(
            "gps-proof-statement".to_string(),
            "GPS Proof Statement Template".to_string(),
            "1.0.0".to_string(),
            "ResearchProcess-GPS".to_string(),
            "proof".to_string(),
            TemplateFormat::Markdown,
        );
        
        template.description = "Standard template for Genealogical Proof Standard proof statements".to_string();
        
        // Template content with variables
        template.content = r#"# Proof Statement: {{title}}

**Research Question**: {{research_question}}

**Date**: {{date}}
**Researcher**: {{researcher_name}}

## Summary of Findings

{{summary}}

## GPS Elements Satisfied

### 1. Reasonably Exhaustive Research

{{exhaustive_research_summary}}

**Sources Consulted**: {{source_count}}
**Repositories Searched**: {{repository_count}}

### 2. Complete and Accurate Source Citations

All {{source_count}} sources have been documented with complete citations following {{citation_style}} format.

### 3. Thorough Analysis and Correlation

{{analysis_summary}}

### 4. Resolution of Conflicting Evidence

{{#if has_conflicts}}
{{conflict_resolution}}
{{else}}
No conflicting evidence was found.
{{/if}}

### 5. Sound Written Conclusion

Based on the evidence analyzed, {{conclusion}}

## Evidence Summary

{{#each evidence_items}}
### {{this.title}}
- **Source**: {{this.source}}
- **Information**: {{this.information}}
- **Assessment**: {{this.assessment}}
{{/each}}

## Appendices

{{#if has_appendices}}
{{appendices}}
{{/if}}

---
*Generated using ResearchProcess-GPS {{version}}*
"#.to_string();
        
        // Define variables
        template.variables.push(TemplateVariable {
            name: "title".to_string(),
            var_type: "string".to_string(),
            description: "Title of the proof statement".to_string(),
            required: true,
            default_value: None,
            example: Some(serde_json::json!("Establishing the Parents of John Smith")),
        });
        
        template.variables.push(TemplateVariable {
            name: "research_question".to_string(),
            var_type: "string".to_string(),
            description: "The specific research question being answered".to_string(),
            required: true,
            default_value: None,
            example: Some(serde_json::json!("Who were the parents of John Smith born 1850 in Ohio?")),
        });
        
        template.variables.push(TemplateVariable {
            name: "source_count".to_string(),
            var_type: "number".to_string(),
            description: "Number of sources consulted".to_string(),
            required: true,
            default_value: None,
            example: Some(serde_json::json!(25)),
        });
        
        template.variables.push(TemplateVariable {
            name: "has_conflicts".to_string(),
            var_type: "boolean".to_string(),
            description: "Whether conflicting evidence was found".to_string(),
            required: false,
            default_value: Some(serde_json::json!(false)),
            example: Some(serde_json::json!(true)),
        });
        
        // Compatible with GPS methodology
        template.compatible_methodologies.push("gps-2021".to_string());
        template.compatible_work_products.push(WorkProductType::ProofStatement);
        
        // Tags
        template.tags.insert("gps".to_string());
        template.tags.insert("proof".to_string());
        template.tags.insert("formal".to_string());
        
        template
    }
    
    /// Validates template variables are properly defined
    pub fn validate_variables(&self) -> Result<()> {
        let mut var_names = HashSet::new();
        
        for var in &self.variables {
            if var.name.is_empty() {
                return Err(Error::ConfigurationError("Template variable name cannot be empty".to_string()));
            }
            
            if !var_names.insert(&var.name) {
                return Err(Error::ConfigurationError(format!("Duplicate variable name: {}", var.name)));
            }
            
            if var.var_type.is_empty() {
                return Err(Error::ConfigurationError(format!("Variable '{}' must have a type", var.name)));
            }
        }
        
        Ok(())
    }
    
    /// Gets required variables
    pub fn required_variables(&self) -> Vec<&TemplateVariable> {
        self.variables.iter()
            .filter(|v| v.required)
            .collect()
    }
    
    /// Checks if template is compatible with a work product type
    pub fn is_compatible_with_work_product(&self, work_product_type: &WorkProductType) -> bool {
        self.compatible_work_products.is_empty() || 
        self.compatible_work_products.contains(work_product_type)
    }
    
    /// Checks if template is compatible with a methodology
    pub fn is_compatible_with_methodology(&self, methodology: &str) -> bool {
        self.compatible_methodologies.is_empty() || 
        self.compatible_methodologies.contains(&methodology.to_string())
    }
}

/// Template source location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSource {
    /// Source type
    pub source_type: TemplateSourceType,
    
    /// Location (path or URL)
    pub location: String,
    
    /// Whether enabled
    pub enabled: bool,
    
    /// Auto-discovery pattern
    pub discovery_pattern: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateSourceType {
    /// Local filesystem
    Local,
    /// Git repository
    Git,
    /// HTTP/HTTPS URL
    Http,
    /// Embedded in binary
    Embedded,
}

/// Registry of available templates
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TemplateRegistry {
    /// Registry metadata
    pub metadata: TemplateRegistryMetadata,
    
    /// Loaded templates
    pub templates: HashMap<String, TemplateConfig>,
    
    /// Template categories
    pub categories: HashMap<String, Vec<String>>,
    
    /// Methodology compatibility map
    pub methodology_compatibility: HashMap<String, Vec<String>>,
    
    /// Work product compatibility map
    pub work_product_compatibility: HashMap<WorkProductType, Vec<String>>,
    
    /// Template sources
    pub template_sources: Vec<TemplateSource>,
    
    /// Custom user templates
    pub custom_templates: HashMap<String, TemplateConfig>,
    
    /// Default template selections
    pub defaults: HashMap<String, String>,
    
    /// Template tags for searching
    pub tag_index: HashMap<String, HashSet<String>>,
    
    /// Custom metadata
    pub custom_metadata: HashMap<String, serde_json::Value>,
}

impl TemplateRegistry {
    /// Creates a new template registry
    pub fn new(registry_key: String, name: String, version: String) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: TemplateRegistryMetadata {
                id: TemplateRegistryId::new(),
                registry_key,
                name,
                version,
                created_at: now,
                updated_at: now,
                last_synced: None,
                source: ConfigSource::UserDefined { created_at: now },
            },
            templates: HashMap::new(),
            categories: HashMap::new(),
            methodology_compatibility: HashMap::new(),
            work_product_compatibility: HashMap::new(),
            template_sources: Vec::new(),
            custom_templates: HashMap::new(),
            defaults: HashMap::new(),
            tag_index: HashMap::new(),
            custom_metadata: HashMap::new(),
        }
    }
    
    /// Creates a default registry with standard templates
    pub fn create_default() -> Self {
        let mut registry = Self::new(
            "default".to_string(),
            "Default Template Registry".to_string(),
            "1.0.0".to_string(),
        );
        
        // Add GPS proof statement template
        let gps_template = TemplateConfig::create_gps_proof_statement_example();
        registry.add_template(gps_template);
        
        // Add default template sources
        registry.template_sources.push(TemplateSource {
            source_type: TemplateSourceType::Local,
            location: "~/.researchprocess-gps/templates".to_string(),
            enabled: true,
            discovery_pattern: Some("*.{md,html,tex}".to_string()),
        });
        
        registry.template_sources.push(TemplateSource {
            source_type: TemplateSourceType::Embedded,
            location: "builtin".to_string(),
            enabled: true,
            discovery_pattern: None,
        });
        
        registry
    }
    
    /// Adds a template to the registry
    pub fn add_template(&mut self, template: TemplateConfig) {
        let key = template.metadata.template_key.clone();
        
        // Update category index
        self.categories
            .entry(template.category.clone())
            .or_insert_with(Vec::new)
            .push(key.clone());
        
        // Update methodology compatibility
        for methodology in &template.compatible_methodologies {
            self.methodology_compatibility
                .entry(methodology.clone())
                .or_insert_with(Vec::new)
                .push(key.clone());
        }
        
        // Update work product compatibility
        for wp_type in &template.compatible_work_products {
            self.work_product_compatibility
                .entry(wp_type.clone())
                .or_insert_with(Vec::new)
                .push(key.clone());
        }
        
        // Update tag index
        for tag in &template.tags {
            self.tag_index
                .entry(tag.clone())
                .or_insert_with(HashSet::new)
                .insert(key.clone());
        }
        
        // Add template
        self.templates.insert(key, template);
        self.metadata.updated_at = Utc::now();
    }
    
    /// Removes a template from the registry
    pub fn remove_template(&mut self, template_key: &str) -> Option<TemplateConfig> {
        if let Some(template) = self.templates.remove(template_key) {
            // Clean up indices
            if let Some(cat_templates) = self.categories.get_mut(&template.category) {
                cat_templates.retain(|k| k != template_key);
            }
            
            for methodology in &template.compatible_methodologies {
                if let Some(templates) = self.methodology_compatibility.get_mut(methodology) {
                    templates.retain(|k| k != template_key);
                }
            }
            
            for wp_type in &template.compatible_work_products {
                if let Some(templates) = self.work_product_compatibility.get_mut(wp_type) {
                    templates.retain(|k| k != template_key);
                }
            }
            
            for tag in &template.tags {
                if let Some(templates) = self.tag_index.get_mut(tag) {
                    templates.remove(template_key);
                }
            }
            
            self.metadata.updated_at = Utc::now();
            Some(template)
        } else {
            None
        }
    }
    
    /// Gets a template by key
    pub fn get_template(&self, key: &str) -> Option<&TemplateConfig> {
        self.templates.get(key).or_else(|| self.custom_templates.get(key))
    }
    
    /// Finds templates by category
    pub fn find_by_category(&self, category: &str) -> Vec<&TemplateConfig> {
        self.categories
            .get(category)
            .map(|keys| {
                keys.iter()
                    .filter_map(|k| self.get_template(k))
                    .collect()
            })
            .unwrap_or_default() // Return empty vec when no templates match
    }
    
    /// Finds templates compatible with a methodology
    pub fn find_by_methodology(&self, methodology: &str) -> Vec<&TemplateConfig> {
        self.methodology_compatibility
            .get(methodology)
            .map(|keys| {
                keys.iter()
                    .filter_map(|k| self.get_template(k))
                    .collect()
            })
            .unwrap_or_default() // Return empty vec when no templates match
    }
    
    /// Finds templates compatible with a work product type
    pub fn find_by_work_product(&self, work_product_type: &WorkProductType) -> Vec<&TemplateConfig> {
        self.work_product_compatibility
            .get(work_product_type)
            .map(|keys| {
                keys.iter()
                    .filter_map(|k| self.get_template(k))
                    .collect()
            })
            .unwrap_or_default() // Return empty vec when no templates match
    }
    
    /// Finds templates by tag
    pub fn find_by_tag(&self, tag: &str) -> Vec<&TemplateConfig> {
        self.tag_index
            .get(tag)
            .map(|keys| {
                keys.iter()
                    .filter_map(|k| self.get_template(k))
                    .collect()
            })
            .unwrap_or_default() // Return empty vec when no templates match
    }
    
    /// Sets a default template for a category
    pub fn set_default(&mut self, category: &str, template_key: &str) -> Result<()> {
        if self.get_template(template_key).is_none() {
            return Err(Error::ConfigurationError(format!("Template '{}' not found", template_key)));
        }
        
        self.defaults.insert(category.to_string(), template_key.to_string());
        self.metadata.updated_at = Utc::now();
        Ok(())
    }
    
    /// Gets the default template for a category
    pub fn get_default(&self, category: &str) -> Option<&TemplateConfig> {
        self.defaults.get(category)
            .and_then(|key| self.get_template(key))
    }
    
    /// Discovers templates from configured sources
    pub async fn discover_templates(&mut self) -> Result<Vec<String>> {
        let mut discovered = Vec::new();
        
        for source in &self.template_sources {
            if !source.enabled {
                continue;
            }
            
            match source.source_type {
                TemplateSourceType::Local => {
                    // In a real implementation, would scan filesystem
                    // For now, just mark as discovered
                    discovered.push(format!("Discovered templates from {}", source.location));
                }
                TemplateSourceType::Git => {
                    // Would clone/pull git repository
                    discovered.push(format!("Synced templates from git: {}", source.location));
                }
                TemplateSourceType::Http => {
                    // Would download templates
                    discovered.push(format!("Downloaded templates from {}", source.location));
                }
                TemplateSourceType::Embedded => {
                    // Already loaded
                    discovered.push("Loaded embedded templates".to_string());
                }
            }
        }
        
        self.metadata.last_synced = Some(Utc::now());
        Ok(discovered)
    }
}

impl ConfigEntity for TemplateRegistry {
    fn id(&self) -> EntityId {
        self.metadata.id
    }
    
    fn config_type(&self) -> &'static str {
        "TemplateRegistry"
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn validate(&self) -> Result<()> {
        if self.metadata.registry_key.is_empty() {
            return Err(Error::ConfigurationError("Registry key cannot be empty".to_string()));
        }
        
        if self.metadata.name.is_empty() {
            return Err(Error::ConfigurationError("Registry name cannot be empty".to_string()));
        }
        
        // Validate all templates
        for (key, template) in &self.templates {
            if key != &template.metadata.template_key {
                return Err(Error::ConfigurationError(format!(
                    "Template key mismatch: {} vs {}", 
                    key, template.metadata.template_key
                )));
            }
            
            template.validate_variables()?;
        }
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl Validatable for TemplateRegistry {
    async fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        
        // Registry key validation
        if self.metadata.registry_key.is_empty() {
            errors.push(ValidationIssue {
                field: "registry_key".to_string(),
                code: "required_field".to_string(),
                message: "Registry key is required".to_string(),
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
        
        // Validate template consistency
        for (key, template) in &self.templates {
            if key != &template.metadata.template_key {
                errors.push(ValidationIssue {
                    field: "templates".to_string(),
                    code: "consistency_error".to_string(),
                    message: format!("Template key mismatch: {} vs {}", key, template.metadata.template_key),
                    severity: ValidationSeverity::Error,
                });
            }
            
            // Validate template
            let template_result = <TemplateConfig as Validatable>::validate(template).await;
            errors.extend(template_result.issues);
        }
        
        // Validate defaults reference existing templates
        for (_category, template_key) in &self.defaults {
            if self.get_template(template_key).is_none() {
                errors.push(ValidationIssue {
                    field: "defaults".to_string(),
                    code: "invalid_reference".to_string(),
                    message: format!("Default template '{}' not found", template_key),
                    severity: ValidationSeverity::Error,
                });
            }
        }
        
        ValidationResult { issues: errors }
    }
}

#[async_trait::async_trait]
impl Validatable for TemplateConfig {
    async fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        
        // Template key validation
        if self.metadata.template_key.is_empty() {
            errors.push(ValidationIssue {
                field: "template_key".to_string(),
                code: "required_field".to_string(),
                message: "Template key is required".to_string(),
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
        
        // Content validation
        if self.content.is_empty() {
            errors.push(ValidationIssue {
                field: "content".to_string(),
                code: "required_field".to_string(),
                message: "Content is required".to_string(),
                severity: ValidationSeverity::Error,
            });
        }
        
        // Category validation
        if self.category.is_empty() {
            errors.push(ValidationIssue {
                field: "category".to_string(),
                code: "required_field".to_string(),
                message: "Category is required".to_string(),
                severity: ValidationSeverity::Error,
            });
        }
        
        // Variable validation
        let mut var_names = HashSet::new();
        for var in &self.variables {
            if var.name.is_empty() {
                errors.push(ValidationIssue {
                    field: "variable.name".to_string(),
                    code: "required_field".to_string(),
                    message: "Variable name is required".to_string(),
                    severity: ValidationSeverity::Error,
                });
            } else if !var_names.insert(&var.name) {
                errors.push(ValidationIssue {
                    field: "variables".to_string(),
                    code: "duplicate_value".to_string(),
                    message: format!("Duplicate variable name: {}", var.name),
                    severity: ValidationSeverity::Error,
                });
            }
        }
        
        // Check for circular inheritance
        if let Some(base) = &self.extends {
            if base == &self.metadata.template_key {
                errors.push(ValidationIssue {
                    field: "extends".to_string(),
                    code: "invalid_reference".to_string(),
                    message: format!("Template cannot extend itself: {}", base),
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
    fn test_template_config_creation() {
        let template = TemplateConfig::new(
            "test-template".to_string(),
            "Test Template".to_string(),
            "1.0.0".to_string(),
            "Test Author".to_string(),
            "test".to_string(),
            TemplateFormat::Markdown,
        );
        
        assert_eq!(template.metadata.template_key, "test-template");
        assert_eq!(template.metadata.name, "Test Template");
        assert_eq!(template.category, "test");
        assert_eq!(template.format, TemplateFormat::Markdown);
    }
    
    #[test]
    fn test_gps_proof_statement_template() {
        let template = TemplateConfig::create_gps_proof_statement_example();
        
        assert_eq!(template.metadata.template_key, "gps-proof-statement");
        assert_eq!(template.category, "proof");
        assert_eq!(template.format, TemplateFormat::Markdown);
        assert!(!template.content.is_empty());
        assert!(!template.variables.is_empty());
        assert!(template.compatible_methodologies.contains(&"gps-2021".to_string()));
    }
    
    #[test]
    fn test_template_variable_filtering() {
        let template = TemplateConfig::create_gps_proof_statement_example();
        
        let required = template.required_variables();
        assert!(required.len() > 0);
        assert!(required.iter().all(|v| v.required));
        
        let optional: Vec<_> = template.variables.iter()
            .filter(|v| !v.required)
            .collect();
        assert!(optional.len() > 0);
    }
    
    #[test]
    fn test_template_compatibility() {
        let template = TemplateConfig::create_gps_proof_statement_example();
        
        assert!(template.is_compatible_with_methodology("gps-2021"));
        assert!(!template.is_compatible_with_methodology("unknown")); // Specific methodology list
        
        assert!(template.is_compatible_with_work_product(&WorkProductType::ProofStatement));
        assert!(!template.is_compatible_with_work_product(&WorkProductType::ResearchReport));
    }
    
    #[test]
    fn test_registry_creation() {
        let registry = TemplateRegistry::new(
            "test-registry".to_string(),
            "Test Registry".to_string(),
            "1.0.0".to_string(),
        );
        
        assert_eq!(registry.metadata.registry_key, "test-registry");
        assert!(registry.templates.is_empty());
        assert!(registry.categories.is_empty());
    }
    
    #[test]
    fn test_default_registry() {
        let registry = TemplateRegistry::create_default();
        
        assert_eq!(registry.metadata.registry_key, "default");
        assert!(!registry.templates.is_empty());
        assert!(!registry.template_sources.is_empty());
    }
    
    #[test]
    fn test_add_remove_template() {
        let mut registry = TemplateRegistry::new(
            "test-registry".to_string(),
            "Test Registry".to_string(),
            "1.0.0".to_string(),
        );
        
        let template = TemplateConfig::create_gps_proof_statement_example();
        let key = template.metadata.template_key.clone();
        
        registry.add_template(template);
        
        assert!(registry.get_template(&key).is_some());
        assert!(registry.categories.contains_key("proof"));
        assert!(!registry.tag_index.is_empty());
        
        let removed = registry.remove_template(&key);
        assert!(removed.is_some());
        assert!(registry.get_template(&key).is_none());
    }
    
    #[test]
    fn test_find_templates() {
        let registry = TemplateRegistry::create_default();
        
        // Find by category
        let proof_templates = registry.find_by_category("proof");
        assert!(!proof_templates.is_empty());
        
        // Find by methodology
        let gps_templates = registry.find_by_methodology("gps-2021");
        assert!(!gps_templates.is_empty());
        
        // Find by work product
        let proof_statement_templates = registry.find_by_work_product(&WorkProductType::ProofStatement);
        assert!(!proof_statement_templates.is_empty());
        
        // Find by tag
        let gps_tagged = registry.find_by_tag("gps");
        assert!(!gps_tagged.is_empty());
    }
    
    #[test]
    fn test_default_templates() {
        let mut registry = TemplateRegistry::create_default();
        
        let template = TemplateConfig::create_gps_proof_statement_example();
        let key = template.metadata.template_key.clone();
        registry.add_template(template);
        
        // Set default
        let result = registry.set_default("proof", &key);
        assert!(result.is_ok());
        
        // Get default
        let default = registry.get_default("proof");
        assert!(default.is_some());
        assert_eq!(default.unwrap().metadata.template_key, key);
        
        // Invalid default
        let result = registry.set_default("proof", "nonexistent");
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_template_validation() {
        let template = TemplateConfig::new(
            "test-template".to_string(),
            "Test Template".to_string(),
            "1.0.0".to_string(),
            "Test Author".to_string(),
            "test".to_string(),
            TemplateFormat::Markdown,
        );
        
        let result = <TemplateConfig as Validatable>::validate(&template).await;
        assert!(!result.is_valid()); // Content is empty
        
        let mut valid_template = template;
        valid_template.content = "Template content".to_string();
        
        let result = <TemplateConfig as Validatable>::validate(&valid_template).await;
        assert!(result.is_valid());
    }
    
    #[tokio::test]
    async fn test_registry_validation() {
        let registry = TemplateRegistry::create_default();
        
        let result = <TemplateRegistry as Validatable>::validate(&registry).await;
        assert!(result.is_valid());
        
        // Test with invalid default
        let mut invalid_registry = registry.clone();
        invalid_registry.defaults.insert("test".to_string(), "nonexistent".to_string());
        
        let result = <TemplateRegistry as Validatable>::validate(&invalid_registry).await;
        assert!(!result.is_valid());
    }
}