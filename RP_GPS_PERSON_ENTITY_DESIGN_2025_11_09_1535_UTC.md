# ResearchProcess-GPS Person Entity: Complete Design Specification

**Document Created**: 2025-11-09 15:35:00 UTC
**Purpose**: Complete technical specification for Person entity implementation in ResearchProcess-GPS
**Session**: Phase 1, Week 2-4 - Person Entity Design
**Parent Documents**:
- KOHA_GENEALOGY_STRATEGIC_ANALYSIS.md
- KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md

---

## Executive Summary

This document provides a complete technical specification for implementing the **Person entity** in ResearchProcess-GPS, applying proven architectural patterns from Koha's 25-year-old authority control system. The Person entity represents **canonical individuals**—the authoritative, deduplicated records that serve as the central organizing principle for genealogical research.

### Design Principles

1. **Canonical First**: One authoritative record per person, no duplicates
2. **Variant Awareness**: All name forms systematically tracked and searchable
3. **Source Attribution**: Every data point links to evidence
4. **Relationship Modeling**: Complex family structures with certainty tracking
5. **Merge Operations**: Production-grade duplicate handling
6. **Audit Complete**: Full history of all changes
7. **MARC Interoperability**: Optional export/import for library systems

### Core Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     PERSON (Canonical)                      │
│  ┌────────────┐  ┌─────────────┐  ┌──────────────────┐    │
│  │ Core Data  │  │  Variants   │  │  Relationships   │    │
│  │            │  │             │  │                  │    │
│  │ - Names    │  │ - Birth     │  │ - Parent         │    │
│  │ - Dates    │  │ - Married   │  │ - Spouse         │    │
│  │ - Places   │  │ - Nickname  │  │ - Sibling        │    │
│  │ - Sex      │  │ - Immigration│ │ - Child          │    │
│  └────────────┘  └─────────────┘  └──────────────────┘    │
└─────────────────────────────────────────────────────────────┘
           ↓                    ↓                     ↓
    ┌──────────────┐    ┌───────────────┐    ┌───────────────┐
    │ SOURCE       │    │ SEARCH INDEX  │    │ MARC EXPORT   │
    │ MENTIONS     │    │ (All Variants)│    │ (Authority)   │
    └──────────────┘    └───────────────┘    └───────────────┘
```

---

## Part 1: Core Person Entity

### 1.1 Database Schema

#### persons Table

```sql
CREATE TABLE persons (
    -- Primary Key
    person_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Canonical Name Components (established form)
    given_name VARCHAR(255),
    surname VARCHAR(255),
    middle_name VARCHAR(255),
    name_prefix VARCHAR(50),     -- Dr., Rev., Hon., etc.
    name_suffix VARCHAR(50),     -- Jr., III, Esq., etc.

    -- Vital Events
    birth_date JSONB,             -- {year, month, day, certainty, circa}
    birth_place_id UUID REFERENCES places(place_id),
    birth_source_id UUID REFERENCES sources(source_id),

    death_date JSONB,
    death_place_id UUID REFERENCES places(place_id),
    death_source_id UUID REFERENCES sources(source_id),

    -- Biographical
    sex VARCHAR(10) CHECK (sex IN ('Male', 'Female', 'Unknown')),
    occupation VARCHAR(500),
    religion VARCHAR(100),

    -- Research Notes
    notes TEXT,
    research_notes TEXT,          -- Private notes for active research
    conclusion_notes TEXT,        -- Why this canonical form was chosen

    -- Confidence
    conclusion_confidence SMALLINT CHECK (conclusion_confidence BETWEEN 1 AND 5),

    -- MARC Export Cache
    marc_binary BYTEA,            -- Binary MARC21 authority
    marc_xml TEXT,                -- XML MARC21 authority
    marc_updated_at TIMESTAMPTZ,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(user_id),
    last_modified_by UUID NOT NULL REFERENCES users(user_id),

    -- Archival (for merged persons)
    archived BOOLEAN DEFAULT FALSE,
    archived_reason TEXT,
    archived_at TIMESTAMPTZ,
    archived_by UUID REFERENCES users(user_id)
);

-- Indexes
CREATE INDEX idx_persons_surname ON persons(surname) WHERE NOT archived;
CREATE INDEX idx_persons_given_name ON persons(given_name) WHERE NOT archived;
CREATE INDEX idx_persons_full_name ON persons((given_name || ' ' || surname)) WHERE NOT archived;
CREATE INDEX idx_persons_birth_year ON persons(((birth_date->>'year')::INT)) WHERE NOT archived;
CREATE INDEX idx_persons_death_year ON persons(((death_date->>'year')::INT)) WHERE NOT archived;
CREATE INDEX idx_persons_updated_at ON persons(updated_at);
CREATE INDEX idx_persons_created_by ON persons(created_by);

-- Full-text search
CREATE INDEX idx_persons_search ON persons
USING GIN(to_tsvector('english', COALESCE(given_name, '') || ' ' || COALESCE(surname, '') || ' ' || COALESCE(notes, '')))
WHERE NOT archived;

-- Auto-update timestamp trigger
CREATE TRIGGER update_persons_updated_at
BEFORE UPDATE ON persons
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Audit logging trigger
CREATE TRIGGER audit_persons_changes
AFTER INSERT OR UPDATE OR DELETE ON persons
FOR EACH ROW EXECUTE FUNCTION audit_log_changes();

COMMENT ON TABLE persons IS 'Canonical person records - one per individual';
COMMENT ON COLUMN persons.person_id IS 'Unique identifier for this person';
COMMENT ON COLUMN persons.birth_date IS 'JSON: {year: INT, month: INT, day: INT, certainty: STRING, circa: BOOL}';
COMMENT ON COLUMN persons.conclusion_confidence IS '1=Speculative, 2=Uncertain, 3=Possible, 4=Probable, 5=Definite';
COMMENT ON COLUMN persons.marc_binary IS 'Cached MARC21 authority record (binary)';
COMMENT ON COLUMN persons.archived IS 'TRUE if person was merged or deleted';
```

#### GenealogyDate Type

```sql
CREATE TYPE genealogy_date AS (
    year INT,
    month SMALLINT,
    day SMALLINT,
    certainty VARCHAR(20),        -- 'exact', 'estimated', 'calculated', 'before', 'after', 'between'
    circa BOOLEAN,
    end_year INT,                 -- For 'between' ranges
    original_text VARCHAR(200)    -- Original date string from source
);

