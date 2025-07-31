# Nesting vs. Genealogical Relationships

## Critical Distinction

ResearchProcess-GPS makes a fundamental distinction between different concepts:

1. **Identity/Persona Nesting** - Research constructs for organizing hypothetical individuals
2. **Organizational Nesting** - Grouping items for research purposes  
3. **Genealogical Relationships** - Actual human connections (biological, legal, social)

## The Identity → Person Journey

### Research Phase (Identities & Personas)
- **Identity**: A hypothetical person who might have existed
- **Persona**: Different aspects/roles/phases of an identity
- These CAN and SHOULD nest for research organization
- Highly flexible, theoretical constructs

### Conclusion Phase (Persons)
- **Person**: A validated, concluded individual
- These have RELATIONSHIPS, not nesting
- Maps to traditional genealogy

## Valid Identity/Persona Nesting

Identity nesting is a powerful research tool for managing uncertainty and complexity:

### Types of Identity Nesting:

✅ **Temporal Phases** (Same person, different life stages)
```python
john_smith_identity = TemporalIdentityPhases("John Smith")
john_smith_identity.add_phase(john_young, start_year=1820, end_year=1840)
john_smith_identity.add_phase(john_adult, start_year=1840, end_year=1870)
john_smith_identity.add_phase(john_elder, start_year=1870, end_year=1890)
```

✅ **Uncertain Identity Groups** (Could be the same person)
```python
unknown_smith = UncertainIdentityGroup("Person who married Mary Jones")
unknown_smith.add_candidate(john_smith_1820, confidence=0.7)
unknown_smith.add_candidate(john_smith_1823, confidence=0.3)
```

✅ **Collective Identities** (Group treated as unit)
```python
smith_brothers = CollectiveIdentity("The Smith Brothers")
smith_brothers.add_member(john_smith_identity)
smith_brothers.add_member(james_smith_identity)
```

✅ **Role-Based Personas** (Same person, different contexts)
```python
john_smith = Identity("John Smith")
john_smith.add_persona(john_merchant, role="merchant")
john_smith.add_persona(john_militia, role="militia_captain")
john_smith.add_persona(john_church, role="deacon")
```

## Other Valid Nesting

Beyond identity nesting, researchers need organizational nesting:

### Examples of Organizational Nesting:

✅ **Research Organization**
- Research Question "Who were the Smiths?" contains sub-questions
- Evidence collection "Smith Family Documents" contains various records
- Location "Virginia" contains "Richmond County"

✅ **Collective Identities**
- "The Smith Brothers" (collective) contains individual Smith brother identities
- "Unknown Miller Children" contains placeholder identities
- "All John Smiths in 1850 Census" contains candidate identities

✅ **Temporal Organization**
- "American Civil War" event contains "Battle of Gettysburg"
- "Smith Migration 1850-1860" contains individual journey stops
- "Life of John Smith" contains life phase identities

✅ **Spatial Hierarchies**
- Country → State → County → City → Street
- Cemetery → Section → Row → Plot

## What are Relationships?

Relationships represent **actual human connections**:

- Biological connections (parent-child, siblings)
- Legal connections (marriage, adoption, guardianship)
- Social connections (godparent, mentor, employer)
- Cultural connections (clan membership, caste)

### Examples of Relationships (NOT Nesting):

❌ **NEVER use nesting for:**
- Parent-child relationships
- Marriages or partnerships
- Sibling relationships
- Any biological connection
- Legal relationships
- Social relationships

## The Right Way to Model Each

### For Organizational Needs (Use Nesting):

```python
# Organizing research about a family
smith_family_research = ResearchCollection("Smith Family Research")
smith_family_research.add_item(john_smith_identity)
smith_family_research.add_item(mary_smith_identity)
smith_family_research.add_item(smith_marriage_event)
smith_family_research.add_item(smith_census_evidence)

# Collective identity for analysis
smith_brothers = CollectiveIdentity("The Smith Brothers")
smith_brothers.add_member(john_smith)
smith_brothers.add_member(james_smith)
smith_brothers.add_member(william_smith)
```

### For Human Connections (Use Relationships):

