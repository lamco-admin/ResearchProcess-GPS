//! Theory entity - represents research questions and hypotheses

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

// Define theory states using our macro
define_states! {
    /// States in the theory lifecycle
    pub enum TheoryState {
        /// Initial state - theory just created
        Draft,
        
        /// Theory is being actively researched
        Active,
        
        /// Research is paused/on hold
        OnHold,
        
        /// Theory has been proven with evidence
        Proven,
        
        /// Theory has been disproven with evidence
        Disproven,
        
        /// Theory cannot be resolved with available evidence
        Unresolvable,
        
        /// Theory was abandoned (no longer relevant)
        Abandoned,
    }
}

impl TheoryState {
    /// Check if this is a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Proven | Self::Disproven | Self::Unresolvable | Self::Abandoned
        )
    }
    
    /// Get valid transitions from this state
    pub fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Draft => vec![Self::Active, Self::Abandoned],
            Self::Active => vec![
                Self::OnHold,
                Self::Proven,
                Self::Disproven,
                Self::Unresolvable,
                Self::Abandoned,
            ],
            Self::OnHold => vec![Self::Active, Self::Abandoned],
            Self::Proven | Self::Disproven | Self::Unresolvable | Self::Abandoned => vec![],
        }
    }
}

/// A theory represents a research question or hypothesis
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Theory {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// The research question or hypothesis
    #[validate(length(min = 10, max = 1000))]
    pub question: String,
    
    /// Detailed description and context
    pub description: Option<String>,
    
    /// Current state of the theory
    pub state: TheoryState,
    
    /// State transition history
    pub state_history: Vec<StateTransition<TheoryState>>,
    
    /// Parent theory ID (if this is a sub-theory)
    pub parent_theory: Option<EntityId>,
    
    /// Child theory IDs
    pub child_theories: Vec<EntityId>,
    
    /// Associated evidence IDs
    pub evidence: Vec<EntityId>,
    
    /// Associated analysis IDs
    pub analyses: Vec<EntityId>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Priority level (1-5, 1 being highest)
    #[validate(range(min = 1, max = 5))]
    pub priority: u8,
    
    /// Geographic scope (if applicable)
    pub geographic_scope: Option<GeographicScope>,
    
    /// Temporal scope (if applicable)
    pub temporal_scope: Option<TemporalScope>,
    
    /// Research log entries
    pub research_log: Vec<ResearchLogEntry>,
    
    /// Conclusion (when in a terminal state)
    pub conclusion: Option<Conclusion>,
}

/// Geographic scope of a theory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicScope {
    pub description: String,
    pub places: Vec<String>,
    pub coordinates: Option<(f64, f64)>, // (latitude, longitude)
}

/// Temporal scope of a theory
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TemporalScope {
    #[validate(length(min = 1))]
    pub description: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub circa: bool, // Approximate dates
}

/// Entry in the research log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchLogEntry {
    pub id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub researcher_id: EntityId,
    pub action: String,
    pub notes: Option<String>,
    pub duration_minutes: Option<u32>,
}

/// Conclusion when theory reaches terminal state
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Conclusion {
    pub summary: String,
    pub supporting_evidence: Vec<EntityId>,
    pub confidence_level: ConfidenceLevel,
    pub notes: Option<String>,
}

/// Confidence level in a conclusion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
    Certain,
}

