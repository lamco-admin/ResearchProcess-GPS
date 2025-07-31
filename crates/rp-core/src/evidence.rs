//! Evidence entity - represents sources, documents, and extracted information

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::{EntityMetadata, NestableEntity},
    EntityId, Result, impl_entity, impl_validatable,
};

/// GPS classification of source types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    /// Original document or artifact
    Original,
    /// Derivative work based on original
    Derivative,
    /// Authored narrative or compiled work
    Authored,
    /// Unknown source type
    Unknown,
}

/// Common genealogical record types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordType {
    BirthCertificate,
    DeathCertificate,
    MarriageLicense,
    Census,
    ChurchRecord,
    MilitaryRecord,
    LandRecord,
    Probate,
    Newspaper,
    Correspondence,
    Photograph,
    DnaResults,
    OralHistory,
    Other,
}

/// Types of facts that can be extracted from evidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactType {
    Name,
    Date,
    Place,
    Age,
    Occupation,
    Relationship,
    Event,
    PhysicalDescription,
    Property,
    MilitaryService,
    Education,
    Religion,
    Nationality,
    Residence,
    Burial,
    DnaMatch,
    Other,
}

/// GPS classification of information type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InformationType {
    /// First-hand information
    Primary,
    /// Second-hand information
    Secondary,
    /// Unknown origin
    Indeterminate,
}

/// GPS classification of evidence type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    /// Directly answers research question
    Direct,
    /// Requires inference to answer question
    Indirect,
    /// Absence of expected evidence
    Negative,
}

/// Quality assessment of evidence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceQuality {
    /// Original document
    Original,
    /// High-quality reproduction
    HighQualityCopy,
    /// Standard reproduction
    StandardCopy,
    /// Poor quality or derivative
    Derivative,
    /// Transcript or abstract only
    TranscriptOnly,
}

/// Full citation information for evidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Citation {
    pub citation_id: EntityId,
    
    #[validate(length(min = 1))]
    pub full_citation: String,
    pub short_citation: String,
    
    // Structured elements
    pub author: Option<String>,
    pub title: Option<String>,
    pub publication_info: Option<String>,
    pub date_accessed: Option<DateTime<Utc>>,
    #[validate(url)]
    pub url: Option<String>,
    pub page_numbers: Option<String>,
    
    // Repository information
    pub repository_id: Option<EntityId>,
    pub call_number: Option<String>,
    pub microfilm_number: Option<String>,
    pub digital_ark: Option<String>,
    
    // Quality indicators
    pub is_complete: bool,
    pub citation_standard: String, // Chicago, MLA, etc.
}

impl Citation {
    pub fn new(full_citation: impl Into<String>) -> Self {
        Self {
            citation_id: EntityId::new(),
            full_citation: full_citation.into(),
            short_citation: String::new(),
            author: None,
            title: None,
            publication_info: None,
            date_accessed: None,
            url: None,
            page_numbers: None,
            repository_id: None,
            call_number: None,
            microfilm_number: None,
            digital_ark: None,
            is_complete: false,
            citation_standard: "Chicago".to_string(),
        }
    }
}

/// A fact extracted from evidence
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ExtractedFact {
    pub fact_id: EntityId,
    pub fact_type: FactType,
    
    // What was found
    #[validate(length(min = 1))]
    pub extracted_text: String, // Exact text from source
    pub normalized_value: String, // Standardized version
    pub interpretation: String, // What we think it means
    
    // Context
    pub location_in_source: String, // "Page 3, line 15"
    pub surrounding_context: String, // Text around the fact
    
    // What it applies to
    pub applies_to_entities: Vec<EntityId>,
    pub applies_to_theories: Vec<EntityId>,
    
    // Confidence in extraction
    #[validate(range(min = 0.0, max = 1.0))]
    pub extraction_confidence: f32,
    pub extraction_method: String, // "manual", "OCR", "AI"
    pub extracted_by: EntityId,
    pub extracted_date: DateTime<Utc>,
}

