# Next Session Prompt: PostgreSQL Schema Design & Event Sourcing
## Created: 2025-07-31 12:20:00 EEST
## Purpose: Begin Database Layer Implementation

---

## 🚀 PROMPT FOR NEXT SESSION

Continue development of ResearchProcess-GPS at `/home/greg/ResearchProcess-GPS/`. The core entity model (23 entities across 3 layers) is 100% complete with all 153 tests passing.

**YOUR MISSION**: Design and implement the PostgreSQL database schema with event sourcing support, then begin API development with Axum.

**CRITICAL READING ORDER** - These documents contain essential context:

1. **`COMPREHENSIVE_HANDOVER_2025_07_31_1217_EEST.md`**
   - Complete project context and current status
   - Technical decisions and patterns
   - Next phase preparation

2. **`ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md`**
   - Master technical architecture
   - Event sourcing strategy
   - PostgreSQL integration approach
   - API layer design (REST/GraphQL)

3. **`engine/UNIFIED_CONCEPTUAL_MODEL_2025_07_30_2000.md`**
   - Original entity specifications
   - Relationships and cardinalities
   - Foundation for database design

4. **`LAYER_3_COMPLETE_SUMMARY_2025_07_31_1216_EEST.md`**
   - Final implementation statistics
   - Architecture overview
   - All 23 entities listed

**CURRENT STATUS**:
- ✅ Core entity model: 100% complete (23/23 entities)
- ✅ All tests passing (153 tests)
- ✅ Layer 1: Core genealogical data (11 entities)
- ✅ Layer 2: Research process (6 entities)
- ✅ Layer 3: Workspace & metadata (6 entities)
- ⏳ PostgreSQL schema: Not started
- ⏳ Event sourcing: Not started
- ⏳ API layer: Not started

**NEXT PHASE OBJECTIVES**:

1. **PostgreSQL Schema Design**
   - Event store table structure
   - Read model projections for each entity
   - Indexes for performance
   - Migration scripts with sqlx

2. **Event Sourcing Implementation**
   - Define events for all 23 entities
   - Event store adapter
   - Projection handlers
   - Snapshot strategy

3. **API Development with Axum**
   - REST endpoints for CRUD operations
   - GraphQL schema with async-graphql
   - WebSocket support for real-time updates
   - Authentication/authorization

4. **Integration Testing**
   - Database integration tests
   - API endpoint tests
   - Event sourcing tests

**KEY ARCHITECTURAL PRINCIPLES TO MAINTAIN**:

1. **Event Sourcing First**
   - All state changes as events
   - Immutable event log
   - Read models as projections

2. **CQRS Pattern**
   - Commands modify state
   - Queries read projections
   - Clear separation

3. **Layer 3's "Standards as Data"**
   - Methodologies stored as JSON/YAML
   - Validation rules as data
   - Templates as files

**EXAMPLE STARTING POINTS**:

```sql
-- Event store schema
CREATE TABLE events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_id UUID NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    event_version INT NOT NULL,
    event_data JSONB NOT NULL,
    metadata JSONB NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Theory projection
CREATE TABLE theories (
    id UUID PRIMARY KEY,
    question TEXT NOT NULL,
    description TEXT,
    state VARCHAR(20) NOT NULL,
    evidence_ids UUID[] NOT NULL DEFAULT '{}',
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    modified_at TIMESTAMPTZ NOT NULL,
    version INT NOT NULL
);
```

```rust
// Event definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    Theory(TheoryEvent),
    Evidence(EvidenceEvent),
    // ... other entities
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TheoryEvent {
    Created {
        id: EntityId,
        question: String,
        created_by: EntityId,
    },
    EvidenceAdded {
        theory_id: EntityId,
        evidence_id: EntityId,
    },
    StateChanged {
        theory_id: EntityId,
        from_state: TheoryState,
        to_state: TheoryState,
    },
}
```

**REMEMBER**:
- Read all handover documents first
- The core model is complete - don't modify entities
- Focus on persistence and API layers
- Maintain event sourcing principles
- Test everything

Good luck! The foundation is solid and ready for the next phase. 🚀

---

## 📋 Quick Command Reference

```bash
# Navigate to project
cd /home/greg/ResearchProcess-GPS

# Run tests
cargo test --package rp-core --lib

# Check compilation
cargo check

# View project structure
eza -la --tree

# Review previous work
bat COMPREHENSIVE_HANDOVER_2025_07_31_1217_EEST.md
```