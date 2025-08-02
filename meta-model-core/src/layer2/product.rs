// Product - Output of research activities

use super::{ProductId, ActivityId};
use crate::common::{AgentId, MetaInfo};
use crate::layer1::{PropertyGraph, Context, TemporalValue};
use serde::{Serialize, Deserialize};

/// Product primitive - can express ANY research output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// Unique identifier
    pub id: ProductId,

    /// Open-ended product type
    /// Examples: "Report", "Dataset", "Theory", "Evidence", "Conclusion"
    pub product_type: String,

    /// Product state
    pub state: String,

    /// Product maturity
    pub maturity: Maturity,

    /// How this was produced
    pub provenance: Provenance,

    /// Validation/review state
    pub validation: Vec<Validation>,

    /// Product properties
    pub properties: PropertyGraph,

    /// Contexts for this product
    pub contexts: Vec<Context>,

    /// Metadata
    pub meta: MetaInfo,
}

/// Product maturity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Maturity {
    Draft,
    Review,
    Revised,
    Final,
    Published,
    Deprecated,
    Custom(String),
}

/// Provenance tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub created_by: Vec<AgentId>,
    pub created_from: Vec<ResourceReference>,
    pub methods_used: Vec<super::Method>,
    pub timeline: Vec<ProvenanceEvent>,
}

/// Provenance event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEvent {
    pub event_type: String,
    pub timestamp: TemporalValue,
    pub agent: AgentId,
    pub description: String,
}

/// Resource reference (reuse from activity)
use super::activity::ResourceReference;

/// Validation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validation {
    pub validation_type: String,
    pub validator: AgentId,
    pub result: ValidationResult,
    pub feedback: String,
    pub timestamp: TemporalValue,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationResult {
    Passed,
    Failed(Vec<String>),
    Conditional(Vec<String>),
    InProgress,
}

impl Product {
    /// Create a new product
    pub fn new(product_type: impl Into<String>) -> Self {
        Product {
            id: ProductId::new(),
            product_type: product_type.into(),
            state: "Draft".to_string(),
            maturity: Maturity::Draft,
            provenance: Provenance {
                created_by: vec![],
                created_from: vec![],
                methods_used: vec![],
                timeline: vec![],
            },
            validation: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::new(),
        }
    }

    /// Add creator
    pub fn add_creator(&mut self, agent: AgentId) {
        if !self.provenance.created_by.contains(&agent) {
            self.provenance.created_by.push(agent);
        }
    }

    /// Add source
    pub fn add_source(&mut self, source: ResourceReference) {
        self.provenance.created_from.push(source);
    }

    /// Add provenance event
    pub fn add_provenance_event(&mut self, event_type: impl Into<String>, agent: AgentId, description: impl Into<String>) {
        self.provenance.timeline.push(ProvenanceEvent {
            event_type: event_type.into(),
            timestamp: TemporalValue::year(2025), // Simplified
            agent,
            description: description.into(),
        });
    }

    /// Add validation
    pub fn add_validation(&mut self, validation: Validation) {
        self.validation.push(validation);
    }

    /// Set maturity
    pub fn set_maturity(&mut self, maturity: Maturity) {
        self.maturity = maturity;
        self.meta.update(AgentId::system());
    }

    /// Check if validated
    pub fn is_validated(&self) -> bool {
        self.validation.iter().any(|v| matches!(v.result, ValidationResult::Passed))
    }
}

/// Common product builders
impl Product {
    /// Create a research report
    pub fn report(title: impl Into<String>) -> Self {
        let mut product = Product::new("Report");
        product.properties.set_text("title", title);
        product
    }

    /// Create a dataset
    pub fn dataset(name: impl Into<String>) -> Self {
        let mut product = Product::new("Dataset");
        product.properties.set_text("name", name);
        product
    }

    /// Create a theory/hypothesis
    pub fn theory(description: impl Into<String>) -> Self {
        let mut product = Product::new("Theory");
        product.properties.set_text("description", description);
        product
    }

    /// Create evidence
    pub fn evidence(description: impl Into<String>) -> Self {
        let mut product = Product::new("Evidence");
        product.properties.set_text("description", description);
        product
    }

    /// Create a conclusion
    pub fn conclusion(statement: impl Into<String>) -> Self {
        let mut product = Product::new("Conclusion");
        product.properties.set_text("statement", statement);
        product.maturity = Maturity::Final;
        product
    }
}

impl Validation {
    /// Create a new validation
    pub fn new(validation_type: impl Into<String>, validator: AgentId) -> Self {
        Validation {
            validation_type: validation_type.into(),
            validator,
            result: ValidationResult::InProgress,
            feedback: String::new(),
            timestamp: TemporalValue::year(2025), // Simplified
        }
    }

    /// Set as passed
    pub fn pass(mut self) -> Self {
        self.result = ValidationResult::Passed;
        self
    }

    /// Set as failed
    pub fn fail(mut self, reasons: Vec<String>) -> Self {
        self.result = ValidationResult::Failed(reasons);
        self
    }
}