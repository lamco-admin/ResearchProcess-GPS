//! IdentityPersona entity - unified person entity from hypothetical to concluded
//!
//! This replaces the old Person/IdentityPersona split with a single entity that uses states
//! to track research progression from Reference → Working → Hypothesis → Concluded → 
//! Verified → Published → Challenged.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    define_states,
    entity::{EntityMetadata, NestableEntity},
    state::{StateMachine, StateTransition},
    validation::{Validatable, ValidationResult},
    EntityId, Error, Result, impl_entity,
};

// Define identity persona states using our macro
define_states! {
    /// States in the identity persona lifecycle
    pub enum IdentityState {
        /// Initial reference - minimal information from source
        Reference,
        
        /// Working state - actively researching this identity
        Working,
        
        /// Hypothesis state - proposed identity based on analysis
        Hypothesis,
        
        /// Concluded state - sufficient evidence for conclusion
        Concluded,
        
        /// Verified state - peer reviewed or independently verified
        Verified,
        
        /// Published state - formal publication or sharing
        Published,
        
        /// Challenged state - conclusion questioned, needs review
        Challenged,
    }
}

impl IdentityState {
    /// Check if this is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Published)
    }
    
    /// Check if this identity can have facts and relationships
    pub fn is_concluded(&self) -> bool {
        matches!(
            self,
            Self::Concluded | Self::Verified | Self::Published
        )
    }
    
    /// Get valid transitions from this state
    pub fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Reference => vec![Self::Working, Self::Hypothesis],
            Self::Working => vec![
                Self::Reference, 
                Self::Hypothesis, 
                Self::Concluded
            ],
            Self::Hypothesis => vec![
                Self::Working, 
                Self::Concluded, 
                Self::Challenged
            ],
            Self::Concluded => vec![
                Self::Verified, 
                Self::Challenged, 
                Self::Published
            ],
            Self::Verified => vec![
                Self::Published, 
                Self::Challenged
            ],
            Self::Published => vec![Self::Challenged],
            Self::Challenged => vec![
                Self::Working, 
                Self::Hypothesis, 
                Self::Concluded
            ],
        }
    }
}

/// Type of identity based on available information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityType {
    /// Person identified by name
    Named,
    /// Person described but not named ("the father", "eldest son")
    Described,
    /// Person identified only through relationship
    Relationship,
    /// Anonymous person in record
    Anonymous,
    /// Hypothetical person (inferred but not documented)
    Hypothetical,
}

/// Reference to evidence with extracted attributes
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EvidenceReference {
    /// ID of the evidence entity
    pub evidence_id: EntityId,
    
    /// What was extracted about this identity from this evidence
    #[validate(length(min = 1))]
    pub extracted_text: String,
    
    /// Normalized/interpreted value
    pub interpreted_value: String,
    
    /// Location within the evidence
    pub location: String,
    
    /// Context surrounding the reference
    pub context: String,
    
    /// Confidence in the extraction/interpretation
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: f32,
    
    /// Who extracted this information
    pub extracted_by: EntityId,
    
    /// When it was extracted
    pub extracted_at: DateTime<Utc>,
}

impl EvidenceReference {
    pub fn new(
        evidence_id: EntityId,
        extracted_text: impl Into<String>,
        extracted_by: EntityId,
    ) -> Self {
        let text = extracted_text.into();
        Self {
            evidence_id,
            extracted_text: text.clone(),
            interpreted_value: text,
            location: String::new(),
            context: String::new(),
            confidence: 0.8,
            extracted_by,
            extracted_at: Utc::now(),
        }
    }
}

/// Geographic scope for identity research
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GeographicScope {
    #[validate(length(min = 1))]
    pub description: String,
    pub places: Vec<String>,
    pub coordinates: Option<(f64, f64)>, // (latitude, longitude)
}

/// Temporal scope for identity research
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TemporalScope {
    #[validate(length(min = 1))]
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub estimated: bool, // Whether dates are estimated
}

