//! ValidationRule entity - Configurable validation rules
//! 
//! Validation rules are defined as data expressions, allowing methodologies
//! to specify their requirements without code changes.

use crate::{
    validation::{Validatable, ValidationResult, ValidationIssue, ValidationSeverity},
    EntityId, Error, Result,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use validator::Validate;

use super::{
    ConfigEntity, ConfigSource, ValidationRuleId, RuleType, Severity, EntityType,
};

/// Validation rule metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRuleMetadata {
    /// Unique ID for this rule
    pub id: ValidationRuleId,
    
    /// Rule identifier (e.g., "gps-exhaustive-research")
    pub rule_key: String,
    
    /// Display name
    pub name: String,
    
    /// Rule version
    pub version: String,
    
    /// Rule author
    pub author: String,
    
    /// When this rule was created
    pub created_at: DateTime<Utc>,
    
    /// When this rule was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Source of this rule
    pub source: ConfigSource,
}

/// Parameter definition for rule expressions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDef {
    /// Parameter name
    pub name: String,
    
    /// Data type
    pub param_type: String,
    
    /// Description
    pub description: String,
    
    /// Whether required
    pub required: bool,
    
    /// Default value
    pub default_value: Option<serde_json::Value>,
}

/// Comparison operators for rules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOp {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanOrEqual,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "matches")]
    Matches,
}

/// Rule expressions for validation logic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RuleExpression {
    /// Field must be present
    Required {
        field: String,
    },
    
    /// Field must match pattern
    Pattern {
        field: String,
        regex: String,
    },
    
    /// Field length constraints
    Length {
        field: String,
        min: Option<usize>,
        max: Option<usize>,
    },
    
    /// Logical AND
    And {
        conditions: Vec<RuleExpression>,
    },
    
    /// Logical OR
    Or {
        conditions: Vec<RuleExpression>,
    },
    
    /// Logical NOT
    Not {
        condition: Box<RuleExpression>,
    },
    
    /// Comparison operations
    Compare {
        field: String,
        op: ComparisonOp,
        value: serde_json::Value,
    },
    
    /// Check if value is in list
    In {
        field: String,
        values: Vec<serde_json::Value>,
    },
    
    /// Cross-entity existence check
    Exists {
        entity_type: String,
        where_clause: Box<RuleExpression>,
    },
    
    /// Count entities matching criteria
    Count {
        entity_type: String,
        where_clause: Option<Box<RuleExpression>>,
        op: ComparisonOp,
        value: i32,
    },
    
    /// All items in collection must match
    All {
        collection: String,
        condition: Box<RuleExpression>,
    },
    
    /// Any item in collection must match
    Any {
        collection: String,
        condition: Box<RuleExpression>,
    },
    
    /// Custom validation function
    Custom {
        function: String,
        args: HashMap<String, serde_json::Value>,
    },
}

/// Context for rule evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleContext {
    /// Current methodology
    pub methodology: String,
    
    /// Current workspace
    pub workspace_id: EntityId,
    
    /// Additional context variables
    pub variables: HashMap<String, serde_json::Value>,
}

/// Configurable validation rule
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ValidationRule {
    /// Rule metadata
    pub metadata: ValidationRuleMetadata,
    
    /// Brief description
    pub description: String,
    
    /// Rule type
    pub rule_type: RuleType,
    
    /// Rule category (e.g., "gps", "bcg", "quality")
    pub category: String,
    
    /// Severity level
    pub severity: Severity,
    
    /// Entity types this rule applies to
    pub applies_to: Vec<EntityType>,
    
    /// Methodologies this rule belongs to
    pub methodology_scope: Vec<String>,
    
    /// The rule expression
    pub condition: RuleExpression,
    
    /// Parameters used in the rule
    pub parameters: HashMap<String, ParameterDef>,
    
    /// Error message template
    pub message_template: String,
    
    /// Remediation hint
    pub remediation_hint: Option<String>,
    
    /// Related documentation
    pub documentation_url: Option<String>,
    
    /// Whether this rule is enabled by default
    pub enabled_by_default: bool,
    
    /// Tags for organization
    pub tags: HashSet<String>,
    
    /// Custom metadata
    pub custom_metadata: HashMap<String, serde_json::Value>,
}

