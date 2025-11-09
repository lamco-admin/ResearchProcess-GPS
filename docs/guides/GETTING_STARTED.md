# Getting Started with ResearchProcess-GPS

## Overview

ResearchProcess-GPS is a universal meta-model platform for collaborative genealogical research. Unlike traditional genealogy software that forces you into a specific data model (GEDCOM, GRAMPS, etc.), ResearchProcess-GPS provides **universal primitives** that can adapt to any data structure through schema definitions.

### Key Features

- **Universal Meta-Model**: Four core primitives (Entity, Relationship, Context, Certainty) can express any data model
- **Schema-Based**: Define your data structure with YAML/JSON schemas
- **Multiple Data Models**: Work with GEDCOM, GRAMPS, and custom models simultaneously
- **WebAssembly**: Runs entirely in your browser with native performance
- **Offline-First**: All data stored locally in IndexedDB
- **Theory Versioning**: Multiple research theories can coexist
- **Advanced Uncertainty**: Quantum, Fuzzy, Bayesian, and Narrative certainty models
- **Calendar-Agnostic**: Support for multiple calendar systems

## Quick Start (5 Minutes)

### Prerequisites

1. **Rust** (1.75+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. **wasm-pack**
   ```bash
   cargo install wasm-pack
   ```

3. **Node.js** (v18+)
   ```bash
   # Install from https://nodejs.org/
   ```

### Installation

1. **Clone the Repository**
   ```bash
   git clone https://github.com/yourusername/researchprocess-gps.git
   cd researchprocess-gps
   ```

2. **Install Web Dependencies**
   ```bash
   cd web
   npm install
   ```

3. **Build WASM Module**
   ```bash
   npm run wasm:build
   # Or for faster development builds:
   # npm run wasm:dev
   ```

4. **Start Development Server**
   ```bash
   npm run dev
   ```

5. **Open in Browser**
   ```
   Navigate to: http://localhost:3000
   ```

You should now see the ResearchProcess-GPS web interface!

## Your First Workflow

### 1. Load a Schema

1. Click the **"Schemas"** tab
2. Select **"GEDCOM Basic"** from the sidebar
3. Click **"Load Schema"**

The schema defines the structure for your data (Person, Family, Source entity types, etc.).

### 2. Create Entities

1. Click the **"Entities"** tab
2. Enter entity type: **"Person"**
3. Click **"Create Entity"**
4. The entity appears in the list

### 3. Add Properties

1. Select the entity you just created
2. In "Property Key", enter: **name**
3. In "Property Value", enter: **John Smith**
4. Click **"Set Property"**
5. Repeat for other properties:
   - Key: `birth_date`, Value: `1850-03-15`
   - Key: `birth_place`, Value: `New York, NY`

### 4. Save Your Work

1. Click the **"Workspace"** tab
2. Click **"Save to IndexedDB"**
3. Your data is now persisted locally

Your workspace persists in your browser's IndexedDB. You can close the browser and return later.

## Core Concepts

### Universal Meta-Model

Traditional genealogy software uses hardcoded data models:

```
❌ Traditional: 17 hardcoded entity types
✅ ResearchProcess-GPS: 4 universal primitives
```

The four primitives are:

#### 1. Entity
A universal object that can represent anything:
- Person, Family, Source (genealogy)
- Document, Hypothesis, Evidence (research)
- Location, Organization, Event (any domain)

```rust
let person = Entity::new("Person");
person.set_property("name", "John Smith");
person.set_property("birth_date", TemporalValue::from_date(1850, 3, 15));
```

#### 2. Relationship
N-ary relationships with typed participants:

```rust
let parent_child = Relationship::new("Parent-Child");
parent_child.add_participant(father_id, "parent");
parent_child.add_participant(mother_id, "parent");
parent_child.add_participant(child_id, "child");
```

#### 3. Context
Scope for theories and research contexts:

```rust
let theory_a = Context::new("Theory A: Immigrant Hypothesis");
let theory_b = Context::new("Theory B: Native Born");

// Same entity can exist in multiple theories
entity.add_context(theory_a);
```

#### 4. Certainty
Multiple uncertainty models:

```rust
// Quantum: Multiple possible states
Certainty::Quantum(vec![
    ("Born in NY".to_string(), 0.7),
    ("Born in PA".to_string(), 0.3),
])

// Fuzzy: Degree of truth
Certainty::Fuzzy {
    membership: 0.8,
    confidence: 0.6
}

// Bayesian: Prior and likelihood
Certainty::Bayesian {
    prior: 0.5,
    likelihood: 0.9,
    posterior: 0.82
}

// Narrative: Textual explanation
Certainty::Narrative("Based on family tradition...".to_string())
```

### Schemas

Schemas define the structure and validation rules for your data model:

```yaml
schema:
  id: my-schema
  version: "1.0.0"
  name: My Research Model

entity_types:
  Person:
    required_properties:
      - name
    properties:
      name:
        type: Text
        validation:
          - rule: Required
      birth_date:
        type: Temporal
```

Schemas are **optional but recommended**. They provide:
- Validation
- Documentation
- Tool support
- Import/export compatibility

### Properties

Properties use a flexible graph structure:

```rust
// Simple value
property.set_text("name", "John");

// Nested entity
property.set_entity("residence", address_entity);

// Collection
property.set_collection("children", vec![child1, child2, child3]);

// Computed value
property.set_computation("age_at_death", "death_date - birth_date");

// Quantum state
property.set_quantum("location", vec![
    ("New York", 0.7),
    ("Pennsylvania", 0.3)
]);
```

## Working with Multiple Models

The power of the universal meta-model is that you can work with multiple data models simultaneously:

### Example: GEDCOM and GRAMPS Together

```rust
// Load both schemas
workspace.load_schema_yaml(gedcom_schema);
workspace.load_schema_yaml(gramps_schema);

// Create GEDCOM-style entities
workspace.set_active_schema("gedcom-7");
let gedcom_person = workspace.create_entity("Individual");

// Create GRAMPS-style entities
workspace.set_active_schema("gramps");
let gramps_person = workspace.create_entity("Person");

// Both validate against their respective schemas!
```

### Cross-Model Relationships

You can even create relationships across different models:

```rust
let cross_model = Relationship::new("SameAs");
cross_model.add_participant(gedcom_person.id, "gedcom_entity");
cross_model.add_participant(gramps_person.id, "gramps_entity");
```

## Browser Storage

All data is stored in IndexedDB:

- **Workspace**: Overall workspace metadata
- **Entities**: All entity objects
- **Relationships**: All relationship objects
- **Schemas**: Loaded schema definitions

```javascript
// In the web interface:
const storage = new WasmStorage('research-gps-db');
await storage.init();

// Save workspace
await storage.save_workspace(workspace.stats());

// Load workspace
const data = await storage.load_workspace();

// Clear all data
await storage.clear();
```

## Next Steps

### Learn More

1. **[Schema Definition Guide](SCHEMA_GUIDE.md)** - Create custom schemas
2. **[Developer Guide](DEVELOPER_GUIDE.md)** - Build adapters and plugins
3. **[API Reference](../api/)** - Complete Rust and JavaScript API

### Example Projects

- **GEDCOM Importer**: Convert GEDCOM files to universal model
- **GRAMPS Connector**: Sync with GRAMPS database
- **Custom Research Model**: Define your own research methodology

### Advanced Features

- **Temporal Reasoning**: Calendar-agnostic date calculations
- **Spatial Analysis**: Geographic clustering and relationships
- **Theory Branching**: Explore alternate hypotheses
- **Provenance Tracking**: Complete audit trail
- **Constraint Validation**: Custom business rules

## Troubleshooting

### "WASM module not found"

Build the WASM module:
```bash
cd web
npm run wasm:build
```

### "Schema validation failed"

Check that:
1. Schema is valid YAML
2. Required properties are present
3. Property types match schema
4. Entity type exists in schema

### "Entity not found"

Entities must be added to workspace:
```javascript
const entity = workspace.create_entity("Person");
workspace.add_entity(entity);  // Don't forget this!
```

### Build Errors

Clean and rebuild:
```bash
cd web
rm -rf pkg/ node_modules/ dist/
npm install
npm run wasm:build
npm run dev
```

## Get Help

- **Documentation**: `docs/`
- **Examples**: `examples/`
- **Issues**: https://github.com/yourusername/researchprocess-gps/issues
- **Discussions**: https://github.com/yourusername/researchprocess-gps/discussions

## Philosophy

ResearchProcess-GPS is built on these principles:

1. **Flexibility First**: Never force a specific data model
2. **Explicit is Better**: No fallbacks, no silent failures
3. **Theory-Aware**: Multiple hypotheses can coexist
4. **Uncertainty-Tolerant**: Embrace ambiguity with proper models
5. **Offline-First**: Your data stays on your device
6. **Open Standards**: Based on proven models (GEDCOM, GRAMPS)

Welcome to collaborative genealogical research without constraints!
