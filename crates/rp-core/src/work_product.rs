//! WorkProduct entity - Base for all research deliverables
//! Part of Layer 2: Research Process Model

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::{EntityMetadata, NestableEntity},
    state::{State, StateMachine, StateTransition},
    EntityId, Error, Result, impl_entity, impl_validatable,
};

/// Types of work products that can be generated
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkProductType {
    /// Research log documenting search activities
    ResearchLog,
    /// GPS-compliant proof statement
    ProofStatement,
    /// Detailed proof argument
    ProofArgument,
    /// Analysis of evidence
    EvidenceAnalysis,
    /// General research report
    ResearchReport,
    /// Timeline visualization
    Timeline,
    /// Correlation matrix of evidence
    CorrelationMatrix,
    /// Detailed source analysis
    SourceAnalysis,
    /// Custom work product type
    Custom(String),
}

/// State of a work product in its lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkProductState {
    /// Initial draft state
    Draft,
    /// Under review
    Review,
    /// Finalized version
    Final,
    /// Published for sharing
    Published,
    /// Archived (no longer active)
    Archived,
}

impl State for WorkProductState {
    fn name(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Review => "Review",
            Self::Final => "Final",
            Self::Published => "Published",
            Self::Archived => "Archived",
        }
    }
    
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Archived)
    }
}

/// Validation status of a work product
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationStatus {
    /// Not yet validated
    Draft,
    /// Validation in progress
    Validating,
    /// Passed validation
    Valid,
    /// Failed validation
    Invalid,
    /// Validation not applicable
    NotApplicable,
}

/// Result of a compliance check
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ComplianceResult {
    /// Name of the standard or rule
    #[validate(length(min = 1))]
    pub standard: String,
    
    /// Whether it passed compliance
    pub passed: bool,
    
    /// Specific checks performed
    pub checks: Vec<ComplianceCheck>,
    
    /// Overall compliance score (0.0 to 1.0)
    #[validate(range(min = 0.0, max = 1.0))]
    pub score: f64,
    
    /// Timestamp of check
    pub checked_at: DateTime<Utc>,
    
    /// Who performed the check
    pub checked_by: EntityId,
}

/// Individual compliance check
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ComplianceCheck {
    /// What was checked
    #[validate(length(min = 1))]
    pub requirement: String,
    
    /// Did it pass
    pub passed: bool,
    
    /// Details about the check
    pub details: Option<String>,
    
    /// Severity if failed
    pub severity: Option<CheckSeverity>,
}

/// Severity of a failed compliance check
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckSeverity {
    /// Must be fixed
    Error,
    /// Should be fixed
    Warning,
    /// Consider fixing
    Info,
}

