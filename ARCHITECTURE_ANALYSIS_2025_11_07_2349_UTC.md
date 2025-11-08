# ResearchProcess-GPS Architecture Analysis & Refactoring Plan

### Timestamp: 2025-11-07 23:49:00 UTC

## Executive Summary

This document analyzes the ResearchProcess-GPS collaborative genealogical research platform and presents a comprehensive refactoring plan to address the core architectural limitation: **an inflexible, domain-specific data model that cannot adapt to different research methodologies or data structures**.

## Current State Analysis

### Repository Structure

The codebase consists of multiple branches representing different evolutionary stages:

1. **claude/examine-collaborative-code-011CUuRNcf8tFHcjjfsy1367** (current)
   - Most recent implementation
   - Includes module system (WASM + native)
   - Event sourcing architecture
   - WebSocket real-time sync
   - PostgreSQL storage with JSONB
   - Well-developed API layer

2. **meta-model-transformation**
   - Experimental branch with universal meta-model
   - Contains theoretical implementations
   - Demonstrates pluggable data model approach
   - NOT yet integrated with main architecture

3. **feature/meta-model-merge-1**
   - Attempted merge of meta-model work
   - Status unknown

### Technology Stack

- **Language**: Rust (2021 edition)
- **Database**: PostgreSQL with JSONB support
- **Architecture**: Event-sourced with CQRS patterns
- **API**: REST (Axum) + WebSocket real-time
- **Modules**: WASM + Native FFI
- **Storage Backends**: PostgreSQL, Git, Filesystem

### Workspace Crates

```
crates/
├── rp-core              # Core entity definitions (HARDCODED)
├── rp-protocol          # Protocol layer
├── rp-engine            # Business logic engine
├── rp-storage           # Storage abstraction
├── rp-storage-postgres  # PostgreSQL implementation
├── rp-storage-git       # Git implementation
├── rp-storage-fs        # Filesystem implementation
├── rp-network           # Network layer
├── rp-modules           # Module system
├── rp-server            # REST + WebSocket server
├── rp-client            # Client library
├── rp-cli               # Command-line interface
├── rp-events            # Event sourcing
└── rp-module-sdk        # Module development SDK
```

## The Core Problem

### Hardcoded Genealogical Data Model

The current `rp-core` crate defines **concrete, genealogy-specific types**:

```rust
// Hardcoded entity types
- IdentityPersona      // Person in genealogical research
- Theory               // Research hypothesis
- Evidence             // Source evidence
- Source               // Historical sources
- Citation             // Source citations
- Fact                 // Biographical facts
- Relationship         // Family relationships
- Location             // Geographic locations
- WorkProduct          // GPS-compliant work products
- ResearchLog          // Research activity logs
- ProofStatement       // GPS proof statements
- AnalysisReport       // Evidence analysis
```

Each entity has **fixed fields** tailored to genealogy:

```rust
pub struct Theory {
    pub question: String,              // Research question
    pub evidence: Vec<EntityId>,       // Evidence IDs
    pub geographic_scope: Option<GeographicScope>,
    pub temporal_scope: Option<TemporalScope>,
    pub research_log: Vec<ResearchLogEntry>,
    // ... many more genealogy-specific fields
}

pub struct IdentityPersona {
    pub identity_type: IdentityType,   // Named, Described, etc.
    pub given_names: Vec<NameComponent>,
    pub surname: Option<NameComponent>,
    pub evidence_refs: Vec<EvidenceReference>,
    // ... genealogy-specific attributes
}
```

### Consequences of the Hardcoded Model

1. **Cannot adapt to different research domains**
   - Archaeological research needs different entity types
   - Scientific research has different relationships
   - Even different genealogical methodologies (Y-DNA vs traditional) need different structures

2. **Cannot accommodate new genealogical standards**
   - GEDCOM evolves
   - BCG/GPS standards change
   - New methodologies emerge

3. **Extensibility requires core changes**
   - Adding new entity types requires modifying `rp-core`
   - New attributes require schema changes
   - Every extension breaks existing deployments

