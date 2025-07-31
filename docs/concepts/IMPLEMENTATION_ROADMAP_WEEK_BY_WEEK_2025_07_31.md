# ResearchProcess-GPS Implementation Roadmap
## Week-by-Week Development Plan
### Starting August 1, 2025

---

## Month 1: Foundation

### Week 1: Project Setup & Core Traits
**Goal**: Establish project structure and define core abstractions

**Tasks**:
```bash
# Day 1-2: Project setup
cargo new --name researchprocess-gps .
cargo new --lib crates/rp-core
cargo new --lib crates/rp-macros

# Structure
├── Cargo.toml
├── crates/
│   ├── rp-core/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── entity.rs      # Core Entity trait
│   │   │   ├── state.rs       # State machine traits
│   │   │   ├── identity.rs    # UUID and versioning
│   │   │   └── errors.rs      # Error types
│   │   └── tests/
│   └── rp-macros/
│       └── src/
│           └── state_machine.rs  # Procedural macros
```

**Day 3-5: Core trait definitions**
```rust
// crates/rp-core/src/entity.rs
pub trait Entity: Send + Sync + 'static {
    fn id(&self) -> Uuid;
    fn entity_type(&self) -> EntityType;
    fn version(&self) -> Version;
    fn validate(&self) -> Result<(), ValidationError>;
}

// crates/rp-core/src/state.rs
pub trait StateMachine {
    type State: State;
    fn current_state(&self) -> &Self::State;
    fn transition(&mut self, new_state: Self::State) -> Result<()>;
}
```

**Deliverables**:
- [x] Workspace structure created
- [x] Core traits defined
- [x] Basic error types
- [x] Initial documentation

### Week 2: First Entity Implementation (Theory)
**Goal**: Implement Theory entity with state machine

**Day 1-2: Theory entity structure**
```rust
// crates/rp-core/src/entities/theory.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theory {
    id: Uuid,
    version: Version,
    state: TheoryState,
    
    // Core fields
    question: String,
    hypothesis: Option<String>,
    
    // Attribution
    created_by: ResearcherId,
    created_at: DateTime<Utc>,
    
    // Relationships
    evidence_ids: Vec<Uuid>,
    analysis_ids: Vec<Uuid>,
}
```

**Day 3-4: State machine implementation**
```rust
#[derive(StateMachine)]
pub enum TheoryState {
    Created,
    Exploring,
    Hypothesized,
    Testing,
    Concluded,
}
```

**Day 5: Tests and validation**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn theory_state_transitions() {
        let mut theory = Theory::new("What happened to John?");
        assert!(theory.transition_to_exploring().is_ok());
        assert!(theory.transition_to_concluded().is_err());
    }
}
```

### Week 3: Storage Layer Foundation
**Goal**: PostgreSQL adapter with basic CRUD

**Day 1-2: Schema design**
```sql
-- migrations/001_initial_schema.sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE entities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type TEXT NOT NULL,
    version BIGINT NOT NULL DEFAULT 1,
    state TEXT NOT NULL,
    data JSONB NOT NULL,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_entity_type ON entities(entity_type);
CREATE INDEX idx_created_by ON entities(created_by);
```

**Day 3-5: Storage adapter**
```rust
// crates/rp-storage-postgres/src/lib.rs
pub struct PostgresAdapter {
    pool: PgPool,
}

#[async_trait]
impl StorageAdapter for PostgresAdapter {
    async fn create(&self, entity: &dyn Entity) -> Result<()> {
        // Implementation
    }
}
```

### Week 4: Serialization & Testing
**Goal**: Complete serialization and comprehensive tests

**Day 1-3: Serialization implementation**
```rust
// Binary format for performance
impl Theory {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes)
    }
}
```

**Day 4-5: Integration tests**
```rust
#[tokio::test]
async fn test_theory_roundtrip() {
    let storage = PostgresAdapter::new("postgres://...").await?;
    let theory = Theory::new("Test question");
    
    storage.create(&theory).await?;
    let loaded = storage.get::<Theory>(theory.id()).await?;
    
    assert_eq!(theory, loaded);
}
```

---

## Month 2: Protocol & Analysis

### Week 5: Protocol Definition
**Goal**: Define core protocol messages

**Day 1-3: Protocol types**
```rust
// crates/rp-protocol/src/messages.rs
#[derive(Serialize, Deserialize)]
pub enum Request {
    // Entity operations
    Create { entity: EntityData },
    Read { id: Uuid },
    Update { id: Uuid, changes: Vec<Change> },
    Delete { id: Uuid },
    
