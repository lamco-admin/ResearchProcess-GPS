# ASSO Extension Final Design

## Extension Overview

This extension enhances GEDCOM 7's ASSO structure to support time-bounded relationships, confidence levels, specific relationship types, and relationship-specific privacy without creating new top-level structures.

## Extension Identifier

```
https://github.com/gedcom7/associations
```

## Extended ASSO Structure

```gedcom
n ASSO @<XREF:INDI>@               {1:M}    g7:ASSO
  +1 PHRASE <Text>                 {0:1}    g7:PHRASE
  +1 ROLE <Enum>                   {0:1}    g7:ROLE
     +2 PHRASE <Text>              {0:1}    g7:PHRASE
  +1 <<NOTE_STRUCTURE>>            {0:M}
  +1 <<SOURCE_CITATION>>           {0:M}
  +1 _TYPE <Text>                  {0:1}    https://github.com/gedcom7/associations#TYPE
  +1 _PERIOD                       {0:1}    https://github.com/gedcom7/associations#PERIOD
     +2 _START <DateValue>         {0:1}    https://github.com/gedcom7/associations#START
     +2 _END <DateValue>           {0:1}    https://github.com/gedcom7/associations#END
  +1 _CONF <Integer>               {0:1}    https://github.com/gedcom7/associations#CONF
  +1 _PRIV <Y|N>                   {0:1}    https://github.com/gedcom7/associations#PRIV
```

## Structure Definitions

### _TYPE (Association Type)
- **Purpose**: Specifies the type of association/relationship beyond the ROLE
- **Datatype**: Text
- **Cardinality**: [0:1]
- **Examples**: "Godparent", "Business Partnership", "Military Service", "Neighbor", "Childhood Friend"
- **Usage**: Complements ROLE to provide more specific relationship categorization

### _PERIOD (Time Period)
- **Purpose**: Defines the time boundaries of the association
- **Datatype**: Container for date substructures
- **Cardinality**: [0:1]
- **Substructures**:
  - `_START` [0:1] - When the association began
  - `_END` [0:1] - When the association ended

### _START (Period Start)
- **Purpose**: Date the association began
- **Datatype**: DateValue (standard GEDCOM 7 date)
- **Cardinality**: [0:1]
- **Parent**: _PERIOD

### _END (Period End)
- **Purpose**: Date the association ended
- **Datatype**: DateValue (standard GEDCOM 7 date)
- **Cardinality**: [0:1]
- **Parent**: _PERIOD

### _CONF (Confidence)
- **Purpose**: Confidence level in the association assertion
- **Datatype**: Integer (0-5)
- **Cardinality**: [0:1]
- **Scale**:
  - 0 = Unreliable evidence or estimated data
  - 1 = Questionable reliability of evidence
  - 2 = Secondary evidence, data probably reliable
  - 3 = Direct source, primary evidence, data considered reliable
  - 4 = Well-supported conclusion from primary evidence
  - 5 = Very high confidence
- **Note**: Matches the confidence scale used in the Evidence extension

### _PRIV (Privacy)
- **Purpose**: Indicates if this specific association should be treated as private
- **Datatype**: Y|N
- **Cardinality**: [0:1]
- **Default**: N (if not specified)
- **Usage**: Allows marking sensitive associations as private without making the entire individual private

## Usage Examples

### Example 1: Time-Bounded Godparent Relationship
```gedcom
0 @I1@ INDI
1 NAME John Smith
1 ASSO @I2@
2 ROLE Godfather
2 _TYPE Godparent
2 _PERIOD
3 _START 15 MAY 1850
3 _END 15 MAY 1868
2 NOTE Spiritual guardian from baptism until ward's majority
2 SOUR @S1@
```

### Example 2: Business Partnership with Confidence
```gedcom
0 @I1@ INDI
1 NAME James Wilson
1 ASSO @I2@
2 ROLE Business Partner
2 _TYPE General Partnership
2 _PERIOD
3 _START 1845
3 _END JUN 1867
2 _CONF 5
2 NOTE Wilson & Associates Mercantile
2 SOUR @S2@
3 PAGE County Business Registry, pp. 45-47
```

