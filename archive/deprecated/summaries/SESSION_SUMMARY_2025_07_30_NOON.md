# Session Summary - 2025-07-30 Noon

## What We Discovered

### ✅ Successfully Updated to Nesting
- Evidence - Now supports document bundles, collections, hierarchies
- Analysis - Supports multi-phase, revisions, collaboration  
- Theory - Supports versioning, sub-theories, archives
- Repository - Now nestable for hierarchical organization

### 🔴 Critical Conceptual Issues Found

1. **ResearchQuestion vs Theory Redundancy**
   - These are the SAME THING
   - Theory = database entity
   - ResearchQuestion = user-friendly term
   - Need to merge into single entity

2. **Repository Misconception**
   - Not a standalone entity
   - Part of source hierarchy: Source → Repository → Collection → Item
   - Current model treats it too independently

3. **Citation Confusion**
   - Currently just a data class
   - Needs to be THREE different things:
     - Reference citations (pointers between entities)
     - Source citations (formal, immutable)
     - Inline citations (flexible attributes)

4. **IdentityPersona/Person Split**
   - Should be ONE entity with states
   - PERSONA state: evidence reference, nestable
   - CONCLUDED state: validated person, relationships
   - Can promote/demote between states

5. **Missing Conclusion Pattern**
   - ALL entities should support promote/demote
   - Working → Concluded → Questioned → Working
   - Maintains full history and evidence

## Deep Dive: Sources & Citations

We explored how different systems handle sources:
- Evidence Explained (layered, quality-focused)
- GEDCOM (limited but standardized)
- FamilySearch (collaborative, templated)
- Ancestry (AI-assisted, dual nature)
- Academic (primary/secondary/tertiary)

Key insights:
- Sources form complex networks, not simple hierarchies
- Citations serve multiple purposes (legal, academic, practical, social)
- Modern challenges: ephemeral URLs, collaborative editing, AI extraction
- Need flexibility for unknown future source types

## Proposed New Architecture

### 1. Unified Identity Entity
```python
class Identity(NestableBaseEntity):
    status: IdentityStatus  # PERSONA, CANDIDATE, CONCLUDED, VALIDATED
    # Same entity evolves through research
```

### 2. Merged Theory/ResearchQuestion
```python
class Theory(NestableBaseEntity):
    # All ResearchQuestion + Theory functionality
    # User sees "Research Question"
    # Database stores as Theory
```

### 3. Source Hierarchy
```python
class Source(NestableBaseEntity):
    # Top level: websites, archives, collections
    
class Repository(Source):
    # Specific type of source
    
class SourceItem(Source):
    # Individual documents
```

### 4. Flexible Citations
```python
class Citation(NestableBaseEntity):
    citation_type: CitationType  # reference, source, inline
    # Completely flexible based on type
```

## Next Steps

1. **Immediate**: Refactor entities to fix conceptual issues
2. **Then**: Design comprehensive source/citation system
3. **Finally**: Storage adapter that handles this flexibility

## The Big Picture

We're building a system that:
- Matches how genealogists actually think
- Handles all types of sources (current and future)
- Supports the full research workflow
- Allows conclusions to be questioned and revised
- Maintains complete provenance and history

The key insight: **Everything is fluid during research, becomes solid when concluded, but can always be reconsidered.**