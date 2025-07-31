//! Citation entity - flexible relationships between entities and sources
//!
//! Citations connect entities to sources with precision, supporting everything from
//! quick citations to detailed forensic-level documentation with element-by-element analysis.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use std::collections::HashMap;

use crate::{
    define_states,
    entity::{EntityMetadata, NestableEntity},
    state::{StateMachine, StateTransition},
    validation::{Validatable, ValidationResult},
    EntityId, Error, Result, impl_entity,
};

// Define citation states
define_states! {
    /// States in the citation lifecycle
    pub enum CitationState {
        /// Quick citation for initial reference
        Quick,
        
        /// Full citation with complete details
        Full,
        
        /// Element-level citation for detailed analysis
        Element,
        
        /// Citation has been analyzed and verified
        Analyzed,
        
        /// Citation is incomplete or problematic
        Incomplete,
        
        /// Citation has been deprecated or superseded
        Deprecated,
    }
}

impl CitationState {
    /// Get valid transitions from this state
    pub fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Quick => vec![Self::Full, Self::Incomplete, Self::Deprecated],
            Self::Full => vec![Self::Element, Self::Analyzed, Self::Incomplete, Self::Deprecated],
            Self::Element => vec![Self::Analyzed, Self::Full, Self::Deprecated],
            Self::Analyzed => vec![Self::Element, Self::Deprecated],
            Self::Incomplete => vec![Self::Quick, Self::Full, Self::Deprecated],
            Self::Deprecated => vec![Self::Quick, Self::Full],
        }
    }
}

/// Type of entity being cited
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitingEntityType {
    Theory,
    Evidence,
    Analysis,
    IdentityPersona,
    Fact,
    Relationship,
    Other,
}

/// Citation purpose or reason
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CitationPurpose {
    /// Primary evidence for a claim
    Primary,
    /// Supporting evidence
    Supporting,
    /// Contradicting evidence
    Contradicting,
    /// Background context
    Context,
    /// Comparative evidence
    Comparative,
    /// Methodological reference
    Methodological,
    /// General reference
    Reference,
}

/// Quality of the citation itself
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CitationQuality {
    /// Complete and accurate citation
    Complete,
    /// Good but missing some details
    Good,
    /// Adequate for basic use
    Adequate,
    /// Incomplete but usable
    Incomplete,
    /// Poor or problematic citation
    Poor,
}

/// Element-level citation for detailed analysis
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CitedElement {
    /// Specific element being cited (field, line, etc.)
    #[validate(length(min = 1))]
    pub element_name: String,
    
    /// Location within source (page, line, field, etc.)
    pub location: String,
    
    /// Exact text being cited
    pub cited_text: String,
    
    /// Interpretation or extraction
    pub interpretation: Option<String>,
    
    /// Confidence in this citation
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: f32,
    
    /// Notes about this element
    pub notes: Option<String>,
}

/// Provenance tracking for citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationProvenance {
    /// How this citation was created
    pub creation_method: String, // "manual", "imported", "generated", "AI-assisted"
    
    /// Tool or system used
    pub creation_tool: Option<String>,
    
    /// Original citation text (if imported/converted)
    pub original_citation: Option<String>,
    
    /// Conversion notes
    pub conversion_notes: Option<String>,
    
    /// Quality of conversion/creation
    pub creation_quality: CitationQuality,
}

/// Citation analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationAnalysis {
    /// Analysis ID reference
    pub analysis_id: EntityId,
    
    /// Type of analysis performed
    pub analysis_type: String,
    
    /// Results summary
    pub results: String,
    
    /// Issues found
    pub issues: Vec<String>,
    
    /// Recommendations
    pub recommendations: Vec<String>,
    
    /// Overall assessment
    pub assessment: CitationQuality,
}

