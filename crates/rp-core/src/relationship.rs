//! Relationship entity - connections between IdentityPersona entities

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::EntityMetadata,
    state::{State, StateMachine, StateTransition},
    EntityId, Error, Result, impl_entity, impl_validatable,
};

/// Types of relationships between personas
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Parent-child relationships
    Parent,
    Child,
    
    /// Spousal relationships
    Spouse,
    ExSpouse,
    
    /// Sibling relationships
    Sibling,
    HalfSibling,
    StepSibling,
    
    /// Extended family
    Grandparent,
    Grandchild,
    Uncle,
    Aunt,
    Nephew,
    Niece,
    Cousin,
    
    /// In-law relationships
    ParentInLaw,
    ChildInLaw,
    SiblingInLaw,
    
    /// Step relationships
    StepParent,
    StepChild,
    
    /// Adoptive relationships
    AdoptiveParent,
    AdoptiveChild,
    
    /// Foster relationships
    FosterParent,
    FosterChild,
    
    /// Guardian relationships
    Guardian,
    Ward,
    
    /// Professional relationships
    Employer,
    Employee,
    BusinessPartner,
    Apprentice,
    Master,
    
    /// Social relationships
    Friend,
    Neighbor,
    Associate,
    
    /// Military relationships
    CommandingOfficer,
    Subordinate,
    Comrade,
    
    /// Religious relationships
    Godparent,
    Godchild,
    
    /// Legal relationships
    Executor,
    Beneficiary,
    Witness,
    
    /// Custom relationship
    Custom(String),
}

/// State of a relationship in the research process
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipState {
    /// Proposed relationship based on initial evidence
    Proposed,
    /// Under investigation
    Investigating,
    /// Verified through evidence
    Verified,
    /// Proven with high confidence
    Proven,
    /// Disputed or conflicting evidence
    Disputed,
    /// Disproven
    Disproven,
}

impl State for RelationshipState {
    fn name(&self) -> &'static str {
        match self {
            Self::Proposed => "Proposed",
            Self::Investigating => "Investigating",
            Self::Verified => "Verified",
            Self::Proven => "Proven",
            Self::Disputed => "Disputed",
            Self::Disproven => "Disproven",
        }
    }
    
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Proven | Self::Disproven)
    }
}

/// Time period when relationship was active
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RelationshipPeriod {
    /// When the relationship started
    pub start_date: Option<DateTime<Utc>>,
    
    /// When the relationship ended (if applicable)
    pub end_date: Option<DateTime<Utc>>,
    
    /// Precision of dates
    pub date_precision: String,
    
    /// Notes about the time period
    pub notes: Option<String>,
}

/// Evidence supporting or contradicting a relationship
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RelationshipEvidence {
    /// Reference to Evidence entity
    pub evidence_id: EntityId,
    
    /// How this evidence relates to the relationship
    pub relationship_type: EvidenceRelationType,
    
    /// Specific details from the evidence
    pub details: Option<String>,
    
    /// Confidence in this evidence
    pub confidence: f32,
}

/// How evidence relates to a relationship
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceRelationType {
    /// Directly states the relationship
    Direct,
    /// Implies the relationship
    Indirect,
    /// Contradicts the relationship
    Contradictory,
    /// Provides context
    Contextual,
}

