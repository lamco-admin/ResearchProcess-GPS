# Koha Authority Control Architecture: Deep Technical Analysis

**Document Created**: 2025-11-09 15:35:00 UTC
**Purpose**: Deep dive into Koha's authority control architecture for ResearchProcess-GPS Person entity design
**Session**: Phase 1, Week 2 - Authority Control Study
**Parent Document**: KOHA_GENEALOGY_STRATEGIC_ANALYSIS.md

---

## Executive Summary

This document provides a comprehensive technical analysis of Koha's authority control architecture, examining database schema, code implementation, MARC21 integration, and linking mechanisms. The goal is to extract proven architectural patterns for implementing canonical person/place records in ResearchProcess-GPS.

### Key Findings

1. **Authority Control is Battle-Tested**: 25+ years of evolution, thousands of production deployments
2. **Elegant Three-Layer Architecture**: Database → MARC Abstraction → Application Logic
3. **Sophisticated Linking System**: Automatic bidirectional links between authorities and bibliographic records
4. **Variant Handling Built-In**: MARC 4XX fields provide standardized variant name management
5. **Hierarchical Relationships**: MARC 5XX fields + authtrees support complex authority relationships
6. **Usage Tracking**: Efficient queries to find all records mentioning an authority
7. **Merge/Split Operations**: Production-grade tools for authority consolidation

---

## Part 1: Database Architecture

### Core Tables Schema

#### 1. auth_header Table

The central authority storage table:

```sql
CREATE TABLE auth_header (
    authid              BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    authtypecode        VARCHAR(10) NOT NULL DEFAULT '',
    datecreated         DATE,
    modification_time   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                        ON UPDATE CURRENT_TIMESTAMP,
    origincode          VARCHAR(20),
    authtrees           LONGTEXT,           -- Hierarchical structure
    marc                BLOB,               -- Binary MARC
    linkid              BIGINT,             -- Related authority link
    marcxml             LONGTEXT NOT NULL   -- XML MARC representation
);
```

**Key Design Patterns**:

1. **Dual Storage Format** (marc + marcxml):
   - **Binary MARC** (`marc`): Original format, preserves all encoding
   - **XML MARC** (`marcxml`): Easier parsing, XPath queries, modern tooling
   - **Trade-off**: Storage redundancy for query flexibility

2. **authtrees Field**:
   - Stores hierarchical authority relationships
   - Used for displaying broader/narrower term trees
   - JSON or delimited format (implementation-specific)

3. **linkid Field**:
   - Links to related authorities
   - Supports "see also" references
   - Enables authority network graphs

4. **modification_time**:
   - AUTO-UPDATE on record changes
   - Crucial for sync operations
   - Merge conflict detection

#### 2. auth_types Table

Authority type definitions:

```sql
CREATE TABLE auth_types (
    authtypecode         VARCHAR(10) NOT NULL PRIMARY KEY,
    authtypetext         VARCHAR(255) NOT NULL DEFAULT '',
    auth_tag_to_report   VARCHAR(3) NOT NULL DEFAULT '',  -- Main MARC tag
    summary              LONGTEXT NOT NULL
);
```

**Standard Authority Types** (MARC21):

| authtypecode | authtypetext | auth_tag_to_report | Use Case |
|--------------|--------------|-------------------|----------|
| PERSO_NAME   | Personal Name | 100 | Individual persons |
| CORPO_NAME   | Corporate Name | 110 | Organizations |
| MEETI_NAME   | Meeting Name | 111 | Conferences, events |
| UNIF_TITLE   | Uniform Title | 130 | Works |
| CHRON_TERM   | Chronological Term | 148 | Time periods |
| TOPIC_TERM   | Topical Term | 150 | Subject headings |
| GEOGR_NAME   | Geographic Name | 151 | Places |
| GENRE/FORM   | Genre/Form | 155 | Document types |

**Genealogical Mapping**:
- **PERSO_NAME** → Canonical Person records
- **GEOGR_NAME** → Place name authority
- **CHRON_TERM** → Historical periods
- **TOPIC_TERM** → Event types (birth, marriage, death)

#### 3. borrower_relationships Table

Relationship modeling (added in Koha 19.11):

```sql
CREATE TABLE borrower_relationships (
    id               INT(11) NOT NULL AUTO_INCREMENT PRIMARY KEY,
    guarantor_id     INT(11) NOT NULL,    -- "Parent" entity
    guarantee_id     INT(11) NOT NULL,    -- "Child" entity
    relationship     VARCHAR(100) NOT NULL,
    FOREIGN KEY (guarantor_id) REFERENCES borrowers(borrowernumber),
    FOREIGN KEY (guarantee_id) REFERENCES borrowers(borrowernumber)
);
```

**Genealogical Applicability**:

```sql
-- Adapted for genealogy:
CREATE TABLE person_relationships (
    id               BIGINT AUTO_INCREMENT PRIMARY KEY,
    person_1_id      BIGINT NOT NULL,
    person_2_id      BIGINT NOT NULL,
    relationship_type VARCHAR(50) NOT NULL,  -- 'parent-child', 'spouse', 'sibling'
    certainty        SMALLINT,                -- Confidence level
    source_id        BIGINT,                  -- Evidence source
    FOREIGN KEY (person_1_id) REFERENCES persons(person_id),
    FOREIGN KEY (person_2_id) REFERENCES persons(person_id),
    FOREIGN KEY (source_id) REFERENCES sources(source_id)
);
```

---

## Part 2: MARC21 Authority Format Structure

### Personal Name Authority Record

**Example: John Smith, 1820-1891**