-- Alternative: JSONB storage allows flexibility
-- Example JSONB: {"year": 1850, "month": 3, "day": 15, "certainty": "exact", "circa": false}
```

### 1.2 Rust Struct

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub person_id: Uuid,

    // Canonical name
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,

    // Vital events
    pub birth_date: Option<GenealogyDate>,
    pub birth_place_id: Option<Uuid>,
    pub birth_source_id: Option<Uuid>,

    pub death_date: Option<GenealogyDate>,
    pub death_place_id: Option<Uuid>,
    pub death_source_id: Option<Uuid>,

    // Biographical
    pub sex: Option<Sex>,
    pub occupation: Option<String>,
    pub religion: Option<String>,

    // Research
    pub notes: Option<String>,
    pub research_notes: Option<String>,
    pub conclusion_notes: Option<String>,
    pub conclusion_confidence: Option<Confidence>,

    // MARC cache
    pub marc_binary: Option<Vec<u8>>,
    pub marc_xml: Option<String>,
    pub marc_updated_at: Option<DateTime<Utc>>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub last_modified_by: Uuid,

    // Archival
    pub archived: bool,
    pub archived_reason: Option<String>,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenealogyDate {
    pub year: Option<i32>,
    pub month: Option<u8>,      // 1-12
    pub day: Option<u8>,        // 1-31
    pub certainty: DateCertainty,
    pub circa: bool,
    pub end_year: Option<i32>,  // For ranges
    pub original_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateCertainty {
    Exact,
    Estimated,
    Calculated,
    Before,
    After,
    Between,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Confidence {
    Speculative = 1,
    Uncertain = 2,
    Possible = 3,
    Probable = 4,
    Definite = 5,
}

impl Person {
    /// Get canonical full name in standard format
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

        parts.join(" ")
    }

    /// Get surname-first format (for indexing/sorting)
    pub fn surname_first(&self) -> String {
        let mut parts = Vec::new();

        if let Some(surname) = &self.surname {
            parts.push(surname.clone());
            parts.push(",".to_string());
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

        parts.join(" ")
    }

    /// Get years lived (for display)
    pub fn life_span(&self) -> Option<String> {
        match (&self.birth_date, &self.death_date) {
            (Some(birth), Some(death)) => {
                let b = birth.display_year();
                let d = death.display_year();
                Some(format!("{} - {}", b, d))
            },
            (Some(birth), None) => {
                Some(format!("{} - ", birth.display_year()))
            },
            (None, Some(death)) => {
                Some(format!(" - {}", death.display_year()))
            },
            (None, None) => None,
        }
    }

    /// Check if person is active (not archived)
    pub fn is_active(&self) -> bool {
        !self.archived
    }
}

impl GenealogyDate {
    /// Display year with certainty indicators
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
            },
            (None, _, _) => "Unknown".to_string(),
        }
    }

    /// Get sortable integer year (for timeline queries)
    pub fn sortable_year(&self) -> Option<i32> {
        self.year
    }
}
```

---

## Part 2: Variant Names

### 2.1 Database Schema

```sql
CREATE TABLE person_variant_names (
    variant_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES persons(person_id) ON DELETE CASCADE,

    -- Variant Name Components
    given_name VARCHAR(255),
    surname VARCHAR(255),
    middle_name VARCHAR(255),
    name_prefix VARCHAR(50),
    name_suffix VARCHAR(50),
    full_name VARCHAR(500) NOT NULL,   -- Searchable combined form

    -- Variant Classification
    variant_type VARCHAR(50) NOT NULL,  -- See VariantNameType enum
    language VARCHAR(10),               -- 'en', 'de', 'pl', etc.

    -- Attribution: Where was this variant found?
    source_id UUID REFERENCES sources(source_id),
    source_citation TEXT,              -- Specific page/line
    notes TEXT,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(user_id),

    CONSTRAINT chk_variant_has_name CHECK (
        given_name IS NOT NULL OR surname IS NOT NULL
    )
);

-- Indexes
CREATE INDEX idx_variant_names_person ON person_variant_names(person_id);
CREATE INDEX idx_variant_names_full ON person_variant_names(full_name);
CREATE INDEX idx_variant_names_surname ON person_variant_names(surname);
CREATE INDEX idx_variant_names_type ON person_variant_names(variant_type);
CREATE INDEX idx_variant_names_source ON person_variant_names(source_id);

-- Full-text search on variants
CREATE INDEX idx_variant_names_search ON person_variant_names
USING GIN(to_tsvector('english', full_name || ' ' || COALESCE(notes, '')));

-- Prevent exact duplicates
CREATE UNIQUE INDEX idx_variant_unique ON person_variant_names(
    person_id, full_name, variant_type
);

COMMENT ON TABLE person_variant_names IS 'All name variants for canonical persons';
COMMENT ON COLUMN person_variant_names.variant_type IS 'birth, married, nickname, immigration, spelling, etc.';
COMMENT ON COLUMN person_variant_names.source_id IS 'Source where this variant was encountered';
```

### 2.2 Rust Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantName {
    pub variant_id: Uuid,
    pub person_id: Uuid,

    // Name components
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub full_name: String,

    // Classification
    pub variant_type: VariantNameType,
    pub language: Option<String>,

    // Attribution
    pub source_id: Option<Uuid>,
    pub source_citation: Option<String>,
    pub notes: Option<String>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariantNameType {
    Birth,             // Name at birth
    Married,           // After marriage
    Divorced,          // After divorce (maiden name restored)
    Nickname,          // Common nickname ("Johnny", "Beth")
    Immigration,       // Name change on immigration
    Spelling,          // Spelling variation
    Translation,       // Language translation
    Abbreviation,      // Abbreviated form ("J. Smith")
    Pseudonym,         // Pen name or alias
    Legal,             // Legal name change
    Religious,         // Religious/monastic name
    Stage,             // Stage/professional name
    Documented,        // Exactly as appears in specific source
}

