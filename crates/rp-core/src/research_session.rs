//! ResearchSession entity - Captures research work sessions
//! Part of Layer 2: Research Process Model

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::{EntityMetadata, NestableEntity},
    state::{State, StateMachine, StateTransition},
    EntityId, Error, Result, impl_entity, impl_validatable,
};

/// Type of research session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    /// Visit to a physical repository
    RepositoryVisit,
    /// Online search session
    OnlineSearch,
    /// Analysis work session
    AnalysisSession,
    /// Document review session
    DocumentReview,
    /// Correlation work session
    CorrelationWork,
    /// Planning session
    Planning,
    /// General research session
    General,
}

/// State of a research session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    /// Session is currently active
    Active,
    /// Session has been paused
    Paused,
    /// Session completed normally
    Completed,
    /// Session was abandoned
    Abandoned,
}

impl State for SessionState {
    fn name(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Paused => "Paused",
            Self::Completed => "Completed",
            Self::Abandoned => "Abandoned",
        }
    }
    
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Abandoned)
    }
}

/// Capture source configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SessionCaptureSource {
    /// Type of capture
    pub source_type: String,
    
    /// Capture configuration
    pub config: serde_json::Value,
    
    /// Whether actively capturing
    pub is_active: bool,
    
    /// When capture started
    pub started_at: DateTime<Utc>,
}

/// Research session entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ResearchSession {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Current session state
    pub state: SessionState,
    
    /// State transition history
    pub state_history: Vec<StateTransition<SessionState>>,
    
    /// Type of session
    pub session_type: SessionType,
    
    /// When session started
    pub start_time: DateTime<Utc>,
    
    /// When session ended
    pub end_time: Option<DateTime<Utc>>,
    
    /// Physical or virtual location
    pub location: Option<String>,
    
    /// Repository being visited
    pub repository: Option<String>,
    
    /// Purpose of the session
    #[validate(length(min = 1))]
    pub purpose: String,
    
    /// Capture sources active during session
    #[validate(nested)]
    pub capture_sources: Vec<SessionCaptureSource>,
    
    /// Activities performed (ResearchActivity IDs)
    pub activities: Vec<EntityId>,
    
    /// Evidence created during session
    pub evidence_created: Vec<EntityId>,
    
    /// Identities created or modified
    pub identities_created: Vec<EntityId>,
    
    /// Work products created
    pub work_products_created: Vec<EntityId>,
    
    /// Parent theory context
    pub theory_id: Option<EntityId>,
    
    /// Primary researcher
    pub researcher_id: EntityId,
    
    /// Additional researchers present
    pub additional_researchers: Vec<EntityId>,
    
    /// Parent session (for nested sessions)
    pub parent_session: Option<EntityId>,
    
    /// Child sessions
    pub child_sessions: Vec<EntityId>,
    
    /// Session notes
    pub notes: Option<String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Custom fields
    pub custom_fields: serde_json::Value,
    
    /// Total duration (calculated)
    pub total_duration: Option<Duration>,
    
    /// Statistics
    pub statistics: serde_json::Value,
}

