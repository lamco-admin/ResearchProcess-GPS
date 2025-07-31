# Identity/Persona Examples

## Core Concept

Each reference to a person in evidence creates an **IdentityPersona**. These can be:
- Grouped together when possibly the same person (through nesting)
- Progress through research states from Reference → Working → Hypothesis → Concluded
- Be challenged and moved back to earlier states if new evidence emerges

**Note**: There is no separate "Person" entity - IdentityPersona serves all purposes through state transitions.

## Example 1: Bob/Robert Jones Research

### Step 1: Evidence Creates Identities

```python
# Letter from 1922
letter_evidence_id = uuid4()  # Evidence: Letter from Mary to daughter

# Create identity from letter reference
bob_identity = IdentityPersona(
    primary_name="Bob (Grandma's neighbor)",
    identity_type=IdentityType.DESCRIBED
)

bob_identity.add_evidence_reference(
    evidence_id=letter_evidence_id,
    reference_text="Bob",
    reference_type="name",
    context="Bob, Grandma's neighbor, helped with the harvest",
    attributes={
        "relationship": "Grandma's neighbor",
        "activity": "helped with harvest",
        "date": "1922"
    }
)

# 1920 Census
census_evidence_id = uuid4()  # Evidence: 1920 Census page

# Create identity from census
robert_identity = IdentityPersona(
    primary_name="Robert Jones (1920 census)",
    identity_type=IdentityType.NAMED
)

robert_identity.add_evidence_reference(
    evidence_id=census_evidence_id,
    reference_text="Robert Jones",
    reference_type="name",
    context="Head of household, next to Edith McIntosh",
    attributes={
        "age": "35",
        "occupation": "Farmer",
        "relationship_to_head": "Head",
        "neighbors": ["Edith McIntosh"],
        "year": "1920"
    }
)
```

### Step 2: Research Suggests Same Person

```python
# Create identity group for research
possible_bob_robert = IdentityGroup(
    group_name="Bob = Robert Jones?",
    grouping_rationale="Bob is Grandma's neighbor; Robert Jones lived next to Edith McIntosh (Grandma)"
)

# Add both identities to group
possible_bob_robert.add_identity(
    bob_identity,
    rationale="Neighbor relationship matches"
)

possible_bob_robert.add_identity(
    robert_identity,
    rationale="Census shows next to Edith McIntosh"
)

# Increase confidence as research progresses
possible_bob_robert.same_person_confidence = ConfidenceLevel.PROBABLE
```

### Step 3: Additional Evidence

```python
# Found 1925 deed
deed_evidence_id = uuid4()

# Add to existing identity
robert_identity.add_evidence_reference(
    evidence_id=deed_evidence_id,
    reference_text="Robert 'Bob' Jones",
    reference_type="name",
    context="Property deed between neighbors",
    attributes={
        "full_name": "Robert 'Bob' Jones",
        "nickname": "Bob",
        "property": "Adjacent to McIntosh farm"
    }
)
```

### Step 4: Conclude Identity Through State Transition

```python
# Research concludes they're the same person - merge identities through nesting
robert_identity.add_nested_identity(
    bob_identity, 
    confidence=0.95,
    rationale="Multiple evidence sources confirm Robert 'Bob' Jones lived next to Edith McIntosh"
)

# Transition to concluded state
robert_identity.transition_to_state(
    IdentityState.CONCLUDED,
    rationale="Research complete - confirmed same person through multiple sources"
)

# Now can access consolidated information and add relationships
print(robert_identity.get_all_names())  
# ["Bob", "Robert Jones", "Robert 'Bob' Jones"]

# Can now add facts and relationships since concluded
robert_identity.add_fact(occupation_fact_id)
robert_identity.add_relationship(spouse_relationship_id)
```

## Example 2: Complex Identity Resolution

### Multiple References in Single Document

