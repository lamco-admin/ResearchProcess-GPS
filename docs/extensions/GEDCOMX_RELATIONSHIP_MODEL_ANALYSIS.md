# GEDCOM X Relationship Model Analysis

## GEDCOM X Relationship Structure

### Basic Model
```
Relationship {
  type: Enumerated Value (e.g., Couple, ParentChild)
  person1: URI reference
  person2: URI reference  
  facts: List of Facts about the relationship
  
  // Inherited from Subject
  extracted: boolean
  evidence: List of EvidenceReferences
  media: List of SourceReferences
  identifiers: List of Identifiers
  
  // Inherited from Conclusion
  confidence: Enumerated confidence level
  lang: language tag
  attribution: Attribution
  notes: List of Notes
  sources: List of SourceCitations
}
```

### What GEDCOM X Does Well

#### 1. First-Class Relationship Records ✅
- Relationships are top-level entities, not embedded in persons
- Can have their own identifiers and be referenced
- Can be exported/imported independently

#### 2. Relationship Facts ✅
```
Relationship {
  type: "http://gedcomx.org/Couple"
  person1: "#p1"
  person2: "#p2"
  facts: [
    {
      type: "http://gedcomx.org/Marriage"
      date: "12 June 1850"
      place: "Boston, MA"
    },
    {
      type: "http://gedcomx.org/Divorce"  
      date: "15 March 1865"
    }
  ]
}
```
- Relationships can have facts (marriage, divorce, etc.)
- Facts can have dates and places
- Multiple facts per relationship

#### 3. Evidence and Attribution ✅
- Full citation support (inherited from Conclusion)
- Evidence references for hypothesis relationships
- Attribution tracking
- Confidence levels

#### 4. Extensible Type System ✅
- Base types: Couple, ParentChild, AncestorDescendant
- Additional types: EnslavedBy, Godparent
- URI-based system allows custom types

### What GEDCOM X Still Lacks

#### 1. Only Two-Person Relationships ❌
```
// Cannot model:
- Three-way business partnership
- Group membership
- Multi-party legal agreements
```

#### 2. No Direct Time Boundaries ❌
```
// Have to use facts as workaround:
facts: [
  { type: "Start", date: "1850" },
  { type: "End", date: "1865" }
]
```

#### 3. No Asymmetric Roles ❌
```
// person1 and person2 - but what are their roles?
// Type implies roles but doesn't specify them
// "EnslavedBy" - but which person is which?
```

#### 4. Limited Built-in Types ❌
Only 5 standard types:
- AncestorDescendant
- Couple
- EnslavedBy
- Godparent
- ParentChild

Missing: Friend, Neighbor, BusinessPartner, Guardian, etc.

## Comparison with Proposed LINK Extension

### GEDCOM X Advantages
1. **Top-level records** - Already implemented
2. **Facts on relationships** - Can have events/dates
3. **Full evidence model** - Citations, confidence
4. **Proven implementation** - FamilySearch uses it

### LINK Extension Advantages  
1. **Multi-party support** - N persons in relationship
2. **Explicit roles** - Each party has defined role
3. **Direct time support** - _PERIOD structure
4. **Richer type vocabulary** - More relationship types

### Key Differences

**GEDCOM X**:
```json
{
  "type": "http://gedcomx.org/Godparent",
  "person1": "#p1",  // Godparent (implied)
  "person2": "#p2",  // Godchild (implied)
  "facts": [{
    "type": "http://gedcomx.org/Baptism",
    "date": "1850-05-15"
  }]
}
```

**LINK Extension**:
```gedcom
0 @L1@ _LINK
1 TYPE Godparent
1 _PARTY @I1@
2 ROLE Godfather
1 _PARTY @I2@
2 ROLE Godchild
1 _PARTY @I3@
2 ROLE Godmother
1 _OCUR @O1@  # Links to baptism occurrence
```

## Lessons from GEDCOM X

### What Worked
1. **First-class relationships** - Not embedded in persons
2. **Facts on relationships** - Dates/places for relationship events
3. **Evidence support** - Full citation capability
4. **Extensible types** - URI-based system

### What Didn't Work
1. **Two-person limit** - Major constraint
2. **Adoption challenges** - Complex model, limited uptake
3. **Role ambiguity** - person1/person2 unclear
4. **Type vocabulary** - Too limited for genealogy

### Why Limited Adoption?
1. **Complexity** - Full RDF/linked data model
2. **FamilySearch-centric** - Designed for their needs
3. **Breaking change** - Not backward compatible
4. **Tooling** - Limited software support

## Implications for LINK Extension

### Adopt from GEDCOM X
1. First-class relationship records
2. Facts/events on relationships
3. Evidence and confidence support
4. Extensible type system

### Improve Upon GEDCOM X
1. Support N-party relationships
2. Explicit role definitions
3. Direct time boundary support
4. Simpler, GEDCOM-compatible model

### Key Design Question
**Should LINK align with GEDCOM X concepts for future compatibility?**

Pros:
- Conceptual alignment with modern standard
- Easier future migration path
- Proven model elements

Cons:
- GEDCOM X has limited adoption
- Different philosophy (RDF vs hierarchical)
- May inherit complexity

## Conclusion

GEDCOM X solved many relationship modeling problems but created new ones through complexity and limited flexibility. The LINK extension can learn from both its successes (first-class relationships, facts, evidence) and its limitations (two-person only, complex model, limited types) to create a more practical solution for GEDCOM 7 users.