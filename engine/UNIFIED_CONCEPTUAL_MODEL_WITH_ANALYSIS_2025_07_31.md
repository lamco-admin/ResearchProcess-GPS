# Unified Conceptual Model with Analysis - ResearchProcess-GPS
## 2025-07-31 (Updated from 2025-07-30)

## Executive Summary

ResearchProcess-GPS represents a revolutionary approach to genealogical data management where **methodologies, standards, and work products become metadata configurations** rather than hard-coded constraints. The system models the entire research process, not just conclusions.

**Major Update**: Analysis is now a first-class entity that can exist standalone or be nested within other entities, providing flexible analytical depth when needed.

## Core Philosophy

### Standards as Configuration, Not Code
```yaml
Principle: "Every standard becomes a configurable ruleset"
Implementation:
  - GPS elements → Compliance checklists
  - BCG standards → Competency metrics
  - Citation formats → Template configurations
  - Work products → Document schemas
  - Analysis methods → Configurable approaches
```

## Unified Entity Architecture

### 1. Researcher (Agent/Actor)
```python
class Researcher(NestableBaseEntity):
    """
    Person or organization conducting research.
    Can nest for teams, departments, or hierarchical organizations.
    Similar to GEDCOM X Agent but focused on research attribution.
    """
    # Identity
    name: str
    researcher_type: ResearcherType
    # INDIVIDUAL, TEAM, ORGANIZATION, SYSTEM
    
    # Professional credentials
    credentials: List[Credential] = field(default_factory=list)
    
    # Contact information
    emails: List[str] = field(default_factory=list)
    addresses: List[Address] = field(default_factory=list)
    identifiers: Dict[str, str] = field(default_factory=dict)
    
    # Organizational structure (nesting)
    parent_researcher: Optional[UUID] = None
    team_members: List[UUID] = field(default_factory=list)
    
    # Activity tracking (including analyses)
    theories_created: List[UUID] = field(default_factory=list)
    evidence_discovered: List[UUID] = field(default_factory=list)
    analyses_performed: List[UUID] = field(default_factory=list)  # NEW
    reviews_conducted: List[UUID] = field(default_factory=list)
```

### 2. Theory (User Term: "Research Question")
```python
class Theory(NestableBaseEntity):
    """
    The central organizing principle - what we're investigating
    Git-like branching for different hypotheses
    """
    # Core question/hypothesis
    question: str
    hypothesis: str
    
    # State progression
    state: TheoryState  # EXPLORING → TESTING → CONCLUDED → QUESTIONED
    
    # Standards compliance tracking
    compliance_configs: Dict[str, ComplianceConfig]
    
    # Work products generated
    work_products: Dict[str, WorkProduct]
    
    # Analyses conducted (NEW)
    analyses: List[UUID] = field(default_factory=list)
    primary_analysis: Optional[UUID] = None
    
    # Research attribution
    created_by: UUID  # Researcher ID
    contributors: List[UUID] = field(default_factory=list)
```

### 3. Analysis (NEW - First-Class Entity)
```python
class Analysis(NestableBaseEntity):
    """
    First-class entity for capturing analytical reasoning and conclusions.
    Can exist standalone or be nested within other entities.
    """
    # Core identification
    analysis_type: AnalysisType
    # IDENTITY_RESOLUTION, CONFLICT_RESOLUTION, FAMILY_RECONSTRUCTION,
    # EVIDENCE_CORRELATION, TIMELINE_ANALYSIS, PATTERN_ANALYSIS, etc.
    
    # What's being analyzed
    scope: AnalysisScope
    # {
    #   "entities": [UUID],
    #   "question": "Are these two personas the same person?",
    #   "hypothesis": "John in 1850 census is John Smith who married in 1852",
    #   "time_period": temporal_range,
    #   "geographic_scope": [location_ids]
    # }
    
    # Analytical methodology
    methodology: AnalysisMethodConfig
    # {
    #   "approach": "FAN_PRINCIPLE",
    #   "standards": ["GPS-2025"],
    #   "tools_used": ["correlation_matrix", "timeline"],
    #   "required_elements": ["cluster_identification"]
    # }
    
    # The reasoning chain
    reasoning_chain: List[ReasoningStep]
    # Each step: observation → reasoning → conclusion
    
    # Conclusions
    conclusions: List[AnalysisConclusion]
    primary_conclusion: Optional[AnalysisConclusion]
    overall_confidence: float
    
    # Alternative analyses
    alternatives: List[UUID]  # Other Analysis entities
    supersedes: Optional[UUID]  # Previous analysis this replaces
    
    # Integration points
    supports_confidence: Optional[UUID]  # Can be nested in Confidence
    supports_identity: Optional[UUID]  # Can be nested in IdentityPersona
    work_product_refs: List[UUID]  # Referenced by WorkProducts
    
    # Attribution
    analyst: UUID  # Primary Researcher
    contributors: List[UUID] = field(default_factory=list)
```

