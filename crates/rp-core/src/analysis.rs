//! Analysis entity - represents the intellectual work of genealogical research

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::{EntityMetadata, NestableEntity},
    EntityId, Result, impl_entity, impl_validatable,
};

/// Types of genealogical analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisType {
    /// Connecting related evidence
    EvidenceCorrelation,
    /// Same person in different records?
    IdentityResolution,
    /// Resolving contradictions
    ConflictResolution,
    /// Building chronology
    TimelineConstruction,
    /// Determining relationships
    RelationshipAnalysis,
    /// Place identification
    LocationAnalysis,
    /// Name variations/changes
    NameAnalysis,
    /// Dating undated records
    DateAnalysis,
    /// Paleography
    HandwritingAnalysis,
    /// Genetic genealogy
    DnaAnalysis,
    /// Evaluating source reliability
    SourceCriticism,
    /// Why something is missing
    NegativeEvidence,
    /// FAN club analysis
    ClusterAnalysis,
    /// Movement patterns
    MigrationAnalysis,
    /// Custom analysis type
    Custom,
}

/// Methodologies used in analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisMethodology {
    /// Evidence directly states
    DirectEvidence,
    /// Requires inference
    IndirectEvidence,
    /// Comparing multiple sources
    Correlation,
    /// Process of elimination
    Elimination,
    /// Three+ sources agree
    Triangulation,
    /// Weight of evidence
    Preponderance,
    /// Timeline-based
    Chronological,
    /// Location-based
    Geographical,
    /// FAN principle
    SocialNetwork,
    /// Statistical analysis
    Statistical,
    /// Comparing similar cases
    Comparative,
}

/// Strength of analytical arguments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgumentStrength {
    /// Meets GPS standard
    Proof,
    /// Very likely correct
    Strong,
    /// More likely than not
    Moderate,
    /// Possible but uncertain
    Weak,
    /// Educated guess
    Speculative,
    /// Shown to be false
    Disproven,
}

/// A single point in an analysis
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AnalyticalPoint {
    pub point_id: EntityId,
    
    /// The assertion
    #[validate(length(min = 1))]
    pub assertion: String,
    
    /// Evidence used
    pub evidence_refs: Vec<EntityId>,
    
    /// Reasoning
    pub reasoning: String,
    pub methodology: AnalysisMethodology,
    
    /// Strength
    pub strength: ArgumentStrength,
    
    /// Alternative interpretations
    pub alternatives: Vec<String>,
    pub why_preferred: String,
    
    /// Dependencies on other points
    pub depends_on: Vec<EntityId>,
    
    /// Confidence (0-1)
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: f32,
}

impl AnalyticalPoint {
    pub fn new(assertion: impl Into<String>, methodology: AnalysisMethodology) -> Self {
        Self {
            point_id: EntityId::new(),
            assertion: assertion.into(),
            evidence_refs: Vec::new(),
            reasoning: String::new(),
            methodology,
            strength: ArgumentStrength::Moderate,
            alternatives: Vec::new(),
            why_preferred: String::new(),
            depends_on: Vec::new(),
            confidence: 0.5,
        }
    }
}

/// A set of correlated evidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CorrelationSet {
    pub correlation_id: EntityId,
    
    /// Evidence being correlated
    pub evidence_items: Vec<EntityId>,
    
    /// What they have in common
    pub correlation_points: Vec<String>,
    
    /// Differences noted
    pub differences: Vec<String>,
    
    /// Conclusion
    pub conclusion: String,
    
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: f32,
}

impl CorrelationSet {
    pub fn new(evidence_items: Vec<EntityId>) -> Self {
        Self {
            correlation_id: EntityId::new(),
            evidence_items,
            correlation_points: Vec::new(),
            differences: Vec::new(),
            conclusion: String::new(),
            confidence: 0.0,
        }
    }
}

/// Resolution of conflicting evidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConflictResolution {
    pub conflict_id: EntityId,
    
    /// Conflicting evidence
    pub evidence_a: EntityId,
    pub evidence_b: EntityId,
    
    /// Nature of conflict
    pub conflict_type: String, // "date", "name", "relationship", etc.
    pub conflict_description: String,
    
    /// Possible explanations
    pub explanations: Vec<String>,
    
    /// Preferred resolution
    pub resolution: String,
    pub resolution_reasoning: String,
    
    /// Which evidence is preferred
    pub preferred_evidence: Option<EntityId>,
    
    /// Confidence in resolution
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: f32,
}

impl ConflictResolution {
    pub fn new(evidence_a: EntityId, evidence_b: EntityId, conflict_type: impl Into<String>) -> Self {
        Self {
            conflict_id: EntityId::new(),
            evidence_a,
            evidence_b,
            conflict_type: conflict_type.into(),
            conflict_description: String::new(),
            explanations: Vec::new(),
            resolution: String::new(),
            resolution_reasoning: String::new(),
            preferred_evidence: None,
            confidence: 0.0,
        }
    }
}

