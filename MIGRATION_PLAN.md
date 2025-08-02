# Migration Plan: ResearchProcess-GPS to Universal Meta-Model

### Timestamp: 2025-08-01 23:44:00 EEST

## Executive Summary

This document outlines the exact work needed to directly replace the existing ResearchProcess-GPS data model with the new universal meta-model. This is a complete migration without compatibility layers.

## Current State Analysis

### Existing ResearchProcess-GPS
- **18 concrete entities** across 3 layers
- **Robust infrastructure**: State machines, validation, transactions, events
- **Advanced features**: Collaboration, module system, multi-backend storage
- **Production readiness**: Error handling, testing, deployment architecture

### New Meta-Model
- **4 universal primitives**: Entity, Relationship, Context, Certainty
- **Infinite extensibility**: Property graphs, open-ended types
- **Abstraction layers**: Transform between formats without semantic loss
- **Clean implementation**: But lacks robustness features

## Migration Strategy: Direct Replacement

### Phase 1: Infrastructure Enhancement (Weeks 1-3)

**Goal**: Add all robustness features to meta-model before migration

#### 1.1 Port Core Infrastructure
```rust
// Add to meta-model-core
pub mod state;      // State machines from rp-core
pub mod validation; // Validation framework from rp-core
pub mod event;      // Event system from rp-core
```

#### 1.2 Enhance Storage Layer
```rust
// Update meta-model-storage to include:
- Transaction support from rp-storage
- Event sourcing capabilities
- Versioning and history
```

#### 1.3 Update Meta-Model Entities
```rust
// Enhance Entity to include state machines
pub struct Entity {
    // ... existing fields ...
    pub state: EntityState,
    pub state_history: Vec<StateTransition<EntityState>>,
    pub version: u64,
    pub parent_version: Option<u64>,
}
```

### Phase 2: Replace Core Entities (Weeks 4-7)

**Goal**: Replace existing entity structs with meta-model

#### 2.1 Delete Old Entity Definitions
```rust
// Remove from rp-core:
- src/source.rs
- src/citation.rs
- src/identity_persona.rs
// ... all 18 entity files
```

#### 2.2 Replace with Meta-Model Usage
```rust
// Instead of:
let source = Source::new("Title", author);

// Use:
let mut source = Entity::new("Source");
source.properties.set("title", "Title");
source.properties.set("author", author);
source.state = EntityState::Active;
```

#### 2.3 Update All Entity References
```rust
// Update function signatures
fn process_source(source: &Entity) -> Result<()> {
    assert_eq!(source.entity_type, "Source");
    let title = source.properties.get_string("title")?;
    // ...
}
```

### Phase 3: Migrate Storage Layer (Weeks 8-10)

**Goal**: Replace entity-specific storage with universal storage

#### 3.1 Drop Old Tables
```sql
-- Remove entity-specific tables
DROP TABLE sources CASCADE;
DROP TABLE citations CASCADE;
DROP TABLE identity_personas CASCADE;
-- ... all entity tables
```

#### 3.2 Create Universal Schema
```sql
-- Single set of tables for all data
CREATE TABLE entities (
    id UUID PRIMARY KEY,
    entity_type TEXT NOT NULL,
    state TEXT NOT NULL,
    properties JSONB NOT NULL,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE relationships (
    id UUID PRIMARY KEY,
    relationship_type TEXT NOT NULL,
    from_entity UUID REFERENCES entities(id),
    to_entity UUID REFERENCES entities(id),
    properties JSONB NOT NULL
);
```

#### 3.3 Migrate Existing Data
```rust
// One-time data migration script
pub async fn migrate_all_data(old_db: &PgPool, new_db: &PgPool) -> Result<()> {
    // Transform each old entity to meta-model
    let sources = fetch_all_sources(old_db).await?;
    for source in sources {
        let entity = source_to_entity(source);
        store_entity(new_db, &entity).await?;
    }
    // Repeat for all entity types
}
```

### Phase 4: Update All Features (Weeks 11-14)

**Goal**: Update all features to work with meta-model

#### 4.1 Update Module System
```rust
// Modules now only work with meta-model
impl Module for ResearchModule {
    fn handle_message(&mut self, msg: Message) -> Result<Response> {
        match msg.entity_type.as_str() {
            "Source" => self.handle_source(&msg.entity),
            "IdentityPersona" => self.handle_persona(&msg.entity),
            _ => Err("Unknown entity type"),
        }
    }
}
```