4. **Storage layer coupling**
   - PostgreSQL schemas are tied to concrete types
   - Migration becomes complex
   - Data transformation is lossy

5. **API inflexibility**
   - REST endpoints are type-specific
   - Cannot query across different models
   - Limited composability

## The Meta-Model Solution

The `meta-model-transformation` branch contains a brilliant solution: **universal primitives** that can express ANY data model.

### Four Universal Primitives

```rust
1. Entity          // Universal container for anything
2. Relationship    // Universal connection between entities
3. Context         // Universal qualifier/scope
4. Certainty       // Universal uncertainty expression
```

### Entity: The Universal Container

```rust
pub struct Entity {
    pub id: EntityId,
    pub entity_type: String,           // OPEN-ENDED: any type
    pub state: String,                 // OPEN-ENDED: any state
    pub properties: PropertyGraph,     // FLEXIBLE: any properties
    pub relationships: Vec<RelationshipId>,
    pub contexts: Vec<Context>,
    pub meta: MetaInfo,
}
```

**Key insight**: Instead of `Theory` and `IdentityPersona` as different structs, they become:
- `Entity { entity_type: "Theory", properties: {...} }`
- `Entity { entity_type: "IdentityPersona", properties: {...} }`
- `Entity { entity_type: "DNASample", properties: {...} }`  (new type!)
- `Entity { entity_type: "ArchaeologicalArtifact", properties: {...} }`  (different domain!)

### Relationship: N-ary, Flexible Connections

```rust
pub struct Relationship {
    pub id: RelationshipId,
    pub relationship_type: String,     // OPEN: any relationship
    pub participants: Vec<Participant>, // N-ary (not just binary!)
    pub properties: PropertyGraph,
    pub contexts: Vec<Context>,
}

pub struct Participant {
    pub entity: EntityId,
    pub role: String,                  // OPEN: any role
    pub certainty: Certainty,
}
```

**Examples**:
- `Relationship { type: "Kinship.Parent", participants: [parent, child] }`
- `Relationship { type: "Identity.PossibleSame", participants: [persona1, persona2] }`
- `Relationship { type: "DNA.Match", participants: [person, sample, lab] }`  (3-way!)

### Context: Universal Qualifier

```rust
pub struct Context {
    pub context_type: String,
    pub scope: Scope,
    pub certainty: Certainty,
    pub properties: PropertyGraph,
}

pub enum Scope {
    Temporal(TemporalScope),   // Time-based
    Spatial(SpatialScope),     // Location-based
    Cultural(CulturalScope),   // Cultural context
    Theoretical(TheoreticalScope), // Research theory
    Evidential(EvidentialScope),   // Source evidence
    Custom(String, PropertyGraph), // Anything else!
}
```

**Power**: The same entity can exist in multiple contexts:
- "John Smith married Mary" in Theory A
- "John Smith married Sarah" in Theory B
- Both can coexist, scoped by theoretical context

### Certainty: Beyond Simple Scores

```rust
pub enum Certainty {
    Quantum(Vec<(String, f64)>),  // Multiple states with probabilities
    Fuzzy { membership: f64, confidence: f64 },
    Bayesian { prior: f64, likelihood: f64, posterior: f64 },
    Narrative(String),            // "Likely based on location proximity"
    Logical(LogicalExpression),   // Complex logic
    Unknown,
    Composite(Vec<Certainty>),
}
```

**Power**: Instead of meaningless 0-100 scores, express real uncertainty.

### PropertyGraph: Infinite Flexibility

```rust
pub struct PropertyGraph {
    properties: HashMap<String, Property>,
}

pub enum Property {
    Value(Value),                  // Simple values
    Entity(Box<Entity>),           // Nested entities!
    Computation(Computation),      // Derived values
    Theoretical(TheoreticalValue), // Hypothetical values
    Quantum(Vec<(Property, f64)>), // Superposition
    Reference(EntityId),           // Entity references
    Collection(Vec<Property>),     // Arrays
    Map(HashMap<String, Property>),// Nested maps
}
```

