# Revolutionary Concepts - 2025-07-30 19:00 EEST

## 1. Identity/Persona → Person Progression

### Identity/Persona (Same Entity, Dual-Named)
```python
class IdentityPersona(NestableBaseEntity):
    """
    Non-conclusive state: a reference to someone in evidence
    Can be called Identity OR Persona based on context
    """
    state: IdentityState
    # REFERENCE → WORKING → HYPOTHESIS → CANDIDATE
    
    # From evidence
    evidence_references: List[EvidenceReference]
    
    # Research work
    possible_matches: List[UUID]  # Other identities
    research_notes: str
    
    # Can nest indefinitely for organization
    # "Bob" contains "Bobby", "Robert", etc.
```

### Person (Conclusive State)
```python
class Person(NestableBaseEntity):
    """
    Concluded individual - promoted from Identity/Persona
    Has additional properties not available to Identity
    """
    state: PersonState
    # CONCLUDED → ACCEPTED → PUBLISHED → CHALLENGED
    
    # Source identities that created this Person
    source_identities: List[UUID]
    
    # Additional properties only for Persons
    verified_vital_events: List[UUID]
    confirmed_relationships: List[UUID]
    validated_timeline: Timeline
    
    # Can be demoted back to Identity if challenged
    demotion_history: List[DemotionEvent]
```

## 2. Revolutionary Confidence System

### Confidence as First-Class Entity
```python
class Confidence(NestableBaseEntity):
    """
    Not a number but a complex, versioned analysis
    Can be attached to ANY other entity
    """
    # What this confidence applies to
    target_entity: UUID
    target_type: str
    
    # The confidence container
    assessments: List[ConfidenceAssessment]
    
    # Versioning
    version: int
    previous_versions: List[UUID]
    
    # Special types
    evidence_type: EvidenceType
    # POSITIVE - normal evidence
    # NEGATIVE - absence of expected evidence  
    # DISPROVEN - evidence that disproves
    # CONFLICTING - contradictory evidence
```

### Confidence Assessment
```python
class ConfidenceAssessment:
    """One analysis within a Confidence entity"""
    assessment_id: UUID
    
    # The assessment
    methodology: str  # "Direct evidence", "Correlation", etc.
    analysis: str  # Full text explanation
    
    # Multi-dimensional scoring
    dimensions: Dict[str, float]
    # {
    #   "source_quality": 0.9,
    #   "information_quality": 0.8,
    #   "evidence_quality": 0.7,
    #   "correlation_strength": 0.85
    # }
    
    # Who and when
    assessor: str
    assessment_date: datetime
    peer_reviewed: bool
    
    # For negative/disproven
    expected_but_absent: Optional[str]
    disproof_explanation: Optional[str]
```

### Negative vs Disproven Evidence
```python
class NegativeEvidence(Evidence):
    """
    Expected evidence that doesn't exist
    Example: No death record found despite thorough search
    """
    what_was_sought: str
    where_searched: List[SearchRecord]
    search_exhaustiveness: float
    implications: List[str]
    
    confidence: Confidence  # With evidence_type = NEGATIVE

class DisprovenEvidence(Evidence):
    """
    Evidence that actively disproves something
    Example: Census shows person in different state than claimed
    """
    what_it_disproves: UUID  # Theory, claim, etc.
    disproof_mechanism: str
    
    confidence: Confidence  # With evidence_type = DISPROVEN
```

## 3. Research Process Tools

### Research Log (First-Class Entity)
```python
class ResearchLog(NestableBaseEntity):
    """
    Documents the research process itself
    Not conclusions but the journey
    """
    log_type: LogType
    # SESSION - Single research session
    # PROJECT - Entire research project
    # REPOSITORY - Visit to specific repository
    
    # Temporal
    entries: List[LogEntry]
    
    # Links to work
    theories_explored: List[UUID]
    sources_consulted: List[UUID]
    evidence_gathered: List[UUID]
    
    # Planning
    next_steps: List[PlannedAction]
    blockers: List[Blocker]
```

