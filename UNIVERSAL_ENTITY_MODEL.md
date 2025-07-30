# Universal Entity Model for ResearchProcess-GPS

## Overview

This document defines the fundamental entity model that captures ALL semantic meaning required for genealogical research while enabling perfect interoperability between systems. The model is designed around semantic preservation rather than traditional database optimization.

## Core Design Principles

### 1. Semantic-First Architecture
Every entity preserves not just data but the meaning, context, and reasoning behind the data.

### 2. Process-Centric Model
The model captures the research process, not just the end products, enabling full reconstruction of how conclusions were reached.

### 3. Uncertainty-Native Design
Uncertainty, confidence levels, and alternative possibilities are first-class concepts, not afterthoughts.

### 4. Immutable Audit Trail
All changes are recorded as events, creating a complete history of research evolution.

### 5. Universal Interoperability
Every entity can be mapped to any existing genealogy system with graceful degradation for unsupported features.

## Core Entity Hierarchy

```
ResearchProject
├── ResearchActivities (the process)
├── Persons (resolved individuals)
├── Identities (uncertain fragments)
├── Relationships (flexible connections)
├── Evidence (analyzed information)
├── Sources (information containers)
├── Theories (hypotheses and conclusions)
├── DNAAnalysis (genetic information)
└── Publications (outputs and deliverables)
```

## Entity Definitions

### 1. ResearchProject
**Purpose**: Top-level container for all research work on a specific genealogical question or family.

**Core Properties**:
```json
{
  "id": "uuid",
  "title": "string",
  "description": "text",
  "research_question": "text",
  "created_date": "datetime",
  "last_modified": "datetime",
  "status": "active|completed|suspended|archived",
  "primary_researcher": "person_id",
  "collaborators": ["person_id"],
  "privacy_level": "private|restricted|public",
  "gps_compliance_level": "preponderance|clear_convincing|beyond_reasonable_doubt"
}
```

**Semantic Metadata**:
- Research methodology framework used
- Quality standards applied (GPS, BCG, academic)
- Collaboration model (individual, team, peer-reviewed)
- Publication permissions and attribution requirements

### 2. ResearchActivity
**Purpose**: Records the actual research process - what was done, when, by whom, and why.

**Core Properties**:
```json
{
  "id": "uuid",
  "project_id": "uuid",
  "activity_type": "search|analysis|theory_development|peer_review|publication",
  "title": "string",
  "description": "text",
  "start_date": "datetime",
  "end_date": "datetime",
  "researcher_id": "person_id",
  "status": "planned|in_progress|completed|abandoned",
  "methodology": "systematic|targeted|exhaustive|preliminary",
  "outcome": "positive|negative|inconclusive|theory_disproven"
}
```

**Research Context**:
- Questions being investigated
- Hypotheses being tested
- Search strategies employed
- Repositories and sources consulted
- Time period and geographic scope
- Alternative approaches considered
- Reasons for abandoning if applicable

**Results Documentation**:
- Findings summary
- Evidence quality assessment
- Confidence levels achieved
- New questions raised
- Next steps recommended

### 3. Person
**Purpose**: Represents a resolved individual - someone we're confident existed as a single person.

**Core Properties**:
```json
{
  "id": "uuid",
  "confidence_level": "float (0.0-1.0)",
  "primary_name": "structured_name",
  "alternate_names": ["structured_name"],
  "birth_event": "event_id",
  "death_event": "event_id",
  "gender": "male|female|unknown|other",
  "created_from_identities": ["identity_id"],
  "merge_reasoning": "text",
  "last_reviewed": "datetime",
  "review_status": "unverified|peer_reviewed|expert_verified"
}
```

**Identity Resolution**:
- Source identities that were merged to create this person
- Confidence score for the merge decision
- Alternative merge theories considered
- Conflicting information and how it was resolved
- Evidence supporting the identity merge

**Life Context**:
- Geographic locations and movements
- Social and economic status
- Historical context and significant events
- Family and community relationships
- Professional and military service

### 4. Identity
**Purpose**: Represents a person-fragment from a single source before identity resolution.

