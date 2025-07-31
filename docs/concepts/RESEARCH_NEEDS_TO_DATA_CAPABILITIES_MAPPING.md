# Research Needs to Data Capabilities Mapping

## Overview
This document maps the identified research process needs and work products to specific data model capabilities, identifying what current systems support and what requires new structures.

## Core Data Entities Required

### 1. Research Activity Entity
**Purpose**: Track individual research actions
**Attributes**:
- activity_id (unique identifier)
- activity_type (search, analysis, correspondence, etc.)
- date_performed
- researcher_id
- repository/source_searched
- search_parameters
- time_period_searched
- result_summary
- negative_result_flag
- linked_items (sources found, people, etc.)
- notes

**Current Support**: None in GEDCOM 5.5/7

### 2. Research Plan Entity
**Purpose**: Document research objectives and strategies
**Attributes**:
- plan_id
- research_question
- creation_date
- last_modified
- status (active, completed, suspended)
- priority
- hypotheses[]
- planned_sources[]
- methodology_notes
- linked_persons[]
- linked_questions[]

**Current Support**: None in GEDCOM

### 3. Evidence Analysis Entity
**Purpose**: Structured evaluation of evidence
**Attributes**:
- analysis_id
- source_id (link to source)
- information_items[] (specific facts extracted)
- evidence_type (direct, indirect, negative)
- quality_assessment
- conflicts_identified[]
- analyst_id
- analysis_date
- conclusions[]

**Current Support**: Partial via NOTE in GEDCOM

### 4. Proof Statement Entity
**Purpose**: Document reasoning from evidence to conclusion
**Attributes**:
- proof_id
- proof_type (summary, argument)
- research_question
- evidence_items[] (ordered list)
- reasoning_text
- conclusion
- author
- creation_date
- review_status
- gps_compliance_flags

**Current Support**: NOTE in GEDCOM (unstructured)

### 5. Hypothesis Entity
**Purpose**: Track alternative theories
**Attributes**:
- hypothesis_id
- description
- status (proposed, testing, proven, disproven)
- supporting_evidence[]
- contradicting_evidence[]
- test_plan
- test_results
- linked_persons[]
- linked_relationships[]

**Current Support**: None in GEDCOM

## Relationship Mappings

### Evidence to Conclusion Chain
```
SOURCE → EVIDENCE_ANALYSIS → PROOF_STATEMENT → CONCLUSION_PERSON/FACT
   ↓           ↓                    ↓
REPOSITORY  CONFLICTS          HYPOTHESIS
```

**Required Relationships**:
- Many-to-many between sources and analyses
- One-to-many between analysis and proof statements  
- Many-to-many between hypotheses and evidence
- One-to-many between proof and conclusions

### Research Process Flow
```
RESEARCH_QUESTION → RESEARCH_PLAN → RESEARCH_ACTIVITY → FINDINGS
        ↓                ↓                  ↓
   HYPOTHESIS     PLANNED_SOURCES    ACTUAL_SOURCES
```

**Required Relationships**:
- One-to-many between plan and activities
- Many-to-many between activities and sources
- Temporal ordering of activities

## Data Capabilities Analysis

### What Current Systems Support

#### GEDCOM 7
- SOURCE records (basic)
- NOTE attachments (unstructured)
- Basic citations
- Repository links
- Simple fact/event model

#### GRAMPS
- Source hierarchy
- Note types
- Attributes for extension
- Place hierarchy
- Some research tools

#### Commercial Systems
- Varying source models
- Limited research tracking
- Task lists (some systems)
- Hints/suggestions (algorithmic)

### Critical Gaps in Current Systems

1. **No Research Process Tracking**
   - Cannot record what was searched
   - No negative search results
   - No research planning tools

2. **No Evidence Analysis Framework**
   - Cannot distinguish evidence from information
   - No quality assessment structure  
   - No conflict tracking

3. **No Hypothesis Management**
   - Cannot track alternative theories
   - No testing documentation
   - No disproven theory storage

4. **Limited Work Product Support**
   - No structured proof statements
   - No research report generation
   - No GPS compliance checking

5. **Poor Process Integration**
   - Research separate from conclusions
   - No workflow support
   - No collaboration features

## Required Data Capabilities

### 1. Temporal Tracking
- Record when research performed
- Version control for evolving hypotheses
- Activity logs with timestamps
- Historical view of research progress

### 2. Relationship Complexity
- Many-to-many relationships throughout
- Hierarchical research plans
- Network of evidence relationships
- Persona to person mappings

### 3. Status Management
- Research task states
- Hypothesis lifecycle
- Evidence quality indicators
- Proof review status

### 4. Text and Structure
- Narrative proof arguments
- Structured evidence analysis
- Template-based work products
- Rich text with citations

### 5. Collaboration Features
- Researcher attribution
- Change tracking
- Access controls
- Merge capabilities

## Implementation Approach

### Phase 1: Basic Research Tracking
```yaml
ResearchLog:
  - activity_id
  - date
  - repository
  - search_description
  - results_summary
  - linked_sources[]
  
ResearchTask:
  - task_id
  - description
  - status
  - linked_persons[]
```

### Phase 2: Evidence Analysis
```yaml
EvidenceAnalysis:
  - source_id
  - information_extracted[]
  - evidence_quality
  - relevance_assessment
  
ProofStatement:
  - question
  - evidence_items[]
  - reasoning
  - conclusion
```

### Phase 3: Full Research Process
```yaml
ResearchPlan:
  - objectives[]
  - methodology
  - sources_to_check[]
  
Hypothesis:
  - description
  - test_plan
  - evidence_for[]
  - evidence_against[]
  - status
```

## GEDCOM 7 Extension Possibilities

### Using Extension Framework
```
0 @R1@ _RESEARCH_LOG
1 _ACTIVITY @A1@
2 DATE 15 JAN 2025
2 _REPOSITORY @REPO1@
2 _SEARCH "Smith births 1850-1860"
2 _RESULT "Found 3 possible matches"
2 SOUR @S1@
2 SOUR @S2@
2 SOUR @S3@
```

### Embedded in Existing Structures
```
0 @I1@ INDI
1 NAME John /Smith/
2 _HYPOTHESIS @H1@
3 _DESC "Possibly son of William Smith"
3 _STATUS TESTING
3 _EVIDENCE_FOR @S1@
3 _EVIDENCE_FOR @S2@
```

## Database Schema Requirements

### Core Tables
- research_plans
- research_activities  
- evidence_analyses
- hypotheses
- proof_statements
- work_products

### Relationship Tables
- activity_sources
- hypothesis_evidence
- proof_evidence
- plan_activities
- person_hypotheses

### Metadata Tables
- activity_types
- evidence_quality_types
- hypothesis_statuses
- gps_compliance_criteria

## Recommendations

1. **Start with Research Logging**
   - Most immediate user need
   - Relatively simple structure
   - High value for genealogists

2. **Build Evidence Framework**
   - Critical for GPS compliance
   - Enables quality research
   - Foundation for proof

3. **Add Process Support**
   - Research planning tools
   - Hypothesis tracking
   - Workflow management

4. **Enable Collaboration**
   - Multi-user support
   - Attribution tracking
   - Merge capabilities