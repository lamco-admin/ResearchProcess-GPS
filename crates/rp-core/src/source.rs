//! Source entity - hierarchical source management system
//!
//! Sources form a flexible hierarchy: ITEM → SERIES → COLLECTION → REPOSITORY → SYSTEM
//! This supports everything from individual documents to entire archive systems.

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

// Define source states
define_states! {
    /// States in the source lifecycle
    pub enum SourceState {
        /// Initial reference - basic source information
        Draft,
        
        /// Active source being used in research
        Active,
        
        /// Source has been verified and quality assessed
        Verified,
        
        /// Source has been formally reviewed and approved
        Approved,
        
        /// Source is deprecated or superseded
        Deprecated,
        
        /// Source has been archived but preserved
        Archived,
    }
}

impl SourceState {
    /// Get valid transitions from this state
    pub fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Draft => vec![Self::Active, Self::Deprecated],
            Self::Active => vec![
                Self::Verified, 
                Self::Deprecated, 
                Self::Archived
            ],
            Self::Verified => vec![
                Self::Approved, 
                Self::Active, 
                Self::Deprecated
            ],
            Self::Approved => vec![
                Self::Deprecated, 
                Self::Archived
            ],
            Self::Deprecated => vec![Self::Active, Self::Archived],
            Self::Archived => vec![Self::Active],
        }
    }
}

/// Type of source in the hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    /// Individual document or record
    Item,
    /// Related group of items
    Series,
    /// Collection of series
    Collection,
    /// Physical or institutional repository
    Repository,
    /// Entire system (website, database, etc.)
    System,
}

/// GPS classification of source quality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceClass {
    /// Original document or artifact
    Original,
    /// Derivative work based on originals
    Derivative,
    /// Authored narrative or compiled work
    Authored,
    /// Unknown classification
    Unknown,
}

/// Information quality classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InformationClass {
    /// First-hand information
    Primary,
    /// Second-hand information
    Secondary,
    /// Unknown origin
    Indeterminate,
}

/// Multi-dimensional source quality assessment
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SourceQuality {
    /// GPS source classification
    pub source_class: SourceClass,
    
    /// GPS information classification
    pub information_class: InformationClass,
    
    /// Overall quality score (0.0 to 1.0)
    #[validate(range(min = 0.0, max = 1.0))]
    pub quality_score: f32,
    
    /// Specific quality dimensions
    pub dimensions: QualityDimensions,
    
    /// Known issues or limitations
    pub issues: Vec<String>,
    
    /// Quality assessment notes
    pub notes: Option<String>,
    
    /// Who assessed the quality
    pub assessed_by: EntityId,
    
    /// When the assessment was made
    pub assessed_at: DateTime<Utc>,
}

/// Detailed quality dimensions
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct QualityDimensions {
    /// How complete is the source
    #[validate(range(min = 0.0, max = 1.0))]
    pub completeness: f32,
    
    /// How legible/readable is the source
    #[validate(range(min = 0.0, max = 1.0))]
    pub legibility: f32,
    
    /// How accurate is the source likely to be
    #[validate(range(min = 0.0, max = 1.0))]
    pub accuracy: f32,
    
    /// How reliable is the source
    #[validate(range(min = 0.0, max = 1.0))]
    pub reliability: f32,
    
    /// Temporal relevance (how close to events)
    #[validate(range(min = 0.0, max = 1.0))]
    pub temporal_relevance: f32,
    
    /// Geographic relevance
    #[validate(range(min = 0.0, max = 1.0))]
    pub geographic_relevance: f32,
}

impl Default for QualityDimensions {
    fn default() -> Self {
        Self {
            completeness: 0.5,
            legibility: 0.5,
            accuracy: 0.5,
            reliability: 0.5,
            temporal_relevance: 0.5,
            geographic_relevance: 0.5,
        }
    }
}

impl Default for SourceQuality {
    fn default() -> Self {
        Self {
            source_class: SourceClass::Unknown,
            information_class: InformationClass::Indeterminate,
            quality_score: 0.5,
            dimensions: QualityDimensions::default(),
            issues: Vec::new(),
            notes: None,
            assessed_by: EntityId::new(), // Will be updated when used
            assessed_at: Utc::now(),
        }
    }
}

/// Template for generating citations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceTemplate {
    /// Template identifier
    pub id: String,
    
    /// Citation style (Chicago, MLA, etc.)
    pub style: String,
    
    /// Template version
    pub version: String,
    
    /// Citation format template
    pub format_template: String,
    
    /// Required fields for this template
    pub required_fields: Vec<String>,
    
    /// Optional fields
    pub optional_fields: Vec<String>,
    
    /// Field validation rules
    pub field_rules: HashMap<String, String>,
}