### 4. IdentityPersona (Dual-Named: Identity OR Persona)
```python
class IdentityPersona(NestableBaseEntity):
    """
    Non-conclusive state: evidence references
    Unlimited nesting for variants and research organization
    """
    state: IdentityState
    # REFERENCE → WORKING → HYPOTHESIS → CANDIDATE
    
    evidence_references: List[EvidenceReference]
    
    # Identity-specific analyses (NEW)
    identity_analyses: List[Analysis] = field(default_factory=list)
    # Can contain:
    # - Identity resolution analyses
    # - Elimination analyses (proving NOT same person)
    # - Correlation analyses with other identities
    
    # Can become Person when concluded
    promotion_eligible: bool
    promotion_criteria: PromotionConfig
    
    # Attribution
    discovered_by: UUID  # Researcher ID
```

### 5. Person (Concluded Identity)
```python
class Person(NestableBaseEntity):
    """
    Promoted from IdentityPersona when research concludes
    Additional properties only available after conclusion
    """
    state: PersonState
    # CONCLUDED → ACCEPTED → PUBLISHED → CHALLENGED
    
    source_identities: List[UUID]  # IdentityPersonas that created this
    
    # Conclusion analysis (NEW)
    conclusion_analysis: Optional[UUID]  # Analysis that justified promotion
    
    # Only Persons can have confirmed relationships
    confirmed_relationships: List[UUID]
    
    # Can be demoted if new evidence emerges
    demotion_triggers: DemotionConfig
    
    # Attribution
    concluded_by: UUID  # Researcher ID
    conclusion_date: datetime
```

### 6. Source (Hierarchical)
```python
class Source(NestableBaseEntity):
    """
    Flexible hierarchy supporting all source types
    """
    source_type: SourceType
    # ITEM → SERIES → COLLECTION → REPOSITORY → SYSTEM
    
    # Multi-dimensional quality
    quality_dimensions: SourceQuality
    
    # Source analysis (NEW)
    quality_analyses: List[UUID] = field(default_factory=list)
    # Detailed analyses of source reliability, provenance, etc.
    
    # Template-based citations
    citation_template: Optional[SourceTemplate]
    template_fields: Dict[str, str]
```

### 7. Evidence (First-Class Research Object)
```python
class Evidence(NestableBaseEntity):
    """
    What we extract from sources
    Can be positive, negative, or disproven
    """
    evidence_type: EvidenceType
    # POSITIVE, NEGATIVE, DISPROVEN
    
    # Extraction tracking
    extraction: ExtractionRecord
    
    # Evidence analysis (NEW)
    evidence_analyses: List[UUID] = field(default_factory=list)
    # Quality assessments, correlation analyses, etc.
    
    # Attribution
    extracted_by: UUID  # Researcher ID
    extraction_date: datetime
```

### 8. Citation (Flexible Relationship)
```python
class Citation(NestableBaseEntity):
    """
    Connects entities to sources with precision
    """
    citation_state: CitationState
    # QUICK → FULL → ELEMENT → ANALYZED
    
    # Flexible connections
    from_entity: UUID
    from_type: str
    to_source: UUID
    
    # Citation analysis (NEW)
    citation_analysis: Optional[UUID] = None
    # Analysis of citation quality, completeness, etc.
    
    # Element-level when needed
    cited_elements: List[CitedElement]
```

### 9. Confidence (Revolutionary Container)
```python
class Confidence(NestableBaseEntity):
    """
    Not a score but a complete research narrative
    """
    # Multiple assessments over time
    assessments: List[ConfidenceAssessment]
    
    # Supporting analyses (NEW)
    supporting_analyses: List[Analysis] = field(default_factory=list)
    # Can contain multiple Analysis objects that build confidence:
    # - Correlation analyses
    # - Conflict resolution analyses
    # - Pattern analyses
    # - Timeline analyses
    
    # Research coverage
    coverage: ResearchCoverage
    
    # GPS compliance
    gps_elements: Dict[str, GPSElementStatus]
    
    class ConfidenceAssessment:
        methodology: str
        analysis: str  # Full explanation
        dimensions: Dict[str, float]
        
        # Link to detailed analysis (NEW)
        detailed_analysis_id: Optional[UUID] = None
        
        assessor_id: UUID  # Researcher ID
        date: datetime
```

### 10. Fact (Unified Events/Attributes)
```python
class Fact(NestableBaseEntity):
    """
    Unified model for events, attributes, and characteristics
    """
    fact_class: FactClass
    # EVENT, ATTRIBUTE, CHARACTERISTIC
    
    fact_type: str  # Extensible vocabulary
    
    # Fact analysis (NEW)
    fact_analyses: List[UUID] = field(default_factory=list)
    # Timeline placement, pattern matching, etc.
    
    # Temporal and spatial data
    temporal_data: Optional[TemporalData]
    spatial_data: Optional[SpatialData]
    
    # Participants (for events)
    participants: List[FactParticipant]
```

