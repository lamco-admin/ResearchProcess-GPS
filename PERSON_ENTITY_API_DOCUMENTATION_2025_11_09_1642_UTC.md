# Person Entity API Documentation
## ResearchProcess-GPS Authority Control System

**Document Version**: 1.0
**Timestamp**: 2025-11-09 16:42:15 UTC
**Status**: Phase 1 Week 5 Complete - Domain Types
**Related**: RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md

---

## Table of Contents

1. [Overview](#overview)
2. [Core Person Entity](#core-person-entity)
3. [VariantName - Name Variants](#variantname---name-variants)
4. [PersonRelationship - Family Relationships](#personrelationship---family-relationships)
5. [SourcePerson - Source Linking](#sourceperson---source-linking)
6. [PersonMerge - Duplicate Handling](#personmerge---duplicate-handling)
7. [Supporting Types](#supporting-types)
8. [Usage Patterns](#usage-patterns)
9. [Integration Guidelines](#integration-guidelines)
10. [Error Handling](#error-handling)

---

## Overview

The Person entity system implements **authority control** - a proven library science pattern from Koha ILS (25+ years in production). This ensures:

- **One canonical record per individual** (no duplicates)
- **All name variants systematically tracked** (MARC 4XX pattern)
- **All relationships documented** (MARC 5XX pattern)
- **Complete source attribution** (MARC $9 pattern)
- **Production-grade merge operations** (reversible, audited)

### Architecture Pattern

```
IdentityPersona (Research Workflow)
         ↓
    Concluded State
         ↓
Person (Canonical Authority Record) ←─── The definitive entity
    ├── VariantName[]          (4XX - all name forms)
    ├── PersonRelationship[]   (5XX - family connections)
    ├── SourcePerson[]         ($9 - source citations)
    └── PersonMerge[]          (audit trail)
```

### Module Location

```rust
use rp_core::prelude::{
    Person, PersonBuilder,
    VariantName, VariantNameBuilder, VariantNameType,
    PersonRelationship, PersonRelationshipBuilder, PersonRelationshipType,
    SourcePerson, SourcePersonBuilder,
    PersonMerge, PersonMergeBuilder,
    Sex, PersonConfidence, GenealogyDate, DateCertainty,
};
```

---

## Core Person Entity

### Overview

`Person` represents the canonical authority record for an individual. This is THE definitive record - one person = one Person entity.

### Type Definition

```rust
pub struct Person {
    // Identity
    pub person_id: Uuid,

    // Canonical Name (MARC 1XX - established heading)
    pub given_name: Option<String>,      // Max 255 chars
    pub surname: Option<String>,         // Max 255 chars
    pub middle_name: Option<String>,     // Max 255 chars
    pub name_prefix: Option<String>,     // Max 50 chars (Dr., Rev., etc.)
    pub name_suffix: Option<String>,     // Max 50 chars (Jr., III, etc.)

    // Vital Events
    pub birth_date: Option<GenealogyDate>,
    pub birth_place_id: Option<Uuid>,
    pub birth_source_id: Option<Uuid>,
    pub death_date: Option<GenealogyDate>,
    pub death_place_id: Option<Uuid>,
    pub death_source_id: Option<Uuid>,

    // Biographical
    pub sex: Sex,                        // Male, Female, Unknown
    pub occupation: Option<String>,      // Max 500 chars
    pub religion: Option<String>,        // Max 100 chars

    // Research Notes
    pub notes: Option<String>,
    pub research_notes: Option<String>,
    pub conclusion_notes: Option<String>,
    pub conclusion_confidence: Option<PersonConfidence>,

    // MARC Export Cache (for library system interoperability)
    pub marc_binary: Option<Vec<u8>>,
    pub marc_xml: Option<String>,
    pub marc_updated_at: Option<DateTime<Utc>>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub last_modified_by: Uuid,

    // Soft Delete
    pub archived: bool,
    pub archived_reason: Option<String>,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
}
```

### Creation API

#### Constructor

```rust
// Minimal creation (requires at least given name OR surname)
let person = Person::new(
    Some("John".to_string()),
    Some("Smith".to_string()),
    user_id,
)?;
```

#### Builder Pattern (Recommended)

```rust
use uuid::Uuid;
use rp_core::prelude::*;

let user_id = Uuid::now_v7();

let person = Person::builder()
    .given_name("John")
    .surname("Smith")
    .middle_name("William")
    .name_suffix("Jr.")
    .birth_date(GenealogyDate::exact(1820, Some(3), Some(15)))
    .death_date(GenealogyDate::exact(1891, Some(12), Some(10)))
    .sex(Sex::Male)
    .occupation("Farmer")
    .conclusion_confidence(PersonConfidence::Definite)
    .notes("Primary ancestor of Smith family")
    .created_by(user_id)
    .build()?;
```

### Display Methods

#### Canonical Name

```rust
// Standard format: [Prefix] [Given] [Middle] [Surname] [Suffix]
let name = person.canonical_name();
// => "John William Smith Jr."

// Surname-first format (for indexing/sorting)
let indexed = person.surname_first();
// => "Smith, John William Jr."

// Display trait (includes life span)
let display = format!("{}", person);
// => "John William Smith Jr. (1820 - 1891)"
```

#### Life Span

```rust
// Get formatted life span
let span = person.life_span();
// => Some("1820 - 1891")

// With circa dates
let span_circa = person_with_circa.life_span();
// => Some("ca. 1820 - 1891")

// Calculate age at death
let age = person.age_at_death();
// => Some(71)
```

### Status Methods

```rust
// Check if active (not archived/merged)
if person.is_active() {
    // Person is available for use
}

// Check if archived (soft deleted/merged away)
if person.is_archived() {
    // This person was merged into another
}
```

### Archive Operations

```rust
// Archive a person (soft delete)
person.archive(
    "Merged into person ABC123".to_string(),
    user_id,
);

// Restore an archived person
person.unarchive(user_id);
```

### MARC Cache Management

```rust
// Check if MARC cache is valid
if !person.has_valid_marc_cache() {
    // Generate MARC export...
}

// Invalidate cache after updates
person.invalidate_marc_cache();
```

### Validation

```rust
use validator::Validate;

// Validate all constraints
person.validate()?;

// Constraints:
// - Must have given_name OR surname (at least one)
// - given_name: 1-255 chars (if present)
// - surname: 1-255 chars (if present)
// - middle_name: max 255 chars
// - name_prefix: max 50 chars
// - name_suffix: max 50 chars
// - occupation: max 500 chars
// - religion: max 100 chars
```

---

## VariantName - Name Variants

### Overview

`VariantName` tracks all alternate name forms a person used throughout their life. This implements the MARC 4XX "See From Tracing" pattern.

### Use Cases

- **Birth name** → Married name (Mary Johnson → Mary Smith)
- **Immigration names** (Johann Schmidt → John Smith)
- **Spelling variations** (Smyth vs Smith)
- **Nicknames** (Jack for John)
- **Professional names** (stage names, pseudonyms)

### Type Definition

```rust
pub struct VariantName {
    pub variant_id: Uuid,
    pub person_id: Uuid,  // Links to canonical Person

    // Name Components
    pub given_name: Option<String>,      // Max 255 chars
    pub surname: Option<String>,         // Max 255 chars
    pub middle_name: Option<String>,     // Max 255 chars
    pub name_prefix: Option<String>,     // Max 50 chars
    pub name_suffix: Option<String>,     // Max 50 chars
    pub full_name: String,               // 1-500 chars (auto-built)

    pub variant_type: VariantNameType,

    // Time Period (when this name was used)
    pub use_from_year: Option<i32>,
    pub use_to_year: Option<i32>,

    pub notes: Option<String>,
    pub source_id: Option<Uuid>,         // Source documenting this variant

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### VariantNameType Enum

```rust
pub enum VariantNameType {
    Birth,          // Name at birth (maiden name)
    Married,        // Name after marriage
    Divorced,       // Name after divorce
    Nickname,       // Informal name, pet name
    Immigration,    // Name changed upon immigration
    Spelling,       // Spelling variation
    Translation,    // Translation to another language
    Abbreviation,   // Shortened form (Wm. for William)
    Pseudonym,      // Pen name, stage name, alias
    Legal,          // Legally changed name
    Religious,      // Name upon taking religious orders
    Stage,          // Professional/stage name
    Documented,     // Name found in specific document
}
```

### Creation API

#### Constructor

```rust
let variant = VariantName::new(
    person_id,
    Some("Mary".to_string()),
    Some("Johnson".to_string()),
    VariantNameType::Birth,
)?;
```

#### Builder Pattern (Recommended)

```rust
let variant = VariantName::builder(person_id)
    .given_name("Mary")
    .surname("Smith")
    .middle_name("Elizabeth")
    .variant_type(VariantNameType::Married)
    .use_from_year(1845)
    .use_to_year(1891)
    .notes("After marriage to John Smith")
    .source_id(marriage_source_id)
    .build()?;
```

### Display Methods

```rust
// Get full name
let name = variant.to_string();
// => "Mary Elizabeth Smith"

// Display with variant type
let display = variant.display_name_with_type();
// => "Mary Elizabeth Smith (Married Name)"

// Get time period
let period = variant.time_period();
// => Some("1845-1891")
```

### Search Methods

```rust
// Check if variant matches search string (case-insensitive)
if variant.matches("mary") {
    // Found!
}

// Check if two variants match (duplicate detection)
if variant1.matches_variant(&variant2) {
    // Same name, possibly duplicate
}
```

### Update Operations

```rust
// Modify components
variant.middle_name = Some("Elizabeth".to_string());

// Rebuild full name after changes
variant.rebuild_full_name();
```

### Complete Example

```rust
use rp_core::prelude::*;

// Create a person
let person = Person::builder()
    .given_name("Mary")
    .surname("Smith")
    .created_by(user_id)
    .build()?;

// Add birth name variant
let birth_name = VariantName::builder(person.person_id)
    .given_name("Mary")
    .surname("Johnson")
    .middle_name("Elizabeth")
    .variant_type(VariantNameType::Birth)
    .use_from_year(1820)
    .use_to_year(1845)
    .notes("Maiden name before marriage")
    .build()?;

// Add married name variant
let married_name = VariantName::builder(person.person_id)
    .given_name("Mary")
    .surname("Smith")
    .middle_name("Elizabeth")
    .variant_type(VariantNameType::Married)
    .use_from_year(1845)
    .notes("After marriage to John Smith")
    .build()?;

// Search across all variants
let variants = vec![birth_name, married_name];
let matches: Vec<_> = variants.iter()
    .filter(|v| v.matches("Johnson"))
    .collect();
```

---

## PersonRelationship - Family Relationships

### Overview

`PersonRelationship` tracks typed family relationships between persons. This implements the MARC 5XX "See Also From Tracing" pattern.

### Type Definition

```rust
pub struct PersonRelationship {
    pub relationship_id: Uuid,
    pub person_id: Uuid,          // Subject (from side)
    pub related_person_id: Uuid,  // Object (to side)
    pub relationship_type: PersonRelationshipType,

    // Time Period
    pub start_date: Option<GenealogyDate>,
    pub end_date: Option<GenealogyDate>,

    pub confidence: PersonConfidence,
    pub notes: Option<String>,
    pub source_id: Option<Uuid>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### PersonRelationshipType Enum

```rust
pub enum PersonRelationshipType {
    // Biological
    Parent,
    Child,
    Sibling,
    Grandparent,
    Grandchild,
    AuntUncle,
    NieceNephew,
    Cousin,

    // Marriage
    Spouse,

    // Step Family
    StepParent,
    StepChild,

    // Adoptive
    AdoptiveParent,
    AdoptiveChild,

    // Foster
    FosterParent,
    FosterChild,

    // Legal
    Guardian,
    Ward,
}
```

### Creation API

#### Constructor

```rust
let rel = PersonRelationship::new(
    parent_id,
    child_id,
    PersonRelationshipType::Parent,
)?;
```

#### Builder Pattern (Recommended)

```rust
let marriage = PersonRelationship::builder()
    .person_id(person1_id)
    .related_person_id(person2_id)
    .relationship_type(PersonRelationshipType::Spouse)
    .start_date(GenealogyDate::exact(1845, Some(6), Some(15)))
    .end_date(GenealogyDate::exact(1891, Some(12), Some(10)))
    .confidence(PersonConfidence::Definite)
    .notes("Married in St. Mary's Church")
    .source_id(marriage_record_id)
    .build()?;
```

### Relationship Type Methods

```rust
// Get reciprocal relationship type
let reciprocal = PersonRelationshipType::Parent.reciprocal();
// => PersonRelationshipType::Child

// Check if reflexive (same both ways)
PersonRelationshipType::Spouse.is_reflexive();  // => true
PersonRelationshipType::Parent.is_reflexive();  // => false

// Check relationship classification
PersonRelationshipType::Parent.is_biological();      // => true
PersonRelationshipType::AdoptiveParent.is_legal();  // => true
PersonRelationshipType::StepParent.is_step();       // => true
PersonRelationshipType::FosterParent.is_foster();   // => true
```

### Reciprocal Relationships

```rust
// Create parent-child relationship
let parent_rel = PersonRelationship::builder()
    .person_id(parent_id)
    .related_person_id(child_id)
    .relationship_type(PersonRelationshipType::Parent)
    .build()?;

// Generate reciprocal child-parent relationship automatically
let child_rel = parent_rel.reciprocal();

// Now have:
// parent_id -> Parent -> child_id
// child_id -> Child -> parent_id
```

### Display Methods

```rust
// Display with dates
let display = relationship.display();
// => "Spouse (1845 - 1891)"

// Check if active (no end date)
if relationship.is_active() {
    // Relationship ongoing
}

// Calculate duration
let years = relationship.duration_years();
// => Some(46)
```

### Complete Example

```rust
use rp_core::prelude::*;

// Create persons
let john = Person::builder()
    .given_name("John")
    .surname("Smith")
    .created_by(user_id)
    .build()?;

let mary = Person::builder()
    .given_name("Mary")
    .surname("Johnson")
    .created_by(user_id)
    .build()?;

// Create marriage relationship
let marriage = PersonRelationship::builder()
    .person_id(john.person_id)
    .related_person_id(mary.person_id)
    .relationship_type(PersonRelationshipType::Spouse)
    .start_date(GenealogyDate::exact(1845, Some(6), Some(15)))
    .confidence(PersonConfidence::Definite)
    .source_id(marriage_source_id)
    .notes("Married at St. Mary's Church, Boston")
    .build()?;

// Create reciprocal relationship
let marriage_reciprocal = marriage.reciprocal();

// Now both sides have the relationship recorded
```

---

## SourcePerson - Source Linking

### Overview

`SourcePerson` links source citations to canonical Person entities, preserving the exact name as it appears in the source. This implements the MARC $9 authority linking pattern.

### Type Definition

```rust
pub struct SourcePerson {
    pub source_person_id: Uuid,
    pub source_id: Uuid,    // Source citation
    pub person_id: Uuid,    // Canonical person

    // Extracted Name (as it appears in source)
    pub extracted_name_full: Option<String>,      // Max 500 chars
    pub extracted_name_given: Option<String>,     // Max 255 chars
    pub extracted_name_surname: Option<String>,   // Max 255 chars

    // Context from Source
    pub extracted_role: Option<String>,          // Max 100 chars
    pub page_reference: Option<String>,          // Max 100 chars
    pub notes: Option<String>,

    pub confidence: PersonConfidence,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Creation API

#### Constructor

```rust
let sp = SourcePerson::new(source_id, person_id);
```

#### Builder Pattern (Recommended)

```rust
let source_mention = SourcePerson::builder()
    .source_id(census_1850_id)
    .person_id(john_smith_person_id)
    .extracted_name_full("Jno Smith")  // Exactly as written
    .extracted_role("farmer")
    .page_reference("15")
    .confidence(PersonConfidence::Probable)
    .notes("Listed as head of household")
    .build()?;
```

### Display Methods

```rust
// Get extracted name
let name = source_person.get_extracted_name();
// => "Jno Smith"

// Display with context
let context = source_person.display_with_context();
// => "Jno Smith (farmer) - p. 15"
```

### Search Methods

```rust
// Search across all fields
if source_person.matches_search("farmer") {
    // Matches role
}

if source_person.matches_search("jno") {
    // Matches name (case-insensitive)
}
```

### Complete Example

```rust
use rp_core::prelude::*;

// Canonical person
let john = Person::builder()
    .given_name("John")
    .surname("Smith")
    .middle_name("William")
    .created_by(user_id)
    .build()?;

// 1850 Census mention (abbreviated name)
let census_1850 = SourcePerson::builder()
    .source_id(census_1850_source_id)
    .person_id(john.person_id)
    .extracted_name_full("Jno Smith")  // As written in census
    .extracted_role("farmer")
    .page_reference("15")
    .confidence(PersonConfidence::Probable)
    .build()?;

// 1870 Census mention (full name)
let census_1870 = SourcePerson::builder()
    .source_id(census_1870_source_id)
    .person_id(john.person_id)
    .extracted_name_given("John")
    .extracted_name_surname("Smith")
    .extracted_role("farmer")
    .page_reference("42")
    .confidence(PersonConfidence::Definite)
    .build()?;

// Now we have evidence trail:
// Person "John William Smith" is mentioned in:
// - 1850 Census as "Jno Smith" (farmer)
// - 1870 Census as "John Smith" (farmer)
```

---

## PersonMerge - Duplicate Handling

### Overview

`PersonMerge` tracks merge operations when duplicate person records are consolidated. This is production-grade with full audit trail and reversibility.

### Type Definition

```rust
pub struct PersonMerge {
    pub merge_id: Uuid,
    pub source_person_id: Uuid,   // Being merged away (archived)
    pub target_person_id: Uuid,   // Being kept

    pub merge_reason: String,      // Min 1 char (required)

    pub merged_at: DateTime<Utc>,
    pub merged_by: Uuid,

    // Reversal Information
    pub reversed_at: Option<DateTime<Utc>>,
    pub reversed_by: Option<Uuid>,
    pub reversed_reason: Option<String>,
}
```

### Creation API

#### Constructor

```rust
let merge = PersonMerge::new(
    source_person_id,
    target_person_id,
    "Same person, census records match".to_string(),
    user_id,
)?;
```

#### Builder Pattern (Recommended)

```rust
let merge = PersonMerge::builder()
    .source_person_id(duplicate_person_id)
    .target_person_id(canonical_person_id)
    .merge_reason("Duplicate detected: same birth date, parents, location")
    .merged_by(user_id)
    .build()?;
```

### Status Methods

```rust
// Check if merge is active (not reversed)
if merge.is_active() {
    // Merge in effect
}

// Check if reversed
if merge.is_reversed() {
    // Merge was undone
}
```

### Reversal Operations

```rust
// Reverse a merge
merge.reverse(
    user_id,
    "Actually different people - birth dates don't match".to_string(),
)?;

// Cannot reverse twice
let result = merge.reverse(user_id, "reason".to_string());
// => Err("Merge has already been reversed")
```

### Display and Metrics

```rust
// Display merge
let display = merge.display();
// => "Merged 2025-11-09 (Same person, census records match)"
// Or if reversed:
// => "Merged 2025-11-09 (REVERSED)"

// Get days since merge
let days = merge.days_since_merge();
// => 7

// Get days merge was active (if reversed)
if let Some(active_days) = merge.days_active() {
    println!("Merge was active for {} days", active_days);
}
```

### Complete Merge Workflow

```rust
use rp_core::prelude::*;

// Step 1: Identify duplicates
let person1 = Person::builder()
    .given_name("John")
    .surname("Smith")
    .birth_date(GenealogyDate::exact(1820, None, None))
    .created_by(user_id)
    .build()?;

let person2 = Person::builder()
    .given_name("John")
    .surname("Smith")
    .middle_name("William")  // More complete
    .birth_date(GenealogyDate::exact(1820, Some(3), Some(15)))
    .created_by(user_id)
    .build()?;

// Step 2: Choose target (person2 has more data)
// Step 3: Create merge operation
let merge = PersonMerge::builder()
    .source_person_id(person1.person_id)
    .target_person_id(person2.person_id)
    .merge_reason("Same person: birth year matches, location matches, family matches")
    .merged_by(user_id)
    .build()?;

// Step 4: Archive source person
person1.archive("Merged into person2".to_string(), user_id);

// Step 5: Transfer all relationships, sources, variants to person2
// (This would be done by storage layer)

// Later: Reverse if needed
if wrong_merge_detected {
    merge.reverse(user_id, "Different people - new evidence found".to_string())?;
    person1.unarchive(user_id);
    // Restore original links (storage layer)
}
```

### Validation

```rust
// Cannot merge person with themselves
let result = PersonMerge::new(person_id, person_id, "reason", user_id);
// => Err("Cannot merge person with themselves")

// Merge reason cannot be empty
let result = PersonMerge::new(person1_id, person2_id, "", user_id);
// => Err("Merge reason cannot be empty")

// Reversal reason cannot be empty
let result = merge.reverse(user_id, "");
// => Err("Reversal reason cannot be empty")
```

---

## Supporting Types

### Sex Enum

```rust
pub enum Sex {
    Male,
    Female,
    Unknown,  // Default
}

// Display
format!("{}", Sex::Male);  // => "Male"
```

### PersonConfidence Enum

```rust
pub enum PersonConfidence {
    Speculative = 1,  // Weak hypothesis
    Uncertain = 2,    // Some evidence, inconclusive
    Possible = 3,     // Reasonable possibility
    Probable = 4,     // Strong evidence
    Definite = 5,     // Conclusive evidence
}

// Ordering
assert!(PersonConfidence::Definite > PersonConfidence::Probable);

// Display
format!("{}", PersonConfidence::Probable);  // => "Probable"
```

### GenealogyDate Struct

Handles date uncertainty common in genealogical research.

```rust
pub struct GenealogyDate {
    pub year: Option<i32>,
    pub month: Option<u8>,        // 1-12
    pub day: Option<u8>,          // 1-31
    pub certainty: DateCertainty,
    pub circa: bool,
    pub end_year: Option<i32>,    // For "between" ranges
    pub original_text: Option<String>,  // Max 200 chars
}
```

#### Creation Methods

```rust
// Exact date
let date = GenealogyDate::exact(1850, Some(3), Some(15));
// => "1850-03-15"

// Circa (approximate)
let date = GenealogyDate::circa(1850);
// => "ca. 1850"

// Estimated
let date = GenealogyDate::estimated(1850);
// => "est. 1850"

// Calculated
let date = GenealogyDate::calculated(1850);
// => "calc. 1850"

// Before
let date = GenealogyDate::before(1850);
// => "bef. 1850"

// After
let date = GenealogyDate::after(1850);
// => "aft. 1850"

// Between (range)
let date = GenealogyDate::between(1850, 1860);
// => "bet. 1850 - 1860"
```

#### Display Methods

```rust
// Display year with certainty
let display = date.display_year();

// Display full date (if available)
let full = date.display_full();
// => "1850-03-15" or "1850-03" or "1850"

// Get sortable year (for timeline queries)
if let Some(year) = date.sortable_year() {
    // Use for sorting
}
```

### DateCertainty Enum

```rust
pub enum DateCertainty {
    Exact,       // Known exact date
    Estimated,   // Estimated from context
    Calculated,  // Calculated from other information
    Before,      // Before this date
    After,       // After this date
    Between,     // Between two dates (range)
}
```

---

## Usage Patterns

### Pattern 1: Creating a Complete Person Record

```rust
use rp_core::prelude::*;
use uuid::Uuid;

fn create_complete_person(user_id: Uuid) -> Result<PersonRecord, String> {
    // 1. Create canonical person
    let person = Person::builder()
        .given_name("John")
        .surname("Smith")
        .middle_name("William")
        .name_suffix("Jr.")
        .birth_date(GenealogyDate::exact(1820, Some(3), Some(15)))
        .death_date(GenealogyDate::exact(1891, Some(12), Some(10)))
        .sex(Sex::Male)
        .occupation("Farmer")
        .conclusion_confidence(PersonConfidence::Definite)
        .created_by(user_id)
        .build()?;

    // 2. Add birth name variant
    let birth_name = VariantName::builder(person.person_id)
        .given_name("John")
        .surname("Schmidt")
        .variant_type(VariantNameType::Birth)
        .use_to_year(1820)
        .notes("Original German surname before immigration")
        .build()?;

    // 3. Add immigration name variant
    let immigration_name = VariantName::builder(person.person_id)
        .given_name("John")
        .surname("Smith")
        .variant_type(VariantNameType::Immigration)
        .use_from_year(1820)
        .notes("Anglicized upon arrival in America")
        .build()?;

    Ok(PersonRecord {
        person,
        variants: vec![birth_name, immigration_name],
        relationships: vec![],
        sources: vec![],
    })
}

struct PersonRecord {
    person: Person,
    variants: Vec<VariantName>,
    relationships: Vec<PersonRelationship>,
    sources: Vec<SourcePerson>,
}
```

### Pattern 2: Building Family Relationships

```rust
fn create_family(user_id: Uuid) -> Result<Family, String> {
    // Create parents
    let father = Person::builder()
        .given_name("John")
        .surname("Smith")
        .sex(Sex::Male)
        .created_by(user_id)
        .build()?;

    let mother = Person::builder()
        .given_name("Mary")
        .surname("Smith")
        .sex(Sex::Female)
        .created_by(user_id)
        .build()?;

    // Create child
    let child = Person::builder()
        .given_name("William")
        .surname("Smith")
        .sex(Sex::Male)
        .created_by(user_id)
        .build()?;

    // Marriage
    let marriage = PersonRelationship::builder()
        .person_id(father.person_id)
        .related_person_id(mother.person_id)
        .relationship_type(PersonRelationshipType::Spouse)
        .start_date(GenealogyDate::exact(1845, None, None))
        .confidence(PersonConfidence::Definite)
        .build()?;

    let marriage_reciprocal = marriage.reciprocal();

    // Parent-child relationships
    let father_child = PersonRelationship::builder()
        .person_id(father.person_id)
        .related_person_id(child.person_id)
        .relationship_type(PersonRelationshipType::Parent)
        .confidence(PersonConfidence::Definite)
        .build()?;

    let child_father = father_child.reciprocal();

    let mother_child = PersonRelationship::builder()
        .person_id(mother.person_id)
        .related_person_id(child.person_id)
        .relationship_type(PersonRelationshipType::Parent)
        .confidence(PersonConfidence::Definite)
        .build()?;

    let child_mother = mother_child.reciprocal();

    Ok(Family {
        persons: vec![father, mother, child],
        relationships: vec![
            marriage,
            marriage_reciprocal,
            father_child,
            child_father,
            mother_child,
            child_mother,
        ],
    })
}

struct Family {
    persons: Vec<Person>,
    relationships: Vec<PersonRelationship>,
}
```

### Pattern 3: Source Attribution Workflow

```rust
fn attribute_census_record(
    person: &Person,
    census_source_id: Uuid,
) -> Result<SourcePerson, String> {
    // Extract name exactly as written in census
    let source_person = SourcePerson::builder()
        .source_id(census_source_id)
        .person_id(person.person_id)
        .extracted_name_full("Jno W. Smith")  // Abbreviation in source
        .extracted_role("farmer")
        .page_reference("15, line 12")
        .confidence(PersonConfidence::Probable)
        .notes("Listed as head of household with wife Mary and son William")
        .build()?;

    Ok(source_person)
}
```

### Pattern 4: Duplicate Detection and Merge

```rust
fn merge_duplicates(
    duplicate: &mut Person,
    canonical: &Person,
    user_id: Uuid,
) -> Result<PersonMerge, String> {
    // Validate they're actually duplicates
    // (In real system, this would use sophisticated matching algorithms)

    // Create merge record
    let merge = PersonMerge::builder()
        .source_person_id(duplicate.person_id)
        .target_person_id(canonical.person_id)
        .merge_reason("Same person: birth date, location, parents all match")
        .merged_by(user_id)
        .build()?;

    // Archive duplicate
    duplicate.archive(
        format!("Merged into {}", canonical.canonical_name()),
        user_id,
    );

    // NOTE: In storage layer, you would also:
    // 1. Transfer all VariantNames to canonical person
    // 2. Transfer all PersonRelationships to canonical person
    // 3. Transfer all SourcePersons to canonical person
    // 4. Update any references from other entities

    Ok(merge)
}
```

---

## Integration Guidelines

### Database Integration

When implementing storage layer (rp-storage-postgres):

```rust
// Repository trait (to be implemented)
pub trait PersonRepository {
    async fn create(&self, person: Person) -> Result<Person>;
    async fn get_by_id(&self, person_id: Uuid) -> Result<Option<Person>>;
    async fn update(&self, person: Person) -> Result<Person>;
    async fn archive(&self, person_id: Uuid, reason: String, user_id: Uuid) -> Result<()>;

    // Relationship management
    async fn add_variant(&self, variant: VariantName) -> Result<VariantName>;
    async fn get_variants(&self, person_id: Uuid) -> Result<Vec<VariantName>>;

    async fn add_relationship(&self, rel: PersonRelationship) -> Result<PersonRelationship>;
    async fn get_relationships(&self, person_id: Uuid) -> Result<Vec<PersonRelationship>>;

    async fn add_source_link(&self, sp: SourcePerson) -> Result<SourcePerson>;
    async fn get_source_links(&self, person_id: Uuid) -> Result<Vec<SourcePerson>>;

    // Merge operations
    async fn merge(&self, merge: PersonMerge) -> Result<()>;
    async fn reverse_merge(&self, merge_id: Uuid, reason: String, user_id: Uuid) -> Result<()>;

    // Search
    async fn search(&self, query: &str) -> Result<Vec<Person>>;
    async fn find_duplicates(&self, person: &Person) -> Result<Vec<Person>>;
}
```

### API Integration

When implementing REST endpoints (rp-server):

```rust
// POST /api/persons
#[derive(Deserialize)]
struct CreatePersonRequest {
    given_name: Option<String>,
    surname: Option<String>,
    middle_name: Option<String>,
    birth_date: Option<GenealogyDate>,
    sex: Sex,
    // ... other fields
}

// GET /api/persons/:id
#[derive(Serialize)]
struct PersonResponse {
    person: Person,
    variants: Vec<VariantName>,
    relationships: Vec<PersonRelationship>,
    sources: Vec<SourcePerson>,
}

// POST /api/persons/:id/variants
#[derive(Deserialize)]
struct AddVariantRequest {
    given_name: Option<String>,
    surname: Option<String>,
    variant_type: VariantNameType,
    use_from_year: Option<i32>,
    use_to_year: Option<i32>,
}

// POST /api/persons/:id/relationships
#[derive(Deserialize)]
struct AddRelationshipRequest {
    related_person_id: Uuid,
    relationship_type: PersonRelationshipType,
    start_date: Option<GenealogyDate>,
    confidence: PersonConfidence,
}

// POST /api/persons/merge
#[derive(Deserialize)]
struct MergeRequest {
    source_person_id: Uuid,
    target_person_id: Uuid,
    reason: String,
}
```

### Search Integration

When implementing Elasticsearch integration:

```rust
// Index mapping for persons
{
  "mappings": {
    "properties": {
      "person_id": {"type": "keyword"},
      "canonical_name": {"type": "text"},
      "given_name": {"type": "text"},
      "surname": {"type": "text"},
      "birth_year": {"type": "integer"},
      "death_year": {"type": "integer"},
      "sex": {"type": "keyword"},

      // Nested variant names for full-text search
      "variants": {
        "type": "nested",
        "properties": {
          "full_name": {"type": "text"},
          "variant_type": {"type": "keyword"}
        }
      },

      // Life events for timeline
      "events": {
        "type": "nested",
        "properties": {
          "event_type": {"type": "keyword"},
          "year": {"type": "integer"},
          "location": {"type": "text"}
        }
      }
    }
  }
}
```

---

## Error Handling

### Error Types

All Person entity operations return `Result<T, String>` for now. In production, use proper error types:

```rust
pub enum PersonError {
    // Validation errors
    ValidationFailed(String),
    MissingRequiredField(&'static str),
    InvalidValue {
        field: &'static str,
        reason: String,
    },

    // Business logic errors
    CannotMergeSelf,
    MergeAlreadyReversed,
    EmptyMergeReason,
    DuplicateNotFound,

    // Storage errors
    DatabaseError(String),
    NotFound(Uuid),

    // State errors
    AlreadyArchived,
    CannotModifyArchived,
}
```

### Error Handling Examples

```rust
use rp_core::prelude::*;

// Validation errors
match person.validate() {
    Ok(()) => // Valid
    Err(e) => eprintln!("Validation failed: {}", e),
}

// Business logic errors
match PersonMerge::new(person_id, person_id, "reason", user_id) {
    Ok(merge) => // Success
    Err(e) => {
        // => "Cannot merge person with themselves"
        eprintln!("Merge failed: {}", e);
    }
}

// Reversal errors
match merge.reverse(user_id, "") {
    Ok(()) => // Success
    Err(e) => {
        // => "Reversal reason cannot be empty"
        eprintln!("Reversal failed: {}", e);
    }
}
```

### Best Practices

1. **Always validate before persisting**
   ```rust
   person.validate()?;
   repository.create(person).await?;
   ```

2. **Check constraints early**
   ```rust
   if source_person_id == target_person_id {
       return Err("Cannot merge person with themselves".to_string());
   }
   ```

3. **Provide context in errors**
   ```rust
   format!("Failed to archive person {}: {}", person_id, reason)
   ```

4. **Use Result propagation**
   ```rust
   let person = Person::builder()
       .given_name("John")
       .surname("Smith")
       .created_by(user_id)
       .build()?;  // Propagate errors with ?
   ```

---

## Testing Examples

### Unit Test Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_person_creation() {
        let user_id = Uuid::now_v7();
        let person = Person::builder()
            .given_name("John")
            .surname("Smith")
            .created_by(user_id)
            .build()
            .unwrap();

        assert_eq!(person.given_name, Some("John".to_string()));
        assert_eq!(person.canonical_name(), "John Smith");
    }

    #[test]
    fn test_variant_name_matching() {
        let person_id = Uuid::now_v7();
        let variant = VariantName::builder(person_id)
            .given_name("Mary")
            .surname("Johnson")
            .variant_type(VariantNameType::Birth)
            .build()
            .unwrap();

        assert!(variant.matches("mary"));
        assert!(variant.matches("Johnson"));
        assert!(!variant.matches("Smith"));
    }

    #[test]
    fn test_relationship_reciprocal() {
        let parent_id = Uuid::now_v7();
        let child_id = Uuid::now_v7();

        let parent_rel = PersonRelationship::builder()
            .person_id(parent_id)
            .related_person_id(child_id)
            .relationship_type(PersonRelationshipType::Parent)
            .build()
            .unwrap();

        let child_rel = parent_rel.reciprocal();

        assert_eq!(child_rel.person_id, child_id);
        assert_eq!(child_rel.related_person_id, parent_id);
        assert_eq!(child_rel.relationship_type, PersonRelationshipType::Child);
    }

    #[test]
    fn test_merge_prevents_self_merge() {
        let person_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();

        let result = PersonMerge::new(
            person_id,
            person_id,
            "test".to_string(),
            user_id,
        );

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cannot merge person with themselves"
        );
    }
}
```

---

## Appendix: MARC Patterns Reference

### MARC Authority Record Structure

The Person entity system implements these MARC authority control patterns:

```
MARC Authority Record for "Smith, John William, 1820-1891"

1XX - Established Heading (Canonical Name)
  100 1_ $a Smith, John William, $d 1820-1891

4XX - See From Tracing (Variant Names)
  400 1_ $a Schmidt, Johann $w nne
  400 1_ $a Smith, Jno $w nne
  400 1_ $a Smith, J. W. $w nne

5XX - See Also From Tracing (Relationships)
  500 1_ $a Smith, Mary Elizabeth, $d 1822-1895 $i spouse
  500 1_ $a Smith, William, $d 1846-1920 $i child

NOTE: $9 subfield would contain authority record ID for linking
```

### Mapping to Person Entity

| MARC Pattern | Person Entity | Purpose |
|--------------|---------------|---------|
| 1XX (Established Heading) | `Person` canonical name fields | The official, authoritative name form |
| 4XX (See From) | `VariantName` entities | All alternate name forms |
| 5XX (See Also) | `PersonRelationship` entities | Related persons (family) |
| $9 (Authority Link) | `SourcePerson` entities | Links from sources to authority |

---

## Version History

- **v1.0** (2025-11-09): Initial API documentation for Phase 1 Week 5 completion
  - Person entity with all core methods
  - VariantName with 14 variant types
  - PersonRelationship with 17 relationship types
  - SourcePerson for source attribution
  - PersonMerge for duplicate handling
  - All supporting types (Sex, PersonConfidence, GenealogyDate, DateCertainty)
  - Complete usage patterns and integration guidelines
  - 63 unit tests documented

---

**End of Document**