impl VariantName {
    /// Build full name from components
    pub fn new(
        person_id: Uuid,
        given: Option<String>,
        surname: Option<String>,
        variant_type: VariantNameType,
        created_by: Uuid,
    ) -> Self {
        let full_name = Self::build_full_name(&given, &surname, &None, &None, &None);

        Self {
            variant_id: Uuid::new_v4(),
            person_id,
            given_name: given,
            surname,
            middle_name: None,
            name_prefix: None,
            name_suffix: None,
            full_name,
            variant_type,
            language: None,
            source_id: None,
            source_citation: None,
            notes: None,
            created_at: Utc::now(),
            created_by,
        }
    }

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

    /// Check if this variant matches a search string
    pub fn matches(&self, search: &str) -> bool {
        let search_lower = search.to_lowercase();
        self.full_name.to_lowercase().contains(&search_lower)
    }
}
```

---

## Part 3: Person Relationships

### 3.1 Database Schema

```sql
CREATE TABLE person_relationships (
    relationship_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- The two persons involved
    person_1_id UUID NOT NULL REFERENCES persons(person_id),
    person_2_id UUID NOT NULL REFERENCES persons(person_id),

    -- Relationship type (directional)
    relationship_type VARCHAR(50) NOT NULL,

    -- Evidence & Certainty
    certainty SMALLINT NOT NULL CHECK (certainty BETWEEN 1 AND 5),
    source_id UUID REFERENCES sources(source_id),
    notes TEXT,

    -- Metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(user_id),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL REFERENCES users(user_id),

    CONSTRAINT chk_different_persons CHECK (person_1_id != person_2_id)
);

-- Indexes
CREATE INDEX idx_relationships_person1 ON person_relationships(person_1_id);
CREATE INDEX idx_relationships_person2 ON person_relationships(person_2_id);
CREATE INDEX idx_relationships_type ON person_relationships(relationship_type);
CREATE INDEX idx_relationships_certainty ON person_relationships(certainty);

-- Composite index for finding specific relationship between two persons
CREATE INDEX idx_relationships_pair ON person_relationships(person_1_id, person_2_id, relationship_type);

-- Auto-update trigger
CREATE TRIGGER update_relationships_updated_at
BEFORE UPDATE ON person_relationships
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

COMMENT ON TABLE person_relationships IS 'Relationships between canonical persons';
COMMENT ON COLUMN person_relationships.relationship_type IS 'parent, child, spouse, sibling, etc.';
COMMENT ON COLUMN person_relationships.certainty IS '1=Speculative, 2=Uncertain, 3=Possible, 4=Probable, 5=Definite';
```

### 3.2 Relationship Semantics

**Directional Relationships** (person_1 → person_2):

| relationship_type | Meaning | Inverse |
|------------------|---------|---------|
| parent | person_1 is parent of person_2 | child |
| child | person_1 is child of person_2 | parent |
| spouse | person_1 is spouse of person_2 | spouse |
| sibling | person_1 is sibling of person_2 | sibling |
| grandparent | person_1 is grandparent of person_2 | grandchild |
| grandchild | person_1 is grandchild of person_2 | grandparent |
| aunt_uncle | person_1 is aunt/uncle of person_2 | niece_nephew |
| niece_nephew | person_1 is niece/nephew of person_2 | aunt_uncle |
| cousin | person_1 is cousin of person_2 | cousin |
| step_parent | person_1 is step-parent of person_2 | step_child |
| step_child | person_1 is step-child of person_2 | step_parent |
| adoptive_parent | person_1 is adoptive parent of person_2 | adoptive_child |
| adoptive_child | person_1 is adoptive child of person_2 | adoptive_parent |
| foster_parent | person_1 is foster parent of person_2 | foster_child |
| foster_child | person_1 is foster child of person_2 | foster_parent |
| guardian | person_1 is guardian of person_2 | ward |
| ward | person_1 is ward of person_2 | guardian |

### 3.3 Rust Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRelationship {
    pub relationship_id: Uuid,
    pub person_1_id: Uuid,
    pub person_2_id: Uuid,
    pub relationship_type: RelationshipType,
    pub certainty: Confidence,
    pub source_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
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

impl RelationshipType {
    /// Get the inverse relationship
    pub fn inverse(&self) -> Self {
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

    /// Check if relationship is symmetric (bidirectional)
    pub fn is_symmetric(&self) -> bool {
        matches!(self, Self::Spouse | Self::Sibling | Self::Cousin)
    }

    /// Display name (human-readable)
    pub fn display(&self) -> &'static str {
        match self {
            Self::Parent => "Parent",
            Self::Child => "Child",
            Self::Spouse => "Spouse",
            Self::Sibling => "Sibling",
            Self::Grandparent => "Grandparent",
            Self::Grandchild => "Grandchild",
            Self::AuntUncle => "Aunt/Uncle",
            Self::NieceNephew => "Niece/Nephew",
            Self::Cousin => "Cousin",
            Self::StepParent => "Step-parent",
            Self::StepChild => "Step-child",
            Self::AdoptiveParent => "Adoptive Parent",
            Self::AdoptiveChild => "Adoptive Child",
            Self::FosterParent => "Foster Parent",
            Self::FosterChild => "Foster Child",
            Self::Guardian => "Guardian",
            Self::Ward => "Ward",
        }
    }
}

impl PersonRelationship {
    /// Create a bidirectional relationship (for symmetric types)
    pub fn create_symmetric(
        person_1_id: Uuid,
        person_2_id: Uuid,
        rel_type: RelationshipType,
        certainty: Confidence,
        source_id: Option<Uuid>,
        user_id: Uuid,
    ) -> (Self, Option<Self>) {
        let rel1 = Self {
            relationship_id: Uuid::new_v4(),
            person_1_id,
            person_2_id,
            relationship_type: rel_type.clone(),
            certainty,
            source_id,
            notes: None,
            created_at: Utc::now(),
            created_by: user_id,
            updated_at: Utc::now(),
            updated_by: user_id,
        };

        let rel2 = if rel_type.is_symmetric() {
            Some(Self {
                relationship_id: Uuid::new_v4(),
                person_1_id: person_2_id,
                person_2_id: person_1_id,
                relationship_type: rel_type,
                certainty,
                source_id,
                notes: None,
                created_at: Utc::now(),
                created_by: user_id,
                updated_at: Utc::now(),
                updated_by: user_id,
            })
        } else {
            None
        };

        (rel1, rel2)
    }
}
```