### Example 3: Military Relationship
```gedcom
0 @I1@ INDI
1 NAME Robert Johnson
1 ASSO @I2@
2 ROLE Fellow Soldier
2 _TYPE Battle Buddy
2 _PERIOD
3 _START JUN 1942
3 _END 8 MAY 1945
2 NOTE Served together in 101st Airborne, European Theater
```

### Example 4: Uncertain Relationship with Low Confidence
```gedcom
0 @I1@ INDI
1 NAME Mary Brown
1 ASSO @I2@
2 ROLE Possible Sister
2 _TYPE Sibling
2 _CONF 1
2 NOTE Same household in 1850 census, ages consistent with siblings
```

### Example 5: Private Association
```gedcom
0 @I1@ INDI
1 NAME Thomas Anderson
1 ASSO @I2@
2 ROLE Informant
2 _TYPE Intelligence Asset
2 _PERIOD
3 _START 1962
3 _END 1975
2 _PRIV Y
2 NOTE Classified relationship - Cold War intelligence
```

## Implementation Guidelines

### Symmetric Relationships
For symmetric relationships (friendship, partnership), both individuals should have corresponding ASSO entries:

```gedcom
0 @I1@ INDI
1 NAME Alice Jones
1 ASSO @I2@
2 ROLE Friend
2 _TYPE Close Friend
2 _PERIOD
3 _START 1920

0 @I2@ INDI
1 NAME Betty Smith
1 ASSO @I1@
2 ROLE Friend
2 _TYPE Close Friend
2 _PERIOD
3 _START 1920
```

### Asymmetric Relationships
For asymmetric relationships, ROLE and potentially dates may differ:

```gedcom
0 @I1@ INDI
1 NAME John Employer
1 ASSO @I2@
2 ROLE Employer
2 _TYPE Employment
2 _PERIOD
3 _START 1 JAN 1850
3 _END 31 DEC 1860

0 @I2@ INDI
1 NAME Jane Worker
1 ASSO @I1@
2 ROLE Employee
2 _TYPE Employment
2 _PERIOD
3 _START 1 JAN 1850
3 _END 15 JUN 1855    # Left employment earlier
```

### Data Consistency
Applications implementing this extension should:
1. Warn users about asymmetric data when editing
2. Offer to synchronize changes across both sides
3. Handle conflicts gracefully
4. Preserve intentional asymmetries

## Backward Compatibility

### Non-Supporting Applications
Applications that don't support this extension will:
- See and preserve the standard ASSO structure
- Ignore all extension tags (_TYPE, _PERIOD, _CONF, _PRIV)
- Maintain data integrity on import/export

### Progressive Enhancement
Supporting applications can:
- Display richer relationship information
- Filter by time periods
- Show confidence levels
- Respect privacy flags
- Provide better relationship analysis

## Schema Declaration

```gedcom
0 HEAD
1 SCHMA
2 TAG _TYPE https://github.com/gedcom7/associations#TYPE
2 TAG _PERIOD https://github.com/gedcom7/associations#PERIOD
2 TAG _START https://github.com/gedcom7/associations#START
2 TAG _END https://github.com/gedcom7/associations#END
2 TAG _CONF https://github.com/gedcom7/associations#CONF
2 TAG _PRIV https://github.com/gedcom7/associations#PRIV
```

## Limitations

This extension does not address:
1. Multi-party relationships (3+ individuals)
2. Relationship events beyond start/end
3. Complex role hierarchies
4. Relationship attributes beyond basic metadata

These advanced features are reserved for future research/GPS extensions.

## Migration Considerations

### From Basic ASSO
No migration needed - extensions are purely additive.

### From Abandoned LINK Design
Simple 2-party LINK records can be converted to enhanced ASSO by:
1. Creating ASSO on both individuals
2. Mapping LINK TYPE to _TYPE
3. Mapping time boundaries to _PERIOD
4. Preserving confidence as _CONF

### To Future GEDCOM Standard
If adopted into GEDCOM 8+:
- `_TYPE` → `TYPE`
- `_PERIOD` → `PERIOD`
- `_START` → `START`
- `_END` → `END`
- `_CONF` → `CONF`
- `_PRIV` → `PRIV`