# Relationship Types in ResearchProcess-GPS

## Overview

ResearchProcess-GPS recognizes that human relationships are complex and varied. The system supports modeling any type of relationship while maintaining clear distinctions between different categories.

## Major Relationship Categories

### 1. Biological Relationships

These are blood/genetic connections between people.

**Characteristics:**
- Cannot be terminated (though can be unknown)
- May be confirmed through DNA testing
- Can exist without social recognition

**Types:**
- Parent-Child (biological)
- Siblings (full, half, step)
- Grandparent-Grandchild
- Aunt/Uncle-Niece/Nephew
- Cousins (various degrees)

**Subtypes:**
- `biological_confirmed` - DNA or documentary proof
- `biological_presumed` - Assumed based on evidence
- `biological_disputed` - Conflicting evidence
- `biological_unknown_parent` - One parent unknown

**Example:**
```python
biological_parent = Relationship(
    category=RelationshipCategory.BIOLOGICAL,
    relationship_type=RelationshipType.PARENT_CHILD,
    subtype="biological_confirmed",
    status=RelationshipStatus.CONCLUDED,
    participants=[
        RelationshipParticipant(mary_smith, role="mother"),
        RelationshipParticipant(john_smith, role="son")
    ]
)
```

### 2. Legal Relationships

These are relationships recognized by law, which may or may not involve biological connections.

**Characteristics:**
- Created through legal process
- Can be terminated legally
- Varies by jurisdiction and time period
- Often documented officially

**Types:**
- Marriage/Civil Union
- Adoption
- Guardianship
- Foster relationships
- Legal custody

**Subtypes:**
- `marriage_civil` - Civil ceremony
- `marriage_religious` - Religious ceremony
- `marriage_common_law` - Common law recognition
- `adoption_legal` - Court-ordered adoption
- `adoption_informal` - Cultural/social adoption
- `guardianship_court` - Legal guardianship
- `guardianship_de_facto` - Informal guardianship

**Example:**
```python
adoption = Relationship(
    category=RelationshipCategory.LEGAL,
    relationship_type=RelationshipType.ADOPTION,
    subtype="adoption_legal",
    status=RelationshipStatus.CONCLUDED,
    legal_framework="California Adoption Law 1970",
    participants=[
        RelationshipParticipant(james_jones, role="adoptive_father"),
        RelationshipParticipant(mary_jones, role="adoptive_mother"),
        RelationshipParticipant(susan_smith, role="adopted_child")
    ],
    defining_events=[adoption_decree_event_id]
)
```

### 3. Social Relationships

These are relationships based on social bonds, community recognition, or personal choice.

**Characteristics:**
- Based on social/cultural norms
- May have no legal standing
- Can be very significant culturally
- Often undocumented officially

**Types:**
- Godparents/Godchildren
- Step-relationships
- In-law relationships
- Mentor/Apprentice
- Household members
- Neighbors
- Friends

**Subtypes:**
- `godparent_religious` - Religious ceremony
- `godparent_cultural` - Cultural tradition
- `step_through_marriage` - Via parent's marriage
- `household_member` - Living together
- `household_boarder` - Paying boarder
- `household_servant` - Domestic servant

**Example:**
```python
godparent = Relationship(
    category=RelationshipCategory.SPIRITUAL,
    relationship_type=RelationshipType.GODPARENT,
    subtype="godparent_religious",
    status=RelationshipStatus.CONCLUDED,
    cultural_context="Catholic baptism tradition",
    participants=[
        RelationshipParticipant(pedro_garcia, role="godfather"),
        RelationshipParticipant(maria_santos, role="godchild")
    ],
    defining_events=[baptism_event_id]
)
```

### 4. Economic/Professional Relationships

These involve economic or professional connections.

**Types:**
- Employer/Employee
- Master/Apprentice
- Business partners
- Landlord/Tenant

