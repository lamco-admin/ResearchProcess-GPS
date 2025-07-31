//! Fact entity - unified model for events, attributes, and characteristics

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::{EntityMetadata, NestableEntity},
    state::{State, StateMachine, StateTransition},
    EntityId, Error, Result, impl_entity, impl_validatable,
};

/// Types of facts that can be recorded
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactType {
    /// Life events
    Birth,
    Death,
    Marriage,
    Divorce,
    Adoption,
    Baptism,
    Burial,
    Census,
    Immigration,
    Naturalization,
    
    /// Attributes
    Name,
    Gender,
    Age,
    Occupation,
    Education,
    Religion,
    Nationality,
    PhysicalDescription,
    
    /// Characteristics
    DNA,
    MedicalCondition,
    PersonalityTrait,
    Skill,
    Achievement,
    
    /// Relationships (basic facts, not full Relationship entities)
    ParentChild,
    Spouse,
    Sibling,
    Association,
    
    /// Property and possessions
    Residence,
    LandOwnership,
    PersonalProperty,
    
    /// Military
    MilitaryService,
    MilitaryRank,
    MilitaryUnit,
    
    /// Custom fact type
    Custom(String),
}

/// State of a fact in the research process
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactState {
    /// Extracted from evidence but not yet analyzed
    Extracted,
    /// Under analysis and verification
    Analyzing,
    /// Disputed or conflicting evidence
    Disputed,
    /// Verified through multiple sources
    Verified,
    /// Accepted as proven
    Proven,
    /// Disproven or rejected
    Disproven,
}

impl State for FactState {
    fn name(&self) -> &'static str {
        match self {
            Self::Extracted => "Extracted",
            Self::Analyzing => "Analyzing",
            Self::Disputed => "Disputed",
            Self::Verified => "Verified",
            Self::Proven => "Proven",
            Self::Disproven => "Disproven",
        }
    }
    
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Proven | Self::Disproven)
    }
}

/// Date precision for historical facts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatePrecision {
    /// Exact date known
    Exact(DateTime<Utc>),
    /// Year and month known
    YearMonth { year: i32, month: u32 },
    /// Only year known
    Year(i32),
    /// Approximate year
    AboutYear(i32),
    /// Date range
    Between { start: DateTime<Utc>, end: DateTime<Utc> },
    /// Before a certain date
    Before(DateTime<Utc>),
    /// After a certain date
    After(DateTime<Utc>),
    /// Unknown date
    Unknown,
}

/// Location reference for facts
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LocationReference {
    /// Reference to a Location entity
    pub location_id: Option<EntityId>,
    
    /// Descriptive place name as found in source
    #[validate(length(min = 1))]
    pub place_name: String,
    
    /// Modern standardized place name
    pub standardized_name: Option<String>,
    
    /// GPS coordinates if known
    pub coordinates: Option<(f64, f64)>,
    
    /// Confidence in location identification
    pub confidence: String,
}

/// Value associated with a fact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactValue {
    /// Text value (names, descriptions, etc.)
    Text(String),
    /// Numeric value (age, count, etc.)
    Number(f64),
    /// Boolean value (yes/no attributes)
    Boolean(bool),
    /// Date/time value
    Date(DatePrecision),
    /// Location value
    Location(LocationReference),
    /// Reference to another entity
    EntityReference(EntityId),
    /// Complex structured data
    Structured(serde_json::Value),
    /// No specific value (event occurrence)
    None,
}

/// Evidence supporting or contradicting a fact
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FactEvidence {
    /// Reference to Evidence entity
    pub evidence_id: EntityId,
    
    /// Specific citation within the evidence
    pub citation_detail: Option<String>,
    
    /// How this evidence relates to the fact
    pub relationship: EvidenceRelationship,
    
    /// Extracted text or data
    pub extracted_text: Option<String>,
    
    /// Quality assessment
    pub quality: EvidenceQuality,
    
    /// Analysis notes
    pub notes: Option<String>,
}

/// How evidence relates to a fact
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceRelationship {
    /// Directly states the fact
    Supports,
    /// Contradicts the fact
    Contradicts,
    /// Provides context
    Contextual,
    /// Suggests but doesn't prove
    Inferential,
}