```
LDR 00000nz  a2200000n  4500
001    123456                    -- Authority record control number
003    DLC                       -- Source
005    20251109153000.0          -- Modification timestamp
008    850101n| azannaabn      |n aaa
040    ‡a DLC ‡c DLC             -- Cataloging source
100 1_ ‡a Smith, John, ‡d 1820-1891   -- ESTABLISHED HEADING
400 1_ ‡a Smith, J. ‡q (John)         -- Variant form
400 1_ ‡a Smith, John                 -- Variant without dates
400 1_ ‡a Schmidt, Johann ‡d 1820-1891 -- Immigration name variant
500 1_ ‡w r ‡i Spouse: ‡a Smith, Mary, ‡d 1822-1895
500 1_ ‡w r ‡i Father: ‡a Smith, William, ‡d 1790-1870
500 1_ ‡w r ‡i Mother: ‡a Jones, Elizabeth, ‡d 1795-1875
670    ‡a 1850 U.S. Census, Ohio ‡b (John Smith, age 30, b. 1820)
670    ‡a Death certificate, 1891 ‡b (d. March 15, 1891, age 71)
```

### MARC21 Field Breakdown

#### Field 100: Established Heading (Personal Name)

**Indicators**:
- **1st indicator**: Type of personal name entry
  - `0` = Forename (e.g., "Leonardo da Vinci")
  - `1` = Surname (e.g., "Smith, John")
  - `3` = Family name (e.g., "Smith family")

**Subfields** (most relevant for genealogy):
- `‡a` = Personal name (required)
- `‡b` = Numeration (e.g., "III", "Jr.")
- `‡c` = Titles and other words (e.g., "Sir", "Jr.")
- `‡d` = Dates (life dates: "1820-1891")
- `‡q` = Fuller form of name (e.g., "(John)")

#### Field 400: See From Tracing (Variant Names)

**Purpose**: Links unauthorized forms to the established heading

**Example Use Cases**:
```
400 1_ ‡a Smith, John               -- Without dates
400 1_ ‡a Smith, J.                 -- Abbreviated
400 1_ ‡a Smith, Johnny ‡d 1820-1891 -- Nickname
400 1_ ‡a Schmidt, Johann ‡d 1820-1891  -- Original immigrant name
400 0_ ‡a John the blacksmith       -- Known-as name
```

**Search Behavior**: When user searches for "John the blacksmith", they find the canonical "Smith, John, 1820-1891" record.

#### Field 500: See Also From Tracing (Related Authorities)

**Purpose**: Links between established headings (relationships)

**Subfield ‡w** (Control subfield) - Position 0 = Special relationship:
- `r` = Relationship (general)
- `g` = Broader term (parent concept)
- `h` = Narrower term (child concept)
- `i` = Relationship information (textual)

**Subfield ‡i**: Relationship designator
- `Spouse:`
- `Parent:`
- `Child:`
- `Sibling:`

**Genealogical Examples**:
```
500 1_ ‡w r ‡i Spouse: ‡a Smith, Mary, ‡d 1822-1895 ‡0 (authid)234567
500 1_ ‡w r ‡i Father: ‡a Smith, William, ‡d 1790-1870 ‡0 (authid)234568
500 1_ ‡w r ‡i Mother: ‡a Jones, Elizabeth, ‡d 1795-1875 ‡0 (authid)234569
500 1_ ‡w r ‡i Son: ‡a Smith, William Jr., ‡d 1845-1920 ‡0 (authid)234570
```

**‡0 Subfield**: Authority record control number (authid) - enables programmatic relationship traversal

#### Field 670: Source Data Found

**Purpose**: Evidence citations

```
670 ‡a 1850 U.S. Census, Summit County, Ohio ‡b (John Smith, age 30)
670 ‡a Marriage record, 1845, Summit County ‡b (John Smith m. Mary Jones)
670 ‡a Death certificate, 1891-03-15 ‡b (John Smith, age 71, b. 1820)
670 ‡a Smith family Bible ‡b (John Smith born March 1820)
```

**Genealogical Value**: Built-in source citation system!

---

## Part 3: Code Architecture & Implementation Patterns

### Module Structure

```
C4::AuthoritiesMarc                    -- Core authority operations
├── AddAuthority()                     -- Create/update authority
├── ModAuthority()                     -- Modify + merge
├── DelAuthority()                     -- Delete + cleanup
├── GetAuthority()                     -- Retrieve by authid
├── merge()                            -- Authority merging
└── FindDuplicateAuthority()           -- Duplicate detection

Koha::Authority                        -- Object-oriented wrapper
├── get_usage_count()                  -- Count linked biblios
├── linked_biblionumbers()             -- Get all linked records
└── (inherits from Koha::Object)

Koha::Authorities                      -- Collection class
└── (class methods for search/retrieval)

C4::Linker                             -- Bibliographic → Authority linking
├── get_link()                         -- Find matching authority
└── update_cache()                     -- Link caching

Koha::SearchEngine::Elasticsearch      -- Search integration
└── Koha::Filter::MARC::EmbedSeeFromHeadings  -- Variant embedding
```

### Core Function: AddAuthority

**Signature**:
```perl
my $authid = AddAuthority($record, $authid, $authtypecode, $params);
```

**Implementation Pattern**:

```perl
sub AddAuthority {
    my ($record, $authid, $authtypecode, $params) = @_;

    # 1. Generate new authid if creating
    unless ($authid) {
        $authid = Koha::Authority->new({
            authtypecode => $authtypecode,
            datecreated => dt_from_string(),
        })->store->authid;
    }

    # 2. Update MARC metadata fields
    my $timestamp = dt_from_string()->ymd('') .
                    dt_from_string()->hms('');
    $record->field('005')->update($timestamp);

    # 3. Ensure proper heading format (MARC21 vs UNIMARC)
    if (C4::Context->preference('marcflavour') eq 'MARC21') {
        # Specific MARC21 processing
        my $heading = C4::AuthoritiesMarc::MARC21::get_heading($record);
        # ... normalization
    }

    # 4. Store record
    my $authority = Koha::Authority->find($authid);
    $authority->marc($record->as_usmarc());
    $authority->marcxml($record->as_xml_record());
    $authority->update();

    # 5. Index for search (unless skipped)
    unless ($params->{skip_index}) {
        index_authority_record($authid);
    }

    return $authid;
}
```

**Key Design Patterns**:

1. **Idempotent**: Same function for create and update
2. **Atomic Metadata**: Field 005 auto-updated
3. **Format Abstraction**: MARC flavor handled internally
4. **Dual Storage**: Binary MARC + XML stored together
5. **Deferred Indexing**: Optional skip for bulk operations