**Core Properties**:
```json
{
  "id": "uuid",
  "source_id": "uuid",
  "source_specific_identifier": "string",
  "name_as_recorded": "string",
  "standardized_name": "structured_name",
  "confidence_in_transcription": "float",
  "possible_person_matches": [
    {
      "person_id": "uuid",
      "match_confidence": "float",
      "match_reasoning": "text"
    }
  ],
  "identity_status": "unresolved|tentatively_assigned|confidently_assigned|conflicted"
}
```

**Source Context**:
- Exact transcription from source
- Interpretation notes and alternatives
- Source quality assessment
- Contemporary context information
- Potential transcription errors noted

**Research Notes**:
- Why this identity was created
- What research questions it addresses
- How it relates to other identities
- Merge candidates and reasoning
- Unresolved questions about this identity

### 5. Relationship
**Purpose**: Captures any connection between persons, identities, or other entities with full semantic context.

**Core Properties**:
```json
{
  "id": "uuid",
  "from_entity": "uuid",
  "to_entity": "uuid",
  "relationship_type": "structured_relationship",
  "confidence_level": "float",
  "certainty_qualifier": "definite|probable|possible|alleged|disproven",
  "temporal_context": "datetime_range",
  "geographic_context": "location",
  "legal_status": "legal|biological|social|economic|religious",
  "relationship_quality": "close|distant|estranged|unknown"
}
```

**Relationship Types** (extensible):
```json
{
  "category": "family|professional|social|legal|spatial|temporal",
  "type": "parent_of|child_of|spouse_of|sibling_of|witnessed_for|business_partner_of|neighbor_of|same_person_as|possibly_same_as",
  "modifiers": ["step", "adopted", "putative", "alleged", "former", "legal_only", "biological_only"]
}
```

**Evidence and Reasoning**:
- Evidence supporting this relationship
- Alternative relationship theories considered
- Confidence evolution over time
- Peer review status
- Conflicts with other evidence

### 6. Evidence
**Purpose**: Represents analyzed information that supports or refutes genealogical conclusions.

**Core Properties**:
```json
{
  "id": "uuid",
  "source_id": "uuid",
  "information_type": "direct|indirect|negative",
  "evidence_class": "primary|secondary|tertiary",
  "information_quality": "original|derivative|authored",
  "reliability_score": "float",
  "relevance_score": "float",
  "extracted_information": "structured_data",
  "analysis_notes": "text",
  "analyst_id": "person_id",
  "analysis_date": "datetime"
}
```

**Information Analysis**:
- What specific claims this evidence supports
- Strength of support (strong, moderate, weak)
- Potential alternative interpretations
- Conflicts with other evidence
- Context needed for proper interpretation
- Assumptions required for validity

**Quality Assessment**:
- Source reliability evaluation
- Information directness assessment
- Contemporary vs. later documentation
- Potential bias or agenda considerations
- Completeness and accuracy evaluation

### 7. Source
**Purpose**: Represents containers of information with full provenance and context.

**Core Properties**:
```json
{
  "id": "uuid",
  "source_type": "document|record|artifact|oral_tradition|digital_resource",
  "title": "string",
  "author_creator": "string",
  "creation_date": "date_range",
  "publication_info": "structured_publication",
  "repository_info": "structured_repository",
  "access_info": "structured_access",
  "format": "original|photocopy|microfilm|digital|transcription|abstract",
  "condition_notes": "text",
  "reliability_assessment": "text"
}
```

**Source Context**:
- Historical context when created
- Purpose and intended audience
- Completeness and known limitations
- Relationship to other sources
- How source was discovered
- Access restrictions and permissions

**Research Value**:
- What types of information it contains
- Time periods and geographic areas covered
- Families or individuals documented
- Unique vs. derivative information
- Research potential for future investigations

### 8. Theory
**Purpose**: Captures hypotheses, conclusions, and the reasoning process behind genealogical theories.

**Core Properties**:
```json
{
  "id": "uuid",
  "theory_type": "hypothesis|working_theory|conclusion|disproven_theory",
  "title": "string",
  "description": "text",
  "confidence_level": "float",
  "evidence_strength": "preponderance|clear_convincing|beyond_reasonable_doubt",
  "status": "active|testing|proven|disproven|abandoned",
  "created_date": "datetime",
  "researcher_id": "person_id",
  "peer_review_status": "unreviewed|under_review|peer_approved|expert_verified"
}
```