    // Queries
    Query { filter: Filter },
    
    // Subscriptions
    Subscribe { filter: Filter },
    Unsubscribe { subscription_id: Uuid },
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Success { data: Value },
    Error { code: ErrorCode, message: String },
    Event { event: Event },
}
```

**Day 4-5: WebSocket handler**
```rust
// crates/rp-server/src/websocket.rs
pub async fn handle_connection(ws: WebSocket) {
    let (tx, rx) = ws.split();
    
    tokio::select! {
        _ = handle_incoming(rx) => {},
        _ = handle_outgoing(tx) => {},
    }
}
```

### Week 6: Analysis Entity
**Goal**: Implement Analysis entity with reasoning chains

**Implementation following the same pattern as Theory**

### Week 7: Basic Server
**Goal**: Working WebSocket server

```rust
// crates/rp-server/src/main.rs
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .route("/health", get(health_check));
        
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Week 8: Client Library
**Goal**: Rust client for testing

```rust
// crates/rp-client/src/lib.rs
pub struct RpClient {
    ws: WebSocket,
}

impl RpClient {
    pub async fn connect(url: &str) -> Result<Self> {
        // Implementation
    }
    
    pub async fn create_theory(&self, question: &str) -> Result<Theory> {
        // Implementation
    }
}
```

---

## Month 3: Module System & UI

### Week 9-10: Module Framework
**Goal**: Basic module loading system

```rust
// crates/rp-modules/src/lib.rs
pub trait Module {
    fn metadata(&self) -> ModuleMetadata;
    fn initialize(&mut self, ctx: ModuleContext) -> Result<()>;
}

pub struct ModuleLoader {
    modules: HashMap<String, Box<dyn Module>>,
}
```

### Week 11-12: First Module (Research Log)
**Goal**: Implement Research Log module

```rust
pub struct ResearchLogModule {
    storage: Arc<dyn StorageAdapter>,
}

impl Module for ResearchLogModule {
    // Auto-capture research activities
}
```

---

## Critical Path Items

### Immediate Decisions Needed (Week 1)
1. **UUID Strategy**: UUID v7 for time-ordering
2. **Async Runtime**: Tokio (most mature)
3. **Serialization**: Bincode + JSON fallback
4. **Database**: PostgreSQL 15+ (for JSONB features)

### Early Prototypes (Week 2-4)
1. **State Machine Macro**: Prove the concept works
2. **Storage Performance**: Benchmark JSONB vs binary
3. **WebSocket Protocol**: Test real-time capabilities

### Risk Mitigation (Ongoing)
1. **Performance Testing**: Every week
2. **API Stability**: Freeze core traits by Week 4
3. **Documentation**: Write as we go

---

## Success Criteria

### Month 1 Milestones
- [ ] Theory entity fully implemented
- [ ] PostgreSQL storage working
- [ ] State machines proven
- [ ] 90% test coverage

### Month 2 Milestones
- [ ] Protocol defined and documented
- [ ] Basic server running
- [ ] Client can create/query entities
- [ ] Analysis entity implemented

### Month 3 Milestones
- [ ] Module system functional
- [ ] Research Log module working
- [ ] Basic UI prototype
- [ ] Performance benchmarks passing

---

## Development Environment Setup

### Prerequisites
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup component add rustfmt clippy

# Database
docker run -d \
  --name rp-postgres \
  -e POSTGRES_PASSWORD=devpassword \
  -e POSTGRES_DB=researchprocess \
  -p 5432:5432 \
  postgres:15

# Development tools
cargo install cargo-watch cargo-nextest cargo-tarpaulin sqlx-cli
```

### Initial Commands
```bash
# Create workspace
mkdir researchprocess-gps && cd researchprocess-gps
cargo init --name rp-workspace

# Add workspace members
cat > Cargo.toml << EOF
[workspace]
members = [
    "crates/rp-core",
    "crates/rp-protocol",
    "crates/rp-storage-postgres",
    "crates/rp-server",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Greg Lamberson <greg@example.com>"]
license = "MIT OR Apache-2.0"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
uuid = { version = "1", features = ["v7", "serde"] }
EOF

# Create first crate
cargo new --lib crates/rp-core
```

---

## Next Steps

1. **Today**: Set up development environment
2. **Tomorrow**: Create workspace and core traits
3. **This Week**: Implement Theory entity
4. **Next Week**: PostgreSQL storage
5. **End of Month**: Working prototype

This roadmap provides concrete, achievable goals with clear weekly deliverables. The focus is on proving core concepts early while maintaining flexibility for future expansion.