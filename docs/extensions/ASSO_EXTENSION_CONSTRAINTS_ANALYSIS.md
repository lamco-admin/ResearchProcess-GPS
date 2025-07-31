# ASSO Extension Constraints Analysis

## GEDCOM 7 ASSO Structure

Based on GEDCOM 7 specification, ASSO has the following standard structure:

```gedcom
n ASSO @<XREF:INDI>@               {1:M}    g7:ASSO
  +1 PHRASE <Text>                 {0:1}    g7:PHRASE
  +1 ROLE <Enum>                   {0:1}    g7:ROLE
     +2 PHRASE <Text>              {0:1}    g7:PHRASE
  +1 <<NOTE_STRUCTURE>>            {0:M}
  +1 <<SOURCE_CITATION>>           {0:M}
```

## Key Constraints

### 1. What ASSO Points To
- **Constraint**: ASSO must point to an INDI record (@<XREF:INDI>@)
- **Implication**: Cannot point to relationship records or other structure types

### 2. Standard Substructures
Current allowed substructures:
- `PHRASE` - User-defined phrase
- `ROLE` - Role enumeration with optional PHRASE
- `NOTE` - Note structures  
- `SOUR` - Source citations

### 3. Cardinality
- ASSO itself: {1:M} - Can have multiple ASSO under INDI
- ROLE: {0:1} - Zero or one role
- PHRASE: {0:1} - Zero or one phrase

## Extension Rules (GEDCOM 7 Spec Section 4)

### What Extensions CAN Do
1. **Add new substructures** to existing structures using extension tags
2. **Add new enumeration values** to existing enumerations
3. **Create new structure types** with extension tags
4. **Use standard structures in new contexts**

### What Extensions CANNOT Do
1. **Change the meaning** of existing structures
2. **Change cardinalities** of existing structures
3. **Change what a pointer points to** (ASSO must still point to INDI)
4. **Remove or modify** existing substructures

## Backward Compatibility Implications

### Scenario 1: Non-Supporting Software
When software doesn't support our extensions:
```gedcom
1 ASSO @I2@
2 ROLE Godfather
2 _PERIOD                    # Ignored
3 _START 1850               # Ignored
3 _END 1868                 # Ignored
2 _CONF 4                   # Ignored
```
Result: Basic ASSO preserved, extensions ignored

### Scenario 2: Mixed Environment
- Supporting software sees full enhanced structure
- Non-supporting software sees basic ASSO
- No data corruption or errors

## What We CAN Add to ASSO

### 1. Time Period Extension
```gedcom
1 ASSO @I2@
2 _PERIOD
3 _START <DateValue>
3 _END <DateValue>
```

### 2. Confidence Extension
```gedcom
1 ASSO @I2@
2 _CONF <Integer 0-5>
```

### 3. Type Extension
```gedcom
1 ASSO @I2@
2 _TYPE <Text>
```

### 4. Privacy Extension
```gedcom
1 ASSO @I2@
2 _PRIV <Y|N>
```

## What We CANNOT Do

### 1. Cannot Change ASSO Target
❌ Cannot make ASSO point to a relationship record:
```gedcom
1 ASSO @R1@  # INVALID - must point to INDI
```

### 2. Cannot Change Existing Behavior
❌ Cannot change what ROLE means or how it works

### 3. Cannot Add Required Fields
❌ Cannot make new fields mandatory - all must be optional

## Design Implications

### 1. Symmetric Relationships
Since ASSO must point to INDI, symmetric relationships require:
```gedcom
0 @I1@ INDI
1 ASSO @I2@
2 ROLE Friend
2 _TYPE Close Friend

0 @I2@ INDI
1 ASSO @I1@
2 ROLE Friend
2 _TYPE Close Friend
```

### 2. Multi-Party Limitations
Cannot directly model 3+ party relationships with ASSO:
- Each ASSO links exactly 2 people
- No way to group multiple ASSO into one relationship
- This is why LINK was considered for complex cases

### 3. Asymmetric Information
Different information can exist on each side:
```gedcom
0 @I1@ INDI
1 ASSO @I2@
2 ROLE Employer
2 _PERIOD
3 _START 1850
3 _END 1860

0 @I2@ INDI
1 ASSO @I1@
2 ROLE Employee
2 _PERIOD
3 _START 1850
3 _END 1855    # Different end date
```

## Comparison with LINK Approach

### ASSO Extension
- **Pros**: Works within existing structure, backward compatible
- **Cons**: Limited to 2-party, must duplicate data, no relationship identity

### LINK Structure (Abandoned)
- **Pros**: Could handle N-party, single source of truth, relationship identity
- **Cons**: New structure, "two ways" problem, complex

## Recommendations

1. **Proceed with ASSO extension** for immediate needs
2. **Accept the limitations** - 2-party only, symmetric duplication
3. **Document best practices** for data consistency
4. **Reserve complex cases** for future research/GPS project

## Open Questions

1. Should we recommend tools auto-sync symmetric ASSO?
2. How to handle conflicts when both sides have different data?
3. Should we define standard _TYPE values or leave open?