# GRAMPS to Layer 1 Entity Mapping Specification

This document provides detailed mapping rules for transforming GRAMPS XML data into ResearchProcess-GPS Layer 1 entities.

## Fundamental Transformation Principles

1. **Nothing is Certain**: All imported data starts as hypotheses, not conclusions
2. **Evidence Floats**: Evidence is not locked to specific persons
3. **Relationships are Theories**: Family connections need verification
4. **Generate Questions**: Every import creates research opportunities

## Entity Mapping Rules

### GRAMPS Person → IdentityPersona

```xml
<!-- GRAMPS -->
<person handle="_abc123" change="1234567890" id="I0001">
  <gender>M</gender>
  <name type="Birth Name">
    <first>John</first>
    <surname>Smith</surname>
  </name>
  <eventref hlink="_event1" role="Primary"/>
  <citationref hlink="_citation1"/>
</person>
```

```rust
// ResearchProcess-GPS
IdentityPersona {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    state: IdentityState::Hypothesis, // NOT Concluded!
    identity_type: IdentityType::Named,
    primary_name: "John Smith",
    alternative_names: vec![], // Will be enriched from name records
    description: Some("Imported from GRAMPS I0001"),
    evidence_references: vec![], // Will be populated from citations
    theory_refs: vec![], // Will link to generated research questions
    geographic_scope: None, // Will be derived from events
    temporal_scope: None, // Will be derived from dates
    priority: 3, // Medium priority default
    notes: Some("IMPORTED: Requires verification of all facts and relationships"),
}
```

**Key Transformations**:
- State is HYPOTHESIS, not CONCLUDED
- GRAMPS ID preserved in description/notes
- Citations become evidence references
- Events are NOT directly attached

### GRAMPS Event → Evidence (not Fact!)

```xml
<!-- GRAMPS -->
<event handle="_event1" change="1234567890" id="E0001">
  <type>Birth</type>
  <dateval val="1850-01-01"/>
  <place hlink="_place1"/>
  <citationref hlink="_citation1"/>
</event>
```

```rust
// ResearchProcess-GPS
Evidence {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    evidence_type: EvidenceType::Extracted,
    description: "Birth event from GRAMPS E0001",
    source_ref: None, // Will be linked to source
    extracted_facts: vec![
        ExtractedFact {
            fact_type: "Birth".to_string(),
            value: FactValue::Date {
                date: "1850-01-01",
                precision: DatePrecision::Day,
            },
            location: Some(location_ref),
            confidence: 0.0, // Unverified
        }
    ],
    quality_assessment: None,
    transcription: None,
    notes: Some("IMPORTED: Event requires source verification"),
}
```

**Key Transformations**:
- Events become Evidence, not Facts
- Evidence must be verified before becoming Facts
- Multiple interpretations possible

### GRAMPS Family → Theory + Proposed Relationships

```xml
<!-- GRAMPS -->
<family handle="_fam1" change="1234567890" id="F0001">
  <father hlink="_abc123"/>
  <mother hlink="_def456"/>
  <childref hlink="_ghi789"/>
  <eventref hlink="_marriage1" role="Family"/>
</family>
```

```rust
// ResearchProcess-GPS

// 1. Create a Theory for the family unit
Theory {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    state: TheoryState::Proposed,
    question: "Verify Smith Family Unit F0001",
    hypothesis: "These individuals formed a family unit as documented",
    supporting_entities: vec![father_id, mother_id, child_id],
    evidence_refs: vec![], // Will be populated
    analysis_refs: vec![],
    notes: Some("IMPORTED: Family structure requires verification"),
}

// 2. Create Proposed Relationships
Relationship {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    state: RelationshipState::Proposed, // Not Verified!
    participant_a: father_id,
    participant_b: mother_id,
    relationship_type: RelationshipType::Partnership("marriage-proposed"),
    evidence_refs: vec![],
    confidence_refs: vec![],
    notes: Some("IMPORTED: Relationship requires documentary evidence"),
}
```

**Key Transformations**:
- Families become Theories about family units
- Relationships are PROPOSED, not verified
- Each relationship needs evidence

### GRAMPS Source → Source + Evidence Quality

```xml
<!-- GRAMPS -->
<source handle="_src1" change="1234567890" id="S0001">
  <stitle>1850 United States Federal Census</stitle>
  <sauthor>U.S. Census Bureau</sauthor>
  <spubinfo>National Archives</spubinfo>
</source>
```

```rust
// ResearchProcess-GPS
Source {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    state: SourceState::Active,
    source_type: SourceType::Collection,
    title: "1850 United States Federal Census",
    author: Some("U.S. Census Bureau"),
    publisher: Some("National Archives"),
    quality: SourceQuality {
        source_class: SourceClass::Original,
        information_class: InformationClass::Primary,
        evidence_class: EvidenceClass::Direct,
        credibility: 0.8,
    },
    hierarchy: SourceHierarchy {
        parent_source: None, // Could link to repository
        child_sources: vec![],
    },
}
```

### GRAMPS Citation → Citation + Evidence Extraction

```xml
<!-- GRAMPS -->
<citation handle="_cit1" change="1234567890" id="C0001">
  <sourceref hlink="_src1"/>
  <page>Page 42, Line 15</page>
  <confidence>4</confidence>
</citation>
```

```rust
// ResearchProcess-GPS
Citation {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    state: CitationState::Full,
    citing_entity: entity_id, // What's being cited
    citing_type: CitingEntityType::Evidence,
    source_ref: source_id,
    location_in_source: "Page 42, Line 15",
    accessed_date: Some(Utc::now()),
    purpose: CitationPurpose::Evidence,
    quality: CitationQuality::from_gramps_confidence(4),
}
```

