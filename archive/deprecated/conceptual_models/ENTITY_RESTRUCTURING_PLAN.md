# Entity Restructuring Plan

## Critical Conceptual Changes

### 1. Research Question = Theory (MERGE)

**Current State**: Two separate entities
- `ResearchQuestion` - User-facing concept
- `Theory` - Database entity with branching

**Target State**: Single entity
- Database name: `Theory` (or `ResearchTheory`)
- User-facing: Still called "Research Question"
- Combines all functionality of both

**Key Features to Preserve**:
- Question/hypothesis formulation
- Evidence gathering and evaluation
- Working hypotheses (sub-theories)
- Branching and merging (Git-like)
- GPS compliance tracking
- Conclusion states

### 2. Repository → Source Hierarchy

**Current State**: 
- `Repository` as standalone entity
- `Evidence` references Repository
- No clear source hierarchy

**Target State**: Source hierarchy
```
Source (top level)
  ├── Repository (archives, libraries, websites)
  │   ├── Collection (record groups, databases)
  │   │   ├── Series (specific record types)
  │   │   │   └── Item (individual documents)
```

**Key Concepts**:
- Sources can be online or physical
- Repositories are just one type of source container
- Citations point to specific locations in hierarchy
- Everything nestable for organization

### 3. Citations as Flexible System

**Current State**: 
- `Citation` as data class in Evidence
- Fixed structure

**Target State**: Three types of citations
1. **Reference Citations** - Pointers between entities
   - Evidence → IdentityPersona
   - Analysis → Evidence
   - Flexible, contextual

2. **Source Citations** - Formal source references
   - Immutable once created
   - Follow citation standards
   - Can be concluded/validated

3. **Inline Citations** - Attributes on entities
   - Quick references
   - Informal notes
   - Can be promoted to formal citations

### 4. IdentityPersona/Person as Single Entity

**Current State**:
- `IdentityPersona` - Evidence references
- `Person` - Concluded individuals
- Two separate classes

**Target State**: Single entity with states
```python
class Identity(NestableBaseEntity):
    status: IdentityStatus = IdentityStatus.PERSONA
    # PERSONA → CANDIDATE → CONCLUDED → VALIDATED
    
    # When PERSONA: evidence references, can nest freely
    # When CONCLUDED: validated person, participates in relationships
    # Can promote/demote between states
```

**Key Benefits**:
- Same entity evolves through research
- Maintains all evidence references
- Can revert if new evidence emerges
- Relationships only between CONCLUDED identities

### 5. All Conclusion Entities Can Evolve

**Pattern**: All entities that can be "concluded" should follow same pattern
- Start as research/working entities
- Can be promoted to conclusions
- Can be demoted if questioned
- Maintain full history

**Applies to**:
- Identity (PERSONA → CONCLUDED)
- Event (HYPOTHETICAL → CONCLUDED)
- Relationship (THEORETICAL → CONCLUDED)
- Location (UNCERTAIN → VERIFIED)

## Implementation Order

1. **First**: Refactor IdentityPersona/Person into single Identity entity
2. **Second**: Merge ResearchQuestion/Theory 
3. **Third**: Create Source hierarchy with Repository as component
4. **Fourth**: Implement flexible Citation system
5. **Fifth**: Update all entities for promote/demote pattern

## Backward Compatibility

- Keep aliases for old names
- Migration utilities for existing data
- Clear documentation on changes

## Benefits

1. **Conceptual Clarity**: Entities match research workflow
2. **Flexibility**: Everything can evolve through research
3. **Consistency**: Same patterns across all entities
4. **Natural Workflow**: Matches how genealogists actually work