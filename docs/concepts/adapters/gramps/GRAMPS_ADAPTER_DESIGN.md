# GRAMPS Adapter Design for ResearchProcess-GPS

## Overview

The GRAMPS adapter provides bidirectional data flow between GRAMPS and ResearchProcess-GPS. Since RGPS has a far more advanced and flexible model, this adapter must handle significant conceptual differences.

## Key Conceptual Mappings

### GRAMPS → RGPS (Import/Liberation)

| GRAMPS Concept | RGPS Concept | Transformation |
|----------------|--------------|----------------|
| Person | Identity + Personas | Each Person becomes an Identity with initial Persona(s) |
| Family | Relationship (multi-party) | Family splits into parent-child and spousal relationships |
| Event (owned by Person/Family) | Event (floating) | Events liberated from ownership, linked via participation |
| Source | Evidence | Sources become Evidence with richer classification |
| Citation | Evidence extraction point | Citations become specific fact extractions |
| Note | Research notes + Theory documentation | Notes analyzed for research process info |
| Place | Location (temporal-aware) | Places gain temporal dimensions and jurisdiction history |
| Repository | Repository + Chain of custody | Repositories enhanced with provenance tracking |

### RGPS → GRAMPS (Export/Constraint)

| RGPS Concept | GRAMPS Mapping | Information Loss |
|--------------|----------------|------------------|
| Theory branches | Single conclusion + Notes | Theory history flattened to notes |
| Floating evidence | Citations attached to conclusions | Evidence forced to attach to persons/families |
| Identity with multiple personas | Best-guess Person | Personas merged based on active theory |
| Multi-party relationships | Family records | Complex relationships simplified |
| Temporal locations | Simple Places | Time-based changes lost |
| GPS compliance tracking | Notes | Structured compliance becomes text |
| Confidence containers | Simple confidence number | Rich audit trails reduced to 0-4 scale |

## Import Process (GRAMPS → RGPS)

### Phase 1: Entity Liberation

```python
def liberate_gramps_person(gramps_person):
    # Create flexible Identity
    identity = Identity(
        id=generate_uuid(),
        existence_status="evidenced",  # Assume GRAMPS data is concluded
    )
    
    # Create initial Persona from GRAMPS Person
    persona = Persona(
        identity_id=identity.id,
        name_forms=extract_names(gramps_person),
        temporal_range=extract_life_span(gramps_person),
        confidence=map_gramps_confidence(gramps_person.confidence)
    )
    
    # Extract all events as floating events
    for event_ref in gramps_person.event_refs:
        liberated_event = liberate_event(event_ref, identity.id)
        # Event now exists independently
    
    return identity, persona
```

### Phase 2: Relationship Transformation

```python
def transform_gramps_family(gramps_family):
    relationships = []
    
    # Parent-child relationships
    for parent_handle in [gramps_family.father, gramps_family.mother]:
        if parent_handle:
            for child_handle in gramps_family.children:
                rel = Relationship(
                    participants=[
                        {"identity": parent_handle, "role": "parent"},
                        {"identity": child_handle, "role": "child"}
                    ],
                    relationship_type={"primary": "biological", "subtype": "parent-child"}
                )
                relationships.append(rel)
    
    # Spousal relationship
    if gramps_family.father and gramps_family.mother:
        rel = Relationship(
            participants=[
                {"identity": gramps_family.father, "role": "spouse"},
                {"identity": gramps_family.mother, "role": "spouse"}
            ],
            relationship_type=determine_relationship_type(gramps_family)
        )
        relationships.append(rel)
    
    return relationships
```

### Phase 3: Evidence Enhancement

```python
def enhance_gramps_source(gramps_source):
    evidence = Evidence(
        source_classification=classify_source(gramps_source),
        provenance=extract_provenance(gramps_source),
        quality_metrics=assess_source_quality(gramps_source)
    )
    
    # Extract all citations as fact extractions
    for citation in find_citations(gramps_source):
        fact = ExtractedFact(
            evidence_id=evidence.id,
            fact_type=determine_fact_type(citation),
            extracted_value=citation.text,
            applies_to=citation.referenced_object
        )
        evidence.extracted_facts.append(fact)
    
    return evidence
```

## Export Process (RGPS → GRAMPS)

### Theory Resolution

Before export, must resolve active theory into GRAMPS-compatible conclusions:

```python
def resolve_theory_for_export(theory, resolution_strategy="most_confident"):
    # Collect all identities in theory
    for identity in theory.identities:
        gramps_person = create_gramps_person()
        
        # Merge personas based on strategy
        if resolution_strategy == "most_confident":
            primary_persona = identity.get_highest_confidence_persona()
        elif resolution_strategy == "user_selected":
            primary_persona = user_select_persona(identity.personas)
        
        # Map to GRAMPS person
        map_persona_to_person(primary_persona, gramps_person)
        
        # Attach relevant events
        for event in theory.events_for_identity(identity):
            gramps_person.add_event_ref(event)
```

### Information Preservation

Critical RGPS information preserved in GRAMPS notes:

```python
def preserve_rgps_metadata(rgps_entity, gramps_entity):
    metadata_note = Note()
    metadata_note.set_format("rgps-metadata")
    
    metadata = {
        "rgps_version": "1.0",
        "original_id": str(rgps_entity.id),
        "theory_context": rgps_entity.theory_context,
        "confidence_details": rgps_entity.confidence.to_dict(),
        "research_log": rgps_entity.research_trail,
        "alternative_interpretations": rgps_entity.alternatives
    }
    
    metadata_note.set_text(json.dumps(metadata, indent=2))
    gramps_entity.add_note(metadata_note.handle)
```

## Sync Strategy

### Live Sync Challenges

1. **Theory Management**: GRAMPS changes map to new theory branch
2. **Conflict Detection**: Changes to same entities in both systems
3. **Round-trip Fidelity**: Preserving RGPS enhancements

### Sync Implementation

```python
class GRAMPSLiveSync:
    def on_gramps_change(self, change_event):
        # Create new micro-theory for GRAMPS changes
        theory = Theory(
            hypothesis=f"GRAMPS sync: {change_event.description}",
            parent_theory=self.current_theory
        )
        
        # Apply changes in theory context
        if change_event.type == "person_modified":
            identity = self.find_identity_for_gramps_person(change_event.handle)
            updated_persona = self.extract_persona_changes(change_event)
            theory.update_persona(identity, updated_persona)
        
        # Optionally auto-merge if no conflicts
        if self.can_auto_merge(theory):
            self.current_theory.merge(theory)
```

## Critical Design Decisions

1. **Import is Liberation**: Don't just map 1:1, free the data from GRAMPS constraints
2. **Export is Best-Effort**: Accept that some RGPS richness will be lost
3. **Preserve Everything**: Use notes/attributes to maintain round-trip data
4. **Theory-Aware**: Each GRAMPS import/sync creates a theory branch
5. **User Control**: Let users decide how to resolve theories for export

## Future Enhancements

1. **GRAMPS Plugin**: Native GRAMPS addon for tighter integration
2. **Smart Mapping**: ML-based persona resolution
3. **Incremental Sync**: Only sync changes, not full database
4. **Conflict UI**: Visual conflict resolution tools