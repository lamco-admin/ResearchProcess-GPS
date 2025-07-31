//! AnalysisReport entity - Evidence analysis matrices and worksheets
//! Part of Layer 2: Research Process Model

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    work_product::{WorkProduct, WorkProductType},
    entity::{Entity, EntityData},
    source::{SourceQuality, InformationClass},
    EntityId, impl_validatable,
};

/// Analysis methodology used
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalysisMethod {
    /// Evidence Explained methodology (E. S. Mills)
    EvidenceExplained,
    /// Board for Certification of Genealogists standards
    BCGStandards,
    /// FamilySearch methodology
    FamilySearch,
    /// Custom methodology
    Custom(String),
}

/// Item being analyzed
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AnalysisReportItem {
    /// Evidence entity ID
    pub evidence_id: EntityId,
    
    /// Source entity ID
    pub source_id: EntityId,
    
    /// Brief description
    #[validate(length(min = 1))]
    pub description: String,
    
    /// Information extracted
    pub information: String,
    
    /// Source quality assessment
    pub source_quality: SourceQuality,
    
    /// Information type
    pub information_class: InformationClass,
    
    /// Evidence type (direct/indirect/negative)
    pub evidence_type: EvidenceType,
    
    /// Relevance to research question
    pub relevance: Relevance,
    
    /// Reliability assessment
    pub reliability: Reliability,
    
    /// Notes about this item
    pub notes: Option<String>,
}

/// Type of evidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    /// Directly answers the question
    Direct,
    /// Requires inference
    Indirect,
    /// Absence of expected evidence
    Negative,
}

/// Relevance to research question
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Relevance {
    /// Highly relevant
    High,
    /// Moderately relevant
    Medium,
    /// Marginally relevant
    Low,
    /// Not relevant
    None,
}

/// Reliability assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reliability {
    /// Highly reliable
    High,
    /// Generally reliable
    Good,
    /// Some concerns
    Fair,
    /// Significant concerns
    Poor,
    /// Not reliable
    Unreliable,
}

/// Correlation between evidence items
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Correlation {
    /// Type of correlation
    #[validate(length(min = 1))]
    pub correlation_type: String,
    
    /// Evidence items involved
    pub evidence_items: Vec<EntityId>,
    
    /// Description of correlation
    #[validate(length(min = 1))]
    pub description: String,
    
    /// Strength of correlation
    pub strength: CorrelationStrength,
    
    /// Supporting details
    pub details: serde_json::Value,
}

/// Strength of correlation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorrelationStrength {
    /// Very strong correlation
    VeryStrong,
    /// Strong correlation
    Strong,
    /// Moderate correlation
    Moderate,
    /// Weak correlation
    Weak,
    /// No correlation
    None,
}

/// Pattern identified in evidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Pattern {
    /// Pattern type
    #[validate(length(min = 1))]
    pub pattern_type: String,
    
    /// Description
    #[validate(length(min = 1))]
    pub description: String,
    
    /// Evidence items showing pattern
    pub evidence_items: Vec<EntityId>,
    
    /// Significance
    pub significance: Significance,
    
    /// Implications
    pub implications: Vec<String>,
}

/// Anomaly in evidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Anomaly {
    /// Anomaly type
    #[validate(length(min = 1))]
    pub anomaly_type: String,
    
    /// Description
    #[validate(length(min = 1))]
    pub description: String,
    
    /// Evidence item with anomaly
    pub evidence_id: EntityId,
    
    /// Severity
    pub severity: AnomalySeverity,
    
    /// Possible explanations
    pub explanations: Vec<String>,
    
    /// Requires follow-up
    pub requires_followup: bool,
}

/// Significance level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Significance {
    /// Critical significance
    Critical,
    /// High significance
    High,
    /// Moderate significance
    Moderate,
    /// Low significance
    Low,
}

/// Anomaly severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    /// Critical issue
    Critical,
    /// Major concern
    Major,
    /// Minor issue
    Minor,
    /// Informational only
    Info,
}

