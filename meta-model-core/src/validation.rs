// Validation framework for meta-model entities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validation severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// A single validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub field: String,
    pub code: String,
    pub message: String,
    pub severity: ValidationSeverity,
}

/// Validation result containing all issues
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    /// Create a new empty validation result
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == ValidationSeverity::Error)
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == ValidationSeverity::Warning)
    }

    /// Add an error
    pub fn add_error(&mut self, field: impl Into<String>, code: impl Into<String>, message: impl Into<String>) {
        self.issues.push(ValidationIssue {
            field: field.into(),
            code: code.into(),
            message: message.into(),
            severity: ValidationSeverity::Error,
        });
    }

    /// Add a warning
    pub fn add_warning(&mut self, field: impl Into<String>, code: impl Into<String>, message: impl Into<String>) {
        self.issues.push(ValidationIssue {
            field: field.into(),
            code: code.into(),
            message: message.into(),
            severity: ValidationSeverity::Warning,
        });
    }

    /// Add an info message
    pub fn add_info(&mut self, field: impl Into<String>, code: impl Into<String>, message: impl Into<String>) {
        self.issues.push(ValidationIssue {
            field: field.into(),
            code: code.into(),
            message: message.into(),
            severity: ValidationSeverity::Info,
        });
    }

    /// Merge another validation result into this one
    pub fn merge(&mut self, other: ValidationResult) {
        self.issues.extend(other.issues);
    }

    /// Convert to an error if invalid
    pub fn to_error(&self) -> Result<(), String> {
        if self.is_valid() {
            Ok(())
        } else {
            let errors: Vec<String> = self.issues
                .iter()
                .filter(|i| i.severity == ValidationSeverity::Error)
                .map(|i| format!("{}: {}", i.field, i.message))
                .collect();
            Err(format!("Validation failed: {}", errors.join("; ")))
        }
    }
}

/// Trait for types that can be validated
pub trait Validatable: Send + Sync {
    /// Perform validation
    fn validate(&self) -> ValidationResult;

    /// Validate and return an error if invalid
    fn validate_strict(&self) -> Result<(), String> {
        self.validate().to_error()
    }
}

/// Validation context for complex validations
#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub values: HashMap<String, serde_json::Value>,
}

impl ValidationContext {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn with_value(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.values.insert(
            key.into(),
            serde_json::to_value(value)
                .expect("ValidationContext value serialization should never fail"),
        );
        self
    }
}

/// Trait for validators that need context
pub trait ContextualValidator: Send + Sync {
    type Target;

    /// Validate with context
    fn validate_with_context(
        &self,
        target: &Self::Target,
        context: &ValidationContext,
    ) -> ValidationResult;
}

/// Common validation functions
pub mod validators {
    use super::*;

    /// Validate that a string is not empty
    pub fn not_empty(value: &str, field: &str) -> Option<ValidationIssue> {
        if value.trim().is_empty() {
            Some(ValidationIssue {
                field: field.to_string(),
                code: "empty".to_string(),
                message: format!("{} cannot be empty", field),
                severity: ValidationSeverity::Error,
            })
        } else {
            None
        }
    }

    /// Validate string length
    pub fn string_length(value: &str, field: &str, min: Option<usize>, max: Option<usize>) -> Option<ValidationIssue> {
        let len = value.len();

        if let Some(min) = min {
            if len < min {
                return Some(ValidationIssue {
                    field: field.to_string(),
                    code: "min_length".to_string(),
                    message: format!("{} must be at least {} characters", field, min),
                    severity: ValidationSeverity::Error,
                });
            }
        }

        if let Some(max) = max {
            if len > max {
                return Some(ValidationIssue {
                    field: field.to_string(),
                    code: "max_length".to_string(),
                    message: format!("{} must be at most {} characters", field, max),
                    severity: ValidationSeverity::Error,
                });
            }
        }

        None
    }

    /// Validate that a list is not empty
    pub fn list_not_empty<T>(value: &[T], field: &str) -> Option<ValidationIssue> {
        if value.is_empty() {
            Some(ValidationIssue {
                field: field.to_string(),
                code: "empty_list".to_string(),
                message: format!("{} cannot be empty", field),
                severity: ValidationSeverity::Error,
            })
        } else {
            None
        }
    }

    /// Validate property graph keys
    pub fn valid_property_keys(properties: &crate::layer1::PropertyGraph, field: &str) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        for key in properties.0.keys() {
            if key.trim().is_empty() {
                issues.push(ValidationIssue {
                    field: format!("{}.{}", field, key),
                    code: "invalid_key".to_string(),
                    message: "Property key cannot be empty".to_string(),
                    severity: ValidationSeverity::Error,
                });
            }

            if key.contains(char::is_control) {
                issues.push(ValidationIssue {
                    field: format!("{}.{}", field, key),
                    code: "invalid_key".to_string(),
                    message: "Property key cannot contain control characters".to_string(),
                    severity: ValidationSeverity::Error,
                });
            }
        }

        issues
    }
}

/// Validation pipeline for complex validation scenarios
pub struct ValidationPipeline<T> {
    validators: Vec<Box<dyn Fn(&T) -> ValidationResult + Send + Sync>>,
}

impl<T> ValidationPipeline<T> {
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn add_validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&T) -> ValidationResult + Send + Sync + 'static,
    {
        self.validators.push(Box::new(validator));
        self
    }

    pub fn validate(&self, target: &T) -> ValidationResult {
        let mut result = ValidationResult::new();

        for validator in &self.validators {
            result.merge(validator(target));
        }

        result
    }
}

// Implement validation for core entities

use crate::layer1::{Entity, Relationship, Context, Certainty};

impl Validatable for Entity {
    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate entity type
        if let Some(issue) = validators::not_empty(&self.entity_type, "entity_type") {
            result.issues.push(issue);
        }

        // Validate state
        if let Some(issue) = validators::not_empty(&self.state, "state") {
            result.issues.push(issue);
        }

        // Validate properties
        result.issues.extend(validators::valid_property_keys(&self.properties, "properties"));

        // Warn if no contexts
        if self.contexts.is_empty() {
            result.add_warning("contexts", "no_context", "Entity has no contexts - consider adding temporal or source context");
        }

        result
    }
}

impl Validatable for Relationship {
    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate relationship type
        if let Some(issue) = validators::not_empty(&self.relationship_type, "relationship_type") {
            result.issues.push(issue);
        }

        // Validate direction
        if let Some(issue) = validators::not_empty(&self.direction, "direction") {
            result.issues.push(issue);
        }

        // Validate properties
        result.issues.extend(validators::valid_property_keys(&self.properties, "properties"));

        // Check for self-relationship
        if self.from == self.to {
            result.add_info("relationship", "self_relationship", "This is a self-referential relationship");
        }

        result
    }
}

impl Validatable for Context {
    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate context type
        if let Some(issue) = validators::not_empty(&self.context_type, "context_type") {
            result.issues.push(issue);
        }

        // Validate properties
        result.issues.extend(validators::valid_property_keys(&self.properties, "properties"));

        result
    }
}

impl Validatable for Certainty {
    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate confidence
        if self.confidence < 0.0 || self.confidence > 1.0 {
            result.add_error("confidence", "out_of_range", "Confidence must be between 0.0 and 1.0");
        }

        // Validate sources
        if self.sources.is_empty() {
            result.add_warning("sources", "no_sources", "Certainty has no supporting sources");
        }

        result
    }
}