---

## Part 4: Source-Person Linking

### 4.1 Database Schema

```sql
CREATE TABLE source_persons (
    source_person_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Source this mention appears in
    source_id UUID NOT NULL REFERENCES sources(source_id) ON DELETE CASCADE,

    -- Canonical person (NULL if not yet linked)
    person_id UUID REFERENCES persons(person_id) ON DELETE SET NULL,

    -- Name exactly as it appears in source
    name_in_source VARCHAR(500) NOT NULL,

    -- Role in this source
    role VARCHAR(100),  -- 'subject', 'author', 'witness', 'informant', etc.

    -- Additional extracted data
    age_in_source INT,
    relationship_in_source VARCHAR(200),  -- "son of", "wife of", etc.
    occupation_in_source VARCHAR(200),
    residence_in_source VARCHAR(500),

    -- Linking metadata
    certainty SMALLINT CHECK (certainty IS NULL OR certainty BETWEEN 1 AND 5),
    linked_at TIMESTAMPTZ,
    linked_by UUID REFERENCES users(user_id),
    linking_notes TEXT,

    -- Extraction metadata
    page_number VARCHAR(50),
    line_number VARCHAR(50),
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES users(user_id)
);

-- Indexes
CREATE INDEX idx_source_persons_source ON source_persons(source_id);
CREATE INDEX idx_source_persons_person ON source_persons(person_id);
CREATE INDEX idx_source_persons_name ON source_persons(name_in_source);
CREATE INDEX idx_source_persons_role ON source_persons(role);
CREATE INDEX idx_source_persons_unlinked ON source_persons(source_id)
    WHERE person_id IS NULL;

-- Full-text search
CREATE INDEX idx_source_persons_search ON source_persons
USING GIN(to_tsvector('english', name_in_source || ' ' || COALESCE(notes, '')));

COMMENT ON TABLE source_persons IS 'Person mentions in sources (analogous to Koha biblio field → authority link)';
COMMENT ON COLUMN source_persons.person_id IS 'Link to canonical person (like MARC $9 subfield)';
COMMENT ON COLUMN source_persons.name_in_source IS 'Exact name as written in source';
COMMENT ON COLUMN source_persons.certainty IS 'Confidence that this mention = canonical person';
```