**Power**: Properties can be ANYTHING - even other entities, computed values, or quantum superpositions.

## Architectural Vision

### Three-Layer Architecture

```
┌─────────────────────────────────────────────────────────┐
│ LAYER 3: Domain Adapters & Applications                │
│ ┌─────────────┐  ┌──────────────┐  ┌────────────────┐ │
│ │  Genealogy  │  │ Archaeology  │  │  Custom User   │ │
│ │   Schema    │  │    Schema    │  │    Schemas     │ │
│ └─────────────┘  └──────────────┘  └────────────────┘ │
│         │                │                  │           │
│         └────────────────┴──────────────────┘           │
│                          │                              │
├─────────────────────────────────────────────────────────┤
│ LAYER 2: Schema Engine & Validation                    │
│ ┌───────────────────────────────────────────────────┐  │
│ │  - Schema Registry                                │  │
│ │  - Type Validation                                │  │
│ │  - Property Constraints                           │  │
│ │  - Relationship Rules                             │  │
│ │  - Context Enforcement                            │  │
│ │  - Query Translation                              │  │
│ └───────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────┤
│ LAYER 1: Universal Meta-Model Core                     │
│ ┌─────────────────────────────────────────────────┐    │
│ │  Entity   Relationship   Context   Certainty    │    │
│ │          PropertyGraph   MetaInfo               │    │
│ └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
            │
            ▼
┌─────────────────────────────────────────────────────────┐
│ EXISTING INFRASTRUCTURE (Keep as-is)                    │
│ ┌──────────┐  ┌──────────┐  ┌───────────┐             │
│ │ Storage  │  │   API    │  │  Modules  │             │
│ │  Layer   │  │  Layer   │  │  System   │             │
│ └──────────┘  └──────────┘  └───────────┘             │
│ ┌──────────┐  ┌──────────┐  ┌───────────┐             │
│ │  Events  │  │ Network  │  │   Sync    │             │
│ └──────────┘  └──────────┘  └───────────┘             │
└─────────────────────────────────────────────────────────┘
```

### New Crates to Create

```
crates/
├── rp-meta-core         # NEW: Universal meta-model primitives
├── rp-schema            # NEW: Schema system & validation
├── rp-schema-genealogy  # NEW: Genealogy domain adapter
├── rp-migration         # NEW: Old → New migration tools
└── [existing crates with minimal changes]
```

## Implementation Plan

### Phase 1: Meta-Core Foundation (Week 1)

**Create `rp-meta-core` crate**:

```rust
// Four universal primitives
pub mod entity;        // Entity with PropertyGraph
pub mod relationship;  // N-ary relationships
pub mod context;       // Universal qualifiers
pub mod certainty;     // Uncertainty expressions

// Supporting systems
pub mod properties;    // PropertyGraph implementation
pub mod temporal;      // Calendar-agnostic time
pub mod spatial;       // Coordinate-agnostic space
pub mod metadata;      // Provenance & versioning
```

**Key files**:
- `entity.rs`: Universal Entity struct
- `relationship.rs`: N-ary Relationship struct
- `context.rs`: Context with Scope enum
- `certainty.rs`: Multiple certainty models
- `properties.rs`: Flexible PropertyGraph
- `temporal.rs`: Multi-calendar temporal values
- `spatial.rs`: Multi-coordinate spatial values

**Deliverables**:
- ✅ All primitives implemented
- ✅ Full NO_FALLBACK_POLICY compliance
- ✅ Comprehensive documentation
- ✅ Unit tests for all types
- ✅ Property tests for invariants

### Phase 2: Schema System (Week 1-2)

**Create `rp-schema` crate**:

```rust
pub mod registry;      // Schema registration
pub mod definition;    // Schema definition format
pub mod validation;    // Runtime validation
pub mod query;         // Schema-aware queries
pub mod migration;     // Schema evolution
```

**Schema Format** (YAML/JSON):

