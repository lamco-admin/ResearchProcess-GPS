# Critical Architectural Decisions for ResearchProcess-GPS
## Rust Implementation Deep Dive
### July 31, 2025

---

## 1. Entity Storage Architecture

### Decision: Hybrid Storage Model

```rust
// Entities stored in PostgreSQL with specialized columns
CREATE TABLE entities (
    id UUID PRIMARY KEY,
    entity_type TEXT NOT NULL,
    state TEXT NOT NULL,
    version BIGINT NOT NULL,
    
    -- JSONB for flexible properties
    data JSONB NOT NULL,
    
    -- Extracted fields for indexing
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    
    -- Full-text search
    search_vector tsvector GENERATED ALWAYS AS (
        to_tsvector('english', data)
    ) STORED,
    
    -- Relationships as array for graph queries
    relationships UUID[] DEFAULT '{}',
    
    -- Binary representation for efficiency
    binary_data BYTEA,
    
    -- Partitioning key
    workspace_id UUID NOT NULL
) PARTITION BY LIST (workspace_id);

-- Specialized tables for hot paths
CREATE TABLE analysis_reasoning_steps (
    id UUID PRIMARY KEY,
    analysis_id UUID NOT NULL,
    step_number INT NOT NULL,
    observation TEXT NOT NULL,
    reasoning TEXT NOT NULL,
    conclusion TEXT NOT NULL,
    evidence_refs UUID[],
    confidence REAL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- For efficient queries
    INDEX idx_analysis_steps ON analysis_reasoning_steps(analysis_id, step_number)
);
```

**Rationale**:
- JSONB gives flexibility while maintaining queryability
- Binary storage for performance-critical paths
- Specialized tables prevent JSONB overhead for hot data
- Partitioning by workspace enables scaling

### Alternative Considered: Pure Document Store
- Pros: Maximum flexibility
- Cons: Lost relational integrity, complex queries
- Decision: Hybrid gives best of both worlds

---

## 2. State Machine Implementation

### Decision: Code Generation with Compile-Time Validation

```rust
// State definition using procedural macros
#[derive(StateMachine)]
#[state_machine(entity = "Theory")]
pub enum TheoryState {
    #[initial]
    #[transitions(Exploring, Abandoned)]
    Created,
    
    #[transitions(Hypothesized, Abandoned)]
    #[validation(has_research_question)]
    Exploring,
    
    #[transitions(Testing, Abandoned)]
    #[validation(has_hypothesis)]
    Hypothesized,
    
    #[transitions(Concluded, Revised, Challenged)]
    #[validation(has_evidence)]
    Testing,
    
    #[final]
    #[validation(has_proof_statement)]
    Concluded,
    
    #[transitions(Testing)]
    Revised,
    
    #[transitions(Testing, Abandoned)]
    Challenged,
    
    #[final]
    Abandoned,
}

// Generated code provides type-safe transitions
impl Theory {
    pub fn transition_to_testing(&mut self) -> Result<()> {
        match self.state {
            TheoryState::Hypothesized => {
                self.validate_has_evidence()?;
                self.state = TheoryState::Testing;
                self.emit_event(StateChanged::new(
                    TheoryState::Hypothesized,
                    TheoryState::Testing
                ));
                Ok(())
            }
            _ => Err(InvalidTransition)
        }
    }
}
```

**Rationale**:
- Compile-time validation prevents invalid states
- Zero runtime overhead
- Self-documenting code
- Easy to visualize and reason about

---

## 3. Module System Architecture

### Decision: Hybrid WASM + Native with Capability-Based Security

```rust
// Module trait for all modules
#[async_trait]
pub trait Module: Send + Sync {
    // Capability requirements
    fn required_capabilities(&self) -> Vec<Capability>;
    
    // Lifecycle
    async fn load(&mut self, context: ModuleContext) -> Result<()>;
    async fn unload(&mut self) -> Result<()>;
    
    // Sandboxed execution
    async fn execute(&self, op: Operation, sandbox: &Sandbox) -> Result<Value>;
}

// WASM modules run in sandbox
pub struct WasmModule {
    module: wasmtime::Module,
    instance: wasmtime::Instance,
    capabilities: GrantedCapabilities,
}

// Native modules for performance-critical code
pub struct NativeModule {
    library: libloading::Library,
    capabilities: GrantedCapabilities,
    resource_limits: ResourceLimits,
}

// Capability definitions
pub enum Capability {
    // Data access
    ReadEntity { entity_type: EntityType },
    WriteEntity { entity_type: EntityType },
    
    // System resources
    NetworkAccess { endpoints: Vec<String> },
    FileSystemAccess { paths: Vec<PathBuf> },
    
    // Computation
    SpawnThread { max_threads: usize },
    AllocateMemory { max_mb: usize },
}
```

**Rationale**:
- WASM for untrusted third-party modules
- Native for core modules needing performance
- Capability system provides fine-grained security
- Resource limits prevent DoS