### Core Function: merge()

**Purpose**: Merge authority records and update all linked bibliographic records

**Signature**:
```perl
merge({
    mergefrom => $authid_old,
    MARCfrom => $record_old,
    mergeto => $authid_new,
    MARCto => $record_new,
    biblionumbers => \@biblios,      # Optional: specific records
    override_limit => 0,              # Override AuthorityMergeLimit
});
```

**Implementation Flow**:

```perl
sub merge {
    my ($params) = @_;

    # 1. Determine which biblios to update
    my @biblionumbers;
    if ($params->{biblionumbers}) {
        # Explicit list provided
        @biblionumbers = @{ $params->{biblionumbers} };
    } elsif ($params->{override_limit}) {
        # Get all linked records
        @biblionumbers = Koha::Authorities->linked_biblionumbers({
            authid => $params->{mergefrom}
        });
    } else {
        # Check merge limit
        my $max = C4::Context->preference('AuthorityMergeLimit') // 0;
        my $hits = Koha::Authorities->get_usage_count({
            authid => $params->{mergefrom}
        });

        if ($hits > 0 && $hits <= $max) {
            # Within limit: merge now
            @biblionumbers = Koha::Authorities->linked_biblionumbers({
                authid => $params->{mergefrom}
            });
        } elsif ($hits > $max) {
            # Exceeds limit: postpone to cron job
            Koha::Authority::MergeRequest->new({
                authid => $params->{mergefrom},
                oldrecord => $params->{MARCfrom},
                authid_new => $params->{mergeto},
            })->store;
            return;
        }
    }

    # 2. Update each bibliographic record
    foreach my $biblionumber (@biblionumbers) {
        my $biblio = Koha::Biblios->find($biblionumber);
        my $record = $biblio->metadata->record;

        # Find all authority-controlled fields
        foreach my $field ($record->fields()) {
            next unless $field->tag() =~ /^(1|6|7)/;  # Authority fields

            my $authid_subfield = $field->subfield('9');
            next unless $authid_subfield;
            next unless $authid_subfield == $params->{mergefrom};

            # Update the linking subfield
            $field->update('9' => $params->{mergeto});

            # Optionally update heading text from new authority
            my $new_heading = get_heading_from_authority($params->{MARCto});
            $field->update('a' => $new_heading);
        }

        # 3. Save updated biblio
        ModBiblio($record, $biblionumber);
    }

    # 4. Log the action
    if (C4::Context->preference("AuthoritiesLog")) {
        logaction("AUTHORITIES", "MERGE", $params->{mergefrom},
                  "Merged to $params->{mergeto}");
    }
}
```

**Merge Strategies**:

1. **Immediate Merge**: For authorities with few linked records
2. **Deferred Merge**: For heavily-used authorities (queued for cron job)
3. **Selective Merge**: Update specific biblios only
4. **Audit Trail**: All merges logged

**Genealogical Application**:
- Merge duplicate person records
- Transfer all source citations to canonical person
- Preserve research trail via logging

### Core Function: linked_biblionumbers()

**Implementation**:

```perl
sub linked_biblionumbers {
    my ($class, $params) = @_;

    my $authid = $params->{authid};
    my $max = $params->{max_results} || 0;
    my $offset = $params->{offset} || 0;

    # Query search engine for records linking to this authority
    my $searcher = Koha::SearchEngine::Elasticsearch->new({
        index => 'biblios'
    });

    # Search for biblio records with this authid in $9 subfield
    my $query = "an:$authid";  # Authority number index
    my $results = $searcher->simple_search_compat($query, $offset, $max);

    # Extract biblionumbers from results
    my @biblionumbers;
    foreach my $result (@{$results->{records}}) {
        my $record = $result->{record};
        my $biblionumber = $record->field('999')->subfield('c');
        push @biblionumbers, $biblionumber;
    }

    return @biblionumbers;
}
```

**Performance Optimization**:
- Searches index, not full table scan
- Supports pagination (offset + max)
- Returns IDs only (lightweight)

**Genealogical Use**:
```rust
// RP-GPS equivalent
impl Person {
    fn linked_sources(&self) -> Vec<SourceId> {
        // All sources mentioning this person
        self.search_engine.query(
            format!("person_id:{}", self.id)
        ).collect()
    }
}
```

---

## Part 4: Authority Linking System

### The C4::Linker Module

**Purpose**: Automatically link bibliographic fields to authority records

**System Preferences**:

| Preference | Values | Behavior |
|------------|--------|----------|
| **LinkerModule** | Default, FirstMatch, LastMatch | Matching strategy |
| **LinkerRelink** | Do, Do not | Relink if better match found |
| **LinkerKeepStale** | Do, Do not | Preserve broken links |
| **AutoCreateAuthorities** | Create, Do not create | Auto-create missing authorities |
| **IncludeSeeFromInSearches** | Include, Don't include | Search variant names |
| **IncludeSeeAlsoFromInSearches** | Include, Don't include | Search related terms |

### Linking Workflow

**Step 1: Identify Authority-Controlled Fields**

MARC21 bibliographic fields that link to authorities:

```
100 = Main Entry - Personal Name     → PERSO_NAME authority
110 = Main Entry - Corporate Name    → CORPO_NAME authority
111 = Main Entry - Meeting Name      → MEETI_NAME authority
600 = Subject - Personal Name        → PERSO_NAME authority
610 = Subject - Corporate Name       → CORPO_NAME authority
650 = Subject - Topical Term         → TOPIC_TERM authority
651 = Subject - Geographic Name      → GEOGR_NAME authority
700 = Added Entry - Personal Name    → PERSO_NAME authority
```

**Step 2: Extract Heading for Matching**

```perl
sub get_heading_for_linking {
    my ($field) = @_;

    # Build heading from subfields
    my $heading = '';
    $heading .= $field->subfield('a') if $field->subfield('a');  # Main
    $heading .= ', ' . $field->subfield('b') if $field->subfield('b');  # Numeration
    $heading .= ' ' . $field->subfield('c') if $field->subfield('c');  # Title
    $heading .= ', ' . $field->subfield('d') if $field->subfield('d');  # Dates
    $heading .= ' (' . $field->subfield('q') . ')' if $field->subfield('q');  # Fuller

    # Normalize: lowercase, strip punctuation
    $heading = lc($heading);
    $heading =~ s/[[:punct:]]//g;
    $heading =~ s/\s+/ /g;
    $heading =~ s/^\s+|\s+$//g;

    return $heading;
}
```