/// Flexible citation entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Citation {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Current state
    pub state: CitationState,
    
    /// State transition history
    pub state_history: Vec<StateTransition<CitationState>>,
    
    /// Entity being cited FROM
    pub from_entity: EntityId,
    pub from_entity_type: CitingEntityType,
    
    /// Source being cited TO
    pub to_source: EntityId,
    
    /// Purpose of this citation
    pub purpose: CitationPurpose,
    
    /// Quick citation text
    #[validate(length(min = 1))]
    pub quick_citation: String,
    
    /// Full formatted citation
    pub full_citation: Option<String>,
    
    /// Citation style used (Chicago, MLA, etc.)
    pub citation_style: Option<String>,
    
    /// Location within source
    pub location: String,
    
    /// Page numbers, if applicable
    pub page_numbers: Option<String>,
    
    /// URL if online source
    #[validate(url)]
    pub url: Option<String>,
    
    /// Date accessed (for online sources)
    pub date_accessed: Option<DateTime<Utc>>,
    
    /// Element-level citations for detailed analysis
    pub cited_elements: Vec<CitedElement>,
    
    /// Quality assessment
    pub quality: CitationQuality,
    
    /// Confidence in this citation
    #[validate(range(min = 0.0, max = 1.0))]
    pub confidence: f32,
    
    /// Issues with this citation
    pub issues: Vec<String>,
    
    /// Citation analysis results
    pub analyses: Vec<CitationAnalysis>,
    
    /// Provenance information
    pub provenance: CitationProvenance,
    
    /// Related citations
    pub related_citations: Vec<EntityId>,
    
    /// Superseded citations
    pub supersedes: Vec<EntityId>,
    
    /// Custom fields for specialized citation types
    pub custom_fields: HashMap<String, String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Notes about this citation
    pub notes: Option<String>,
    
    /// Nesting support (for grouped citations)
    pub parent_citation: Option<EntityId>,
    pub child_citations: Vec<EntityId>,
    
    /// Attribution
    pub created_by: EntityId,
    pub creation_date: DateTime<Utc>,
}

