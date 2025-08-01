# NEXT SESSION PROMPT - ResearchProcess-GPS Module System
## Phase 4: WASM Module Testing & SDK Development
### Generated: 2025-08-01 13:25:00 EEST

---

## 🚀 SESSION INITIALIZATION

**CRITICAL**: Read these documents IN THIS ORDER before starting any work:

1. **Comprehensive Handover**:
   ```bash
   cat COMPREHENSIVE_HANDOVER_2025_08_01_1321_EEST.md
   ```
   Key points: Build times are NORMAL, module system is sound, tests compile

2. **Master Plan**:
   ```bash
   cat RESEARCHPROCESS_GPS_MASTER_PLAN_v3.2_2025_08_01_1321_EEST.md
   ```
   Note: Phase 4 is 47% complete and UNBLOCKED

3. **Build Performance Guide**:
   ```bash
   cat docs/development/BUILD_PERFORMANCE_GUIDE_2025_08_01_1230_EEST.md
   ```
   Remember: 5-10 minute builds are NORMAL

---

## 📋 IMMEDIATE TASKS

### 1. Build WASM Module
```bash
# Navigate to WASM module
cd /home/greg/ResearchProcess-GPS/modules/research-log-wasm

# Build for WASM target
cargo build --target wasm32-wasip1 --release

# Verify output exists
ls -la ../../target/wasm32-wasip1/release/research_log_wasm.wasm
```

### 2. Test WASM Module Loading
```bash
# Return to project root
cd /home/greg/ResearchProcess-GPS

# Run WASM-specific tests
cargo test --package rp-modules wasm_module_test -- --nocapture

# Also run simple WASM test
cargo test --package rp-modules simple_wasm_test -- --nocapture
```

### 3. Debug Any Issues
If tests fail:
- Check WASM module exports/imports
- Verify host function bindings
- Ensure resource limits are reasonable
- Check for missing WASI imports

### 4. Create Module SDK (After WASM Tests Pass)
```bash
# Create new crate
cd crates
cargo new rp-module-sdk --lib

# Add to workspace
# Edit /home/greg/ResearchProcess-GPS/Cargo.toml
```

SDK should include:
- Module trait with derive macros
- Message passing helpers
- Capability checking utilities
- Resource limit helpers
- WASM-specific utilities

---

## 🔧 TECHNICAL CONTEXT

### Environment
- **Working Directory**: `/home/greg/ResearchProcess-GPS`
- **Database**: PostgreSQL on port 15432 (if needed)
- **Build Mode**: Use longer timeouts, single-threaded if needed

### Key Module Files
- Module system: `crates/rp-modules/src/`
- Native module: `modules/research-log/src/lib.rs`
- WASM module: `modules/research-log-wasm/src/lib.rs`
- Tests: `crates/rp-modules/tests/`

### Build Performance Expectations
```bash
# Normal compilation times:
cargo check: 30-60 seconds
cargo build: 3-5 minutes
cargo test: 5-10 minutes

# If builds seem slow:
export CARGO_BUILD_JOBS=1  # Single-threaded
```

---

## ⚠️ CRITICAL REMINDERS

### What IS Normal
- ✅ 5-10 minute build times
- ✅ High CPU during compilation
- ✅ Swap usage during builds
- ✅ Xorg CPU usage (display rendering)

### What is NOT a Problem
- ❌ "Resource exhaustion" - it's just compilation
- ❌ "Infinite loops" - tests run fine
- ❌ "Architecture flaws" - design is sound
- ❌ "Module system broken" - it works correctly

### NO_FALLBACK_POLICY
- ALWAYS handle errors explicitly
- NO silent failures
- NO workarounds
- Fix issues properly

---

## 📊 SUCCESS CRITERIA

You'll know the session is successful when:

1. **WASM Module Loads**: Tests pass showing WASM module can be loaded
2. **Communication Works**: Module can send/receive messages
3. **Resource Limits Enforced**: Limits are applied correctly
4. **SDK Started**: Basic structure for module SDK created

---

## 🎯 STRETCH GOALS

If time permits after main tasks:

1. **Module Documentation**
   - How to create a module
   - Module lifecycle
   - Security model
   - Best practices

2. **Helper Macros**
   - `#[derive(ResearchModule)]`
   - Message handling macros
   - Capability checking macros

3. **Additional Tests**
   - Error cases
   - Resource exhaustion
   - Multiple module instances

---

## 📚 REFERENCE COMMANDS

```bash
# Check system resources
free -h
ps aux | grep cargo

# Build with timeout
timeout 600 cargo build

# Run specific test
cargo test --package rp-modules test_name

# Check WASM module info
wasm-objdump -x target/wasm32-wasip1/release/research_log_wasm.wasm
```

---

*Ready to continue Phase 4 development. Focus on WASM module testing first, then SDK development.*