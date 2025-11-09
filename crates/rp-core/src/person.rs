//! Person Entity - Canonical Authority Control
//!
//! This module implements Person as a canonical entity (authority record) based on
//! library science best practices from Koha's 25-year-old authority control system.
//!
//! ## Architecture
//!
//! Person is separate from IdentityPersona:
//! - **IdentityPersona**: Research workflow (Reference → Working → Hypothesis → Concluded)
//! - **Person**: Canonical authority record (the definitive person entity)
//!
//! When an IdentityPersona reaches "Concluded" state, it links to or creates a Person.
//!
//! ## Key Features
//!
//! 1. **Canonical Names**: One authoritative record per individual
//! 2. **Variant Names**: All name forms systematically tracked (MARC 4XX pattern)
//! 3. **Relationships**: Typed family relationships with certainty (MARC 5XX pattern)
//! 4. **Source Linking**: Citations link to canonical persons (MARC $9 pattern)
//! 5. **Merge Operations**: Production-grade duplicate handling with audit trail
//! 6. **Search Integration**: Variant names embedded in search index
//!
//! ## References
//!
//! - Design: RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md
//! - Research: KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md
//!
//! ## Example
//!
//! ```
//! use rp_core::person::{Person, Sex, GenealogyDate, DateCertainty};
//!
//! let person = Person::new_builder()
//!     .given_name("John")
//!     .surname("Smith")
//!     .birth_date(GenealogyDate::exact(1820, Some(3), Some(15)))
//!     .sex(Sex::Male)
//!     .build()?;
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;
use validator::Validate;

// =============================================================================
// CORE TYPES
// =============================================================================

/// Sex/Gender of a person
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Sex {
    Male,
    Female,
    Unknown,
}

impl Default for Sex {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Confidence level in a conclusion (1-5 scale)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonConfidence {
    Speculative = 1,   // Weak hypothesis
    Uncertain = 2,     // Some evidence, but inconclusive
    Possible = 3,      // Reasonable possibility
    Probable = 4,      // Strong evidence
    Definite = 5,      // Conclusive evidence
}

impl fmt::Display for PersonConfidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Speculative => write!(f, "Speculative"),
            Self::Uncertain => write!(f, "Uncertain"),
            Self::Possible => write!(f, "Possible"),
            Self::Probable => write!(f, "Probable"),
            Self::Definite => write!(f, "Definite"),
        }
    }
}

/// Date certainty type for genealogical dates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DateCertainty {
    Exact,        // Known exact date
    Estimated,    // Estimated from context
    Calculated,   // Calculated from other information
    Before,       // Before this date
    After,        // After this date
    Between,      // Between two dates (range)
}

impl fmt::Display for DateCertainty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => write!(f, "exact"),
            Self::Estimated => write!(f, "estimated"),
            Self::Calculated => write!(f, "calculated"),
            Self::Before => write!(f, "before"),
            Self::After => write!(f, "after"),
            Self::Between => write!(f, "between"),
        }
    }
}

/// Genealogical date with uncertainty modeling
///
/// Supports various certainty levels and date formats common in genealogy:
/// - Exact: 1850-03-15
/// - Circa: ca. 1850
/// - Estimated: est. 1850
/// - Before: bef. 1850
/// - After: aft. 1850
/// - Between: bet. 1850 and 1860
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GenealogyDate {
    pub year: Option<i32>,
    #[validate(range(min = 1, max = 12))]
    pub month: Option<u8>,
    #[validate(range(min = 1, max = 31))]
    pub day: Option<u8>,
    pub certainty: DateCertainty,
    pub circa: bool,
    pub end_year: Option<i32>,  // For "between" ranges
    #[validate(length(max = 200))]
    pub original_text: Option<String>,  // Original date string from source
}

impl GenealogyDate {
    /// Create an exact date
    pub fn exact(year: i32, month: Option<u8>, day: Option<u8>) -> Self {
        Self {
            year: Some(year),
            month,
            day,
            certainty: DateCertainty::Exact,
            circa: false,
            end_year: None,
            original_text: None,
        }
    }