**Theory Components**:
- Central claim or hypothesis
- Supporting evidence items
- Contradicting evidence and how addressed
- Alternative theories considered
- Assumptions and dependencies
- Testable predictions
- Required future research

**Reasoning Chain**:
- Logical steps from evidence to conclusion
- Quality of inference at each step
- Potential weaknesses or gaps
- Peer reviewer comments
- Evolution of theory over time

### 9. DNAAnalysis
**Purpose**: Captures genetic genealogy analysis with privacy controls and interpretation context.

**Core Properties**:
```json
{
  "id": "uuid",
  "analysis_type": "autosomal|y_dna|mtdna|x_dna",
  "test_provider": "string",
  "test_date": "date",
  "raw_data_available": "boolean",
  "privacy_level": "private|matches_only|public",
  "analysis_methodology": "text",
  "analyst_id": "person_id",
  "analysis_date": "datetime"
}
```

**Genetic Information**:
- Match groups and triangulation
- Shared segment analysis
- Relationship predictions with confidence
- Ethnicity estimates with limitations
- Chromosome mapping where applicable
- Haplogroup assignments

**Genealogical Integration**:
- How DNA supports or conflicts with paper trail
- Match theories and hypotheses
- Unresolved genetic relationships
- Research questions raised by DNA
- Privacy preferences for sharing
- Consent status for research use

### 10. Publication
**Purpose**: Represents research outputs from informal notes to peer-reviewed articles.

**Core Properties**:
```json
{
  "id": "uuid",
  "publication_type": "research_note|report|article|book|presentation|database_submission",
  "title": "string",
  "format": "text|pdf|html|presentation|structured_data",
  "audience": "personal|family|professional|academic|public",
  "publication_status": "draft|peer_review|published|archived",
  "copyright_info": "structured_copyright",
  "citation_preference": "text"
}
```

**Content Organization**:
- Research questions addressed
- Sources and evidence used
- Conclusions reached with confidence levels
- Future research suggestions
- Proper attribution for all contributors
- Version history and amendments

## Advanced Semantic Features

### 1. Confidence Modeling
Every assertion includes:
- Numerical confidence level (0.0-1.0)
- Confidence evolution over time
- Factors affecting confidence
- Sensitivity analysis for key assumptions
- Peer reviewer confidence assessments

### 2. Alternative Theories
Support for:
- Multiple working theories simultaneously
- Theory branching and merging
- Comparative evidence analysis
- Theory abandonment with reasoning
- Theory revival with new evidence

### 3. Temporal Sequences
Track:
- Research timeline and discovery order
- Information dating with uncertainty ranges
- Relationship timing and duration
- Event sequences and causation
- Historical context integration

### 4. Collaboration Model
Enable:
- Multi-researcher attribution
- Role-based permissions
- Conflict resolution mechanisms
- Peer review workflows
- Version control and merging

### 5. Quality Metrics
Quantify:
- Research thoroughness
- Evidence quality scores
- Source reliability assessments
- Analysis completeness
- Peer review status

## System Interoperability

### Mapping Strategies
Each entity includes:
- Native representation in universal model
- GEDCOM 7 mapping with extensions
- Fallback representations for limited systems
- Data loss warnings and mitigation
- Round-trip fidelity assessment

### Graceful Degradation
When full semantics aren't supported:
- Essential information preserved
- Semantic richness stored in extensions
- Reconstruction guidance provided
- Quality indicators maintained
- Loss documentation included

## Implementation Considerations

### Storage Independence
Model works with:
- Relational databases (PostgreSQL)
- Document stores (MongoDB, CouchDB)
- Graph databases (Neo4j)
- File systems (.rgps archives)
- Event streams (Kafka, EventStore)

### API Design
GraphQL schema enabling:
- Flexible querying across entities
- Real-time subscriptions for collaboration
- Batch operations for efficiency
- Version control integration
- Privacy-aware data filtering

### Performance Optimization
- UUID-based entities for distribution
- Lazy loading of semantic metadata
- Caching strategies for common queries
- Indexing for research workflows
- Archival strategies for old data

This entity model forms the foundation for universal genealogical data exchange while preserving complete semantic meaning across all research contexts.