**Step 3: Search for Matching Authority**

```perl
sub find_matching_authority {
    my ($heading, $authtypecode) = @_;

    # Search authority index
    my $searcher = Koha::SearchEngine::Elasticsearch->new({
        index => 'authorities'
    });

    # Match-heading field includes 1XX + 4XX (established + variants)
    my $query = qq{
        match-heading:"$heading"
        AND authtypecode:$authtypecode
    };

    my $results = $searcher->search($query);

    if ($results->{total} == 1) {
        # Perfect match
        return $results->{records}[0]->{authid};
    } elsif ($results->{total} > 1) {
        # Multiple matches: use LinkerModule preference
        my $strategy = C4::Context->preference('LinkerModule');
        if ($strategy eq 'FirstMatch') {
            return $results->{records}[0]->{authid};
        } elsif ($strategy eq 'LastMatch') {
            return $results->{records}[-1]->{authid};
        } else {
            # Default: no link (ambiguous)
            return undef;
        }
    } else {
        # No match
        if (C4::Context->preference('AutoCreateAuthorities')) {
            # Create new authority on-the-fly
            return create_authority_from_heading($heading, $authtypecode);
        }
        return undef;
    }
}
```

**Step 4: Update Bibliographic Field**

```perl
sub link_biblio_field_to_authority {
    my ($field, $authid) = @_;

    # Add/update $9 subfield with authid
    if ($field->subfield('9')) {
        $field->update('9' => $authid);
    } else {
        $field->add_subfields('9' => $authid);
    }

    # Optionally: Update heading from authority (normalize)
    my $authority = Koha::Authorities->find($authid);
    my $auth_record = $authority->record;
    my $auth_heading = get_established_heading($auth_record);

    # Update main subfield
    $field->update('a' => $auth_heading->{a});
    $field->update('d' => $auth_heading->{d}) if $auth_heading->{d};

    return 1;
}
```

### Cron Job: link_bibs_to_authorities.pl

**Purpose**: Batch linking for existing records

```bash
# Link all unlinked bibliographic records
./misc/link_bibs_to_authorities.pl --verbose

# Relink all records (use better authorities)
./misc/link_bibs_to_authorities.pl --relink

# Link specific bib numbers
./misc/link_bibs_to_authorities.pl --bib 123,456,789
```

**Genealogical Application**:

```bash
# After importing census records, link all person mentions to authorities
./link_sources_to_persons.pl --source-type "census" --relink
```

---

## Part 5: Search Architecture with Authority Embedding

### The EmbedSeeFromHeadings Filter

**Purpose**: Index variant names (4XX fields) into bibliographic records for broader search

**System Preference**: `IncludeSeeFromInSearches`

**How It Works**:

1. **Authority Record**:
```
100 1_ ‡a Smith, John, ‡d 1820-1891   -- Established
400 1_ ‡a Schmidt, Johann              -- Variant
```

2. **Bibliographic Record Links to Authority**:
```
600 10 ‡a Smith, John, ‡d 1820-1891 ‡9 123456
```

3. **During Indexing**: Filter extracts variants from authority 123456
4. **Index Contains**:
   - "Smith, John, 1820-1891" (from biblio field)
   - "Schmidt, Johann" (embedded from authority 4XX)

5. **Search Behavior**:
   - User searches: "Schmidt, Johann"
   - Result includes biblio (even though it says "Smith, John")

**Implementation**:

```perl
package Koha::Filter::MARC::EmbedSeeFromHeadings;

sub filter {
    my ($self, $record) = @_;

    # Find all authority-linked fields
    foreach my $field ($record->fields()) {
        next unless $field->tag() =~ /^(1|6|7)/;

        my $authid = $field->subfield('9');
        next unless $authid;

        # Get authority record
        my $authority = Koha::Authorities->find($authid);
        next unless $authority;

        my $auth_record = $authority->record;

        # Extract 4XX (see from) fields
        my @variants = $auth_record->field('4..');

        foreach my $variant_field (@variants) {
            # Create virtual field for indexing
            my $virtual_field = $field->clone();

            # Replace heading with variant
            $virtual_field->update('a' => $variant_field->subfield('a'));
            $virtual_field->update('d' => $variant_field->subfield('d'))
                if $variant_field->subfield('d');

            # Add to record (temporarily, for indexing only)
            $record->append_fields($virtual_field);
        }
    }

    return $record;
}
```

**Benefits for Genealogy**:

- Researcher searches for "Johann Schmidt" (immigrant name)
- Finds sources citing "John Smith" (Americanized name)
- Authority linking makes the connection transparent

### Elasticsearch Index Structure

**Index: authorities**

```json
{
  "mappings": {
    "properties": {
      "authid": { "type": "long" },
      "authtypecode": { "type": "keyword" },
      "match-heading": {
        "type": "text",
        "fields": {
          "raw": { "type": "keyword" }
        }
      },
      "see-from": { "type": "text" },       // 4XX fields
      "see-also-from": { "type": "text" },  // 5XX fields
      "heading": { "type": "text" },        // 1XX field
      "authid-cross-indexes": { "type": "long" },  // Related authorities
      "marc-xml": { "type": "text", "index": false }
    }
  }
}
```

**Query Example**:

```json
{
  "query": {
    "bool": {
      "should": [
        { "match": { "match-heading": "Smith, John" }},
        { "match": { "see-from": "Schmidt, Johann" }},
        { "match": { "see-also-from": "Smith family" }}
      ]
    }
  }
}
```

---

## Part 6: Architectural Patterns for ResearchProcess-GPS

### Pattern 1: Dual Storage (MARC + Native Format)

**Koha Approach**:
```sql
auth_header:
    marc BLOB           -- Binary MARC21
    marcxml LONGTEXT    -- XML MARC21
```