/// Quality assessment for a source
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SourceQualityAssessment {
    /// Source ID
    pub source_id: EntityId,
    
    /// Overall quality
    pub quality: SourceQuality,
    
    /// Quality factors
    pub factors: serde_json::Value,
    
    /// Assessment notes
    pub notes: String,
}

/// Quality assessment for information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct InformationQualityAssessment {
    /// Evidence ID
    pub evidence_id: EntityId,
    
    /// Information class
    pub class: InformationClass,
    
    /// Quality factors
    pub factors: serde_json::Value,
    
    /// Assessment notes
    pub notes: String,
}

/// Quality assessment for evidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EvidenceQualityAssessment {
    /// Evidence ID
    pub evidence_id: EntityId,
    
    /// Evidence type
    pub evidence_type: EvidenceType,
    
    /// Overall quality score (0.0 to 1.0)
    #[validate(range(min = 0.0, max = 1.0))]
    pub quality_score: f64,
    
    /// Quality factors
    pub factors: serde_json::Value,
}

/// Evidence analysis entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AnalysisReport {
    /// Base work product
    #[serde(flatten)]
    pub work_product: WorkProduct,
    
    /// Analysis methodology
    pub analysis_method: AnalysisMethod,
    
    /// Evidence items being analyzed
    #[validate(nested)]
    pub evidence_items: Vec<AnalysisReportItem>,
    
    /// Correlations found
    #[validate(nested)]
    pub correlations: Vec<Correlation>,
    
    /// Correlation categories
    pub correlation_categories: Vec<(String, Vec<usize>)>,
    
    /// Source quality assessments
    #[validate(nested)]
    pub source_quality_matrix: Vec<SourceQualityAssessment>,
    
    /// Information quality assessments
    #[validate(nested)]
    pub information_quality_matrix: Vec<InformationQualityAssessment>,
    
    /// Evidence quality assessments
    #[validate(nested)]
    pub evidence_quality_matrix: Vec<EvidenceQualityAssessment>,
    
    /// Patterns identified
    #[validate(nested)]
    pub patterns: Vec<Pattern>,
    
    /// Anomalies found
    #[validate(nested)]
    pub anomalies: Vec<Anomaly>,
    
    /// Synthesis of findings
    pub synthesis: String,
    
    /// Link to confidence assessment
    pub confidence_assessment: Option<EntityId>,
    
    /// Follow-up recommendations
    pub followup_recommendations: Vec<String>,
    
    /// Analysis statistics
    pub statistics: serde_json::Value,
}

impl AnalysisReport {
    /// Create a new evidence analysis
    pub fn new(
        analysis_method: AnalysisMethod,
        theory_id: EntityId,
        created_by: EntityId,
    ) -> Self {
        let work_product = WorkProduct::new(
            WorkProductType::AnalysisReport,
            "evidence-analysis-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            created_by,
        );
        
        Self {
            work_product,
            analysis_method,
            evidence_items: vec![],
            correlations: vec![],
            correlation_categories: vec![],
            source_quality_matrix: vec![],
            information_quality_matrix: vec![],
            evidence_quality_matrix: vec![],
            patterns: vec![],
            anomalies: vec![],
            synthesis: String::new(),
            confidence_assessment: None,
            followup_recommendations: vec![],
            statistics: serde_json::json!({
                "total_evidence": 0,
                "direct_evidence": 0,
                "indirect_evidence": 0,
                "negative_evidence": 0,
                "correlations_found": 0,
                "patterns_identified": 0,
                "anomalies_detected": 0,
            }),
        }
    }
    
