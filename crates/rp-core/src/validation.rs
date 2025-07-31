//! Validation framework for entities

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::ValidationErrors;

use crate::{Error, Result};

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
    pub fn to_error(&self) -> Result<()> {
        if self.is_valid() {
            Ok(())
        } else {
            let errors: Vec<String> = self.issues
                .iter()
                .filter(|i| i.severity == ValidationSeverity::Error)
                .map(|i| format!("{}: {}", i.field, i.message))
                .collect();
            Err(Error::ValidationError(errors.join("; ")))
        }
    }
}

/// Trait for types that can be validated
#[async_trait]
pub trait Validatable: Send + Sync {
    /// Perform validation
    async fn validate(&self) -> ValidationResult;
    
    /// Validate and return an error if invalid
    async fn validate_strict(&self) -> Result<()> {
        self.validate().await.to_error()
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
                .expect("ValidationResult value serialization should never fail"),
        );
        self
    }
}

/// Trait for validators that need context
#[async_trait]
pub trait ContextualValidator: Send + Sync {
    type Target;
    
    /// Validate with context
    async fn validate_with_context(
        &self,
        target: &Self::Target,
        context: &ValidationContext,
    ) -> ValidationResult;
}

/// Convert from validator crate errors
impl From<ValidationErrors> for ValidationResult {
    fn from(errors: ValidationErrors) -> Self {
        let mut result = ValidationResult::new();
        
        for (field, errors) in errors.field_errors() {
            for error in errors {
                result.add_error(
                    field,
                    error.code.to_string(),
                    error.message.as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Validation failed for {}", field)), // Default message when error has none
                );
            }
        }
        
        result
    }
}

/// Helper macro to implement Validatable using the validator crate
#[macro_export]
macro_rules! impl_validatable {
    ($type:ty) => {
        #[async_trait::async_trait]
        impl $crate::validation::Validatable for $type {
            async fn validate(&self) -> $crate::validation::ValidationResult {
                match <$type as validator::Validate>::validate(self) {
                    Ok(_) => $crate::validation::ValidationResult::new(),
                    Err(e) => e.into(),
                }
            }
        }
    };
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
}

/// Validation error type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub issues: Vec<ValidationIssue>,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let errors: Vec<String> = self.issues
            .iter()
            .map(|i| format!("{}: {}", i.field, i.message))
            .collect();
        write!(f, "Validation failed: {}", errors.join("; "))
    }
}

impl std::error::Error for ValidationError {}