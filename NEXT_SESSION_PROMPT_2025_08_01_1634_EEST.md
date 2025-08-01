# NEXT SESSION PROMPT - Complete Module System & SDK
## ResearchProcess-GPS Module System Completion
### Generated: 2025-08-01 16:34:00 EEST

---

## 🚨 CRITICAL - READ FIRST

**MANDATORY INITIAL STEPS**:
1. Read the comprehensive handover:
   ```bash
   cat COMPREHENSIVE_HANDOVER_2025_08_01_1634_EEST.md
   ```

2. Read the updated master plan:
   ```bash
   cat RESEARCHPROCESS_GPS_MASTER_PLAN_v3.5_2025_08_01_1634_EEST.md
   ```

3. Check NO_FALLBACK_POLICY:
   ```bash
   cat /home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md
   ```

---

## 🎯 SESSION OBJECTIVES

### Primary Goal: Complete Native Module Integration & Fix SDK

**Current Status**: Module system 88% complete
- WASM modules: ✅ Fully working with C-style exports
- Native modules: ⚠️ Loading works, execution needs completion
- Module SDK: ⚠️ Structure complete, API misaligned

### Critical Issues to Fix:

1. **ModuleLoader needs Clone implementation**
   - Required for concurrent operations
   - Blocking native module tests
   - Affects module lifecycle management

2. **Command execution uses mock responses**
   - execute_command returns hardcoded JSON
   - Need actual command routing to modules
   - Both native and WASM paths affected

3. **SDK API doesn't match actual module system**
   - Wrong ModuleMessage variants
   - Incorrect import paths
   - Missing proper event handling

---

## 📋 SPECIFIC TASKS

### Task 1: Implement Clone for ModuleLoader
```rust
// In crates/rp-modules/src/loader.rs
#[derive(Clone)]
pub struct ModuleLoader {
    engine: Arc<Engine>, // Already Arc, just need to derive Clone
    instances: Arc<RwLock<Vec<Box<dyn ModuleInstance>>>>,
    wasm_compiler: Arc<wasmtime::Engine>,
}
```

### Task 2: Fix execute_command Implementation
- Remove the mock response that returns fake log_id
- Implement actual command routing through ModuleMessage
- Handle CommandResponse properly
- Test with real module execution

### Task 3: Fix Module SDK API Alignment
```rust
// Current (wrong):
use rp_protocol::module::{ModuleMessage, ModuleCapability, ResourceLimits};

// Should be:
use rp_modules::{
    communication::ModuleMessage,
    capabilities::Capability as ModuleCapability,
    resource_limits::ResourceLimits,
};
```

### Task 4: Run Native Module Tests
```bash
CARGO_BUILD_JOBS=1 cargo test --package rp-modules --test native_module_test
```

### Task 5: Create Working Module Example
- Use the SDK to create a simple module
- Test it loads and executes commands
- Document the process

---

## 🔧 TECHNICAL CONTEXT

### Working Directory
```bash
cd /home/greg/ResearchProcess-GPS
```

### Build Commands
```bash
# Always use single job for stability
export CARGO_BUILD_JOBS=1

# Build everything
cargo build

# Run specific tests
cargo test --package rp-modules
```

### Module Paths
- Native module: `/modules/research-log/`
- WASM module: `/modules/research-log-wasm/`
- Module SDK: `/crates/rp-module-sdk/`

### Key Files to Modify
1. `/crates/rp-modules/src/loader.rs` - Add Clone, fix execute_command
2. `/crates/rp-module-sdk/src/lib.rs` - Fix imports
3. `/crates/rp-module-sdk/src/native.rs` - Fix ModuleMessage usage
4. `/crates/rp-modules/tests/native_module_test.rs` - Already written, needs Clone

---

## ⚠️ IMPORTANT REMINDERS

1. **NO_FALLBACK_POLICY**: Zero tolerance for workarounds
2. **Build Times**: 5-10 minutes are NORMAL - be patient
3. **Test Everything**: Don't assume anything works
4. **Document Changes**: Update master plan with progress

---

## 🎯 SUCCESS CRITERIA

By end of session:
1. ✅ ModuleLoader implements Clone
2. ✅ Native module tests pass
3. ✅ Command execution works (no mocks)
4. ✅ SDK compiles without errors
5. ✅ At least one example module using SDK
6. ✅ Module system >90% complete

---

## 🚀 QUICK START COMMANDS

```bash
# Start here
cd /home/greg/ResearchProcess-GPS
cat COMPREHENSIVE_HANDOVER_2025_08_01_1634_EEST.md

# Check current test status
CARGO_BUILD_JOBS=1 cargo test --package rp-modules --test native_module_test

# You'll see Clone errors - fix those first!
```

---

*Begin by reading the handover, then implement Clone for ModuleLoader. The path forward is clear.*