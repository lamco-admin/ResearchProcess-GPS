// Configuration - Settings and configurations for workspaces

use super::ConfigurationId;
use crate::layer1::PropertyGraph;
use serde::{Serialize, Deserialize};

/// Configuration primitive - ANY configuration/settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// Unique identifier
    pub id: ConfigurationId,

    /// Open-ended configuration type
    /// Examples: "Settings", "Preferences", "Schema", "Rules"
    pub config_type: String,

    /// Configuration schema
    pub schema: Schema,

    /// Configuration values
    pub values: PropertyGraph,

    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

/// Schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub schema_type: String,
    pub definition: PropertyGraph,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: String,
    pub expression: String,
    pub message: String,
}

impl Configuration {
    /// Create a new configuration
    pub fn new(config_type: impl Into<String>) -> Self {
        Configuration {
            id: ConfigurationId::new(),
            config_type: config_type.into(),
            schema: Schema {
                schema_type: "Generic".to_string(),
                definition: PropertyGraph::new(),
            },
            values: PropertyGraph::new(),
            validation: Vec::new(),
        }
    }

    /// Add a validation rule
    pub fn add_validation(&mut self, rule: ValidationRule) {
        self.validation.push(rule);
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        for rule in &self.validation {
            // In real implementation, would evaluate the expression
            // For now, just placeholder
            if rule.expression == "false" {
                errors.push(rule.message.clone());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}