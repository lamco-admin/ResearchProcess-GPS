//! MethodologyConfig entity - Defines research methodologies as configuration
//! 
//! Methodologies like GPS (Genealogical Proof Standard) and BCG (Board for 
//! Certification of Genealogists) standards are loaded as data, not code.

use crate::{
    validation::{Validatable, ValidationResult},
    EntityId, Error, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use validator::Validate;

use super::{
    ConfigEntity, ConfigSource, MethodologyConfigId, RuleType, Severity,
};

/// Methodology-specific metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyMetadata {
    /// Unique ID for this configuration instance
    pub id: MethodologyConfigId,
    
    /// Methodology identifier (e.g., "gps-2021", "bcg-2023")
    pub methodology_key: String,
    
    /// Display name
    pub name: String,
    
    /// Version of this methodology
    pub version: String,
    
    /// Authority that defines this methodology
    pub authority: String,
    
    /// When this config was created
    pub created_at: DateTime<Utc>,
    
    /// When this config was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Source of this configuration
    pub source: ConfigSource,
}

/// A stage in the research workflow
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WorkflowStage {
    /// Stage identifier
    pub stage_id: String,
    
    /// Display name
    pub name: String,
    
    /// Description of this stage
    pub description: String,
    
    /// Required work products for this stage
    pub required_work_products: Vec<String>,
    
    /// Validation rules specific to this stage
    pub stage_rules: Vec<String>,
    
    /// Can proceed to these stages
    pub next_stages: Vec<String>,
}

/// Requirement specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementSpec {
    /// Requirement identifier
    pub requirement_id: String,
    
    /// Display name
    pub name: String,
    
    /// Detailed description
    pub description: String,
    
    /// Whether this is mandatory
    pub mandatory: bool,
    
    /// Validation rules that check this requirement
    pub validation_rules: Vec<String>,
}

/// Compliance rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRule {
    /// Rule identifier
    pub rule_id: String,
    
    /// What this rule checks
    pub description: String,
    
    /// Rule type
    pub rule_type: RuleType,
    
    /// Severity if violated
    pub severity: Severity,
    
    /// Which entities this applies to
    pub applies_to: Vec<String>,
    
    /// Rule expression (simplified for now)
    pub condition: serde_json::Value,
}

/// Work product schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkProductSchema {
    /// Required sections
    pub required_sections: Vec<String>,
    
    /// Optional sections
    pub optional_sections: Vec<String>,
    
    /// Minimum quality score
    pub min_quality_score: Option<f32>,
    
    /// Required metadata fields
    pub required_metadata: Vec<String>,
    
    /// Validation rules for this work product type
    pub validation_rules: Vec<String>,
}

/// GPS-specific elements (for GPS methodology)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPSElements {
    /// Reasonably exhaustive research
    pub exhaustive_research: RequirementSpec,
    
    /// Complete and accurate citations
    pub complete_citations: RequirementSpec,
    
    /// Analysis and correlation
    pub analysis_correlation: RequirementSpec,
    
    /// Resolution of conflicts
    pub conflict_resolution: RequirementSpec,
    
    /// Sound written conclusion
    pub written_conclusion: RequirementSpec,
}

/// Configuration for a research methodology
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MethodologyConfig {
    /// Methodology metadata
    pub metadata: MethodologyMetadata,
    
    /// Workflow stages
    pub workflow_stages: Vec<WorkflowStage>,
    
    /// Required elements for compliance
    pub required_elements: HashMap<String, RequirementSpec>,
    
    /// Compliance rules
    pub compliance_rules: Vec<ComplianceRule>,
    
    /// Work product schemas by type
    pub work_product_schemas: HashMap<String, WorkProductSchema>,
    
    /// Features to enable when this methodology is active
    pub enabled_features: HashSet<String>,
    
    /// Features to disable when this methodology is active
    pub disabled_features: HashSet<String>,
    
    /// Default templates for this methodology
    pub default_templates: HashMap<String, String>,
    
    /// GPS-specific elements (if this is a GPS methodology)
    pub gps_elements: Option<GPSElements>,
    
    /// Custom configuration data
    pub custom_config: HashMap<String, serde_json::Value>,
}