/// Quality of evidence for a fact
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceQuality {
    /// Primary source, direct evidence
    Primary,
    /// Secondary source, indirect evidence
    Secondary,
    /// Tertiary or compiled source
    Tertiary,
    /// Questionable reliability
    Questionable,
}

/// A unified fact entity representing events, attributes, or characteristics
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Fact {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Current state of the fact
    pub state: FactState,
    
    /// State transition history
    pub state_history: Vec<StateTransition<FactState>>,
    
    /// Type of fact
    pub fact_type: FactType,
    
    /// Primary subject of the fact (usually an IdentityPersona)
    pub subject_id: EntityId,
    
    /// Additional participants (for events like marriage)
    pub participant_ids: Vec<EntityId>,
    
    /// The fact value
    pub value: FactValue,
    
    /// When the fact occurred or was true
    pub date: Option<DatePrecision>,
    
    /// Where the fact occurred or was relevant
    pub location: Option<LocationReference>,
    
    /// Supporting evidence
    #[validate(nested)]
    pub evidence: Vec<FactEvidence>,
    
    /// Confidence assessment
    pub confidence_id: Option<EntityId>,
    
    /// Related analysis
    pub analysis_ids: Vec<EntityId>,
    
    /// Source of the fact assertion
    pub asserted_by: EntityId,
    
    /// Language of original record
    pub language: Option<String>,
    
    /// Cultural context
    pub cultural_context: Option<String>,
    
    /// Research notes
    pub notes: Option<String>,
    
    /// Sort order for chronological display
    pub sort_date: Option<DateTime<Utc>>,
    
    /// Parent fact (for nested facts)
    pub parent_fact: Option<EntityId>,
    
    /// Child facts
    pub child_facts: Vec<EntityId>,
    
    /// Custom attributes
    pub custom_fields: serde_json::Value,
    
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Fact {
    /// Create a new fact
    pub fn new(
        fact_type: FactType,
        subject_id: EntityId,
        value: FactValue,
        asserted_by: EntityId,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata {
                id: EntityId::new(),
                created_by: asserted_by,
                created_at: now,
                modified_by: asserted_by,
                modified_at: now,
                is_active: true,
                version: 1,
                parent_version: None,
            },
            state: FactState::Extracted,
            state_history: vec![],
            fact_type,
            subject_id,
            participant_ids: vec![],
            value,
            date: None,
            location: None,
            evidence: vec![],
            confidence_id: None,
            analysis_ids: vec![],
            asserted_by,
            language: None,
            cultural_context: None,
            notes: None,
            sort_date: None,
            parent_fact: None,
            child_facts: vec![],
            custom_fields: serde_json::Value::Object(serde_json::Map::new()),
            tags: vec![],
        }
    }
    
    /// Create a simple text fact
    pub fn text(
        fact_type: FactType,
        subject_id: EntityId,
        text: impl Into<String>,
        asserted_by: EntityId,
    ) -> Self {
        Self::new(fact_type, subject_id, FactValue::Text(text.into()), asserted_by)
    }
    
    /// Create an event fact
    pub fn event(
        fact_type: FactType,
        subject_id: EntityId,
        date: DatePrecision,
        location: Option<LocationReference>,
        asserted_by: EntityId,
    ) -> Self {
        let mut fact = Self::new(fact_type, subject_id, FactValue::None, asserted_by);
        fact.date = Some(date.clone());
        fact.location = location;
        
        // Set sort date for chronological ordering
        fact.sort_date = match &date {
            DatePrecision::Exact(dt) => Some(*dt),
            DatePrecision::YearMonth { year, month } => {
                DateTime::parse_from_rfc3339(&format!("{}-{:02}-01T00:00:00Z", year, month))
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }
            DatePrecision::Year(year) | DatePrecision::AboutYear(year) => {
                DateTime::parse_from_rfc3339(&format!("{}-01-01T00:00:00Z", year))
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            }
            DatePrecision::Between { start, .. } => Some(*start),
            DatePrecision::After(dt) | DatePrecision::Before(dt) => Some(*dt),
            DatePrecision::Unknown => None,
        };
        
        fact
    }
    
    /// Add supporting evidence
    pub fn add_evidence(&mut self, evidence: FactEvidence) {
        self.evidence.push(evidence);
        self.metadata.update(self.asserted_by);
    }
    
    /// Add a participant to an event
    pub fn add_participant(&mut self, participant_id: EntityId) {
        if !self.participant_ids.contains(&participant_id) {
            self.participant_ids.push(participant_id);
            self.metadata.update(self.asserted_by);
        }
    }
    
    /// Check if fact is in a verified state
    pub fn is_verified(&self) -> bool {
        matches!(self.state, FactState::Verified | FactState::Proven)
    }
    
    /// Check if fact is disputed
    pub fn is_disputed(&self) -> bool {
        matches!(self.state, FactState::Disputed)
    }
    
    /// Get display date for the fact
    pub fn display_date(&self) -> String {
        match &self.date {
            Some(DatePrecision::Exact(dt)) => dt.format("%d %b %Y").to_string(),
            Some(DatePrecision::YearMonth { year, month }) => {
                format!("{} {}", 
                    match month {
                        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
                        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
                        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
                        _ => "???",
                    },
                    year
                )
            }
            Some(DatePrecision::Year(year)) => year.to_string(),
            Some(DatePrecision::AboutYear(year)) => format!("about {}", year),
            Some(DatePrecision::Between { start, end }) => {
                format!("between {} and {}", 
                    start.format("%d %b %Y"),
                    end.format("%d %b %Y")
                )
            }
            Some(DatePrecision::Before(dt)) => format!("before {}", dt.format("%d %b %Y")),
            Some(DatePrecision::After(dt)) => format!("after {}", dt.format("%d %b %Y")),
            Some(DatePrecision::Unknown) | None => "date unknown".to_string(),
        }
    }
}