/// Unified entity for all person references - from hypothetical to concluded
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct IdentityPersona {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Current state of the identity
    pub state: IdentityState,
    
    /// State transition history
    pub state_history: Vec<StateTransition<IdentityState>>,
    
    /// Type of identity based on available information
    pub identity_type: IdentityType,
    
    /// Primary name or description
    #[validate(length(min = 1, max = 500))]
    pub primary_name: String,
    
    /// Alternative names and variations
    pub alternative_names: Vec<String>,
    
    /// Description when name is unknown
    pub description: Option<String>,
    
    /// Evidence references with extracted attributes
    pub evidence_references: Vec<EvidenceReference>,
    
    /// Related research entities (references only, no embedding)
    pub theory_refs: Vec<EntityId>,
    pub confidence_refs: Vec<EntityId>,
    pub analysis_refs: Vec<EntityId>,
    
    /// Facts and relationships (only valid when concluded)
    /// These are references to Fact and Relationship entities
    pub fact_refs: Vec<EntityId>,
    pub relationship_refs: Vec<EntityId>,
    
    /// Geographic scope of research
    pub geographic_scope: Option<GeographicScope>,
    
    /// Temporal scope of research
    pub temporal_scope: Option<TemporalScope>,
    
    /// Research priority (1-5, 1 being highest)
    #[validate(range(min = 1, max = 5))]
    pub priority: u8,
    
    /// Tags for categorization and organization
    pub tags: Vec<String>,
    
    /// Research notes
    pub notes: Option<String>,
    
    /// Nesting support for identity grouping
    pub parent_identity: Option<EntityId>,
    pub child_identities: Vec<EntityId>,
    
    /// Attribution
    pub discovered_by: EntityId,
    pub discovery_date: DateTime<Utc>,
}