impl MethodologyConfig {
    /// Creates a new methodology configuration
    pub fn new(
        methodology_key: String,
        name: String,
        version: String,
        authority: String,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: MethodologyMetadata {
                id: MethodologyConfigId::new(),
                methodology_key,
                name,
                version,
                authority,
                created_at: now,
                updated_at: now,
                source: ConfigSource::UserDefined { created_at: now },
            },
            workflow_stages: Vec::new(),
            required_elements: HashMap::new(),
            compliance_rules: Vec::new(),
            work_product_schemas: HashMap::new(),
            enabled_features: HashSet::new(),
            disabled_features: HashSet::new(),
            default_templates: HashMap::new(),
            gps_elements: None,
            custom_config: HashMap::new(),
        }
    }
    
    /// Creates a basic GPS methodology configuration
    pub fn create_gps_2021() -> Self {
        let mut config = Self::new(
            "gps-2021".to_string(),
            "Genealogical Proof Standard (2021)".to_string(),
            "2021.1".to_string(),
            "Board for Certification of Genealogists".to_string(),
        );
        
        // Add GPS elements
        config.gps_elements = Some(GPSElements {
            exhaustive_research: RequirementSpec {
                requirement_id: "gps-exhaustive".to_string(),
                name: "Reasonably Exhaustive Research".to_string(),
                description: "Search all viable sources".to_string(),
                mandatory: true,
                validation_rules: vec!["rule-exhaustive-research".to_string()],
            },
            complete_citations: RequirementSpec {
                requirement_id: "gps-citations".to_string(),
                name: "Complete and Accurate Citations".to_string(),
                description: "Document all sources completely".to_string(),
                mandatory: true,
                validation_rules: vec!["rule-complete-citations".to_string()],
            },
            analysis_correlation: RequirementSpec {
                requirement_id: "gps-analysis".to_string(),
                name: "Analysis and Correlation".to_string(),
                description: "Analyze and correlate all evidence".to_string(),
                mandatory: true,
                validation_rules: vec!["rule-analysis-correlation".to_string()],
            },
            conflict_resolution: RequirementSpec {
                requirement_id: "gps-conflicts".to_string(),
                name: "Resolution of Conflicts".to_string(),
                description: "Resolve any conflicting evidence".to_string(),
                mandatory: true,
                validation_rules: vec!["rule-conflict-resolution".to_string()],
            },
            written_conclusion: RequirementSpec {
                requirement_id: "gps-conclusion".to_string(),
                name: "Sound Written Conclusion".to_string(),
                description: "Write a coherent conclusion".to_string(),
                mandatory: true,
                validation_rules: vec!["rule-written-conclusion".to_string()],
            },
        });
        
        // Add required elements
        if let Some(ref gps) = config.gps_elements {
            config.required_elements.insert(
                "exhaustive_research".to_string(),
                gps.exhaustive_research.clone()
            );
            config.required_elements.insert(
                "complete_citations".to_string(),
                gps.complete_citations.clone()
            );
            config.required_elements.insert(
                "analysis_correlation".to_string(),
                gps.analysis_correlation.clone()
            );
            config.required_elements.insert(
                "conflict_resolution".to_string(),
                gps.conflict_resolution.clone()
            );
            config.required_elements.insert(
                "written_conclusion".to_string(),
                gps.written_conclusion.clone()
            );
        }
        
        // Add workflow stages
        config.workflow_stages = vec![
            WorkflowStage {
                stage_id: "research".to_string(),
                name: "Research".to_string(),
                description: "Conduct reasonably exhaustive research".to_string(),
                required_work_products: vec!["research_log".to_string()],
                stage_rules: vec!["rule-exhaustive-research".to_string()],
                next_stages: vec!["analysis".to_string()],
            },
            WorkflowStage {
                stage_id: "analysis".to_string(),
                name: "Analysis".to_string(),
                description: "Analyze and correlate evidence".to_string(),
                required_work_products: vec!["evidence_analysis".to_string()],
                stage_rules: vec!["rule-analysis-correlation".to_string()],
                next_stages: vec!["conclusion".to_string()],
            },
            WorkflowStage {
                stage_id: "conclusion".to_string(),
                name: "Conclusion".to_string(),
                description: "Write proof statement".to_string(),
                required_work_products: vec!["proof_statement".to_string()],
                stage_rules: vec!["rule-written-conclusion".to_string()],
                next_stages: vec![],
            },
        ];
        
        // Enable GPS-specific features
        config.enabled_features.insert("gps_compliance_tracking".to_string());
        config.enabled_features.insert("proof_statement_generation".to_string());
        config.enabled_features.insert("evidence_analysis_matrix".to_string());
        
        config
    }
    
    /// Gets a requirement by ID
    pub fn get_requirement(&self, requirement_id: &str) -> Option<&RequirementSpec> {
        self.required_elements.values()
            .find(|r| r.requirement_id == requirement_id)
    }
    
    /// Gets all validation rule IDs
    pub fn get_all_validation_rules(&self) -> HashSet<String> {
        let mut rules = HashSet::new();
        
        // From requirements
        for requirement in self.required_elements.values() {
            rules.extend(requirement.validation_rules.iter().cloned());
        }
        
        // From workflow stages
        for stage in &self.workflow_stages {
            rules.extend(stage.stage_rules.iter().cloned());
        }
        
        // From compliance rules
        for rule in &self.compliance_rules {
            rules.insert(rule.rule_id.clone());
        }
        
        rules
    }
    
    /// Checks if a feature should be enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.enabled_features.contains(feature) && !self.disabled_features.contains(feature)
    }
}

impl ConfigEntity for MethodologyConfig {
    fn id(&self) -> EntityId {
        self.metadata.id.clone()
    }
    
