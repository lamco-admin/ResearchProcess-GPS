# Cohesive Data Model - 2025-07-30 18:45 EEST

## Core Philosophy

**Everything has states, everything can nest, everything evolves through research**

## Primary Entities

### 1. Theory (User-Facing: "Research Question")
```python
class Theory(NestableBaseEntity):
    # What we're researching
    question: str  # "Who were the parents of John Smith?"
    hypothesis: str  # "John Smith's parents were James and Mary"
    
    # State progression
    state: TheoryState  # EXPLORING → TESTING → CONCLUDED → QUESTIONED
    
    # Git-like branching
    parent_theory: Optional[UUID]
    child_theories: List[UUID]
    
    # GPS compliance
    gps_compliance: GPSCompliance
```

### 2. Identity (Unified Person/Persona)
```python
class Identity(NestableBaseEntity):
    # State determines behavior
    identity_state: IdentityState
    # PERSONA → CANDIDATE → CONCLUDED → VALIDATED
    
    # When PERSONA: Evidence references, can nest freely
    # When CONCLUDED: Validated person, can have relationships
    
    # Evidence references (always maintained)
    evidence_references: List[EvidenceReference]
    
    # Relationships (only when CONCLUDED)
    relationships: List[UUID]
```

### 3. Evidence
```python
class Evidence(NestableBaseEntity):
    # What we found
    evidence_state: EvidenceState
    # RAW → PROCESSED → ANALYZED → INTEGRATED
    
    # Extraction tracking
    extracted_from: Source
    extraction_method: str
    extracted_facts: List[ExtractedFact]
```

### 4. Source (Hierarchical)
```python
class Source(NestableBaseEntity):
    # Where evidence comes from
    source_state: SourceState
    # REFERENCE → DOCUMENTED → VERIFIED → QUESTIONED
    
    source_type: SourceType
    # ITEM → SERIES → COLLECTION → REPOSITORY → SYSTEM
    
    # Quality dimensions
    originality: OriginalityScale
    contemporaneity: ContemporaneityscaleScale
    
    # Derivation chain
    derived_from: Optional[UUID]
    derivation_type: Optional[str]
```

### 5. Citation (Relationship Entity)
```python
class Citation(NestableBaseEntity):
    # How we connect evidence to sources
    citation_state: CitationState
    # QUICK → FULL → ELEMENT → ANALYZED
    
    # The connection
    from_entity: UUID  # Evidence, Identity, etc.
    to_source: UUID
    
    # Precision
    location_in_source: str
    cited_elements: List[CitedElement]  # When ELEMENT state
```

### 6. Analysis
```python
class Analysis(NestableBaseEntity):
    # How we interpret evidence
    analysis_state: AnalysisState
    # INITIAL → DETAILED → REVIEWED → ACCEPTED
    
    # What we're analyzing
    evidence_analyzed: List[UUID]
    analytical_points: List[AnalyticalPoint]
    
    # Conclusions
    conclusions: List[str]
    supports_theories: Dict[UUID, float]
```

### 7. Event
```python
class Event(NestableBaseEntity):
    # What happened
    event_state: EventState
    # HYPOTHETICAL → EVIDENCED → CONCLUDED → VERIFIED
    
    # Participants (only CONCLUDED identities)
    participants: List[EventParticipation]
    
    # Temporal
    when: TemporalExpression
    where: Optional[UUID]  # Location
```

### 8. Relationship
```python
class Relationship(NestableBaseEntity):
    # Connections between CONCLUDED identities
    relationship_state: RelationshipState
    # THEORETICAL → PROBABLE → CONCLUDED → DISPROVEN
    
    # Multi-party
    participants: List[RelationshipParticipant]
    relationship_type: RelationshipType
```

### 9. Location
```python
class Location(NestableBaseEntity):
    # Where things happened
    location_state: LocationState
    # UNCERTAIN → PROBABLE → VERIFIED → HISTORICAL
    
    # Temporal awareness
    temporal_instances: List[TemporalLocation]
    
    # Spatial hierarchy (natural nesting)
    location_type: LocationType  # address, city, county, etc.
```

## Key Patterns

### 1. State Progression
Every entity can evolve:
- Start uncertain/hypothetical
- Gather evidence
- Reach conclusions
- Question if new evidence emerges
- Revert or re-conclude

### 2. Nesting Philosophy
- **Organizational**: How we work (research collections)
- **Natural**: Inherent hierarchies (locations, sources)
- **Logical**: Grouping related items (identity variants)
- **Never for Relationships**: Use Relationship entity

### 3. Evidence-First
- Everything traces to evidence
- Evidence traces to sources
- Sources have quality dimensions
- Quality propagates but can be overridden

### 4. Promotion/Demotion
- Identities: PERSONA ↔ CONCLUDED
- Events: HYPOTHETICAL ↔ CONCLUDED
- Relationships: THEORETICAL ↔ CONCLUDED
- Maintains full history

### 5. GPS Compliance Built-In
- Reasonably exhaustive search tracking
- Complete citations with elements
- Analysis and correlation explicit
- Conflicts resolved with reasoning
- Conclusion states require compliance

## Revolutionary Features

### 1. Element-Level Precision
```python
class CitedElement:
    element_name: str  # "age"
    original_text: str  # "forty"
    interpreted: str  # "40"
    confidence: float  # 0.8
```

### 2. Living Documents
- Sources can update
- Version tracking
- Change notifications
- Collaborative improvement

### 3. Multi-Dimensional Quality
Not just "original vs copy" but:
- How original?
- How contemporary?
- How official?
- How complete?
- How accessible?

### 4. Derivation Chains
Track: Original → Microfilm → Digital → Transcription → Translation

### 5. True Collaboration
- Multiple researchers
- Different theories (branches)
- Merge when consensus
- Fork when disagreement

## Benefits

1. **Conceptual Clarity**: Entities match how genealogists think
2. **Maximum Flexibility**: Simple cases stay simple, complex cases possible
3. **Natural Workflow**: Research → Conclude → Question → Revise
4. **Complete Provenance**: Every conclusion traces to evidence
5. **Collaborative**: Multiple viewpoints, merge when ready
6. **Future-Proof**: New entity types just need states

## Next Steps

1. Implement state machines for each entity
2. Create promotion/demotion workflows  
3. Design storage adapter for Git
4. Build citation formatting system
5. Create migration tools from existing formats

## The Vision

A system where genealogists can:
- Work naturally with uncertainty
- Collaborate without conflict
- Trace every fact to its source
- Question and revise conclusions
- Share research at any stage
- Build on each other's work

All while maintaining GPS compliance and professional standards.