/// Review of a work product
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WorkProductReview {
    /// Unique review ID
    pub id: EntityId,
    
    /// Who performed the review
    pub reviewer_id: EntityId,
    
    /// When reviewed
    pub reviewed_at: DateTime<Utc>,
    
    /// Review decision
    pub decision: ReviewDecision,
    
    /// Review comments
    pub comments: String,
    
    /// Specific issues found
    pub issues: Vec<ReviewIssue>,
    
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

/// Decision from a review
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewDecision {
    /// Approved as-is
    Approved,
    /// Approved with minor changes
    ApprovedWithChanges,
    /// Needs major revision
    NeedsRevision,
    /// Rejected
    Rejected,
}

/// Issue found during review
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ReviewIssue {
    /// Type of issue
    pub issue_type: IssueType,
    
    /// Severity
    pub severity: CheckSeverity,
    
    /// Description
    #[validate(length(min = 1))]
    pub description: String,
    
    /// Location in content
    pub location: Option<String>,
    
    /// Suggested fix
    pub suggested_fix: Option<String>,
}

/// Types of issues that can be found
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueType {
    /// Factual error
    Factual,
    /// Methodology issue
    Methodology,
    /// Missing evidence
    MissingEvidence,
    /// Formatting problem
    Formatting,
    /// Citation issue
    Citation,
    /// Logic error
    Logic,
    /// Other issue
    Other,
}

/// Base work product entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WorkProduct {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Current state
    pub state: WorkProductState,
    
    /// State transition history
    pub state_history: Vec<StateTransition<WorkProductState>>,
    
    /// Type of work product
    pub product_type: WorkProductType,
    
    /// Template used
    #[validate(length(min = 1))]
    pub template_id: String,
    
    /// Template version
    #[validate(length(min = 1))]
    pub template_version: String,
    
    /// Schema version
    #[validate(length(min = 1))]
    pub schema_version: String,
    
    /// Parent theory/research question
    pub theory_id: EntityId,
    
    /// Referenced evidence
    pub evidence_ids: Vec<EntityId>,
    
    /// Referenced identities
    pub identity_ids: Vec<EntityId>,
    
    /// Referenced analyses
    pub analysis_ids: Vec<EntityId>,
    
    /// Generation metadata
    pub generation_metadata: serde_json::Value,
    
    /// Structured content (based on template)
    pub content: serde_json::Value,
    
    /// Validation status
    pub validation_status: ValidationStatus,
    
    /// Compliance check results
    pub compliance_checks: Vec<ComplianceResult>,
    
    /// Available export formats
    pub export_formats: Vec<String>,
    
    /// Reviews received
    #[validate(nested)]
    pub reviews: Vec<WorkProductReview>,
    
    /// Contributors beyond creator
    pub contributors: Vec<EntityId>,
    
    /// Parent work product (for nested products)
    pub parent_product: Option<EntityId>,
    
    /// Child work products
    pub child_products: Vec<EntityId>,
    
    /// Custom attributes
    pub custom_fields: serde_json::Value,
    
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl WorkProduct {
    /// Create a new work product
    pub fn new(
        product_type: WorkProductType,
        template_id: impl Into<String>,
        template_version: impl Into<String>,
        schema_version: impl Into<String>,
        theory_id: EntityId,
        created_by: EntityId,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata {
                id: EntityId::new(),
                created_by,
                created_at: now,
                modified_by: created_by,
                modified_at: now,
                is_active: true,
                version: 1,
                parent_version: None,
            },
            state: WorkProductState::Draft,
            state_history: vec![],
            product_type,
            template_id: template_id.into(),
            template_version: template_version.into(),
            schema_version: schema_version.into(),
            theory_id,
            evidence_ids: vec![],
            identity_ids: vec![],
            analysis_ids: vec![],
            generation_metadata: serde_json::Value::Object(serde_json::Map::new()),
            content: serde_json::Value::Object(serde_json::Map::new()),
            validation_status: ValidationStatus::Draft,
            compliance_checks: vec![],
            export_formats: vec!["markdown".to_string(), "json".to_string()],
            reviews: vec![],
            contributors: vec![],
            parent_product: None,
            child_products: vec![],
            custom_fields: serde_json::Value::Object(serde_json::Map::new()),
            tags: vec![],
        }
    }
    
    /// Add evidence reference
    pub fn add_evidence(&mut self, evidence_id: EntityId) {
        if !self.evidence_ids.contains(&evidence_id) {
            self.evidence_ids.push(evidence_id);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Add identity reference
    pub fn add_identity(&mut self, identity_id: EntityId) {
        if !self.identity_ids.contains(&identity_id) {
            self.identity_ids.push(identity_id);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Add analysis reference
    pub fn add_analysis(&mut self, analysis_id: EntityId) {
        if !self.analysis_ids.contains(&analysis_id) {
            self.analysis_ids.push(analysis_id);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Add contributor
    pub fn add_contributor(&mut self, contributor_id: EntityId) {
        if !self.contributors.contains(&contributor_id) {
            self.contributors.push(contributor_id);
            self.metadata.update(contributor_id);
        }
    }
    
    /// Add compliance check result
    pub fn add_compliance_check(&mut self, result: ComplianceResult) {
        self.compliance_checks.push(result);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Add review
    pub fn add_review(&mut self, review: WorkProductReview) {
        self.reviews.push(review);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Check if ready for review
    pub fn is_ready_for_review(&self) -> bool {
        self.state == WorkProductState::Draft 
            && self.validation_status == ValidationStatus::Valid
    }
    
    /// Check if approved
    pub fn is_approved(&self) -> bool {
        self.reviews.iter().any(|r| 
            matches!(r.decision, ReviewDecision::Approved | ReviewDecision::ApprovedWithChanges)
        )
    }
    
    /// Get latest review
    pub fn latest_review(&self) -> Option<&WorkProductReview> {
        self.reviews.iter()
            .max_by_key(|r| r.reviewed_at)
    }
    
    /// Check overall compliance
    pub fn overall_compliance_score(&self) -> Option<f64> {
        if self.compliance_checks.is_empty() {
            return None;
        }
        
        let total: f64 = self.compliance_checks.iter()
            .map(|c| c.score)
            .sum();
        
        Some(total / self.compliance_checks.len() as f64)
    }
}

impl_entity!(WorkProduct, "WorkProduct");
impl_validatable!(WorkProduct);

// Implement StateMachine trait
#[async_trait]
impl StateMachine for WorkProduct {
    type State = WorkProductState;
    
    fn current_state(&self) -> &Self::State {
        &self.state
    }
    
    fn state_history(&self) -> Vec<StateTransition<Self::State>> {
        self.state_history.clone()
    }
    
    fn can_transition(&self, to: &Self::State) -> bool {
        match (&self.state, to) {
            (WorkProductState::Draft, WorkProductState::Review) => true,
            (WorkProductState::Draft, WorkProductState::Archived) => true,
            (WorkProductState::Review, WorkProductState::Draft) => true,
            (WorkProductState::Review, WorkProductState::Final) => true,
            (WorkProductState::Review, WorkProductState::Archived) => true,
            (WorkProductState::Final, WorkProductState::Published) => true,
            (WorkProductState::Final, WorkProductState::Review) => true,
            (WorkProductState::Final, WorkProductState::Archived) => true,
            (WorkProductState::Published, WorkProductState::Archived) => true,
            _ => false,
        }
    }
    
    async fn transition(
        &mut self,
        to: Self::State,
        triggered_by: EntityId,
        reason: Option<String>,
    ) -> Result<()> {
        if !self.can_transition(&to) {
            return Err(Error::InvalidStateTransition(
                format!("Cannot transition from {:?} to {:?}", self.state, to)
            ));
        }
        
        let transition = StateTransition {
            from: self.state.clone(),
            to: to.clone(),
            triggered_by,
            timestamp: Utc::now(),
            reason,
        };
        
        self.state_history.push(transition);
        self.state = to;
        self.metadata.update(triggered_by);
        
        Ok(())
    }
    
    fn valid_transitions(&self) -> Vec<Self::State> {
        match self.state {
            WorkProductState::Draft => vec![
                WorkProductState::Review,
                WorkProductState::Archived,
            ],
            WorkProductState::Review => vec![
                WorkProductState::Draft,
                WorkProductState::Final,
                WorkProductState::Archived,
            ],
            WorkProductState::Final => vec![
                WorkProductState::Published,
                WorkProductState::Review,
                WorkProductState::Archived,
            ],
            WorkProductState::Published => vec![
                WorkProductState::Archived,
            ],
            WorkProductState::Archived => vec![],
        }
    }
}

#[async_trait]
impl NestableEntity for WorkProduct {
    fn children(&self) -> Vec<EntityId> {
        self.child_products.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        // Work products can contain other work products
        entity_type == "WorkProduct"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if !self.child_products.contains(&child_id) {
            self.child_products.push(child_id);
            self.metadata.update(self.metadata.modified_by);
        }
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_products.len();
        self.child_products.retain(|id| id != &child_id);
        if self.child_products.len() < initial_len {
            self.metadata.update(self.metadata.modified_by);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_work_product_creation() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let product = WorkProduct::new(
            WorkProductType::ProofStatement,
            "gps-proof-statement-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            researcher_id,
        );
        
        assert_eq!(product.product_type, WorkProductType::ProofStatement);
        assert_eq!(product.state, WorkProductState::Draft);
        assert_eq!(product.theory_id, theory_id);
        assert_eq!(product.validation_status, ValidationStatus::Draft);
    }
    
    #[test]
    fn test_add_references() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let evidence_id = EntityId::new();
        let identity_id = EntityId::new();
        let analysis_id = EntityId::new();
        
        let mut product = WorkProduct::new(
            WorkProductType::EvidenceAnalysis,
            "evidence-analysis-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            researcher_id,
        );
        
        product.add_evidence(evidence_id);
        product.add_identity(identity_id);
        product.add_analysis(analysis_id);
        
        assert_eq!(product.evidence_ids.len(), 1);
        assert_eq!(product.identity_ids.len(), 1);
        assert_eq!(product.analysis_ids.len(), 1);
        
        // Test duplicate prevention
        product.add_evidence(evidence_id);
        assert_eq!(product.evidence_ids.len(), 1);
    }
    
    #[test]
    fn test_compliance_checks() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut product = WorkProduct::new(
            WorkProductType::ProofStatement,
            "gps-proof-statement-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            researcher_id,
        );
        
        let check1 = ComplianceCheck {
            requirement: "Must cite all sources".to_string(),
            passed: true,
            details: Some("All 5 sources properly cited".to_string()),
            severity: None,
        };
        
        let check2 = ComplianceCheck {
            requirement: "Must analyze conflicts".to_string(),
            passed: false,
            details: Some("Conflict between sources 2 and 3 not addressed".to_string()),
            severity: Some(CheckSeverity::Error),
        };
        
        let compliance = ComplianceResult {
            standard: "GPS-2025".to_string(),
            passed: false,
            checks: vec![check1, check2],
            score: 0.5,
            checked_at: Utc::now(),
            checked_by: researcher_id,
        };
        
        product.add_compliance_check(compliance);
        assert_eq!(product.compliance_checks.len(), 1);
        assert_eq!(product.overall_compliance_score(), Some(0.5));
    }
    
    #[test]
    fn test_reviews() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        let reviewer_id = EntityId::new();
        
        let mut product = WorkProduct::new(
            WorkProductType::ResearchReport,
            "research-report-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            researcher_id,
        );
        
        let issue = ReviewIssue {
            issue_type: IssueType::Citation,
            severity: CheckSeverity::Warning,
            description: "Missing page number in citation 3".to_string(),
            location: Some("Section 2.3".to_string()),
            suggested_fix: Some("Add page number from source document".to_string()),
        };
        
        let review = WorkProductReview {
            id: EntityId::new(),
            reviewer_id,
            reviewed_at: Utc::now(),
            decision: ReviewDecision::ApprovedWithChanges,
            comments: "Good analysis, minor citation fixes needed".to_string(),
            issues: vec![issue],
            suggestions: vec!["Consider adding timeline visualization".to_string()],
        };
        
        product.add_review(review);
        assert_eq!(product.reviews.len(), 1);
        assert!(product.is_approved());
        assert!(product.latest_review().is_some());
    }
    
    #[tokio::test]
    async fn test_state_transitions() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut product = WorkProduct::new(
            WorkProductType::ProofStatement,
            "gps-proof-statement-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            researcher_id,
        );
        
        // Valid transition: Draft -> Review
        assert!(product.transition(
            WorkProductState::Review,
            researcher_id,
            Some("Ready for peer review".to_string())
        ).await.is_ok());
        assert_eq!(product.state, WorkProductState::Review);
        
        // Valid transition: Review -> Final
        assert!(product.transition(
            WorkProductState::Final,
            researcher_id,
            Some("Approved by reviewer".to_string())
        ).await.is_ok());
        assert_eq!(product.state, WorkProductState::Final);
        
        // Invalid transition: Final -> Draft
        assert!(product.transition(
            WorkProductState::Draft,
            researcher_id,
            None
        ).await.is_err());
        
        // Valid transition: Final -> Published
        assert!(product.transition(
            WorkProductState::Published,
            researcher_id,
            Some("Publishing to repository".to_string())
        ).await.is_ok());
        
        assert_eq!(product.state_history.len(), 3);
    }
    
    #[test]
    fn test_ready_for_review() {
        let theory_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut product = WorkProduct::new(
            WorkProductType::ProofStatement,
            "gps-proof-statement-v1",
            "1.0.0",
            "2025.1",
            theory_id,
            researcher_id,
        );
        
        // Not ready - validation not complete
        assert!(!product.is_ready_for_review());
        
        // Mark as validated
        product.validation_status = ValidationStatus::Valid;
        assert!(product.is_ready_for_review());
        
        // Change state - no longer ready
        product.state = WorkProductState::Review;
        assert!(!product.is_ready_for_review());
    }
}