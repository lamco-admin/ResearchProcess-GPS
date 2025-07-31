# ResearchProcess-GPS Complete Conceptual Model

## Core Research Flow

```
Research Question (What we want to know)
    ↓
Evidence Gathering (Finding sources)
    ↓
Analysis (Interpreting evidence)  
    ↓
Working Hypothesis (Tentative answer)
    ↓
Testing & Validation (GPS compliance)
    ↓
Conclusion (Defensible answer)
```

## Primary Entities

### 1. Research Question
- The genealogical question we're trying to answer
- **Can have sub-questions** (hierarchical nesting)
- Has scope (identities, time period, location)
- Tracks research plan and progress
- Sub-questions help break down complex research

### 2. Evidence
- Source material (documents, records, etc.)
- Information extracted from sources
- Facts derived from information
- Can support multiple research questions
- **Now supports nesting:**
  - Document bundles (multiple pages/items from same source)
  - Evidence collections (related evidence grouped for analysis)
  - Derivative evidence (evidence derived from other evidence)
  - Evidence hierarchies (original → transcription → translation)

### 3. Analysis
- The work of interpreting evidence
- Correlating multiple pieces of evidence
- Identifying conflicts
- Proposing resolutions
- Building arguments
- **Now supports nesting:**
  - Multi-phase analysis (initial → detailed → final)
  - Analytical hierarchies (main analysis → sub-analyses)
  - Analysis revisions (original → revision 1 → revision 2)
  - Collaborative analysis (multiple analysts' work combined)

### 4. Working Hypothesis
- A tentative answer to a research question
- Based on current evidence
- Can be revised as new evidence emerges
- Multiple hypotheses can exist for same question
- Tracked within Research Questions (not standalone nested)

### 5. Conclusion
- A research question with sufficient evidence
- Meets GPS standards
- Has been peer reviewed
- Ready for publication/sharing
- Maps to traditional genealogy (persons, events, relationships)

## Supporting Entities

### Identity
- A person who might have existed
- Can have different interpretations in different working hypotheses
- Becomes a "Person" in conclusions
- **Supports sophisticated nesting:**
  - Collective identities (groups like "The Smith Brothers")
  - Temporal phases (life stages of same person)
  - Uncertain identity groups (candidates for unknown person)
  - Family placeholders ("Unknown children of...")
  - Theoretical sets (research groupings)

### Event
- Something that happened
- Not owned by persons
- Interpreted within context of research questions
- **Supports sequencing** (preceded_by, followed_by, caused_by)
- Potential for compositional nesting (events containing sub-events)

### Relationship
- Connections between identities
- Multi-party, culturally aware
- Resolved in conclusions
- Currently no explicit nesting

### Location
- Places with temporal awareness
- Change over time
- Used across all entities
- **Supports spatial hierarchy** (parent/child locations)
- Natural nesting: address → street → neighborhood → city → county → state → country

## Nesting Capabilities

### What Can Nest (ALL ENTITIES NOW NESTABLE)

1. **Research Questions** (Hierarchical)
   - Parent questions can have sub-questions
   - Helps decompose complex research problems
   - Example: "Who were the Smiths of County X?" → sub-questions for each Smith family

2. **IdentityPersona** (Most sophisticated - Logical)
   - Name variants and personas
   - Geographic variants (same person in different locations)
   - Temporal variants (person at different life stages)
   - Research groupings (possible same person candidates)
   - Unlimited depth hierarchies

3. **Person** (Categorical)
   - Family groups for organization (NOT relationships)
   - Research collections
   - Administrative groupings

4. **Evidence** (Compositional)
   - Document bundles (multiple pages/items)
   - Evidence collections for analysis
   - Derivative evidence chains
   - Evidence hierarchies (original → transcription → translation)

5. **Analysis** (Progressive)
   - Multi-phase analysis
   - Analytical hierarchies
   - Analysis revisions
   - Collaborative analysis

6. **Theory** (Versioned)
   - Theory hierarchies (main → sub-theories → experiments)
   - Theory evolution (v1 → v2 → v3)
   - Collaborative theories
   - Theory archives

7. **Event** (Compositional)
   - Complex events containing sub-events
   - Event hierarchies (war → campaign → battle)
   - Temporal sequences

8. **Relationship** (Logical)
   - Complex social structures
   - Relationship groups
   - Temporal relationships

9. **Location** (Spatial)
   - Geographic hierarchies
   - Administrative boundaries
   - Jurisdictional nesting

10. **Repository** (Hierarchical)
    - Repository hierarchies (Archive → Branch → Collection)
    - Virtual repositories
    - Collection organization

### Nesting Types

The system supports multiple types of nesting relationships:

- **Hierarchical**: Strict parent-child (locations, research questions)
- **Compositional**: Part of a whole (events within events)
- **Categorical**: Grouping by category (identity sets)
- **Temporal**: Time-based organization (life phases)
- **Spatial**: Space-based organization (locations)
- **Logical**: Research-based grouping
- **Administrative**: Organizational hierarchy

### Nesting Philosophy

- **Maximally Permissive**: No artificial limits on depth or types
- **Researcher Decides**: What makes sense for their research
- **Only Safety Constraint**: Circular reference prevention
- **Nesting ≠ Relationships**: Organization vs. genealogical connections

## Key Principles

1. **Research Process Tracking**: Everything tracks HOW we got there
2. **Evidence Independence**: Evidence exists separately from conclusions
3. **Multiple Working Hypotheses**: Can explore different interpretations
4. **GPS Compliance**: Built into the conclusion process
5. **Backwards Compatibility**: Conclusions map to traditional genealogy
6. **Flexible Nesting**: Entities can organize hierarchically where it makes sense
7. **Cultural Flexibility**: Nesting supports non-Western concepts

## What This Enables

### Research Organization
- Complex questions broken into manageable sub-questions
- Identity candidates tracked together
- Locations understood in proper hierarchical context

### Theoretical Exploration
- Multiple identity theories for same person
- Collective identities for groups
- Placeholder identities for unknowns

### Cultural Adaptability
- Non-Western identity concepts (clan identities, name changes)
- Complex location hierarchies
- Flexible relationship structures

## Evolution Path

```
Research Question: "Who were John Smith's parents?"
    ↓
Sub-Question 1: "Which John Smith in census?"
Sub-Question 2: "What records exist for candidate Johns?"
    ↓
Evidence: 1850 census, birth record, will
    ↓
Analysis: Census shows John (30) with James (55) as head
    ↓
Identity Nesting: UncertainIdentityGroup for "John Smith"
  - Candidate 1: John Smith b.1820 (confidence: 0.7)
  - Candidate 2: John Smith b.1823 (confidence: 0.3)
    ↓
Working Hypothesis 1: James is John's father
Working Hypothesis 2: James is John's uncle (guardian)
    ↓
More Evidence: Will shows "my nephew John"
    ↓
Resolution: Candidate 1 is our John Smith
    ↓
Conclusion: James Smith was John's uncle and guardian
    ↓
Traditional Mapping: 
- Person: John Smith (b. ~1820)
- Person: James Smith (b. ~1795)
- Relationship: Uncle/Nephew
- Event: Guardianship ~1830
```

## Implementation Architecture

The nesting capabilities are implemented through:

1. **Base Abstraction** (`nesting.py`)
   - `NestableEntity` base class
   - Tree traversal operations
   - Validation framework
   - Constraint checking

2. **Entity-Specific Implementations**
   - `identity_nesting.py`: Full implementation for identities
   - Research Questions: Simple parent/child fields
   - Locations: Parent/child hierarchy fields
   - Events: Sequence relationships

3. **Analysis Tools**
   - Find common ancestors
   - Calculate relationship paths
   - Flatten hierarchies for display
   - Extract individuals from groups

## Future Considerations

1. **Standardize Nesting**: Consider migrating all nestable entities to use `NestableEntity`
2. **Event Nesting**: Implement true compositional nesting for events
3. **Evidence Grouping**: Consider evidence collections/folders
4. **Analysis Nesting**: Sub-analyses for complex evaluations
5. **Cross-Entity Nesting**: Research collections containing mixed entity types