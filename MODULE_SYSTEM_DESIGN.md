# ResearchProcess-GPS Module System Design
## Architecture for Extensible Research Tools
### Created: 2025-08-01 03:50:00 EEST

---

## 🎯 EXECUTIVE SUMMARY

The ResearchProcess-GPS Module System will provide a secure, extensible framework for custom research tools and integrations. The system will support both **native Rust modules** for maximum performance and **WASM modules via Wasmtime** for secure sandboxing of untrusted code.

---

## 📐 ARCHITECTURE OVERVIEW

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    Module Host (rp-modules)                 │
├─────────────────────────────────────────────────────────────┤
│  Module Manager │ Registry │ Loader │ Communication Bridge │
├─────────────────────────────────────────────────────────────┤
│                    Wasmtime Runtime                         │
│  Sandboxing │ Resource Limits │ WASI │ Memory Isolation   │
├─────────────────────────────────────────────────────────────┤
│                    Module Interface                         │
│  Entity Access │ Event Bus │ Storage │ Capabilities API    │
└─────────────────────────────────────────────────────────────┘
```

### Module Types

1. **Native Rust Modules** (Phase 1)
   - Compiled as dynamic libraries
   - Full access to ResearchProcess-GPS APIs
   - For trusted, first-party modules

2. **WASM Modules** (Phase 2)
   - Sandboxed execution via Wasmtime
   - Limited capabilities through WASI
   - For third-party or untrusted modules

---

## 🔒 SECURITY MODEL

### Wasmtime Selection Rationale

**Why Wasmtime over Wasmer:**
1. **Security Transparency**: Published CVEs with responsible disclosure
2. **Defense in Depth**: Multiple security layers (PCC, CFI, guard regions)
3. **Community Governance**: Bytecode Alliance non-profit backing
4. **Rust Safety**: Leverages Rust's type system for API safety

### Capability-Based Security

Modules request capabilities in their manifest:
```toml
[module]
name = "research-log"
version = "1.0.0"
type = "wasm"

[capabilities]
entity_read = ["ResearchLog", "Theory", "Evidence"]
entity_write = ["ResearchLog"]
event_subscribe = ["Theory.StateChanged", "Evidence.Created"]
storage_quota = "10MB"
memory_limit = "64MB"
cpu_time_limit = "100ms"
```

### Resource Limits

Using Wasmtime's ResourceLimiter trait:
```rust
pub struct ModuleResourceLimits {
    memory_bytes: usize,      // Max memory allocation
    table_elements: u32,      // Max table size
    instances: u32,           // Max instances
    cpu_time_ms: u64,         // Max CPU time per call
    storage_bytes: usize,     // Max storage usage
}
```

---

## 🔌 MODULE API DESIGN

### Core Module Trait

```rust
#[async_trait]
pub trait ResearchModule: Send + Sync {
    /// Module metadata
    fn metadata(&self) -> &ModuleMetadata;
    
    /// Initialize module with host context
    async fn initialize(&mut self, context: ModuleContext) -> Result<()>;
    
    /// Handle incoming events
    async fn handle_event(&mut self, event: ModuleEvent) -> Result<()>;
    
    /// Execute module-specific commands
    async fn execute_command(
        &mut self, 
        command: &str, 
        args: serde_json::Value
    ) -> Result<serde_json::Value>;
    
    /// Cleanup before unload
    async fn shutdown(&mut self) -> Result<()>;
}
```

### Communication Protocol

**Message-Passing Architecture:**
```rust
pub enum ModuleMessage {
    // Host -> Module
    Event(DomainEvent),
    Command { id: Uuid, cmd: String, args: Value },
    QueryRequest { id: Uuid, query: Query },
    
    // Module -> Host
    EntityOperation { op: Operation, entity: StorageEntity },
    EventEmit { event: DomainEvent },
    QueryResponse { id: Uuid, result: QueryResult },
    Log { level: LogLevel, message: String },
}
```

### Module Context

```rust
pub struct ModuleContext {
    /// Module's unique instance ID
    pub instance_id: Uuid,
    
    /// Workspace context
    pub workspace_id: WorkspaceId,
    
    /// Actor (researcher) context
    pub actor_id: EntityId,
    
    /// Capability tokens
    pub capabilities: ModuleCapabilities,
    
    /// Communication channel to host
    pub host_channel: Sender<ModuleMessage>,
}
```

---

## 📦 MODULE PACKAGING

### Module Structure

```
research-log-module/
├── Cargo.toml
├── module.toml          # Module manifest
├── src/
│   ├── lib.rs          # Module implementation
│   └── api.rs          # Public API
├── assets/             # Static resources
└── README.md
```

### Module Manifest (module.toml)

```toml
[module]
name = "research-log"
version = "1.0.0"
description = "Advanced research logging and analysis"
author = "ResearchProcess-GPS Team"
license = "MIT"
type = "wasm"  # or "native"