/// Analysis is the intellectual work of genealogical research
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Analysis {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Basic properties
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    pub analysis_type: AnalysisType,
    
    /// What's being analyzed
    pub research_question_id: Option<EntityId>,
    pub scope_description: String,
    
    /// Evidence analyzed
    pub evidence_analyzed: Vec<EntityId>,
    
    /// Methodology
    pub methodology: Vec<AnalysisMethodology>,
    pub methodology_notes: String,
    
    /// The analysis itself
    pub analytical_points: Vec<AnalyticalPoint>,
    
    /// Correlations found
    pub correlations: Vec<CorrelationSet>,
    
    /// Conflicts resolved
    pub conflict_resolutions: Vec<ConflictResolution>,
    
    /// Building the argument
    pub argument_structure: Vec<String>,
    
    /// Conclusions reached
    pub conclusions: Vec<String>,
    pub overall_strength: ArgumentStrength,
    
    /// Limitations
    pub limitations: Vec<String>,
    pub assumptions: Vec<String>,
    
    /// What's still needed
    pub gaps_identified: Vec<String>,
    pub further_research: Vec<String>,
    
    /// Visual aids
    pub charts: Vec<ChartReference>,
    
    /// Peer review
    pub peer_reviews: Vec<PeerReview>,
    
    /// Metadata
    pub analyst: EntityId,
    pub analysis_date: DateTime<Utc>,
    pub last_revised: DateTime<Utc>,
    
    /// Nesting support (for multi-phase analysis)
    pub parent_analysis: Option<EntityId>,
    pub child_analyses: Vec<EntityId>,
}

/// Reference to a chart or visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartReference {
    pub chart_id: EntityId,
    pub chart_type: String, // "timeline", "map", "relationship", etc.
    pub title: String,
    pub description: String,
}

/// Peer review information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerReview {
    pub review_id: EntityId,
    pub reviewer: EntityId,
    pub review_date: DateTime<Utc>,
    pub comments: String,
    pub suggestions: Vec<String>,
    pub approved: bool,
}

impl Analysis {
    /// Create a new analysis
    pub fn new(
        title: impl Into<String>,
        analysis_type: AnalysisType,
        analyst: EntityId,
    ) -> Self {
        let now = Utc::now();
        Self {
            metadata: EntityMetadata::new(analyst),
            title: title.into(),
            analysis_type,
            research_question_id: None,
            scope_description: String::new(),
            evidence_analyzed: Vec::new(),
            methodology: Vec::new(),
            methodology_notes: String::new(),
            analytical_points: Vec::new(),
            correlations: Vec::new(),
            conflict_resolutions: Vec::new(),
            argument_structure: Vec::new(),
            conclusions: Vec::new(),
            overall_strength: ArgumentStrength::Moderate,
            limitations: Vec::new(),
            assumptions: Vec::new(),
            gaps_identified: Vec::new(),
            further_research: Vec::new(),
            charts: Vec::new(),
            peer_reviews: Vec::new(),
            analyst,
            analysis_date: now,
            last_revised: now,
            parent_analysis: None,
            child_analyses: Vec::new(),
        }
    }
    
    /// Add an analytical point
    pub fn add_point(&mut self, point: AnalyticalPoint) {
        self.analytical_points.push(point);
        self.last_revised = Utc::now();
        self.metadata.update(self.analyst);
    }
    
    /// Add a correlation
    pub fn add_correlation(&mut self, correlation: CorrelationSet) {
        self.correlations.push(correlation);
        self.last_revised = Utc::now();
        self.metadata.update(self.analyst);
    }
    
    /// Document a conflict resolution
    pub fn add_conflict_resolution(&mut self, resolution: ConflictResolution) {
        self.conflict_resolutions.push(resolution);
        self.last_revised = Utc::now();
        self.metadata.update(self.analyst);
    }
    
    /// Calculate overall strength of the analysis
    pub fn calculate_overall_strength(&mut self) -> ArgumentStrength {
        if self.analytical_points.is_empty() {
            self.overall_strength = ArgumentStrength::Speculative;
            return self.overall_strength;
        }
        
        // Map strength to numeric values
        let strength_value = |s: &ArgumentStrength| match s {
            ArgumentStrength::Proof => 5,
            ArgumentStrength::Strong => 4,
            ArgumentStrength::Moderate => 3,
            ArgumentStrength::Weak => 2,
            ArgumentStrength::Speculative => 1,
            ArgumentStrength::Disproven => 0,
        };
        
        // Average the strength of all points
        let total: u32 = self.analytical_points
            .iter()
            .map(|p| strength_value(&p.strength))
            .sum();
        let average = total as f32 / self.analytical_points.len() as f32;
        
        // Map back to strength
        self.overall_strength = match average {
            x if x >= 4.5 => ArgumentStrength::Proof,
            x if x >= 3.5 => ArgumentStrength::Strong,
            x if x >= 2.5 => ArgumentStrength::Moderate,
            x if x >= 1.5 => ArgumentStrength::Weak,
            _ => ArgumentStrength::Speculative,
        };
        
        self.overall_strength
    }
    