impl ValidationRule {
    /// Creates a new validation rule
    pub fn new(
        rule_key: String,
        name: String,
        version: String,
        author: String,
        rule_type: RuleType,
        severity: Severity,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: ValidationRuleMetadata {
                id: ValidationRuleId::new(),
                rule_key,
                name,
                version,
                author,
                created_at: now,
                updated_at: now,
                source: ConfigSource::UserDefined { created_at: now },
            },
            description: String::new(),
            rule_type,
            category: String::new(),
            severity,
            applies_to: Vec::new(),
            methodology_scope: Vec::new(),
            condition: RuleExpression::Required { field: String::new() },
            parameters: HashMap::new(),
            message_template: String::new(),
            remediation_hint: None,
            documentation_url: None,
            enabled_by_default: true,
            tags: HashSet::new(),
            custom_metadata: HashMap::new(),
        }
    }
    
    /// Creates an example GPS exhaustive research rule
    pub fn create_gps_exhaustive_research_example() -> Self {
        let mut rule = Self::new(
            "gps-exhaustive-research".to_string(),
            "GPS Exhaustive Research Requirement".to_string(),
            "1.0.0".to_string(),
            "ResearchProcess-GPS".to_string(),
            RuleType::Compliance,
            Severity::Error,
        );
        
        rule.description = "Ensures reasonably exhaustive research per GPS standard".to_string();
        rule.category = "gps".to_string();
        rule.applies_to = vec![EntityType::ProofStatement];
        rule.methodology_scope = vec!["gps-2021".to_string()];
        
        // Complex rule: Must have at least 3 sources and evidence from 2+ repositories
        rule.condition = RuleExpression::And {
            conditions: vec![
                RuleExpression::Count {
                    entity_type: "Source".to_string(),
                    where_clause: Some(Box::new(RuleExpression::Compare {
                        field: "linked_to_proof".to_string(),
                        op: ComparisonOp::Equal,
                        value: serde_json::json!("{{entity_id}}"),
                    })),
                    op: ComparisonOp::GreaterThanOrEqual,
                    value: 3,
                },
                RuleExpression::Count {
                    entity_type: "Repository".to_string(),
                    where_clause: Some(Box::new(RuleExpression::Exists {
                        entity_type: "Source".to_string(),
                        where_clause: Box::new(RuleExpression::And {
                            conditions: vec![
                                RuleExpression::Compare {
                                    field: "repository_id".to_string(),
                                    op: ComparisonOp::Equal,
                                    value: serde_json::json!("{{repository.id}}"),
                                },
                                RuleExpression::Compare {
                                    field: "linked_to_proof".to_string(),
                                    op: ComparisonOp::Equal,
                                    value: serde_json::json!("{{entity_id}}"),
                                },
                            ],
                        }),
                    })),
                    op: ComparisonOp::GreaterThanOrEqual,
                    value: 2,
                },
            ],
        };
        
        rule.message_template = "GPS requires reasonably exhaustive research. Found {{source_count}} sources from {{repository_count}} repositories. Minimum required: 3 sources from 2+ repositories.".to_string();
        rule.remediation_hint = Some("Search additional repositories and document negative searches to demonstrate exhaustive research".to_string());
        rule.documentation_url = Some("https://bcgcertification.org/gps-standard#exhaustive-research".to_string());
        
        rule.tags.insert("gps".to_string());
        rule.tags.insert("core-requirement".to_string());
        
        rule
    }
    
    /// Creates an example field requirement rule
    pub fn create_required_field_example() -> Self {
        let mut rule = Self::new(
            "theory-question-required".to_string(),
            "Theory Question Required".to_string(),
            "1.0.0".to_string(),
            "ResearchProcess-GPS".to_string(),
            RuleType::RequiredField,
            Severity::Error,
        );
        
        rule.description = "Theory must have a research question".to_string();
        rule.category = "data-quality".to_string();
        rule.applies_to = vec![EntityType::Theory];
        
        rule.condition = RuleExpression::And {
            conditions: vec![
                RuleExpression::Required {
                    field: "question".to_string(),
                },
                RuleExpression::Length {
                    field: "question".to_string(),
                    min: Some(10),
                    max: None,
                },
            ],
        };
        
        rule.message_template = "Theory research question is required and must be at least 10 characters".to_string();
        rule.remediation_hint = Some("Add a clear, specific research question to the theory".to_string());
        
        rule
    }
    
    /// Creates an example format validation rule
    pub fn create_format_validation_example() -> Self {
        let mut rule = Self::new(
            "valid-email-format".to_string(),
            "Valid Email Format".to_string(),
            "1.0.0".to_string(),
            "ResearchProcess-GPS".to_string(),
            RuleType::Format,
            Severity::Warning,
        );
        
        rule.description = "Email addresses must be in valid format".to_string();
        rule.category = "data-quality".to_string();
        rule.applies_to = vec![EntityType::Researcher];
        
        rule.condition = RuleExpression::Pattern {
            field: "email".to_string(),
            regex: r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$".to_string(),
        };
        
        rule.message_template = "Email address '{{email}}' is not in valid format".to_string();
        rule.remediation_hint = Some("Use format: user@example.com".to_string());
        
        rule
    }
    
    /// Checks if rule applies to an entity type
    pub fn applies_to_entity(&self, entity_type: &EntityType) -> bool {
        self.applies_to.is_empty() || self.applies_to.contains(entity_type)
    }
    
    /// Checks if rule applies to a methodology
    pub fn applies_to_methodology(&self, methodology: &str) -> bool {
        self.methodology_scope.is_empty() || 
        self.methodology_scope.contains(&methodology.to_string())
    }
    
    /// Validates the rule expression is well-formed
    pub fn validate_expression(&self) -> Result<()> {
        self.validate_expression_recursive(&self.condition, 0)
    }
    
    fn validate_expression_recursive(&self, expr: &RuleExpression, depth: usize) -> Result<()> {
        if depth > 10 {
            return Err(Error::ConfigurationError("Rule expression too deeply nested".to_string()));
        }
        
        match expr {
            RuleExpression::Required { field } => {
                if field.is_empty() {
                    return Err(Error::ConfigurationError("Required field name cannot be empty".to_string()));
                }
            }
            RuleExpression::Pattern { field, regex } => {
                if field.is_empty() {
                    return Err(Error::ConfigurationError("Pattern field name cannot be empty".to_string()));
                }
                // Try to compile regex
                if regex::Regex::new(regex).is_err() {
                    return Err(Error::ConfigurationError(format!("Invalid regex pattern: {}", regex)));
                }
            }
            RuleExpression::And { conditions } | RuleExpression::Or { conditions } => {
                if conditions.is_empty() {
                    return Err(Error::ConfigurationError("Logical expression must have conditions".to_string()));
                }
                for condition in conditions {
                    self.validate_expression_recursive(condition, depth + 1)?;
                }
            }
            RuleExpression::Not { condition } => {
                self.validate_expression_recursive(condition, depth + 1)?;
            }
            RuleExpression::Exists { entity_type, where_clause } => {
                if entity_type.is_empty() {
                    return Err(Error::ConfigurationError("Entity type cannot be empty".to_string()));
                }
                self.validate_expression_recursive(where_clause, depth + 1)?;
            }
            RuleExpression::Count { entity_type, where_clause, .. } => {
                if entity_type.is_empty() {
                    return Err(Error::ConfigurationError("Entity type cannot be empty".to_string()));
                }
                if let Some(where_clause) = where_clause {
                    self.validate_expression_recursive(where_clause, depth + 1)?;
                }
            }
            RuleExpression::All { collection, condition } | 
            RuleExpression::Any { collection, condition } => {
                if collection.is_empty() {
                    return Err(Error::ConfigurationError("Collection name cannot be empty".to_string()));
                }
                self.validate_expression_recursive(condition, depth + 1)?;
            }
            _ => {}
        }
        
        Ok(())
    }
}