---

## 4. Collaboration Protocol

### Decision: Hybrid CRDT + Event Sourcing

```rust
// Each entity is both CRDT and event-sourced
pub struct CollaborativeEntity<T: Entity> {
    // CRDT for real-time collaboration
    crdt: AutomergeDoc,
    
    // Event log for audit trail
    events: Vec<Event>,
    
    // Canonical state
    entity: T,
    
    // Vector clock for ordering
    clock: VectorClock,
}

// Sync protocol
pub enum SyncMessage {
    // CRDT operations for live collaboration
    CrdtOperation(automerge::Change),
    
    // Events for permanent record
    Event(SignedEvent),
    
    // Periodic state snapshots
    Snapshot {
        entity_id: Uuid,
        version: Version,
        state: Vec<u8>,
        clock: VectorClock,
    },
    
    // Conflict detection
    ConflictDetected {
        entity_id: Uuid,
        branches: Vec<Branch>,
    },
}

// Conflict resolution strategies
pub enum ConflictResolution {
    // Automatic resolution
    LastWriteWins,
    MultiValueRegister,
    
    // Manual resolution required
    RequireUserIntervention {
        notification: ConflictNotification,
    },
}
```

**Rationale**:
- CRDTs enable real-time collaboration without central coordination
- Event sourcing provides complete audit trail
- Hybrid approach balances performance and consistency
- Flexible conflict resolution strategies

---

## 5. Query Language Design

### Decision: GraphQL-Inspired with Genealogy Extensions

```rust
// Query examples
query {
    theory(id: "...") {
        state
        evidence {
            quality
            sources {
                repository
                citations(style: ChicagoNotes)
            }
        }
        analyses(type: IDENTITY_RESOLUTION) {
            conclusions {
                confidence
                reasoning_chain
            }
        }
    }
}

// Implementation
pub struct QueryEngine {
    parser: QueryParser,
    planner: QueryPlanner,
    executor: QueryExecutor,
    optimizer: QueryOptimizer,
}

// Custom genealogy query extensions
pub enum GenealogyQuery {
    // Find all ancestors to depth N
    Ancestors { person: Uuid, max_depth: usize },
    
    // Find common ancestors
    CommonAncestors { persons: Vec<Uuid> },
    
    // Relationship path
    RelationshipPath { from: Uuid, to: Uuid },
    
    // Evidence chain
    EvidenceChain { 
        claim: Claim,
        max_hops: usize 
    },
}
```

**Rationale**:
- GraphQL familiar to developers
- Extensible for genealogy-specific needs
- Efficient nested data fetching
- Strong typing throughout

---

## 6. Performance Architecture

### Decision: Multi-Level Caching with Intelligent Invalidation

```rust
pub struct CacheArchitecture {
    // L1: In-memory cache per connection
    connection_cache: DashMap<Uuid, Arc<Entity>>,
    
    // L2: Shared Redis cache
    redis_cache: RedisCache,
    
    // L3: Materialized views in PostgreSQL
    materialized_views: Vec<MaterializedView>,
    
    // Intelligent invalidation
    invalidator: CacheInvalidator,
}

// Invalidation strategies
pub enum InvalidationStrategy {
    // Time-based
    TTL { duration: Duration },
    
    // Event-based
    OnChange { entity_types: Vec<EntityType> },
    
    // Pattern-based
    TagBased { tags: Vec<String> },
    
    // Smart invalidation
    DependencyGraph { 
        track_reads: bool,
        cascade: bool 
    },
}

// Read-through cache pattern
impl CacheArchitecture {
    pub async fn get<T: Entity>(&self, id: Uuid) -> Result<T> {
        // Try L1
        if let Some(cached) = self.connection_cache.get(&id) {
            return Ok(cached.clone());
        }
        
        // Try L2
        if let Some(cached) = self.redis_cache.get(&id).await? {
            self.connection_cache.insert(id, cached.clone());
            return Ok(cached);
        }
        
        // Load from database
        let entity = self.storage.load(&id).await?;
        
        // Populate caches
        self.redis_cache.set(&id, &entity).await?;
        self.connection_cache.insert(id, entity.clone());
        
        Ok(entity)
    }
}
```

**Rationale**:
- Multi-level caching reduces database load
- Smart invalidation maintains consistency
- Read-through pattern simplifies code
- Materialized views for complex queries

---

## 7. Security Architecture

### Decision: Zero-Trust with End-to-End Encryption