**Subtypes:**
- `employment_contract` - Formal employment
- `employment_informal` - Casual labor
- `apprentice_guild` - Guild system
- `apprentice_informal` - Informal training
- `business_partnership` - Business partners
- `business_investor` - Financial backer

### 5. Historical Relationships

These are relationships specific to historical contexts that may not exist today.

**Types:**
- Enslaver/Enslaved person
- Feudal relationships
- Indenture relationships
- Colonial relationships

**Subtypes:**
- `enslaved_chattel` - Chattel slavery
- `enslaved_debt` - Debt bondage
- `indenture_contract` - Indentured servant
- `feudal_lord_vassal` - Feudal hierarchy
- `feudal_serf` - Bound to land

**Note:** These are included for historical accuracy and research completeness. They represent historical injustices and should be handled with appropriate sensitivity.

## Complex and Nuanced Relationships

### Uncertain Relationships

During research, relationships may be uncertain:

```python
possible_parent = Relationship(
    category=RelationshipCategory.BIOLOGICAL,
    relationship_type=RelationshipType.PARENT_CHILD,
    status=RelationshipStatus.THEORETICAL,
    subtype="biological_presumed",
    confidence=ConfidenceContainer(
        summary_confidence=ConfidenceLevel.PROBABLE
    ),
    research_notes=["1850 census shows in same household", 
                   "Age difference consistent with parent-child"]
)
```

### Multiple Relationship Types

People can have multiple types of relationships:

```python
# Adoptive parents who are also godparents
adoptive_and_godparent = [
    Relationship(
        category=RelationshipCategory.LEGAL,
        relationship_type=RelationshipType.ADOPTION,
        subtype="adoption_legal"
    ),
    Relationship(
        category=RelationshipCategory.SPIRITUAL,
        relationship_type=RelationshipType.GODPARENT,
        subtype="godparent_religious"
    )
]
```

### Cultural Variations

Relationships must respect cultural contexts:

```python
# Polygamous marriage (where legal)
polygamous_marriage = Relationship(
    category=RelationshipCategory.LEGAL,
    relationship_type=RelationshipType.MARRIAGE,
    subtype="marriage_polygamous",
    cultural_context="Islamic law, Ottoman Empire 1850",
    legal_framework="Sharia law",
    participants=[
        RelationshipParticipant(ahmed, role="husband"),
        RelationshipParticipant(fatima, role="wife"),
        RelationshipParticipant(aisha, role="wife")
    ]
)

# Matrilineal clan relationships
clan_relationship = Relationship(
    category=RelationshipCategory.SOCIAL,
    relationship_type=RelationshipType.CUSTOM,
    custom_type="clan_membership",
    subtype="matrilineal_clan",
    cultural_context="Cherokee Wolf Clan",
    participants=[
        RelationshipParticipant(clan_mother, role="clan_mother"),
        RelationshipParticipant(member1, role="clan_member"),
        RelationshipParticipant(member2, role="clan_member")
    ]
)
```

## Best Practices

### 1. Be Specific
Always use the most specific relationship type and subtype available.

### 2. Document Context
Include cultural, legal, and social context to properly interpret relationships.

### 3. Track Status
Use RelationshipStatus to distinguish theoretical from concluded relationships.

### 4. Respect Privacy
Some relationships may be sensitive. Use appropriate privacy levels.

### 5. Avoid Assumptions
Don't assume nuclear family structures. Support all family forms.

### 6. Historical Accuracy
Represent historical relationships accurately while being sensitive to historical injustices.

### 7. Multiple Relationships
Remember that people can have multiple simultaneous relationships of different types.

## Relationship vs. Nesting Reminder

**NEVER** use nesting to imply relationships:
- ❌ Wrong: Parent identity contains child identities
- ✅ Right: Parent and child identities connected by relationship

**DO** use nesting for research organization:
- ✅ Right: "Smith Family Research" collection contains all Smith identities
- ✅ Right: "Household Members 1850" collection contains all residents