impl ResearchSession {
    /// Create a new research session
    pub fn new(
        session_type: SessionType,
        purpose: impl Into<String>,
        researcher_id: EntityId,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata {
                id: EntityId::new(),
                created_by: researcher_id,
                created_at: now,
                modified_by: researcher_id,
                modified_at: now,
                is_active: true,
                version: 1,
                parent_version: None,
            },
            state: SessionState::Active,
            state_history: vec![],
            session_type,
            start_time: now,
            end_time: None,
            location: None,
            repository: None,
            purpose: purpose.into(),
            capture_sources: vec![],
            activities: vec![],
            evidence_created: vec![],
            identities_created: vec![],
            work_products_created: vec![],
            theory_id: None,
            researcher_id,
            additional_researchers: vec![],
            parent_session: None,
            child_sessions: vec![],
            notes: None,
            tags: vec![],
            custom_fields: serde_json::Value::Object(serde_json::Map::new()),
            total_duration: None,
            statistics: serde_json::json!({
                "activities_count": 0,
                "evidence_count": 0,
                "identities_count": 0,
                "work_products_count": 0,
            }),
        }
    }
    
    /// Create a repository visit session
    pub fn repository_visit(
        repository: impl Into<String>,
        purpose: impl Into<String>,
        researcher_id: EntityId,
    ) -> Self {
        let mut session = Self::new(SessionType::RepositoryVisit, purpose, researcher_id);
        session.repository = Some(repository.into());
        session
    }
    
    /// Add an activity to the session
    pub fn add_activity(&mut self, activity_id: EntityId) {
        if !self.activities.contains(&activity_id) {
            self.activities.push(activity_id);
            self.update_statistics();
            self.metadata.update(self.researcher_id);
        }
    }
    
    /// Record evidence created
    pub fn add_evidence_created(&mut self, evidence_id: EntityId) {
        if !self.evidence_created.contains(&evidence_id) {
            self.evidence_created.push(evidence_id);
            self.update_statistics();
            self.metadata.update(self.researcher_id);
        }
    }
    
    /// Record identity created
    pub fn add_identity_created(&mut self, identity_id: EntityId) {
        if !self.identities_created.contains(&identity_id) {
            self.identities_created.push(identity_id);
            self.update_statistics();
            self.metadata.update(self.researcher_id);
        }
    }
    
    /// Record work product created
    pub fn add_work_product_created(&mut self, product_id: EntityId) {
        if !self.work_products_created.contains(&product_id) {
            self.work_products_created.push(product_id);
            self.update_statistics();
            self.metadata.update(self.researcher_id);
        }
    }
    
    /// Add additional researcher
    pub fn add_researcher(&mut self, researcher_id: EntityId) {
        if researcher_id != self.researcher_id && !self.additional_researchers.contains(&researcher_id) {
            self.additional_researchers.push(researcher_id);
            self.update_statistics();
            self.metadata.update(self.researcher_id);
        }
    }
    
    /// Add capture source
    pub fn add_capture_source(&mut self, source: SessionCaptureSource) {
        self.capture_sources.push(source);
        self.update_statistics();
        self.metadata.update(self.researcher_id);
    }
    
    /// Pause the session
    pub async fn pause(&mut self, paused_by: EntityId) -> Result<()> {
        self.transition(SessionState::Paused, paused_by, Some("Pausing session".to_string())).await
    }
    
    /// Resume a paused session
    pub async fn resume(&mut self, resumed_by: EntityId) -> Result<()> {
        self.transition(SessionState::Active, resumed_by, Some("Resuming session".to_string())).await
    }
    
    /// Complete the session
    pub async fn complete(&mut self, completed_by: EntityId) -> Result<()> {
        self.end_time = Some(Utc::now());
        self.calculate_duration();
        self.transition(SessionState::Completed, completed_by, Some("Session completed".to_string())).await
    }
    
    /// Abandon the session
    pub async fn abandon(&mut self, abandoned_by: EntityId, reason: Option<String>) -> Result<()> {
        self.end_time = Some(Utc::now());
        self.calculate_duration();
        self.transition(SessionState::Abandoned, abandoned_by, reason).await
    }
    
    /// Calculate session duration
    fn calculate_duration(&mut self) {
        if let Some(end) = self.end_time {
            self.total_duration = Some(end - self.start_time);
        }
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics = serde_json::json!({
            "activities_count": self.activities.len(),
            "evidence_count": self.evidence_created.len(),
            "identities_count": self.identities_created.len(),
            "work_products_count": self.work_products_created.len(),
            "capture_sources_active": self.capture_sources.iter().filter(|s| s.is_active).count(),
            "total_researchers": self.additional_researchers.len() + 1,
        });
    }
    
    /// Check if session is active
    pub fn is_active(&self) -> bool {
        matches!(self.state, SessionState::Active)
    }
    
    /// Check if session can be resumed
    pub fn can_resume(&self) -> bool {
        matches!(self.state, SessionState::Paused)
    }
    
    /// Get session duration
    pub fn duration(&self) -> Duration {
        if let Some(duration) = self.total_duration {
            duration
        } else if self.is_active() {
            // Calculate current duration for active session
            Utc::now() - self.start_time
        } else {
            Duration::zero()
        }
    }
}

