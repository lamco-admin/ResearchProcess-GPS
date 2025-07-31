//! ProofStatement entity - GPS-compliant proof statements and arguments
//! Part of Layer 2: Research Process Model

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    work_product::{WorkProduct, WorkProductType},
    entity::{Entity, EntityData},
    EntityId, impl_validatable,
};

/// Type of proof document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofType {
    /// Formal GPS proof statement
    Statement,
    /// Detailed proof argument
    Argument,
    /// Proof summary
    Summary,
    /// Research conclusion
    Conclusion,
}

/// Section of a proof statement
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProofSection {
    /// Section title
    #[validate(length(min = 1))]
    pub title: String,
    
    /// Section content (markdown/text)
    pub content: String,
    
    /// Evidence references in this section
    pub evidence_refs: Vec<EntityId>,
    
    /// Analysis references in this section
    pub analysis_refs: Vec<EntityId>,
    
    /// Citations specific to this section
    pub citations: Vec<String>,
    
    /// Section metadata
    pub metadata: serde_json::Value,
}

impl ProofSection {
    /// Create a new proof section
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            evidence_refs: vec![],
            analysis_refs: vec![],
            citations: vec![],
            metadata: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

/// GPS (Genealogical Proof Standard) element
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GPSElement {
    /// Reasonably exhaustive research
    ExhaustiveResearch,
    /// Complete and accurate source citations
    CompleteCitations,
    /// Thorough analysis and correlation
    ThoroughAnalysis,
    /// Resolution of conflicting evidence
    ConflictResolution,
    /// Sound written conclusion
    WrittenConclusion,
}

impl GPSElement {
    /// Get all GPS elements
    pub fn all() -> Vec<Self> {
        vec![
            Self::ExhaustiveResearch,
            Self::CompleteCitations,
            Self::ThoroughAnalysis,
            Self::ConflictResolution,
            Self::WrittenConclusion,
        ]
    }
    
    /// Get element name
    pub fn name(&self) -> &'static str {
        match self {
            Self::ExhaustiveResearch => "Reasonably Exhaustive Research",
            Self::CompleteCitations => "Complete and Accurate Source Citations",
            Self::ThoroughAnalysis => "Thorough Analysis and Correlation",
            Self::ConflictResolution => "Resolution of Conflicting Evidence",
            Self::WrittenConclusion => "Sound Written Conclusion",
        }
    }
}

/// Description of a conflict in evidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConflictDescription {
    /// Unique conflict ID
    pub id: EntityId,
    
    /// Type of conflict
    #[validate(length(min = 1))]
    pub conflict_type: String,
    
    /// Description of the conflict
    #[validate(length(min = 1))]
    pub description: String,
    
    /// Evidence items in conflict
    pub conflicting_evidence: Vec<EntityId>,
    
    /// Severity of conflict
    pub severity: ConflictSeverity,
    
    /// How the conflict was identified
    pub identification_method: String,
    
    /// Date identified
    pub identified_date: DateTime<Utc>,
}

/// Severity of evidence conflict
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictSeverity {
    /// Minor discrepancy
    Minor,
    /// Moderate conflict
    Moderate,
    /// Major contradiction
    Major,
    /// Critical conflict affecting conclusions
    Critical,
}

/// Intended audience for the proof
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofAudience {
    /// Self documentation
    Personal,
    /// Family members
    Family,
    /// Peer review
    PeerReview,
    /// Client report
    Client,
    /// Publication
    Publication,
    /// Academic
    Academic,
}

/// Publication status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationStatus {
    /// Not for publication
    Private,
    /// Being prepared
    InPreparation,
    /// Submitted for review
    Submitted,
    /// Accepted for publication
    Accepted,
    /// Published
    Published,
    /// Withdrawn
    Withdrawn,
}