```rust
pub struct SecurityArchitecture {
    // Authentication
    auth: AuthenticationService,
    
    // Authorization
    authz: AuthorizationService,
    
    // Encryption
    crypto: CryptoService,
    
    // Audit
    audit: AuditService,
}

// Every entity can be encrypted
pub trait EncryptedEntity: Entity {
    fn encrypt(&self, key: &Key) -> Result<EncryptedData>;
    fn decrypt(data: &EncryptedData, key: &Key) -> Result<Self>;
}

// Field-level encryption
#[derive(Encrypted)]
pub struct SensitiveData {
    #[encrypted]
    ssn: String,
    
    #[encrypted]
    dob: Date,
    
    #[plain]
    public_info: String,
}

// Zero-knowledge proofs for sensitive operations
pub struct ZKProof {
    statement: Statement,
    proof: Proof,
    verifier: Verifier,
}

impl Analysis {
    // Prove conclusion without revealing evidence
    pub fn prove_conclusion(&self) -> ZKProof {
        // Implementation
    }
}
```

**Rationale**:
- Zero-trust assumes no component is trusted
- E2E encryption protects data at rest and in transit
- Field-level encryption for granular protection
- ZK proofs enable privacy-preserving collaboration

---

## 8. Testing Strategy

### Decision: Property-Based Testing with Simulation

```rust
#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    
    // Property: state transitions maintain invariants
    proptest! {
        #[test]
        fn state_transitions_valid(
            initial_state in any::<TheoryState>(),
            transitions in vec(any::<StateTransition>(), 0..100)
        ) {
            let mut theory = Theory::new_in_state(initial_state);
            
            for transition in transitions {
                if let Ok(_) = theory.try_transition(transition) {
                    // Invariant: theory always has valid state
                    assert!(theory.validate().is_ok());
                    
                    // Invariant: version increases
                    assert!(theory.version() > 0);
                }
            }
        }
    }
    
    // Simulation testing for collaboration
    #[test]
    fn collaboration_simulation() {
        let mut sim = CollaborationSimulator::new();
        
        // Add actors
        sim.add_actor("Alice", BehaviorProfile::FastTypist);
        sim.add_actor("Bob", BehaviorProfile::Methodical);
        sim.add_actor("Carol", BehaviorProfile::Intermittent);
        
        // Run simulation
        sim.run_for(Duration::hours(1));
        
        // Check invariants
        assert_no_data_loss(&sim);
        assert_eventual_consistency(&sim);
        assert_conflict_resolution(&sim);
    }
}
```

**Rationale**:
- Property testing catches edge cases
- Simulation tests real-world scenarios
- Invariant checking ensures correctness
- Behavior profiles test different usage patterns

---

## 9. Error Handling Philosophy

### Decision: Explicit Error Types with Recovery Strategies

```rust
// Comprehensive error type hierarchy
#[derive(Error, Debug)]
pub enum RpError {
    #[error("Entity not found: {id}")]
    EntityNotFound { id: Uuid },
    
    #[error("Invalid state transition: {from} -> {to}")]
    InvalidTransition { 
        from: String, 
        to: String,
        #[recovery]
        allowed: Vec<String>
    },
    
    #[error("Conflict detected: {description}")]
    Conflict { 
        description: String,
        #[recovery]
        resolution_strategies: Vec<ResolutionStrategy>
    },
    
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
}

// Recovery strategies embedded in errors
pub trait Recoverable {
    fn recovery_strategies(&self) -> Vec<RecoveryStrategy>;
    fn can_retry(&self) -> bool;
    fn suggested_action(&self) -> UserAction;
}
```

**Rationale**:
- Explicit errors make handling clear
- Recovery strategies guide resolution
- User actions for better UX
- Retry logic built into errors

---

## 10. Deployment Architecture

### Decision: Cloud-Native with Edge Computing

```rust
// Deployment configuration
pub struct Deployment {
    // Core services
    core: CoreServices {
        api_servers: Vec<ApiServer>,
        databases: Vec<Database>,
        cache_cluster: CacheCluster,
    },
    
    // Edge nodes for low latency
    edge_nodes: Vec<EdgeNode>,
    
    // Specialized services
    analysis_workers: WorkerPool,
    storage_tier: StorageTier,
}

// Edge computing for local-first
pub struct EdgeNode {
    location: GeographicRegion,
    capabilities: Vec<Capability>,
    sync_strategy: SyncStrategy,
}

// Kubernetes operator for deployment
pub struct RpOperator {
    k8s_client: kube::Client,
    reconciler: Reconciler,
}
```

**Rationale**:
- Cloud-native enables scaling
- Edge nodes reduce latency
- Local-first improves user experience
- K8s operator simplifies deployment

---

## Summary of Key Decisions

1. **Hybrid Storage**: PostgreSQL + JSONB + specialized tables
2. **Compile-Time State Machines**: Type-safe with zero overhead
3. **Dual Module System**: WASM + Native with capabilities
4. **CRDT + Event Sourcing**: For collaboration
5. **GraphQL-Inspired Queries**: With genealogy extensions
6. **Multi-Level Caching**: With smart invalidation
7. **Zero-Trust Security**: With E2E encryption
8. **Property-Based Testing**: With simulation
9. **Explicit Errors**: With recovery strategies
10. **Cloud-Native + Edge**: For global deployment

These decisions provide a solid foundation for building a revolutionary genealogical research platform that is performant, secure, and scalable.