# Sources and Citations Redesign - 2025-07-30 18:30 EEST

## Key Insights from dthaler's Extensions

### 1. Source Derivation (_SOUR)
- Sources can be derived from other sources (copy, translation, transcript)
- Creates a chain: Original → Copy → Transcription → Translation
- Critical for understanding source quality and provenance

### 2. Source Stewardship (_DATE)
- Tracks WHO held a source WHEN
- Repository transitions over time
- Important for physical sources that move

### 3. Templates (_TPLT, _FIEL)
- Template-based citation formatting
- Field-value pairs for flexibility
- Bash-style patterns (controversial but powerful)

## Core Design Principles for RGPS

### 1. Single Entity, Multiple States

Just like Identity (PERSONA → CONCLUDED), sources and citations should have states:

```python
class Source(NestableBaseEntity):
    source_state: SourceState
    # REFERENCE → DOCUMENTED → VERIFIED → QUESTIONED
    
    # When REFERENCE: just a pointer, minimal info
    # When DOCUMENTED: full citation details
    # When VERIFIED: checked and validated
    # When QUESTIONED: issues found, needs review
```

### 2. Source Hierarchy (Not Separate Entities)

```python
class Source(NestableBaseEntity):
    source_type: SourceType
    # ITEM → SERIES → COLLECTION → REPOSITORY → SYSTEM
    
    # Examples:
    # ITEM: Single census page, one letter
    # SERIES: All 1920 census pages for a county
    # COLLECTION: All US Federal Census records
    # REPOSITORY: National Archives
    # SYSTEM: FamilySearch, Ancestry.com
```

### 3. Citation as Relationship + State

Citations aren't separate entities but relationships with state:

```python
class Citation(NestableBaseEntity):
    citation_state: CitationState
    # QUICK → FULL → ELEMENT → ANALYZED
    
    # QUICK: Just enough to find again
    # FULL: Complete formal citation
    # ELEMENT: Specific data elements cited
    # ANALYZED: Quality assessed, conflicts noted
    
    # Relationships
    from_entity: UUID  # What's being cited (Evidence, Analysis, etc.)
    to_source: UUID    # The source
    
    # Element-level when needed
    elements: List[CitedElement]  # Specific facts cited
```

### 4. Source Quality as Multi-Dimensional

Not just original/derivative but:
- **Originality**: original → copy → transcription → translation → compilation
- **Contemporaneity**: contemporary → near-contemporary → retrospective
- **Officiality**: official → quasi-official → informal → hearsay
- **Completeness**: complete → partial → fragmentary → reconstructed
- **Accessibility**: public → restricted → private → lost

### 5. The Derivation Chain

```
Physical Document (1850 Census)
    ↓ [photographed]
Microfilm (1950s)
    ↓ [digitized]
Digital Images (Ancestry.com)
    ↓ [transcribed]
Database Index
    ↓ [excerpted]
Published Book
    ↓ [translated]
Foreign Language Version
```

Each step potentially introduces errors and needs tracking.

## Revolutionary Concepts

### 1. Living Sources
- Sources that update (websites, databases)
- Version tracking built in
- Change notifications
- Wayback Machine integration

### 2. Collaborative Sources
- Multiple researchers improve source documentation
- Crowd-sourced transcriptions
- Quality ratings by community
- Shared template libraries

### 3. Evidence Extraction Tracking
```python
class EvidenceExtraction:
    source: UUID
    extraction_method: str  # manual, OCR, AI
    extractor: str  # who/what
    confidence: float
    original_text: str
    interpreted_text: str
    normalized_text: str
```

### 4. Smart Citations
- Auto-generate from source hierarchy
- Template-based but flexible
- Support all citation styles
- Element-level precision

## Practical Implementation

### Phase 1: Core Structure
```python
class Source(NestableBaseEntity):
    # Identity
    source_state: SourceState
    source_type: SourceType
    
    # Basic info
    title: str
    creator: str
    date_created: Optional[date]
    
    # Quality dimensions
    originality: OriginalityScale
    contemporaneity: ContemporaneityscaleScale
    officiality: OfficialityScale
    completeness: float  # 0-1
    
    # Access
    access_info: Dict
    last_accessed: datetime
    
    # Derivation
    derived_from: Optional[UUID]  # Parent source
    derivation_type: Optional[str]  # copy, translation, etc.
    
    # Templates (optional)
    template_uri: Optional[str]
    template_fields: Dict[str, str]  # _FIEL equivalent
```

### Phase 2: Citation Relationships
```python
class Citation(NestableBaseEntity):
    # What's citing what
    from_entity: UUID
    from_type: str  # Evidence, Analysis, Identity, etc.
    to_source: UUID
    
    # Citation details
    citation_state: CitationState
    location_in_source: str  # page, timestamp, etc.
    
    # Element-level (when ELEMENT state)
    cited_elements: List[CitedElement]
    
    # Quality
    extraction_confidence: float
    extraction_method: str
    
    # Display
    citation_text: str  # Human-readable
    citation_format: str  # Chicago, MLA, etc.
```

### Phase 3: Element-Level Citations
```python
class CitedElement:
    element_name: str  # "Age", "Name", etc.
    original_text: str  # Exactly as written
    interpreted_text: str  # What we think it says
    normalized_value: str  # Standardized
    confidence: float
    notes: str
```

## Key Advantages

1. **Flexibility**: Same entities work for simple and complex cases
2. **Evolution**: Sources and citations can improve over time
3. **Precision**: Can cite exactly what supports each conclusion
4. **Collaboration**: Multiple researchers can contribute
5. **Future-Proof**: Extensible for new source types

## Migration Path

1. Import existing sources as REFERENCE state
2. Gradually enhance to DOCUMENTED state
3. Add element-level citations where helpful
4. Build derivation chains for quality assessment

## The Vision

A system where:
- Every fact traces to its exact source
- Source quality is transparent and multi-dimensional
- Citations serve all their purposes efficiently
- Collaboration improves everyone's research
- Nothing is lost to changing URLs or access
- Element-level precision when needed, simplicity when not