/// Proof statement entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProofStatement {
    /// Base work product
    #[serde(flatten)]
    pub work_product: WorkProduct,
    
    /// Type of proof
    pub proof_type: ProofType,
    
    /// Proof sections
    pub sections: Vec<(String, ProofSection)>,
    
    /// GPS compliance tracking
    pub gps_elements: Vec<(GPSElement, bool)>,
    
    /// Overall GPS compliance
    pub gps_compliant: bool,
    
    /// Conflicts identified
    #[validate(nested)]
    pub conflicts_identified: Vec<ConflictDescription>,
    
    /// Conflict resolutions (conflict_id -> resolution)
    pub resolutions: Vec<(EntityId, String)>,
    
    /// Supporting evidence matrix
    pub evidence_matrix_id: Option<EntityId>,
    
    /// Supporting timeline
    pub timeline_id: Option<EntityId>,
    
    /// Supporting analyses
    pub supporting_analyses: Vec<EntityId>,
    
    /// Intended audience
    pub intended_audience: ProofAudience,
    
    /// Publication status
    pub publication_status: PublicationStatus,
    
    /// Peer reviewers
    pub peer_reviewers: Vec<EntityId>,
    
    /// Version for publication
    pub publication_version: Option<String>,
    
    /// DOI or other permanent identifier
    pub permanent_identifier: Option<String>,
}

impl ProofStatement {
    /// Create a new proof statement
    pub fn new(
        proof_type: ProofType,
        theory_id: EntityId,
        created_by: EntityId,
    ) -> Self {
        let work_product = WorkProduct::new(
            WorkProductType::ProofStatement,
            "gps-proof-statement-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            created_by,
        );
        
        // Initialize GPS elements as not yet satisfied
        let gps_elements = GPSElement::all()
            .into_iter()
            .map(|elem| (elem, false))
            .collect();
        
        Self {
            work_product,
            proof_type,
            sections: vec![],
            gps_elements,
            gps_compliant: false,
            conflicts_identified: vec![],
            resolutions: vec![],
            evidence_matrix_id: None,
            timeline_id: None,
            supporting_analyses: vec![],
            intended_audience: ProofAudience::Personal,
            publication_status: PublicationStatus::Private,
            peer_reviewers: vec![],
            publication_version: None,
            permanent_identifier: None,
        }
    }
    
    /// Create standard GPS proof statement structure
    pub fn new_gps_standard(theory_id: EntityId, created_by: EntityId) -> Self {
        let mut proof = Self::new(ProofType::Statement, theory_id, created_by);
        
        // Add standard sections
        proof.add_section("question", ProofSection::new(
            "Research Question",
            "State the research question clearly and completely."
        ));
        
        proof.add_section("sources", ProofSection::new(
            "Sources Consulted",
            "Document all sources searched, including negative searches."
        ));
        
        proof.add_section("evidence", ProofSection::new(
            "Evidence Summary",
            "Present all relevant evidence discovered."
        ));
        
        proof.add_section("analysis", ProofSection::new(
            "Analysis and Correlation",
            "Analyze and correlate the evidence."
        ));
        
        proof.add_section("conflicts", ProofSection::new(
            "Conflicting Evidence",
            "Identify and resolve any conflicts."
        ));
        
        proof.add_section("conclusion", ProofSection::new(
            "Conclusion",
            "State the conclusion based on the evidence."
        ));
        
        proof
    }
    