**RP-GPS Adaptation**:
```rust
pub struct Person {
    pub person_id: Uuid,
    pub canonical_data: PersonCanonical,  // Native Rust struct
    pub marc_export: Option<Vec<u8>>,     // Optional MARC21 binary
    pub marc_xml: Option<String>,         // Optional MARC21 XML
}

pub struct PersonCanonical {
    pub given_name: String,
    pub surname: String,
    pub birth_date: Option<GenealogyDate>,
    pub death_date: Option<GenealogyDate>,
    pub variant_names: Vec<VariantName>,
    pub relationships: Vec<PersonRelationship>,
}
```

**Benefits**:
- Native format optimized for RP-GPS operations
- MARC export enables library system integration
- Round-trip capability (import/export)

### Pattern 2: Variant Name Handling

**Koha Pattern** (MARC 4XX fields):

```
100 1_ ‡a Smith, John, ‡d 1820-1891
400 1_ ‡a Schmidt, Johann
400 1_ ‡a Smith, J.
400 1_ ‡a Smith, Johnny
```

**RP-GPS Implementation**:

```rust
pub struct VariantName {
    pub variant_id: Uuid,
    pub person_id: Uuid,          // References canonical person
    pub name_type: VariantNameType,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub notes: Option<String>,
    pub source_id: Option<Uuid>,  // Where this variant was found
    pub created_at: DateTime<Utc>,
}

pub enum VariantNameType {
    Birth,              // Birth name
    Married,            // Married name
    Nickname,           // Known as
    Immigration,        // Name change on immigration
    Spelling,           // Spelling variation
    Translation,        // Language translation
    Abbreviation,       // Abbreviated form
    Pseudonym,          // Pen name
}

impl Person {
    pub fn add_variant(&mut self, variant: VariantName) {
        // Ensure no duplicates
        if !self.has_variant(&variant) {
            self.variants.push(variant);
        }
    }

    pub fn matches_name(&self, search_name: &str) -> bool {
        // Check canonical name
        if self.canonical_name_matches(search_name) {
            return true;
        }

        // Check all variants
        self.variants.iter().any(|v| v.matches(search_name))
    }
}
```

### Pattern 3: Authority Linking (Subfield $9)

**Koha Pattern**:

Bibliographic field:
```
600 10 ‡a Smith, John, ‡d 1820-1891 ‡9 123456
```

**RP-GPS Implementation**:

```rust
pub struct SourcePerson {
    pub source_person_id: Uuid,
    pub source_id: Uuid,
    pub person_id: Option<Uuid>,     // Link to canonical Person
    pub person_name_in_source: String,
    pub role: String,                // "subject", "author", "witness"
    pub certainty: Certainty,
    pub extraction_date: DateTime<Utc>,
}

impl Source {
    pub fn link_mention_to_person(&mut self,
        mention_id: Uuid,
        person_id: Uuid,
        certainty: Certainty
    ) -> Result<()> {
        let mention = self.find_mention_mut(mention_id)?;

        // Link to canonical person
        mention.person_id = Some(person_id);
        mention.certainty = certainty;

        // Update person's source list
        Person::add_source_mention(person_id, mention_id)?;

        Ok(())
    }
}
```

### Pattern 4: Relationship Modeling

**Koha Pattern** (MARC 5XX + borrower_relationships):

```
500 1_ ‡w r ‡i Spouse: ‡a Smith, Mary ‡0 234567
500 1_ ‡w r ‡i Father: ‡a Smith, William ‡0 234568
```

**RP-GPS Implementation**:

```rust
pub struct PersonRelationship {
    pub relationship_id: Uuid,
    pub person_1_id: Uuid,
    pub person_2_id: Uuid,
    pub relationship_type: RelationshipType,
    pub certainty: Certainty,
    pub source_id: Option<Uuid>,
    pub notes: Option<String>,
}

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
    Guardian,
    Ward,
}

impl Person {
    pub fn get_relationships(&self, rel_type: RelationshipType) -> Vec<PersonRelationship> {
        self.relationships.iter()
            .filter(|r| r.relationship_type == rel_type)
            .cloned()
            .collect()
    }

    pub fn get_family_network(&self, depth: usize) -> FamilyGraph {
        // Build relationship graph to specified depth
        // Similar to Koha's authtrees traversal
        FamilyGraph::build(self.person_id, depth)
    }
}
```

### Pattern 5: Merge Operations

**Koha Pattern**:

```perl
merge({
    mergefrom => $duplicate_authid,
    mergeto => $canonical_authid,
    override_limit => 1,
});
```

**RP-GPS Implementation**:

```rust
pub struct PersonMergeOperation {
    pub merge_id: Uuid,
    pub from_person_id: Uuid,      // Duplicate
    pub to_person_id: Uuid,        // Canonical
    pub merge_strategy: MergeStrategy,
    pub performed_by: UserId,
    pub performed_at: DateTime<Utc>,
    pub reversible: bool,
}

pub enum MergeStrategy {
    KeepAll,           // Keep all data from both
    PreferCanonical,   // Prefer "to" person's data
    PreferSource,      // Prefer "from" person's data
    Manual,            // User-specified for each field
}

impl Person {
    pub fn merge(
        from_id: Uuid,
        to_id: Uuid,
        strategy: MergeStrategy
    ) -> Result<MergeOperation> {

        // 1. Retrieve both persons
        let from_person = Person::find(from_id)?;
        let to_person = Person::find(to_id)?;

        // 2. Transfer all source mentions
        for mention in from_person.source_mentions() {
            mention.update_person_id(to_id)?;
        }

        // 3. Merge variant names
        for variant in from_person.variants {
            if !to_person.has_variant(&variant) {
                to_person.add_variant(variant)?;
            }
        }

        // 4. Merge relationships
        for rel in from_person.relationships {
            // Update relationship to point to canonical person
            if rel.person_1_id == from_id {
                rel.person_1_id = to_id;
            } else if rel.person_2_id == from_id {
                rel.person_2_id = to_id;
            }
            to_person.add_relationship(rel)?;
        }

        // 5. Archive the duplicate person
        from_person.archive(format!("Merged into {}", to_id))?;

        // 6. Log the operation
        let merge_op = MergeOperation {
            merge_id: Uuid::new_v4(),
            from_person_id: from_id,
            to_person_id: to_id,
            strategy,
            performed_at: Utc::now(),
        };
        merge_op.save()?;

        Ok(merge_op)
    }
}
```