impl_entity!(Fact, "Fact");
impl_validatable!(Fact);

// Implement StateMachine trait
#[async_trait]
impl StateMachine for Fact {
    type State = FactState;
    
    fn current_state(&self) -> &Self::State {
        &self.state
    }
    
    fn state_history(&self) -> Vec<StateTransition<Self::State>> {
        self.state_history.clone()
    }
    
    fn can_transition(&self, to: &Self::State) -> bool {
        match (&self.state, to) {
            (FactState::Extracted, FactState::Analyzing) => true,
            (FactState::Extracted, FactState::Disputed) => true,
            (FactState::Analyzing, FactState::Disputed) => true,
            (FactState::Analyzing, FactState::Verified) => true,
            (FactState::Analyzing, FactState::Disproven) => true,
            (FactState::Disputed, FactState::Analyzing) => true,
            (FactState::Disputed, FactState::Verified) => true,
            (FactState::Disputed, FactState::Disproven) => true,
            (FactState::Verified, FactState::Proven) => true,
            (FactState::Verified, FactState::Disputed) => true,
            (FactState::Verified, FactState::Disproven) => true,
            (FactState::Proven, FactState::Disputed) => true,
            (FactState::Disproven, FactState::Disputed) => true,
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
            FactState::Extracted => vec![FactState::Analyzing, FactState::Disputed],
            FactState::Analyzing => vec![FactState::Disputed, FactState::Verified, FactState::Disproven],
            FactState::Disputed => vec![FactState::Analyzing, FactState::Verified, FactState::Disproven],
            FactState::Verified => vec![FactState::Proven, FactState::Disputed, FactState::Disproven],
            FactState::Proven => vec![FactState::Disputed],
            FactState::Disproven => vec![FactState::Disputed],
        }
    }
}