### 11. ResearchLog (Process Documentation)
```python
class ResearchLog(NestableBaseEntity):
    """
    Documents the journey, not the destination
    Auto-captured + manual entries
    """
    log_type: LogType
    schema_config: ResearchLogConfig
    
    entries: List[LogEntry]
    
    # Analytical entries (NEW)
    analysis_entries: List[UUID] = field(default_factory=list)
    # References to Analysis entities created during research
    
    # Auto-capture integration
    browser_sessions: List[BrowserSession]
    extracted_data: List[ExtractedData]
```

## Analysis Types and Methodologies

### Analysis Types (Extensible)
```yaml
AnalysisTypes:
  Identity:
    - IDENTITY_RESOLUTION  # Same person?
    - IDENTITY_CORRELATION  # Related identities
    - IDENTITY_ELIMINATION  # NOT same person
    
  Relationships:
    - RELATIONSHIP_PROOF  # Parent-child, marriage, etc.
    - FAMILY_RECONSTRUCTION  # Build family structure
    - KINSHIP_ANALYSIS  # Extended relationships
    
  Evidence:
    - EVIDENCE_CORRELATION  # How pieces relate
    - CONFLICT_RESOLUTION  # Resolve contradictions
    - EVIDENCE_QUALITY  # Assess source/info/evidence
    - NEGATIVE_EVIDENCE  # What's NOT found
    
  Patterns:
    - MIGRATION_PATTERN  # Movement tracking
    - NAMING_PATTERN  # Cultural practices
    - CLUSTER_ANALYSIS  # FAN principle
    - SOCIAL_NETWORK  # Community relationships
    
  Chronological:
    - TIMELINE_ANALYSIS  # Temporal sequence
    - LIFE_SKETCH  # Biographical timeline
    - CHRONOLOGY_VALIDATION  # Date consistency
    
  DNA:
    - DNA_MATCH_ANALYSIS  # Genetic matches
    - DNA_TRIANGULATION  # Relationship confirmation
```

### Analysis Methodologies (Configurable)
```yaml
AnalysisMethodologies:
  FAN_PRINCIPLE:
    description: "Friends, Associates, Neighbors"
    required_elements:
      - cluster_identification
      - relationship_mapping
      - geographic_proximity
      
  TIMELINE_ANALYSIS:
    description: "Chronological sequence analysis"
    required_elements:
      - event_ordering
      - gap_identification
      - impossibility_checking
      
  EVIDENCE_ANALYSIS:
    description: "Standard evidence evaluation"
    required_elements:
      - source_criticism
      - information_assessment
      - evidence_classification
```

## How Analysis Integrates

### 1. Standalone Analysis
```python
# Major research conclusion
analysis = Analysis(
    analysis_type=AnalysisType.FAMILY_RECONSTRUCTION,
    scope={
        "entities": [persona1, persona2, persona3],
        "question": "Reconstruct Smith family 1850-1880"
    }
)
```

### 2. Nested in Confidence
```python
confidence.supporting_analyses.append(
    Analysis(
        analysis_type=AnalysisType.EVIDENCE_CORRELATION,
        scope={"question": "How do these 5 sources correlate?"}
    )
)
```

### 3. Nested in IdentityPersona
```python
persona.identity_analyses.append(
    Analysis(
        analysis_type=AnalysisType.IDENTITY_RESOLUTION,
        scope={"question": "Same as John in 1850 census?"}
    )
)
```

### 4. Referenced by WorkProducts
```python
proof_statement.supporting_analyses = [
    analysis1.id,  # Identity resolution
    analysis2.id,  # Conflict resolution
    analysis3.id   # Timeline analysis
]
```

## The Revolutionary Aspects

### 1. Everything Has States (Including Analysis)
- Research progresses through defined states
- Analysis evolves from observation to conclusion
- Can be promoted, demoted, or superseded
- History maintained throughout

### 2. Standards as Data
- No hard-coded genealogy rules
- Analysis methodologies loaded as configurations
- Multiple standards simultaneously
- Easy updates as standards evolve

### 3. Process-First Design
- Research journey documented
- Analysis captures reasoning chains
- Not just conclusions but how we got there
- Negative evidence and elimination explicit

### 4. True Flexibility
- Analysis is optional but powerful when needed
- Can be as simple or complex as required
- Unlimited nesting where sensible
- Cultural adaptability built-in

### 5. Professional-Grade Analysis
- Not simple scores or notes
- Complete reasoning chains
- Alternative interpretations
- Peer review integrated

## Key Benefits of Analysis as First-Class Entity

1. **Optional Complexity**: Simple entries don't need analysis, complex research can use extensively

2. **Flexible Placement**: Analysis can exist anywhere it's needed

3. **Methodology Agnostic**: Supports any analytical approach

4. **Process Capture**: Documents reasoning, not just conclusions

5. **Evolution Tracking**: Analysis can evolve and be superseded

6. **Professional Standards**: Matches how genealogists actually work

This unified model with Analysis provides the **infrastructure** for professional genealogical research, where analytical depth is available when needed but not forced on every data entry.