    /// Add a section
    pub fn add_section(&mut self, key: impl Into<String>, section: ProofSection) {
        let key = key.into();
        // Remove if exists and add at end
        self.sections.retain(|(k, _)| k != &key);
        self.sections.push((key, section));
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Get a section by key
    pub fn get_section(&self, key: &str) -> Option<&ProofSection> {
        self.sections.iter()
            .find(|(k, _)| k == key)
            .map(|(_, section)| section)
    }
    
    /// Update a section
    pub fn update_section(&mut self, key: &str, content: impl Into<String>) -> bool {
        for (k, section) in &mut self.sections {
            if k == key {
                section.content = content.into();
                self.work_product.metadata.update(self.work_product.metadata.modified_by);
                return true;
            }
        }
        false
    }
    
    /// Mark GPS element as satisfied
    pub fn satisfy_gps_element(&mut self, element: GPSElement, satisfied: bool) {
        for (elem, status) in &mut self.gps_elements {
            if *elem == element {
                *status = satisfied;
                break;
            }
        }
        self.check_gps_compliance();
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Check overall GPS compliance
    fn check_gps_compliance(&mut self) {
        self.gps_compliant = self.gps_elements.iter().all(|(_, satisfied)| *satisfied);
    }
    
    /// Add conflict
    pub fn add_conflict(&mut self, conflict: ConflictDescription) {
        self.conflicts_identified.push(conflict);
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add resolution
    pub fn add_resolution(&mut self, conflict_id: EntityId, resolution: impl Into<String>) {
        self.resolutions.push((conflict_id, resolution.into()));
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Set supporting materials
    pub fn set_supporting_materials(
        &mut self, 
        evidence_matrix: Option<EntityId>,
        timeline: Option<EntityId>,
    ) {
        self.evidence_matrix_id = evidence_matrix;
        self.timeline_id = timeline;
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Add supporting analysis
    pub fn add_supporting_analysis(&mut self, analysis_id: EntityId) {
        if !self.supporting_analyses.contains(&analysis_id) {
            self.supporting_analyses.push(analysis_id);
            self.work_product.metadata.update(self.work_product.metadata.modified_by);
        }
    }
    
    /// Add peer reviewer
    pub fn add_peer_reviewer(&mut self, reviewer_id: EntityId) {
        if !self.peer_reviewers.contains(&reviewer_id) {
            self.peer_reviewers.push(reviewer_id);
            self.work_product.metadata.update(self.work_product.metadata.modified_by);
        }
    }
    
    /// Set publication details
    pub fn set_publication_details(
        &mut self,
        status: PublicationStatus,
        version: Option<String>,
        identifier: Option<String>,
    ) {
        self.publication_status = status;
        self.publication_version = version;
        self.permanent_identifier = identifier;
        self.work_product.metadata.update(self.work_product.metadata.modified_by);
    }
    
    /// Check if ready for peer review
    pub fn is_ready_for_peer_review(&self) -> bool {
        // Must have all standard sections with content
        let required_sections = ["question", "sources", "evidence", "analysis", "conclusion"];
        let has_sections = required_sections.iter().all(|&key| {
            self.get_section(key).map(|s| !s.content.is_empty()).unwrap_or(false)
        });
        
        // Must have evidence
        has_sections && !self.work_product.evidence_ids.is_empty()
    }
    
    /// Get GPS compliance summary
    pub fn gps_compliance_summary(&self) -> serde_json::Value {
        let elements: Vec<serde_json::Value> = self.gps_elements.iter()
            .map(|(elem, satisfied)| {
                serde_json::json!({
                    "element": elem.name(),
                    "satisfied": satisfied,
                })
            })
            .collect();
        
        serde_json::json!({
            "overall_compliant": self.gps_compliant,
            "elements": elements,
            "conflicts_resolved": self.conflicts_identified.len() == self.resolutions.len(),
            "ready_for_review": self.is_ready_for_peer_review(),
        })
    }
}

// Implement Entity trait for ProofStatement
#[async_trait]
impl Entity for ProofStatement {
    fn id(&self) -> EntityId {
        self.work_product.id()
    }
    
    fn entity_type(&self) -> &'static str {
        "ProofStatement"
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

impl_validatable!(ProofStatement);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_proof_statement_creation() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let proof = ProofStatement::new(
            ProofType::Statement,
            theory_id,
            researcher_id,
        );
        
        assert_eq!(proof.proof_type, ProofType::Statement);
        assert!(!proof.gps_compliant);
        assert_eq!(proof.gps_elements.len(), 5);
        assert_eq!(proof.intended_audience, ProofAudience::Personal);
    }
    
    #[test]
    fn test_gps_standard_structure() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let proof = ProofStatement::new_gps_standard(theory_id, researcher_id);
        
        assert_eq!(proof.sections.len(), 6);
        assert!(proof.get_section("question").is_some());
        assert!(proof.get_section("sources").is_some());
        assert!(proof.get_section("evidence").is_some());
        assert!(proof.get_section("analysis").is_some());
        assert!(proof.get_section("conflicts").is_some());
        assert!(proof.get_section("conclusion").is_some());
    }
    
    #[test]
    fn test_section_management() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut proof = ProofStatement::new(ProofType::Statement, theory_id, researcher_id);
        
        let section = ProofSection::new("Test Section", "Initial content");
        proof.add_section("test", section);
        
        assert_eq!(proof.sections.len(), 1);
        assert_eq!(proof.get_section("test").unwrap().title, "Test Section");
        
        // Update section
        assert!(proof.update_section("test", "Updated content"));
        assert_eq!(proof.get_section("test").unwrap().content, "Updated content");
        
        // Update non-existent section
        assert!(!proof.update_section("missing", "Content"));
    }
    
    #[test]
    fn test_gps_compliance() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut proof = ProofStatement::new(ProofType::Statement, theory_id, researcher_id);
        
        // Initially not compliant
        assert!(!proof.gps_compliant);
        
        // Satisfy all elements
        for element in GPSElement::all() {
            proof.satisfy_gps_element(element, true);
        }
        
        assert!(proof.gps_compliant);
        
        // Unsatisfy one element
        proof.satisfy_gps_element(GPSElement::ConflictResolution, false);
        assert!(!proof.gps_compliant);
    }
    
    #[test]
    fn test_conflict_management() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let evidence1 = EntityId::new();
        let evidence2 = EntityId::new();
        
        let mut proof = ProofStatement::new(ProofType::Argument, theory_id, researcher_id);
        
        let conflict = ConflictDescription {
            id: EntityId::new(),
            conflict_type: "Date discrepancy".to_string(),
            description: "Birth date differs by 2 years".to_string(),
            conflicting_evidence: vec![evidence1, evidence2],
            severity: ConflictSeverity::Moderate,
            identification_method: "Direct comparison".to_string(),
            identified_date: Utc::now(),
        };
        
        let conflict_id = conflict.id;
        proof.add_conflict(conflict);
        
        assert_eq!(proof.conflicts_identified.len(), 1);
        
        proof.add_resolution(conflict_id, "Earlier date more reliable based on proximity to event");
        assert_eq!(proof.resolutions.len(), 1);
    }
    
