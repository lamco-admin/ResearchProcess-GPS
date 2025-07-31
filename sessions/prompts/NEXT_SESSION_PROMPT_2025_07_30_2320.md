# Next Session Prompt - ResearchProcess-GPS
## For Session Starting After 2025-07-30 23:20 EEST

## Context Setting

I'm Greg Lamberson, continuing development of ResearchProcess-GPS, a revolutionary genealogical research system. In the previous session (ending 2025-07-30 23:20 EEST), we completed the conceptual data model including:

1. **Core Genealogical Entities**: Theory (Research Question), Researcher, IdentityPersona, Person, Source, Evidence, Citation, Confidence, and Fact
2. **Research Process Entities**: WorkProduct, ResearchLog, ResearchSession, ResearchActivity
3. **Infrastructure Layer**: Workspace, MethodologyConfig, StandardsRegistry, ModuleConfig

**CRITICAL**: Read COMPREHENSIVE_SESSION_HANDOVER_2025_07_30_2318.md first for complete context.

## Key Principles to Maintain

1. **Standards as Configuration**: GPS, BCG, etc. are YAML/JSON configs, not hard-coded
2. **Everything Has States**: All entities have defined state progressions
3. **Process-First**: Document the research journey, not just conclusions
4. **Confidence as Containers**: Full narratives with assessments, not simple scores
5. **Attribution Throughout**: Every action tracked to a Researcher entity

## Important Corrections Already Made

- Theory and ResearchQuestion are the SAME entity (different user-facing terms)
- Repository is part of Source hierarchy, not standalone
- Citations have states (QUICK→FULL→ELEMENT→ANALYZED), not types
- IdentityPersona is dual-named (Identity OR Persona)
- Person is important - it's the promoted/concluded form of IdentityPersona
- Confidence must be revolutionary (containers with analyses)
- Fact is unified model for events/attributes/characteristics

## Current Status

The conceptual model is COMPLETE. We have:
- All entities defined with relationships
- Three-layer architecture (genealogy/process/infrastructure)
- Extensible vocabulary system for facts
- Full attribution via Researcher entity
- State-based progression for all entities

## Priority Tasks for This Session

1. **Create Formal State Diagrams**
   - Define all states for each entity
   - Specify transition rules and triggers
   - Document side effects of transitions

2. **Design Configuration Schemas**
   - Formalize YAML structure for methodologies
   - Create JSON schemas for validation
   - Define template specification format

3. **Plan Implementation Phases**
   - Prioritize which entities to implement first
   - Define minimal viable subset
   - Create dependency graph

4. **API Surface Design**
   - Entity construction patterns
   - State transition methods
   - Query/filter interfaces
   - Relationship traversal

5. **Storage Architecture**
   - Git adapter detailed design
   - Performance optimization strategies
   - Migration path from existing systems

## Resources to Reference

**Conceptual Model**:
- /home/greg/genealogy-ai/ResearchProcess-GPS/engine/UNIFIED_CONCEPTUAL_MODEL_2025_07_30_2000.md
- /home/greg/genealogy-ai/ResearchProcess-GPS/engine/ENTITY_RELATIONSHIP_REFINEMENT_2025_07_30_2100.md
- /home/greg/genealogy-ai/ResearchProcess-GPS/engine/FACTS_EVENTS_ATTRIBUTES_MODEL_2025_07_30_2150.md

**Infrastructure**:
- /home/greg/genealogy-ai/ResearchProcess-GPS/engine/RESEARCH_PROCESS_ENTITIES_2025_07_30_2105.md
- /home/greg/genealogy-ai/ResearchProcess-GPS/engine/WORKSPACE_METADATA_INFRASTRUCTURE_2025_07_30_2110.md

**Implementation**:
- /home/greg/genealogy-ai/ResearchProcess-GPS/engine/core/models/researcher.py
- /home/greg/genealogy-ai/ResearchProcess-GPS/engine/core/models/confidence_enhanced.py

**Original Vision**:
- /home/greg/genealogy-ai/ResearchProcess-GPS/UNLEASHED_CORE_DATA_MODEL.md
- /home/greg/genealogy-ai/ResearchProcess-GPS/COMPREHENSIVE_CONFIDENCE_FRAMEWORK.md

## Questions to Address

1. Should we implement a minimal subset first (Theory + Evidence + Confidence)?
2. How do we handle state machines - decorator pattern or separate manager?
3. What's the migration story from GEDCOM 7 to our model?
4. How do we ensure plugin/module boundaries are clean?
5. Should workspace configurations be versioned in Git too?

## Remember

- We're building infrastructure for tools, not the tools themselves
- The data model enables functionality, it doesn't implement it
- Flexibility and extensibility are paramount
- Professional genealogists are the target users
- The journey (research process) is as important as the destination (conclusions)

Start by reading the handover document, then assess what aspect of the conceptual model needs the most attention for moving toward implementation. Focus on infrastructure and data model refinements, not tool development.