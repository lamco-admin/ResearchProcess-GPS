# Identity/Persona Examples

## Core Concept

Each reference to a person in evidence creates an **IdentityPersona**. These can be:
- Grouped together when possibly the same person
- Promoted to **Person** status when research concludes
- Demoted back to IdentityPersona if new evidence emerges

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

### Step 4: Promote to Person

```python
# Research concludes they're the same person
robert_jones_person = Person.from_identities(
    identities=[bob_identity, robert_identity],
    primary=robert_identity,
    rationale="Multiple evidence sources confirm Robert 'Bob' Jones lived next to Edith McIntosh"
)

# Person now has consolidated information
print(robert_jones_person.all_names)  
# ["Bob", "Robert Jones", "Robert 'Bob' Jones"]

print(robert_jones_person.validated_attributes)
# {"occupation": "Farmer", "nickname": "Bob", "residence": "Next to McIntosh"}
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

## Example 3: Demotion Scenario

```python
# Previously concluded person
mary_jones_person = Person(
    primary_name="Mary Jones",
    birth_date=date(1820, 1, 1),
    validated_attributes={"parents": "James and Sarah Jones"}
)

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

# Demote person back to identities
original_identities = mary_jones_person.demote_to_identities(
    reason="Conflicting birth and parentage information"
)

# Now have multiple identity groups to research
mary_group_1 = IdentityGroup(group_name="Mary Jones b.1820")
mary_group_2 = IdentityGroup(group_name="Mary Jones b.1822")
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

### 1. Evidence → Identity → Person
```
Evidence (census) → IdentityPersona ("John Smith") → 
Evidence (birth) → IdentityPersona ("John S.") →
Group as possibly same → Research → Promote to Person
```

### 2. Person → Identity (Demotion)
```
Person (concluded) → New conflicting evidence →
Demote to Identities → Separate identity groups →
Further research
```

### 3. Identity Merging
```
Identity A + Identity B → Same person confidence high →
Merge into single Identity → Continue research →
Eventually promote to Person
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

## Benefits of This Approach

1. **Evidence-First**: Every identity tied to actual evidence
2. **Flexible Research**: Can group/ungroup as needed
3. **Clear Progression**: Evidence → Identity → Person
4. **Reversible**: Can demote if new evidence emerges
5. **Audit Trail**: Track all evidence and reasoning
6. **Supports Uncertainty**: Work with possibilities before concluding
7. **Unlimited Nesting**: Organize as complex as research requires
8. **Multiple Hierarchies**: Same identity can be in different groupings