impl ConfigEntity for ValidationRule {
    fn id(&self) -> EntityId {
        self.metadata.id
    }
    
    fn config_type(&self) -> &'static str {
        "ValidationRule"
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn validate(&self) -> Result<()> {
        if self.metadata.rule_key.is_empty() {
            return Err(Error::ConfigurationError("Rule key cannot be empty".to_string()));
        }
        
        if self.metadata.name.is_empty() {
            return Err(Error::ConfigurationError("Rule name cannot be empty".to_string()));
        }
        
        if self.message_template.is_empty() {
            return Err(Error::ConfigurationError("Message template cannot be empty".to_string()));
        }
        
        // Validate expression
        self.validate_expression()?;
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl Validatable for ValidationRule {
    async fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        
        // Rule key validation
        if self.metadata.rule_key.is_empty() {
            errors.push(ValidationIssue {
                field: "rule_key".to_string(),
                code: "required_field".to_string(),
                message: "Rule key is required".to_string(),
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
        
        // Description validation
        if self.description.is_empty() {
            errors.push(ValidationIssue {
                field: "description".to_string(),
                code: "required_field".to_string(),
                message: "Description is required".to_string(),
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
        
        // Message template validation
        if self.message_template.is_empty() {
            errors.push(ValidationIssue {
                field: "message_template".to_string(),
                code: "required_field".to_string(),
                message: "Message template is required".to_string(),
                severity: ValidationSeverity::Error,
            });
        }
        
        // Validate expression
        if let Err(e) = self.validate_expression() {
            errors.push(ValidationIssue {
                field: "condition".to_string(),
                code: "invalid_format".to_string(),
                message: e.to_string(),
                severity: ValidationSeverity::Error,
            });
        }
        
        ValidationResult { issues: errors }
    }
}

/// Rule set containing multiple validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    /// Rule set identifier
    pub id: String,
    
    /// Name
    pub name: String,
    
    /// Description
    pub description: String,
    
    /// Rules in this set
    pub rules: Vec<ValidationRule>,
    
    /// Whether all rules must pass (AND) or any rule (OR)
    pub require_all: bool,
    
    /// Methodology this set belongs to
    pub methodology: Option<String>,
}

impl RuleSet {
    /// Creates a new rule set
    pub fn new(id: String, name: String, require_all: bool) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            rules: Vec::new(),
            require_all,
            methodology: None,
        }
    }
    
    /// Adds a rule to the set
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.push(rule);
    }
    
    /// Gets rules that apply to an entity type
    pub fn rules_for_entity(&self, entity_type: &EntityType) -> Vec<&ValidationRule> {
        self.rules.iter()
            .filter(|r| r.applies_to_entity(entity_type))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_rule_creation() {
        let rule = ValidationRule::new(
            "test-rule".to_string(),
            "Test Rule".to_string(),
            "1.0.0".to_string(),
            "Test Author".to_string(),
            RuleType::RequiredField,
            Severity::Error,
        );
        
        assert_eq!(rule.metadata.rule_key, "test-rule");
        assert_eq!(rule.metadata.name, "Test Rule");
        assert_eq!(rule.rule_type, RuleType::RequiredField);
        assert_eq!(rule.severity, Severity::Error);
        assert!(rule.enabled_by_default);
    }
    
    #[test]
    fn test_gps_exhaustive_research_rule() {
        let rule = ValidationRule::create_gps_exhaustive_research_example();
        
        assert_eq!(rule.metadata.rule_key, "gps-exhaustive-research");
        assert_eq!(rule.category, "gps");
        assert_eq!(rule.severity, Severity::Error);
        assert!(rule.applies_to.contains(&EntityType::ProofStatement));
        assert!(rule.methodology_scope.contains(&"gps-2021".to_string()));
        
        // Check complex condition
        match &rule.condition {
            RuleExpression::And { conditions } => {
                assert_eq!(conditions.len(), 2);
            }
            _ => panic!("Expected And expression"),
        }
    }
    
    #[test]
    fn test_required_field_rule() {
        let rule = ValidationRule::create_required_field_example();
        
        assert_eq!(rule.rule_type, RuleType::RequiredField);
        assert!(rule.applies_to.contains(&EntityType::Theory));
        
        match &rule.condition {
            RuleExpression::And { conditions } => {
                assert_eq!(conditions.len(), 2);
            }
            _ => panic!("Expected And expression"),
        }
    }
    
    #[test]
    fn test_format_validation_rule() {
        let rule = ValidationRule::create_format_validation_example();
        
        assert_eq!(rule.rule_type, RuleType::Format);
        assert_eq!(rule.severity, Severity::Warning);
        
        match &rule.condition {
            RuleExpression::Pattern { field, regex } => {
                assert_eq!(field, "email");
                assert!(!regex.is_empty());
            }
            _ => panic!("Expected Pattern expression"),
        }
    }
    
    #[test]
    fn test_rule_applicability() {
        let rule = ValidationRule::create_gps_exhaustive_research_example();
        
        assert!(rule.applies_to_entity(&EntityType::ProofStatement));
        assert!(!rule.applies_to_entity(&EntityType::Theory));
        
        assert!(rule.applies_to_methodology("gps-2021"));
        assert!(!rule.applies_to_methodology("bcg-2023"));
        
        // Test rule with empty scopes
        let mut universal_rule = rule.clone();
        universal_rule.applies_to.clear();
        universal_rule.methodology_scope.clear();
        
        assert!(universal_rule.applies_to_entity(&EntityType::Theory));
        assert!(universal_rule.applies_to_methodology("any-methodology"));
    }
    
    #[test]
    fn test_expression_validation() {
        let mut rule = ValidationRule::create_required_field_example();
        
        // Valid expression
        assert!(rule.validate_expression().is_ok());
        
        // Invalid expression - empty field
        rule.condition = RuleExpression::Required { field: String::new() };
        assert!(rule.validate_expression().is_err());
        
        // Invalid expression - bad regex
        rule.condition = RuleExpression::Pattern {
            field: "test".to_string(),
            regex: "[invalid regex".to_string(),
        };
        assert!(rule.validate_expression().is_err());
        
        // Test deep nesting limit
        let mut deeply_nested = RuleExpression::Required { field: "test".to_string() };
        for _ in 0..15 {
            deeply_nested = RuleExpression::Not {
                condition: Box::new(deeply_nested),
            };
        }
        rule.condition = deeply_nested;
        assert!(rule.validate_expression().is_err());
    }
    
    #[test]
    fn test_rule_set() {
        let mut rule_set = RuleSet::new(
            "gps-core".to_string(),
            "GPS Core Rules".to_string(),
            true,
        );
        
        rule_set.add_rule(ValidationRule::create_gps_exhaustive_research_example());
        rule_set.add_rule(ValidationRule::create_required_field_example());
        
        assert_eq!(rule_set.rules.len(), 2);
        
        let proof_rules = rule_set.rules_for_entity(&EntityType::ProofStatement);
        assert_eq!(proof_rules.len(), 1);
        
        let theory_rules = rule_set.rules_for_entity(&EntityType::Theory);
        assert_eq!(theory_rules.len(), 1);
    }
    
    #[tokio::test]
    async fn test_rule_validation() {
        let rule = ValidationRule::create_gps_exhaustive_research_example();
        
        let result = <ValidationRule as Validatable>::validate(&rule).await;
        assert!(result.is_valid());
        
        // Test invalid rule
        let mut invalid_rule = rule.clone();
        invalid_rule.metadata.rule_key = String::new();
        invalid_rule.message_template = String::new();
        
        let result = <ValidationRule as Validatable>::validate(&invalid_rule).await;
        assert!(!result.is_valid());
        assert!(result.issues.len() >= 2);
    }
    
    #[test]
    fn test_comparison_operators() {
        use serde_json::json;
        
        // Test serialization
        let op = ComparisonOp::GreaterThan;
        let serialized = serde_json::to_string(&op).unwrap();
        assert_eq!(serialized, "\">\"");
        
        // Test in expression
        let expr = RuleExpression::Compare {
            field: "count".to_string(),
            op: ComparisonOp::GreaterThanOrEqual,
            value: json!(5),
        };
        
        let serialized = serde_json::to_value(&expr).unwrap();
        assert_eq!(serialized["type"], "compare");
        assert_eq!(serialized["op"], ">=");
    }
}