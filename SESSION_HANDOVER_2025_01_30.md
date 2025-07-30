# ResearchProcess-GPS Session Handover - January 30, 2025

## Session Overview

This session focused on building the core protocol engine for ResearchProcess-GPS, establishing it as a **protocol** (like Git) rather than just another genealogy platform. Major conceptual refinements were made to create a cleaner, more logical model.

## Key Accomplishments

### 1. Conceptual Model Refinement
- **Replaced "Theory" with "Research Question"** as the primary driver
- **Made Analysis a first-class entity** for genealogical interpretation work
- **Working Hypotheses** exist within Research Questions (not as separate entities)
- **Conclusions** are the validated end-state of Research Questions that map to traditional genealogy

### 2. Core Models Created

#### Research Flow Models
- `ResearchQuestion` - What we're trying to answer
- `WorkingHypothesis` - Tentative answers being tested  
- `Analysis` - The interpretive work
- `Conclusion` - GPS-validated answers

#### Domain Models  
- `Identity` & `Persona` - Flexible person representation
- `Evidence` - First-class entity with fact extraction
- `Event` - Floating events (not owned by persons)
- `Relationship` - Multi-party, culturally aware
- `Location` - Temporal-aware places
- `Confidence` - Comprehensive tracking beyond simple scores

### 3. Abstraction Layers Implemented

#### Temporal Abstraction (`temporal.py`)
- Multiple calendar systems (Gregorian, Julian, Hebrew, Islamic, Chinese, etc.)
- Complex date expressions:
  - Date ranges
  - Multiple possible dates
  - NOT dates (explicitly not this date)
  - Approximate dates
  - Relative dates
  - Calculated dates

#### Naming Abstraction (`naming.py`)
- Cultural naming systems:
  - Western (given + middle + surname)
  - Islamic (ism + nasab + laqab + nisba + kunya)
  - Patronymic
  - Chinese/Japanese/Korean
  - Extensible for others

#### Spatial Abstraction (`spatial.py`)
- Multiple coordinate systems (WGS84, NAD83, etc.)
- Address systems (street, rural, traditional)
- Cadastral systems (PLSS, metes & bounds)
- Traditional/indigenous location descriptions

### 4. Nesting Capabilities
- Created `nesting.py` abstraction for hierarchical entities
- Implemented `identity_nesting.py` showing how identities can nest:
  - Collective identities ("The Smith Brothers")
  - Temporal phases (life stages)
  - Uncertain identity groups
  - Family placeholders
  - Theoretical sets for analysis

### 5. Architecture Established

```
ResearchProcess-GPS/
├── engine/
│   ├── core/
│   │   ├── models/         # Domain models
│   │   └── abstractions/   # Cultural/system abstractions
│   ├── protocols/          # Protocol definitions
│   ├── storage/           # Storage abstraction
│   └── adapters/          # External system adapters
```

## Critical Design Decisions

1. **Protocol Over Platform**: RGPS is a protocol others can implement
2. **Research Process Focus**: Track HOW, not just WHAT
3. **Cultural Flexibility**: Not Western-centric
4. **Everything Nests**: Most entities can contain same-type entities
5. **Conclusions Map Backwards**: To traditional genealogy formats

## Current State

- Repository: https://github.com/lamco-admin/ResearchProcess-GPS (private)
- Core models: Complete
- Protocols: Defined
- Abstractions: Implemented
- Next: Implementation of storage backends and adapters

## Key Insights Gained

1. **Research Question → Analysis → Conclusion** is the natural flow
2. **Analysis is crucial** - It's the intellectual work of genealogy
3. **Nesting is fundamental** - Almost everything benefits from hierarchy
4. **Abstractions enable flexibility** - Cultural systems plug in cleanly
5. **GPS compliance** must be built in, not bolted on

## Important Context

### Project Background
- Greg Lamberson is VP of Genealogy for Clan Henderson Society
- Real use case: Managing multiple GEDCOM files, DNA projects, Google Docs
- Privacy is critical - treating all data with DNA-level security
- Goal: "GitHub for Genealogy" - version control for genealogical research

