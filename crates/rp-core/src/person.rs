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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, utoipa::ToSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Validate, utoipa::ToSchema)]
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
// VARIANT NAME (MARC 4XX Pattern)
// =============================================================================

/// Type of name variant
///
/// Follows genealogical conventions for different name forms a person used:
/// - **birth**: Name at birth (maiden name)
/// - **married**: Name after marriage
/// - **divorced**: Name after divorce
/// - **nickname**: Informal name, pet name
/// - **immigration**: Name changed upon immigration
/// - **spelling**: Spelling variation (e.g., Smith/Smyth)
/// - **translation**: Translation to another language
/// - **abbreviation**: Shortened form (e.g., Wm. for William)
/// - **pseudonym**: Pen name, stage name, alias
/// - **legal**: Legally changed name
/// - **religious**: Name upon taking religious orders
/// - **stage**: Professional/stage name
/// - **documented**: Name found in a specific document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum VariantNameType {
    Birth,
    Married,
    Divorced,
    Nickname,
    Immigration,
    Spelling,
    Translation,
    Abbreviation,
    Pseudonym,
    Legal,
    Religious,
    Stage,
    Documented,
}

impl fmt::Display for VariantNameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Birth => write!(f, "Birth Name"),
            Self::Married => write!(f, "Married Name"),
            Self::Divorced => write!(f, "Post-Divorce Name"),
            Self::Nickname => write!(f, "Nickname"),
            Self::Immigration => write!(f, "Immigration Name"),
            Self::Spelling => write!(f, "Spelling Variation"),
            Self::Translation => write!(f, "Translation"),
            Self::Abbreviation => write!(f, "Abbreviation"),
            Self::Pseudonym => write!(f, "Pseudonym"),
            Self::Legal => write!(f, "Legal Name"),
            Self::Religious => write!(f, "Religious Name"),
            Self::Stage => write!(f, "Stage Name"),
            Self::Documented => write!(f, "Documented Name"),
        }
    }
}

