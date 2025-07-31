# Sources, Citations, and Repositories: A Deep Dive

## The Genealogical Source Ecosystem

### Source Models in Practice

#### 1. Evidence Explained Model (Elizabeth Shown Mills)
- **Layered Citations**: Source → Repository → Specific Location
- **Quality Assessment**: Original vs Derivative, Primary vs Secondary
- **Citation as Documentation**: Not just pointing but explaining

#### 2. GEDCOM Approach
- **SOUR** records with nested elements
- **REPO** as separate entities
- Citations can be shared or embedded
- Limited flexibility for complex sources

#### 3. FamilySearch Model
- **Source Box**: User's collection of sources
- **Memories**: Media as sources
- **Standardized Templates**: Pre-filled citation formats
- **Collaborative Sources**: Shared across users

#### 4. Ancestry.com Model
- **Source Collections**: Databases as meta-sources
- **Record Hints**: AI-suggested sources
- **Source Categories**: Birth, Death, Census, etc.
- **Image + Index**: Dual nature of sources

#### 5. Academic History Model
- **Primary Sources**: Contemporary to events
- **Secondary Sources**: Analysis and interpretation
- **Tertiary Sources**: Compilations and indexes
- **Source Criticism**: Provenance and reliability

### Repository Concepts

#### Traditional View
```
Physical Archive
  └── Collection (Papers of John Smith)
      └── Series (Correspondence)
          └── Folder (Letters 1920-1925)
              └── Item (Letter from Mary, 1922)
```

#### Modern Digital Reality
```
Website (FamilySearch, Ancestry)
  └── Database Collection (US Census Records)
      └── Specific Census (1920 US Census)
          └── State/County Subset
              └── Individual Image/Entry
```

#### Hybrid Repositories
- Physical archives with digital catalogs
- Digital images of physical documents
- Born-digital records
- User-uploaded content

### Citation Philosophies

#### 1. **Precision School**
- Every detail documented
- Reproducible path to source
- Machine-readable formats
- Version tracking for online sources

#### 2. **Pragmatic School**
- "Good enough" citations
- Focus on findability
- Simplified for sharing
- Templates and shortcuts

#### 3. **Layered Approach**
- Basic citation for display
- Full citation for documentation
- Research notes for context
- Analysis notes for interpretation

### The Citation Paradox

Citations serve multiple masters:
1. **Legal**: Proof of due diligence
2. **Academic**: Scholarly standards
3. **Practical**: Finding it again
4. **Social**: Sharing with others
5. **Analytical**: Understanding quality

### Source Relationships

#### Source Networks
```
Birth Certificate
  ├── Mentions: Parents' Marriage
  ├── References: Hospital
  ├── Created by: Doctor
  ├── Filed at: County Clerk
  └── Derived from: Hospital Records
```

#### Evidence Chains
```
Family Bible → Transcription → Published History → Online Database
(Each step potentially introduces errors)
```

#### Corroborating Sources
- Independent sources confirming same fact
- Related sources from same origin
- Conflicting sources requiring analysis

### Modern Challenges

#### 1. **Ephemeral Digital Sources**
- URLs that change
- Databases that merge
- Access that expires
- Content that updates

#### 2. **Collaborative Sources**
- WikiTree edits
- FamilySearch changes
- User-contributed indexes
- Crowd-sourced transcriptions

#### 3. **AI and Sources**
- OCR transcriptions
- Automated indexes
- Suggested matches
- Generated summaries

#### 4. **Media Sources**
- Photo metadata
- Audio interviews
- Video recordings
- Social media posts

### Design Principles for RGPS

#### 1. **Source Hierarchy**
```python
class Source(NestableBaseEntity):
    """Top-level source concept"""
    source_type: SourceType  # repository, database, document, media
    persistence: SourcePersistence  # permanent, stable, volatile
    
class Repository(Source):
    """Physical or digital repository"""
    repository_type: str  # archive, library, website
    access_info: Dict
    
class Collection(Source):
    """Group of related sources"""
    collection_type: str
    parent_repository: Optional[UUID]
    
class SourceItem(Source):
    """Individual source document/record"""
    media_type: str  # text, image, audio, video
    original_date: Optional[date]
```

#### 2. **Citation Flexibility**
```python
class Citation(NestableBaseEntity):
    """Flexible citation system"""
    citation_type: CitationType  # reference, source, inline
    
    # For source citations
    source_hierarchy: List[UUID]  # [repository, collection, item]
    location_detail: str  # page, timestamp, etc.
    
    # For reference citations
    from_entity: UUID
    to_entity: UUID
    relationship: str  # "mentions", "extracted_from", etc.
    
    # Common to all
    citation_text: str  # Human-readable
    citation_data: Dict  # Machine-readable
    accessed_date: datetime
    validity_period: Optional[timedelta]
```

#### 3. **Evidence Extraction**
```python
class EvidenceExtraction:
    """How we get from source to evidence"""
    source_citation: Citation
    extraction_method: str  # manual, OCR, AI
    extracted_text: str
    normalized_text: str
    confidence: float
    extractor: str  # who/what extracted
```

#### 4. **Source Quality**
```python
class SourceQuality:
    """Multi-dimensional quality assessment"""
    originality: Scale  # original, derivative, compiled
    contemporaneity: Scale  # contemporary, near, distant
    credibility: Scale  # official, informal, hearsay
    completeness: float  # 0-1
    legibility: float  # 0-1
    bias_assessment: str
```

### Revolutionary Concepts for RGPS

#### 1. **Living Citations**
- Citations that update when sources change
- Version tracking for online sources
- Wayback Machine integration
- Change notifications

#### 2. **Citation Networks**
- Sources citing other sources
- Citation analysis (like academic papers)
- Impact tracking (how often used)
- Quality propagation

#### 3. **Collaborative Citations**
- Multiple researchers improve citations
- Citation consensus building
- Quality ratings by community
- Citation templates by locality/type

#### 4. **Smart Citations**
- Auto-complete from known repositories
- Format detection and correction
- Duplicate detection
- Citation from image EXIF/metadata

#### 5. **Evidence Provenance**
```
Source → Access Event → Extraction → Evidence → Analysis → Conclusion
(Each step tracked with who, when, how, confidence)
```

### Implementation Strategies

#### Phase 1: Flexible Foundation
- Nestable source hierarchy
- Multiple citation types
- Source quality dimensions
- Basic extraction tracking

#### Phase 2: Intelligence Layer
- Duplicate detection
- Format validation
- Template matching
- Quality scoring

#### Phase 3: Collaboration
- Shared source definitions
- Citation improvement workflows
- Community quality ratings
- Template libraries

#### Phase 4: Advanced Features
- Version tracking
- Change detection
- AI-assisted extraction
- Network analysis

### The Ultimate Goal

A system where:
1. Every piece of evidence traces clearly to its source
2. Sources can be as simple or complex as needed
3. Citations serve all their purposes efficiently
4. Quality and reliability are transparent
5. Collaboration improves everyone's research
6. Nothing is lost to link rot or access changes
7. The system adapts to new source types automatically