```yaml
schema:
  name: "genealogy.bcg.v1"
  version: "1.0.0"

  entity_types:
    IdentityPersona:
      description: "Person entity in genealogical research"
      required_properties:
        - identity_type
      optional_properties:
        - given_names
        - surname
        - birth_date
      states:
        - Reference
        - Working
        - Hypothesis
        - Concluded

  relationship_types:
    Kinship.Parent:
      participants:
        - role: "parent"
          entity_types: ["IdentityPersona"]
        - role: "child"
          entity_types: ["IdentityPersona"]
      properties:
        biological: boolean
        adoptive: boolean
```

**Deliverables**:
- ✅ Schema definition format
- ✅ Schema registry with loading
- ✅ Runtime validation engine
- ✅ Schema-aware query builder
- ✅ Built-in schemas (genealogy, basic)

### Phase 3: Storage Layer Adaptation (Week 2)

**Update `rp-storage` for meta-model**:

```sql
-- Universal entity storage
CREATE TABLE entities (
    id UUID PRIMARY KEY,
    entity_type TEXT NOT NULL,
    state TEXT NOT NULL,
    properties JSONB NOT NULL,
    contexts JSONB NOT NULL,
    metadata JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    modified_at TIMESTAMPTZ NOT NULL
);

-- Universal relationship storage
CREATE TABLE relationships (
    id UUID PRIMARY KEY,
    relationship_type TEXT NOT NULL,
    participants JSONB NOT NULL,  -- Array of {entity_id, role, certainty}
    properties JSONB NOT NULL,
    contexts JSONB NOT NULL,
    metadata JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);

-- Indexes for performance
CREATE INDEX idx_entities_type ON entities(entity_type);
CREATE INDEX idx_entities_props ON entities USING GIN(properties);
CREATE INDEX idx_relationships_type ON relationships(relationship_type);
CREATE INDEX idx_relationships_participants ON relationships USING GIN(participants);
```

**Storage trait updates**:

```rust
#[async_trait]
pub trait MetaStorage {
    async fn create_entity(&self, entity: &Entity) -> Result<EntityId>;
    async fn get_entity(&self, id: EntityId) -> Result<Option<Entity>>;
    async fn update_entity(&self, entity: &Entity) -> Result<()>;
    async fn query_entities(&self, query: &EntityQuery) -> Result<Vec<Entity>>;

    async fn create_relationship(&self, rel: &Relationship) -> Result<RelationshipId>;
    async fn get_relationships(&self, entity_id: EntityId) -> Result<Vec<Relationship>>;
    async fn query_relationships(&self, query: &RelQuery) -> Result<Vec<Relationship>>;
}
```

**Deliverables**:
- ✅ PostgreSQL schema updated
- ✅ Storage trait adapted for meta-model
- ✅ Query builder for flexible queries
- ✅ Migration scripts from old schema
- ✅ Performance benchmarks

### Phase 4: Genealogy Domain Adapter (Week 2-3)

**Create `rp-schema-genealogy` crate**:

This provides the **same user experience** as the old system, but powered by the meta-model underneath.

```rust
// High-level API that looks like the old system
pub struct GenealogyAdapter {
    schema: Schema,
    storage: Arc<dyn MetaStorage>,
}

impl GenealogyAdapter {
    // User creates "Theory" just like before
    pub async fn create_theory(&self, question: String) -> Result<Theory> {
        // Internally creates Entity with entity_type="Theory"
        let entity = Entity {
            entity_type: "Theory".to_string(),
            properties: properties! {
                "question" => question,
                "state" => "Draft",
            },
            ...
        };
        self.storage.create_entity(&entity).await?;
        Ok(Theory::from_entity(entity))
    }

    // User works with "IdentityPersona" just like before
    pub async fn create_identity(&self, name: String) -> Result<IdentityPersona> {
        let entity = Entity {
            entity_type: "IdentityPersona".to_string(),
            properties: properties! {
                "given_names" => vec![name],
                "identity_type" => "Named",
            },
            ...
        };
        self.storage.create_entity(&entity).await?;
        Ok(IdentityPersona::from_entity(entity))
    }
}

// Theory wrapper (looks like old Theory but is just a view)
pub struct Theory {
    entity: Entity,
}

impl Theory {
    pub fn question(&self) -> &str {
        self.entity.properties.get_text("question")
            .expect("Theory must have question")
    }

    pub fn state(&self) -> TheoryState {
        self.entity.state.parse()
            .expect("Invalid theory state")
    }

    // All the same methods as before!
}
```