```python
# 1850 Will mentions several people
will_evidence_id = uuid4()

# "my son John"
son_john = IdentityPersona(primary_name="John (son in will)")
son_john.add_evidence_reference(
    evidence_id=will_evidence_id,
    reference_text="my son John",
    reference_type="relationship",
    attributes={"relationship": "son of testator"}
)

# "John Smith of Richmond"
john_smith = IdentityPersona(primary_name="John Smith of Richmond")
john_smith.add_evidence_reference(
    evidence_id=will_evidence_id,
    reference_text="John Smith of Richmond",
    reference_type="name",
    attributes={"residence": "Richmond"}
)

# Research question: Same person?
john_group = IdentityGroup(
    group_name="John son = John Smith of Richmond?",
    grouping_rationale="Both mentioned in same will, might be same person"
)
```

## Example 3: Challenge and State Transition Scenario

```python
# Previously concluded identity
mary_jones = IdentityPersona(
    primary_name="Mary Jones",
    state=IdentityState.CONCLUDED
)
mary_jones.add_fact(birth_1820_fact_id)
mary_jones.add_fact(parents_james_sarah_fact_id)

# New evidence challenges conclusion
conflicting_evidence_id = uuid4()

# Create new identity from conflicting evidence
mary_1822 = IdentityPersona(primary_name="Mary Jones (b. 1822)")
mary_1822.add_evidence_reference(
    evidence_id=conflicting_evidence_id,
    reference_text="Mary Jones",
    reference_type="name",
    attributes={
        "birth_year": "1822",
        "parents": "William and Elizabeth Jones"
    }
)

# Challenge the concluded identity - transition back to working state
mary_jones.transition_to_state(
    IdentityState.CHALLENGED,
    reason="Conflicting birth and parentage information found"
)

# Move to working state to resolve conflict
mary_jones.transition_to_state(
    IdentityState.WORKING,
    reason="Need to analyze conflicting evidence"
)

# Can no longer add facts/relationships until resolved
# mary_jones.add_relationship(spouse_id)  # Would fail - not concluded

# Create analysis to resolve the conflict
conflict_analysis = Analysis(
    analysis_type=AnalysisType.CONFLICT_RESOLUTION,
    scope="Resolve Mary Jones birth date conflict"
)
mary_jones.analysis_refs.append(conflict_analysis.id)
```

## Example 4: Placeholder Identities

```python
# Create placeholder for unknown person
unknown_mother = IdentityPersona(
    primary_name="Unknown mother of John Smith",
    identity_type=IdentityType.PLACEHOLDER,
    status=PersonaStatus.RESEARCHER_CREATED
)

# As research progresses, add evidence
unknown_mother.add_evidence_reference(
    evidence_id=census_1850_id,
    reference_text="[female, age 45]",
    reference_type="description",
    context="Living in Smith household",
    attributes={"age": "45", "year": "1850"}
)

# Later, find name
unknown_mother.add_evidence_reference(
    evidence_id=marriage_record_id,
    reference_text="Elizabeth Brown",
    reference_type="name",
    context="Marriage to James Smith 1825"
)

# Update primary name
unknown_mother.primary_name = "Elizabeth Brown (mother of John)"
```

## Key Workflows

### 1. Evidence → Identity → Concluded Identity
```
Evidence (census) → IdentityPersona ("John Smith", state=Reference) → 
Evidence (birth) → IdentityPersona ("John S.", state=Reference) →
Group through nesting → Research → Transition to Concluded state
```

### 2. Concluded → Challenged (State Transitions)
```
IdentityPersona (state=Concluded) → New conflicting evidence →
Challenge → Transition to Working state → Further research
```

### 3. Identity Merging
```
Identity A + Identity B → Same person confidence high →
Merge through nesting → Continue research →
Eventually transition to Concluded state
```

### 4. Identity Splitting
```
Identity (thought to be one) → Evidence suggests two people →
Split into Identity A and Identity B → 
Research separately
```

## Example 5: Complex Multi-Level Nesting

### Deep Identity Hierarchies

