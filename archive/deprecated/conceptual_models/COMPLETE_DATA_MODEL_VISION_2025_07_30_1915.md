# Complete Data Model Vision - 2025-07-30 19:15 EEST

## Core Philosophy

**The system models the entire research process, not just conclusions**

## Entity Hierarchy

### 1. Research Process Entities

#### ResearchLog
- Documents the journey, not the destination
- Sessions, repository visits, thought processes
- Links to all work done
- Blockers and next steps

#### Theory (Research Question)
- What we're trying to figure out
- Branches like Git for different hypotheses
- Can have sub-theories
- States: EXPLORING → TESTING → CONCLUDED → QUESTIONED

### 2. Source & Citation Entities

#### Source
- Hierarchical: ITEM → SERIES → COLLECTION → REPOSITORY → SYSTEM
- Multi-dimensional quality assessment
- Derivation chains tracked
- Templates for consistent formatting
- States: REFERENCE → DOCUMENTED → VERIFIED

#### Citation  
- Connects any entity to sources
- States: QUICK → FULL → ELEMENT → ANALYZED
- Element-level precision when needed
- Template-driven formatting
- Can cite specific facts within sources

#### SourceTemplate
- Defines fields for types of sources
- Multiple output formats (Chicago, MLA, etc.)
- Inherited from dthaler but enhanced
- Supports source hierarchies

### 3. Evidence & Analysis Entities

#### Evidence
- What we extract from sources
- States: RAW → PROCESSED → ANALYZED → INTEGRATED
- Can be:
  - Positive (normal evidence)
  - Negative (absence of expected)
  - Disproven (contradicts claims)

#### Analysis
- Interpretation of evidence
- Correlations and conflict resolution
- States: INITIAL → DETAILED → REVIEWED → ACCEPTED
- Links evidence to theories

### 4. Identity Entities

#### IdentityPersona (dual-named: Identity OR Persona)
- Non-conclusive references to people in evidence
- Can nest infinitely (Bob → Bobby → Rob)
- States: REFERENCE → WORKING → HYPOTHESIS → CANDIDATE
- No confirmed relationships (just theoretical)

#### Person
- Concluded individual promoted from IdentityPersona
- Additional properties: verified events, confirmed relationships
- States: CONCLUDED → ACCEPTED → PUBLISHED → CHALLENGED
- Can be demoted back to IdentityPersona

### 5. Relationship & Event Entities

#### Event
- Something that happened
- States: HYPOTHETICAL → EVIDENCED → CONCLUDED
- Only CONCLUDED Persons can participate
- Compositional nesting (battle within war)

#### Relationship
- Connections between CONCLUDED Persons only
- Multi-party, culturally aware
- States: THEORETICAL → PROBABLE → CONCLUDED → DISPROVEN
- Never use nesting for relationships

### 6. Location Entity

#### Location
- Spatial and temporal awareness
- Natural hierarchy (address → city → county → state)
- States: UNCERTAIN → PROBABLE → VERIFIED
- Historical names tracked

### 7. Confidence System (Revolutionary)

#### Confidence (First-Class Entity!)
- Not a score but a container of assessments
- Can attach to ANY entity
- Versioned as understanding evolves
- Supports negative and disproven evidence

#### ConfidenceAssessment
- One analysis within a Confidence
- Multi-dimensional scoring
- Full explanations
- Peer reviewable
- Dates and assessor tracked

### 8. Output Entities

#### ProofStatement
- GPS-compliant writeup
- Links theory, evidence, analysis, conclusion
- Versioned as understanding evolves
- Peer reviewable

#### ReportTemplate
- Reusable formats for various outputs
- Client reports, lineage applications, etc.
- Variable substitution
- Multiple output formats

## Key Patterns

### 1. Everything Has States
```
Working States → Conclusive States → Questioned States
     ↓                   ↓                    ↓
  Can promote      Can be accepted      Can demote
```

### 2. Dual Nature Entities
- **IdentityPersona**: Called Identity OR Persona contextually
- **Theory**: Called Research Question by users
- **Citation**: Both pointer and formal reference

### 3. Confidence Is Complex
- Not `confidence: 0.8`
- But `confidence: [Assessment1, Assessment2, ...]`
- Each assessment has methodology, explanation, multi-dimensional scores
- Whole books worth of confidence reasoning

### 4. Templates Drive Consistency
- Sources use templates for citations
- Reports use templates for output
- Templates can inherit and nest
- Based on dthaler's work but extended

### 5. Research Process Is First-Class
- Not just data but HOW we got it
- Research logs track the journey
- Blockers and next steps explicit
- Sessions and repository visits

## Revolutionary Features

### 1. Negative Evidence Support
```python
# Explicit absence tracking
negative_evidence = Evidence(
    evidence_type=NEGATIVE,
    what_was_sought="Death certificate",
    where_searched=["County clerk", "State archives"],
    implications=["Possibly died elsewhere"]
)
```

### 2. Disproven vs Conflicting
- **Conflicting**: Two sources disagree
- **Disproven**: Evidence actively refutes
- Different handling and implications

### 3. Living Research
- Everything versioned
- Understanding evolves
- Can always question and revise
- Full history maintained

### 4. Process Documentation
- Research logs capture thinking
- Not just what but why
- Failed searches documented
- Reasoning preserved

### 5. Element-Level Everything
- Citations can target specific facts
- Confidence per element
- Original vs interpreted text
- Extraction method tracked

## What This Enables

### For Researchers
- Work naturally with uncertainty
- Document thinking process
- Build on previous work
- Question and revise safely

### For Collaboration
- Multiple theories in parallel
- Merge when consensus
- Fork when disagreement  
- Peer review built in

### For Quality
- GPS compliance enforced
- Full audit trail
- Confidence transparent
- Sources fully documented

### For Sharing
- Generate any report format
- Element-level precision
- Complete provenance
- Professional standards

## The Complete System

```
Research Planning (Logs, Theories)
    ↓
Source Discovery (Templates, Quality)
    ↓
Evidence Extraction (Elements, Confidence)
    ↓
Identity Building (IdentityPersona nesting)
    ↓
Analysis & Correlation (Multi-dimensional)
    ↓
Conclusion (Person, GPS compliance)
    ↓
Documentation (Proof Statements, Reports)
    ↓
Review & Revision (Questioning, Demotion)
```

Every step tracked, versioned, assessable, and reversible.

## Next Steps

1. Implement Confidence as first-class entity
2. Design IdentityPersona → Person promotion
3. Create template system based on dthaler
4. Build research log infrastructure
5. Design proof statement structure
6. Create report generation system

This is not just a genealogy database - it's a complete research environment.