/// Physical or digital location information
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LocationInfo {
    /// Physical address or digital URL
    pub address: String,
    
    /// Call number or identifier
    pub call_number: Option<String>,
    
    /// Specific location within repository
    pub location_details: Option<String>,
    
    /// Access conditions
    pub access_conditions: Option<String>,
    
    /// Contact information
    pub contact_info: Option<String>,
    
    /// Operating hours or availability
    pub availability: Option<String>,
}

/// Provenance tracking for sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    /// Previous owner or custodian
    pub previous_custodian: String,
    
    /// Date of transfer
    pub transfer_date: Option<DateTime<Utc>>,
    
    /// How the transfer occurred
    pub transfer_method: Option<String>,
    
    /// Documentation of the transfer
    pub documentation: Option<String>,
    
    /// Chain of custody notes
    pub notes: Option<String>,
}

/// Hierarchical source entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Source {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Current state
    pub state: SourceState,
    
    /// State transition history
    pub state_history: Vec<StateTransition<SourceState>>,
    
    /// Type in the hierarchy
    pub source_type: SourceType,
    
    /// Title or name of the source
    #[validate(length(min = 1, max = 1000))]
    pub title: String,
    
    /// Alternative titles or names
    pub alternative_titles: Vec<String>,
    
    /// Detailed description
    pub description: Option<String>,
    
    /// Creator or author
    pub creator: Option<String>,
    
    /// Publication information
    pub publication_info: Option<String>,
    
    /// Date range this source covers
    pub date_range: Option<DateRange>,
    
    /// Creation date of the source itself
    pub creation_date: Option<DateTime<Utc>>,
    
    /// Geographic coverage
    pub geographic_coverage: Vec<String>,
    
    /// Subject matter covered
    pub subjects: Vec<String>,
    
    /// Languages used in the source
    pub languages: Vec<String>,
    
    /// Quality assessment
    pub quality: SourceQuality,
    
    /// Physical or digital location
    pub location: Option<LocationInfo>,
    
    /// Provenance chain
    pub provenance: Vec<ProvenanceRecord>,
    
    /// Citation template and fields
    pub citation_template: Option<SourceTemplate>,
    pub template_fields: HashMap<String, String>,
    
    /// Format information
    pub format: Option<String>,
    pub extent: Option<String>, // Size, page count, etc.
    
    /// Access information
    pub access_restrictions: Option<String>,
    pub copyright_status: Option<String>,
    pub usage_rights: Option<String>,
    
    /// Related sources
    pub related_sources: Vec<EntityId>,
    pub superseded_sources: Vec<EntityId>,
    
    /// Analysis references
    pub quality_analyses: Vec<EntityId>,
    
    /// Nesting support for hierarchy
    pub parent_source: Option<EntityId>,
    pub child_sources: Vec<EntityId>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Research notes
    pub notes: Option<String>,
    
    /// Attribution
    pub catalogued_by: EntityId,
    pub catalogued_at: DateTime<Utc>,
}

/// Date range for source coverage
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct DateRange {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_approximate: bool,
    pub description: Option<String>,
}