    /// Create a circa date (approximate)
    pub fn circa(year: i32) -> Self {
        Self {
            year: Some(year),
            month: None,
            day: None,
            certainty: DateCertainty::Exact,
            circa: true,
            end_year: None,
            original_text: None,
        }
    }

    /// Create an estimated date
    pub fn estimated(year: i32) -> Self {
        Self {
            year: Some(year),
            month: None,
            day: None,
            certainty: DateCertainty::Estimated,
            circa: false,
            end_year: None,
            original_text: None,
        }
    }

    /// Create a calculated date
    pub fn calculated(year: i32) -> Self {
        Self {
            year: Some(year),
            month: None,
            day: None,
            certainty: DateCertainty::Calculated,
            circa: false,
            end_year: None,
            original_text: None,
        }
    }

    /// Create a "before" date
    pub fn before(year: i32) -> Self {
        Self {
            year: Some(year),
            month: None,
            day: None,
            certainty: DateCertainty::Before,
            circa: false,
            end_year: None,
            original_text: None,
        }
    }

    /// Create an "after" date
    pub fn after(year: i32) -> Self {
        Self {
            year: Some(year),
            month: None,
            day: None,
            certainty: DateCertainty::After,
            circa: false,
            end_year: None,
            original_text: None,
        }
    }

    /// Create a "between" date range
    pub fn between(start_year: i32, end_year: i32) -> Self {
        Self {
            year: Some(start_year),
            month: None,
            day: None,
            certainty: DateCertainty::Between,
            circa: false,
            end_year: Some(end_year),
            original_text: None,
        }
    }

    /// Display year with certainty indicators
    ///
    /// Examples:
    /// - 1850 (exact)
    /// - ca. 1850 (circa)
    /// - est. 1850 (estimated)
    /// - bef. 1850 (before)
    /// - aft. 1850 (after)
    /// - bet. 1850 - 1860 (between)
    pub fn display_year(&self) -> String {
        match (&self.year, &self.certainty, self.circa) {
            (Some(year), DateCertainty::Exact, false) => year.to_string(),
            (Some(year), DateCertainty::Exact, true) => format!("ca. {}", year),
            (Some(year), DateCertainty::Estimated, _) => format!("est. {}", year),
            (Some(year), DateCertainty::Calculated, _) => format!("calc. {}", year),
            (Some(year), DateCertainty::Before, _) => format!("bef. {}", year),
            (Some(year), DateCertainty::After, _) => format!("aft. {}", year),
            (Some(year), DateCertainty::Between, _) => {
                if let Some(end) = self.end_year {
                    format!("bet. {} - {}", year, end)
                } else {
                    format!("bet. {}", year)
                }
            }
            (None, _, _) => "Unknown".to_string(),
        }
    }

    /// Display full date if available
    ///
    /// Examples:
    /// - 1850-03-15 (full date)
    /// - 1850-03 (year and month)
    /// - 1850 (year only)
    pub fn display_full(&self) -> String {
        match (&self.year, &self.month, &self.day) {
            (Some(y), Some(m), Some(d)) => format!("{:04}-{:02}-{:02}", y, m, d),
            (Some(y), Some(m), None) => format!("{:04}-{:02}", y, m),
            (Some(_y), None, None) => self.display_year(),
            _ => "Unknown".to_string(),
        }
    }

    /// Get sortable year (for timeline queries)
    pub fn sortable_year(&self) -> Option<i32> {
        self.year
    }
}

impl fmt::Display for GenealogyDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_full())
    }
}

// =============================================================================
// PERSON ENTITY (Canonical Authority Record)
// =============================================================================

