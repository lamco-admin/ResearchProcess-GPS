# Storage Abstraction Architecture for ResearchProcess-GPS

## Overview

ResearchProcess-GPS is designed as a storage-agnostic protocol and engine. While our reference implementation uses PostgreSQL with pgvector and Apache AGE extensions, the system must support diverse storage backends to accommodate different use cases and deployment scenarios.

## Core Design Principles

### 1. Protocol-First Architecture
- Storage is an implementation detail, not a protocol requirement
- All storage operations go through abstraction traits
- Storage backends are pluggable modules

### 2. Feature-Based Capabilities
- Not all backends support all features
- Modules declare their storage requirements
- Runtime capability detection and graceful degradation

### 3. Standalone-First, Collaboration-Ready
- Default to local, single-user operation
- Collaboration features are optional extensions
- No mandatory network connectivity

## Storage Backend Types

### 1. Database Backends

#### PostgreSQL (Reference Implementation)
```toml
[features]
postgres = ["sqlx", "sqlx-postgres"]
postgres-vector = ["postgres", "pgvector"]
postgres-graph = ["postgres", "apache-age"]
```

**Capabilities:**
- Full ACID transactions
- JSONB for flexible schema evolution
- Binary storage for efficient protocol messages
- Vector search via pgvector extension
- Graph queries via Apache AGE extension
- Materialized views for performance
- Listen/Notify for real-time updates

#### SQLite
```toml
[features]
sqlite = ["sqlx", "sqlx-sqlite"]
sqlite-vec = ["sqlite", "sqlite-vec"]
```

**Capabilities:**
- Single-file database
- ACID transactions
- JSON storage (text-based)
- Limited concurrent access
- Optional vector search via sqlite-vec

#### Other Databases
- **MySQL/MariaDB**: Via JSON columns
- **DuckDB**: For analytical workloads
- **SurrealDB**: Native graph + document store

### 2. File-Based Backends

#### Git Repository
```toml
[features]
storage-git = ["git2"]
```

**Capabilities:**
- Version control built-in
- Distributed by design
- Human-readable formats (JSON/YAML)
- Natural collaboration model
- GitHub/GitLab integration

**Structure:**
```
.researchprocess/
├── config.toml
├── theories/
│   ├── {uuid-v7}.json
│   └── {uuid-v7}/
│       ├── evidence/
│       ├── analyses/
│       └── attachments/
├── persons/
├── sources/
└── index/
    ├── persons.json
    └── theories.json
```

#### Plain Filesystem
```toml
[features]
storage-fs = ["tokio/fs"]
```

**Capabilities:**
- Simple directory structure
- No external dependencies
- Easy backup/restore
- Limited query capabilities
- Manual indexing required

### 3. Cloud/API Backends

#### S3-Compatible Object Storage
```toml
[features]
storage-s3 = ["aws-sdk-s3"]
```

**Capabilities:**
- Scalable storage
- CDN integration
- Versioning support
- Event notifications
- Cross-region replication

#### Custom API Backend
```toml
[features]
storage-api = ["reqwest"]
```

**Capabilities:**
- Complete flexibility
- Integrate with existing systems
- Custom authentication
- Rate limiting aware

## Storage Abstraction Layer

### Core Traits

