//! Confidence framework - narrative-based confidence assessment

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::EntityMetadata,
    EntityId, impl_entity, impl_validatable,
};

/// Confidence is a narrative container that captures the full reasoning
/// behind confidence assessments, not just a simple score
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Confidence {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// The entity this confidence assessment applies to
    pub target_entity: EntityId,
    
    /// Type of entity being assessed
    pub target_type: String,
    
    /// Overall confidence level (derived from narrative)
    pub level: ConfidenceLevel,
    
    /// Detailed narrative explaining the confidence assessment
    #[validate(length(min = 10))]
    pub narrative: String,
    
    /// Specific factors that influenced this assessment
    pub factors: Vec<ConfidenceFactor>,
    
    /// Supporting evidence referenced in the narrative
    pub supporting_evidence: Vec<EntityId>,
    
    /// Conflicting evidence that reduces confidence
    pub conflicting_evidence: Vec<EntityId>,
    
    /// Analysis chains that contributed to this assessment
    pub analyses: Vec<EntityId>,
    
    /// Tags for categorizing confidence assessments
    pub tags: Vec<String>,
    
    /// Whether this assessment supersedes previous ones
    pub supersedes: Vec<EntityId>,
}

/// Confidence level categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    /// Very low confidence - speculation only
    VeryLow,
    /// Low confidence - some evidence but significant gaps
    Low,
    /// Medium confidence - reasonable evidence with some uncertainty
    Medium,
    /// High confidence - strong evidence with minor gaps
    High,
    /// Very high confidence - overwhelming evidence
    VeryHigh,
    /// Certain - no reasonable doubt (use sparingly)
    Certain,
}

/// Specific factor that influences confidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConfidenceFactor {
    /// Category of factor
    pub category: FactorCategory,
    
    /// Description of this specific factor
    #[validate(length(min = 1))]
    pub description: String,
    
    /// Impact on confidence (positive or negative)
    pub impact: FactorImpact,
    
    /// Weight of this factor (0.0 to 1.0)
    #[validate(range(min = 0.0, max = 1.0))]
    pub weight: f32,
}

/// Categories of confidence factors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorCategory {
    /// Quality and reliability of sources
    SourceQuality,
    /// Directness of evidence (primary vs secondary)
    EvidenceDirectness,
    /// Consistency across multiple sources
    Consistency,
    /// Temporal proximity to events
    Temporal,
    /// Geographic relevance
    Geographic,
    /// Expertise of information provider
    Expertise,
    /// Completeness of information
    Completeness,
    /// Corroboration from independent sources
    Corroboration,
    /// Known biases or conflicts of interest
    Bias,
    /// Other factors
    Other,
}

/// Impact of a factor on confidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorImpact {
    /// Strongly increases confidence
    StronglyPositive,
    /// Moderately increases confidence
    Positive,
    /// Neither positive nor negative
    Neutral,
    /// Moderately decreases confidence
    Negative,
    /// Strongly decreases confidence
    StronglyNegative,
}

impl Confidence {
    /// Create a new confidence assessment
    pub fn new(
        target_entity: EntityId,
        target_type: impl Into<String>,
        level: ConfidenceLevel,
        narrative: impl Into<String>,
        created_by: EntityId,
    ) -> Self {
        Self {
            metadata: EntityMetadata::new(created_by),
            target_entity,
            target_type: target_type.into(),
            level,
            narrative: narrative.into(),
            factors: Vec::new(),
            supporting_evidence: Vec::new(),
            conflicting_evidence: Vec::new(),
            analyses: Vec::new(),
            tags: Vec::new(),
            supersedes: Vec::new(),
        }
    }
    
    /// Add a confidence factor
    pub fn add_factor(&mut self, factor: ConfidenceFactor) {
        self.factors.push(factor);
    }
    