impl ExtractedFact {
    pub fn new(
        fact_type: FactType,
        extracted_text: impl Into<String>,
        extracted_by: EntityId,
    ) -> Self {
        let text = extracted_text.into();
        Self {
            fact_id: EntityId::new(),
            fact_type,
            extracted_text: text.clone(),
            normalized_value: text.clone(),
            interpretation: text,
            location_in_source: String::new(),
            surrounding_context: String::new(),
            applies_to_entities: Vec::new(),
            applies_to_theories: Vec::new(),
            extraction_confidence: 0.8,
            extraction_method: "manual".to_string(),
            extracted_by,
            extracted_date: Utc::now(),
        }
    }
}

/// GPS classification of evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceClassification {
    pub source_type: SourceType,
    pub information_type: InformationType,
    pub evidence_type: EvidenceType,
    pub record_type: RecordType,
    pub original_purpose: String,
    pub known_errors: Vec<String>,
    pub reliability_issues: Vec<String>,
}

impl Default for EvidenceClassification {
    fn default() -> Self {
        Self {
            source_type: SourceType::Unknown,
            information_type: InformationType::Indeterminate,
            evidence_type: EvidenceType::Indirect,
            record_type: RecordType::Other,
            original_purpose: String::new(),
            known_errors: Vec::new(),
            reliability_issues: Vec::new(),
        }
    }
}

/// Evidence is a first-class entity in ResearchProcess-GPS
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Evidence {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Classification
    pub classification: EvidenceClassification,
    
    /// Citation
    pub citation: Option<Citation>,
    
    /// Content
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    pub description: String,
    pub transcription: Option<String>,
    pub abstract_text: Option<String>,
    
    /// Language and translation
    pub original_language: String,
    pub translations: Vec<Translation>,
    
    /// Digital representation
    pub files: Vec<FileReference>,
    pub ocr_text: Option<String>,
    
    /// Provenance
    pub creator: String,
    pub creation_date: Option<DateTime<Utc>>,
    pub creation_purpose: String,
    pub chain_of_custody: Vec<CustodyEntry>,
    
    /// Repository information
    pub repository_id: Option<EntityId>,
    pub acquisition_info: AcquisitionInfo,
    
    /// Extracted facts
    pub extracted_facts: Vec<ExtractedFact>,
    
    /// Research process
    pub found_date: DateTime<Utc>,
    pub found_by: EntityId,
    pub search_context: String,
    pub is_negative_evidence: bool,
    
    /// Quality assessment
    pub quality: EvidenceQuality,
    #[validate(range(min = 0.0, max = 1.0))]
    pub legibility: f32,
    #[validate(range(min = 0.0, max = 1.0))]
    pub completeness: f32,
    
    /// Theory relationships
    pub applicable_theories: Vec<EntityId>,
    pub theory_interpretations: Vec<TheoryInterpretation>,
    
    /// Nesting support (for document bundles, etc.)
    pub parent_evidence: Option<EntityId>,
    pub child_evidence: Vec<EntityId>,
}

/// Translation of evidence content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub language: String,
    pub translated_text: String,
    pub translator: String,
    pub translation_date: DateTime<Utc>,
    pub translation_notes: Option<String>,
}

/// Reference to a digital file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReference {
    pub file_id: EntityId,
    pub filename: String,
    pub file_type: String,
    pub file_size: u64,
    pub checksum: String,
    pub storage_location: String,
}

/// Chain of custody entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyEntry {
    pub date: DateTime<Utc>,
    pub custodian: String,
    pub location: String,
    pub notes: Option<String>,
}

/// Acquisition information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcquisitionInfo {
    pub acquired_date: Option<DateTime<Utc>>,
    pub acquired_from: Option<String>,
    pub acquisition_method: Option<String>,
    pub cost: Option<String>,
    pub notes: Option<String>,
}

/// Theory-specific interpretation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoryInterpretation {
    pub theory_id: EntityId,
    pub interpretation: String,
    pub support_level: f32, // -1.0 to 1.0 (negative = contradicts)
    pub notes: Option<String>,
}

