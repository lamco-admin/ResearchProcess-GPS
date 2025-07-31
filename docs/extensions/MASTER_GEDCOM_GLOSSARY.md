# Master GEDCOM Glossary

This glossary covers GEDCOM 7 core terms, Citation extensions (dthaler), and our new extensions for the unified genealogy format project.

## Naming Conventions for Extension Repositories

Based on established patterns:
- **dthaler**: `gedcom-citations` (Citation extensions for Evidence Explained and other styles)
- **Proposed pattern**: `gedcom-[feature]`

Suggested repository names:
- `gedcom-occurrences` - For the OCUR extension
- `gedcom-evidence` - For evidence/persona extensions
- `gedcom-research` - For research process extensions
- `gedcom-analysis` - For analysis/correlation extensions

## Core GEDCOM 7 Terms

### Record Types
- **HEAD**: Header record containing metadata about the file
- **TRLR**: Trailer record marking end of file
- **INDI**: Individual person record
- **FAM**: Family record linking parents and children
- **SOUR**: Source record describing source materials
- **REPO**: Repository record for source locations
- **OBJE**: Multimedia object record
- **SNOTE**: Shared note record
- **SUBM**: Submitter record

### Event/Attribute Tags
- **EVEN**: Generic event (requires TYPE)
- **BIRT**: Birth event
- **DEAT**: Death event
- **MARR**: Marriage event (in FAM)
- **CENS**: Census event
- **OCCU**: Occupation attribute
- **RESI**: Residence attribute/event

### Structure Tags
- **NAME**: Personal name
- **SEX**: Sex/gender
- **DATE**: Date value
- **PLAC**: Place name
- **NOTE**: Note text
- **SOUR**: Source citation (when under other structures)
- **PAGE**: Page/location within source
- **ASSO**: Association to another person
- **RELA**: Relationship (under ASSO)

### Data Types
- **Age**: Age value (e.g., "45y", "3m", "INFANT")
- **Date**: Temporal value (exact, range, period)
- **Enum**: Enumerated value from controlled vocabulary
- **List**: Comma-separated values
- **PersonalName**: Name with surname delimiters
- **Pointer**: Cross-reference (e.g., @I1@)

### GEDCOM 7 Concepts
- **Extension**: Additional functionality via URI-based tags
- **SCHMA**: Schema declaration for extensions
- **Payload**: The main value of a structure
- **Substructure**: Child element of a structure
- **Superstructure**: Parent element that can contain a structure
- **Cardinality**: How many times a structure can appear {0:1}, {0:M}, {1:1}

## Citation Extensions (dthaler)

### Source Layering
- **_SOUR** (relocated standard): Source citation as substructure of SOUR
  - Origin: Citation methodology concept of layered citations
  - Purpose: Show source derivation (source of a source)
  - URI: Uses standard g7:SOUR (relocated)

### Field-Based Citations
- **_FIEL**: Field identifier within a source
  - Origin: Citation methodology for structured sources
  - Purpose: Identify specific fields in structured sources
  - URI: `https://github.com/dthaler/gedcom-citations/_FIEL`
  
- **_FIEL-TEXT**: Text found in a field
  - Origin: Evidence Explained
  - Purpose: Capture exact text from source field
  - URI: `https://github.com/dthaler/gedcom-citations/_FIEL-TEXT`

### Citation Templates
- **_TPLT**: Citation template reference
  - Origin: Standardized citation format models
  - Purpose: Reference standard citation formats
  - URI: `https://github.com/dthaler/gedcom-citations/_TPLT`

- **_TPLT-SOUR**: Template-based source record
  - Origin: Evidence Explained
  - Purpose: Source record using template
  - URI: `https://github.com/dthaler/gedcom-citations/_TPLT-SOUR`

### Date Extensions
- **_DATE** (relocated standard): Date as substructure
  - Origin: Need for dates on more structures
  - Purpose: Add dates to names, sources, etc.
  - URI: Uses standard g7:DATE (relocated)

## Our New Extensions

### Occurrence Extension (Event Independence)
- **_OCUR**: Occurrence record (top-level)
  - Origin: GEDCOM X events + GRAMPS event model
  - Purpose: Independent events with multiple participants
  - URI: `https://gedcom.io/terms/v7/_OCUR`
  - Etymology: "Occurrence" distinguishes from embedded "Event"

- **_PART**: Participant in occurrence
  - Origin: GEDCOM X event roles
  - Purpose: Identify participant and role
  - URI: `https://gedcom.io/terms/v7/_PART`
  - Payload: Pointer to INDI or FAM

- **_OCREF**: Occurrence reference
  - Origin: Need to link from person to occurrence
  - Purpose: Person/family participation in occurrence
  - URI: `https://gedcom.io/terms/v7/_OCREF`
  - Payload: Pointer to _OCUR

### Evidence/Persona Extension (Planned)
- **_PERS**: Persona record
  - Origin: GEDCOM X persona concept
  - Purpose: Evidence/extracted record
  - Etymology: "Persona" = evidence-level person

- **_EVID**: Evidence reference
  - Origin: GEDCOM X evidence references
  - Purpose: Link conclusion to supporting evidence
  - Payload: Pointer to evidence record

- **_EXTR**: Extracted marker
  - Origin: GEDCOM X extracted flag
  - Purpose: Mark record as extracted from source
  - Payload: Y/N

- **_CONF**: Confidence level
  - Origin: GEDCOM X confidence
  - Purpose: Confidence in evidence/conclusion
  - Payload: 0-4 scale

### Research Process Extension (Planned)
- **_RSRCH**: Research activity
  - Origin: Genealogical research methodology
  - Purpose: Document research activities

- **_ANAL**: Analysis document
  - Origin: Evidence analysis methodology
  - Purpose: Document reasoning about evidence

- **_CORR**: Correlation
  - Origin: Record linkage methodology
  - Purpose: Document correlation between records

## Controlled Vocabularies

### OCUR Roles
Based on GEDCOM X and GRAMPS:
- Principal: Primary person in occurrence
- Witness: Observer/informant
- Officiator: Person performing ceremony
- Spouse/Bride/Groom: Marriage participants
- Parent/Child: Family relationships
- Godparent: Spiritual parent
- Beneficiary: Recipient in will/probate
- Informant: Information provider

### Confidence Levels
- 0: Unreliable evidence
- 1: Questionable reliability
- 2: Secondary evidence
- 3: Primary evidence
- 4: Direct evidence

### Evidence Types
- Extracted: Taken directly from source
- Concluded: Derived from analysis
- Negative: Absence of expected evidence

## URI Patterns

### Extension URIs
- GEDCOM 7 Standard: `https://gedcom.io/terms/v7/[TAG]`
- Our Extensions: `https://gedcom.io/terms/v7/[_TAG]`
- Citation Extensions: `https://github.com/dthaler/gedcom-citations/[_TAG]`
- Future: `https://github.com/gedcom-[feature]/[_TAG]`

### Type URIs
- FamilySearch IDs: `https://gedcom.io/exid-type/FamilySearch-[Type]Id`
- GRAMPS handles: `https://gramps-project.org/handle`
- Custom types: `https://example.org/types/[type]`