    /// Add evidence item for analysis
    pub fn add_evidence_item(&mut self, item: AnalysisReportItem) {
        self.evidence_items.push(item);
        self.update_statistics();
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add correlation
    pub fn add_correlation(&mut self, correlation: Correlation) {
        self.correlations.push(correlation);
        self.update_statistics();
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Group correlations by category
    pub fn categorize_correlations(&mut self, category: impl Into<String>, correlation_indices: Vec<usize>) {
        self.correlation_categories.push((category.into(), correlation_indices));
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add source quality assessment
    pub fn assess_source_quality(&mut self, assessment: SourceQualityAssessment) {
        // Remove existing assessment for same source
        self.source_quality_matrix.retain(|a| a.source_id != assessment.source_id);
        self.source_quality_matrix.push(assessment);
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add information quality assessment
    pub fn assess_information_quality(&mut self, assessment: InformationQualityAssessment) {
        // Remove existing assessment for same evidence
        self.information_quality_matrix.retain(|a| a.evidence_id != assessment.evidence_id);
        self.information_quality_matrix.push(assessment);
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add evidence quality assessment
    pub fn assess_evidence_quality(&mut self, assessment: EvidenceQualityAssessment) {
        // Remove existing assessment for same evidence
        self.evidence_quality_matrix.retain(|a| a.evidence_id != assessment.evidence_id);
        self.evidence_quality_matrix.push(assessment);
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add pattern
    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(pattern);
        self.update_statistics();
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add anomaly
    pub fn add_anomaly(&mut self, anomaly: Anomaly) {
        self.anomalies.push(anomaly);
        self.update_statistics();
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Set synthesis
    pub fn set_synthesis(&mut self, synthesis: impl Into<String>) {
        self.synthesis = synthesis.into();
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add follow-up recommendation
    pub fn add_followup(&mut self, recommendation: impl Into<String>) {
        self.followup_recommendations.push(recommendation.into());
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        let total = self.evidence_items.len();
        let direct = self.evidence_items.iter()
            .filter(|i| matches!(i.evidence_type, EvidenceType::Direct))
            .count();
        let indirect = self.evidence_items.iter()
            .filter(|i| matches!(i.evidence_type, EvidenceType::Indirect))
            .count();
        let negative = self.evidence_items.iter()
            .filter(|i| matches!(i.evidence_type, EvidenceType::Negative))
            .count();
        
        self.statistics = serde_json::json!({
            "total_evidence": total,
            "direct_evidence": direct,
            "indirect_evidence": indirect,
            "negative_evidence": negative,
            "correlations_found": self.correlations.len(),
            "patterns_identified": self.patterns.len(),
            "anomalies_detected": self.anomalies.len(),
            "critical_anomalies": self.anomalies.iter()
                .filter(|a| matches!(a.severity, AnomalySeverity::Critical))
                .count(),
        });
    }
    
    /// Get evidence by type
    pub fn evidence_by_type(&self, evidence_type: EvidenceType) -> Vec<&AnalysisReportItem> {
        self.evidence_items.iter()
            .filter(|i| i.evidence_type == evidence_type)
            .collect()
    }
    
    /// Get high relevance evidence
    pub fn high_relevance_evidence(&self) -> Vec<&AnalysisReportItem> {
        self.evidence_items.iter()
            .filter(|i| matches!(i.relevance, Relevance::High))
            .collect()
    }
    
    /// Get critical patterns
    pub fn critical_patterns(&self) -> Vec<&Pattern> {
        self.patterns.iter()
            .filter(|p| matches!(p.significance, Significance::Critical))
            .collect()
    }
    
    /// Get anomalies requiring follow-up
    pub fn followup_required(&self) -> Vec<&Anomaly> {
        self.anomalies.iter()
            .filter(|a| a.requires_followup)
            .collect()
    }
    
    /// Generate analysis summary
    pub fn generate_summary(&self) -> serde_json::Value {
        serde_json::json!({
            "methodology": format!("{:?}", self.analysis_method),
            "evidence_analyzed": self.evidence_items.len(),
            "evidence_breakdown": {
                "direct": self.statistics["direct_evidence"],
                "indirect": self.statistics["indirect_evidence"],
                "negative": self.statistics["negative_evidence"],
            },
            "correlations": self.correlations.len(),
            "patterns": self.patterns.len(),
            "anomalies": self.anomalies.len(),
            "critical_findings": self.critical_patterns().len(),
            "followup_needed": self.followup_required().len(),
            "has_synthesis": !self.synthesis.is_empty(),
        })
    }
}

// Implement Entity trait for AnalysisReport
#[async_trait]
impl Entity for AnalysisReport {
    fn id(&self) -> EntityId {
        self.work_product.id()
    }
    
    fn entity_type(&self) -> &'static str {
        "AnalysisReport"
    }
    
    fn created_by(&self) -> EntityId {
        self.work_product.created_by()
    }
    
    fn created_at(&self) -> DateTime<Utc> {
        self.work_product.created_at()
    }
    
    fn modified_by(&self) -> EntityId {
        self.work_product.modified_by()
    }
    
    fn modified_at(&self) -> DateTime<Utc> {
        self.work_product.modified_at()
    }
    
    fn is_active(&self) -> bool {
        self.work_product.is_active()
    }
    
    fn as_entity(&self) -> EntityData {
        EntityData {
            id: self.id(),
            entity_type: self.entity_type().to_string(),
            created_by: self.created_by(),
            created_at: self.created_at(),
            modified_by: self.modified_by(),
            modified_at: self.modified_at(),
            is_active: self.is_active(),
            version: Some(self.work_product.metadata.version),
            data: serde_json::to_value(self).unwrap_or_default(),
        }
    }
}

impl_validatable!(AnalysisReport);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceClass;
    
    #[test]
    fn test_evidence_analysis_creation() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let analysis = AnalysisReport::new(
            AnalysisMethod::EvidenceExplained,
            theory_id,
            researcher_id,
        );
        
        assert_eq!(analysis.analysis_method, AnalysisMethod::EvidenceExplained);
        assert_eq!(analysis.evidence_items.len(), 0);
        assert_eq!(analysis.statistics["total_evidence"], 0);
    }
    
    #[test]
    fn test_add_evidence_items() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let evidence_id = EntityId::new();
        let source_id = EntityId::new();
        
        let mut analysis = AnalysisReport::new(
            AnalysisMethod::BCGStandards,
            theory_id,
            researcher_id,
        );
        
        let source_quality = SourceQuality {
            source_class: SourceClass::Original,
            information_class: InformationClass::Primary,
            quality_score: 0.9,
            dimensions: crate::source::QualityDimensions {
                completeness: 1.0,
                legibility: 1.0,
                accuracy: 0.9,
                reliability: 1.0,
                temporal_relevance: 1.0,
                geographic_relevance: 1.0,
            },
            issues: vec![],
            notes: None,
            assessed_by: researcher_id,
            assessed_at: Utc::now(),
        };
        
        let item = AnalysisReportItem {
            evidence_id,
            source_id,
            description: "Birth certificate".to_string(),
            information: "John Smith born 1850".to_string(),
            source_quality,
            information_class: InformationClass::Primary,
            evidence_type: EvidenceType::Direct,
            relevance: Relevance::High,
            reliability: Reliability::High,
            notes: None,
        };
        
        analysis.add_evidence_item(item);
        
        assert_eq!(analysis.evidence_items.len(), 1);
        assert_eq!(analysis.statistics["total_evidence"], 1);
        assert_eq!(analysis.statistics["direct_evidence"], 1);
    }
    
    #[test]
    fn test_correlations() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let evidence1 = EntityId::new();
        let evidence2 = EntityId::new();
        
        let mut analysis = AnalysisReport::new(
            AnalysisMethod::Custom("My Method".to_string()),
            theory_id,
            researcher_id,
        );
        
        let correlation = Correlation {
            correlation_type: "Name variant".to_string(),
            evidence_items: vec![evidence1, evidence2],
            description: "John and Johann appear to be same person".to_string(),
            strength: CorrelationStrength::Strong,
            details: serde_json::json!({
                "reasoning": "Same location and family members"
            }),
        };
        
        analysis.add_correlation(correlation);
        analysis.categorize_correlations("Name variations", vec![0]);
        
        assert_eq!(analysis.correlations.len(), 1);
        assert_eq!(analysis.correlation_categories.len(), 1);
        assert_eq!(analysis.statistics["correlations_found"], 1);
    }
    
    #[test]
    fn test_quality_assessments() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let source_id = EntityId::new();
        let evidence_id = EntityId::new();
        
        let mut analysis = AnalysisReport::new(
            AnalysisMethod::EvidenceExplained,
            theory_id,
            researcher_id,
        );
        
        // Source quality
        let quality = SourceQuality {
            source_class: SourceClass::Original,
            information_class: InformationClass::Primary,
            quality_score: 0.95,
            dimensions: crate::source::QualityDimensions {
                completeness: 1.0,
                legibility: 1.0,
                accuracy: 1.0,
                reliability: 1.0,
                temporal_relevance: 1.0,
                geographic_relevance: 1.0,
            },
            issues: vec![],
            notes: Some("Well-preserved original".to_string()),
            assessed_by: researcher_id,
            assessed_at: Utc::now(),
        };
        
        let source_assessment = SourceQualityAssessment {
            source_id,
            quality,
            factors: serde_json::json!({
                "condition": "excellent",
                "legibility": "clear",
            }),
            notes: "Well-preserved original".to_string(),
        };
        
        analysis.assess_source_quality(source_assessment);
        
        // Information quality
        let info_assessment = InformationQualityAssessment {
            evidence_id,
            class: InformationClass::Primary,
            factors: serde_json::json!({
                "informant": "participant",
                "time_lag": "contemporaneous",
            }),
            notes: "Firsthand account".to_string(),
        };
        
        analysis.assess_information_quality(info_assessment);
        
        // Evidence quality
        let evidence_assessment = EvidenceQualityAssessment {
            evidence_id,
            evidence_type: EvidenceType::Direct,
            quality_score: 0.95,
            factors: serde_json::json!({
                "clarity": "unambiguous",
                "completeness": "full",
            }),
        };
        
        analysis.assess_evidence_quality(evidence_assessment);
        
        assert_eq!(analysis.source_quality_matrix.len(), 1);
        assert_eq!(analysis.information_quality_matrix.len(), 1);
        assert_eq!(analysis.evidence_quality_matrix.len(), 1);
    }
    
    #[test]
    fn test_patterns_and_anomalies() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let evidence1 = EntityId::new();
        let evidence2 = EntityId::new();
        
        let mut analysis = AnalysisReport::new(
            AnalysisMethod::FamilySearch,
            theory_id,
            researcher_id,
        );
        
        // Add pattern
        let pattern = Pattern {
            pattern_type: "Migration".to_string(),
            description: "Family moved westward over 3 generations".to_string(),
            evidence_items: vec![evidence1, evidence2],
            significance: Significance::High,
            implications: vec!["Check western states for descendants".to_string()],
        };
        
        analysis.add_pattern(pattern);
        
        // Add anomaly
        let anomaly = Anomaly {
            anomaly_type: "Age discrepancy".to_string(),
            description: "Age differs by 10 years between censuses".to_string(),
            evidence_id: evidence1,
            severity: AnomalySeverity::Major,
            explanations: vec![
                "Enumeration error".to_string(),
                "Different person with same name".to_string(),
            ],
            requires_followup: true,
        };
        
        analysis.add_anomaly(anomaly);
        
        assert_eq!(analysis.patterns.len(), 1);
        assert_eq!(analysis.anomalies.len(), 1);
        assert_eq!(analysis.statistics["patterns_identified"], 1);
        assert_eq!(analysis.statistics["anomalies_detected"], 1);
    }
    
    #[test]
    fn test_synthesis_and_followup() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut analysis = AnalysisReport::new(
            AnalysisMethod::BCGStandards,
            theory_id,
            researcher_id,
        );
        
        analysis.set_synthesis("Evidence strongly supports John Smith born 1850 in Boston");
        analysis.add_followup("Check Boston city directories 1870-1880");
        analysis.add_followup("Obtain death certificate");
        
        assert!(!analysis.synthesis.is_empty());
        assert_eq!(analysis.followup_recommendations.len(), 2);
    }
    
