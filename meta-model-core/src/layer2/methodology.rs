// Methodology - Research methodology definitions

use super::MethodologyId;
use crate::layer1::PropertyGraph;
use serde::{Serialize, Deserialize};

/// Methodology primitive - can express ANY research methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Methodology {
    /// Unique identifier
    pub id: MethodologyId,
    
    /// Open-ended methodology type
    /// Examples: "GPS", "Scientific.Method", "Grounded.Theory", "Design.Thinking"
    pub methodology_type: String,
    
    /// Rules/constraints of this methodology
    pub rules: Vec<Rule>,
    
    /// Quality criteria
    pub quality_criteria: Vec<Criterion>,
    
    /// Compliance checking
    pub compliance: ComplianceFramework,
    
    /// Methodology properties
    pub properties: PropertyGraph,
}

/// Rule in a methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub rule_type: String,
    pub expression: String, // Could be logical expression, natural language, etc.
    pub enforcement: Enforcement,
}

/// Rule enforcement level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Enforcement {
    Required,
    Recommended,
    Optional,
    Conditional(String),
}

/// Quality criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Criterion {
    pub criterion_type: String,
    pub measurement: String,
    pub threshold: String,
}

/// Compliance framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFramework {
    pub framework_type: String,
    pub checks: Vec<ComplianceCheck>,
}

/// Compliance check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub check_type: String,
    pub expression: String,
    pub severity: String,
}

impl Methodology {
    /// Create a new methodology
    pub fn new(methodology_type: impl Into<String>) -> Self {
        Methodology {
            id: MethodologyId::new(),
            methodology_type: methodology_type.into(),
            rules: Vec::new(),
            quality_criteria: Vec::new(),
            compliance: ComplianceFramework {
                framework_type: "Standard".to_string(),
                checks: Vec::new(),
            },
            properties: PropertyGraph::new(),
        }
    }
    
    /// Add a rule
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
    
    /// Add a quality criterion
    pub fn add_criterion(&mut self, criterion: Criterion) {
        self.quality_criteria.push(criterion);
    }
    
    /// Add a compliance check
    pub fn add_compliance_check(&mut self, check: ComplianceCheck) {
        self.compliance.checks.push(check);
    }
    
    /// Get required rules
    pub fn required_rules(&self) -> Vec<&Rule> {
        self.rules
            .iter()
            .filter(|r| matches!(r.enforcement, Enforcement::Required))
            .collect()
    }
}

/// Common methodology builders
impl Methodology {
    /// Create GPS methodology
    pub fn gps() -> Self {
        let mut methodology = Methodology::new("Genealogical.ProofStandard");
        
        // GPS rules
        methodology.add_rule(Rule {
            rule_type: "ExhaustiveSearch".to_string(),
            expression: "All reasonable sources searched".to_string(),
            enforcement: Enforcement::Required,
        });
        
        methodology.add_rule(Rule {
            rule_type: "CompleteCitations".to_string(),
            expression: "All sources fully cited".to_string(),
            enforcement: Enforcement::Required,
        });
        
        methodology.add_rule(Rule {
            rule_type: "EvidenceAnalysis".to_string(),
            expression: "All evidence analyzed and correlated".to_string(),
            enforcement: Enforcement::Required,
        });
        
        methodology.add_rule(Rule {
            rule_type: "ConflictResolution".to_string(),
            expression: "Conflicting evidence resolved".to_string(),
            enforcement: Enforcement::Required,
        });
        
        methodology.add_rule(Rule {
            rule_type: "WrittenConclusion".to_string(),
            expression: "Sound written conclusion".to_string(),
            enforcement: Enforcement::Required,
        });
        
        methodology
    }
    
    /// Create scientific method
    pub fn scientific_method() -> Self {
        let mut methodology = Methodology::new("Scientific.Method");
        
        methodology.add_rule(Rule {
            rule_type: "Falsifiability".to_string(),
            expression: "Hypothesis must be falsifiable".to_string(),
            enforcement: Enforcement::Required,
        });
        
        methodology.add_rule(Rule {
            rule_type: "Reproducibility".to_string(),
            expression: "Experiments must be reproducible".to_string(),
            enforcement: Enforcement::Required,
        });
        
        methodology.add_criterion(Criterion {
            criterion_type: "StatisticalSignificance".to_string(),
            measurement: "p-value".to_string(),
            threshold: "< 0.05".to_string(),
        });
        
        methodology
    }
}

impl Rule {
    /// Create a new rule
    pub fn new(rule_type: impl Into<String>, expression: impl Into<String>) -> Self {
        Rule {
            rule_type: rule_type.into(),
            expression: expression.into(),
            enforcement: Enforcement::Required,
        }
    }
    
    /// Set enforcement level
    pub fn with_enforcement(mut self, enforcement: Enforcement) -> Self {
        self.enforcement = enforcement;
        self
    }
}