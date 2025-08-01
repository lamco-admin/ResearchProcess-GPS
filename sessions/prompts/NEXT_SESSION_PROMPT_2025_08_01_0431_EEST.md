# ResearchProcess-GPS Next Session Prompt
## Continue Phase 4: Module System Implementation
### Created: 2025-08-01 04:31:00 EEST

---

🚨 START HERE - CRITICAL CONTEXT

MANDATORY READING:
1. Session Handover: sessions/handovers/SESSION_HANDOVER_2025_08_01_0429_EEST.md
2. Master Plan v2.8: RESEARCHPROCESS_GPS_MASTER_PLAN_v2.8_2025_08_01_0325_EEST.md
3. Module Design: MODULE_SYSTEM_DESIGN.md
4. CLAUDE.md: Project-specific instructions and NO_FALLBACK_POLICY

Current Status:
- Phase 3: 98% complete (only optional rate limiting remains)
- Phase 4: 20% complete (module system structure created)
- Module architecture designed with Wasmtime + native support
- rp-modules crate compiles successfully
- All tests passing (event sourcing tests properly deferred)

---

🎯 PRIORITY TASKS FOR NEXT SESSION

1. Complete Module Loading Implementation
   - Implement native module loading with libloading
   - Complete WASM module instantiation
   - Set up WASI bindings and capabilities
   - Implement module lifecycle methods
   - Test with simple "hello world" modules

2. Create Research Log Module Prototype
   - Create modules/research-log directory
   - Write module.toml manifest
   - Implement ResearchModule trait
   - Create both native and WASM versions
   - Test capability enforcement
   - Test resource limiting

3. Begin Module SDK Development
   - Create rp-module-sdk crate
   - Define helper macros for module creation
   - Create module template/boilerplate
   - Write basic documentation

4. Fix Module Execution
   - Complete WasmModuleInstance implementation
   - Wire up message passing
   - Implement proper WASI context
   - Test module initialization and shutdown

---

💻 ENVIRONMENT SETUP

Database Connection:
```bash
export DB_NAME=researchprocess_gps
export DB_USER=researchprocess_gps
export DB_PASSWORD=researchprocess_gps
export DB_HOST=192.168.10.90
export DB_PORT=5432
```

Run Server:
```bash
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run --bin rp-server
```

Test Endpoints:
- Health: http://localhost:8080/health
- OpenAPI JSON: http://localhost:8080/api-docs/openapi.json
- Swagger UI: http://localhost:8080/swagger-ui/

---

⚠️ CRITICAL REMINDERS

1. **NO_FALLBACK_POLICY**: Maintain 100% compliance - no shortcuts!
2. **Module Security**: Capabilities must be explicitly checked
3. **Resource Limits**: Must be enforced for WASM modules
4. **Test Coverage**: Create tests for each module component

---

📊 PHASE 4 PROGRESS

Completed:
- ✅ WASM runtime research (Wasmtime selected)
- ✅ Module system architecture design
- ✅ Core traits and types defined
- ✅ Capability system implemented
- ✅ Resource limiting framework
- ✅ Communication protocol defined
- ✅ Module registry and loader structure

Remaining:
- 🔲 Native module loading implementation
- 🔲 WASM module execution
- 🔲 Module SDK
- 🔲 Research Log example module
- 🔲 Hot-reload capability
- 🔲 Module lifecycle tests

---

🔧 TECHNICAL CONTEXT

Module System Stack:
- Wasmtime 25.0 for WASM sandboxing
- libloading 0.8 for native modules
- Message-passing communication
- Capability-based security
- Hard resource limits

Key Files:
- crates/rp-modules/src/loader.rs - Needs completion
- crates/rp-modules/src/module.rs - ResearchModule trait
- crates/rp-modules/src/capabilities.rs - Security model
- crates/rp-modules/src/communication.rs - Message protocol

---

📋 CODE QUALITY CHECKLIST

Before committing:
1. Run `cargo test` - all tests must pass
2. Run `cargo clippy` - address all warnings
3. Check for unwrap() usage - NO_FALLBACK_POLICY
4. Ensure proper error handling throughout
5. Document public APIs

---

🚀 QUICK START COMMANDS

```bash
# Switch to project
psw research

# Check current status
git status

# Run tests
cargo test

# Check module crate
cargo check -p rp-modules

# Build everything
cargo build
```

---

Start by reading the session handover, then focus on implementing module loading
and creating the Research Log prototype. Remember: NO_FALLBACK_POLICY throughout!