/// Canonical person entity (authority record)
///
/// Represents THE definitive person - one record per individual.
/// This is analogous to Koha's authority records (auth_header table).
///
/// ## Design Principles
///
/// 1. **One Person = One Record**: No duplicates allowed
/// 2. **Canonical Form**: Established name form with variants tracked separately
/// 3. **Source Attribution**: Every claim backed by evidence
/// 4. **Audit Trail**: Complete history of all changes
/// 5. **Merge Support**: Production-grade duplicate handling
///
/// ## Workflow
///
/// ```text
/// IdentityPersona (Research) → Person (Canonical)
///       ↓                            ↓
///   Working state              Authority record
///   Hypothesis                 Variant names
///   Analysis                   Relationships
///       ↓                            ↓
///   Concluded → Links to → Person entity
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Person {
    /// Unique identifier
    pub person_id: Uuid,

    // --- Canonical Name Components (established form) ---
    #[validate(length(min = 1, max = 255))]
    pub given_name: Option<String>,

    #[validate(length(min = 1, max = 255))]
    pub surname: Option<String>,

    #[validate(length(max = 255))]
    pub middle_name: Option<String>,

    #[validate(length(max = 50))]
    pub name_prefix: Option<String>,  // Dr., Rev., Hon., etc.

    #[validate(length(max = 50))]
    pub name_suffix: Option<String>,  // Jr., III, Esq., etc.

    // --- Vital Events ---
    pub birth_date: Option<GenealogyDate>,
    pub birth_place_id: Option<Uuid>,
    pub birth_source_id: Option<Uuid>,

    pub death_date: Option<GenealogyDate>,
    pub death_place_id: Option<Uuid>,
    pub death_source_id: Option<Uuid>,

    // --- Biographical ---
    pub sex: Sex,

    #[validate(length(max = 500))]
    pub occupation: Option<String>,

    #[validate(length(max = 100))]
    pub religion: Option<String>,

    // --- Research Notes ---
    pub notes: Option<String>,
    pub research_notes: Option<String>,     // Private notes
    pub conclusion_notes: Option<String>,   // Why this canonical form

    /// Overall confidence in this canonical person conclusion
    pub conclusion_confidence: Option<PersonConfidence>,

    // --- MARC Export Cache (for library system interoperability) ---
    pub marc_binary: Option<Vec<u8>>,
    pub marc_xml: Option<String>,
    pub marc_updated_at: Option<DateTime<Utc>>,

    // --- Metadata ---
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub last_modified_by: Uuid,

    // --- Archival (soft delete for merged persons) ---
    pub archived: bool,
    pub archived_reason: Option<String>,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
}

impl Person {
    /// Create a new Person with minimal information
    pub fn new(
        given_name: Option<String>,
        surname: Option<String>,
        created_by: Uuid,
    ) -> Result<Self, String> {
        if given_name.is_none() && surname.is_none() {
            return Err("Person must have either given name or surname".to_string());
        }

        Ok(Self {
            person_id: Uuid::now_v7(),
            given_name,
            surname,
            middle_name: None,
            name_prefix: None,
            name_suffix: None,
            birth_date: None,
            birth_place_id: None,
            birth_source_id: None,
            death_date: None,
            death_place_id: None,
            death_source_id: None,
            sex: Sex::Unknown,
            occupation: None,
            religion: None,
            notes: None,
            research_notes: None,
            conclusion_notes: None,
            conclusion_confidence: None,
            marc_binary: None,
            marc_xml: None,
            marc_updated_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by,
            last_modified_by: created_by,
            archived: false,
            archived_reason: None,
            archived_at: None,
            archived_by: None,
        })
    }

    /// Create a PersonBuilder for fluent API
    pub fn builder() -> PersonBuilder {
        PersonBuilder::default()
    }

    // --- Name Methods ---

