# NEXT SESSION PROMPT
## ResearchProcess-GPS Module System Development
### Timestamp: 2025-08-01 18:31:16 EEST

---

## 🚨 MANDATORY INITIAL STEPS

1. **READ COMPREHENSIVE HANDOVER FIRST**:
   ```bash
   cat COMPREHENSIVE_HANDOVER_2025_08_01_1831_EEST.md
   ```

2. **READ UPDATED MASTER PLAN**:
   ```bash
   cat RESEARCHPROCESS_GPS_MASTER_PLAN_v3.8_2025_08_01_1831_EEST.md
   ```

3. **PAY SPECIAL ATTENTION TO**:
   - CRITICAL BUILD REQUIREMENTS section in master plan
   - ALWAYS use CARGO_BUILD_JOBS=1 for ALL cargo commands
   - Use extended timeouts (600000ms for builds, 300000ms for tests)

---

## 🎯 SESSION OBJECTIVES

### Continue Module System Development (Phase 4 - 85% Complete)

**Context**: FFI architecture is now implemented and working! Native modules use message-based communication. No more segfaults!

**Priority Tasks**:

1. **Align WASM Module Message Format** (Quick Win)
   - Update WASM modules to use same JSON protocol as native
   - Ensure consistent message structure
   - Test both module types can use same interface

2. **Create Module SDK**
   - Implement message builder utilities
   - Add response parsing helpers
   - Create macros for common patterns
   - Location: `crates/rp-module-sdk/`

3. **Update ModuleLoader**
   - Remove trait object handling code
   - Implement message routing layer
   - Add unified module interface

4. **Write Module Developer Documentation**
   - Create FFI pattern guide
   - Document message protocol
   - Provide example module template
   - Use test-minimal as reference

5. **Memory Sanitizer Testing**
   - Run tests with ASAN/MSAN
   - Verify no memory leaks
   - Test edge cases

---

## 🔧 CRITICAL CONTEXT

### Build Requirements (MANDATORY)
```bash
# ALWAYS set before ANY cargo command
export CARGO_BUILD_JOBS=1

# Build commands
CARGO_BUILD_JOBS=1 cargo build --release

# Test commands  
CARGO_BUILD_JOBS=1 cargo test --test test_name -- --nocapture

# Timeouts
# Builds: 600000ms (10 minutes)
# Tests: 300000ms (5 minutes)
```

### Current Architecture
- Native modules: Use C-compatible FFI with JSON messages ✅
- WASM modules: Use C-style exports (needs alignment)
- Protocol: Unified JSON message format
- No trait objects across FFI boundaries!

### Key Files
- Test module: `modules/test-minimal/src/lib.rs`
- Research Log: `modules/research-log/src/lib.rs` (updated)
- Tests: `crates/rp-modules/tests/*_ffi_test.rs`

### Module Locations
- Libraries build to: `target/release/lib*.so`
- NOT in module directories (workspace precedence)

---

## 📋 SPECIFIC NEXT STEPS

1. **Start with WASM alignment**:
   ```bash
   cd /home/greg/ResearchProcess-GPS
   # Check current WASM module implementation
   cat modules/research-log-wasm/src/lib.rs
   ```

2. **Update to match native protocol**:
   - Same message types (init, command, query, mutation, event)
   - Same response format
   - Remove any trait dependencies

3. **Test both module types**:
   ```bash
   CARGO_BUILD_JOBS=1 cargo test --package rp-modules
   ```

4. **Begin SDK implementation**:
   - Start with basic message builders
   - Add serialization helpers
   - Create convenience macros

---

## ⚠️ IMPORTANT REMINDERS

- **NO_FALLBACK_POLICY**: Zero tolerance for silent failures
- **Build Performance**: Expect 5-10 minute builds, this is NORMAL
- **Xorg CPU**: High usage is X11 forwarding, not actual load
- **Always use**: CARGO_BUILD_JOBS=1 (no exceptions!)

---

## 🎉 RECENT VICTORIES

- ✅ FFI architecture completely redesigned
- ✅ Native modules working without segfaults!
- ✅ Message-based protocol implemented
- ✅ Comprehensive tests passing
- ✅ Build requirements documented

The hard part is DONE! Now it's mostly alignment and documentation work.

---

*Remember: The module system is 85% complete. Focus on WASM alignment first, then SDK, then documentation.*