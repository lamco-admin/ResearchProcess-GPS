# Meta-Model Transformation Plan

## Overview

Transform ResearchProcess-GPS from a concrete genealogical system to a universal meta-model that can express ANY genealogical data model, research methodology, and workflow system.

## Current State

- **Branch**: `meta-model-transformation`
- **Status**: Experimental prototypes completed
- **Location**: `/experiments/meta-model/`

## Architecture Shift

### From (Current)
- Concrete types: `IdentityPersona`, `LocationRef`, `Theory`
- Fixed genealogical model
- GPS-specific research process
- Hardcoded workflow

### To (Meta-Model)
- Abstract primitives: `Entity`, `Relationship`, `Context`, `Certainty`
- Universal data model via property graphs
- Any research methodology expressible
- Any workflow system integratable

## Implementation Approach

### All Layers Together
We implement all three layers simultaneously as they're interdependent:
- Layer 1 needs abstraction layers to be useful
- Layer 2/3 need Layer 1's primitives
- Abstraction layers tie everything together

### Abstraction Layer Storage

Abstraction layers will be stored as:

1. **Built-in Abstractions** (Compiled into binary)
   - GRAMPS XML
   - GEDCOM
   - Calendar systems (Gregorian, Julian, etc.)
   - Common naming systems

2. **Plugin Abstractions** (Dynamic loading)
   - Directory: `~/.researchprocess-gps/abstractions/`
   - Format: WASM modules or Rust dylibs
   - Auto-discovered on startup

3. **User Abstractions** (Configuration)
   - Directory: `~/.researchprocess-gps/user-abstractions/`
   - Format: JSON/YAML transformation rules
   - For simple format mappings

4. **Registry Storage**
   ```
   abstractions/
   ├── builtin/              # Compiled abstractions
   ├── plugins/              # Dynamic abstractions
   │   ├── familysearch.wasm
   │   └── ancestry.wasm
   └── user/                 # User-defined
       ├── my-format.yaml
       └── custom-calendar.json
   ```

### Implementation Tasks

1. **Core Infrastructure**
   - Create `meta-model-core` crate
   - Set up all three layers
   - Build property graph system
   - Implement abstraction registry

2. **Abstraction Framework**
   - Define abstraction trait/interface
   - Create plugin loading system
   - Build transformation engine
   - Add validation system

3. **Standard Abstractions**
   - GRAMPS XML (complete with attributes)
   - Basic GEDCOM support
   - Gregorian/Julian calendars
   - Western naming conventions

4. **Storage Layer**
   - Entity storage (likely SQLite with JSON)
   - Relationship indexing
   - Query optimization
   - Abstraction caching

## Key Decisions

### What to Keep
- Research process focus
- GPS methodology support (as one option)
- Existing API structure (adapted)
- Current data (migrated)

### What Changes
- Data model becomes abstract
- All concrete types become entity types
- Enums become open strings
- Fixed relationships become flexible

### What's New
- Universal expressibility
- Quantum states
- Theoretical values
- Abstraction layers
- Cross-model queries

## Next Steps

1. **Prototype Validation** ✓
   - Theoretical model works
   - Can express multiple genealogical models
   - Supports identity correlation scenario

2. **Architecture Design** ← We are here
   - Decide on module structure
   - Plan API changes
   - Design migration strategy

3. **Implementation Start**
   - Begin with core primitives
   - Add abstraction layers
   - Create examples

## Directory Structure

```
ResearchProcess-GPS/
├── meta-model-core/          # New core implementation
│   ├── src/
│   │   ├── primitives/       # Entity, Relationship, etc.
│   │   ├── properties/       # Property graph system
│   │   ├── temporal/         # Time handling
│   │   └── abstractions/     # Format layers
│   └── tests/
├── meta-model-api/           # API layer
├── meta-model-migration/     # Migration tools
├── examples/                 # Usage examples
└── experiments/              # Current prototypes
```

## Questions to Resolve

1. Keep ResearchProcess-GPS name or rename project?
2. Maintain backward compatibility or clean break?
3. Single repository or multi-repo?
4. How to handle existing installations?

## Benefits

- **Universal**: Works with ANY genealogical model
- **Flexible**: Adapts to user needs, not vice versa
- **Preserving**: Never loses data through transformations
- **Extensible**: New models without code changes
- **Theoretical**: Supports research, not just conclusions

## Risks

- Complexity of abstraction
- Performance considerations
- Learning curve for users
- Migration effort

## Recommendation

Proceed with implementation in current repository on new branch. This allows:
- Side-by-side comparison
- Gradual migration option
- Preservation of history
- Community involvement

The meta-model represents the true vision: a system that adapts to how people think about genealogy, not forcing them into one model.