/// Name variant for a person (MARC 4XX "See From Tracing")
///
/// Represents alternative name forms a person used throughout their life.
/// This follows the library science pattern where:
/// - 1XX field = Established heading (canonical name in Person entity)
/// - 4XX field = See From Tracing (variant names in this struct)
///
/// ## Examples
///
/// - Birth name: "Mary Elizabeth Johnson" → Married: "Mary Elizabeth Smith"
/// - Immigration: "Johann Schmidt" → Americanized: "John Smith"
/// - Spelling: "Smyth" vs "Smith"
/// - Nickname: "Jack" for "John"
///
/// ## Search Integration
///
/// All variant names are included in full-text search indexes, allowing users
/// to find canonical Person records by any known name form.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VariantName {
    /// Unique identifier
    pub variant_id: Uuid,

    /// Person this variant belongs to
    pub person_id: Uuid,

    // --- Name Components ---
    #[validate(length(max = 255))]
    pub given_name: Option<String>,

    #[validate(length(max = 255))]
    pub surname: Option<String>,

    #[validate(length(max = 255))]
    pub middle_name: Option<String>,

    #[validate(length(max = 50))]
    pub name_prefix: Option<String>,

    #[validate(length(max = 50))]
    pub name_suffix: Option<String>,

    /// Full assembled name (for search indexing)
    #[validate(length(min = 1, max = 500))]
    pub full_name: String,

    /// Type of variant
    pub variant_type: VariantNameType,

    // --- Time Period (when this name was used) ---
    pub use_from_year: Option<i32>,
    pub use_to_year: Option<i32>,

    /// Notes about this variant
    pub notes: Option<String>,

    /// Source documenting this name variant
    pub source_id: Option<Uuid>,

    // --- Metadata ---
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VariantName {
    /// Create a new variant name
    pub fn new(
        person_id: Uuid,
        given_name: Option<String>,
        surname: Option<String>,
        variant_type: VariantNameType,
    ) -> Result<Self, String> {
        // Build full name
        let full_name = Self::build_full_name(
            &given_name,
            &surname,
            &None,
            &None,
            &None,
        );

        if full_name.is_empty() {
            return Err("Variant name must have at least given name or surname".to_string());
        }

        Ok(Self {
            variant_id: Uuid::now_v7(),
            person_id,
            given_name,
            surname,
            middle_name: None,
            name_prefix: None,
            name_suffix: None,
            full_name,
            variant_type,
            use_from_year: None,
            use_to_year: None,
            notes: None,
            source_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Create a VariantNameBuilder for fluent API
    pub fn builder(person_id: Uuid) -> VariantNameBuilder {
        VariantNameBuilder::new(person_id)
    }

    // --- Name Methods ---

    /// Build full name from components
    fn build_full_name(
        given: &Option<String>,
        surname: &Option<String>,
        middle: &Option<String>,
        prefix: &Option<String>,
        suffix: &Option<String>,
    ) -> String {
        let mut parts = Vec::new();

        if let Some(p) = prefix {
            parts.push(p.clone());
        }
        if let Some(g) = given {
            parts.push(g.clone());
        }
        if let Some(m) = middle {
            parts.push(m.clone());
        }
        if let Some(s) = surname {
            parts.push(s.clone());
        }
        if let Some(sf) = suffix {
            parts.push(sf.clone());
        }

        parts.join(" ")
    }

    /// Rebuild full name from current components
    pub fn rebuild_full_name(&mut self) {
        self.full_name = Self::build_full_name(
            &self.given_name,
            &self.surname,
            &self.middle_name,
            &self.name_prefix,
            &self.name_suffix,
        );
        self.updated_at = Utc::now();
    }

    /// Get display name with variant type
    ///
    /// Example: "Mary Elizabeth Johnson (Birth Name)"
    pub fn display_name_with_type(&self) -> String {
        format!("{} ({})", self.full_name, self.variant_type)
    }

    /// Get time period string if available
    ///
    /// Examples:
    /// - "1820-1845"
    /// - "1820-"
    /// - "-1845"
    pub fn time_period(&self) -> Option<String> {
        match (&self.use_from_year, &self.use_to_year) {
            (Some(from), Some(to)) => Some(format!("{}-{}", from, to)),
            (Some(from), None) => Some(format!("{}-", from)),
            (None, Some(to)) => Some(format!("-{}", to)),
            (None, None) => None,
        }
    }

    // --- Matching Methods ---

    /// Check if this variant matches a search string (case-insensitive)
    pub fn matches(&self, search: &str) -> bool {
        let search_lower = search.to_lowercase();

        // Match against full name
        if self.full_name.to_lowercase().contains(&search_lower) {
            return true;
        }

        // Match against individual components
        if let Some(ref given) = self.given_name {
            if given.to_lowercase().contains(&search_lower) {
                return true;
            }
        }
        if let Some(ref surname) = self.surname {
            if surname.to_lowercase().contains(&search_lower) {
                return true;
            }
        }

        false
    }

    /// Check if this variant matches another variant (for duplicate detection)
    pub fn matches_variant(&self, other: &VariantName) -> bool {
        // Exact full name match (case-insensitive)
        if self.full_name.to_lowercase() == other.full_name.to_lowercase() {
            return true;
        }

        // Component match (both given and surname must match)
        let given_match = match (&self.given_name, &other.given_name) {
            (Some(a), Some(b)) => a.to_lowercase() == b.to_lowercase(),
            (None, None) => true,
            _ => false,
        };

        let surname_match = match (&self.surname, &other.surname) {
            (Some(a), Some(b)) => a.to_lowercase() == b.to_lowercase(),
            (None, None) => true,
            _ => false,
        };

        given_match && surname_match
    }
}

impl fmt::Display for VariantName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}

// =============================================================================
// VARIANT NAME BUILDER
// =============================================================================

/// Builder for creating VariantName entities with fluent API
pub struct VariantNameBuilder {
    person_id: Uuid,
    given_name: Option<String>,
    surname: Option<String>,
    middle_name: Option<String>,
    name_prefix: Option<String>,
    name_suffix: Option<String>,
    variant_type: Option<VariantNameType>,
    use_from_year: Option<i32>,
    use_to_year: Option<i32>,
    notes: Option<String>,
    source_id: Option<Uuid>,
}

impl VariantNameBuilder {
    pub fn new(person_id: Uuid) -> Self {
        Self {
            person_id,
            given_name: None,
            surname: None,
            middle_name: None,
            name_prefix: None,
            name_suffix: None,
            variant_type: None,
            use_from_year: None,
            use_to_year: None,
            notes: None,
            source_id: None,
        }
    }

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

    pub fn variant_type(mut self, variant_type: VariantNameType) -> Self {
        self.variant_type = Some(variant_type);
        self
    }

    pub fn use_from_year(mut self, year: i32) -> Self {
        self.use_from_year = Some(year);
        self
    }

    pub fn use_to_year(mut self, year: i32) -> Self {
        self.use_to_year = Some(year);
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn source_id(mut self, source_id: Uuid) -> Self {
        self.source_id = Some(source_id);
        self
    }

    pub fn build(self) -> Result<VariantName, String> {
        let variant_type = self.variant_type.ok_or("variant_type is required")?;

        // Build full name
        let full_name = VariantName::build_full_name(
            &self.given_name,
            &self.surname,
            &self.middle_name,
            &self.name_prefix,
            &self.name_suffix,
        );

        if full_name.is_empty() {
            return Err("Variant name must have at least given name or surname".to_string());
        }

        let now = Utc::now();

        Ok(VariantName {
            variant_id: Uuid::now_v7(),
            person_id: self.person_id,
            given_name: self.given_name,
            surname: self.surname,
            middle_name: self.middle_name,
            name_prefix: self.name_prefix,
            name_suffix: self.name_suffix,
            full_name,
            variant_type,
            use_from_year: self.use_from_year,
            use_to_year: self.use_to_year,
            notes: self.notes,
            source_id: self.source_id,
            created_at: now,
            updated_at: now,
        })
    }
}

// =============================================================================
// PERSON RELATIONSHIP (MARC 5XX Pattern)
// =============================================================================

/// Type of relationship between two persons
///
/// Covers all standard genealogical relationships including biological,
/// legal, and social relationships. Relationships are directional:
/// - **parent**: person_id is parent of related_person_id
/// - **child**: person_id is child of related_person_id
/// - **spouse**: person_id is spouse of related_person_id
/// - **sibling**: person_id is sibling of related_person_id
///
/// Extended family and non-biological relationships are also supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PersonRelationshipType {
    Parent,
    Child,
    Spouse,
    Sibling,
    Grandparent,
    Grandchild,
    AuntUncle,
    NieceNephew,
    Cousin,
    StepParent,
    StepChild,
    AdoptiveParent,
    AdoptiveChild,
    FosterParent,
    FosterChild,
    Guardian,
    Ward,
}

impl PersonRelationshipType {
    /// Get the reciprocal relationship type
    ///
    /// Examples:
    /// - Parent ↔ Child
    /// - Spouse ↔ Spouse (reflexive)
    /// - Grandparent ↔ Grandchild
    pub fn reciprocal(&self) -> Self {
        match self {
            Self::Parent => Self::Child,
            Self::Child => Self::Parent,
            Self::Spouse => Self::Spouse,
            Self::Sibling => Self::Sibling,
            Self::Grandparent => Self::Grandchild,
            Self::Grandchild => Self::Grandparent,
            Self::AuntUncle => Self::NieceNephew,
            Self::NieceNephew => Self::AuntUncle,
            Self::Cousin => Self::Cousin,
            Self::StepParent => Self::StepChild,
            Self::StepChild => Self::StepParent,
            Self::AdoptiveParent => Self::AdoptiveChild,
            Self::AdoptiveChild => Self::AdoptiveParent,
            Self::FosterParent => Self::FosterChild,
            Self::FosterChild => Self::FosterParent,
            Self::Guardian => Self::Ward,
            Self::Ward => Self::Guardian,
        }
    }

    /// Check if this relationship type is reflexive (same both ways)
    pub fn is_reflexive(&self) -> bool {
        matches!(self, Self::Spouse | Self::Sibling | Self::Cousin)
    }

    /// Check if this is a biological relationship
    pub fn is_biological(&self) -> bool {
        matches!(
            self,
            Self::Parent
                | Self::Child
                | Self::Sibling
                | Self::Grandparent
                | Self::Grandchild
                | Self::AuntUncle
                | Self::NieceNephew
                | Self::Cousin
        )
    }

    /// Check if this is a legal/adoptive relationship
    pub fn is_legal(&self) -> bool {
        matches!(
            self,
            Self::AdoptiveParent | Self::AdoptiveChild | Self::Guardian | Self::Ward
        )
    }

    /// Check if this is a step relationship
    pub fn is_step(&self) -> bool {
        matches!(self, Self::StepParent | Self::StepChild)
    }

    /// Check if this is a foster relationship
    pub fn is_foster(&self) -> bool {
        matches!(self, Self::FosterParent | Self::FosterChild)
    }
}

impl fmt::Display for PersonRelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parent => write!(f, "Parent"),
            Self::Child => write!(f, "Child"),
            Self::Spouse => write!(f, "Spouse"),
            Self::Sibling => write!(f, "Sibling"),
            Self::Grandparent => write!(f, "Grandparent"),
            Self::Grandchild => write!(f, "Grandchild"),
            Self::AuntUncle => write!(f, "Aunt/Uncle"),
            Self::NieceNephew => write!(f, "Niece/Nephew"),
            Self::Cousin => write!(f, "Cousin"),
            Self::StepParent => write!(f, "Step-Parent"),
            Self::StepChild => write!(f, "Step-Child"),
            Self::AdoptiveParent => write!(f, "Adoptive Parent"),
            Self::AdoptiveChild => write!(f, "Adoptive Child"),
            Self::FosterParent => write!(f, "Foster Parent"),
            Self::FosterChild => write!(f, "Foster Child"),
            Self::Guardian => write!(f, "Guardian"),
            Self::Ward => write!(f, "Ward"),
        }
    }
}