**Deliverables**:
- ✅ Genealogy schema definition
- ✅ High-level genealogy API
- ✅ Backward-compatible types (Theory, IdentityPersona, etc.)
- ✅ Comprehensive genealogy tests
- ✅ GPS compliance validation

### Phase 5: API Layer Update (Week 3)

**Update `rp-server` for meta-model**:

```rust
// Generic entity endpoints
POST   /entities
GET    /entities/:id
PUT    /entities/:id
DELETE /entities/:id
GET    /entities?type=Theory&state=Active  // Flexible queries

// Generic relationship endpoints
POST   /relationships
GET    /relationships/:id
GET    /entities/:id/relationships

// Schema-specific endpoints (optional, for convenience)
POST   /genealogy/theories      // Calls schema adapter
GET    /genealogy/identities
POST   /genealogy/relationships/kinship

// Schema management
GET    /schemas
POST   /schemas                 // Upload new schema
GET    /schemas/:name
```

**Deliverables**:
- ✅ Generic meta-model REST API
- ✅ Schema-specific convenience endpoints
- ✅ OpenAPI documentation updates
- ✅ WebSocket event updates
- ✅ API backward compatibility layer

### Phase 6: Module System Integration (Week 3-4)

**Update `rp-modules` for meta-model**:

Modules should work with meta-model entities instead of concrete types.

```rust
// Module interface
pub trait Module {
    fn process_entity(&self, entity: &Entity) -> Result<Vec<Entity>>;
    fn process_relationship(&self, rel: &Relationship) -> Result<Vec<Relationship>>;
    fn supported_types(&self) -> Vec<String>;
}

// Example: DNA analysis module
pub struct DNAModule;

impl Module for DNAModule {
    fn process_entity(&self, entity: &Entity) -> Result<Vec<Entity>> {
        if entity.entity_type == "DNASample" {
            // Process DNA sample, create match entities
            Ok(vec![/* match entities */])
        } else {
            Ok(vec![])
        }
    }

    fn supported_types(&self) -> Vec<String> {
        vec!["DNASample".to_string(), "DNAMatch".to_string()]
    }
}
```

**Deliverables**:
- ✅ Module SDK updated for meta-model
- ✅ Example modules (research-log, etc.) refactored
- ✅ Module discovery updated
- ✅ FFI bridge adapted
- ✅ WASM modules updated

### Phase 7: Migration Tools (Week 4)

**Create `rp-migration` crate**:

```rust
pub struct Migration {
    old_storage: Arc<dyn OldStorage>,
    new_storage: Arc<dyn MetaStorage>,
    schema: Schema,
}

impl Migration {
    pub async fn migrate_all(&self) -> Result<MigrationReport> {
        // Migrate all old entities to new meta-model
        self.migrate_theories().await?;
        self.migrate_identities().await?;
        self.migrate_evidence().await?;
        // ... all entity types

        Ok(report)
    }

    async fn migrate_theories(&self) -> Result<()> {
        let old_theories = self.old_storage.get_all_theories().await?;

        for old_theory in old_theories {
            let entity = Entity {
                id: old_theory.id,
                entity_type: "Theory".to_string(),
                state: old_theory.state.to_string(),
                properties: properties! {
                    "question" => old_theory.question,
                    "description" => old_theory.description,
                    "priority" => old_theory.priority,
                    "evidence" => old_theory.evidence,
                    // Map all fields to properties
                },
                contexts: vec![/* geographic, temporal contexts */],
                metadata: MetaInfo {
                    created: old_theory.created_at,
                    modified: old_theory.modified_at,
                    created_by: old_theory.created_by,
                    // ... map metadata
                },
            };

            self.new_storage.create_entity(&entity).await?;
        }

        Ok(())
    }
}
```