#### 4.2 Update Collaboration Engine
```rust
// CRDT operations on property graphs
impl CollaborativeEdit for Entity {
    fn apply_operation(&mut self, op: Operation) {
        match op {
            Operation::SetProperty { key, value } => {
                self.properties.set(&key, value);
            }
            Operation::AddRelationship { rel } => {
                self.relationships.push(rel.id);
            }
        }
    }
}
```

#### 4.3 Update Query System
```rust
// Queries now work on universal schema
impl QueryEngine {
    pub async fn query(&self, q: Query) -> Result<Vec<Entity>> {
        let sql = "SELECT * FROM entities WHERE entity_type = $1
                   AND properties @> $2";
        // Universal query execution
    }
}
```

### Phase 5: Clean Up and Optimize (Weeks 15-16)

**Goal**: Remove all old code and optimize for meta-model

#### 5.1 Delete Old Code
```bash
# Remove old entity definitions
rm -rf crates/rp-core/src/{source,citation,identity_persona,...}.rs

# Remove old storage implementations
rm -rf crates/rp-storage-postgres/src/entities/

# Update Cargo.toml to remove old dependencies
```

#### 5.2 Optimize Meta-Model Performance
```rust
// Add specialized indexes
CREATE INDEX idx_entity_type_properties ON entities(entity_type, properties);
CREATE INDEX idx_properties_gin ON entities USING GIN(properties);

// Add materialized views for common queries
CREATE MATERIALIZED VIEW person_relationships AS ...
```

#### 5.3 Update Documentation
- Remove references to old entities
- Document meta-model usage patterns
- Update API documentation

## Specific Work Items

### 1. Infrastructure Enhancement (3 weeks)
- [ ] Port state machines to meta-model-core
- [ ] Port validation framework to meta-model-core
- [ ] Port event system to meta-model-core
- [ ] Add transaction support to meta-model-storage
- [ ] Add versioning to Entity struct

### 2. Entity Replacement (4 weeks)
- [ ] Create migration script for each entity type
- [ ] Delete old entity definitions
- [ ] Update all code references to use Entity
- [ ] Update tests to use meta-model

### 3. Storage Migration (3 weeks)
- [ ] Create new universal PostgreSQL schema
- [ ] Write data migration scripts
- [ ] Test data integrity after migration
- [ ] Drop old tables after verification

### 4. Feature Updates (4 weeks)
- [ ] Update module system for meta-model
- [ ] Update collaboration engine for property graphs
- [ ] Update query system for universal schema
- [ ] Update caching for new structure

### 5. Clean Up & Optimization (2 weeks)
- [ ] Remove all old entity code
- [ ] Create performance indexes
- [ ] Build materialized views
- [ ] Update all documentation

## Risk Mitigation

### Performance Risks
- **Risk**: Property graphs slower than structured entities
- **Mitigation**: Implement specialized indexes, caching strategies

### Data Loss Risks
- **Risk**: Migration could lose data
- **Mitigation**: Comprehensive backup, rollback capability, parallel running

### Feature Parity Risks
- **Risk**: Some features might not translate well
- **Mitigation**: Compatibility layer maintains 100% backwards compatibility

## Benefits After Migration

1. **Infinite Extensibility**: Add new entity types without code changes
2. **Format Agnostic**: Import/export any genealogy format
3. **Future Proof**: Adapt to new standards without restructuring
4. **Simplified Core**: Complex logic moves to abstraction layers
5. **Research Freedom**: Support any research methodology

## Implementation Order

1. **Complete infrastructure first** before touching any entities
2. **Migrate all entities in one go** to avoid mixed states
3. **Full data migration** with downtime window
4. **Cut over completely** - no gradual rollout

## Success Criteria

- [ ] All existing tests pass with meta-model
- [ ] No performance degradation (< 5% impact)
- [ ] 100% feature parity maintained
- [ ] Successful migration of production data
- [ ] Module system fully compatible
- [ ] Collaboration features working

## Next Steps

1. Review and approve this plan
2. Create detailed technical specifications
3. Set up migration branch
4. Begin Phase 1 implementation

---

This migration preserves everything that makes ResearchProcess-GPS robust while gaining the flexibility of the universal meta-model. The key is the hybrid approach that allows gradual migration without disrupting existing functionality.