# Next Session Prompt for ResearchProcess-GPS
### Generated: 2025-08-01 05:07:00 EEST

---

## 🚨 START HERE - CRITICAL CONTEXT

**MANDATORY READING**:
1. **Session Handover**: `COMPREHENSIVE_HANDOVER_2025_08_01_0505_EEST.md`
2. **Master Plan v2.9**: `RESEARCHPROCESS_GPS_MASTER_PLAN_v2.9_2025_08_01_0502_EEST.md`
3. **Module Design**: `MODULE_SYSTEM_DESIGN.md`
4. **CLAUDE.md**: Project-specific instructions and NO_FALLBACK_POLICY

**Current Status**:
- Phase 3: 98% complete (only optional rate limiting remains)
- Phase 4: 40% complete (module system foundation ready)
- Module architecture implemented with Wasmtime + native support
- Research Log native module working
- All tests passing

---

## 🎯 PRIORITY TASKS FOR NEXT SESSION

### 1. Create Research Log WASM Module
```bash
# Set up WASM target
rustup target add wasm32-wasi

# Create WASM version in modules/research-log-wasm/
# Adapt the native code for WASM compilation
# Key changes needed:
# - Remove tokio dependencies (use wasi for async)
# - Ensure no_std compatibility if possible
# - Create proper WASM exports
```

### 2. Begin Module SDK Development
```bash
# Create new crate
cargo new crates/rp-module-sdk --lib

# Design SDK to provide:
# - Trait implementations helpers
# - Macro for module boilerplate
# - Communication utilities
# - Common patterns
```

### 3. Test WASM Module Loading
```bash
# Compile Research Log to WASM
cd modules/research-log-wasm
cargo build --target wasm32-wasi --release

# Test loading in module system
# Verify sandboxing works
# Check resource limits enforcement
```

### 4. Create Module Development Guide
- Write comprehensive documentation
- Include examples for both native and WASM
- Document security best practices
- Add troubleshooting section

---

## 💻 ENVIRONMENT SETUP

### Database Connection
```bash
export DB_NAME=researchprocess_gps
export DB_USER=researchprocess_gps
export DB_PASSWORD=researchprocess_gps
export DB_HOST=192.168.10.90
export DB_PORT=5432
```

### Run Server
```bash
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run --bin rp-server
```

### Test Endpoints
- Health: http://localhost:8080/health
- OpenAPI JSON: http://localhost:8080/api-docs/openapi.json
- Swagger UI: http://localhost:8080/swagger-ui/

---

## ⚠️ CRITICAL REMINDERS

1. **NO_FALLBACK_POLICY**: Maintain 100% compliance - no shortcuts!
2. **Module Security**: Capabilities must be explicitly checked
3. **Resource Limits**: Must be enforced for WASM modules
4. **Test Coverage**: Create tests for each module component
5. **Error Messages**: Make them helpful and specific

---

## 📊 PHASE 4 PROGRESS TRACKING

**Completed (40%)**:
- ✅ WASM runtime research (Wasmtime selected)
- ✅ Module system architecture design
- ✅ Core traits and types defined
- ✅ Capability system implemented
- ✅ Resource limiting framework
- ✅ Communication protocol defined
- ✅ Module registry and loader structure
- ✅ Native module loading implementation
- ✅ WASM module execution setup
- ✅ Research Log native module

**Remaining (60%)**:
- 🔲 Research Log WASM module
- 🔲 Module SDK (rp-module-sdk)
- 🔲 Helper macros for module creation
- 🔲 Module development documentation
- 🔲 Hot-reload capability
- 🔲 Additional example modules
- 🔲 Module marketplace design
- 🔲 Module versioning system
- 🔲 Module dependency management
- 🔲 Module testing framework

---

## 🔧 TECHNICAL CONTEXT

### Module System Stack
- **Native**: libloading 0.8 for dynamic libraries
- **WASM**: Wasmtime 25.0 with WASI Preview1
- **Communication**: Async message-passing
- **Security**: Capability-based model
- **Resources**: WasmtimeResourceLimiter

### Key Module Files
- `crates/rp-modules/src/loader.rs` - Module loading logic
- `crates/rp-modules/src/module.rs` - ResearchModule trait
- `crates/rp-modules/src/capabilities.rs` - Security model
- `modules/research-log/src/lib.rs` - Example native module

### Known Issues
- System may have Xorg CPU issues (see CPU_ISSUE_DIAGNOSIS.md)
- Integration tests can be memory-intensive
- WASM module cloning not implemented (not needed yet)

---

## 📋 CODE QUALITY CHECKLIST

Before committing:
1. Run `cargo test` - all tests must pass
2. Run `cargo clippy` - address all warnings
3. Check for `unwrap()` usage - NO_FALLBACK_POLICY
4. Ensure proper error handling throughout
5. Document public APIs
6. Update relevant documentation

---

## 🚀 QUICK START COMMANDS

```bash
# Switch to project
psw research

# Check current status
git status

# Run all tests
cargo test

# Check specific crate
cargo check -p rp-modules

# Build everything
cargo build

# Build Research Log module
cargo build -p research-log-module
```

---

**Start by reading the comprehensive handover, then focus on creating the WASM version of the Research Log module. This will validate our dual module support architecture and provide a template for future WASM modules.**

Remember: NO_FALLBACK_POLICY throughout all work!