    #[test]
    fn test_evidence_filtering() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut analysis = AnalysisReport::new(
            AnalysisMethod::EvidenceExplained,
            theory_id,
            researcher_id,
        );
        
        // Add various evidence items
        for i in 0..5 {
            let source_quality = SourceQuality {
                source_class: SourceClass::Derivative,
                information_class: InformationClass::Secondary,
                quality_score: 0.7,
                dimensions: crate::source::QualityDimensions {
                    completeness: 0.8,
                    legibility: 0.9,
                    accuracy: 0.7,
                    reliability: 0.8,
                    temporal_relevance: 0.8,
                    geographic_relevance: 0.8,
                },
                issues: vec![],
                notes: None,
                assessed_by: researcher_id,
                assessed_at: Utc::now(),
            };
            
            let item = AnalysisReportItem {
                evidence_id: EntityId::new(),
                source_id: EntityId::new(),
                description: format!("Evidence {}", i),
                information: format!("Info {}", i),
                source_quality,
                information_class: InformationClass::Secondary,
                evidence_type: match i {
                    0..=1 => EvidenceType::Direct,
                    2..=3 => EvidenceType::Indirect,
                    _ => EvidenceType::Negative,
                },
                relevance: if i == 0 { Relevance::High } else { Relevance::Medium },
                reliability: Reliability::Good,
                notes: None,
            };
            
            analysis.add_evidence_item(item);
        }
        
