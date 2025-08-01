// Context - Provides scope and constraints

use super::{ContextId, TemporalValue, Certainty, PropertyGraph};
use serde::{Serialize, Deserialize};

/// Context that scopes entities and relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Unique identifier
    pub id: ContextId,
    
    /// Open-ended context type
    /// Examples: "Temporal", "Geographic", "Cultural", "Research", "Hypothetical"
    pub context_type: String,
    
    /// The scope this context provides
    pub scope: Scope,
    
    /// Certainty of this context
    pub certainty: Certainty,
    
    /// Additional context properties
    pub properties: PropertyGraph,
}

/// Different types of scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Scope {
    /// Temporal scope - when
    Temporal(TemporalScope),
    
    /// Geographic scope - where
    Geographic(GeographicScope),
    
    /// Cultural scope - cultural context
    Cultural(CulturalScope),
    
    /// Research scope - research context
    Research(ResearchScope),
    
    /// Theoretical scope - hypothetical context
    Theoretical(TheoreticalScope),
    
    /// Generic scope
    Generic {
        scope_type: String,
        parameters: serde_json::Value,
    },
}

/// Temporal scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalScope {
    pub description: String,
    pub bounds: Option<(TemporalValue, TemporalValue)>,
}

/// Geographic scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicScope {
    pub description: String,
    pub location_type: String, // "Point", "Area", "Region", etc.
    pub coordinates: Option<Vec<f64>>,
    pub place_names: Vec<PlaceName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceName {
    pub name: String,
    pub name_type: String, // "Current", "Historical", "Vernacular"
    pub temporal_scope: Option<TemporalScope>,
}

/// Cultural scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalScope {
    pub culture: String,
    pub aspects: Vec<String>, // ["naming", "calendar", "kinship"]
    pub description: String,
}

/// Research scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchScope {
    pub research_context: String,
    pub methodology: String,
    pub limitations: Vec<String>,
    pub assumptions: Vec<String>,
}

/// Theoretical scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoreticalScope {
    pub theory: String,
    pub assumptions: Vec<String>,
}

impl Context {
    /// Create a temporal context
    pub fn temporal(description: impl Into<String>, start: TemporalValue, end: TemporalValue) -> Self {
        Context {
            id: ContextId::new(),
            context_type: "Temporal".to_string(),
            scope: Scope::Temporal(TemporalScope {
                description: description.into(),
                bounds: Some((start, end)),
            }),
            certainty: Certainty::Unknown,
            properties: PropertyGraph::new(),
        }
    }
    
    /// Create a geographic context
    pub fn geographic(description: impl Into<String>, place: impl Into<String>) -> Self {
        Context {
            id: ContextId::new(),
            context_type: "Geographic".to_string(),
            scope: Scope::Geographic(GeographicScope {
                description: description.into(),
                location_type: "Named".to_string(),
                coordinates: None,
                place_names: vec![PlaceName {
                    name: place.into(),
                    name_type: "Current".to_string(),
                    temporal_scope: None,
                }],
            }),
            certainty: Certainty::Unknown,
            properties: PropertyGraph::new(),
        }
    }
    
    /// Create a research context
    pub fn research(methodology: impl Into<String>) -> Self {
        Context {
            id: ContextId::new(),
            context_type: "Research".to_string(),
            scope: Scope::Research(ResearchScope {
                research_context: "Active research".to_string(),
                methodology: methodology.into(),
                limitations: Vec::new(),
                assumptions: Vec::new(),
            }),
            certainty: Certainty::Unknown,
            properties: PropertyGraph::new(),
        }
    }
    
    /// Create a theoretical context
    pub fn theoretical(theory: impl Into<String>, assumptions: Vec<String>) -> Self {
        Context {
            id: ContextId::new(),
            context_type: "Theoretical".to_string(),
            scope: Scope::Theoretical(TheoreticalScope {
                theory: theory.into(),
                assumptions,
            }),
            certainty: Certainty::Unknown,
            properties: PropertyGraph::new(),
        }
    }
}