**Deliverables**:
- ✅ Migration tool for all entity types
- ✅ Data validation during migration
- ✅ Rollback capability
- ✅ Progress reporting
- ✅ Migration testing on real data

### Phase 8: Testing & Documentation (Week 4-5)

**Comprehensive Testing**:

```rust
// Unit tests
#[test]
fn test_entity_creation() { ... }

#[test]
fn test_property_graph() { ... }

// Integration tests
#[tokio::test]
async fn test_genealogy_workflow() {
    // Create theory
    // Create identities
    // Create relationships
    // Query data
    // Verify results
}

// Property-based tests
proptest! {
    #[test]
    fn test_entity_invariants(entity in arbitrary_entity()) {
        // Verify invariants hold
    }
}

// Performance tests
#[bench]
fn bench_entity_query(b: &mut Bencher) { ... }
```

**Documentation**:

```markdown
1. Architecture Guide
2. Meta-Model Concepts
3. Schema System Guide
4. Migration Guide
5. API Reference
6. Module Development Guide
7. Performance Tuning
8. Best Practices
```

**Deliverables**:
- ✅ 90%+ code coverage
- ✅ All property tests passing
- ✅ Performance benchmarks
- ✅ Complete documentation
- ✅ Example applications

## Benefits of the Refactoring

### 1. Universal Adaptability

**Before**: Only genealogy
**After**: ANY research domain

```rust
// Genealogy
Entity { type: "IdentityPersona", ... }

// Archaeology
Entity { type: "Artifact", properties: { "carbon_date": "1200 BCE", "site": ... }}

// Scientific Research
Entity { type: "Experiment", properties: { "hypothesis": ..., "results": ... }}

// Custom User Domain
Entity { type: "MyCustomType", properties: { ... }}
```

### 2. Flexible Data Models

**Before**: Fixed GEDCOM/GPS model
**After**: Any genealogical model

```rust
// Traditional genealogy
Entity { type: "Person", properties: { "birth": ..., "death": ... }}

// Y-DNA genealogy
Entity { type: "Haplogroup", properties: { "snp": "L-M20", "tmrca": ... }}

// Mitochondrial genealogy
Entity { type: "MtDNASequence", properties: { "haplotype": "H1a1", ... }}
```

### 3. Theory Versioning (The Killer Feature)

**Before**: Difficult to implement
**After**: Natural with contexts

```rust
// Theory A: John married Mary
let john_mary_rel = Relationship {
    type: "Kinship.Spouse",
    participants: [john, mary],
    contexts: [Context::Theoretical("Theory_A")],
};

// Theory B: John married Sarah
let john_sarah_rel = Relationship {
    type: "Kinship.Spouse",
    participants: [john, sarah],
    contexts: [Context::Theoretical("Theory_B")],
};

// Both coexist! Query by context to see different theories
```

### 4. Quantum Uncertainty

**Before**: Simple confidence scores
**After**: True quantum superposition

```rust
let identity_certainty = Certainty::Quantum(vec![
    ("same_person", 0.75),
    ("different_person", 0.20),
    ("insufficient_evidence", 0.05),
]);

let relationship = Relationship {
    participants: [
        Participant {
            entity: persona1,
            role: "subject",
            certainty: identity_certainty,
        },
        ...
    ],
    ...
};
```

### 5. No Semantic Loss

**Before**: Data transformations lose information
**After**: Everything preserved in PropertyGraph

```rust
// Import from ANY format, preserve ALL data
let entity = Entity {
    entity_type: "Person",
    properties: properties! {
        // Standard fields
        "name" => "John Smith",
        "birth" => "1850",

        // Preserve proprietary extensions
        "ancestry.com:confidence" => 95,
        "myheritage:tree_id" => "12345",

        // Preserve unknown fields
        "unknown_field_from_gedcom" => "preserve this",
    },
};
```