        assert_eq!(analysis.evidence_by_type(EvidenceType::Direct).len(), 2);
        assert_eq!(analysis.evidence_by_type(EvidenceType::Indirect).len(), 2);
        assert_eq!(analysis.evidence_by_type(EvidenceType::Negative).len(), 1);
        assert_eq!(analysis.high_relevance_evidence().len(), 1);
    }
    
    #[test]
    fn test_critical_findings() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut analysis = AnalysisReport::new(
            AnalysisMethod::BCGStandards,
            theory_id,
            researcher_id,
        );
        
        // Add critical pattern
        let critical_pattern = Pattern {
            pattern_type: "Identity match".to_string(),
            description: "Multiple evidence points to same person".to_string(),
            evidence_items: vec![EntityId::new(); 3],
            significance: Significance::Critical,
            implications: vec!["Confirms identity hypothesis".to_string()],
        };
        
        analysis.add_pattern(critical_pattern);
        
        // Add non-critical pattern
        let minor_pattern = Pattern {
            pattern_type: "Spelling variation".to_string(),
            description: "Name spelled differently".to_string(),
            evidence_items: vec![EntityId::new()],
            significance: Significance::Low,
            implications: vec![],
        };
        
        analysis.add_pattern(minor_pattern);
        
        assert_eq!(analysis.critical_patterns().len(), 1);
    }
    
    #[test]
    fn test_analysis_summary() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut analysis = AnalysisReport::new(
            AnalysisMethod::EvidenceExplained,
            theory_id,
            researcher_id,
        );
        
        // Add some evidence
        for _ in 0..3 {
            let source_quality = SourceQuality {
                source_class: SourceClass::Original,
                information_class: InformationClass::Primary,
                quality_score: 0.9,
                dimensions: crate::source::QualityDimensions {
                    completeness: 1.0,
                    legibility: 1.0,
                    accuracy: 0.9,
                    reliability: 1.0,
                    temporal_relevance: 1.0,
                    geographic_relevance: 1.0,
                },
                issues: vec![],
                notes: None,
                assessed_by: researcher_id,
                assessed_at: Utc::now(),
            };
            
            let item = AnalysisReportItem {
                evidence_id: EntityId::new(),
                source_id: EntityId::new(),
                description: "Test evidence".to_string(),
                information: "Test info".to_string(),
                source_quality,
                information_class: InformationClass::Primary,
                evidence_type: EvidenceType::Direct,
                relevance: Relevance::High,
                reliability: Reliability::High,
                notes: None,
            };
            analysis.add_evidence_item(item);
        }
        
        analysis.set_synthesis("Test synthesis");
        
        let summary = analysis.generate_summary();
        
        assert_eq!(summary["evidence_analyzed"], 3);
        assert_eq!(summary["has_synthesis"], true);
        assert!(summary["evidence_breakdown"].is_object());
    }
}