```python
# Top-level identity from formal records
robert_jones = IdentityPersona(primary_name="Robert Jones (formal)")

# Informal variants
bob_casual = IdentityPersona(primary_name="Bob (casual references)")
bobby_family = IdentityPersona(primary_name="Bobby (family letters)")
rob_friends = IdentityPersona(primary_name="Rob (friend letters)")

# Bob contains Bobby and Rob as sub-variants
bob_casual.add_nested_identity(bobby_family, confidence=0.9, 
    rationale="Family diminutive of Bob")
bob_casual.add_nested_identity(rob_friends, confidence=0.8,
    rationale="Casual variant used by friends")

# Professional variants
r_jones = IdentityPersona(primary_name="R. Jones (professional)")
rj_initials = IdentityPersona(primary_name="R.J. (signatures)")
jones_r = IdentityPersona(primary_name="Jones, R. (directories)")

# R. Jones contains formal variants
r_jones.add_nested_identity(rj_initials, confidence=0.95,
    rationale="Initials version of R. Jones")
r_jones.add_nested_identity(jones_r, confidence=0.95,
    rationale="Directory listing format")

# Top level contains major variants
robert_jones.add_nested_identity(bob_casual, confidence=0.85,
    rationale="Common nickname for Robert")
robert_jones.add_nested_identity(r_jones, confidence=0.9,
    rationale="Professional usage")

# Result: Deep hierarchy
# Robert Jones
#   ├── Bob
#   │   ├── Bobby
#   │   └── Rob
#   └── R. Jones
#       ├── R.J.
#       └── Jones, R.

# Can get all evidence from entire hierarchy
all_evidence = robert_jones.get_all_evidence_references()
all_names = robert_jones.consolidate_names()
# {"Robert Jones", "Bob", "Bobby", "Rob", "R. Jones", "R.J.", "Jones, R."}
```

### Research Evolution Through Nesting

```python
# Start with separate identities from different sources
census_1850 = IdentityPersona(primary_name="R. Jones (1850 census)")
census_1860 = IdentityPersona(primary_name="Robert Jones (1860 census)")
census_1870 = IdentityPersona(primary_name="Bob Jones (1870 census)")

# Initially group 1850 and 1860
early_robert = IdentityGroup(group_name="R/Robert 1850-1860")
early_robert.add_identity(census_1850)
early_robert.add_identity(census_1860)

# Later, realize 1870 Bob might be same person
# Create encompassing identity
full_robert = IdentityPersona(primary_name="Robert 'Bob' Jones (all census)")
full_robert.add_nested_identity(early_robert, confidence=0.8)
full_robert.add_nested_identity(census_1870, confidence=0.7)

# As research progresses, can reorganize
# Move 1850 directly under main
census_1850.move_to_parent(full_robert)

# Final structure reflects research understanding
```

### Geographic Variants Nesting

```python
# Same person in different locations
john_smith = IdentityPersona(primary_name="John Smith (master)")

# Virginia records
john_va = IdentityPersona(primary_name="John Smith of Richmond")
john_henrico = IdentityPersona(primary_name="John Smith (Henrico Co)")

# Kentucky records  
john_ky = IdentityPersona(primary_name="John Smith of Lexington")
j_smith_ky = IdentityPersona(primary_name="J. Smith (Kentucky)")

# Nest by state
john_smith.add_nested_identity(john_va, rationale="Virginia period")
john_va.add_nested_identity(john_henrico, rationale="Specific county")

john_smith.add_nested_identity(john_ky, rationale="After migration")
john_ky.add_nested_identity(j_smith_ky, rationale="Abbreviated form")

# Can query by location/time period through nesting structure
```

## Benefits of This Unified Approach

1. **Evidence-First**: Every identity tied to actual evidence
2. **Flexible Research**: Can group/ungroup through nesting as needed
3. **State-Based Progression**: Reference → Working → Hypothesis → Concluded
4. **Reversible States**: Can challenge and move back to earlier states
5. **Audit Trail**: Track all evidence, reasoning, and state transitions
6. **Supports Uncertainty**: Work with possibilities before concluding
7. **Unlimited Nesting**: Organize as complex as research requires
8. **Single Entity Model**: No confusion between IdentityPersona and Person
9. **Behavioral States**: Concluded identities can have facts/relationships
10. **Multiple Hierarchies**: Same identity can be in different groupings