### 6. Extensibility Without Core Changes

**Before**: New features require core changes
**After**: Just add entity types and relationships

```rust
// Want DNA features? Just define new types
Entity { type: "DNASample", ... }
Entity { type: "DNAMatch", ... }
Relationship { type: "DNA.Matches", ... }

// No core code changes needed!
```

### 7. Cross-Model Queries

**Before**: Locked into one model
**After**: Query across models

```rust
// Find all entities related to a location, ANY type
query! {
    match: Entity,
    where: {
        relationships: {
            any: {
                type: "Located.At",
                participants: {
                    any: { entity_id: location_id }
                }
            }
        }
    }
}

// This works for Person, Event, Artifact, etc.!
```

## Risk Mitigation

### Risk 1: Performance Degradation

**Mitigation**:
- Comprehensive benchmarking
- PostgreSQL JSONB indexes
- Query optimization
- Caching layer
- Lazy loading of properties

### Risk 2: Complexity Increase

**Mitigation**:
- High-level domain adapters (genealogy, etc.)
- Comprehensive documentation
- Example applications
- Migration guides
- Gradual rollout

### Risk 3: Breaking Changes

**Mitigation**:
- Backward compatibility layer
- Parallel deployment option
- Migration tools with rollback
- Extensive testing
- Staged release

### Risk 4: Learning Curve

**Mitigation**:
- Domain adapters hide meta-model complexity
- Most users use genealogy adapter (looks like old system)
- Power users can use meta-model directly
- Comprehensive tutorials
- Example schemas

## Success Criteria

### Must Have

- ✅ All existing functionality preserved
- ✅ Backward compatible API
- ✅ NO_FALLBACK_POLICY compliance
- ✅ Migration from old to new works
- ✅ Performance within 20% of original
- ✅ All tests passing

### Should Have

- ✅ Genealogy schema matches old system exactly
- ✅ At least one alternative schema (e.g., archaeology)
- ✅ User-defined schema support
- ✅ Schema evolution/migration tools
- ✅ Comprehensive documentation

### Nice to Have

- ✅ Visual schema editor
- ✅ Schema marketplace
- ✅ Multiple genealogy schemas (BCG, Y-DNA, etc.)
- ✅ Cross-schema queries
- ✅ Schema validation tools

## Timeline

| Phase | Duration | Deliverables |
|-------|----------|-------------|
| 1. Meta-Core | 1 week | Universal primitives |
| 2. Schema System | 1 week | Schema engine |
| 3. Storage Layer | 1 week | Adapted storage |
| 4. Genealogy Adapter | 1-2 weeks | Domain adapter |
| 5. API Layer | 1 week | Updated API |
| 6. Module System | 1 week | Module integration |
| 7. Migration Tools | 1 week | Data migration |
| 8. Testing & Docs | 1-2 weeks | Complete system |
| **Total** | **8-10 weeks** | **Production ready** |

## Next Steps

1. ✅ Complete this analysis document
2. ✅ Get stakeholder approval
3. ✅ Create feature branch: `feature/universal-meta-model`
4. ✅ Begin Phase 1 implementation
5. ✅ Continuous integration and testing
6. ✅ Regular progress updates

## Conclusion

The refactoring from a hardcoded genealogical data model to a universal meta-model is **essential** for the long-term viability of ResearchProcess-GPS. The current system is robust and well-architected, but fundamentally limited by its inflexible core.

The meta-model approach:
- ✅ Preserves all existing functionality
- ✅ Enables unlimited extensibility
- ✅ Supports any research domain
- ✅ Allows multiple data models
- ✅ Eliminates semantic loss
- ✅ Enables true theory versioning

This is not just a refactoring—it's the **evolution from a genealogy platform to a universal research process protocol**.

---

*Analysis completed: 2025-11-07 23:49:00 UTC*