impl Citation {
    /// Create a new quick citation
    pub fn new_quick(
        from_entity: EntityId,
        from_entity_type: CitingEntityType,
        to_source: EntityId,
        quick_citation: impl Into<String>,
        created_by: EntityId,
    ) -> Self {
        let initial_state = CitationState::Quick;
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata::new(created_by),
            state: initial_state.clone(),
            state_history: vec![StateTransition {
                from: initial_state.clone(),
                to: initial_state,
                triggered_by: created_by,
                timestamp: now,
                reason: Some("Citation created".to_string()),
            }],
            from_entity,
            from_entity_type,
            to_source,
            purpose: CitationPurpose::Reference,
            quick_citation: quick_citation.into(),
            full_citation: None,
            citation_style: None,
            location: String::new(),
            page_numbers: None,
            url: None,
            date_accessed: None,
            cited_elements: Vec::new(),
            quality: CitationQuality::Adequate,
            confidence: 0.7,
            issues: Vec::new(),
            analyses: Vec::new(),
            provenance: CitationProvenance {
                creation_method: "manual".to_string(),
                creation_tool: None,
                original_citation: None,
                conversion_notes: None,
                creation_quality: CitationQuality::Adequate,
            },
            related_citations: Vec::new(),
            supersedes: Vec::new(),
            custom_fields: HashMap::new(),
            tags: Vec::new(),
            notes: None,
            parent_citation: None,
            child_citations: Vec::new(),
            created_by,
            creation_date: now,
        }
    }
    
    /// Create a full citation
    pub fn new_full(
        from_entity: EntityId,
        from_entity_type: CitingEntityType,
        to_source: EntityId,
        quick_citation: impl Into<String>,
        full_citation: impl Into<String>,
        citation_style: impl Into<String>,
        created_by: EntityId,
    ) -> Self {
        let mut citation = Self::new_quick(
            from_entity,
            from_entity_type,
            to_source,
            quick_citation,
            created_by,
        );
        
        citation.state = CitationState::Full;
        citation.full_citation = Some(full_citation.into());
        citation.citation_style = Some(citation_style.into());
        citation.quality = CitationQuality::Good;
        citation.confidence = 0.8;
        
        // Update state history
        citation.state_history.push(StateTransition {
            from: CitationState::Quick,
            to: CitationState::Full,
            triggered_by: created_by,
            timestamp: Utc::now(),
            reason: Some("Upgraded to full citation".to_string()),
        });
        
        citation
    }
    
    /// Add a cited element for element-level analysis
    pub fn add_cited_element(&mut self, element: CitedElement) {
        self.cited_elements.push(element);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Set location within source
    pub fn set_location(&mut self, location: impl Into<String>) {
        self.location = location.into();
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Set page numbers
    pub fn set_page_numbers(&mut self, pages: impl Into<String>) {
        self.page_numbers = Some(pages.into());
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Set URL and access date
    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = Some(url.into());
        self.date_accessed = Some(Utc::now());
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Add an issue with this citation
    pub fn add_issue(&mut self, issue: impl Into<String>) {
        self.issues.push(issue.into());
        // Degrade quality based on number of issues, but don't upgrade
        let new_quality = match self.issues.len() {
            1 => CitationQuality::Good,
            2 => CitationQuality::Adequate,
            3 => CitationQuality::Incomplete,
            _ => CitationQuality::Poor,
        };
        
        // Only degrade quality, never upgrade
        if new_quality as u8 > self.quality as u8 {
            self.quality = new_quality;
        }
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Add citation analysis
    pub fn add_analysis(&mut self, analysis: CitationAnalysis) {
        self.analyses.push(analysis);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Add a related citation
    pub fn add_related_citation(&mut self, citation_id: EntityId) {
        if !self.related_citations.contains(&citation_id) {
            self.related_citations.push(citation_id);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Mark as superseding another citation
    pub fn supersede(&mut self, citation_id: EntityId) {
        if !self.supersedes.contains(&citation_id) {
            self.supersedes.push(citation_id);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Set custom field
    pub fn set_custom_field(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.custom_fields.insert(key.into(), value.into());
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Get display citation (prefers full over quick)
    pub fn display_citation(&self) -> &str {
        self.full_citation.as_ref().unwrap_or(&self.quick_citation)
    }
    
    /// Check if this is an online citation
    pub fn is_online(&self) -> bool {
        self.url.is_some()
    }
    
    /// Check if citation has element-level detail
    pub fn has_element_detail(&self) -> bool {
        !self.cited_elements.is_empty()
    }
    
    /// Get overall quality score
    pub fn quality_score(&self) -> f32 {
        let base_score = match self.quality {
            CitationQuality::Complete => 1.0,
            CitationQuality::Good => 0.8,
            CitationQuality::Adequate => 0.6,
            CitationQuality::Incomplete => 0.4,
            CitationQuality::Poor => 0.2,
        };
        
        // Adjust for confidence
        base_score * self.confidence
    }
}

// Implement Entity trait
impl_entity!(Citation, "Citation");

// Implement StateMachine trait
#[async_trait]
impl StateMachine for Citation {
    type State = CitationState;
    
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

// Implement NestableEntity for grouped citations
#[async_trait]
impl NestableEntity for Citation {
    fn children(&self) -> Vec<EntityId> {
        self.child_citations.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "Citation"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if self.child_citations.contains(&child_id) {
            return Ok(());
        }
        
        self.child_citations.push(child_id);
        self.metadata.update(self.metadata.modified_by);
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_citations.len();
        self.child_citations.retain(|id| id != &child_id);
        if self.child_citations.len() < initial_len {
            self.metadata.update(self.metadata.modified_by);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// Implement Validatable
#[async_trait]
impl Validatable for Citation {
    async fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Basic validation using validator crate
        if let Err(e) = <Citation as validator::Validate>::validate(self) {
            result.merge(e.into());
        }
        
        // Custom validation
        if self.quick_citation.trim().is_empty() {
            result.add_error(
                "quick_citation",
                "empty",
                "Quick citation cannot be empty"
            );
        }
        
        // State-specific validation
        if matches!(self.state, CitationState::Full) && self.full_citation.is_none() {
            result.add_error(
                "full_citation",
                "missing",
                "Full citation state requires full_citation text"
            );
        }
        
        if matches!(self.state, CitationState::Element) && self.cited_elements.is_empty() {
            result.add_error(
                "cited_elements",
                "missing",
                "Element citation state requires cited elements"
            );
        }
        
        // URL validation for online sources
        if let Some(url) = &self.url {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                result.add_warning(
                    "url",
                    "missing_protocol",
                    "URL should include http:// or https:// protocol"
                );
            }
            
            if self.date_accessed.is_none() {
                result.add_warning(
                    "date_accessed",
                    "missing",
                    "Online sources should have access date"
                );
            }
        }
        
        // Quality vs confidence consistency check
        if self.quality == CitationQuality::Complete && self.confidence < 0.8 {
            result.add_warning(
                "confidence",
                "inconsistent",
                "Complete citations should have high confidence"
            );
        }
        
        // Element validation
        for (i, element) in self.cited_elements.iter().enumerate() {
            if element.element_name.trim().is_empty() {
                result.add_error(
                    &format!("cited_elements[{}].element_name", i),
                    "empty",
                    "Element name cannot be empty"
                );
            }
            
            if element.confidence < 0.3 {
                result.add_warning(
                    &format!("cited_elements[{}].confidence", i),
                    "low_confidence",
                    "Element has very low confidence"
                );
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_citation_creation() {
        let creator_id = EntityId::new();
        let entity_id = EntityId::new();
        let source_id = EntityId::new();
        
        let citation = Citation::new_quick(
            entity_id,
            CitingEntityType::Evidence,
            source_id,
            "Smith Family Bible, births recorded 1850-1880",
            creator_id,
        );
        
        assert_eq!(citation.state, CitationState::Quick);
        assert_eq!(citation.from_entity, entity_id);
        assert_eq!(citation.to_source, source_id);
        assert!(validator::Validate::validate(&citation).is_ok());
    }
    
    #[tokio::test]
    async fn test_full_citation() {
        let creator_id = EntityId::new();
        let entity_id = EntityId::new();
        let source_id = EntityId::new();
        
        let citation = Citation::new_full(
            entity_id,
            CitingEntityType::Theory,
            source_id,
            "1920 Census",
            "U.S. Bureau of the Census, 1920 United States Federal Census, Washington D.C.: National Archives and Records Administration, 1920.",
            "Chicago",
            creator_id,
        );
        
        assert_eq!(citation.state, CitationState::Full);
        assert!(citation.full_citation.is_some());
        assert_eq!(citation.citation_style, Some("Chicago".to_string()));
        assert_eq!(citation.state_history.len(), 2); // Quick -> Full
    }
    
    #[tokio::test]
    async fn test_state_transitions() {
        let creator_id = EntityId::new();
        let entity_id = EntityId::new();
        let source_id = EntityId::new();
        
        let mut citation = Citation::new_quick(
            entity_id,
            CitingEntityType::Evidence,
            source_id,
            "Test Citation",
            creator_id,
        );
        
        // Valid transition: Quick -> Full
        assert!(citation.transition(
            CitationState::Full,
            creator_id,
            Some("Adding full details".to_string())
        ).await.is_ok());
        
        assert_eq!(citation.state, CitationState::Full);
        
        // Valid transition: Full -> Element
        assert!(citation.transition(
            CitationState::Element,
            creator_id,
            Some("Adding element detail".to_string())
        ).await.is_ok());
        
        // Invalid transition: Element -> Quick
        assert!(citation.transition(
            CitationState::Quick,
            creator_id,
            None
        ).await.is_err());
    }
    
    #[tokio::test]
    async fn test_element_level_citation() {
        let creator_id = EntityId::new();
        let entity_id = EntityId::new();
        let source_id = EntityId::new();
        
        let mut citation = Citation::new_quick(
            entity_id,
            CitingEntityType::Analysis,
            source_id,
            "Birth Certificate",
            creator_id,
        );
        
        let element = CitedElement {
            element_name: "Child's Name".to_string(),
            location: "Line 1, Field 2".to_string(),
            cited_text: "John Smith".to_string(),
            interpretation: Some("Given name and surname clearly recorded".to_string()),
            confidence: 0.95,
            notes: Some("Handwriting is clear and legible".to_string()),
        };
        
        citation.add_cited_element(element);
        
        assert_eq!(citation.cited_elements.len(), 1);
        assert!(citation.has_element_detail());
        assert_eq!(citation.cited_elements[0].element_name, "Child's Name");
    }
    
    #[tokio::test]
    async fn test_online_citation() {
        let creator_id = EntityId::new();
        let entity_id = EntityId::new();
        let source_id = EntityId::new();
        
        let mut citation = Citation::new_quick(
            entity_id,
            CitingEntityType::Evidence,
            source_id,
            "FamilySearch Record",
            creator_id,
        );
        
        citation.set_url("https://familysearch.org/ark:/61903/1:1:ABCD-1234");
        
        assert!(citation.is_online());
        assert!(citation.url.is_some());
        assert!(citation.date_accessed.is_some());
    }
    
    #[tokio::test]
    async fn test_citation_quality() {
        let creator_id = EntityId::new();
        let entity_id = EntityId::new();
        let source_id = EntityId::new();
        
        let mut citation = Citation::new_quick(
            entity_id,
            CitingEntityType::Evidence,
            source_id,
            "Test Citation",
            creator_id,
        );
        
        // Start with adequate quality
        assert_eq!(citation.quality, CitationQuality::Adequate);
        assert!(citation.quality_score() > 0.0);
        
        // Since citation starts at Adequate, adding issues won't degrade it to Good
        // Let's test the actual behavior
        
        // First issue - quality should remain Adequate
        citation.add_issue("Missing page number");
        assert_eq!(citation.quality, CitationQuality::Adequate);
        
        // More issues - quality should degrade further
        citation.add_issue("Unclear location");
        citation.add_issue("Partial citation");
        assert_eq!(citation.quality, CitationQuality::Incomplete);
        
        citation.add_issue("Major problems");
        assert_eq!(citation.quality, CitationQuality::Poor);
    }
    
    #[tokio::test]
    async fn test_citation_nesting() {
        let creator_id = EntityId::new();
        let entity_id = EntityId::new();
        let source_id = EntityId::new();
        
        let mut parent_citation = Citation::new_quick(
            entity_id,
            CitingEntityType::Theory,
            source_id,
            "Family Bible Collection",
            creator_id,
        );
        
        let child_citation_id = EntityId::new();
        
        assert!(parent_citation.add_child(child_citation_id).await.is_ok());
        assert!(parent_citation.children().contains(&child_citation_id));
        
        assert!(parent_citation.remove_child(child_citation_id).await.unwrap());
        assert!(!parent_citation.children().contains(&child_citation_id));
    }
    
    #[tokio::test]
    async fn test_citation_relationships() {
        let creator_id = EntityId::new();
        let entity_id = EntityId::new();
        let source_id = EntityId::new();
        
        let mut citation = Citation::new_quick(
            entity_id,
            CitingEntityType::Evidence,
            source_id,
            "Primary Citation",
            creator_id,
        );
        
        let related_id = EntityId::new();
        let superseded_id = EntityId::new();
        
        citation.add_related_citation(related_id);
        citation.supersede(superseded_id);
        
        assert!(citation.related_citations.contains(&related_id));
        assert!(citation.supersedes.contains(&superseded_id));
        
        // Test duplicate prevention
        citation.add_related_citation(related_id);
        assert_eq!(citation.related_citations.len(), 1);
    }
}