    /// Calculate weighted confidence score (0.0 to 1.0)
    /// This is supplementary to the narrative, not a replacement
    pub fn calculate_score(&self) -> f32 {
        if self.factors.is_empty() {
            return self.level.to_score();
        }
        
        let total_weight: f32 = self.factors.iter().map(|f| f.weight).sum();
        if total_weight == 0.0 {
            return self.level.to_score();
        }
        
        let weighted_sum: f32 = self.factors
            .iter()
            .map(|f| f.weight * f.impact.to_score_modifier())
            .sum();
            
        let base_score = self.level.to_score();
        let modifier = weighted_sum / total_weight;
        
        (base_score + modifier).clamp(0.0, 1.0)
    }
    
    /// Get a summary of positive factors
    pub fn positive_factors(&self) -> Vec<&ConfidenceFactor> {
        self.factors
            .iter()
            .filter(|f| matches!(f.impact, FactorImpact::Positive | FactorImpact::StronglyPositive))
            .collect()
    }
    
    /// Get a summary of negative factors
    pub fn negative_factors(&self) -> Vec<&ConfidenceFactor> {
        self.factors
            .iter()
            .filter(|f| matches!(f.impact, FactorImpact::Negative | FactorImpact::StronglyNegative))
            .collect()
    }
}

impl ConfidenceLevel {
    /// Convert to a base score (0.0 to 1.0)
    pub fn to_score(&self) -> f32 {
        match self {
            Self::VeryLow => 0.1,
            Self::Low => 0.3,
            Self::Medium => 0.5,
            Self::High => 0.7,
            Self::VeryHigh => 0.9,
            Self::Certain => 1.0,
        }
    }
    
    /// Create from a score (0.0 to 1.0)
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s <= 0.2 => Self::VeryLow,
            s if s <= 0.4 => Self::Low,
            s if s <= 0.6 => Self::Medium,
            s if s <= 0.8 => Self::High,
            s if s <= 0.95 => Self::VeryHigh,
            _ => Self::Certain,
        }
    }
}

impl FactorImpact {
    /// Convert to score modifier (-0.3 to +0.3)
    pub fn to_score_modifier(&self) -> f32 {
        match self {
            Self::StronglyPositive => 0.3,
            Self::Positive => 0.15,
            Self::Neutral => 0.0,
            Self::Negative => -0.15,
            Self::StronglyNegative => -0.3,
        }
    }
}

// Implement Entity trait
impl_entity!(Confidence, "Confidence");

// Implement Validatable trait
impl_validatable!(Confidence);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_confidence_creation() {
        let researcher_id = EntityId::new();
        let target_id = EntityId::new();
        
        let confidence = Confidence::new(
            target_id,
            "Theory",
            ConfidenceLevel::High,
            "Strong evidence from multiple primary sources confirms this theory.",
            researcher_id,
        );
        
        assert_eq!(confidence.level, ConfidenceLevel::High);
        assert!(confidence.validate().await.is_valid());
    }
    
    #[tokio::test]
    async fn test_confidence_factors() {
        let researcher_id = EntityId::new();
        let target_id = EntityId::new();
        
        let mut confidence = Confidence::new(
            target_id,
            "Person",
            ConfidenceLevel::Medium,
            "Evidence suggests this identification but some gaps remain.",
            researcher_id,
        );
        
        confidence.add_factor(ConfidenceFactor {
            category: FactorCategory::SourceQuality,
            description: "Original birth certificate".to_string(),
            impact: FactorImpact::StronglyPositive,
            weight: 0.8,
        });
        
        confidence.add_factor(ConfidenceFactor {
            category: FactorCategory::Consistency,
            description: "Name spelling varies across documents".to_string(),
            impact: FactorImpact::Negative,
            weight: 0.3,
        });
        
        let score = confidence.calculate_score();
        assert!(score > 0.5); // Should be above medium due to positive factor weight
        assert!(score < 0.8); // Should be below high due to negative factor
    }
}