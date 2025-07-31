# ResearchProcess-GPS: API Layer Implementation Session
## Phase 3 - Protocol & Networking

### 🚨 CRITICAL: Read These Documents First (IN THIS EXACT ORDER)

**DO NOT SKIP THIS STEP - Your success depends on understanding the full context**

#### 1. Comprehensive Handover Document
```bash
cd ~/ResearchProcess-GPS
bat COMPREHENSIVE_HANDOVER_EVENT_SOURCING_2025_07_31_1615_EEST.md
```
This contains:
- Current project state summary
- All reference document locations
- Technical decisions made
- Database connection details
- What was completed in Phase 2

#### 2. Master Architecture Plan
```bash
bat ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md
```
Focus on:
- Line 159-207: Protocol Definition section
- Line 293-344: Networking Layer section
- Line 420-436: Phase 3 requirements

#### 3. Entity Model
```bash
bat engine/UNIFIED_CONCEPTUAL_MODEL_2025_07_30_2000.md
```
Understand the 23 entities that need API endpoints.

#### 4. Event Sourcing Implementation
```bash
bat EVENT_SOURCING_IMPLEMENTATION_2025_07_31_1325_EEST.md
```
Shows how events are published - needed for real-time streaming.

---

## 🎯 Your Mission: Implement API Layer (Phase 3)

### Current Status
- ✅ Phase 1: Core entities + PostgreSQL CRUD - COMPLETE
- ✅ Phase 2: Event sourcing infrastructure - COMPLETE
- 🚧 Phase 3: Protocol & API implementation - YOUR TASK

### Phase 3 Deliverables (from ULTRATHINK plan)

1. **Protocol Specification Document**
   - Define request/response formats
   - WebSocket message protocol
   - Subscription patterns

2. **`rp-protocol` Crate**
   - Request/Response enums
   - Serialization formats
   - Protocol versioning

3. **`rp-server` Crate with Axum**
   - REST endpoints for all entities
   - GraphQL schema and resolvers
   - WebSocket for real-time updates
   - Authentication middleware

4. **Change Streaming**
   - PostgreSQL LISTEN/NOTIFY integration
   - Event delivery to WebSocket clients
   - Subscription management

5. **Basic Client Library**
   - `rp-client` crate
   - Type-safe API client
   - WebSocket client support

6. **Protocol Test Suite**
   - Integration tests
   - WebSocket tests
   - Performance benchmarks

---

## 💡 Implementation Strategy

### Step 1: Create Protocol Definitions
```rust
// In crates/rp-protocol/src/lib.rs
pub enum Request {
    // Entity operations
    CreateEntity(CreateEntityRequest),
    UpdateEntity(UpdateEntityRequest),
    QueryEntities(QueryRequest),
    
    // Workspace operations
    CreateWorkspace(WorkspaceConfig),
    JoinWorkspace { id: Uuid, credentials: Credentials },
    
    // Collaboration
    Subscribe(SubscriptionRequest),
    PublishChange(ChangeEvent),
    
    // Analysis
    StartAnalysis(AnalysisRequest),
    GetAnalysisStatus { id: Uuid },
}
```

### Step 2: Implement Axum Server
```rust
// In crates/rp-server/src/main.rs
let app = Router::new()
    .route("/api/v1/entities", post(create_entity))
    .route("/api/v1/entities/:id", get(get_entity))
    .route("/ws", get(websocket_handler))
    .route("/graphql", GraphQL::new(schema))
    .layer(auth_middleware)
    .layer(cors);
```

### Step 3: Real-time Event Streaming
- Use existing event store from Phase 2
- PostgreSQL NOTIFY on new events
- Deliver to WebSocket subscribers

---

## ⚠️ Critical Context

### Database Connection
```bash
# ALWAYS use this pattern to override system DB vars:
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run
```

### Existing Infrastructure
- Event store is fully operational
- Use `EventSourcedTransaction` for writes
- Projections can power read APIs
- All 23 entities have Rust types in `rp-core`

### Technical Constraints
- Use runtime SQL queries (not compile-time macros)
- Maintain backward compatibility
- Follow existing patterns from Phase 1 & 2

---

## 📋 Quick Reference

### Entity Types (need API endpoints)
**Layer 1**: Theory, IdentityPersona, Person, Source, Evidence, Citation, Confidence, Relationship, EvidenceAnalysis, ProofStatement, Fact

**Layer 2**: Researcher, ResearchLog, ResearchSession, ResearchActivity, WorkProduct, Analysis

**Layer 3**: Workspace, MethodologyConfig, ModuleConfig, StandardsRegistry, TemplateRegistry, ValidationRule

### Key Crates
- `rp-core` - Entity definitions
- `rp-storage` - Storage traits
- `rp-storage-postgres` - PostgreSQL implementation
- `rp-events` - Event sourcing
- `rp-protocol` - TO BE CREATED
- `rp-server` - TO BE CREATED

---

## 🚀 Start Commands

```bash
# 1. Switch to project
cd ~/ResearchProcess-GPS

# 2. Read all documentation (MANDATORY - 15-20 minutes)
bat COMPREHENSIVE_HANDOVER_EVENT_SOURCING_2025_07_31_1615_EEST.md
bat ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md
# ... read other docs listed above

# 3. Create protocol crate
cargo new --lib crates/rp-protocol

# 4. Create server crate
cargo new --bin crates/rp-server

# 5. Start implementing!
```

---

**Remember**: 
- The foundation is solid (Phases 1 & 2 complete)
- Event sourcing is working
- Focus on exposing existing functionality via APIs
- Read ALL documentation before starting

*Generated: 2025-07-31 16:18:00 EEST*
*Purpose: Enable seamless continuation of ResearchProcess-GPS development*
*Goal: Complete Phase 3 - API Layer Implementation*