    /// Export analysis as narrative text
    pub fn export_narrative(&self) -> String {
        let mut narrative = vec![format!("# {}\n", self.title)];
        
        // Scope
        narrative.push(format!("## Scope\n{}\n", self.scope_description));
        
        // Methodology
        narrative.push("## Methodology\n".to_string());
        for method in &self.methodology {
            narrative.push(format!("- {:?}\n", method));
        }
        if !self.methodology_notes.is_empty() {
            narrative.push(format!("\n{}\n", self.methodology_notes));
        }
        
        // Analysis
        narrative.push("\n## Analysis\n".to_string());
        for (i, point) in self.analytical_points.iter().enumerate() {
            narrative.push(format!("\n### Point {}: {}\n", i + 1, point.assertion));
            narrative.push(format!("{}\n", point.reasoning));
            
            if !point.alternatives.is_empty() {
                narrative.push("\nAlternative interpretations considered:\n".to_string());
                for alt in &point.alternatives {
                    narrative.push(format!("- {}\n", alt));
                }
                narrative.push(format!("\n{}\n", point.why_preferred));
            }
        }
        
        // Conclusions
        narrative.push("\n## Conclusions\n".to_string());
        for conclusion in &self.conclusions {
            narrative.push(format!("- {}\n", conclusion));
        }
        narrative.push(format!("\nOverall strength: {:?}\n", self.overall_strength));
        
        // Limitations
        if !self.limitations.is_empty() {
            narrative.push("\n## Limitations\n".to_string());
            for limitation in &self.limitations {
                narrative.push(format!("- {}\n", limitation));
            }
        }
        
        narrative.join("")
    }
}

// Implement Entity trait
impl_entity!(Analysis, "Analysis");

// Implement NestableEntity for multi-phase analysis
#[async_trait::async_trait]
impl NestableEntity for Analysis {
    fn children(&self) -> Vec<EntityId> {
        self.child_analyses.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "Analysis"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if !self.child_analyses.contains(&child_id) {
            self.child_analyses.push(child_id);
            self.metadata.update(self.analyst);
        }
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_analyses.len();
        self.child_analyses.retain(|id| id != &child_id);
        if self.child_analyses.len() < initial_len {
            self.metadata.update(self.analyst);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// Implement Validatable trait
impl_validatable!(Analysis);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_analysis_creation() {
        let analyst_id = EntityId::new();
        let analysis = Analysis::new(
            "Identity Resolution - John Smith",
            AnalysisType::IdentityResolution,
            analyst_id,
        );
        
        assert_eq!(analysis.title, "Identity Resolution - John Smith");
        assert_eq!(analysis.analysis_type, AnalysisType::IdentityResolution);
        assert!(analysis.validate().await.is_valid());
    }
    
    #[tokio::test]
    async fn test_analytical_points() {
        let analyst_id = EntityId::new();
        let mut analysis = Analysis::new(
            "Test Analysis",
            AnalysisType::Custom,
            analyst_id,
        );
        
        let point = AnalyticalPoint::new(
            "John in 1850 census is same person as John in 1860 census",
            AnalysisMethodology::Correlation,
        );
        
        analysis.add_point(point);
        assert_eq!(analysis.analytical_points.len(), 1);
    }
    
    #[tokio::test]
    async fn test_overall_strength_calculation() {
        let analyst_id = EntityId::new();
        let mut analysis = Analysis::new(
            "Test Analysis",
            AnalysisType::Custom,
            analyst_id,
        );
        
        // Add points with different strengths
        let mut strong_point = AnalyticalPoint::new("Strong assertion", AnalysisMethodology::DirectEvidence);
        strong_point.strength = ArgumentStrength::Strong;
        analysis.add_point(strong_point);
        
        let mut weak_point = AnalyticalPoint::new("Weak assertion", AnalysisMethodology::IndirectEvidence);
        weak_point.strength = ArgumentStrength::Weak;
        analysis.add_point(weak_point);
        
        let strength = analysis.calculate_overall_strength();
        assert_eq!(strength, ArgumentStrength::Moderate); // Average of Strong(4) and Weak(2) = 3
    }
}