    fn config_type(&self) -> &'static str {
        "MethodologyConfig"
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn validate(&self) -> Result<()> {
        let result = self.validate_config();
        if result.is_valid() {
            Ok(())
        } else {
            Err(Error::ValidationError("Configuration validation failed".to_string()))
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Validatable for MethodologyConfig {
    async fn validate(&self) -> ValidationResult {
        self.validate_config()
    }
}

impl MethodologyConfig {
    fn validate_config(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Validate metadata
        if self.metadata.methodology_key.trim().is_empty() {
            result.add_error(
                "methodology_key",
                "required",
                "Methodology key cannot be empty"
            );
        }
        
        if self.metadata.name.trim().is_empty() {
            result.add_error(
                "name",
                "required",
                "Methodology name cannot be empty"
            );
        }
        
        if self.metadata.version.trim().is_empty() {
            result.add_error(
                "version",
                "required",
                "Version cannot be empty"
            );
        }
        
        // Validate workflow stages
        let stage_ids: HashSet<_> = self.workflow_stages.iter()
            .map(|s| &s.stage_id)
            .collect();
        
        for stage in &self.workflow_stages {
            // Check next stages exist
            for next in &stage.next_stages {
                if !stage_ids.contains(next) {
                    result.add_error(
                        "workflow_stages",
                        "invalid_reference",
                        format!("Stage '{}' references non-existent next stage '{}'", 
                                       stage.stage_id, next)
                    );
                }
            }
        }
        
        // Validate GPS elements if present
        if let Some(ref _gps) = self.gps_elements {
            // GPS must have all five elements in required_elements
            let gps_keys = ["exhaustive_research", "complete_citations", 
                           "analysis_correlation", "conflict_resolution", 
                           "written_conclusion"];
            
            for key in &gps_keys {
                if !self.required_elements.contains_key(*key) {
                    result.add_error(
                        "gps_elements",
                        "missing_element",
                        format!("GPS methodology missing required element '{}'", key)
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
    fn test_methodology_creation() {
        let config = MethodologyConfig::new(
            "custom-2025".to_string(),
            "Custom Methodology".to_string(),
            "1.0.0".to_string(),
            "My Organization".to_string(),
        );
        
        assert_eq!(config.metadata.methodology_key, "custom-2025");
        assert_eq!(config.metadata.name, "Custom Methodology");
        assert_eq!(config.metadata.version, "1.0.0");
        assert!(config.workflow_stages.is_empty());
        assert!(config.required_elements.is_empty());
    }
    
    #[test]
    fn test_gps_methodology_creation() {
        let config = MethodologyConfig::create_gps_2021();
        
        assert_eq!(config.metadata.methodology_key, "gps-2021");
        assert!(config.gps_elements.is_some());
        assert_eq!(config.required_elements.len(), 5);
        assert_eq!(config.workflow_stages.len(), 3);
        assert!(config.enabled_features.contains("gps_compliance_tracking"));
    }
    
    #[test]
    fn test_requirement_lookup() {
        let config = MethodologyConfig::create_gps_2021();
        
        let req = config.get_requirement("gps-exhaustive");
        assert!(req.is_some());
        assert_eq!(req.unwrap().name, "Reasonably Exhaustive Research");
        
        assert!(config.get_requirement("non-existent").is_none());
    }
    
    #[test]
    fn test_validation_rule_collection() {
        let config = MethodologyConfig::create_gps_2021();
        let rules = config.get_all_validation_rules();
        
        assert!(rules.contains("rule-exhaustive-research"));
        assert!(rules.contains("rule-complete-citations"));
        assert!(rules.contains("rule-analysis-correlation"));
        assert!(rules.contains("rule-conflict-resolution"));
        assert!(rules.contains("rule-written-conclusion"));
    }
    
    #[test]
    fn test_feature_management() {
        let mut config = MethodologyConfig::new(
            "test".to_string(),
            "Test".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
        );
        
        config.enabled_features.insert("feature1".to_string());
        config.enabled_features.insert("feature2".to_string());
        config.disabled_features.insert("feature2".to_string());
        
        assert!(config.is_feature_enabled("feature1"));
        assert!(!config.is_feature_enabled("feature2")); // Disabled overrides enabled
        assert!(!config.is_feature_enabled("feature3")); // Not in either set
    }
    
    #[tokio::test]
    async fn test_methodology_validation() {
        let mut config = MethodologyConfig::new(
            "".to_string(), // Invalid empty key
            "Test".to_string(),
            "1.0".to_string(),
            "Test".to_string(),
        );
        
        let result = <MethodologyConfig as Validatable>::validate(&config).await;
        assert!(!result.is_valid());
        
        // Fix the key
        config.metadata.methodology_key = "test-2025".to_string();
        
        // Add invalid workflow stage
        config.workflow_stages.push(WorkflowStage {
            stage_id: "stage1".to_string(),
            name: "Stage 1".to_string(),
            description: "Test stage".to_string(),
            required_work_products: vec![],
            stage_rules: vec![],
            next_stages: vec!["non-existent".to_string()], // Invalid reference
        });
        
        let result = <MethodologyConfig as Validatable>::validate(&config).await;
        assert!(!result.is_valid());
    }
}