impl IdentityPersona {
    /// Create a new identity persona
    pub fn new(
        primary_name: impl Into<String>,
        identity_type: IdentityType,
        discovered_by: EntityId,
    ) -> Self {
        let initial_state = IdentityState::Reference;
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata::new(discovered_by),
            state: initial_state.clone(),
            state_history: vec![StateTransition {
                from: initial_state.clone(),
                to: initial_state,
                triggered_by: discovered_by,
                timestamp: now,
                reason: Some("Identity persona created".to_string()),
            }],
            identity_type,
            primary_name: primary_name.into(),
            alternative_names: Vec::new(),
            description: None,
            evidence_references: Vec::new(),
            theory_refs: Vec::new(),
            confidence_refs: Vec::new(),
            analysis_refs: Vec::new(),
            fact_refs: Vec::new(),
            relationship_refs: Vec::new(),
            geographic_scope: None,
            temporal_scope: None,
            priority: 3, // Medium priority by default
            tags: Vec::new(),
            notes: None,
            parent_identity: None,
            child_identities: Vec::new(),
            discovered_by,
            discovery_date: now,
        }
    }
    
    /// Check if this identity is concluded and can have facts/relationships
    pub fn is_concluded(&self) -> bool {
        self.state.is_concluded()
    }
    
    /// Check if this identity can have relationships
    pub fn can_have_relationships(&self) -> bool {
        self.is_concluded()
    }
    
    /// Check if this identity can have facts
    pub fn can_have_facts(&self) -> bool {
        self.is_concluded()
    }
    
    /// Add an evidence reference
    pub fn add_evidence_reference(&mut self, evidence_ref: EvidenceReference) {
        self.evidence_references.push(evidence_ref);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Add a theory reference
    pub fn add_theory_reference(&mut self, theory_id: EntityId) {
        if !self.theory_refs.contains(&theory_id) {
            self.theory_refs.push(theory_id);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Add an analysis reference
    pub fn add_analysis_reference(&mut self, analysis_id: EntityId) {
        if !self.analysis_refs.contains(&analysis_id) {
            self.analysis_refs.push(analysis_id);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Add a fact reference (only if concluded)
    pub fn add_fact_reference(&mut self, fact_id: EntityId) -> Result<()> {
        if !self.can_have_facts() {
            return Err(Error::InvalidStateTransition(
                format!("Cannot add facts to identity in state {:?}", self.state)
            ));
        }
        
        if !self.fact_refs.contains(&fact_id) {
            self.fact_refs.push(fact_id);
            self.metadata.update(self.metadata.modified_by);
        }
        Ok(())
    }
    
    /// Add a relationship reference (only if concluded)
    pub fn add_relationship_reference(&mut self, relationship_id: EntityId) -> Result<()> {
        if !self.can_have_relationships() {
            return Err(Error::InvalidStateTransition(
                format!("Cannot add relationships to identity in state {:?}", self.state)
            ));
        }
        
        if !self.relationship_refs.contains(&relationship_id) {
            self.relationship_refs.push(relationship_id);
            self.metadata.update(self.metadata.modified_by);
        }
        Ok(())
    }
    
    /// Add an alternative name
    pub fn add_alternative_name(&mut self, name: impl Into<String>) {
        let name = name.into();
        if !self.alternative_names.contains(&name) {
            self.alternative_names.push(name);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Get all names (primary + alternatives)
    pub fn all_names(&self) -> Vec<&str> {
        let mut names = vec![self.primary_name.as_str()];
        names.extend(self.alternative_names.iter().map(|s| s.as_str()));
        names
    }
    
    /// Get display name for UI
    pub fn display_name(&self) -> String {
        match self.identity_type {
            IdentityType::Named => self.primary_name.clone(),
            IdentityType::Described => {
                self.description.as_ref()
                    .unwrap_or(&self.primary_name)
                    .clone()
            },
            IdentityType::Relationship => {
                format!("{} (relationship-based)", self.primary_name)
            },
            IdentityType::Anonymous => {
                format!("Anonymous person ({})", self.primary_name)
            },
            IdentityType::Hypothetical => {
                format!("{} (hypothetical)", self.primary_name)
            },
        }
    }
}

// Implement Entity trait
impl_entity!(IdentityPersona, "IdentityPersona");

// Implement StateMachine trait
#[async_trait]
impl StateMachine for IdentityPersona {
    type State = IdentityState;
    
    fn current_state(&self) -> &Self::State {
        &self.state
    }
    
    fn state_history(&self) -> Vec<StateTransition<Self::State>> {
        self.state_history.clone()
    }
    
    fn can_transition(&self, to: &Self::State) -> bool {
        self.state.valid_transitions().contains(to)
    }
    
    async fn transition(
        &mut self,
        to: Self::State,
        triggered_by: EntityId,
        reason: Option<String>,
    ) -> Result<()> {
        if !self.can_transition(&to) {
            return Err(Error::InvalidStateTransition(format!(
                "Cannot transition from {:?} to {:?}",
                self.state, to
            )));
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
        self.state.valid_transitions()
    }
}

// Implement NestableEntity for hierarchical identities
#[async_trait]
impl NestableEntity for IdentityPersona {
    fn children(&self) -> Vec<EntityId> {
        self.child_identities.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "IdentityPersona"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if self.child_identities.contains(&child_id) {
            return Ok(());
        }
        self.child_identities.push(child_id);
        self.metadata.update(self.metadata.modified_by);
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_identities.len();
        self.child_identities.retain(|id| id != &child_id);
        if self.child_identities.len() < initial_len {
            self.metadata.update(self.metadata.modified_by);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// Implement Validatable
#[async_trait]
impl Validatable for IdentityPersona {
    async fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Basic validation using validator crate
        if let Err(e) = <IdentityPersona as validator::Validate>::validate(self) {
            result.merge(e.into());
        }
        
        // Custom validation
        if self.primary_name.trim().is_empty() {
            result.add_error(
                "primary_name",
                "empty",
                "Primary name cannot be empty"
            );
        }
        
        // State-specific validation
        if !self.can_have_facts() && !self.fact_refs.is_empty() {
            result.add_error(
                "fact_refs",
                "invalid_state",
                "Facts can only be assigned to concluded identities"
            );
        }
        
        if !self.can_have_relationships() && !self.relationship_refs.is_empty() {
            result.add_error(
                "relationship_refs",
                "invalid_state",
                "Relationships can only be assigned to concluded identities"
            );
        }
        
        // Geographic scope validation
        if let Some(temporal) = &self.temporal_scope {
            if let (Some(start), Some(end)) = (temporal.start_date, temporal.end_date) {
                if start > end {
                    result.add_error(
                        "temporal_scope",
                        "invalid_range",
                        "Start date must be before end date"
                    );
                }
            }
        }
        
        // Evidence reference validation
        if self.evidence_references.is_empty() {
            result.add_warning(
                "evidence_references",
                "no_evidence",
                "Identity should have at least one evidence reference"
            );
        }
        
        // State progression warnings
        if matches!(self.state, IdentityState::Working | IdentityState::Hypothesis) 
            && self.theory_refs.is_empty() {
            result.add_warning(
                "theory_refs",
                "no_theory",
                "Working/Hypothesis identities should be associated with theories"
            );
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_identity_persona_creation() {
        let researcher_id = EntityId::new();
        let identity = IdentityPersona::new(
            "John Smith", 
            IdentityType::Named, 
            researcher_id
        );
        
        assert_eq!(identity.state, IdentityState::Reference);
        assert_eq!(identity.primary_name, "John Smith");
        assert_eq!(identity.identity_type, IdentityType::Named);
        assert!(identity.validate().await.is_valid());
    }
    
    #[tokio::test]
    async fn test_state_transitions() {
        let researcher_id = EntityId::new();
        let mut identity = IdentityPersona::new(
            "Jane Doe", 
            IdentityType::Named, 
            researcher_id
        );
        
        // Valid transition: Reference -> Working
        assert!(identity.transition(
            IdentityState::Working,
            researcher_id,
            Some("Starting research".to_string())
        ).await.is_ok());
        
        assert_eq!(identity.state, IdentityState::Working);
        assert_eq!(identity.state_history.len(), 2);
        
        // Valid transition: Working -> Concluded
        assert!(identity.transition(
            IdentityState::Concluded,
            researcher_id,
            Some("Found sufficient evidence".to_string())
        ).await.is_ok());
        
        assert!(identity.is_concluded());
        assert!(identity.can_have_facts());
        assert!(identity.can_have_relationships());
    }
    
    #[tokio::test]
    async fn test_fact_reference_validation() {
        let researcher_id = EntityId::new();
        let mut identity = IdentityPersona::new(
            "Test Person", 
            IdentityType::Named, 
            researcher_id
        );
        
        let fact_id = EntityId::new();
        
        // Should fail - not concluded yet
        assert!(identity.add_fact_reference(fact_id).is_err());
        
        // Transition to concluded state
        identity.transition(
            IdentityState::Concluded,
            researcher_id,
            None
        ).await.unwrap();
        
        // Should succeed now
        assert!(identity.add_fact_reference(fact_id).is_ok());
        assert!(identity.fact_refs.contains(&fact_id));
    }
    
    #[tokio::test]
    async fn test_evidence_references() {
        let researcher_id = EntityId::new();
        let mut identity = IdentityPersona::new(
            "Evidence Test", 
            IdentityType::Named, 
            researcher_id
        );
        
        let evidence_id = EntityId::new();
        let evidence_ref = EvidenceReference::new(
            evidence_id,
            "John Smith, age 25",
            researcher_id
        );
        
        identity.add_evidence_reference(evidence_ref);
        assert_eq!(identity.evidence_references.len(), 1);
        assert_eq!(identity.evidence_references[0].evidence_id, evidence_id);
    }
    
    #[tokio::test]
    async fn test_nesting_capability() {
        let researcher_id = EntityId::new();
        let mut parent_identity = IdentityPersona::new(
            "Parent Identity", 
            IdentityType::Named, 
            researcher_id
        );
        
        let child_identity_id = EntityId::new();
        
        assert!(parent_identity.add_child(child_identity_id).await.is_ok());
        assert!(parent_identity.children().contains(&child_identity_id));
        
        assert!(parent_identity.remove_child(child_identity_id).await.unwrap());
        assert!(!parent_identity.children().contains(&child_identity_id));
    }
    
    #[tokio::test]
    async fn test_alternative_names() {
        let researcher_id = EntityId::new();
        let mut identity = IdentityPersona::new(
            "John Smith", 
            IdentityType::Named, 
            researcher_id
        );
        
        identity.add_alternative_name("Jon Smith");
        identity.add_alternative_name("J. Smith");
        identity.add_alternative_name("John Smith"); // Duplicate - should not add
        
        assert_eq!(identity.alternative_names.len(), 2);
        
        let all_names = identity.all_names();
        assert_eq!(all_names.len(), 3);
        assert!(all_names.contains(&"John Smith"));
        assert!(all_names.contains(&"Jon Smith"));
        assert!(all_names.contains(&"J. Smith"));
    }
    
    #[tokio::test]
    async fn test_display_names() {
        let researcher_id = EntityId::new();
        
        let named = IdentityPersona::new("John Smith", IdentityType::Named, researcher_id);
        assert_eq!(named.display_name(), "John Smith");
        
        let mut described = IdentityPersona::new("Unknown Father", IdentityType::Described, researcher_id);
        described.description = Some("Father of Mary Smith".to_string());
        assert_eq!(described.display_name(), "Father of Mary Smith");
        
        let relationship = IdentityPersona::new("Husband", IdentityType::Relationship, researcher_id);
        assert_eq!(relationship.display_name(), "Husband (relationship-based)");
        
        let anonymous = IdentityPersona::new("Person #1", IdentityType::Anonymous, researcher_id);
        assert_eq!(anonymous.display_name(), "Anonymous person (Person #1)");
        
        let hypothetical = IdentityPersona::new("Possible John", IdentityType::Hypothetical, researcher_id);
        assert_eq!(hypothetical.display_name(), "Possible John (hypothetical)");
    }
}