### Pattern 6: Usage Tracking

**Koha Pattern**:

```perl
my $count = Koha::Authorities->get_usage_count({ authid => $id });
my @biblios = Koha::Authorities->linked_biblionumbers({ authid => $id });
```

**RP-GPS Implementation**:

```rust
impl Person {
    pub fn source_citation_count(&self) -> usize {
        SourcePerson::count_by_person(self.person_id)
    }

    pub fn linked_sources(&self) -> Vec<Source> {
        Source::find_by_person_id(self.person_id)
    }

    pub fn source_timeline(&self) -> Vec<SourceCitation> {
        // Chronological list of all sources mentioning this person
        self.linked_sources()
            .into_iter()
            .flat_map(|s| s.citations_for_person(self.person_id))
            .sorted_by(|a, b| a.date.cmp(&b.date))
            .collect()
    }
}
```

---

## Part 7: Key Architectural Lessons

### 1. Separation of Canonical vs. Mentions

**Koha Clarity**:
- **Authority** = Canonical entity (THE person)
- **Bibliographic field** = Mention in source (A mention of the person)

**RP-GPS Must Maintain**:
- **Person** table = Canonical individuals
- **SourcePerson** table = Mentions/citations in sources
- **Clear linking**: SourcePerson.person_id → Person.person_id

### 2. Variant Handling is Critical

**Koha's 4XX Fields Teach Us**:
- Store ALL name variants systematically
- Link variants to canonical form
- Search must include variants automatically
- Track WHERE each variant was encountered

**RP-GPS Implementation**:
- VariantName table with source attribution
- Full-text search index includes all variants
- UI clearly shows canonical vs. variant names

### 3. Relationship Modeling Needs Two Layers

**Koha Uses**:
1. **MARC 5XX**: Loose relationships (textual)
2. **borrower_relationships**: Strict relationships (database FK)

**RP-GPS Should Use**:
1. **PersonRelationship** table: Definite relationships
2. **Research notes**: Hypothesized relationships
3. **Clear distinction**: Proven vs. theoretical

### 4. Merge Operations are Essential

**Koha Handles**:
- Duplicate detection
- Progressive merging (limit-based)
- Deferred merging (cron job for large sets)
- Audit logging