impl Theory {
    /// Create a new theory
    pub fn new(question: impl Into<String>, created_by: EntityId) -> Self {
        let initial_state = TheoryState::Draft;
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata::new(created_by),
            question: question.into(),
            description: None,
            state: initial_state.clone(),
            state_history: vec![StateTransition {
                from: initial_state.clone(),
                to: initial_state,
                triggered_by: created_by,
                timestamp: now,
                reason: Some("Theory created".to_string()),
            }],
            parent_theory: None,
            child_theories: Vec::new(),
            evidence: Vec::new(),
            analyses: Vec::new(),
            tags: Vec::new(),
            priority: 3, // Medium priority by default
            geographic_scope: None,
            temporal_scope: None,
            research_log: Vec::new(),
            conclusion: None,
        }
    }
    
    /// Add a research log entry
    pub fn add_log_entry(
        &mut self,
        researcher_id: EntityId,
        action: impl Into<String>,
        notes: Option<String>,
        duration_minutes: Option<u32>,
    ) {
        self.research_log.push(ResearchLogEntry {
            id: EntityId::new(),
            timestamp: Utc::now(),
            researcher_id,
            action: action.into(),
            notes,
            duration_minutes,
        });
        self.metadata.update(researcher_id);
    }
    
    /// Add evidence to this theory
    pub fn add_evidence(&mut self, evidence_id: EntityId) {
        if !self.evidence.contains(&evidence_id) {
            self.evidence.push(evidence_id);
        }
    }
    
    /// Add an analysis to this theory
    pub fn add_analysis(&mut self, analysis_id: EntityId) {
        if !self.analyses.contains(&analysis_id) {
            self.analyses.push(analysis_id);
        }
    }
    
    /// Set conclusion when moving to terminal state
    pub fn set_conclusion(&mut self, conclusion: Conclusion) -> Result<()> {
        if !self.state.is_terminal() {
            return Err(Error::InvalidStateTransition(
                "Cannot set conclusion on non-terminal state".to_string()
            ));
        }
        self.conclusion = Some(conclusion);
        Ok(())
    }
}

// Implement Entity trait
impl_entity!(Theory, "Theory");

// Implement StateMachine trait
#[async_trait]
impl StateMachine for Theory {
    type State = TheoryState;
    
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

// Implement NestableEntity for hierarchical theories
#[async_trait]
impl NestableEntity for Theory {
    fn children(&self) -> Vec<EntityId> {
        self.child_theories.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "Theory"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if self.child_theories.contains(&child_id) {
            return Ok(());
        }
        self.child_theories.push(child_id);
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_theories.len();
        self.child_theories.retain(|id| id != &child_id);
        Ok(self.child_theories.len() < initial_len)
    }
}

// Implement Validatable
#[async_trait]
impl Validatable for Theory {
    async fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Basic validation using validator crate
        if let Err(e) = <Theory as validator::Validate>::validate(self) {
            result.merge(e.into());
        }
        
        // Custom validation
        if self.question.trim().ends_with('?') {
            result.add_info(
                "question",
                "format",
                "Research questions typically end with a question mark"
            );
        }
        
        if self.state.is_terminal() && self.conclusion.is_none() {
            result.add_warning(
                "conclusion",
                "missing",
                "Terminal state theories should have a conclusion"
            );
        }
        
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
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_theory_creation() {
        let researcher_id = EntityId::new();
        let theory = Theory::new("Who were the parents of John Doe?", researcher_id);
        
        assert_eq!(theory.state, TheoryState::Draft);
        assert_eq!(theory.question, "Who were the parents of John Doe?");
        assert_eq!(theory.priority, 3);
        assert!(validator::Validate::validate(&theory).is_ok());
    }
    
    #[tokio::test]
    async fn test_theory_state_transitions() {
        let researcher_id = EntityId::new();
        let mut theory = Theory::new("Test theory", researcher_id);
        
        // Valid transition: Draft -> Active
        assert!(theory.transition(
            TheoryState::Active,
            researcher_id,
            Some("Starting research".to_string())
        ).await.is_ok());
        
        assert_eq!(theory.state, TheoryState::Active);
        assert_eq!(theory.state_history.len(), 2);
        
        // Invalid transition: Active -> Draft
        assert!(theory.transition(
            TheoryState::Draft,
            researcher_id,
            None
        ).await.is_err());
        
        // Valid transition: Active -> Proven
        assert!(theory.transition(
            TheoryState::Proven,
            researcher_id,
            Some("Found conclusive evidence".to_string())
        ).await.is_ok());
        
        // No transitions from terminal state
        assert!(theory.valid_transitions().is_empty());
    }
}