[dependencies]
rp-module-sdk = "0.1.0"

[capabilities]
entity_read = ["ResearchLog", "Theory", "Evidence", "Analysis"]
entity_write = ["ResearchLog", "Analysis"]
entity_create = ["ResearchLog", "Analysis"]
event_subscribe = ["*"]
event_emit = ["ResearchLog.*"]

[resources]
memory_limit = "64MB"
storage_quota = "10MB"
cpu_time_limit = "100ms"
network_access = false

[exports]
commands = ["analyze", "summarize", "export"]
ui_components = ["research-log-viewer", "analysis-chart"]
```

---

## 🔄 MODULE LIFECYCLE

### 1. Discovery
- Scan module directories
- Validate manifests
- Check signatures (future)

### 2. Loading
- Parse module manifest
- Verify capabilities
- Initialize Wasmtime instance (for WASM)
- Load module code

### 3. Initialization
- Create module context
- Establish communication channels
- Call module.initialize()
- Register event subscriptions

### 4. Runtime
- Route events to modules
- Handle command execution
- Monitor resource usage
- Enforce limits

### 5. Hot-Reload
- Detect module changes
- Graceful shutdown of old instance
- Load new version
- Migrate state if needed

### 6. Shutdown
- Call module.shutdown()
- Clean up resources
- Remove event subscriptions
- Destroy Wasmtime instance

---

## 🚀 IMPLEMENTATION PLAN

### Phase 1: Native Module Support (Week 1)
1. Create `rp-modules` crate structure
2. Define Module trait and APIs
3. Implement module registry and loader
4. Create Research Log example module
5. Test hot-reload capability

### Phase 2: WASM Sandbox (Week 2)
1. Integrate Wasmtime
2. Implement ResourceLimiter
3. Create WASI bindings for module API
4. Build WASM module SDK
5. Convert Research Log to WASM

### Phase 3: Module SDK (Week 3)
1. Create `rp-module-sdk` crate
2. Provide helper macros for module creation
3. Document module development
4. Create module template
5. Build example modules

---

## 🛡️ SECURITY CONSIDERATIONS

### Threat Model
1. **Malicious Modules**: Prevented by sandboxing
2. **Resource Exhaustion**: Prevented by limits
3. **Data Exfiltration**: Prevented by capability model
4. **Privilege Escalation**: Prevented by WASI capabilities

### Defense Strategies
1. **Sandbox All Third-Party Modules**: Mandatory WASM
2. **Capability Principle of Least Privilege**: Only granted what's needed
3. **Resource Quotas**: Hard limits on CPU, memory, storage
4. **Audit Logging**: All module operations logged
5. **Module Signing**: Future enhancement for trust

---

## 📊 EXAMPLE MODULES

### 1. Research Log Module
- Enhanced logging with templates
- Activity analysis
- Time tracking
- Research coverage reports

### 2. Evidence Matrix Module
- Visual evidence organization
- Correlation analysis
- Conflict detection
- Source quality scoring

### 3. Timeline Module
- Chronological event visualization
- Date range analysis
- Temporal clustering
- Missing period detection

### 4. GEDCOM Import/Export
- GEDCOM 7 compatibility
- Data mapping
- Validation
- Round-trip preservation

### 5. AI Assistant Module
- Research suggestions
- Pattern detection
- Hypothesis generation
- Natural language queries

---

## 🔧 TECHNICAL DECISIONS

### Decision: WASM Runtime
**Choice**: Wasmtime
**Alternatives**: Wasmer, WasmEdge, Native only
**Rationale**: Security, transparency, Rust integration

### Decision: Communication Model
**Choice**: Message-passing
**Alternatives**: Direct function calls, Shared memory
**Rationale**: Isolation, async support, debugging

### Decision: Module Format
**Choice**: Dual (Native + WASM)
**Alternatives**: WASM only, Native only
**Rationale**: Performance for trusted, security for untrusted

### Decision: Resource Limits
**Choice**: Hard limits with quotas
**Alternatives**: Soft limits, No limits
**Rationale**: Prevent DoS, ensure fairness

---

## 📈 SUCCESS METRICS

1. **Module Load Time**: < 100ms
2. **Message Latency**: < 1ms
3. **Memory Overhead**: < 10MB per module
4. **Hot-Reload Time**: < 500ms
5. **API Coverage**: 100% of core entities accessible

---

## 🔄 FUTURE ENHANCEMENTS

1. **Module Marketplace**: Community modules
2. **Module Composition**: Modules using other modules
3. **Visual Module Builder**: No-code module creation
4. **Cross-Module Communication**: Direct module-to-module
5. **Distributed Modules**: Network-capable modules

---

*This design provides a secure, performant, and extensible module system for ResearchProcess-GPS, enabling custom research tools while maintaining system integrity.*