**RP-GPS Must Implement**:
- Person duplicate detection algorithm
- Merge wizard (UI-guided)
- Reversible merges (archive, don't delete)
- Complete audit trail

### 5. Search Must Embed Authority Data

**Koha's EmbedSeeFromHeadings Pattern**:
- During indexing, pull authority variants into biblio index
- Single search returns results for canonical + all variants
- User doesn't need to know about authority system

**RP-GPS Elasticsearch Strategy**:
```json
{
  "source_index": {
    "person_canonical_name": "Smith, John",
    "person_id": "uuid-123",
    "person_variant_names": [
      "Schmidt, Johann",
      "Smith, J.",
      "Johnny Smith"
    ],
    "person_birth_year": 1820,
    "person_death_year": 1891
  }
}
```

### 6. Metadata Timestamps are Critical

**Koha's modification_time Field**:
- Auto-updated on every change
- Enables sync detection
- Merge conflict resolution
- Incremental indexing

**RP-GPS Must Have**:
```rust
pub struct Person {
    pub person_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,  // Auto-update trigger
    pub created_by: UserId,
    pub last_modified_by: UserId,
}
```

### 7. Plugin/Module System Lifecycle

**Koha's Pattern** (from limited research):
- `install()`: First-time setup
- `upgrade()`: Version migration
- `uninstall()`: Cleanup
- `store_data()` / `retrieve_data()`: Plugin persistence

**RP-GPS Has This!**:
- Already implemented in WASM/native module system
- `ModuleLifecycle` trait
- Plugin data storage via module SDK

**No Changes Needed** - RP-GPS module system already follows best practices!

---

## Part 8: Recommendations for RP-GPS

### Immediate Actions (Week 3-4)

1. **Create Person Entity** (analogous to PERSO_NAME authority)
   - Canonical person records
   - person_id as primary key
   - created_at, updated_at timestamps

2. **Create VariantName Table**
   - All name variants for each person
   - Type classification (birth, married, nickname, etc.)
   - Source attribution (where variant was encountered)

3. **Create PersonRelationship Table**
   - person_1_id, person_2_id, relationship_type
   - Certainty and source attribution
   - Support for complex relationships

4. **Implement SourcePerson Linking**
   - Link source mentions to canonical Person records
   - Like Koha's subfield $9 pattern
   - Maintain certainty/confidence for each link

### Phase 2 Enhancements (Weeks 5-12)

5. **Search Index Authority Embedding**
   - When indexing sources, embed person variant names
   - Enable searches for "Johann Schmidt" to find "John Smith" sources
   - Implement like Koha's EmbedSeeFromHeadings

6. **Person Merge Operations**
   - Duplicate detection algorithm
   - Merge wizard UI
   - Transfer all SourcePerson links
   - Merge variant names and relationships
   - Archive (don't delete) merged persons

7. **Usage Tracking Methods**
   - `Person::source_count()` - how many sources mention this person
   - `Person::linked_sources()` - retrieve all sources
   - `Person::timeline()` - chronological source view

### Phase 3: MARC Integration (Weeks 13-16)

8. **MARC21 Authority Export**
   - Person → MARC21 authority record
   - Variants → 4XX fields
   - Relationships → 5XX fields
   - Source citations → 670 fields

9. **MARC21 Authority Import**
   - Parse incoming MARC21 authority
   - Create/update Person records
   - Import variants and relationships

10. **Koha Plugin Development** (Optional Future)
    - If institutional demand exists
    - Bidirectional sync with Koha installations
    - Authority-based cataloging for genealogical archives

---

## Part 9: Technical Specifications for Person Entity

### Database Schema

```sql
-- Core Person table (canonical individuals)
CREATE TABLE persons (
    person_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Canonical Name Components
    given_name VARCHAR(255),
    surname VARCHAR(255),
    middle_name VARCHAR(255),
    prefix VARCHAR(50),        -- Dr., Rev., etc.
    suffix VARCHAR(50),        -- Jr., III, etc.

    -- Vital Dates (using genealogy date type)
    birth_date JSONB,          -- {year, month, day, certainty, circa}
    death_date JSONB,

    -- Location
    birth_place_id UUID REFERENCES places(place_id),
    death_place_id UUID REFERENCES places(place_id),

    -- Metadata
    sex VARCHAR(10),           -- Male, Female, Unknown
    notes TEXT,

    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(user_id),
    last_modified_by UUID REFERENCES users(user_id),

    -- MARC Export Cache
    marc_export BYTEA,         -- Binary MARC21
    marc_xml TEXT,             -- XML MARC21
    marc_updated_at TIMESTAMPTZ
);

-- Auto-update trigger
CREATE TRIGGER update_persons_updated_at
BEFORE UPDATE ON persons
FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Indexes
CREATE INDEX idx_persons_surname ON persons(surname);
CREATE INDEX idx_persons_given_name ON persons(given_name);
CREATE INDEX idx_persons_birth_year ON persons((birth_date->>'year'));
CREATE INDEX idx_persons_updated_at ON persons(updated_at);

-- Variant Names
CREATE TABLE person_variant_names (
    variant_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES persons(person_id) ON DELETE CASCADE,

    -- Variant Name Components
    given_name VARCHAR(255),
    surname VARCHAR(255),
    middle_name VARCHAR(255),
    full_name VARCHAR(500),    -- Searchable combined form

    -- Variant Type
    variant_type VARCHAR(50) NOT NULL,  -- birth, married, nickname, etc.

    -- Attribution
    source_id UUID REFERENCES sources(source_id),
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(user_id)
);

CREATE INDEX idx_variant_names_person ON person_variant_names(person_id);
CREATE INDEX idx_variant_names_full ON person_variant_names(full_name);

-- Person Relationships
CREATE TABLE person_relationships (
    relationship_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_1_id UUID NOT NULL REFERENCES persons(person_id),
    person_2_id UUID NOT NULL REFERENCES persons(person_id),

    relationship_type VARCHAR(50) NOT NULL,  -- parent, child, spouse, etc.

    -- Evidence
    certainty SMALLINT CHECK (certainty >= 1 AND certainty <= 5),
    source_id UUID REFERENCES sources(source_id),
    notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_by UUID REFERENCES users(user_id),

    CONSTRAINT different_persons CHECK (person_1_id != person_2_id)
);

CREATE INDEX idx_relationships_person1 ON person_relationships(person_1_id);
CREATE INDEX idx_relationships_person2 ON person_relationships(person_2_id);
CREATE INDEX idx_relationships_type ON person_relationships(relationship_type);

-- Source Person Mentions (linking table)
CREATE TABLE source_persons (
    source_person_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_id UUID NOT NULL REFERENCES sources(source_id),
    person_id UUID REFERENCES persons(person_id),  -- Null if unlinked

    -- Name as it appears in source
    name_in_source VARCHAR(500) NOT NULL,
    role VARCHAR(100),         -- subject, author, witness, etc.

    -- Linking metadata
    certainty SMALLINT CHECK (certainty >= 1 AND certainty <= 5),
    linked_at TIMESTAMPTZ,
    linked_by UUID REFERENCES users(user_id),

    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_source_persons_source ON source_persons(source_id);
CREATE INDEX idx_source_persons_person ON source_persons(person_id);
CREATE INDEX idx_source_persons_name ON source_persons(name_in_source);

-- Person Merge Log
CREATE TABLE person_merges (
    merge_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_person_id UUID NOT NULL,  -- Archived person
    to_person_id UUID NOT NULL REFERENCES persons(person_id),

    merge_strategy VARCHAR(50),
    performed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    performed_by UUID REFERENCES users(user_id),

    -- Reversibility
    reversible BOOLEAN DEFAULT TRUE,
    reversed_at TIMESTAMPTZ,
    reversed_by UUID REFERENCES users(user_id),

    notes TEXT
);

CREATE INDEX idx_merges_from ON person_merges(from_person_id);
CREATE INDEX idx_merges_to ON person_merges(to_person_id);
```

### Rust Data Structures

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub person_id: Uuid,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub birth_date: Option<GenealogyDate>,
    pub death_date: Option<GenealogyDate>,
    pub birth_place_id: Option<Uuid>,
    pub death_place_id: Option<Uuid>,
    pub sex: Option<Sex>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub last_modified_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sex {
    Male,
    Female,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenealogyDate {
    pub year: Option<i32>,
    pub month: Option<u8>,
    pub day: Option<u8>,
    pub certainty: DateCertainty,
    pub circa: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateCertainty {
    Exact,
    Estimated,
    Calculated,
    Before,
    After,
    Between(i32, i32),  // Range
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantName {
    pub variant_id: Uuid,
    pub person_id: Uuid,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_name: Option<String>,
    pub full_name: String,
    pub variant_type: VariantNameType,
    pub source_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariantNameType {
    Birth,
    Married,
    Nickname,
    Immigration,
    Spelling,
    Translation,
    Abbreviation,
    Pseudonym,
    Legal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRelationship {
    pub relationship_id: Uuid,
    pub person_1_id: Uuid,
    pub person_2_id: Uuid,
    pub relationship_type: RelationshipType,
    pub certainty: Certainty,
    pub source_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Certainty {
    Definite = 5,
    Probable = 4,
    Possible = 3,
    Uncertain = 2,
    Speculative = 1,
}

impl Person {
    /// Get canonical name in standard format
    pub fn canonical_name(&self) -> String {
        let mut parts = Vec::new();

        if let Some(prefix) = &self.prefix {
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
        if let Some(suffix) = &self.suffix {
            parts.push(suffix.clone());
        }

        parts.join(" ")
    }

    /// Get all variant names for this person
    pub async fn variants(&self, db: &PgPool) -> Result<Vec<VariantName>> {
        sqlx::query_as::<_, VariantName>(
            "SELECT * FROM person_variant_names WHERE person_id = $1 ORDER BY created_at"
        )
        .bind(self.person_id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Add a variant name
    pub async fn add_variant(
        &self,
        db: &PgPool,
        variant: VariantName
    ) -> Result<VariantName> {
        sqlx::query_as::<_, VariantName>(
            r#"
            INSERT INTO person_variant_names
            (person_id, given_name, surname, middle_name, full_name,
             variant_type, source_id, notes, created_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#
        )
        .bind(&self.person_id)
        .bind(&variant.given_name)
        .bind(&variant.surname)
        .bind(&variant.middle_name)
        .bind(&variant.full_name)
        .bind(&variant.variant_type)
        .bind(&variant.source_id)
        .bind(&variant.notes)
        .bind(&variant.created_by)
        .fetch_one(db)
        .await
        .map_err(Into::into)
    }

    /// Get all relationships for this person
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
    pub async fn get_relations(
        &self,
        db: &PgPool,
        rel_type: RelationshipType
    ) -> Result<Vec<Person>> {
        sqlx::query_as::<_, Person>(
            r#"
            SELECT p.* FROM persons p
            JOIN person_relationships r ON
                (r.person_1_id = $1 AND r.person_2_id = p.person_id) OR
                (r.person_2_id = $1 AND r.person_1_id = p.person_id)
            WHERE r.relationship_type = $2
            "#
        )
        .bind(self.person_id)
        .bind(rel_type)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Count source citations for this person
    pub async fn source_count(&self, db: &PgPool) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM source_persons WHERE person_id = $1"
        )
        .bind(self.person_id)
        .fetch_one(db)
        .await?;

        Ok(count.0)
    }

    /// Get all sources mentioning this person
    pub async fn linked_sources(&self, db: &PgPool) -> Result<Vec<Source>> {
        sqlx::query_as::<_, Source>(
            r#"
            SELECT s.* FROM sources s
            JOIN source_persons sp ON sp.source_id = s.source_id
            WHERE sp.person_id = $1
            ORDER BY s.source_date
            "#
        )
        .bind(self.person_id)
        .fetch_all(db)
        .await
        .map_err(Into::into)
    }

    /// Merge another person into this one
    pub async fn merge_from(
        &self,
        db: &PgPool,
        from_person_id: Uuid,
        strategy: MergeStrategy,
        user_id: Uuid
    ) -> Result<PersonMerge> {
        let mut tx = db.begin().await?;

        // 1. Transfer all source mentions
        sqlx::query(
            "UPDATE source_persons SET person_id = $1 WHERE person_id = $2"
        )
        .bind(self.person_id)
        .bind(from_person_id)
        .execute(&mut *tx)
        .await?;

        // 2. Copy variant names (if not duplicates)
        sqlx::query(
            r#"
            INSERT INTO person_variant_names
            (person_id, given_name, surname, middle_name, full_name,
             variant_type, source_id, notes, created_by)
            SELECT $1, given_name, surname, middle_name, full_name,
                   variant_type, source_id, notes, $3
            FROM person_variant_names
            WHERE person_id = $2
            ON CONFLICT DO NOTHING
            "#
        )
        .bind(self.person_id)
        .bind(from_person_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

        // 3. Update relationships
        sqlx::query(
            r#"
            UPDATE person_relationships
            SET person_1_id = $1
            WHERE person_1_id = $2
            "#
        )
        .bind(self.person_id)
        .bind(from_person_id)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            r#"
            UPDATE person_relationships
            SET person_2_id = $1
            WHERE person_2_id = $2
            "#
        )
        .bind(self.person_id)
        .bind(from_person_id)
        .execute(&mut *tx)
        .await?;

        // 4. Log the merge
        let merge = sqlx::query_as::<_, PersonMerge>(
            r#"
            INSERT INTO person_merges
            (from_person_id, to_person_id, merge_strategy, performed_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(from_person_id)
        .bind(self.person_id)
        .bind(strategy)
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await?;

        // 5. Delete the merged person
        sqlx::query("DELETE FROM persons WHERE person_id = $1")
            .bind(from_person_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(merge)
    }
}
```

---

## Part 10: Conclusion

Koha's authority control architecture represents **25 years of battle-tested evolution** in managing canonical entities, variant names, and complex relationships. The system's core patterns—dual storage formats, systematic variant handling, automatic linking, hierarchical relationships, and production-grade merge operations—provide a proven blueprint for ResearchProcess-GPS Person entity design.

### Key Takeaways

1. **Authority vs. Mention**: Clear separation between canonical person records and source citations
2. **Variant Names are First-Class**: Not afterthoughts, but core to the data model
3. **Linking is Automatic**: System handles authority linking, not manual user effort
4. **Search Includes Variants**: Index embedding makes variant search transparent
5. **Merge Operations are Critical**: Real-world data always has duplicates
6. **Audit Everything**: Timestamps, modification tracking, merge logs

### Next Steps

**Week 3 Tasks**:
1. Review Koha plugin system patterns (for RP-GPS module enhancement)
2. Analyze Elasticsearch integration architecture
3. Document specific plugin lifecycle hooks

**Week 4 Deliverables**:
1. RP_GPS_PERSON_ENTITY_DESIGN.md (comprehensive specification)
2. MARC_ADAPTER_SPECIFICATION.md (export/import architecture)
3. Implementation roadmap for Phases 2-3

---

**Document Status**: Week 2 analysis complete
**Confidence Level**: High - based on comprehensive source review
**Next Document**: RP_GPS_PERSON_ENTITY_DESIGN.md (Week 4)

---

**References**:
- Koha GitHub Repository: https://github.com/Koha-Community/Koha
- C4::AuthoritiesMarc module documentation
- MARC21 Authority Format specification (Library of Congress)
- Koha database schema documentation
- Elasticsearch integration wiki
- ByWater Solutions educational resources

---

*This deep dive provides the technical foundation for implementing ResearchProcess-GPS Person entities using proven patterns from 25 years of library science best practices.*