### GRAMPS Place → Location

```xml
<!-- GRAMPS -->
<place handle="_place1" change="1234567890" id="P0001">
  <ptitle>Boston, Massachusetts, USA</ptitle>
  <coord long="-71.0589" lat="42.3601"/>
</place>
```

```rust
// ResearchProcess-GPS
Location {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    name: "Boston, Massachusetts, USA",
    location_type: LocationType::City,
    coordinates: Some(Coordinates {
        latitude: 42.3601,
        longitude: -71.0589,
        precision: CoordinatePrecision::Exact,
    }),
    hierarchical_path: vec!["USA", "Massachusetts", "Boston"],
    historical_names: vec![],
    time_period: None,
}
```

## Research Question Generation Rules

For every imported entity, generate appropriate research questions:

### Person → Identity Questions
1. "Verify identity of [name] from GRAMPS import"
2. "Research alternative names/spellings for [name]"
3. "Locate primary evidence for [name]'s existence"
4. "Investigate possible duplicate identities for [name]"

### Family → Relationship Questions
1. "Verify marriage between [person A] and [person B]"
2. "Confirm parent-child relationship: [parent] and [child]"
3. "Research family timeline and locations"
4. "Investigate additional family members"

### Event → Evidence Questions
1. "Find primary source for [event type] of [person]"
2. "Verify date and location of [event]"
3. "Research context and circumstances of [event]"
4. "Check for conflicting evidence about [event]"

### Missing Data → Gap Questions
1. "Find birth record for [person without birth event]"
2. "Locate death information for [person without death]"
3. "Research parents of [person without parents]"
4. "Investigate spouse for [unmarried adult]"

## Confidence and Analysis Generation

### Auto-Generated Confidence Containers

For each imported identity:
```rust
Confidence {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    assessments: vec![
        ConfidenceAssessment {
            methodology: "GRAMPS Import Assessment",
            analysis: "Imported data requires verification",
            dimensions: HashMap::from([
                ("source_verification", 0.0),
                ("evidence_correlation", 0.0),
                ("conflict_resolution", 0.0),
            ]),
            assessor_id: SYSTEM_ASSESSOR_ID,
            date: Utc::now(),
        }
    ],
    coverage: ResearchCoverage {
        geographic: HashSet::new(),
        temporal: (None, None),
        record_types: HashSet::new(),
        repositories: HashSet::new(),
    },
    gps_elements: HashMap::from([
        ("exhaustive_search", GPSElementStatus::NotStarted),
        ("complete_citations", GPSElementStatus::Partial),
        ("analysis", GPSElementStatus::NotStarted),
        ("conflict_resolution", GPSElementStatus::NotStarted),
        ("written_conclusion", GPSElementStatus::NotStarted),
    ]),
}
```

### Auto-Generated Analysis Placeholders

For complex imports, create analysis frameworks:
```rust
Analysis {
    metadata: EntityMetadata::new(IMPORT_RESEARCHER_ID),
    title: "Import Analysis: GRAMPS Database",
    analysis_type: AnalysisType::Custom,
    scope_description: "Analyze imported GRAMPS data for research opportunities",
    methodology: vec![AnalysisMethodology::Correlation],
    analytical_points: vec![
        AnalyticalPoint::new(
            "Data completeness assessment",
            AnalysisMethodology::Statistical,
        ),
        AnalyticalPoint::new(
            "Relationship verification needs",
            AnalysisMethodology::Correlation,
        ),
    ],
}
```

## State Management Rules

### Initial States After Import

| GRAMPS Entity | Layer 1 Entity | Initial State |
|---------------|----------------|---------------|
| Person | IdentityPersona | Hypothesis |
| Family | Theory | Proposed |
| Family | Relationship | Proposed |
| Event | Evidence | Unverified |
| Source | Source | Active |
| Citation | Citation | Full |
| Place | Location | Active |

### State Progression Requirements

**IdentityPersona: Hypothesis → Working**
- At least one evidence reference added
- Research question created
- Initial analysis started

**IdentityPersona: Working → Concluded**
- Minimum 3 evidence items
- All evidence verified
- Conflicts resolved
- Analysis completed

**Relationship: Proposed → Verified**
- Documentary evidence located
- Both participants verified
- Date/location confirmed
- No unresolved conflicts

## Import Statistics Tracking

Track liberation metrics:
```rust
pub struct ImportStatistics {
    // Source counts
    pub gramps_persons: usize,
    pub gramps_families: usize,
    pub gramps_events: usize,
    pub gramps_sources: usize,
    
    // Liberation counts
    pub identities_created: usize,
    pub theories_generated: usize,
    pub relationships_proposed: usize,
    pub evidence_extracted: usize,
    pub questions_generated: usize,
    
    // Quality metrics
    pub persons_without_sources: usize,
    pub events_without_dates: usize,
    pub relationships_without_evidence: usize,
    pub duplicate_person_candidates: usize,
}
```

## Error Handling and Edge Cases

### Missing Required Data
- Person without name → IdentityType::Anonymous
- Event without date → Evidence with uncertain temporal scope
- Family without parents → Theory about sibling group
- Citation without source → Create placeholder source

### Data Conflicts
- Multiple birth events → Multiple evidence items, create analysis
- Conflicting death dates → Generate conflict resolution theory
- Impossible relationships → Flag for immediate research

### Large Imports
- Batch processing in transactions
- Progress reporting
- Resumable imports
- Memory-efficient streaming

---

This mapping ensures that GRAMPS data is truly liberated into a research-oriented model where nothing is taken as fact and everything becomes an opportunity for genealogical investigation.