### 4.2 Rust Struct

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcePerson {
    pub source_person_id: Uuid,
    pub source_id: Uuid,
    pub person_id: Option<Uuid>,  // NULL if not linked yet

    // Source data
    pub name_in_source: String,
    pub role: Option<String>,
    pub age_in_source: Option<i32>,
    pub relationship_in_source: Option<String>,
    pub occupation_in_source: Option<String>,
    pub residence_in_source: Option<String>,

    // Linking
    pub certainty: Option<Confidence>,
    pub linked_at: Option<DateTime<Utc>>,
    pub linked_by: Option<Uuid>,
    pub linking_notes: Option<String>,

    // Location in source
    pub page_number: Option<String>,
    pub line_number: Option<String>,
    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

impl SourcePerson {
    /// Check if this mention is linked to a canonical person
    pub fn is_linked(&self) -> bool {
        self.person_id.is_some()
    }

    /// Link to a canonical person
    pub fn link_to_person(
        &mut self,
        person_id: Uuid,
        certainty: Confidence,
        user_id: Uuid,
        notes: Option<String>,
    ) {
        self.person_id = Some(person_id);
        self.certainty = Some(certainty);
        self.linked_at = Some(Utc::now());
        self.linked_by = Some(user_id);
        self.linking_notes = notes;
    }

    /// Unlink from canonical person
    pub fn unlink(&mut self) {
        self.person_id = None;
        self.certainty = None;
        self.linked_at = None;
        self.linked_by = None;
        self.linking_notes = None;
    }
}
```

---

## Part 5: Person Merge Operations

### 5.1 Database Schema

```sql
CREATE TABLE person_merges (
    merge_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- The merge
    from_person_id UUID NOT NULL,  -- Duplicate (archived)
    to_person_id UUID NOT NULL REFERENCES persons(person_id),  -- Canonical

    -- Strategy used
    merge_strategy VARCHAR(50) NOT NULL,

    -- Execution
    performed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    performed_by UUID NOT NULL REFERENCES users(user_id),

    -- Reversibility
    reversible BOOLEAN DEFAULT TRUE,
    reversed_at TIMESTAMPTZ,
    reversed_by UUID REFERENCES users(user_id),

    -- Data backup (for reversal)
    from_person_backup JSONB,  -- Full Person record before merge

    notes TEXT
);

CREATE INDEX idx_merges_from ON person_merges(from_person_id);
CREATE INDEX idx_merges_to ON person_merges(to_person_id);
CREATE INDEX idx_merges_performed_at ON person_merges(performed_at);
CREATE INDEX idx_merges_reversible ON person_merges(reversible) WHERE NOT reversed_at IS NULL;

COMMENT ON TABLE person_merges IS 'Log of person merge operations (for audit and reversal)';
COMMENT ON COLUMN person_merges.from_person_backup IS 'JSON snapshot of archived person (enables reversal)';
```

### 5.2 Rust Implementation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonMerge {
    pub merge_id: Uuid,
    pub from_person_id: Uuid,
    pub to_person_id: Uuid,
    pub merge_strategy: MergeStrategy,
    pub performed_at: DateTime<Utc>,
    pub performed_by: Uuid,
    pub reversible: bool,
    pub reversed_at: Option<DateTime<Utc>>,
    pub reversed_by: Option<Uuid>,
    pub from_person_backup: Option<serde_json::Value>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeStrategy {
    KeepAll,           // Keep all data from both persons
    PreferCanonical,   // Prefer "to" person's data
    PreferSource,      // Prefer "from" person's data
    Manual,            // User manually chose each field
}

impl Person {
    /// Merge another person into this one
    pub async fn merge_from(
        db: &PgPool,
        from_person_id: Uuid,
        to_person_id: Uuid,
        strategy: MergeStrategy,
        user_id: Uuid,
        notes: Option<String>,
    ) -> Result<PersonMerge> {
        let mut tx = db.begin().await?;

        // 1. Retrieve both persons
        let from_person = Person::find(&mut tx, from_person_id).await?;
        let to_person = Person::find(&mut tx, to_person_id).await?;

        // 2. Backup from_person for potential reversal
        let from_backup = serde_json::to_value(&from_person)?;

        // 3. Transfer all source mentions
        sqlx::query(
            r#"
            UPDATE source_persons
            SET person_id = $1, updated_at = NOW()
            WHERE person_id = $2
            "#
        )
        .bind(to_person_id)
        .bind(from_person_id)
        .execute(&mut *tx)
        .await?;

        // 4. Copy variant names (avoid duplicates)
        sqlx::query(
            r#"
            INSERT INTO person_variant_names
            (person_id, given_name, surname, middle_name, name_prefix, name_suffix,
             full_name, variant_type, language, source_id, source_citation, notes, created_by)
            SELECT $1, given_name, surname, middle_name, name_prefix, name_suffix,
                   full_name, variant_type, language, source_id, source_citation, notes, $3
            FROM person_variant_names
            WHERE person_id = $2
            ON CONFLICT (person_id, full_name, variant_type) DO NOTHING
            "#
        )
        .bind(to_person_id)
        .bind(from_person_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // 5. Update relationships (person_1)
        sqlx::query(
            r#"
            UPDATE person_relationships
            SET person_1_id = $1, updated_at = NOW(), updated_by = $3
            WHERE person_1_id = $2
            "#
        )
        .bind(to_person_id)
        .bind(from_person_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // 6. Update relationships (person_2)
        sqlx::query(
            r#"
            UPDATE person_relationships
            SET person_2_id = $1, updated_at = NOW(), updated_by = $3
            WHERE person_2_id = $2
            "#
        )
        .bind(to_person_id)
        .bind(from_person_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // 7. Archive the from_person (don't delete - preserve for audit)
        sqlx::query(
            r#"
            UPDATE persons
            SET archived = TRUE,
                archived_reason = 'Merged into person ' || $2::TEXT,
                archived_at = NOW(),
                archived_by = $3,
                updated_at = NOW()
            WHERE person_id = $1
            "#
        )
        .bind(from_person_id)
        .bind(to_person_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // 8. Log the merge
        let merge = sqlx::query_as::<_, PersonMerge>(
            r#"
            INSERT INTO person_merges
            (from_person_id, to_person_id, merge_strategy, performed_by, from_person_backup, notes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(from_person_id)
        .bind(to_person_id)
        .bind(&strategy)
        .bind(user_id)
        .bind(&from_backup)
        .bind(&notes)
        .fetch_one(&mut *tx)
        .await?;

        // 9. Invalidate MARC cache on to_person
        sqlx::query(
            r#"
            UPDATE persons
            SET marc_binary = NULL, marc_xml = NULL, marc_updated_at = NULL
            WHERE person_id = $1
            "#
        )
        .bind(to_person_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(merge)
    }

    /// Reverse a merge operation
    pub async fn reverse_merge(
        db: &PgPool,
        merge_id: Uuid,
        user_id: Uuid,
    ) -> Result<()> {
        let mut tx = db.begin().await?;

        // 1. Get merge record
        let merge = PersonMerge::find(&mut tx, merge_id).await?;

        if !merge.reversible {
            return Err(Error::NotReversible("Merge marked as non-reversible".into()));
        }

        if merge.reversed_at.is_some() {
            return Err(Error::AlreadyReversed("Merge already reversed".into()));
        }

        // 2. Restore from_person from backup
        let from_backup: Person = serde_json::from_value(
            merge.from_person_backup
                .ok_or_else(|| Error::NoBackup("No backup data found".into()))?
        )?;

        // Restore the person (un-archive)
        Person::restore(&mut tx, from_backup).await?;

        // 3. Re-link source mentions that belonged to from_person
        // (This is complex - might need additional tracking)
        // For simplicity, we keep them with to_person and add a note

        // 4. Mark merge as reversed
        sqlx::query(
            r#"
            UPDATE person_merges
            SET reversed_at = NOW(), reversed_by = $2
            WHERE merge_id = $1
            "#
        )
        .bind(merge_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
```

---

## Part 6: Search & Query Operations

### 6.1 Core Query Methods

```rust
impl Person {
    /// Find person by ID
    pub async fn find(db: &PgPool, person_id: Uuid) -> Result<Person> {
        sqlx::query_as::<_, Person>(
            "SELECT * FROM persons WHERE person_id = $1 AND NOT archived"
        )
        .bind(person_id)
        .fetch_one(db)
        .await
        .map_err(Into::into)
    }

    /// Search persons by name (canonical + variants)
    pub async fn search_by_name(
        db: &PgPool,
        search_term: &str,
        limit: i64,
    ) -> Result<Vec<Person>> {
        // Search canonical names
        let canonical = sqlx::query_as::<_, Person>(
            r#"
            SELECT DISTINCT p.*
            FROM persons p
            WHERE NOT p.archived
              AND (
                  p.given_name ILIKE $1 OR
                  p.surname ILIKE $1 OR
                  (p.given_name || ' ' || p.surname) ILIKE $1
              )
            LIMIT $2
            "#
        )
        .bind(format!("%{}%", search_term))
        .bind(limit)
        .fetch_all(db)
        .await?;

        // Search variant names
        let variants = sqlx::query_as::<_, Person>(
            r#"
            SELECT DISTINCT p.*
            FROM persons p
            JOIN person_variant_names v ON v.person_id = p.person_id
            WHERE NOT p.archived
              AND v.full_name ILIKE $1
            LIMIT $2
            "#
        )
        .bind(format!("%{}%", search_term))
        .bind(limit)
        .fetch_all(db)
        .await?;

        // Combine and deduplicate
        let mut results: Vec<Person> = canonical;
        for v in variants {
            if !results.iter().any(|p| p.person_id == v.person_id) {
                results.push(v);
            }
        }

        Ok(results)
    }

    /// Full-text search
    pub async fn full_text_search(
        db: &PgPool,
        query: &str,
        limit: i64,
    ) -> Result<Vec<(Person, f32)>> {  // Returns (person, rank)
        sqlx::query_as::<_, (Person, f32)>(
            r#"
            SELECT p.*, ts_rank(
                to_tsvector('english', COALESCE(p.given_name, '') || ' ' ||
                            COALESCE(p.surname, '') || ' ' || COALESCE(p.notes, '')),
                plainto_tsquery('english', $1)
            ) as rank
            FROM persons p
            WHERE NOT p.archived
              AND to_tsvector('english', COALESCE(p.given_name, '') || ' ' ||
                              COALESCE(p.surname, '') || ' ' || COALESCE(p.notes, ''))
                  @@ plainto_tsquery('english', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#
        )
        .bind(query)
        .bind(limit)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Find persons by year range
    pub async fn find_by_year_range(
        db: &PgPool,
        from_year: i32,
        to_year: i32,
    ) -> Result<Vec<Person>> {
        sqlx::query_as::<_, Person>(
            r#"
            SELECT *
            FROM persons
            WHERE NOT archived
              AND (
                  (birth_date->>'year')::INT BETWEEN $1 AND $2
                  OR
                  (death_date->>'year')::INT BETWEEN $1 AND $2
              )
            ORDER BY (birth_date->>'year')::INT
            "#
        )
        .bind(from_year)
        .bind(to_year)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Get all variant names
    pub async fn variants(&self, db: &PgPool) -> Result<Vec<VariantName>> {
        sqlx::query_as::<_, VariantName>(
            "SELECT * FROM person_variant_names WHERE person_id = $1 ORDER BY created_at"
        )
        .bind(self.person_id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Get all relationships
    pub async fn relationships(&self, db: &PgPool) -> Result<Vec<PersonRelationship>> {
        sqlx::query_as::<_, PersonRelationship>(
            r#"
            SELECT * FROM person_relationships
            WHERE person_1_id = $1 OR person_2_id = $1
            ORDER BY relationship_type, created_at
            "#
        )
        .bind(self.person_id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Get specific relationship type (e.g., all children)
    pub async fn get_related_persons(
        &self,
        db: &PgPool,
        rel_type: RelationshipType,
    ) -> Result<Vec<(Person, Confidence)>> {
        sqlx::query_as::<_, (Person, Confidence)>(
            r#"
            SELECT p.*, r.certainty
            FROM persons p
            JOIN person_relationships r ON
                (r.person_1_id = $1 AND r.person_2_id = p.person_id AND r.relationship_type = $2)
                OR
                (r.person_2_id = $1 AND r.person_1_id = p.person_id AND r.relationship_type = $3)
            WHERE NOT p.archived
            ORDER BY r.certainty DESC, p.surname, p.given_name
            "#
        )
        .bind(self.person_id)
        .bind(&rel_type)
        .bind(&rel_type.inverse())
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Count source citations
    pub async fn source_count(&self, db: &PgPool) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM source_persons WHERE person_id = $1"
        )
        .bind(self.person_id)
        .fetch_one(db)
        .await?;

        Ok(count.0)
    }

    /// Get all linked sources
    pub async fn linked_sources(&self, db: &PgPool) -> Result<Vec<Source>> {
        sqlx::query_as::<_, Source>(
            r#"
            SELECT DISTINCT s.*
            FROM sources s
            JOIN source_persons sp ON sp.source_id = s.source_id
            WHERE sp.person_id = $1
            ORDER BY s.source_date, s.title
            "#
        )
        .bind(self.person_id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Get timeline of sources mentioning this person
    pub async fn source_timeline(&self, db: &PgPool) -> Result<Vec<SourcePersonCitation>> {
        sqlx::query_as::<_, SourcePersonCitation>(
            r#"
            SELECT sp.*, s.title, s.source_date, s.source_type
            FROM source_persons sp
            JOIN sources s ON s.source_id = sp.source_id
            WHERE sp.person_id = $1
            ORDER BY s.source_date, s.title
            "#
        )
        .bind(self.person_id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Find potential duplicates
    pub async fn find_duplicates(
        &self,
        db: &PgPool,
        threshold: f32,
    ) -> Result<Vec<(Person, f32)>> {
        // Use fuzzy matching on names and dates
        // Simplified example - real implementation would use more sophisticated algorithms

        sqlx::query_as::<_, (Person, f32)>(
            r#"
            SELECT p.*, similarity(
                p.given_name || ' ' || p.surname,
                $2 || ' ' || $3
            ) as sim
            FROM persons p
            WHERE p.person_id != $1
              AND NOT p.archived
              AND similarity(p.given_name || ' ' || p.surname, $2 || ' ' || $3) > $4
            ORDER BY sim DESC
            LIMIT 20
            "#
        )
        .bind(self.person_id)
        .bind(&self.given_name)
        .bind(&self.surname)
        .bind(threshold)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }
}
```

---

## Part 7: Elasticsearch Index Mapping

### 7.1 Person Index Mapping

```json
{
  "settings": {
    "number_of_shards": 1,
    "number_of_replicas": 1,
    "analysis": {
      "analyzer": {
        "name_analyzer": {
          "type": "custom",
          "tokenizer": "standard",
          "filter": ["lowercase", "name_synonym", "name_phonetic"]
        }
      },
      "filter": {
        "name_synonym": {
          "type": "synonym",
          "synonyms": [
            "john, jon, jonathon, jonathan",
            "william, will, bill, billy",
            "elizabeth, liz, beth, betty",
            "margaret, maggie, peggy, meg"
          ]
        },
        "name_phonetic": {
          "type": "phonetic",
          "encoder": "metaphone",
          "replace": false
        }
      }
    }
  },
  "mappings": {
    "properties": {
      "person_id": { "type": "keyword" },
      "canonical_name": {
        "type": "text",
        "analyzer": "name_analyzer",
        "fields": {
          "keyword": { "type": "keyword" },
          "raw": { "type": "text", "analyzer": "standard" }
        }
      },
      "given_name": {
        "type": "text",
        "analyzer": "name_analyzer",
        "fields": {"keyword": {"type": "keyword"}}
      },
      "surname": {
        "type": "text",
        "analyzer": "name_analyzer",
        "fields": {"keyword": {"type": "keyword"}}
      },
      "variant_names": {
        "type": "text",
        "analyzer": "name_analyzer"
      },
      "birth_year": { "type": "integer" },
      "death_year": { "type": "integer" },
      "birth_place": {
        "type": "text",
        "fields": {"keyword": {"type": "keyword"}}
      },
      "death_place": {
        "type": "text",
        "fields": {"keyword": {"type": "keyword"}}
      },
      "sex": { "type": "keyword" },
      "occupation": { "type": "text" },
      "notes": { "type": "text" },
      "source_count": { "type": "integer" },
      "updated_at": { "type": "date" },
      "created_at": { "type": "date" }
    }
  }
}
```

### 7.2 Indexing Function

```rust
use elasticsearch::{Elasticsearch, IndexParts};
use serde_json::json;

impl Person {
    /// Index this person in Elasticsearch
    pub async fn index_to_elasticsearch(&self, es: &Elasticsearch, db: &PgPool) -> Result<()> {
        // Get all variant names
        let variants = self.variants(db).await?;
        let variant_names: Vec<String> = variants
            .into_iter()
            .map(|v| v.full_name)
            .collect();

        // Get source count
        let source_count = self.source_count(db).await?;

        // Build document
        let doc = json!({
            "person_id": self.person_id.to_string(),
            "canonical_name": self.canonical_name(),
            "given_name": self.given_name,
            "surname": self.surname,
            "variant_names": variant_names,  // ALL variants indexed!
            "birth_year": self.birth_date.as_ref().and_then(|d| d.year),
            "death_year": self.death_date.as_ref().and_then(|d| d.year),
            "birth_place": self.birth_place_id,  // TODO: resolve to name
            "death_place": self.death_place_id,
            "sex": self.sex,
            "occupation": self.occupation,
            "notes": self.notes,
            "source_count": source_count,
            "updated_at": self.updated_at,
            "created_at": self.created_at,
        });

        // Index
        es.index(IndexParts::IndexId("persons", &self.person_id.to_string()))
            .body(doc)
            .send()
            .await?;

        Ok(())
    }

    /// Search Elasticsearch for persons
    pub async fn search_es(
        es: &Elasticsearch,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Person>> {
        let response = es.search(SearchParts::Index(&["persons"]))
            .body(json!({
                "query": {
                    "multi_match": {
                        "query": query,
                        "fields": [
                            "canonical_name^3",    // Boost canonical
                            "variant_names^2",      // Boost variants
                            "given_name",
                            "surname",
                            "notes"
                        ],
                        "fuzziness": "AUTO"
                    }
                },
                "size": limit
            }))
            .send()
            .await?;

        // Parse and return persons
        // ... (parse Elasticsearch response)

        Ok(Vec::new())  // Placeholder
    }
}
```

---

## Part 8: API Endpoints

### 8.1 RESTful API Design

**Base URL**: `/api/v1/persons`

#### GET /persons
List persons with pagination and filtering

**Query Parameters**:
- `page` (int): Page number (default: 1)
- `limit` (int): Results per page (default: 20, max: 100)
- `search` (string): Search term (name, notes)
- `birth_year_from` (int): Min birth year
- `birth_year_to` (int): Max birth year
- `sex` (string): Male|Female|Unknown
- `has_sources` (boolean): Only persons with sources
- `sort` (string): surname|birth_year|created_at (default: surname)

**Response**:
```json
{
  "data": [
    {
      "person_id": "uuid",
      "canonical_name": "John Smith",
      "given_name": "John",
      "surname": "Smith",
      "birth_date": {"year": 1820, "month": 3, "certainty": "exact"},
      "death_date": {"year": 1891, "month": 3, "certainty": "exact"},
      "life_span": "1820 - 1891",
      "source_count": 15,
      "created_at": "2025-11-09T15:00:00Z",
      "updated_at": "2025-11-09T16:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 156,
    "pages": 8
  }
}
```

#### GET /persons/:id
Get single person with full details

**Response**:
```json
{
  "person_id": "uuid",
  "canonical_name": "John Smith",
  "given_name": "John",
  "surname": "Smith",
  "middle_name": null,
  "name_prefix": null,
  "name_suffix": "Jr.",
  "birth_date": {"year": 1820, "month": 3, "day": 15, "certainty": "exact"},
  "birth_place": {
    "place_id": "uuid",
    "name": "Akron, Summit County, Ohio, USA"
  },
  "death_date": {"year": 1891, "month": 12, "day": 10, "certainty": "exact"},
  "death_place": {
    "place_id": "uuid",
    "name": "Cleveland, Cuyahoga County, Ohio, USA"
  },
  "sex": "Male",
  "occupation": "Blacksmith",
  "notes": "Immigrated from Germany in 1840",
  "conclusion_confidence": 5,
  "source_count": 15,
  "variant_count": 4,
  "relationship_count": 8,
  "created_at": "2025-11-09T15:00:00Z",
  "updated_at": "2025-11-09T16:00:00Z"
}
```

#### POST /persons
Create new person

**Request Body**:
```json
{
  "given_name": "John",
  "surname": "Smith",
  "birth_date": {"year": 1820, "month": 3, "certainty": "exact"},
  "sex": "Male",
  "notes": "Found in 1850 census"
}
```

#### PUT /persons/:id
Update person

#### DELETE /persons/:id
Archive person (soft delete)

#### GET /persons/:id/variants
Get all variant names

#### POST /persons/:id/variants
Add variant name

#### GET /persons/:id/relationships
Get all relationships

#### POST /persons/:id/relationships
Add relationship

#### GET /persons/:id/sources
Get all linked sources

#### POST /persons/:id/merge
Merge another person into this one

**Request Body**:
```json
{
  "from_person_id": "uuid",
  "merge_strategy": "keep_all",
  "notes": "Duplicate found in 1850 census"
}
```

---

## Part 9: Testing Strategy

### 9.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_name() {
        let person = Person {
            person_id: Uuid::new_v4(),
            given_name: Some("John".to_string()),
            surname: Some("Smith".to_string()),
            middle_name: Some("William".to_string()),
            name_suffix: Some("Jr.".to_string()),
            // ... other fields
        };

        assert_eq!(person.canonical_name(), "John William Smith Jr.");
    }

    #[test]
    fn test_life_span() {
        let person = Person {
            birth_date: Some(GenealogyDate {
                year: Some(1820),
                certainty: DateCertainty::Exact,
                ..Default::default()
            }),
            death_date: Some(GenealogyDate {
                year: Some(1891),
                certainty: DateCertainty::Exact,
                ..Default::default()
            }),
            // ... other fields
        };

        assert_eq!(person.life_span(), Some("1820 - 1891".to_string()));
    }

    #[test]
    fn test_relationship_inverse() {
        assert_eq!(RelationshipType::Parent.inverse(), RelationshipType::Child);
        assert_eq!(RelationshipType::Spouse.inverse(), RelationshipType::Spouse);
    }
}
```

### 9.2 Integration Tests

```rust
#[tokio::test]
async fn test_person_crud() {
    let db = setup_test_db().await;

    // Create
    let person = Person::create(&db, PersonCreate {
        given_name: Some("John".to_string()),
        surname: Some("Smith".to_string()),
        ..Default::default()
    }).await.unwrap();

    // Read
    let found = Person::find(&db, person.person_id).await.unwrap();
    assert_eq!(found.given_name, Some("John".to_string()));

    // Update
    person.update(&db, PersonUpdate {
        occupation: Some("Blacksmith".to_string()),
        ..Default::default()
    }).await.unwrap();

    // Delete (archive)
    person.archive(&db, "Test".to_string()).await.unwrap();

    teardown_test_db(db).await;
}