### Technical Stack Decisions
- Django backend (modular architecture)
- Git for version control (hidden from users)  
- PostgreSQL + Apache AGE for graph database
- Private encrypted repositories

### GRAMPS Integration
- GRAMPS adapter design shows liberation/constraint mapping
- RGPS model is far more advanced than GRAMPS
- Import liberates rigid structures
- Export is lossy but functional

## Files to Review

### Conceptual Models
- `/engine/CONCEPTUAL_MODEL_REFINED.md` - Clean conceptual flow
- `/engine/CONCEPTUAL_MODEL_DIAGRAM.md` - Visual representations
- `/v0.1-framework/UNLEASHED_CORE_DATA_MODEL.md` - Original vision

### Implementation
- `/engine/core/models/` - All domain models
- `/engine/core/abstractions/` - Cultural system abstractions
- `/engine/protocols/` - Protocol definitions

### Architecture
- `/engine/ARCHITECTURE.md` - Technical architecture
- `/v0.1-framework/SECURE_MODULAR_FOUNDATION.md` - Django plan

## What Needs Attention

1. **Storage Implementation** - Git, PostgreSQL adapters need building
2. **Theory/Hypothesis Branching** - Core feature needs implementation
3. **Model Integration** - Ensure all models work together properly
4. **Validation & Testing** - Models need comprehensive validation
5. **GRAMPS Adapter** - Actual implementation of the design

## Session Statistics
- Created 15+ model files
- Defined 3 major abstraction layers
- Established protocol architecture
- Set up private GitHub repository
- Completed major conceptual refinement

---

# Next Session Prompt

I'm Greg Lamberson, continuing development of ResearchProcess-GPS - a revolutionary genealogical research protocol (like Git for genealogy).

## Current Status

In the last session, we:
1. Refined the conceptual model: Research Question → Analysis → Conclusion
2. Built core domain models with proper abstractions
3. Implemented cultural flexibility layers (temporal, naming, spatial)
4. Established that most entities support nesting
5. Created the protocol architecture in `/home/greg/genealogy-ai/ResearchProcess-GPS/engine/`

## Key Architecture Points

- **Protocol, not platform** - Others can implement RGPS
- **Research Questions drive everything** (not "Theory")
- **Analysis is a first-class entity** 
- **Conclusions map to traditional genealogy**
- **Everything can nest** (identities, events, locations, etc.)

## Repository Structure

```
/home/greg/genealogy-ai/ResearchProcess-GPS/
├── engine/
│   ├── core/
│   │   ├── models/          # ResearchQuestion, Analysis, Identity, etc.
│   │   └── abstractions/    # temporal.py, naming.py, spatial.py, nesting.py
│   ├── protocols/           # entity_protocol.py, storage_protocol.py, etc.
│   └── adapters/           # GRAMPS adapter design
├── v0.1-framework/         # Original framework docs
└── examples/               # Usage examples
```

## Critical Context

1. I'm VP of Genealogy for Clan Henderson - this has real-world application
2. Privacy is paramount - DNA-level security for all data
3. The model properly separates research process from conclusions
4. Cultural abstractions allow non-Western genealogy
5. Repository is private at https://github.com/lamco-admin/ResearchProcess-GPS

## Next Steps Needed

1. **Review and integrate the models** - Ensure they work together properly
2. **Implement storage backends** - Start with Git storage adapter
3. **Build the branching/merging engine** for Working Hypotheses
4. **Create validation framework** - Models need robust validation
5. **Start GRAMPS adapter implementation** - Based on the design

## Key Questions

1. Should we implement a simple proof-of-concept to validate the model?
2. What's the best approach for the storage abstraction implementation?
3. How should we handle the complexity of nesting in practice?
4. What's the minimal viable feature set to demonstrate the concept?

Please review:
- `/engine/CONCEPTUAL_MODEL_REFINED.md` for the clean model
- `/engine/core/models/` for all domain models
- `/SESSION_HANDOVER_2025_01_30.md` for complete session details

The revolution continues: genealogists will have Git-like capabilities without knowing Git exists!