impl_entity!(ResearchSession, "ResearchSession");
impl_validatable!(ResearchSession);

// Implement StateMachine trait
#[async_trait]
impl StateMachine for ResearchSession {
    type State = SessionState;
    
    fn current_state(&self) -> &Self::State {
        &self.state
    }
    
    fn state_history(&self) -> Vec<StateTransition<Self::State>> {
        self.state_history.clone()
    }
    
    fn can_transition(&self, to: &Self::State) -> bool {
        match (&self.state, to) {
            (SessionState::Active, SessionState::Paused) => true,
            (SessionState::Active, SessionState::Completed) => true,
            (SessionState::Active, SessionState::Abandoned) => true,
            (SessionState::Paused, SessionState::Active) => true,
            (SessionState::Paused, SessionState::Completed) => true,
            (SessionState::Paused, SessionState::Abandoned) => true,
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
            SessionState::Active => vec![
                SessionState::Paused,
                SessionState::Completed,
                SessionState::Abandoned,
            ],
            SessionState::Paused => vec![
                SessionState::Active,
                SessionState::Completed,
                SessionState::Abandoned,
            ],
            SessionState::Completed | SessionState::Abandoned => vec![],
        }
    }
}

#[async_trait]
impl NestableEntity for ResearchSession {
    fn children(&self) -> Vec<EntityId> {
        self.child_sessions.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        // Sessions can contain other sessions
        entity_type == "ResearchSession"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if !self.child_sessions.contains(&child_id) {
            self.child_sessions.push(child_id);
            self.metadata.update(self.metadata.modified_by);
        }
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_sessions.len();
        self.child_sessions.retain(|id| id != &child_id);
        if self.child_sessions.len() < initial_len {
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
    fn test_session_creation() {
        let researcher_id = EntityId::new();
        
        let session = ResearchSession::new(
            SessionType::OnlineSearch,
            "Search for Smith family in 1850 census",
            researcher_id,
        );
        
        assert_eq!(session.session_type, SessionType::OnlineSearch);
        assert_eq!(session.state, SessionState::Active);
        assert_eq!(session.researcher_id, researcher_id);
        assert!(session.is_active());
    }
    
    #[test]
    fn test_repository_visit() {
        let researcher_id = EntityId::new();
        
        let session = ResearchSession::repository_visit(
            "National Archives",
            "Research Civil War pension records",
            researcher_id,
        );
        
        assert_eq!(session.session_type, SessionType::RepositoryVisit);
        assert_eq!(session.repository, Some("National Archives".to_string()));
    }
    
    #[test]
    fn test_add_activities() {
        let researcher_id = EntityId::new();
        let activity1 = EntityId::new();
        let activity2 = EntityId::new();
        
        let mut session = ResearchSession::new(
            SessionType::AnalysisSession,
            "Analyze conflicting evidence",
            researcher_id,
        );
        
        session.add_activity(activity1);
        session.add_activity(activity2);
        session.add_activity(activity1); // Duplicate
        
        assert_eq!(session.activities.len(), 2);
        assert_eq!(session.statistics["activities_count"], 2);
    }
    
    #[test]
    fn test_add_created_entities() {
        let researcher_id = EntityId::new();
        let evidence_id = EntityId::new();
        let identity_id = EntityId::new();
        let product_id = EntityId::new();
        
        let mut session = ResearchSession::new(
            SessionType::DocumentReview,
            "Review parish records",
            researcher_id,
        );
        
        session.add_evidence_created(evidence_id);
        session.add_identity_created(identity_id);
        session.add_work_product_created(product_id);
        
        assert_eq!(session.evidence_created.len(), 1);
        assert_eq!(session.identities_created.len(), 1);
        assert_eq!(session.work_products_created.len(), 1);
        assert_eq!(session.statistics["evidence_count"], 1);
        assert_eq!(session.statistics["identities_count"], 1);
        assert_eq!(session.statistics["work_products_count"], 1);
    }
    
    #[test]
    fn test_additional_researchers() {
        let researcher1 = EntityId::new();
        let researcher2 = EntityId::new();
        let researcher3 = EntityId::new();
        
        let mut session = ResearchSession::new(
            SessionType::CorrelationWork,
            "Correlate census records",
            researcher1,
        );
        
        session.add_researcher(researcher2);
        session.add_researcher(researcher3);
        session.add_researcher(researcher1); // Primary researcher
        session.add_researcher(researcher2); // Duplicate
        
        assert_eq!(session.additional_researchers.len(), 2);
        assert_eq!(session.statistics["total_researchers"], 3);
    }
    
    #[tokio::test]
    async fn test_session_lifecycle() {
        let researcher_id = EntityId::new();
        
        let mut session = ResearchSession::new(
            SessionType::General,
            "General research",
            researcher_id,
        );
        
        // Pause session
        assert!(session.pause(researcher_id).await.is_ok());
        assert_eq!(session.state, SessionState::Paused);
        assert!(session.can_resume());
        
        // Resume session
        assert!(session.resume(researcher_id).await.is_ok());
        assert_eq!(session.state, SessionState::Active);
        
        // Complete session
        assert!(session.complete(researcher_id).await.is_ok());
        assert_eq!(session.state, SessionState::Completed);
        assert!(session.end_time.is_some());
        assert!(session.total_duration.is_some());
        
        // Cannot transition from completed
        assert!(session.pause(researcher_id).await.is_err());
    }
    
    #[tokio::test]
    async fn test_abandon_session() {
        let researcher_id = EntityId::new();
        
        let mut session = ResearchSession::new(
            SessionType::OnlineSearch,
            "Search interrupted",
            researcher_id,
        );
        
        assert!(session.abandon(
            researcher_id,
            Some("Internet connection lost".to_string())
        ).await.is_ok());
        
        assert_eq!(session.state, SessionState::Abandoned);
        assert!(session.end_time.is_some());
        assert_eq!(session.state_history.len(), 1);
        assert_eq!(
            session.state_history[0].reason,
            Some("Internet connection lost".to_string())
        );
    }
    
    #[test]
    fn test_capture_sources() {
        let researcher_id = EntityId::new();
        
        let mut session = ResearchSession::new(
            SessionType::OnlineSearch,
            "Online research",
            researcher_id,
        );
        
        let source = SessionCaptureSource {
            source_type: "BrowserExtension".to_string(),
            config: serde_json::json!({
                "capture_screenshots": true,
                "interval": 30,
            }),
            is_active: true,
            started_at: Utc::now(),
        };
        
        session.add_capture_source(source);
        
        assert_eq!(session.capture_sources.len(), 1);
        assert_eq!(session.statistics["capture_sources_active"], 1);
    }
    
    #[test]
    fn test_session_duration() {
        let researcher_id = EntityId::new();
        
        let mut session = ResearchSession::new(
            SessionType::Planning,
            "Plan research strategy",
            researcher_id,
        );
        
        // Active session duration
        let duration = session.duration();
        assert!(duration >= Duration::zero());
        
        // Set specific duration
        session.end_time = Some(session.start_time + Duration::hours(2));
        session.calculate_duration();
        
        assert_eq!(session.total_duration, Some(Duration::hours(2)));
        assert_eq!(session.duration(), Duration::hours(2));
    }
    
    #[tokio::test]
    async fn test_session_nesting() {
        let researcher_id = EntityId::new();
        let child_session_id = EntityId::new();
        
        let mut parent = ResearchSession::new(
            SessionType::RepositoryVisit,
            "Full day at archives",
            researcher_id,
        );
        
        assert!(parent.can_contain("ResearchSession"));
        
        assert!(parent.add_child(child_session_id).await.is_ok());
        assert_eq!(parent.children().len(), 1);
        
        assert!(parent.remove_child(child_session_id).await.unwrap());
        assert_eq!(parent.children().len(), 0);
    }
}