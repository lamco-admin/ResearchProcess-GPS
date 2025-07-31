# GEDCOM 7 Extensions for Genealogical Research

## Design Principles

1. **Research Process First**: What do genealogists actually need?
2. **Evidence-Based**: Support proper source analysis
3. **Flexibility**: Don't force one methodology
4. **Compatibility**: Work with existing tools where possible
5. **Clarity**: Clear semantics, not implementation details

## Core Research Needs

### 1. Evidence vs Conclusions
**Need**: Distinguish between what sources say and what we conclude

**Extension Set**:
```yaml
# Evidence container - what a source actually says
_EVID: 
  uri: https://gedcom.io/x/evidence
  payload: pointer to evidence record
  substructures:
    _CONF: confidence level (0-4)
    _METH: extraction method
    NOTE: analysis notes

# Evidence record - extracted from source
_EXTR:
  uri: https://gedcom.io/x/extracted
  payload: none
  superstructures: [INDI, FAM, _EVNT]
  substructures: [all normal person/family/event substructures]
```

### 2. Independent Events
**Need**: Events involve multiple people with different roles

**Extension Set**:
```yaml
# Shared event record
_EVNT:
  uri: https://gedcom.io/x/event
  payload: none
  substructures:
    TYPE: event type
    DATE: when it happened
    PLAC: where it happened
    _PART: participants

# Event participation
_PART:
  uri: https://gedcom.io/x/participant
  payload: pointer to person
  substructures:
    ROLE: role in event
    NOTE: participant-specific notes
    AGE: age at event

# Reference from person to event
_EVREF:
  uri: https://gedcom.io/x/event-reference
  payload: pointer to event
  substructures:
    ROLE: person's role
```

### 3. Source Layers (Evidence Explained)
**Need**: Sources derive from other sources

**Already handled by dthaler's extensions**:
- `_SOUR`: Source derivation
- `_FIEL`: Field-based citations
- `_TPLT`: Citation templates

### 4. Research Process
**Need**: Track research activities, not just results

**Extension Set**:
```yaml
# Research activity
_RSRCH:
  uri: https://gedcom.io/x/research
  payload: none
  substructures:
    DATE: when performed
    _RSRCHER: who did it
    _VENUE: where researched
    _PURPOSE: research goal
    _RESULT: what was found
    NOTE: details

# Research log entry
_LOG:
  uri: https://gedcom.io/x/log-entry
  payload: text description
  substructures:
    DATE: when logged
    _TASK: associated task
```

### 5. Analysis & Correlation
**Need**: Document reasoning about evidence

**Extension Set**:
```yaml
# Analysis document
_ANAL:
  uri: https://gedcom.io/x/analysis
  payload: pointer or text
  substructures:
    _SUBJ: subjects analyzed
    _EVID: evidence considered
    _CONCL: conclusions reached
    AUTH: author
    DATE: when written

# Correlation between records
_CORR:
  uri: https://gedcom.io/x/correlation
  payload: correlation type
  substructures:
    _ITEM: correlated items
    _CONF: confidence
    NOTE: reasoning
```

### 6. Negative Evidence
**Need**: Record what we looked for but didn't find

**Extension Set**:
```yaml
# Negative search result
_NSRCH:
  uri: https://gedcom.io/x/negative-search
  payload: what was sought
  substructures:
    DATE: when searched
    _VENUE: where searched
    _SCOPE: search parameters
    NOTE: implications
```

## Mapping Strategy

### From GEDCOM X

| GEDCOM X Concept | GEDCOM 7 Extension |
|------------------|--------------------|
| Person.extracted | INDI with _EXTR |
| Person.evidence | _EVID pointers |
| Event with roles | _EVNT with _PART |
| EvidenceReference | _EVID |
| SourceDescription.sources | _SOUR chains |
| Document | _ANAL |
| Agent | Existing INDI/SUBM |

### From GRAMPS

| GRAMPS Concept | GEDCOM 7 Extension |
|----------------|--------------------|
| Event refs | _EVREF |
| Person refs in Event | _PART |
| Citation hierarchy | _SOUR chains |
| Research notes | _LOG entries |
| Associations | _CORR |

### From Evidence Explained

| EE Concept | GEDCOM 7 Extension |
|------------|--------------------|
| Layered citations | _SOUR (dthaler) |
| Field-based cites | _FIEL (dthaler) |
| Templates | _TPLT (dthaler) |
| Analysis | _ANAL |

## Implementation Order

### Phase 1: Core Evidence Model
1. _EXTR (mark extracted records)
2. _EVID (evidence references)
3. _CONF (confidence levels)

### Phase 2: Event Independence  
1. _EVNT (shared events)
2. _PART (participants)
3. _EVREF (person→event refs)

### Phase 3: Research Process
1. _RSRCH (research activities)
2. _LOG (research log)
3. _ANAL (analysis docs)

### Phase 4: Advanced Features
1. _CORR (correlations)
2. _NSRCH (negative evidence)
3. Additional qualifiers

## Example Usage

```gedcom
0 HEAD
1 SCHMA
2 TAG _EXTR https://gedcom.io/x/extracted
2 TAG _EVID https://gedcom.io/x/evidence
2 TAG _EVNT https://gedcom.io/x/event
2 TAG _SOUR https://github.com/dthaler/gedcom-citations/_SOUR

# Extracted record from 1850 census
0 @E1@ INDI
1 _EXTR
1 NAME John /Smith/
1 BIRT
2 DATE ABT 1810
2 NOTE Age 40 in 1850 census
1 SOUR @S1@
2 PAGE Sheet 5, Line 12

# Extracted record from death certificate  
0 @E2@ INDI
1 _EXTR
1 NAME John /Smith/
1 BIRT  
2 DATE 15 MAR 1809
2 PLAC Ohio
1 DEAT
2 DATE 3 JUN 1885
1 SOUR @S2@

# Conclusion person
0 @I1@ INDI
1 NAME John /Smith/
1 BIRT
2 DATE 15 MAR 1809
2 PLAC Ohio
2 NOTE Death certificate more reliable than census age
1 DEAT
2 DATE 3 JUN 1885
1 _EVID @E1@
2 _CONF 2
2 NOTE Census ages often approximate
1 _EVID @E2@
2 _CONF 4
2 NOTE Death certificate from son

# Shared census event
0 @EV1@ _EVNT
1 TYPE Census
1 DATE 15 AUG 1850
1 PLAC Cuyahoga, Ohio
1 _PART @I1@
2 ROLE Head
2 AGE 40y
1 _PART @I2@
2 ROLE Wife
2 AGE 35y
1 SOUR @S1@
```

## Benefits

1. **Clean Separation**: Evidence vs conclusions clearly marked
2. **Flexibility**: Use only what you need
3. **Traceable**: Can follow reasoning
4. **Compatible**: Maps to multiple systems
5. **Extensible**: Room for growth

## Next Steps

1. Create YAML definitions for each extension
2. Write mapping rules from GEDCOM X
3. Write mapping rules from GRAMPS
4. Test with real genealogical data
5. Submit to GEDCOM registry