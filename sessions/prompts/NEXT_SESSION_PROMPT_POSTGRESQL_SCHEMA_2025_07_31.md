# NEXT SESSION PROMPT - PostgreSQL Schema Design

I'm continuing development of ResearchProcess-GPS, a Rust-based protocol, engine, and platform for professional genealogical research. In the previous session, I made a major architectural pivot from Python/Django to Rust and established the foundational implementation.

## Current Status

**Project Location**: `/home/greg/ResearchProcess-GPS/` (standalone repository)

**Architecture**: ResearchProcess-GPS is designed as:
1. **A Protocol** - standardized format for genealogical research data
2. **An Engine** - high-performance analysis and reasoning system  
3. **A Platform** - extensible framework for building genealogy tools

**Key Design Decisions**:
- Storage-agnostic with pluggable backends (PostgreSQL reference implementation)
- UUID v7 for all entity identifiers
- Standalone-first, collaboration-ready architecture
- Rust workspace with 12 planned crates

**Completed**:
- Core trait system (Entity, StateMachine, Validation)
- Researcher entity with full attribution
- Theory entity with state machine (Draft→Active→Terminal)
- Confidence framework (narrative-based, not just scores)
- Storage abstraction architecture documented

**Next Priorities** (Week 1 continuation):
1. Design PostgreSQL schema for hybrid JSONB + binary storage
2. Implement storage traits and PostgreSQL backend
3. Add remaining core entities (Evidence, Analysis, Person, IdentityPersona)
4. Design binary protocol format

**Critical Documents**:
- `ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md` - Full project plan
- `STORAGE_ABSTRACTION_ARCHITECTURE_2025_07_31.md` - Storage design
- `engine/UNIFIED_CONCEPTUAL_MODEL_WITH_ANALYSIS_2025_07_31.md` - Complete data model
- `SESSION_HANDOVER_RUST_PIVOT_COMPLETE_2025_07_31.md` - Detailed handover

## PostgreSQL Schema Requirements

Please help me continue with the PostgreSQL schema design and storage implementation. The schema needs to support:

### Core Requirements
1. **Hybrid Storage Model**
   - JSONB for flexible entity data and schema evolution
   - Binary (BYTEA) for efficient protocol messages
   - Relational tables for indexes and common queries

2. **Entity Storage Pattern**
   - Common entity fields (id, type, created_by, etc.) in base table
   - Entity-specific data in JSONB column
   - State tracking for entities with state machines
   - Version history tracking

3. **Performance Features**
   - Efficient querying of entities and relationships
   - PostgreSQL indexes on JSONB paths
   - Materialized views for complex queries
   - Partitioning strategy for large datasets

4. **Advanced Capabilities**
   - Change data capture for sync/collaboration
   - Extension points for pgvector (semantic search)
   - Extension points for Apache AGE (graph queries)
   - LISTEN/NOTIFY for real-time updates

### Specific Tables Needed
1. `entities` - Base table for all entities
2. `entity_versions` - Version history
3. `entity_states` - State machine tracking
4. `relationships` - Entity relationships
5. `workspaces` - Multi-tenancy support
6. `changes` - Change data capture
7. `modules` - Installed modules/extensions

### Development Path
1. Create SQL migration files
2. Implement `rp-storage` traits
3. Create `rp-storage-postgres` crate
4. Add connection pooling with sqlx
5. Write integration tests

The schema should balance flexibility (JSONB) with performance (indexes, binary storage) while maintaining forward compatibility for future features.