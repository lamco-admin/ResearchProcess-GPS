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

## Implementation Phases

### Phase 1: Core Meta-Model (Week 1)
1. Create new crate structure: `meta-model-core`
2. Implement fundamental primitives
3. Build property graph system
4. Add temporal abstractions
5. Create basic tests

### Phase 2: Abstraction Layers (Week 2)
1. Implement abstraction layer framework
2. Create GRAMPS XML abstraction
3. Add calendar systems
4. Build media handling
5. Test round-trip preservation

### Phase 3: Data Migration (Week 3)
1. Create migration tools from old model
2. Build compatibility layer
3. Implement gradual migration path
4. Preserve existing data
5. Validate transformations

### Phase 4: Integration (Week 4)
1. Update API to support both models
2. Create query system for meta-model
3. Build UI abstraction layer
4. Implement demonstration
5. Documentation

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