# BetterGEDCOM Events and Attributes Analysis
## 2025-07-30 21:45 EEST

Based on mining the BetterGEDCOM database, here are the key findings about how different approaches handle Events and Attributes:

## Key Discussions Found

### 1. **UGM - Unified Genealogical Model** (Discussion 380)
- Emphasizes modeling the "genealogical world" (evidence & conclusions) rather than the "real-life world"
- Events should be part of a representational model that describes genealogical data structure
- Should support both pure evidence and pure conjecture

### 2. **STEMMA Extended Vocabularies** (Discussion 146)
- Unified custom types, sub-types, roles, and properties into a single extensible scheme
- Properties (like Age, Occupation) are no longer hard-coded but use extended vocabularies
- Multi-role events are supported through this vocabulary system

### 3. **User Extensibility of Events and Characteristics** (Syntax05 requirement)
- Events, properties, and characteristics MUST be extensible by users
- Extensions must be kept permanently separate from standard definitions
- This prevents conflicts when standards evolve

### 4. **Events as Values, not Defining Entities**
- Discussion about making events act as qualifiers/properties rather than standalone entities
- Example: "birth" as a property of a person rather than a separate birth event entity
- This approach simplifies relationships and reduces complexity

## Key Concepts Emerging

### 1. **Events vs Facts vs Attributes**
BetterGEDCOM discussions reveal three overlapping concepts:

**Events**: 
- Have temporal and spatial dimensions
- Can involve multiple participants with different roles
- Examples: Birth, Marriage, Census enumeration

**Facts**: 
- Assertions about an entity at a point in time
- May or may not have location
- Examples: Occupation, Religion, Military rank

**Attributes/Characteristics**:
- Properties that describe an entity
- May be static or change over time
- Examples: Hair color, Height, Language spoken

### 2. **Multi-Role Participation**
- Events should support multiple participants with different roles
- Roles should be extensible, not hard-coded
- Example: A baptism has the person baptized, officiant, witnesses, godparents

### 3. **Extensibility is Critical**
- Users MUST be able to define custom event types
- Custom attributes/properties must be supported
- Namespace separation prevents conflicts with future standards

### 4. **Evidence-Based Approach**
- Events/facts/attributes come from evidence
- The same evidence might support multiple interpretations
- Confidence levels apply to the interpretation, not the evidence

## Comparison with GEDCOM X

GEDCOM X takes a similar but more structured approach:
- **Facts** are the primary concept (encompassing events and attributes)
- Facts have:
  - Type (birth, death, occupation, etc.)
  - Date (optional)
  - Place (optional)
  - Value (for non-event facts)
  - Confidence level
  - Attribution

## Recommendations for ResearchProcess-GPS

### 1. **Unified Fact Model**
```python
class Fact(NestableBaseEntity):
    """
    Unified model for events, attributes, and characteristics.
    Can represent both conclusions and evidence extractions.
    """
    fact_type: FactType  # EVENT, ATTRIBUTE, CHARACTERISTIC
    fact_subtype: str    # User-extensible: "birth", "occupation", "height"
    
    # Temporal aspect (optional)
    date: Optional[TemporalDescription]
    
    # Spatial aspect (optional)
    place: Optional[PlaceReference]
    
    # Value (for non-event facts)
    value: Optional[str]
    
    # Participants (for events)
    participants: List[FactParticipant]
    
    # Evidence basis
    evidence_references: List[EvidenceReference]
    confidence: Confidence  # Full container
    
    # State tracking
    state: FactState  # EXTRACTED → INTERPRETED → CONCLUDED → QUESTIONED
```

### 2. **Extensible Vocabularies**
```yaml
# config/vocabularies/fact_types.yaml
standard_facts:
  events:
    birth:
      requires_date: true
      requires_place: true
      standard_roles: ["principal", "mother", "father", "witness"]
    marriage:
      requires_date: true
      requires_place: true
      standard_roles: ["groom", "bride", "witness", "officiant"]
      
  attributes:
    occupation:
      requires_date: false
      requires_place: false
      has_value: true
      
custom_facts:
  namespace: "user.custom"
  definitions:
    - type: "DNA_test"
      fact_type: EVENT
      standard_roles: ["test_subject", "test_company"]
```

### 3. **Multi-Participant Events**
```python
@dataclass
class FactParticipant:
    """Participant in a fact/event"""
    identity_id: UUID  # IdentityPersona or Person
    role: str         # Extensible vocabulary
    role_confidence: Confidence
    notes: Optional[str]
```

### 4. **State-Based Facts**
- Facts extracted from evidence start as EXTRACTED
- Interpretation adds context (INTERPRETED)
- Conclusion promotes to CONCLUDED
- Can be QUESTIONED if new evidence emerges

## For ResearchSession Clarification

Based on the discussions, ResearchSession should indeed encompass:
- **Research trips** (repository visits, cemetery trips)
- **Interviews** (oral history sessions)
- **Billable time tracking** (professional genealogy needs)
- **Online research sessions**
- **Analysis sessions** (no new evidence, just correlation)

The key is that ResearchSession documents the research PROCESS, while Facts document what was FOUND.

## Next Steps

1. Define the Fact entity with extensible vocabulary support
2. Create FactParticipant for multi-role events
3. Design the vocabulary configuration system
4. Ensure Facts can be nested (compound facts)
5. Link Facts to both Evidence (source) and IdentityPersona (subject)