#[tokio::test]
async fn test_variant_names() {
    let db = setup_test_db().await;

    let person = create_test_person(&db).await;

    // Add variants
    person.add_variant(&db, VariantName {
        given_name: Some("Johann".to_string()),
        surname: Some("Schmidt".to_string()),
        variant_type: VariantNameType::Immigration,
        ..Default::default()
    }).await.unwrap();

    let variants = person.variants(&db).await.unwrap();
    assert_eq!(variants.len(), 1);
    assert_eq!(variants[0].variant_type, VariantNameType::Immigration);

    teardown_test_db(db).await;
}

#[tokio::test]
async fn test_person_merge() {
    let db = setup_test_db().await;

    let person1 = create_test_person(&db).await;
    let person2 = create_test_person(&db).await;

    // Link sources to person2
    create_test_source_link(&db, person2.person_id).await;

    // Merge person2 into person1
    Person::merge_from(
        &db,
        person2.person_id,
        person1.person_id,
        MergeStrategy::KeepAll,
        test_user_id(),
        None,
    ).await.unwrap();

    // Verify person2 is archived
    let person2_after = Person::find(&db, person2.person_id).await;
    assert!(person2_after.is_err());  // Not found (archived)

    // Verify sources moved to person1
    let source_count = person1.source_count(&db).await.unwrap();
    assert_eq!(source_count, 1);

    teardown_test_db(db).await;
}
```

---

## Part 10: Migration & Implementation Plan

### Phase 1: Core Entity (Weeks 5-6)

**Week 5**:
- [ ] Create `persons` table migration
- [ ] Create `person_variant_names` table migration
- [ ] Implement `Person` struct with basic CRUD
- [ ] Implement `VariantName` struct
- [ ] Write unit tests for data structures
- [ ] Database triggers (updated_at, audit log)

**Week 6**:
- [ ] Create `person_relationships` table
- [ ] Create `source_persons` table
- [ ] Implement relationship CRUD
- [ ] Implement source linking
- [ ] Integration tests

### Phase 2: Operations (Weeks 7-8)

**Week 7**:
- [ ] Implement search functions (name, year, full-text)
- [ ] Elasticsearch index mapping
- [ ] Indexing pipeline
- [ ] Search API endpoints

**Week 8**:
- [ ] Person merge operations
- [ ] Duplicate detection
- [ ] Merge reversal
- [ ] Audit logging

### Phase 3: API & UI (Weeks 9-10)

**Week 9**:
- [ ] REST API endpoints
- [ ] OpenAPI/Swagger specification
- [ ] API documentation
- [ ] API integration tests

**Week 10**:
- [ ] UI components for person management
- [ ] Person detail view
- [ ] Relationship graph visualization
- [ ] Merge wizard UI

### Phase 4: MARC Integration (Weeks 11-12)

**Week 11**:
- [ ] MARC21 authority export
- [ ] Person → MARC conversion
- [ ] MARC caching system

**Week 12**:
- [ ] MARC21 authority import
- [ ] MARC → Person conversion
- [ ] Round-trip testing

---

## Conclusion

This comprehensive design specification provides a complete blueprint for implementing the Person entity in ResearchProcess-GPS, applying 25 years of proven patterns from Koha's authority control system while tailoring them specifically for genealogical research needs.

### Key Achievements

1. **Canonical Person Records**: One authoritative record per individual
2. **Variant Name System**: Systematic tracking of all name forms
3. **Relationship Modeling**: Complex family structures with certainty
4. **Source Attribution**: Every data point linked to evidence
5. **Merge Operations**: Production-grade duplicate handling
6. **Search Integration**: Elasticsearch with variant embedding
7. **API Complete**: RESTful endpoints for all operations
8. **MARC Ready**: Optional export/import for library integration

### Implementation Priority

**Must Have** (Phase 1-2):
- Core Person CRUD
- Variant names
- Source linking
- Basic search

**Should Have** (Phase 3):
- Relationships
- Merge operations
- Full API

**Could Have** (Phase 4):
- MARC integration
- Advanced duplicate detection
- Genealogy-specific analysis tools

---

**Document Status**: Week 4 design specification complete
**Next Steps**: Begin Phase 1 implementation (Week 5)
**Related Documents**:
- KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md
- MARC_ADAPTER_SPECIFICATION_2025_11_09_1535_UTC.md (to be created)

---

*This specification provides everything needed to implement a world-class Person entity system for genealogical research, built on library science best practices.*
