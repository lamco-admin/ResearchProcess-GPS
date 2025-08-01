# NEXT SESSION PROMPT - Fix Native Module FFI Architecture

## MANDATORY INITIAL STEPS:
1. Read the comprehensive handover:
   ```
   cat COMPREHENSIVE_HANDOVER_2025_08_01_1717_EEST.md
   ```
2. Read the updated master plan:
   ```
   cat RESEARCHPROCESS_GPS_MASTER_PLAN_v3.6_2025_08_01_1717_EEST.md
   ```

## SESSION OBJECTIVES: Fix Native Module FFI Architecture

### Current Status: Module system 90% complete
- WASM modules: ✅ Fully working
- Native modules: ❌ SEGFAULT on initialization due to FFI issues
- Module SDK: ✅ API aligned and compiling

### Critical Issue: Native Module FFI
The current FFI design passes trait objects (`dyn ResearchModule`) through C boundaries, which is fundamentally unsafe. Double-boxing creates complex pointer indirection that causes segfaults.

### Priority Tasks:
1. **Research FFI Best Practices**:
   - Look at stable plugin architectures (cargo, neovim, etc.)
   - Understand C ABI limitations with Rust trait objects
   - Find proven patterns for Rust plugin systems

2. **Design New Architecture** - Choose one:
   - **Option A**: C-style vtable with function pointers
   - **Option B**: Message passing with serialization
   - **Option C**: Stable C API with opaque handles

3. **Implement Minimal Test**:
   - Create simplest possible native module
   - Test FFI boundary thoroughly
   - Use memory sanitizers to catch issues

4. **Fix Research Log Module**:
   - Update to use new FFI design
   - Ensure all tests pass
   - Document the pattern for other modules

### Key Context:
- Build timeout: 600000ms (10 minutes) 
- Use `CARGO_BUILD_JOBS=1` for stability
- Current directory: `/home/greg/ResearchProcess-GPS`
- NO_FALLBACK_POLICY: Do NOT switch to WASM to avoid the problem!

### Test Commands:
```bash
# Test native modules
CARGO_BUILD_JOBS=1 cargo test --package rp-modules --test native_module_test

# Build with sanitizers
RUSTFLAGS="-Z sanitizer=address" cargo +nightly test
```

### Architecture Ideas to Explore:
```rust
// Option A: Function pointer table
#[repr(C)]
pub struct ModuleApi {
    version: u32,
    create: extern "C" fn() -> *mut c_void,
    destroy: extern "C" fn(*mut c_void),
    initialize: extern "C" fn(*mut c_void, config: *const c_char) -> i32,
    execute: extern "C" fn(*mut c_void, cmd: *const c_char, args: *const c_char) -> *mut c_char,
}

// Option B: Message passing
extern "C" fn module_send_message(msg_type: u32, payload: *const u8, len: usize) -> i32;
extern "C" fn module_recv_message(buffer: *mut u8, len: usize) -> i32;
```

Remember: We MUST fix native modules properly. No fallbacks, no workarounds!