#[async_trait]
impl NestableEntity for Fact {
    fn children(&self) -> Vec<EntityId> {
        self.child_facts.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        // Facts can contain other facts (sub-facts)
        entity_type == "Fact"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if !self.child_facts.contains(&child_id) {
            self.child_facts.push(child_id);
            self.metadata.update(self.metadata.modified_by);
        }
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_facts.len();
        self.child_facts.retain(|id| id != &child_id);
        if self.child_facts.len() < initial_len {
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
    fn test_fact_creation() {
        let subject_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let fact = Fact::text(
            FactType::Name,
            subject_id,
            "John Smith",
            researcher_id,
        );
        
        assert_eq!(fact.fact_type, FactType::Name);
        assert_eq!(fact.subject_id, subject_id);
        assert!(matches!(fact.value, FactValue::Text(ref s) if s == "John Smith"));
        assert_eq!(fact.state, FactState::Extracted);
    }
    
    #[test]
    fn test_event_fact() {
        let subject_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let date = DatePrecision::Exact(Utc::now());
        let location = LocationReference {
            location_id: None,
            place_name: "Boston, Massachusetts".to_string(),
            standardized_name: Some("Boston, Suffolk, Massachusetts, USA".to_string()),
            coordinates: Some((42.3601, -71.0589)),
            confidence: "High".to_string(),
        };
        
        let fact = Fact::event(
            FactType::Birth,
            subject_id,
            date.clone(),
            Some(location),
            researcher_id,
        );
        
        assert_eq!(fact.fact_type, FactType::Birth);
        assert!(matches!(fact.value, FactValue::None));
        assert!(fact.date.is_some());
        assert!(fact.location.is_some());
        assert!(fact.sort_date.is_some());
    }
    
    #[test]
    fn test_fact_evidence() {
        let subject_id = EntityId::new();
        let researcher_id = EntityId::new();
        let evidence_id = EntityId::new();
        
        let mut fact = Fact::text(
            FactType::Occupation,
            subject_id,
            "Carpenter",
            researcher_id,
        );
        
        let evidence = FactEvidence {
            evidence_id,
            citation_detail: Some("Page 42, line 15".to_string()),
            relationship: EvidenceRelationship::Supports,
            extracted_text: Some("Occupation: Carpenter".to_string()),
            quality: EvidenceQuality::Primary,
            notes: None,
        };
        
        fact.add_evidence(evidence);
        assert_eq!(fact.evidence.len(), 1);
        assert_eq!(fact.evidence[0].evidence_id, evidence_id);
    }
    
    #[tokio::test]
    async fn test_fact_state_transitions() {
        let subject_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut fact = Fact::text(
            FactType::Name,
            subject_id,
            "John Smith",
            researcher_id,
        );
        
        // Valid transition
        assert!(fact.transition(
            FactState::Analyzing,
            researcher_id,
            Some("Beginning analysis".to_string())
        ).await.is_ok());
        
        assert_eq!(fact.state, FactState::Analyzing);
        assert_eq!(fact.state_history.len(), 1);
        
        // Invalid transition
        assert!(fact.transition(
            FactState::Proven,
            researcher_id,
            None
        ).await.is_err());
    }
    
    #[test]
    fn test_date_precision_display() {
        let subject_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        // Test exact date
        let exact_date = DateTime::parse_from_rfc3339("2023-07-15T10:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let fact1 = Fact::event(
            FactType::Birth,
            subject_id,
            DatePrecision::Exact(exact_date),
            None,
            researcher_id,
        );
        assert_eq!(fact1.display_date(), "15 Jul 2023");
        
        // Test year only
        let fact2 = Fact::event(
            FactType::Death,
            subject_id,
            DatePrecision::Year(1850),
            None,
            researcher_id,
        );
        assert_eq!(fact2.display_date(), "1850");
        
        // Test about year
        let fact3 = Fact::event(
            FactType::Marriage,
            subject_id,
            DatePrecision::AboutYear(1875),
            None,
            researcher_id,
        );
        assert_eq!(fact3.display_date(), "about 1875");
    }
    
    #[test]
    fn test_fact_validation() {
        let subject_id = EntityId::new();
        let researcher_id = EntityId::new();
        
        let mut fact = Fact::text(
            FactType::Name,
            subject_id,
            "John Smith",
            researcher_id,
        );
        
        // Add invalid evidence (empty place name)
        let invalid_evidence = FactEvidence {
            evidence_id: EntityId::new(),
            citation_detail: None,
            relationship: EvidenceRelationship::Supports,
            extracted_text: None,
            quality: EvidenceQuality::Primary,
            notes: None,
        };
        
        fact.add_evidence(invalid_evidence);
        
        // Validation should pass as FactEvidence doesn't have required fields
        let result = fact.validate();
        assert!(result.is_ok());
    }
}