//! Universal context system for qualifying entities and relationships

use serde::{Deserialize, Serialize};

use crate::{
    Certainty, ContextId, MetaInfo, PropertyGraph,
    spatial::SpatialScope,
    temporal::TemporalScope,
};

/// Universal context that can scope/qualify anything
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Context {
    /// Unique identifier
    pub id: ContextId,

    /// Context type (open-ended)
    /// Examples: "Temporal", "Spatial", "Cultural", "Theoretical", "Research", "Source"
    pub context_type: String,

    /// What this context applies to
    pub scope: Scope,

    /// Certainty of this context
    pub certainty: Certainty,

    /// Additional properties
    pub properties: PropertyGraph,

    /// Meta-information
    pub meta: MetaInfo,
}

impl Context {
    /// Create a new context
    pub fn new(context_type: impl Into<String>, scope: Scope) -> Self {
        Self {
            id: ContextId::new(),
            context_type: context_type.into(),
            scope,
            certainty: Certainty::unknown(),
            properties: PropertyGraph::new(),
            meta: MetaInfo::now(),
        }
    }

    /// Create a temporal context
    pub fn temporal(description: impl Into<String>, bounds: Option<(crate::TemporalValue, crate::TemporalValue)>) -> Self {
        Self::new(
            "Temporal",
            Scope::Temporal(TemporalScope {
                description: description.into(),
                bounds,
            }),
        )
    }

    /// Create a spatial context
    pub fn spatial(description: impl Into<String>) -> Self {
        Self::new(
            "Spatial",
            Scope::Spatial(SpatialScope {
                description: description.into(),
                bounds: None,
            }),
        )
    }

    /// Create a theoretical context
    pub fn theoretical(theory: impl Into<String>, assumptions: Vec<String>) -> Self {
        Self::new(
            "Theoretical",
            Scope::Theoretical(TheoreticalScope {
                theory: theory.into(),
                assumptions,
            }),
        )
    }

    /// Create a cultural context
    pub fn cultural(culture: impl Into<String>) -> Self {
        Self::new(
            "Cultural",
            Scope::Cultural(CulturalScope {
                culture: culture.into(),
                period: None,
                region: None,
            }),
        )
    }

    /// Create an evidential context
    pub fn evidential(source_type: impl Into<String>, quality: impl Into<String>) -> Self {
        Self::new(
            "Evidential",
            Scope::Evidential(EvidentialScope {
                source_type: source_type.into(),
                quality: quality.into(),
                limitations: vec![],
            }),
        )
    }

    /// Set certainty
    pub fn with_certainty(mut self, certainty: Certainty) -> Self {
        self.certainty = certainty;
        self
    }

    /// Add a property
    pub fn with_property(mut self, key: impl Into<String>, value: crate::Property) -> Self {
        self.properties.set(key, value);
        self
    }
}

/// Different types of scopes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Scope {
    /// Time-related scope
    Temporal(TemporalScope),

    /// Space-related scope
    Spatial(SpatialScope),

    /// Culture/society scope
    Cultural(CulturalScope),

    /// Theoretical/research scope
    Theoretical(TheoreticalScope),

    /// Source/evidence scope
    Evidential(EvidentialScope),

    /// Combination of multiple scopes
    Composite(Vec<Scope>),

    /// Open-ended custom scope
    Custom(String, PropertyGraph),
}

/// Cultural scope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CulturalScope {
    pub culture: String,
    pub period: Option<TemporalScope>,
    pub region: Option<SpatialScope>,
}

/// Theoretical/research scope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TheoreticalScope {
    pub theory: String,
    pub assumptions: Vec<String>,
}

/// Evidential/source scope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidentialScope {
    pub source_type: String,
    pub quality: String,
    pub limitations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_context() {
        let ctx = Context::temporal("19th century", None);
        assert_eq!(ctx.context_type, "Temporal");
        match ctx.scope {
            Scope::Temporal(ts) => {
                assert_eq!(ts.description, "19th century");
            }
            _ => panic!("Expected Temporal scope"),
        }
    }

    #[test]
    fn test_spatial_context() {
        let ctx = Context::spatial("Boston, MA");
        assert_eq!(ctx.context_type, "Spatial");
        match ctx.scope {
            Scope::Spatial(ss) => {
                assert_eq!(ss.description, "Boston, MA");
            }
            _ => panic!("Expected Spatial scope"),
        }
    }

    #[test]
    fn test_theoretical_context() {
        let ctx = Context::theoretical("Migration Theory", vec![
            "Family moved westward".to_string(),
        ]);
        assert_eq!(ctx.context_type, "Theoretical");
        match ctx.scope {
            Scope::Theoretical(ts) => {
                assert_eq!(ts.theory, "Migration Theory");
                assert_eq!(ts.assumptions.len(), 1);
            }
            _ => panic!("Expected Theoretical scope"),
        }
    }

    #[test]
    fn test_cultural_context() {
        let ctx = Context::cultural("New England Colonial");
        assert_eq!(ctx.context_type, "Cultural");
        match ctx.scope {
            Scope::Cultural(cs) => {
                assert_eq!(cs.culture, "New England Colonial");
            }
            _ => panic!("Expected Cultural scope"),
        }
    }

    #[test]
    fn test_evidential_context() {
        let ctx = Context::evidential("Census Record", "Primary");
        assert_eq!(ctx.context_type, "Evidential");
        match ctx.scope {
            Scope::Evidential(es) => {
                assert_eq!(es.source_type, "Census Record");
                assert_eq!(es.quality, "Primary");
            }
            _ => panic!("Expected Evidential scope"),
        }
    }

    #[test]
    fn test_context_with_certainty() {
        let ctx = Context::temporal("circa 1850", None)
            .with_certainty(Certainty::probability(0.75));

        match ctx.certainty {
            Certainty::Probability(p) => assert_eq!(p, 0.75),
            _ => panic!("Expected Probability certainty"),
        }
    }

    #[test]
    fn test_context_with_properties() {
        let ctx = Context::spatial("Boston")
            .with_property("verified", crate::Property::Value(crate::Value::Boolean(true)));

        assert!(ctx.properties.get_bool("verified").is_ok());
    }

    #[test]
    fn test_composite_scope() {
        let scope = Scope::Composite(vec![
            Scope::Temporal(TemporalScope {
                description: "19th century".to_string(),
                bounds: None,
            }),
            Scope::Spatial(SpatialScope {
                description: "Massachusetts".to_string(),
                bounds: None,
            }),
        ]);

        match scope {
            Scope::Composite(scopes) => assert_eq!(scopes.len(), 2),
            _ => panic!("Expected Composite scope"),
        }
    }
}