```rust
// crates/rp-storage/src/traits.rs

use async_trait::async_trait;
use uuid::Uuid;

/// Core storage operations that all backends must support
#[async_trait]
pub trait StorageBackend: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    type Transaction: Transaction<Error = Self::Error>;
    
    /// Initialize storage (create tables, directories, etc.)
    async fn initialize(&self) -> Result<(), Self::Error>;
    
    /// Check if storage is properly configured and accessible
    async fn health_check(&self) -> Result<HealthStatus, Self::Error>;
    
    /// Begin a new transaction
    async fn begin_transaction(&self) -> Result<Self::Transaction, Self::Error>;
    
    /// Get storage capabilities
    fn capabilities(&self) -> StorageCapabilities;
}

/// Transaction operations
#[async_trait]
pub trait Transaction: Send {
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Store an entity
    async fn put_entity(&mut self, entity: &Entity) -> Result<(), Self::Error>;
    
    /// Retrieve an entity by ID
    async fn get_entity(&mut self, id: Uuid) -> Result<Option<Entity>, Self::Error>;
    
    /// Delete an entity
    async fn delete_entity(&mut self, id: Uuid) -> Result<bool, Self::Error>;
    
    /// Commit the transaction
    async fn commit(self) -> Result<(), Self::Error>;
    
    /// Rollback the transaction
    async fn rollback(self) -> Result<(), Self::Error>;
}

/// Optional query capabilities
#[async_trait]
pub trait QueryableBackend: StorageBackend {
    /// Execute a query
    async fn query(&self, query: Query) -> Result<QueryResult, Self::Error>;
    
    /// Create an index
    async fn create_index(&self, index: IndexDefinition) -> Result<(), Self::Error>;
}

/// Optional vector search capabilities
#[async_trait]
pub trait VectorSearchBackend: StorageBackend {
    /// Store embeddings for an entity
    async fn store_embeddings(
        &self,
        entity_id: Uuid,
        embeddings: &[f32],
    ) -> Result<(), Self::Error>;
    
    /// Search by vector similarity
    async fn vector_search(
        &self,
        query_vector: &[f32],
        limit: usize,
    ) -> Result<Vec<(Uuid, f32)>, Self::Error>;
}

/// Optional graph capabilities
#[async_trait]
pub trait GraphBackend: StorageBackend {
    /// Store a relationship
    async fn store_relationship(
        &self,
        relationship: &Relationship,
    ) -> Result<(), Self::Error>;
    
    /// Traverse relationships
    async fn traverse(
        &self,
        start: Uuid,
        pattern: TraversalPattern,
    ) -> Result<Vec<Path>, Self::Error>;
}

/// Storage capabilities declaration
#[derive(Debug, Clone)]
pub struct StorageCapabilities {
    pub transactions: bool,
    pub queries: QueryCapabilities,
    pub vector_search: bool,
    pub graph_operations: bool,
    pub real_time_updates: bool,
    pub full_text_search: bool,
    pub binary_storage: bool,
    pub compression: CompressionSupport,
    pub max_entity_size: Option<usize>,
    pub concurrent_connections: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct QueryCapabilities {
    pub basic_filters: bool,
    pub json_path: bool,
    pub aggregations: bool,
    pub joins: bool,
    pub recursive_ctes: bool,
}
```

### Backend Registry

```rust
// crates/rp-storage/src/registry.rs

use std::collections::HashMap;
use std::sync::Arc;

pub struct StorageRegistry {
    backends: HashMap<String, Arc<dyn BackendFactory>>,
}

impl StorageRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            backends: HashMap::new(),
        };
        
        // Register built-in backends
        #[cfg(feature = "postgres")]
        registry.register("postgres", PostgresBackendFactory::new());
        
        #[cfg(feature = "sqlite")]
        registry.register("sqlite", SqliteBackendFactory::new());
        
        #[cfg(feature = "storage-git")]
        registry.register("git", GitBackendFactory::new());
        
        #[cfg(feature = "storage-fs")]
        registry.register("filesystem", FilesystemBackendFactory::new());
        
        registry
    }
    
    pub fn create_backend(
        &self,
        url: &str,
    ) -> Result<Box<dyn StorageBackend>, StorageError> {
        let scheme = url.split("://").next()
            .ok_or(StorageError::InvalidUrl)?;
            
        let factory = self.backends.get(scheme)
            .ok_or(StorageError::UnknownBackend(scheme.to_string()))?;
            
        factory.create(url)
    }
}
```

## Configuration

### Storage URL Schemes

```toml
# PostgreSQL with extensions
storage_url = "postgres://user:pass@localhost/genealogy?vector=true&graph=true"

# SQLite with optional vector extension
storage_url = "sqlite:///path/to/genealogy.db?vector=true"

# Git repository (local or remote)
storage_url = "git:///home/user/genealogy-research"
storage_url = "git+ssh://git@github.com/user/research.git"

# Filesystem
storage_url = "file:///home/user/genealogy-data"

# S3-compatible
storage_url = "s3://bucket-name/prefix?region=us-east-1"

# Custom API
storage_url = "https://api.mygenealogy.com/v1/storage"
```

### Module Requirements Declaration

```toml
# In a module's manifest
[module]
name = "relationship-analyzer"
version = "1.0.0"

[module.requires]
storage = ["queries", "graph_operations"]

[module.optional]
storage = ["vector_search"]
```

## Implementation Strategy