    #[test]
    fn test_supporting_materials() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let matrix_id = EntityId::new();
        let timeline_id = EntityId::new();
        let analysis_id = EntityId::new();
        
        let mut proof = ProofStatement::new(ProofType::Summary, theory_id, researcher_id);
        
        proof.set_supporting_materials(Some(matrix_id), Some(timeline_id));
        proof.add_supporting_analysis(analysis_id);
        
        assert_eq!(proof.evidence_matrix_id, Some(matrix_id));
        assert_eq!(proof.timeline_id, Some(timeline_id));
        assert_eq!(proof.supporting_analyses.len(), 1);
        
        // Test duplicate prevention
        proof.add_supporting_analysis(analysis_id);
        assert_eq!(proof.supporting_analyses.len(), 1);
    }
    
    #[test]
    fn test_publication_workflow() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let reviewer1 = EntityId::new();
        let reviewer2 = EntityId::new();
        
        let mut proof = ProofStatement::new(ProofType::Statement, theory_id, researcher_id);
        
        // Set intended audience
        proof.intended_audience = ProofAudience::Publication;
        
        // Add peer reviewers
        proof.add_peer_reviewer(reviewer1);
        proof.add_peer_reviewer(reviewer2);
        
        assert_eq!(proof.peer_reviewers.len(), 2);
        
        // Set publication details
        proof.set_publication_details(
            PublicationStatus::Submitted,
            Some("v2.1".to_string()),
            None,
        );
        
        assert_eq!(proof.publication_status, PublicationStatus::Submitted);
        assert_eq!(proof.publication_version, Some("v2.1".to_string()));
    }
    
    #[test]
    fn test_ready_for_peer_review() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let evidence_id = EntityId::new();
        
        let mut proof = ProofStatement::new_gps_standard(theory_id, researcher_id);
        
        // Not ready - empty sections
        assert!(!proof.is_ready_for_peer_review());
        
        // Fill required sections
        proof.update_section("question", "What is the birthdate of John Smith?");
        proof.update_section("sources", "Searched vital records...");
        proof.update_section("evidence", "Birth certificate shows...");
        proof.update_section("analysis", "Evidence strongly suggests...");
        proof.update_section("conclusion", "John Smith was born on...");
        
        // Still not ready - no evidence
        assert!(!proof.is_ready_for_peer_review());
        
        // Add evidence
        proof.work_product.add_evidence(evidence_id);
        
        // Now ready
        assert!(proof.is_ready_for_peer_review());
    }
    
    #[test]
    fn test_gps_compliance_summary() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut proof = ProofStatement::new(ProofType::Statement, theory_id, researcher_id);
        
        // Satisfy some elements
        proof.satisfy_gps_element(GPSElement::ExhaustiveResearch, true);
        proof.satisfy_gps_element(GPSElement::CompleteCitations, true);
        
        let summary = proof.gps_compliance_summary();
        
        assert_eq!(summary["overall_compliant"], false);
        assert_eq!(summary["conflicts_resolved"], true); // No conflicts
        
        let elements = summary["elements"].as_array().unwrap();
        assert_eq!(elements.len(), 5);
    }
}