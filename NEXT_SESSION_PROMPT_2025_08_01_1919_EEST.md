# NEXT SESSION PROMPT
## Timestamp: 2025-08-01 19:19:49 EEST

```
🚨 MANDATORY INITIAL STEPS

1. Read comprehensive handover:
cat COMPREHENSIVE_HANDOVER_2025_08_01_1919_EEST.md

2. Read updated master plan:
cat RESEARCHPROCESS_GPS_MASTER_PLAN_v3.9_2025_08_01_1919_EEST.md

SESSION OBJECTIVES: Complete ModuleLoader FFI Refactoring

Current Status: ModuleLoader 60% refactored to message-based FFI
- NativeModuleInstance uses FFI function pointers
- AtomicPtr implementation started but incomplete
- Build errors due to Send/Sync issues

IMMEDIATE TASKS:
1. Fix AtomicPtr Implementation
   - Complete all atomic operations in send_message, shutdown
   - Ensure thread safety for FFI calls
   - Fix Send/Sync trait implementation

2. Complete Build
   export CARGO_BUILD_JOBS=1
   cargo build --release

3. Test Native Modules
   cargo test --test native_module_test -- --nocapture
   # Should get 7/7 tests passing

4. Verify No Regressions
   cargo test --test wasm_module_test -- --nocapture
   cargo test --test research_log_ffi_test -- --nocapture

🔧 CRITICAL BUILD REQUIREMENTS

# MANDATORY for ALL cargo commands
export CARGO_BUILD_JOBS=1

# Timeouts: 600000ms for builds, 300000ms for tests

Key Context:
- Working on: /crates/rp-modules/src/loader.rs
- Issue: Raw pointers not Send/Sync, using AtomicPtr
- Platform difference: Linux (no underscore), macOS (underscore) in symbols
- NO_FALLBACK_POLICY: Do NOT work around problems!

After ModuleLoader complete:
- Create Module SDK
- Write FFI documentation
- Memory sanitizer testing
```