```python
# Actual family relationships
parent_child = Relationship(
    relationship_type=RelationshipType.BIOLOGICAL_PARENT,
    participants=[
        RelationshipParticipant(john_smith, role="parent"),
        RelationshipParticipant(jane_smith, role="child")
    ]
)

marriage = Relationship(
    relationship_type=RelationshipType.MARRIAGE,
    participants=[
        RelationshipParticipant(john_smith, role="husband"),
        RelationshipParticipant(mary_jones, role="wife")
    ]
)

siblings = Relationship(
    relationship_type=RelationshipType.SIBLINGS,
    participants=[
        RelationshipParticipant(john_smith, role="sibling"),
        RelationshipParticipant(james_smith, role="sibling"),
        RelationshipParticipant(william_smith, role="sibling")
    ]
)
```

## Key Principles

1. **Nesting is NEVER genealogy** - It's purely organizational
2. **Relationships are NEVER nesting** - They're actual human connections
3. **Both can coexist** - The Smith brothers can be in a collective (nesting) AND have sibling relationships
4. **Clear semantics** - Always use the right tool for the right purpose

## Common Pitfalls to Avoid

### ❌ Wrong: Using nesting for genealogy
```python
# NEVER DO THIS
john_smith.add_child(jane_smith)  # This implies organizational hierarchy!
```

### ✅ Right: Using relationships for genealogy
```python
# DO THIS INSTEAD
parent_child_rel = create_parent_child_relationship(
    parent=john_smith,
    child=jane_smith
)
```

### ❌ Wrong: Confusing collective identity with family
```python
# This is NOT a family relationship
smith_family = CollectiveIdentity("Smith Family")
```

### ✅ Right: Clear distinction
```python
# Collective for research organization
smith_family_research = CollectiveIdentity("Smith Family Members")

# Actual family relationships tracked separately
family_relationships = [
    parent_child_rel_1,
    parent_child_rel_2,
    marriage_rel
]
```

## Implementation Guidelines

### In the UI/UX:
- Clearly label organizational structures vs. relationships
- Use different icons/colors for nesting vs. relationships
- Never imply nesting creates genealogical connections

### In the API:
- Separate methods for nesting operations vs. relationship operations
- Clear naming: `add_to_collection()` vs. `create_relationship()`
- Different query methods: `get_nested_items()` vs. `get_relatives()`

### In Documentation:
- Always clarify which concept is being used
- Provide examples showing the distinction
- Warn against conflating the concepts

## The Identity → Person Progression

### During Research (Working with Identities):
```python
# Create uncertain identity group
unknown_parent = UncertainIdentityGroup("Parent of John Smith")
unknown_parent.add_candidate(mary_jones_1, confidence=0.6)
unknown_parent.add_candidate(mary_jones_2, confidence=0.4)

# Work with temporal phases
john_phases = TemporalIdentityPhases("John Smith")
john_phases.add_phase(john_youth, 1820, 1840)
john_phases.add_phase(john_adult, 1840, 1880)

# These can have theoretical relationships
theoretical_rel = TheoreticalRelationship(
    type="possible_parent_child",
    participants=[unknown_parent, john_phases],
    confidence=0.7
)
```

### After Conclusion (Working with Persons):
```python
# Resolve identities to persons
mary_jones_person = unknown_parent.resolve_to_person(mary_jones_1)
john_smith_person = john_phases.resolve_to_person()

# Create actual relationships
parent_child = Relationship(
    type=RelationshipType.BIOLOGICAL_PARENT,
    participants=[
        RelationshipParticipant(mary_jones_person, role="mother"),
        RelationshipParticipant(john_smith_person, role="child")
    ]
)
```

## Key Principles for Identity/Person Distinction

1. **Identities nest, Persons relate** - Identities use nesting for organization, Persons use relationships
2. **Flexibility in research** - Identities can be reorganized, split, merged as needed
3. **Stability in conclusions** - Persons are stable, concluded entities
4. **Clear progression** - Track how Identities become Persons through research
5. **Maintain provenance** - Remember which Identities led to which Persons

## Benefits of Clear Distinction

1. **Research Flexibility** - Organize uncertain identities without committing to relationships
2. **Progressive Certainty** - Move from hypothesis to conclusion naturally
3. **Error Correction** - Can reorganize identities without breaking concluded relationships
4. **Collaboration** - Different researchers can organize identities differently
5. **Cultural Adaptability** - Supports various concepts of identity and personhood