### Log Entry
```python
class LogEntry:
    timestamp: datetime
    action_taken: str
    
    # What was found/not found
    results: List[Result]
    
    # Reflection
    thoughts: str
    significance: str
    
    # Links
    created_entities: List[UUID]
    modified_entities: List[UUID]
```

### Genealogy Proof Statement
```python
class ProofStatement(NestableBaseEntity):
    """
    Formal GPS-compliant proof writeup
    Links all the pieces together
    """
    # The claim
    theory: UUID
    assertion: str
    
    # GPS elements
    exhaustive_search: ExhaustiveSearchRecord
    citations: List[UUID]  # Complete citations
    analysis: UUID  # Thorough analysis
    conflicts_resolved: List[ConflictResolution]
    conclusion: Conclusion
    
    # Versioning (as understanding evolves)
    version: int
    previous_versions: List[UUID]
    
    # Review
    peer_reviews: List[PeerReview]
    publication_info: Optional[PublicationInfo]
```

### Research Report Templates
```python
class ReportTemplate(NestableBaseEntity):
    """
    Reusable templates for various reports
    """
    template_type: ReportType
    # CLIENT_REPORT
    # LINEAGE_SOCIETY_APPLICATION  
    # CASE_STUDY
    # RESEARCH_PLAN
    # PROGRESS_REPORT
    
    # Structure
    sections: List[ReportSection]
    
    # Variables to fill
    required_inputs: List[TemplateVariable]
    
    # Formatting
    output_format: str  # markdown, html, pdf
    style_guide: str
```

## 4. Template System for Sources/Citations

### Source Template
```python
class SourceTemplate(NestableBaseEntity):
    """
    Based on dthaler's work but enhanced
    """
    template_uri: str
    template_name: str
    
    # Field definitions
    source_fields: List[TemplateField]  # Master source
    detail_fields: List[TemplateField]  # Citation details
    
    # Multiple format outputs
    formats: Dict[str, FormatPattern]
    # {
    #   "Chicago": "...",
    #   "MLA": "...",
    #   "Evidence Style": "..."
    # }
    
    # Quality hints
    expected_quality: SourceQuality
    
    # Relationships
    collection_template: Optional[UUID]
    child_templates: List[UUID]
```

### Dynamic Citation Building
```python
class CitationBuilder:
    """
    Builds citations from templates + data
    """
    def build_citation(
        template: SourceTemplate,
        field_values: Dict[str, str],
        format_name: str,
        element_level: Optional[List[CitedElement]] = None
    ) -> Citation:
        # Build basic citation from template
        # Add element-level if provided
        # Return Citation entity with appropriate state
```

## 5. Bringing It All Together

### Research Process Flow
```
ResearchLog (documenting process)
    ↓
Theory (what we're investigating)
    ↓
Sources (where we look)
    ↓
Evidence (what we find)
    ↓
IdentityPersona (who we find)
    ↓
Analysis (what it means)
    ↓
Confidence (how sure we are, why, evolution)
    ↓
Person (concluded individual)
    ↓
ProofStatement (GPS writeup)
    ↓
Report (shareable output)
```

### Everything Is:
- **Versioned** - Track evolution of understanding
- **Assessable** - Confidence can attach to anything
- **Nestable** - Organize however makes sense
- **Stateful** - Progress through research stages
- **Reversible** - Can always reconsider

## The Missing Pieces We're Adding:

1. **Process Documentation** - Research logs, session tracking
2. **Formal Outputs** - Proof statements, reports
3. **Complex Confidence** - Not numbers but full analyses
4. **Negative Evidence** - Explicit support for absence
5. **Disproven Evidence** - Different from just conflicting
6. **Template System** - Flexible citation building
7. **Workflow Tools** - Plans, blockers, next steps

This creates a complete research environment, not just a data model.