    /// Get canonical full name in standard format
    ///
    /// Format: [Prefix] [Given] [Middle] [Surname] [Suffix]
    /// Example: "Dr. John William Smith Jr."
    pub fn canonical_name(&self) -> String {
        let mut parts = Vec::new();

        if let Some(prefix) = &self.name_prefix {
            parts.push(prefix.clone());
        }
        if let Some(given) = &self.given_name {
            parts.push(given.clone());
        }
        if let Some(middle) = &self.middle_name {
            parts.push(middle.clone());
        }
        if let Some(surname) = &self.surname {
            parts.push(surname.clone());
        }
        if let Some(suffix) = &self.name_suffix {
            parts.push(suffix.clone());
        }

        if parts.is_empty() {
            "Unknown Person".to_string()
        } else {
            parts.join(" ")
        }
    }

    /// Get surname-first format (for indexing/sorting)
    ///
    /// Format: [Surname], [Prefix] [Given] [Middle] [Suffix]
    /// Example: "Smith, Dr. John William Jr."
    pub fn surname_first(&self) -> String {
        let mut parts = Vec::new();

        if let Some(surname) = &self.surname {
            parts.push(format!("{},", surname));
        }
        if let Some(prefix) = &self.name_prefix {
            parts.push(prefix.clone());
        }
        if let Some(given) = &self.given_name {
            parts.push(given.clone());
        }
        if let Some(middle) = &self.middle_name {
            parts.push(middle.clone());
        }
        if let Some(suffix) = &self.name_suffix {
            parts.push(suffix.clone());
        }

        if parts.is_empty() {
            "Unknown Person".to_string()
        } else {
            parts.join(" ")
        }
    }

    // --- Date Methods ---

    /// Get years lived (for display)
    ///
    /// Examples:
    /// - "1820 - 1891"
    /// - "ca. 1820 - 1891"
    /// - "1820 - " (still living or death unknown)
    /// - " - 1891" (birth unknown)
    pub fn life_span(&self) -> Option<String> {
        match (&self.birth_date, &self.death_date) {
            (Some(birth), Some(death)) => {
                let b = birth.display_year();
                let d = death.display_year();
                Some(format!("{} - {}", b, d))
            }
            (Some(birth), None) => Some(format!("{} - ", birth.display_year())),
            (None, Some(death)) => Some(format!(" - {}", death.display_year())),
            (None, None) => None,
        }
    }

