# Enhanced ASSO Extension Design

## Overview

This document proposes a minimal extension to GEDCOM 7's ASSO structure to address immediate needs, particularly from GRAMPS, without creating the complexity of a full LINK structure.

## Design Philosophy

1. **Minimal Enhancement**: Add only what's truly needed
2. **Backward Compatible**: Must not break existing ASSO usage
3. **Forward Thinking**: Design with GEDCOM 8 adoption in mind
4. **Developer Friendly**: Simple to implement and understand

## Current ASSO Limitations

GEDCOM 7 ASSO provides:
```gedcom
1 ASSO @I2@
2 ROLE Godfather
2 PHRASE Godfather to this person
2 NOTE Additional information
2 SOUR @S1@
```

What's missing:
- Time boundaries (when did relationship start/end)
- Confidence levels
- Specific relationship types beyond generic ROLE
- Privacy flags (inherited from individual only)

## Proposed Enhancement

### Extension Name: `_ASSO-PLUS`

### New Substructures

```gedcom
1 ASSO @I2@
2 ROLE Godfather              # Standard GEDCOM 7
2 _TYPE Godparent             # Specific relationship type
2 _PERIOD                     # Time boundaries
3 _START 1850-05-15          # Baptism date
3 _END 1868-05-15            # Child's majority
2 _CONF 4                     # Confidence (0-5 scale)
2 _PRIV Y                     # Relationship-specific privacy
```

### Structure Definitions

#### _TYPE
- **Purpose**: Specific relationship type complementing ROLE
- **Cardinality**: [0:1]
- **Datatype**: Text
- **Examples**: "Godparent", "Business Partner", "Military Buddy", "Neighbor"

#### _PERIOD
- **Purpose**: Define time boundaries for the relationship
- **Cardinality**: [0:1]
- **Substructures**:
  - `_START` [0:1] - DateValue when relationship began
  - `_END` [0:1] - DateValue when relationship ended

#### _CONF
- **Purpose**: Confidence in the relationship assertion
- **Cardinality**: [0:1]
- **Datatype**: Integer (0-5)
- **Scale**: Same as Evidence extension
  - 0 = Unreliable evidence or estimated data
  - 1 = Questionable reliability
  - 2 = Secondary evidence, probably reliable
  - 3 = Direct source, primary evidence
  - 4 = Well-supported conclusion
  - 5 = Very high confidence

#### _PRIV
- **Purpose**: Relationship-specific privacy flag
- **Cardinality**: [0:1]
- **Datatype**: Y|N
- **Note**: Overrides individual's privacy for this relationship

## Use Cases

### 1. Time-Bounded Godparent
```gedcom
0 @I1@ INDI
1 NAME John Smith
1 ASSO @I2@
2 ROLE Godfather
2 _TYPE Godparent
2 _PERIOD
3 _START 1850-05-15
3 _END 1868-05-15
2 NOTE Spiritual guardian until majority
```

### 2. Business Partnership
```gedcom
0 @I1@ INDI
1 NAME James Wilson
1 ASSO @I2@
2 ROLE Business Partner
2 _TYPE General Partnership
2 _PERIOD
3 _START 1845
3 _END 1867
2 _CONF 5
2 SOUR @S1@
3 PAGE County Business Registry 1845-1867
```

### 3. Military Relationship
```gedcom
0 @I1@ INDI
1 NAME Robert Johnson
1 ASSO @I2@
2 ROLE Military Comrade
2 _TYPE Battle Buddy
2 _PERIOD
3 _START 1942-06
3 _END 1945-05
2 NOTE Served together in 101st Airborne
```

### 4. Uncertain Relationship
```gedcom
0 @I1@ INDI
1 NAME Mary Brown
1 ASSO @I2@
2 ROLE Possible Sister
2 _TYPE Sibling
2 _CONF 1
2 NOTE Census shows same household, ages suggest siblings
```

## Implementation Notes

### For Symmetric Relationships
Both individuals should have ASSO pointing to each other:
```gedcom
0 @I1@ INDI
1 ASSO @I2@
2 ROLE Friend
2 _TYPE Close Friend
2 _PERIOD
3 _START 1920

0 @I2@ INDI  
1 ASSO @I1@
2 ROLE Friend
2 _TYPE Close Friend
2 _PERIOD
3 _START 1920
```

### For Asymmetric Relationships
ROLE and _TYPE can differ:
```gedcom
0 @I1@ INDI
1 ASSO @I2@
2 ROLE Employer
2 _TYPE Employment

0 @I2@ INDI
1 ASSO @I1@
2 ROLE Employee  
2 _TYPE Employment
```

## What This Does NOT Solve

1. **Multi-party relationships** (3+ people)
2. **Complex role hierarchies**
3. **Relationship events** (beyond start/end)
4. **Relationship attributes** (beyond basic metadata)

These complex needs should be addressed in the research/GPS project.

## Migration Path

### From Standard ASSO
No migration needed - extensions are additive.

### To Future GEDCOM 8
If adopted into standard:
- `_PERIOD` → `PERIOD`
- `_START` → `START`
- `_END` → `END`
- `_TYPE` → `TYPE`
- `_CONF` → `CONF`

### From Abandoned LINK Design
Simple 2-party LINK records can be converted:
```gedcom
# From LINK:
0 @L1@ _LINK
1 TYPE Godparent
1 _PARTY @I1@
2 ROLE Godfather
1 _PARTY @I2@
2 ROLE Godchild
1 _START 1850
1 _END 1868

# To Enhanced ASSO:
0 @I1@ INDI
1 ASSO @I2@
2 ROLE Godfather
2 _TYPE Godparent
2 _PERIOD
3 _START 1850
3 _END 1868

0 @I2@ INDI
1 ASSO @I1@
2 ROLE Godchild
2 _TYPE Godparent
2 _PERIOD
3 _START 1850
3 _END 1868
```

## Comparison with Alternatives

### vs Full LINK Extension
- **Pros**: Simpler, no new top-level structure, no "two ways" problem
- **Cons**: Can't handle multi-party or complex relationships

### vs Status Quo (Plain ASSO)
- **Pros**: Handles time boundaries, confidence, specific types
- **Cons**: Requires extension support

### vs Waiting for GEDCOM 8
- **Pros**: Available now, can influence standard
- **Cons**: Might need migration later

## Recommendation

1. **Implement this enhanced ASSO extension** for immediate needs
2. **Move complex relationship modeling** to research/GPS project
3. **Advocate for adoption** in GEDCOM 8 standard
4. **Keep it simple** - resist scope creep

## Next Steps

1. Finalize extension design based on feedback
2. Create formal extension documentation
3. Implement in GRAMPS as proof of concept
4. Submit to GEDCOM registry
5. Prepare GEDCOM 8 proposal