impl Source {
    /// Create a new source
    pub fn new(
        title: impl Into<String>,
        source_type: SourceType,
        catalogued_by: EntityId,
    ) -> Self {
        let initial_state = SourceState::Draft;
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata::new(catalogued_by),
            state: initial_state.clone(),
            state_history: vec![StateTransition {
                from: initial_state.clone(),
                to: initial_state,
                triggered_by: catalogued_by,
                timestamp: now,
                reason: Some("Source created".to_string()),
            }],
            source_type,
            title: title.into(),
            alternative_titles: Vec::new(),
            description: None,
            creator: None,
            publication_info: None,
            date_range: None,
            creation_date: None,
            geographic_coverage: Vec::new(),
            subjects: Vec::new(),
            languages: vec!["English".to_string()], // Default language
            quality: SourceQuality::default(),
            location: None,
            provenance: Vec::new(),
            citation_template: None,
            template_fields: HashMap::new(),
            format: None,
            extent: None,
            access_restrictions: None,
            copyright_status: None,
            usage_rights: None,
            related_sources: Vec::new(),
            superseded_sources: Vec::new(),
            quality_analyses: Vec::new(),
            parent_source: None,
            child_sources: Vec::new(),
            tags: Vec::new(),
            notes: None,
            catalogued_by,
            catalogued_at: now,
        }
    }
    
    /// Add an alternative title
    pub fn add_alternative_title(&mut self, title: impl Into<String>) {
        let title = title.into();
        if !self.alternative_titles.contains(&title) {
            self.alternative_titles.push(title);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Add a subject
    pub fn add_subject(&mut self, subject: impl Into<String>) {
        let subject = subject.into();
        if !self.subjects.contains(&subject) {
            self.subjects.push(subject);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Add geographic coverage
    pub fn add_geographic_coverage(&mut self, location: impl Into<String>) {
        let location = location.into();
        if !self.geographic_coverage.contains(&location) {
            self.geographic_coverage.push(location);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Update quality assessment
    pub fn update_quality(&mut self, quality: SourceQuality) {
        self.quality = quality;
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Add a related source
    pub fn add_related_source(&mut self, source_id: EntityId) {
        if !self.related_sources.contains(&source_id) {
            self.related_sources.push(source_id);
            self.metadata.update(self.metadata.modified_by);
        }
    }
    
    /// Add a provenance record
    pub fn add_provenance(&mut self, provenance: ProvenanceRecord) {
        self.provenance.push(provenance);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Set citation template
    pub fn set_citation_template(&mut self, template: SourceTemplate, fields: HashMap<String, String>) {
        self.citation_template = Some(template);
        self.template_fields = fields;
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Generate citation using template
    pub fn generate_citation(&self) -> Option<String> {
        let template = self.citation_template.as_ref()?;
        let mut citation = template.format_template.clone();
        
        // Simple template substitution
        for (field, value) in &self.template_fields {
            let placeholder = format!("{{{}}}", field);
            citation = citation.replace(&placeholder, value);
        }
        
        Some(citation)
    }
    
    /// Check if this source can contain child sources
    pub fn can_contain_children(&self) -> bool {
        match self.source_type {
            SourceType::Item => false,
            _ => true,
        }
    }
    
    /// Get hierarchy level
    pub fn hierarchy_level(&self) -> u8 {
        match self.source_type {
            SourceType::System => 0,
            SourceType::Repository => 1,
            SourceType::Collection => 2,
            SourceType::Series => 3,
            SourceType::Item => 4,
        }
    }
    
    /// Get display name for UI
    pub fn display_name(&self) -> String {
        if self.alternative_titles.is_empty() {
            self.title.clone()
        } else {
            format!("{} ({})", self.title, self.alternative_titles.join(", "))
        }
    }
}

// Implement Entity trait
impl_entity!(Source, "Source");

// Implement StateMachine trait
#[async_trait]
impl StateMachine for Source {
    type State = SourceState;
    
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

// Implement NestableEntity for hierarchical sources
#[async_trait]
impl NestableEntity for Source {
    fn children(&self) -> Vec<EntityId> {
        self.child_sources.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "Source" && self.can_contain_children()
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if !self.can_contain_children() {
            return Err(Error::InvalidStateTransition(
                format!("{:?} sources cannot contain children", self.source_type)
            ));
        }
        
        if self.child_sources.contains(&child_id) {
            return Ok(());
        }
        
        self.child_sources.push(child_id);
        self.metadata.update(self.metadata.modified_by);
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_sources.len();
        self.child_sources.retain(|id| id != &child_id);
        if self.child_sources.len() < initial_len {
            self.metadata.update(self.metadata.modified_by);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// Implement Validatable
#[async_trait]
impl Validatable for Source {
    async fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Basic validation using validator crate
        if let Err(e) = <Source as validator::Validate>::validate(self) {
            result.merge(e.into());
        }
        
        // Custom validation
        if self.title.trim().is_empty() {
            result.add_error(
                "title",
                "empty",
                "Source title cannot be empty"
            );
        }
        
        // Hierarchy validation
        if self.source_type == SourceType::Item && !self.child_sources.is_empty() {
            result.add_error(
                "child_sources",
                "invalid_hierarchy",
                "Item sources cannot have children"
            );
        }
        
        // Date range validation
        if let Some(date_range) = &self.date_range {
            if let (Some(start), Some(end)) = (date_range.start_date, date_range.end_date) {
                if start > end {
                    result.add_error(
                        "date_range",
                        "invalid_range",
                        "Start date must be before end date"
                    );
                }
            }
        }
        
        // Citation template validation
        if let Some(template) = &self.citation_template {
            for required_field in &template.required_fields {
                if !self.template_fields.contains_key(required_field) {
                    result.add_warning(
                        "template_fields",
                        "missing_required_field",
                        &format!("Missing required citation field: {}", required_field)
                    );
                }
            }
        }
        
        // Quality assessment validation
        if self.quality.quality_score == 0.5 && 
           self.quality.source_class == SourceClass::Unknown {
            result.add_info(
                "quality",
                "not_assessed",
                "Source quality has not been assessed"
            );
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_source_creation() {
        let cataloguer_id = EntityId::new();
        let source = Source::new(
            "1920 US Census", 
            SourceType::Collection, 
            cataloguer_id
        );
        
        assert_eq!(source.title, "1920 US Census");
        assert_eq!(source.source_type, SourceType::Collection);
        assert_eq!(source.state, SourceState::Draft);
        assert!(validator::Validate::validate(&source).is_ok());
    }
    
    #[tokio::test]
    async fn test_source_hierarchy() {
        let cataloguer_id = EntityId::new();
        let mut collection = Source::new(
            "US Census Records", 
            SourceType::Collection, 
            cataloguer_id
        );
        
        let series_id = EntityId::new();
        
        // Collection can contain series
        assert!(collection.can_contain_children());
        assert!(collection.add_child(series_id).await.is_ok());
        assert!(collection.children().contains(&series_id));
        
        // Item cannot contain children
        let item = Source::new(
            "John Smith Census Entry", 
            SourceType::Item, 
            cataloguer_id
        );
        assert!(!item.can_contain_children());
    }
    
    #[tokio::test]
    async fn test_state_transitions() {
        let cataloguer_id = EntityId::new();
        let mut source = Source::new(
            "Test Source", 
            SourceType::Item, 
            cataloguer_id
        );
        
        // Valid transition: Draft -> Active
        assert!(source.transition(
            SourceState::Active,
            cataloguer_id,
            Some("Putting source into use".to_string())
        ).await.is_ok());
        
        assert_eq!(source.state, SourceState::Active);
        assert_eq!(source.state_history.len(), 2);
        
        // Valid transition: Active -> Verified
        assert!(source.transition(
            SourceState::Verified,
            cataloguer_id,
            Some("Quality verified".to_string())
        ).await.is_ok());
        
        // Invalid transition: Verified -> Draft
        assert!(source.transition(
            SourceState::Draft,
            cataloguer_id,
            None
        ).await.is_err());
    }
    
    #[tokio::test]
    async fn test_citation_generation() {
        let cataloguer_id = EntityId::new();
        let mut source = Source::new(
            "Test Source", 
            SourceType::Item, 
            cataloguer_id
        );
        
        let template = SourceTemplate {
            id: "basic_citation".to_string(),
            style: "Chicago".to_string(),
            version: "1.0".to_string(),
            format_template: "{author}, \"{title},\" {publication}, {date}.".to_string(),
            required_fields: vec!["author".to_string(), "title".to_string()],
            optional_fields: vec!["publication".to_string(), "date".to_string()],
            field_rules: HashMap::new(),
        };
        
        let mut fields = HashMap::new();
        fields.insert("author".to_string(), "John Doe".to_string());
        fields.insert("title".to_string(), "Sample Document".to_string());
        fields.insert("publication".to_string(), "Test Journal".to_string());
        fields.insert("date".to_string(), "2023".to_string());
        
        source.set_citation_template(template, fields);
        
        let citation = source.generate_citation().unwrap();
        assert_eq!(citation, "John Doe, \"Sample Document,\" Test Journal, 2023.");
    }
    
    #[tokio::test]
    async fn test_quality_assessment() {
        let cataloguer_id = EntityId::new();
        let mut source = Source::new(
            "High Quality Source", 
            SourceType::Item, 
            cataloguer_id
        );
        
        let quality = SourceQuality {
            source_class: SourceClass::Original,
            information_class: InformationClass::Primary,
            quality_score: 0.9,
            dimensions: QualityDimensions {
                completeness: 0.95,
                legibility: 0.9,
                accuracy: 0.9,
                reliability: 0.85,
                temporal_relevance: 1.0,
                geographic_relevance: 0.8,
            },
            issues: vec!["Slight water damage in corner".to_string()],
            notes: Some("Excellent primary source".to_string()),
            assessed_by: cataloguer_id,
            assessed_at: Utc::now(),
        };
        
        source.update_quality(quality);
        
        assert_eq!(source.quality.source_class, SourceClass::Original);
        assert_eq!(source.quality.information_class, InformationClass::Primary);
        assert_eq!(source.quality.quality_score, 0.9);
        assert_eq!(source.quality.issues.len(), 1);
    }
    
    #[tokio::test]
    async fn test_hierarchy_levels() {
        let cataloguer_id = EntityId::new();
        
        let system = Source::new("FamilySearch", SourceType::System, cataloguer_id);
        let repo = Source::new("National Archives", SourceType::Repository, cataloguer_id);
        let collection = Source::new("Census Records", SourceType::Collection, cataloguer_id);
        let series = Source::new("1920 Census", SourceType::Series, cataloguer_id);
        let item = Source::new("John Smith Entry", SourceType::Item, cataloguer_id);
        
        assert_eq!(system.hierarchy_level(), 0);
        assert_eq!(repo.hierarchy_level(), 1);
        assert_eq!(collection.hierarchy_level(), 2);
        assert_eq!(series.hierarchy_level(), 3);
        assert_eq!(item.hierarchy_level(), 4);
    }
}