impl Evidence {
    /// Create new evidence
    pub fn new(
        title: impl Into<String>,
        record_type: RecordType,
        created_by: EntityId,
    ) -> Self {
        Self {
            metadata: EntityMetadata::new(created_by),
            classification: EvidenceClassification {
                record_type,
                ..Default::default()
            },
            citation: None,
            title: title.into(),
            description: String::new(),
            transcription: None,
            abstract_text: None,
            original_language: "English".to_string(),
            translations: Vec::new(),
            files: Vec::new(),
            ocr_text: None,
            creator: String::new(),
            creation_date: None,
            creation_purpose: String::new(),
            chain_of_custody: Vec::new(),
            repository_id: None,
            acquisition_info: AcquisitionInfo::default(),
            extracted_facts: Vec::new(),
            found_date: Utc::now(),
            found_by: created_by,
            search_context: String::new(),
            is_negative_evidence: false,
            quality: EvidenceQuality::Derivative,
            legibility: 1.0,
            completeness: 1.0,
            applicable_theories: Vec::new(),
            theory_interpretations: Vec::new(),
            parent_evidence: None,
            child_evidence: Vec::new(),
        }
    }
    
    /// Extract a fact from this evidence
    pub fn extract_fact(&mut self, fact: ExtractedFact) {
        self.extracted_facts.push(fact);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Add a theory interpretation
    pub fn add_interpretation(
        &mut self,
        theory_id: EntityId,
        interpretation: impl Into<String>,
        support_level: f32,
    ) {
        self.theory_interpretations.push(TheoryInterpretation {
            theory_id,
            interpretation: interpretation.into(),
            support_level,
            notes: None,
        });
        
        if !self.applicable_theories.contains(&theory_id) {
            self.applicable_theories.push(theory_id);
        }
        
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Get GPS classification
    pub fn gps_classification(&self) -> (&SourceType, &InformationType, &EvidenceType) {
        (
            &self.classification.source_type,
            &self.classification.information_type,
            &self.classification.evidence_type,
        )
    }
    
    /// Check if this is negative evidence
    pub fn is_negative(&self) -> bool {
        self.is_negative_evidence || 
        matches!(self.classification.evidence_type, EvidenceType::Negative)
    }
}

// Implement Entity trait
impl_entity!(Evidence, "Evidence");

// Implement NestableEntity for document bundles
#[async_trait::async_trait]
impl NestableEntity for Evidence {
    fn children(&self) -> Vec<EntityId> {
        self.child_evidence.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "Evidence"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if !self.child_evidence.contains(&child_id) {
            self.child_evidence.push(child_id);
            self.metadata.update(self.metadata.modified_by);
        }
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_evidence.len();
        self.child_evidence.retain(|id| id != &child_id);
        if self.child_evidence.len() < initial_len {
            self.metadata.update(self.metadata.modified_by);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

// Implement Validatable trait
impl_validatable!(Evidence);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_evidence_creation() {
        let researcher_id = EntityId::new();
        let evidence = Evidence::new(
            "1920 Census - Smith Family",
            RecordType::Census,
            researcher_id,
        );
        
        assert_eq!(evidence.title, "1920 Census - Smith Family");
        assert_eq!(evidence.classification.record_type, RecordType::Census);
        assert!(evidence.validate().await.is_valid());
    }
    
    #[tokio::test]
    async fn test_fact_extraction() {
        let researcher_id = EntityId::new();
        let mut evidence = Evidence::new(
            "Birth Certificate",
            RecordType::BirthCertificate,
            researcher_id,
        );
        
        let fact = ExtractedFact::new(
            FactType::Name,
            "John Smith",
            researcher_id,
        );
        
        evidence.extract_fact(fact);
        assert_eq!(evidence.extracted_facts.len(), 1);
        assert_eq!(evidence.extracted_facts[0].fact_type, FactType::Name);
    }
}