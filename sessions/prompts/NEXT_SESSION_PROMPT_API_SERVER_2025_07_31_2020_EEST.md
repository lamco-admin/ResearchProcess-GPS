# ResearchProcess-GPS: API Server Implementation Session
## Phase 3 - Continue Protocol & Networking Implementation

### 🚨 CRITICAL: Read These Documents First (IN THIS EXACT ORDER)

**DO NOT SKIP THIS STEP - Your success depends on understanding the full context**

#### 1. Comprehensive Handover Document
```bash
cd ~/ResearchProcess-GPS
bat COMPREHENSIVE_HANDOVER_API_PHASE_2025_07_31_2015_EEST.md
```
This contains:
- Current project state with Phase 2 COMPLETE
- NO_FALLBACK_POLICY requirements
- All reference document locations
- Technical decisions made
- Database connection details
- What was completed today (protocol layer + event sourcing fix)

#### 2. Master Architecture Plan
```bash
bat ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md
```
Focus on:
- Line 293-344: Networking Layer implementation
- Line 159-207: Protocol definitions (now implemented)
- Line 420-436: Phase 3 requirements

#### 3. Protocol Specification
```bash
bat docs/PROTOCOL_SPECIFICATION_v1.md
```
Understand the REST, WebSocket, and error handling protocols you'll be implementing.

#### 4. API Design Decision
```bash
bat docs/API_DESIGN_OPTIONS_ANALYSIS.md
```
Review the hybrid approach (generic + specific endpoints) that was chosen.

#### 5. Progress Summary
```bash
bat PROGRESS_SUMMARY_2025_07_31_2012_EEST.md
```
See current state and what's been accomplished.

---

## 🎯 Your Mission: Implement API Server (Phase 3 Continuation)

### Current Status
- ✅ Phase 1: Core entities + PostgreSQL CRUD - COMPLETE
- ✅ Phase 2: Event sourcing infrastructure - COMPLETE (fixed today!)
- 🚧 Phase 3: Protocol & API implementation - 20% COMPLETE
  - ✅ Protocol specification document
  - ✅ `rp-protocol` crate with all types
  - 🚧 `rp-server` crate - YOUR TASK

### Today's Achievements (Previous Session)
- Fixed critical EventBuilder API issue
- Implemented factory methods for domain events
- Restored full event sourcing functionality
- Created complete protocol definitions
- Zero warnings across entire workspace

### Your Primary Tasks

1. **Create `rp-server` with Axum**
   ```rust
   // In crates/rp-server/src/main.rs
   use axum::{Router, routing::{get, post}};
   use rp_protocol::*;
   use rp_storage_postgres::PostgresBackend;
   
   #[tokio::main]
   async fn main() {
       let app = Router::new()
           .route("/api/v1/entities", post(create_entity).get(list_entities))
           .route("/api/v1/entities/:id", get(get_entity).put(update_entity))
           .route("/ws/v1", get(websocket_handler))
           .layer(cors)
           .layer(auth_middleware);
           
       axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
           .serve(app.into_make_service())
           .await
           .unwrap();
   }
   ```

2. **Implement REST Endpoints**
   - Start with generic entity endpoints
   - Use the protocol types from `rp-protocol`
   - Connect to PostgreSQL backend
   - Ensure events are published on mutations

3. **WebSocket Implementation**
   - Real-time event streaming
   - Subscription management
   - Use WebSocket protocol from `rp-protocol`

4. **PostgreSQL LISTEN/NOTIFY**
   - Set up database notifications
   - Connect to event store
   - Stream events to WebSocket clients

---

## ⚠️ Critical Requirements

### NO_FALLBACK_POLICY (MANDATORY)
- **ZERO** warnings allowed
- **NO** placeholder implementations without clear TODOs
- **ALL** errors must be handled explicitly
- **NO** silent failures or degraded operation

### Database Connection Pattern
```bash
# ALWAYS use this pattern:
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run
```

### Event Publishing Pattern
Events are now fully functional! When implementing endpoints:
```rust
// After successful CRUD operation
let (event, metadata) = DomainEvent::entity_created(
    entity.id,
    &entity.entity_type,
    entity.data.clone(),
    actor_id,
);
// Events are published automatically by EventSourcedTransaction
```

---

## 💡 Implementation Strategy

### Step 1: Set up Axum Server
1. Update `crates/rp-server/Cargo.toml` with dependencies
2. Create basic server with health endpoint
3. Add CORS and basic middleware
4. Test with simple ping endpoint

### Step 2: Generic Entity Endpoints
1. Implement `/api/v1/entities` POST (create)
2. Implement `/api/v1/entities/{id}` GET (read)
3. Implement `/api/v1/entities/{id}` PUT (update)
4. Implement `/api/v1/entities/{id}` DELETE (delete)
5. Implement `/api/v1/entities` GET (list with filters)

### Step 3: WebSocket Handler
1. Set up WebSocket upgrade
2. Implement subscription protocol
3. Connect to event store
4. Test real-time updates

### Step 4: LISTEN/NOTIFY Integration
1. Set up PostgreSQL notification trigger
2. Create listener in server
3. Forward events to WebSocket clients

---

## 📋 Quick Reference

### Protocol Types Available
- `CreateEntityRequest` / `EntityResponse`
- `UpdateEntityRequest` / `ListResponse`
- `ClientMessage` / `ServerMessage` (WebSocket)
- `ApiError` / `ProtocolError`
- All defined in `rp-protocol` crate

### Storage Interface
- Use `PostgresBackend` from `rp-storage-postgres`
- Transactions via `EventSourcedTransaction`
- Events automatically published on mutations

### Entity Types
All 23 entities defined in `rp-core`:
- Layer 1: Theory, Person, Evidence, etc.
- Layer 2: Researcher, ResearchLog, etc.
- Layer 3: Workspace, MethodologyConfig, etc.

---

## 🚀 Start Commands

```bash
# 1. Switch to project
cd ~/ResearchProcess-GPS

# 2. Read ALL documentation (MANDATORY - 20-30 minutes)
bat COMPREHENSIVE_HANDOVER_API_PHASE_2025_07_31_2015_EEST.md
# ... read other docs listed above

# 3. Check server crate
cd crates/rp-server
bat Cargo.toml
bat src/main.rs

# 4. Start implementing!
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run --bin rp-server
```

---

**Remember**: 
- Event sourcing is NOW WORKING - events flow automatically
- Protocol types are fully defined in `rp-protocol`
- NO_FALLBACK_POLICY - fix ALL warnings and issues
- The foundation is solid - focus on the API layer

*Generated: 2025-07-31 20:20:00 EEST*
*Purpose: Enable seamless continuation of ResearchProcess-GPS API server implementation*
*Goal: Complete Phase 3 with working REST and WebSocket endpoints*