    /// Get estimated age at death
    pub fn age_at_death(&self) -> Option<i32> {
        match (&self.birth_date, &self.death_date) {
            (Some(birth), Some(death)) => {
                match (birth.sortable_year(), death.sortable_year()) {
                    (Some(b), Some(d)) => Some(d - b),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    // --- Status Methods ---

    /// Check if person is active (not archived)
    pub fn is_active(&self) -> bool {
        !self.archived
    }

    /// Check if person is archived (soft deleted/merged)
    pub fn is_archived(&self) -> bool {
        self.archived
    }

    /// Archive this person (soft delete)
    pub fn archive(&mut self, reason: String, archived_by: Uuid) {
        self.archived = true;
        self.archived_reason = Some(reason);
        self.archived_at = Some(Utc::now());
        self.archived_by = Some(archived_by);
        self.updated_at = Utc::now();
        self.last_modified_by = archived_by;
    }

    /// Un-archive this person (restore)
    pub fn unarchive(&mut self, restored_by: Uuid) {
        self.archived = false;
        self.archived_reason = None;
        self.archived_at = None;
        self.archived_by = None;
        self.updated_at = Utc::now();
        self.last_modified_by = restored_by;
    }

    // --- MARC Methods ---

    /// Check if MARC cache is valid
    pub fn has_valid_marc_cache(&self) -> bool {
        self.marc_binary.is_some() && self.marc_xml.is_some()
    }

    /// Invalidate MARC cache (call after any update)
    pub fn invalidate_marc_cache(&mut self) {
        self.marc_binary = None;
        self.marc_xml = None;
        self.marc_updated_at = None;
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.canonical_name())?;
        if let Some(life_span) = self.life_span() {
            write!(f, " ({})", life_span)?;
        }
        Ok(())
    }
}

// =============================================================================
// PERSON BUILDER (Fluent API)
// =============================================================================

/// Builder for creating Person entities with fluent API
#[derive(Default)]
pub struct PersonBuilder {
    given_name: Option<String>,
    surname: Option<String>,
    middle_name: Option<String>,
    name_prefix: Option<String>,
    name_suffix: Option<String>,
    birth_date: Option<GenealogyDate>,
    birth_place_id: Option<Uuid>,
    birth_source_id: Option<Uuid>,
    death_date: Option<GenealogyDate>,
    death_place_id: Option<Uuid>,
    death_source_id: Option<Uuid>,
    sex: Sex,
    occupation: Option<String>,
    religion: Option<String>,
    notes: Option<String>,
    conclusion_confidence: Option<PersonConfidence>,
    created_by: Option<Uuid>,
}

impl PersonBuilder {
    pub fn given_name(mut self, name: impl Into<String>) -> Self {
        self.given_name = Some(name.into());
        self
    }

    pub fn surname(mut self, name: impl Into<String>) -> Self {
        self.surname = Some(name.into());
        self
    }

    pub fn middle_name(mut self, name: impl Into<String>) -> Self {
        self.middle_name = Some(name.into());
        self
    }

    pub fn name_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.name_prefix = Some(prefix.into());
        self
    }

    pub fn name_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.name_suffix = Some(suffix.into());
        self
    }

    pub fn birth_date(mut self, date: GenealogyDate) -> Self {
        self.birth_date = Some(date);
        self
    }

    pub fn birth_place(mut self, place_id: Uuid) -> Self {
        self.birth_place_id = Some(place_id);
        self
    }

    pub fn birth_source(mut self, source_id: Uuid) -> Self {
        self.birth_source_id = Some(source_id);
        self
    }

    pub fn death_date(mut self, date: GenealogyDate) -> Self {
        self.death_date = Some(date);
        self
    }

    pub fn death_place(mut self, place_id: Uuid) -> Self {
        self.death_place_id = Some(place_id);
        self
    }

    pub fn death_source(mut self, source_id: Uuid) -> Self {
        self.death_source_id = Some(source_id);
        self
    }

    pub fn sex(mut self, sex: Sex) -> Self {
        self.sex = sex;
        self
    }

    pub fn occupation(mut self, occupation: impl Into<String>) -> Self {
        self.occupation = Some(occupation.into());
        self
    }

    pub fn religion(mut self, religion: impl Into<String>) -> Self {
        self.religion = Some(religion.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn conclusion_confidence(mut self, confidence: PersonConfidence) -> Self {
        self.conclusion_confidence = Some(confidence);
        self
    }

    pub fn created_by(mut self, user_id: Uuid) -> Self {
        self.created_by = Some(user_id);
        self
    }

    pub fn build(self) -> Result<Person, String> {
        let created_by = self.created_by.ok_or("created_by is required")?;

        if self.given_name.is_none() && self.surname.is_none() {
            return Err("Person must have either given name or surname".to_string());
        }

        let now = Utc::now();

        Ok(Person {
            person_id: Uuid::now_v7(),
            given_name: self.given_name,
            surname: self.surname,
            middle_name: self.middle_name,
            name_prefix: self.name_prefix,
            name_suffix: self.name_suffix,
            birth_date: self.birth_date,
            birth_place_id: self.birth_place_id,
            birth_source_id: self.birth_source_id,
            death_date: self.death_date,
            death_place_id: self.death_place_id,
            death_source_id: self.death_source_id,
            sex: self.sex,
            occupation: self.occupation,
            religion: self.religion,
            notes: self.notes,
            research_notes: None,
            conclusion_notes: None,
            conclusion_confidence: self.conclusion_confidence,
            marc_binary: None,
            marc_xml: None,
            marc_updated_at: None,
            created_at: now,
            updated_at: now,
            created_by,
            last_modified_by: created_by,
            archived: false,
            archived_reason: None,
            archived_at: None,
            archived_by: None,
        })
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_name() {
        let person = Person::builder()
            .given_name("John")
            .surname("Smith")
            .middle_name("William")
            .name_suffix("Jr.")
            .created_by(Uuid::now_v7())
            .build()
            .unwrap();

        assert_eq!(person.canonical_name(), "John William Smith Jr.");
    }

    #[test]
    fn test_surname_first() {
        let person = Person::builder()
            .given_name("John")
            .surname("Smith")
            .created_by(Uuid::now_v7())
            .build()
            .unwrap();

        assert_eq!(person.surname_first(), "Smith, John");
    }

    #[test]
    fn test_life_span() {
        let person = Person::builder()
            .given_name("John")
            .surname("Smith")
            .birth_date(GenealogyDate::exact(1820, None, None))
            .death_date(GenealogyDate::exact(1891, None, None))
            .created_by(Uuid::now_v7())
            .build()
            .unwrap();

        assert_eq!(person.life_span(), Some("1820 - 1891".to_string()));
    }

    #[test]
    fn test_life_span_circa() {
        let person = Person::builder()
            .given_name("John")
            .surname("Smith")
            .birth_date(GenealogyDate::circa(1820))
            .death_date(GenealogyDate::exact(1891, None, None))
            .created_by(Uuid::now_v7())
            .build()
            .unwrap();

        assert_eq!(person.life_span(), Some("ca. 1820 - 1891".to_string()));
    }

    #[test]
    fn test_age_at_death() {
        let person = Person::builder()
            .given_name("John")
            .surname("Smith")
            .birth_date(GenealogyDate::exact(1820, None, None))
            .death_date(GenealogyDate::exact(1891, None, None))
            .created_by(Uuid::now_v7())
            .build()
            .unwrap();

        assert_eq!(person.age_at_death(), Some(71));
    }

    #[test]
    fn test_genealogy_date_display() {
        assert_eq!(GenealogyDate::exact(1850, None, None).display_year(), "1850");
        assert_eq!(GenealogyDate::circa(1850).display_year(), "ca. 1850");
        assert_eq!(GenealogyDate::estimated(1850).display_year(), "est. 1850");
        assert_eq!(GenealogyDate::before(1850).display_year(), "bef. 1850");
        assert_eq!(GenealogyDate::after(1850).display_year(), "aft. 1850");
        assert_eq!(GenealogyDate::between(1850, 1860).display_year(), "bet. 1850 - 1860");
    }

    #[test]
    fn test_genealogy_date_full() {
        let date = GenealogyDate::exact(1850, Some(3), Some(15));
        assert_eq!(date.display_full(), "1850-03-15");

        let date = GenealogyDate::exact(1850, Some(3), None);
        assert_eq!(date.display_full(), "1850-03");

        let date = GenealogyDate::exact(1850, None, None);
        assert_eq!(date.display_full(), "1850");
    }

    #[test]
    fn test_archive_unarchive() {
        let user_id = Uuid::now_v7();
        let mut person = Person::builder()
            .given_name("John")
            .surname("Smith")
            .created_by(user_id)
            .build()
            .unwrap();

        assert!(person.is_active());
        assert!(!person.is_archived());

        person.archive("Merged into another person".to_string(), user_id);
        assert!(!person.is_active());
        assert!(person.is_archived());
        assert_eq!(person.archived_reason, Some("Merged into another person".to_string()));

        person.unarchive(user_id);
        assert!(person.is_active());
        assert!(!person.is_archived());
        assert_eq!(person.archived_reason, None);
    }

    #[test]
    fn test_must_have_name() {
        let result = Person::builder()
            .created_by(Uuid::now_v7())
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Person must have either given name or surname");
    }

    #[test]
    fn test_confidence_ordering() {
        assert!(PersonConfidence::Definite > PersonConfidence::Probable);
        assert!(PersonConfidence::Probable > PersonConfidence::Possible);
        assert!(PersonConfidence::Possible > PersonConfidence::Uncertain);
        assert!(PersonConfidence::Uncertain > PersonConfidence::Speculative);
    }
}