/// Relationship between two persons (MARC 5XX "See Also From Tracing")
///
/// Represents typed family relationships with time periods and confidence levels.
/// This follows the library science pattern where:
/// - 1XX field = Established heading (canonical Person)
/// - 5XX field = See Also From Tracing (related persons)
///
/// ## Directionality
///
/// Relationships are directional. For example:
/// - Person A (Parent) → Person B
/// - Person B (Child) → Person A
///
/// The system can automatically create reciprocal relationships.
///
/// ## Time Periods
///
/// Relationships can have start/end dates:
/// - Marriage: start = marriage date, end = divorce/death date
/// - Foster care: start/end = placement period
/// - Guardianship: start/end = legal guardianship period
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PersonRelationship {
    /// Unique identifier
    pub relationship_id: Uuid,

    /// Subject person (the "from" side of the relationship)
    pub person_id: Uuid,

    /// Related person (the "to" side of the relationship)
    pub related_person_id: Uuid,

    /// Type of relationship
    pub relationship_type: PersonRelationshipType,

    // --- Time Period ---
    pub start_date: Option<GenealogyDate>,
    pub end_date: Option<GenealogyDate>,

    /// Confidence in this relationship conclusion
    pub confidence: PersonConfidence,

    /// Notes about this relationship
    pub notes: Option<String>,

    /// Source documenting this relationship
    pub source_id: Option<Uuid>,

    // --- Metadata ---
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PersonRelationship {
    /// Create a new relationship
    pub fn new(
        person_id: Uuid,
        related_person_id: Uuid,
        relationship_type: PersonRelationshipType,
    ) -> Result<Self, String> {
        if person_id == related_person_id {
            return Err("Person cannot have relationship with themselves".to_string());
        }

        Ok(Self {
            relationship_id: Uuid::now_v7(),
            person_id,
            related_person_id,
            relationship_type,
            start_date: None,
            end_date: None,
            confidence: PersonConfidence::Possible,
            notes: None,
            source_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Create a PersonRelationshipBuilder for fluent API
    pub fn builder() -> PersonRelationshipBuilder {
        PersonRelationshipBuilder::default()
    }

    /// Get the reciprocal relationship
    ///
    /// Creates the inverse relationship (e.g., if this is "A is parent of B",
    /// returns "B is child of A")
    pub fn reciprocal(&self) -> Self {
        Self {
            relationship_id: Uuid::now_v7(),
            person_id: self.related_person_id,
            related_person_id: self.person_id,
            relationship_type: self.relationship_type.reciprocal(),
            start_date: self.start_date.clone(),
            end_date: self.end_date.clone(),
            confidence: self.confidence,
            notes: self.notes.clone(),
            source_id: self.source_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Check if relationship is currently active (not ended)
    pub fn is_active(&self) -> bool {
        self.end_date.is_none()
    }

    /// Get display name for this relationship
    ///
    /// Example: "Parent (1845-1891)"
    pub fn display(&self) -> String {
        match (&self.start_date, &self.end_date) {
            (Some(start), Some(end)) => {
                format!(
                    "{} ({} - {})",
                    self.relationship_type,
                    start.display_year(),
                    end.display_year()
                )
            }
            (Some(start), None) => {
                format!("{} ({} - )", self.relationship_type, start.display_year())
            }
            (None, Some(end)) => {
                format!("{} ( - {})", self.relationship_type, end.display_year())
            }
            (None, None) => format!("{}", self.relationship_type),
        }
    }

    /// Get duration in years (if both dates available)
    pub fn duration_years(&self) -> Option<i32> {
        match (&self.start_date, &self.end_date) {
            (Some(start), Some(end)) => match (start.sortable_year(), end.sortable_year()) {
                (Some(s), Some(e)) => Some(e - s),
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for PersonRelationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

// =============================================================================
// PERSON RELATIONSHIP BUILDER
// =============================================================================

/// Builder for creating PersonRelationship entities with fluent API
#[derive(Default)]
pub struct PersonRelationshipBuilder {
    person_id: Option<Uuid>,
    related_person_id: Option<Uuid>,
    relationship_type: Option<PersonRelationshipType>,
    start_date: Option<GenealogyDate>,
    end_date: Option<GenealogyDate>,
    confidence: Option<PersonConfidence>,
    notes: Option<String>,
    source_id: Option<Uuid>,
}

impl PersonRelationshipBuilder {
    pub fn person_id(mut self, id: Uuid) -> Self {
        self.person_id = Some(id);
        self
    }

    pub fn related_person_id(mut self, id: Uuid) -> Self {
        self.related_person_id = Some(id);
        self
    }

    pub fn relationship_type(mut self, rel_type: PersonRelationshipType) -> Self {
        self.relationship_type = Some(rel_type);
        self
    }

    pub fn start_date(mut self, date: GenealogyDate) -> Self {
        self.start_date = Some(date);
        self
    }

    pub fn end_date(mut self, date: GenealogyDate) -> Self {
        self.end_date = Some(date);
        self
    }

    pub fn confidence(mut self, confidence: PersonConfidence) -> Self {
        self.confidence = Some(confidence);
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn source_id(mut self, source_id: Uuid) -> Self {
        self.source_id = Some(source_id);
        self
    }

    pub fn build(self) -> Result<PersonRelationship, String> {
        let person_id = self.person_id.ok_or("person_id is required")?;
        let related_person_id = self
            .related_person_id
            .ok_or("related_person_id is required")?;
        let relationship_type = self.relationship_type.ok_or("relationship_type is required")?;

        if person_id == related_person_id {
            return Err("Person cannot have relationship with themselves".to_string());
        }

        let now = Utc::now();

        Ok(PersonRelationship {
            relationship_id: Uuid::now_v7(),
            person_id,
            related_person_id,
            relationship_type,
            start_date: self.start_date,
            end_date: self.end_date,
            confidence: self.confidence.unwrap_or(PersonConfidence::Possible),
            notes: self.notes,
            source_id: self.source_id,
            created_at: now,
            updated_at: now,
        })
    }
}

// =============================================================================
// SOURCE-PERSON LINKING (MARC $9 Pattern)
// =============================================================================

/// Link between a Source and a canonical Person (MARC $9 authority linking)
///
/// This represents a mention/appearance of a person in a source document.
/// Each SourcePerson links:
/// - A specific source citation
/// - The canonical Person entity it refers to
/// - The extracted name as it appears in the source
///
/// This follows the MARC $9 authority linking pattern where source mentions
/// ($9) link to canonical authority records (Person entities).
///
/// ## Use Cases
///
/// 1. **Source Transcription**: Extract names exactly as written
/// 2. **Authority Linking**: Connect extracted names to canonical persons
/// 3. **Evidence Trail**: Track all source mentions of a person
/// 4. **Name Variant Discovery**: Find name variations across sources
///
/// ## Example
///
/// ```text
/// Source: 1850 Census
/// Extracted: "Jno Smith" (farmer)
/// Links to: Person "John William Smith" (1820-1891)
/// Confidence: Probable
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SourcePerson {
    /// Unique identifier
    pub source_person_id: Uuid,

    /// Source citation ID
    pub source_id: Uuid,

    /// Canonical person this mention refers to
    pub person_id: Uuid,

    // --- Extracted Name (as it appears in source) ---
    #[validate(length(max = 500))]
    pub extracted_name_full: Option<String>,

    #[validate(length(max = 255))]
    pub extracted_name_given: Option<String>,

    #[validate(length(max = 255))]
    pub extracted_name_surname: Option<String>,

    // --- Context ---
    /// Role/occupation/description in this source
    #[validate(length(max = 100))]
    pub extracted_role: Option<String>,

    /// Page/line reference within source
    #[validate(length(max = 100))]
    pub page_reference: Option<String>,

    /// Notes about this mention
    pub notes: Option<String>,

    /// Confidence that extracted name refers to this person
    pub confidence: PersonConfidence,

    // --- Metadata ---
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SourcePerson {
    /// Create a new source-person link
    pub fn new(source_id: Uuid, person_id: Uuid) -> Self {
        Self {
            source_person_id: Uuid::now_v7(),
            source_id,
            person_id,
            extracted_name_full: None,
            extracted_name_given: None,
            extracted_name_surname: None,
            extracted_role: None,
            page_reference: None,
            notes: None,
            confidence: PersonConfidence::Possible,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Create a SourcePersonBuilder for fluent API
    pub fn builder() -> SourcePersonBuilder {
        SourcePersonBuilder::default()
    }

    /// Get extracted name or build from components
    pub fn get_extracted_name(&self) -> String {
        if let Some(ref full) = self.extracted_name_full {
            return full.clone();
        }

        let mut parts = Vec::new();
        if let Some(ref given) = self.extracted_name_given {
            parts.push(given.clone());
        }
        if let Some(ref surname) = self.extracted_name_surname {
            parts.push(surname.clone());
        }

        if parts.is_empty() {
            "Unknown".to_string()
        } else {
            parts.join(" ")
        }
    }

    /// Get display string with source context
    ///
    /// Example: "Jno Smith (farmer) - p. 15"
    pub fn display_with_context(&self) -> String {
        let mut parts = vec![self.get_extracted_name()];

        if let Some(ref role) = self.extracted_role {
            parts.push(format!("({})", role));
        }

        if let Some(ref page) = self.page_reference {
            parts.push(format!("- p. {}", page));
        }

        parts.join(" ")
    }

    /// Check if extracted name matches a search string (case-insensitive)
    pub fn matches_search(&self, search: &str) -> bool {
        let search_lower = search.to_lowercase();

        if let Some(ref full) = self.extracted_name_full {
            if full.to_lowercase().contains(&search_lower) {
                return true;
            }
        }

        if let Some(ref given) = self.extracted_name_given {
            if given.to_lowercase().contains(&search_lower) {
                return true;
            }
        }

        if let Some(ref surname) = self.extracted_name_surname {
            if surname.to_lowercase().contains(&search_lower) {
                return true;
            }
        }

        if let Some(ref role) = self.extracted_role {
            if role.to_lowercase().contains(&search_lower) {
                return true;
            }
        }

        false
    }
}

impl fmt::Display for SourcePerson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_extracted_name())
    }
}

// =============================================================================
// SOURCE-PERSON BUILDER
// =============================================================================

/// Builder for creating SourcePerson entities with fluent API
#[derive(Default)]
pub struct SourcePersonBuilder {
    source_id: Option<Uuid>,
    person_id: Option<Uuid>,
    extracted_name_full: Option<String>,
    extracted_name_given: Option<String>,
    extracted_name_surname: Option<String>,
    extracted_role: Option<String>,
    page_reference: Option<String>,
    notes: Option<String>,
    confidence: Option<PersonConfidence>,
}

impl SourcePersonBuilder {
    pub fn source_id(mut self, id: Uuid) -> Self {
        self.source_id = Some(id);
        self
    }

    pub fn person_id(mut self, id: Uuid) -> Self {
        self.person_id = Some(id);
        self
    }

    pub fn extracted_name_full(mut self, name: impl Into<String>) -> Self {
        self.extracted_name_full = Some(name.into());
        self
    }

    pub fn extracted_name_given(mut self, name: impl Into<String>) -> Self {
        self.extracted_name_given = Some(name.into());
        self
    }

    pub fn extracted_name_surname(mut self, name: impl Into<String>) -> Self {
        self.extracted_name_surname = Some(name.into());
        self
    }

    pub fn extracted_role(mut self, role: impl Into<String>) -> Self {
        self.extracted_role = Some(role.into());
        self
    }

    pub fn page_reference(mut self, page: impl Into<String>) -> Self {
        self.page_reference = Some(page.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn confidence(mut self, confidence: PersonConfidence) -> Self {
        self.confidence = Some(confidence);
        self
    }

    pub fn build(self) -> Result<SourcePerson, String> {
        let source_id = self.source_id.ok_or("source_id is required")?;
        let person_id = self.person_id.ok_or("person_id is required")?;

        let now = Utc::now();

        Ok(SourcePerson {
            source_person_id: Uuid::now_v7(),
            source_id,
            person_id,
            extracted_name_full: self.extracted_name_full,
            extracted_name_given: self.extracted_name_given,
            extracted_name_surname: self.extracted_name_surname,
            extracted_role: self.extracted_role,
            page_reference: self.page_reference,
            notes: self.notes,
            confidence: self.confidence.unwrap_or(PersonConfidence::Possible),
            created_at: now,
            updated_at: now,
        })
    }
}

// =============================================================================
// PERSON MERGE OPERATIONS
// =============================================================================

/// Person merge operation with full audit trail
///
/// Tracks when duplicate person records are merged together. This is a
/// production-grade implementation with:
/// - Complete audit trail (who, when, why)
/// - Reversible operations
/// - Soft-delete pattern (source archived, not deleted)
///
/// ## Merge Process
///
/// 1. **Identify Duplicates**: Two Person records refer to same individual
/// 2. **Choose Target**: Select which record to keep (usually more complete)
/// 3. **Create Merge**: Record the merge operation
/// 4. **Archive Source**: Source person marked as archived
/// 5. **Transfer Links**: All relationships, sources, variants point to target
///
/// ## Reversal
///
/// Merges can be reversed:
/// 1. Un-archive source person
/// 2. Restore original links (if needed)
/// 3. Record reversal reason and who did it
///
/// ## Example
///
/// ```text
/// Source: Person "John Smith" (1820-1891) - partial data
/// Target: Person "John William Smith" (1820-1891) - complete data
/// Reason: "Same person, census records match"
/// Result: Source archived, all links transferred to target
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PersonMerge {
    /// Unique identifier
    pub merge_id: Uuid,

    /// Source person (the one being merged away/archived)
    pub source_person_id: Uuid,

    /// Target person (the one being kept)
    pub target_person_id: Uuid,

    /// Reason for the merge
    #[validate(length(min = 1))]
    pub merge_reason: String,

    /// When the merge was performed
    pub merged_at: DateTime<Utc>,

    /// Who performed the merge
    pub merged_by: Uuid,

    // --- Reversal Information ---
    /// When the merge was reversed (if applicable)
    pub reversed_at: Option<DateTime<Utc>>,

    /// Who reversed the merge
    pub reversed_by: Option<Uuid>,

    /// Reason for reversal
    pub reversed_reason: Option<String>,
}

impl PersonMerge {
    /// Create a new person merge operation
    pub fn new(
        source_person_id: Uuid,
        target_person_id: Uuid,
        merge_reason: String,
        merged_by: Uuid,
    ) -> Result<Self, String> {
        if source_person_id == target_person_id {
            return Err("Cannot merge person with themselves".to_string());
        }

        if merge_reason.trim().is_empty() {
            return Err("Merge reason cannot be empty".to_string());
        }

        Ok(Self {
            merge_id: Uuid::now_v7(),
            source_person_id,
            target_person_id,
            merge_reason,
            merged_at: Utc::now(),
            merged_by,
            reversed_at: None,
            reversed_by: None,
            reversed_reason: None,
        })
    }

    /// Create a PersonMergeBuilder for fluent API
    pub fn builder() -> PersonMergeBuilder {
        PersonMergeBuilder::default()
    }

    /// Check if this merge has been reversed
    pub fn is_reversed(&self) -> bool {
        self.reversed_at.is_some()
    }

    /// Check if this merge is currently active (not reversed)
    pub fn is_active(&self) -> bool {
        !self.is_reversed()
    }

    /// Reverse this merge
    pub fn reverse(&mut self, reversed_by: Uuid, reason: String) -> Result<(), String> {
        if self.is_reversed() {
            return Err("Merge has already been reversed".to_string());
        }

        if reason.trim().is_empty() {
            return Err("Reversal reason cannot be empty".to_string());
        }

        self.reversed_at = Some(Utc::now());
        self.reversed_by = Some(reversed_by);
        self.reversed_reason = Some(reason);

        Ok(())
    }

    /// Get display string
    ///
    /// Example: "Merged 2025-11-09 (Same person, census records match)"
    pub fn display(&self) -> String {
        let date = self.merged_at.format("%Y-%m-%d");
        if self.is_reversed() {
            format!("Merged {} (REVERSED)", date)
        } else {
            format!("Merged {} ({})", date, self.merge_reason)
        }
    }

    /// Get duration since merge (in days)
    pub fn days_since_merge(&self) -> i64 {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.merged_at);
        duration.num_days()
    }

    /// Get duration merge was active (in days, if reversed)
    pub fn days_active(&self) -> Option<i64> {
        self.reversed_at.map(|reversed| {
            let duration = reversed.signed_duration_since(self.merged_at);
            duration.num_days()
        })
    }
}

impl fmt::Display for PersonMerge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

// =============================================================================
// PERSON MERGE BUILDER
// =============================================================================

/// Builder for creating PersonMerge entities with fluent API
#[derive(Default)]
pub struct PersonMergeBuilder {
    source_person_id: Option<Uuid>,
    target_person_id: Option<Uuid>,
    merge_reason: Option<String>,
    merged_by: Option<Uuid>,
}

impl PersonMergeBuilder {
    pub fn source_person_id(mut self, id: Uuid) -> Self {
        self.source_person_id = Some(id);
        self
    }

    pub fn target_person_id(mut self, id: Uuid) -> Self {
        self.target_person_id = Some(id);
        self
    }

    pub fn merge_reason(mut self, reason: impl Into<String>) -> Self {
        self.merge_reason = Some(reason.into());
        self
    }

    pub fn merged_by(mut self, user_id: Uuid) -> Self {
        self.merged_by = Some(user_id);
        self
    }

    pub fn build(self) -> Result<PersonMerge, String> {
        let source_person_id = self.source_person_id.ok_or("source_person_id is required")?;
        let target_person_id = self.target_person_id.ok_or("target_person_id is required")?;
        let merge_reason = self.merge_reason.ok_or("merge_reason is required")?;
        let merged_by = self.merged_by.ok_or("merged_by is required")?;

        if source_person_id == target_person_id {
            return Err("Cannot merge person with themselves".to_string());
        }

        if merge_reason.trim().is_empty() {
            return Err("Merge reason cannot be empty".to_string());
        }

        Ok(PersonMerge {
            merge_id: Uuid::now_v7(),
            source_person_id,
            target_person_id,
            merge_reason,
            merged_at: Utc::now(),
            merged_by,
            reversed_at: None,
            reversed_by: None,
            reversed_reason: None,
        })
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

    // --- VariantName Tests ---

    #[test]
    fn test_variant_name_creation() {
        let person_id = Uuid::now_v7();
        let variant = VariantName::new(
            person_id,
            Some("Mary".to_string()),
            Some("Johnson".to_string()),
            VariantNameType::Birth,
        ).unwrap();

        assert_eq!(variant.person_id, person_id);
        assert_eq!(variant.given_name, Some("Mary".to_string()));
        assert_eq!(variant.surname, Some("Johnson".to_string()));
        assert_eq!(variant.full_name, "Mary Johnson");
        assert_eq!(variant.variant_type, VariantNameType::Birth);
    }

    #[test]
    fn test_variant_name_builder() {
        let person_id = Uuid::now_v7();
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Smith")
            .middle_name("Elizabeth")
            .variant_type(VariantNameType::Married)
            .use_from_year(1845)
            .use_to_year(1891)
            .notes("After marriage to John Smith")
            .build()
            .unwrap();

        assert_eq!(variant.full_name, "Mary Elizabeth Smith");
        assert_eq!(variant.variant_type, VariantNameType::Married);
        assert_eq!(variant.use_from_year, Some(1845));
        assert_eq!(variant.use_to_year, Some(1891));
    }

    #[test]
    fn test_variant_name_full_components() {
        let person_id = Uuid::now_v7();
        let variant = VariantName::builder(person_id)
            .name_prefix("Dr.")
            .given_name("Mary")
            .middle_name("Elizabeth")
            .surname("Smith")
            .name_suffix("Jr.")
            .variant_type(VariantNameType::Legal)
            .build()
            .unwrap();

        assert_eq!(variant.full_name, "Dr. Mary Elizabeth Smith Jr.");
    }

    #[test]
    fn test_variant_name_display_with_type() {
        let person_id = Uuid::now_v7();
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Johnson")
            .variant_type(VariantNameType::Birth)
            .build()
            .unwrap();

        assert_eq!(variant.display_name_with_type(), "Mary Johnson (Birth Name)");
    }

    #[test]
    fn test_variant_name_time_period() {
        let person_id = Uuid::now_v7();

        // Both years
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Smith")
            .variant_type(VariantNameType::Married)
            .use_from_year(1845)
            .use_to_year(1891)
            .build()
            .unwrap();
        assert_eq!(variant.time_period(), Some("1845-1891".to_string()));

        // From year only
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Smith")
            .variant_type(VariantNameType::Married)
            .use_from_year(1845)
            .build()
            .unwrap();
        assert_eq!(variant.time_period(), Some("1845-".to_string()));

        // To year only
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Smith")
            .variant_type(VariantNameType::Married)
            .use_to_year(1891)
            .build()
            .unwrap();
        assert_eq!(variant.time_period(), Some("-1891".to_string()));

        // No years
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Smith")
            .variant_type(VariantNameType::Married)
            .build()
            .unwrap();
        assert_eq!(variant.time_period(), None);
    }

    #[test]
    fn test_variant_name_matches_search() {
        let person_id = Uuid::now_v7();
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Johnson")
            .variant_type(VariantNameType::Birth)
            .build()
            .unwrap();

        // Full name match
        assert!(variant.matches("Mary Johnson"));
        assert!(variant.matches("mary johnson")); // case-insensitive

        // Partial matches
        assert!(variant.matches("Mary"));
        assert!(variant.matches("Johnson"));
        assert!(variant.matches("ary")); // substring

        // No match
        assert!(!variant.matches("Smith"));
    }

    #[test]
    fn test_variant_name_matches_variant() {
        let person_id = Uuid::now_v7();

        let variant1 = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Johnson")
            .variant_type(VariantNameType::Birth)
            .build()
            .unwrap();

        // Exact match (different type)
        let variant2 = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Johnson")
            .variant_type(VariantNameType::Spelling)
            .build()
            .unwrap();
        assert!(variant1.matches_variant(&variant2));

        // Case-insensitive match
        let variant3 = VariantName::builder(person_id)
            .given_name("mary")
            .surname("johnson")
            .variant_type(VariantNameType::Birth)
            .build()
            .unwrap();
        assert!(variant1.matches_variant(&variant3));

        // Different name
        let variant4 = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Smith")
            .variant_type(VariantNameType::Married)
            .build()
            .unwrap();
        assert!(!variant1.matches_variant(&variant4));
    }

    #[test]
    fn test_variant_name_rebuild_full_name() {
        let person_id = Uuid::now_v7();
        let mut variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Johnson")
            .variant_type(VariantNameType::Birth)
            .build()
            .unwrap();

        assert_eq!(variant.full_name, "Mary Johnson");

        // Modify components
        variant.middle_name = Some("Elizabeth".to_string());
        variant.rebuild_full_name();

        assert_eq!(variant.full_name, "Mary Elizabeth Johnson");
    }

    #[test]
    fn test_variant_name_must_have_name() {
        let person_id = Uuid::now_v7();
        let result = VariantName::builder(person_id)
            .variant_type(VariantNameType::Birth)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Variant name must have at least given name or surname");
    }

    #[test]
    fn test_variant_name_types_display() {
        assert_eq!(format!("{}", VariantNameType::Birth), "Birth Name");
        assert_eq!(format!("{}", VariantNameType::Married), "Married Name");
        assert_eq!(format!("{}", VariantNameType::Immigration), "Immigration Name");
        assert_eq!(format!("{}", VariantNameType::Nickname), "Nickname");
        assert_eq!(format!("{}", VariantNameType::Spelling), "Spelling Variation");
    }

    #[test]
    fn test_variant_name_surname_only() {
        let person_id = Uuid::now_v7();
        let variant = VariantName::builder(person_id)
            .surname("Smith")
            .variant_type(VariantNameType::Documented)
            .build()
            .unwrap();

        assert_eq!(variant.full_name, "Smith");
        assert_eq!(variant.given_name, None);
        assert_eq!(variant.surname, Some("Smith".to_string()));
    }

    #[test]
    fn test_variant_name_given_only() {
        let person_id = Uuid::now_v7();
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .variant_type(VariantNameType::Nickname)
            .build()
            .unwrap();

        assert_eq!(variant.full_name, "Mary");
        assert_eq!(variant.given_name, Some("Mary".to_string()));
        assert_eq!(variant.surname, None);
    }

    // --- PersonRelationship Tests ---

    #[test]
    fn test_relationship_creation() {
        let person1 = Uuid::now_v7();
        let person2 = Uuid::now_v7();

        let rel = PersonRelationship::new(person1, person2, PersonRelationshipType::Parent)
            .unwrap();

        assert_eq!(rel.person_id, person1);
        assert_eq!(rel.related_person_id, person2);
        assert_eq!(rel.relationship_type, PersonRelationshipType::Parent);
        assert_eq!(rel.confidence, PersonConfidence::Possible);
    }

    #[test]
    fn test_relationship_builder() {
        let person1 = Uuid::now_v7();
        let person2 = Uuid::now_v7();

        let rel = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person2)
            .relationship_type(PersonRelationshipType::Spouse)
            .start_date(GenealogyDate::exact(1845, Some(6), Some(15)))
            .end_date(GenealogyDate::exact(1891, Some(12), Some(10)))
            .confidence(PersonConfidence::Definite)
            .notes("Married in church")
            .build()
            .unwrap();

        assert_eq!(rel.relationship_type, PersonRelationshipType::Spouse);
        assert_eq!(rel.confidence, PersonConfidence::Definite);
        assert!(rel.start_date.is_some());
        assert!(rel.end_date.is_some());
    }

    #[test]
    fn test_relationship_reciprocal() {
        let person1 = Uuid::now_v7();
        let person2 = Uuid::now_v7();

        let parent_rel = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person2)
            .relationship_type(PersonRelationshipType::Parent)
            .build()
            .unwrap();

        let child_rel = parent_rel.reciprocal();

        assert_eq!(child_rel.person_id, person2);
        assert_eq!(child_rel.related_person_id, person1);
        assert_eq!(child_rel.relationship_type, PersonRelationshipType::Child);
    }

    #[test]
    fn test_relationship_type_reciprocal() {
        assert_eq!(
            PersonRelationshipType::Parent.reciprocal(),
            PersonRelationshipType::Child
        );
        assert_eq!(
            PersonRelationshipType::Child.reciprocal(),
            PersonRelationshipType::Parent
        );
        assert_eq!(
            PersonRelationshipType::Spouse.reciprocal(),
            PersonRelationshipType::Spouse
        );
        assert_eq!(
            PersonRelationshipType::Grandparent.reciprocal(),
            PersonRelationshipType::Grandchild
        );
        assert_eq!(
            PersonRelationshipType::StepParent.reciprocal(),
            PersonRelationshipType::StepChild
        );
    }

    #[test]
    fn test_relationship_type_reflexive() {
        assert!(PersonRelationshipType::Spouse.is_reflexive());
        assert!(PersonRelationshipType::Sibling.is_reflexive());
        assert!(PersonRelationshipType::Cousin.is_reflexive());

        assert!(!PersonRelationshipType::Parent.is_reflexive());
        assert!(!PersonRelationshipType::Child.is_reflexive());
    }

    #[test]
    fn test_relationship_type_biological() {
        assert!(PersonRelationshipType::Parent.is_biological());
        assert!(PersonRelationshipType::Child.is_biological());
        assert!(PersonRelationshipType::Sibling.is_biological());
        assert!(PersonRelationshipType::Grandparent.is_biological());

        assert!(!PersonRelationshipType::AdoptiveParent.is_biological());
        assert!(!PersonRelationshipType::StepParent.is_biological());
        assert!(!PersonRelationshipType::FosterParent.is_biological());
    }

    #[test]
    fn test_relationship_type_legal() {
        assert!(PersonRelationshipType::AdoptiveParent.is_legal());
        assert!(PersonRelationshipType::AdoptiveChild.is_legal());
        assert!(PersonRelationshipType::Guardian.is_legal());
        assert!(PersonRelationshipType::Ward.is_legal());

        assert!(!PersonRelationshipType::Parent.is_legal());
        assert!(!PersonRelationshipType::StepParent.is_legal());
    }

    #[test]
    fn test_relationship_type_step() {
        assert!(PersonRelationshipType::StepParent.is_step());
        assert!(PersonRelationshipType::StepChild.is_step());

        assert!(!PersonRelationshipType::Parent.is_step());
        assert!(!PersonRelationshipType::AdoptiveParent.is_step());
    }

    #[test]
    fn test_relationship_type_foster() {
        assert!(PersonRelationshipType::FosterParent.is_foster());
        assert!(PersonRelationshipType::FosterChild.is_foster());

        assert!(!PersonRelationshipType::Parent.is_foster());
        assert!(!PersonRelationshipType::AdoptiveParent.is_foster());
    }

    #[test]
    fn test_relationship_display() {
        let person1 = Uuid::now_v7();
        let person2 = Uuid::now_v7();

        // No dates
        let rel = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person2)
            .relationship_type(PersonRelationshipType::Parent)
            .build()
            .unwrap();
        assert_eq!(rel.display(), "Parent");

        // Both dates
        let rel = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person2)
            .relationship_type(PersonRelationshipType::Spouse)
            .start_date(GenealogyDate::exact(1845, None, None))
            .end_date(GenealogyDate::exact(1891, None, None))
            .build()
            .unwrap();
        assert_eq!(rel.display(), "Spouse (1845 - 1891)");

        // Start date only
        let rel = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person2)
            .relationship_type(PersonRelationshipType::Spouse)
            .start_date(GenealogyDate::exact(1845, None, None))
            .build()
            .unwrap();
        assert_eq!(rel.display(), "Spouse (1845 - )");
    }

    #[test]
    fn test_relationship_is_active() {
        let person1 = Uuid::now_v7();
        let person2 = Uuid::now_v7();

        // No end date = active
        let rel = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person2)
            .relationship_type(PersonRelationshipType::Spouse)
            .start_date(GenealogyDate::exact(1845, None, None))
            .build()
            .unwrap();
        assert!(rel.is_active());

        // With end date = not active
        let rel = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person2)
            .relationship_type(PersonRelationshipType::Spouse)
            .start_date(GenealogyDate::exact(1845, None, None))
            .end_date(GenealogyDate::exact(1891, None, None))
            .build()
            .unwrap();
        assert!(!rel.is_active());
    }

    #[test]
    fn test_relationship_duration_years() {
        let person1 = Uuid::now_v7();
        let person2 = Uuid::now_v7();

        let rel = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person2)
            .relationship_type(PersonRelationshipType::Spouse)
            .start_date(GenealogyDate::exact(1845, None, None))
            .end_date(GenealogyDate::exact(1891, None, None))
            .build()
            .unwrap();

        assert_eq!(rel.duration_years(), Some(46));
    }

    #[test]
    fn test_relationship_no_self_relationship() {
        let person1 = Uuid::now_v7();

        let result = PersonRelationship::new(person1, person1, PersonRelationshipType::Sibling);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Person cannot have relationship with themselves"
        );

        let result = PersonRelationship::builder()
            .person_id(person1)
            .related_person_id(person1)
            .relationship_type(PersonRelationshipType::Parent)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_relationship_type_display() {
        assert_eq!(format!("{}", PersonRelationshipType::Parent), "Parent");
        assert_eq!(format!("{}", PersonRelationshipType::Spouse), "Spouse");
        assert_eq!(format!("{}", PersonRelationshipType::AuntUncle), "Aunt/Uncle");
        assert_eq!(
            format!("{}", PersonRelationshipType::AdoptiveParent),
            "Adoptive Parent"
        );
    }

    // --- SourcePerson Tests ---

    #[test]
    fn test_source_person_creation() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        let sp = SourcePerson::new(source_id, person_id);

        assert_eq!(sp.source_id, source_id);
        assert_eq!(sp.person_id, person_id);
        assert_eq!(sp.confidence, PersonConfidence::Possible);
    }

    #[test]
    fn test_source_person_builder() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_full("Jno Smith")
            .extracted_role("farmer")
            .page_reference("15")
            .confidence(PersonConfidence::Probable)
            .notes("Listed as head of household")
            .build()
            .unwrap();

        assert_eq!(sp.extracted_name_full, Some("Jno Smith".to_string()));
        assert_eq!(sp.extracted_role, Some("farmer".to_string()));
        assert_eq!(sp.page_reference, Some("15".to_string()));
        assert_eq!(sp.confidence, PersonConfidence::Probable);
    }

    #[test]
    fn test_source_person_extracted_name_full() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_full("Jno Smith")
            .build()
            .unwrap();

        assert_eq!(sp.get_extracted_name(), "Jno Smith");
    }

    #[test]
    fn test_source_person_extracted_name_components() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_given("Jno")
            .extracted_name_surname("Smith")
            .build()
            .unwrap();

        assert_eq!(sp.get_extracted_name(), "Jno Smith");
    }

    #[test]
    fn test_source_person_display_with_context() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        // Full context
        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_full("Jno Smith")
            .extracted_role("farmer")
            .page_reference("15")
            .build()
            .unwrap();

        assert_eq!(sp.display_with_context(), "Jno Smith (farmer) - p. 15");

        // Name and role only
        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_full("Jno Smith")
            .extracted_role("farmer")
            .build()
            .unwrap();

        assert_eq!(sp.display_with_context(), "Jno Smith (farmer)");

        // Name only
        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_full("Jno Smith")
            .build()
            .unwrap();

        assert_eq!(sp.display_with_context(), "Jno Smith");
    }

    #[test]
    fn test_source_person_matches_search() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_full("Jno Smith")
            .extracted_role("farmer")
            .build()
            .unwrap();

        // Match on name
        assert!(sp.matches_search("Jno"));
        assert!(sp.matches_search("Smith"));
        assert!(sp.matches_search("jno smith")); // case-insensitive

        // Match on role
        assert!(sp.matches_search("farmer"));

        // No match
        assert!(!sp.matches_search("Johnson"));
    }

    #[test]
    fn test_source_person_matches_search_components() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_given("Jno")
            .extracted_name_surname("Smith")
            .build()
            .unwrap();

        assert!(sp.matches_search("Jno"));
        assert!(sp.matches_search("Smith"));
        assert!(!sp.matches_search("Johnson"));
    }

    #[test]
    fn test_source_person_display() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .extracted_name_full("Jno Smith")
            .build()
            .unwrap();

        assert_eq!(format!("{}", sp), "Jno Smith");
    }

    #[test]
    fn test_source_person_empty_name() {
        let source_id = Uuid::now_v7();
        let person_id = Uuid::now_v7();

        let sp = SourcePerson::builder()
            .source_id(source_id)
            .person_id(person_id)
            .build()
            .unwrap();

        assert_eq!(sp.get_extracted_name(), "Unknown");
    }

    // --- PersonMerge Tests ---

    #[test]
    fn test_person_merge_creation() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let merge = PersonMerge::new(
            source_id,
            target_id,
            "Same person, census records match".to_string(),
            user_id,
        )
        .unwrap();

        assert_eq!(merge.source_person_id, source_id);
        assert_eq!(merge.target_person_id, target_id);
        assert_eq!(merge.merge_reason, "Same person, census records match");
        assert_eq!(merge.merged_by, user_id);
        assert!(!merge.is_reversed());
        assert!(merge.is_active());
    }

    #[test]
    fn test_person_merge_builder() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let merge = PersonMerge::builder()
            .source_person_id(source_id)
            .target_person_id(target_id)
            .merge_reason("Duplicate detected")
            .merged_by(user_id)
            .build()
            .unwrap();

        assert_eq!(merge.source_person_id, source_id);
        assert_eq!(merge.target_person_id, target_id);
        assert_eq!(merge.merge_reason, "Duplicate detected");
    }

    #[test]
    fn test_person_merge_reverse() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let mut merge = PersonMerge::new(
            source_id,
            target_id,
            "Same person".to_string(),
            user_id,
        )
        .unwrap();

        assert!(merge.is_active());
        assert!(!merge.is_reversed());

        // Reverse the merge
        merge
            .reverse(user_id, "Actually different people".to_string())
            .unwrap();

        assert!(!merge.is_active());
        assert!(merge.is_reversed());
        assert_eq!(
            merge.reversed_reason,
            Some("Actually different people".to_string())
        );
        assert_eq!(merge.reversed_by, Some(user_id));
    }

    #[test]
    fn test_person_merge_cannot_reverse_twice() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let mut merge = PersonMerge::new(
            source_id,
            target_id,
            "Same person".to_string(),
            user_id,
        )
        .unwrap();

        // First reversal succeeds
        merge
            .reverse(user_id, "Different people".to_string())
            .unwrap();

        // Second reversal fails
        let result = merge.reverse(user_id, "Another reason".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Merge has already been reversed");
    }

    #[test]
    fn test_person_merge_no_self_merge() {
        let person_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let result = PersonMerge::new(
            person_id,
            person_id,
            "Trying to merge with self".to_string(),
            user_id,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot merge person with themselves");

        let result = PersonMerge::builder()
            .source_person_id(person_id)
            .target_person_id(person_id)
            .merge_reason("Test")
            .merged_by(user_id)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_person_merge_empty_reason() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let result = PersonMerge::new(source_id, target_id, "".to_string(), user_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Merge reason cannot be empty");

        let result = PersonMerge::new(source_id, target_id, "   ".to_string(), user_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_person_merge_empty_reversal_reason() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let mut merge = PersonMerge::new(
            source_id,
            target_id,
            "Same person".to_string(),
            user_id,
        )
        .unwrap();

        let result = merge.reverse(user_id, "".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Reversal reason cannot be empty");

        let result = merge.reverse(user_id, "   ".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_person_merge_display() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let merge = PersonMerge::new(
            source_id,
            target_id,
            "Census records match".to_string(),
            user_id,
        )
        .unwrap();

        let display = merge.display();
        assert!(display.contains("Merged"));
        assert!(display.contains("Census records match"));
    }

    #[test]
    fn test_person_merge_display_reversed() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let mut merge = PersonMerge::new(
            source_id,
            target_id,
            "Same person".to_string(),
            user_id,
        )
        .unwrap();

        merge.reverse(user_id, "Different people".to_string()).unwrap();

        let display = merge.display();
        assert!(display.contains("REVERSED"));
    }

    #[test]
    fn test_person_merge_days_since_merge() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let merge = PersonMerge::new(
            source_id,
            target_id,
            "Same person".to_string(),
            user_id,
        )
        .unwrap();

        // Should be 0 days since just created
        assert_eq!(merge.days_since_merge(), 0);
    }

    #[test]
    fn test_person_merge_days_active() {
        let source_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let mut merge = PersonMerge::new(
            source_id,
            target_id,
            "Same person".to_string(),
            user_id,
        )
        .unwrap();

        // Not reversed yet
        assert_eq!(merge.days_active(), None);

        // Reverse it
        merge.reverse(user_id, "Different people".to_string()).unwrap();

        // Now it should return Some(0) since reversed immediately
        assert_eq!(merge.days_active(), Some(0));
    }
}