### Phase 1: Core Abstraction (Week 1-2)
1. Define storage traits
2. Implement PostgreSQL backend
3. Create backend registry
4. Add capability detection

### Phase 2: Alternative Backends (Week 3-4)
1. SQLite implementation
2. Git backend for researchers who prefer version control
3. Filesystem for simple deployments

### Phase 3: Advanced Features (Week 5-6)
1. Vector search abstraction
2. Graph operations abstraction
3. Real-time updates via channels

### Phase 4: Cloud Backends (Month 2)
1. S3-compatible backend
2. API backend framework
3. Caching layer

## Usage Examples

### Basic Usage

```rust
use rp_storage::StorageRegistry;

// Create storage from URL
let storage = StorageRegistry::new()
    .create_backend("postgres://localhost/genealogy")?;

// Check capabilities
if storage.capabilities().vector_search {
    println!("Vector search available!");
}

// Start a transaction
let mut tx = storage.begin_transaction().await?;

// Store a theory
let theory = Theory::new("Who were the parents of John Doe?");
tx.put_entity(&theory.into()).await?;

// Commit
tx.commit().await?;
```

### Module Checking Requirements

```rust
pub struct RelationshipAnalyzer {
    storage: Arc<dyn StorageBackend>,
}

impl Module for RelationshipAnalyzer {
    fn check_requirements(&self) -> Result<(), ModuleError> {
        let caps = self.storage.capabilities();
        
        if !caps.graph_operations {
            return Err(ModuleError::MissingCapability(
                "graph_operations required for relationship analysis"
            ));
        }
        
        Ok(())
    }
}
```

### Fallback Strategies

```rust
// Adapter that provides graph operations on top of basic storage
pub struct GraphAdapter<S: StorageBackend> {
    storage: S,
    index: GraphIndex,
}

impl<S: StorageBackend> GraphAdapter<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            index: GraphIndex::new(),
        }
    }
    
    // Implement graph operations using basic queries
    pub async fn traverse(&self, start: Uuid, pattern: TraversalPattern) 
        -> Result<Vec<Path>, Error> {
        // Use basic queries to simulate graph traversal
        // Less efficient but works on any backend
    }
}
```

## Testing Strategy

### Backend Compliance Tests

```rust
// crates/rp-storage/tests/compliance.rs

/// Test suite that all storage backends must pass
pub fn storage_compliance_tests<B: StorageBackend>(
    backend: B,
) -> Result<(), TestError> {
    // Test basic CRUD
    test_entity_storage(&backend)?;
    
    // Test transactions if supported
    if backend.capabilities().transactions {
        test_transactions(&backend)?;
    }
    
    // Test queries if supported
    if backend.capabilities().queries.basic_filters {
        test_basic_queries(&backend)?;
    }
    
    Ok(())
}
```

## Security Considerations

1. **Authentication**: Each backend handles its own auth
2. **Encryption**: Optional at-rest encryption
3. **Access Control**: Module-level permissions
4. **Audit Logging**: Storage-level audit trail

## Performance Considerations

1. **Caching**: LRU cache in front of any backend
2. **Batching**: Batch operations for efficiency
3. **Indexes**: Backend-specific optimizations
4. **Connection Pooling**: For database backends

## Future Extensions

1. **Blockchain Backend**: For immutable audit trails
2. **IPFS Backend**: For distributed storage
3. **Hybrid Backends**: Combine multiple backends
4. **Replication**: Cross-backend sync

## Decision: UUID v7

We will use UUID v7 (time-ordered UUIDs) for all entity identifiers:

- **Time-ordered**: Natural chronological sorting
- **Globally unique**: No coordination required
- **Standard**: RFC-compliant, widely supported
- **Efficient**: Better index performance than random UUIDs
- **Timestamp extraction**: Can derive creation time

```rust
use uuid::Uuid;

pub fn generate_id() -> Uuid {
    Uuid::now_v7()
}
```

## Summary

This storage abstraction architecture ensures ResearchProcess-GPS can:

1. Run standalone with just SQLite or filesystem
2. Scale to PostgreSQL with advanced features
3. Integrate with existing version control workflows
4. Deploy to cloud environments
5. Support custom storage solutions

The key is that storage is always an implementation detail, never a protocol requirement. Users choose the backend that fits their needs, from a simple local file to a distributed graph database.