/// A relationship between two IdentityPersona entities
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Relationship {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Current state of the relationship
    pub state: RelationshipState,
    
    /// State transition history
    pub state_history: Vec<StateTransition<RelationshipState>>,
    
    /// Type of relationship
    pub relationship_type: RelationshipType,
    
    /// First person in the relationship
    pub persona1_id: EntityId,
    
    /// Second person in the relationship
    pub persona2_id: EntityId,
    
    /// Role of persona1 (e.g., "father" in parent-child)
    pub persona1_role: String,
    
    /// Role of persona2 (e.g., "son" in parent-child)
    pub persona2_role: String,
    
    /// Time period when relationship was active
    pub period: Option<RelationshipPeriod>,
    
    /// Supporting evidence
    #[validate(nested)]
    pub evidence: Vec<RelationshipEvidence>,
    
    /// Confidence assessment
    pub confidence_id: Option<EntityId>,
    
    /// Related analyses
    pub analysis_ids: Vec<EntityId>,
    
    /// Theory this relationship supports
    pub theory_id: Option<EntityId>,
    
    /// Is this the reciprocal of another relationship?
    pub reciprocal_of: Option<EntityId>,
    
    /// Cultural context
    pub cultural_context: Option<String>,
    
    /// Legal implications
    pub legal_implications: Option<String>,
    
    /// Research notes
    pub notes: Option<String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Relationship {
    /// Create a new relationship
    pub fn new(
        relationship_type: RelationshipType,
        persona1_id: EntityId,
        persona1_role: impl Into<String>,
        persona2_id: EntityId,
        persona2_role: impl Into<String>,
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
            state: RelationshipState::Proposed,
            state_history: vec![],
            relationship_type,
            persona1_id,
            persona2_id,
            persona1_role: persona1_role.into(),
            persona2_role: persona2_role.into(),
            period: None,
            evidence: vec![],
            confidence_id: None,
            analysis_ids: vec![],
            theory_id: None,
            reciprocal_of: None,
            cultural_context: None,
            legal_implications: None,
            notes: None,
            tags: vec![],
        }
    }
    
    /// Create a parent-child relationship
    pub fn parent_child(
        parent_id: EntityId,
        child_id: EntityId,
        created_by: EntityId,
    ) -> Self {
        Self::new(
            RelationshipType::Parent,
            parent_id,
            "parent",
            child_id,
            "child",
            created_by,
        )
    }
    
    /// Create a spousal relationship
    pub fn spouse(
        spouse1_id: EntityId,
        spouse2_id: EntityId,
        created_by: EntityId,
    ) -> Self {
        Self::new(
            RelationshipType::Spouse,
            spouse1_id,
            "spouse",
            spouse2_id,
            "spouse",
            created_by,
        )
    }
    
    /// Create a sibling relationship
    pub fn sibling(
        sibling1_id: EntityId,
        sibling2_id: EntityId,
        created_by: EntityId,
    ) -> Self {
        Self::new(
            RelationshipType::Sibling,
            sibling1_id,
            "sibling",
            sibling2_id,
            "sibling",
            created_by,
        )
    }
    
    /// Add supporting evidence
    pub fn add_evidence(&mut self, evidence: RelationshipEvidence) {
        self.evidence.push(evidence);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Set the time period
    pub fn set_period(&mut self, period: RelationshipPeriod) {
        self.period = Some(period);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Check if relationship is verified
    pub fn is_verified(&self) -> bool {
        matches!(self.state, RelationshipState::Verified | RelationshipState::Proven)
    }
    
    /// Check if relationship is disputed
    pub fn is_disputed(&self) -> bool {
        matches!(self.state, RelationshipState::Disputed)
    }
    
    /// Get display string for the relationship
    pub fn display_relationship(&self) -> String {
        format!(
            "{} is {} of {}",
            self.persona1_id,
            self.persona1_role,
            self.persona2_id
        )
    }
    
    /// Create the reciprocal relationship
    pub fn create_reciprocal(&self, created_by: EntityId) -> Self {
        let mut reciprocal = Self::new(
            self.relationship_type.clone(),
            self.persona2_id,
            &self.persona2_role,
            self.persona1_id,
            &self.persona1_role,
            created_by,
        );
        
        reciprocal.reciprocal_of = Some(self.metadata.id);
        reciprocal.period = self.period.clone();
        reciprocal.evidence = self.evidence.clone();
        reciprocal.confidence_id = self.confidence_id;
        reciprocal.theory_id = self.theory_id;
        reciprocal.cultural_context = self.cultural_context.clone();
        reciprocal.legal_implications = self.legal_implications.clone();
        reciprocal.notes = self.notes.clone();
        reciprocal.tags = self.tags.clone();
        
        reciprocal
    }
}

impl_entity!(Relationship, "Relationship");
impl_validatable!(Relationship);

// Implement StateMachine trait
#[async_trait]
impl StateMachine for Relationship {
    type State = RelationshipState;
    
    fn current_state(&self) -> &Self::State {
        &self.state
    }
    
    fn state_history(&self) -> Vec<StateTransition<Self::State>> {
        self.state_history.clone()
    }
    
    fn can_transition(&self, to: &Self::State) -> bool {
        match (&self.state, to) {
            (RelationshipState::Proposed, RelationshipState::Investigating) => true,
            (RelationshipState::Proposed, RelationshipState::Disputed) => true,
            (RelationshipState::Proposed, RelationshipState::Disproven) => true,
            (RelationshipState::Investigating, RelationshipState::Verified) => true,
            (RelationshipState::Investigating, RelationshipState::Disputed) => true,
            (RelationshipState::Investigating, RelationshipState::Disproven) => true,
            (RelationshipState::Verified, RelationshipState::Proven) => true,
            (RelationshipState::Verified, RelationshipState::Disputed) => true,
            (RelationshipState::Disputed, RelationshipState::Investigating) => true,
            (RelationshipState::Disputed, RelationshipState::Verified) => true,
            (RelationshipState::Disputed, RelationshipState::Disproven) => true,
            (RelationshipState::Proven, RelationshipState::Disputed) => true,
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
            RelationshipState::Proposed => vec![
                RelationshipState::Investigating,
                RelationshipState::Disputed,
                RelationshipState::Disproven,
            ],
            RelationshipState::Investigating => vec![
                RelationshipState::Verified,
                RelationshipState::Disputed,
                RelationshipState::Disproven,
            ],
            RelationshipState::Verified => vec![
                RelationshipState::Proven,
                RelationshipState::Disputed,
            ],
            RelationshipState::Disputed => vec![
                RelationshipState::Investigating,
                RelationshipState::Verified,
                RelationshipState::Disproven,
            ],
            RelationshipState::Proven => vec![RelationshipState::Disputed],
            RelationshipState::Disproven => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_relationship_creation() {
        let persona1_id = EntityId::new();
        let persona2_id = EntityId::new();
        let creator_id = EntityId::new();
        
        let relationship = Relationship::parent_child(
            persona1_id,
            persona2_id,
            creator_id,
        );
        
        assert_eq!(relationship.relationship_type, RelationshipType::Parent);
        assert_eq!(relationship.persona1_id, persona1_id);
        assert_eq!(relationship.persona2_id, persona2_id);
        assert_eq!(relationship.persona1_role, "parent");
        assert_eq!(relationship.persona2_role, "child");
        assert_eq!(relationship.state, RelationshipState::Proposed);
    }
    
    #[test]
    fn test_relationship_evidence() {
        let persona1_id = EntityId::new();
        let persona2_id = EntityId::new();
        let creator_id = EntityId::new();
        let evidence_id = EntityId::new();
        
        let mut relationship = Relationship::spouse(
            persona1_id,
            persona2_id,
            creator_id,
        );
        
        let evidence = RelationshipEvidence {
            evidence_id,
            relationship_type: EvidenceRelationType::Direct,
            details: Some("Marriage certificate dated 1875".to_string()),
            confidence: 0.95,
        };
        
        relationship.add_evidence(evidence);
        assert_eq!(relationship.evidence.len(), 1);
        assert_eq!(relationship.evidence[0].confidence, 0.95);
    }
    
    #[tokio::test]
    async fn test_state_transitions() {
        let persona1_id = EntityId::new();
        let persona2_id = EntityId::new();
        let creator_id = EntityId::new();
        
        let mut relationship = Relationship::sibling(
            persona1_id,
            persona2_id,
            creator_id,
        );
        
        // Valid transition
        assert!(relationship.transition(
            RelationshipState::Investigating,
            creator_id,
            Some("Starting investigation".to_string())
        ).await.is_ok());
        
        assert_eq!(relationship.state, RelationshipState::Investigating);
        
        // Invalid transition
        assert!(relationship.transition(
            RelationshipState::Proven,
            creator_id,
            None
        ).await.is_err());
    }
    
    #[test]
    fn test_reciprocal_relationship() {
        let parent_id = EntityId::new();
        let child_id = EntityId::new();
        let creator_id = EntityId::new();
        
        let parent_child = Relationship::parent_child(
            parent_id,
            child_id,
            creator_id,
        );
        
        let child_parent = parent_child.create_reciprocal(creator_id);
        
        assert_eq!(child_parent.persona1_id, child_id);
        assert_eq!(child_parent.persona2_id, parent_id);
        assert_eq!(child_parent.persona1_role, "child");
        assert_eq!(child_parent.persona2_role, "parent");
        assert_eq!(child_parent.reciprocal_of, Some(parent_child.metadata.id));
    }
    
    #[test]
    fn test_relationship_period() {
        let persona1_id = EntityId::new();
        let persona2_id = EntityId::new();
        let creator_id = EntityId::new();
        
        let mut relationship = Relationship::spouse(
            persona1_id,
            persona2_id,
            creator_id,
        );
        
        let period = RelationshipPeriod {
            start_date: Some(Utc::now()),
            end_date: None,
            date_precision: "Exact".to_string(),
            notes: Some("Married in Boston".to_string()),
        };
        
        relationship.set_period(period);
        assert!(relationship.period.is_some());
        assert_eq!(
            relationship.period.as_ref().unwrap().notes,
            Some("Married in Boston".to_string())
        );
    }
    
    #[test]
    fn test_custom_relationship_type() {
        let persona1_id = EntityId::new();
        let persona2_id = EntityId::new();
        let creator_id = EntityId::new();
        
        let relationship = Relationship::new(
            RelationshipType::Custom("Mentor".to_string()),
            persona1_id,
            "mentor",
            persona2_id,
            "protégé",
            creator_id,
        );
        
        assert!(matches!(
            relationship.relationship_type,
            RelationshipType::Custom(ref s) if s == "Mentor"
        ));
    }
}