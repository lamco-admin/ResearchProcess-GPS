# NEXT SESSION PROMPT - ResearchProcess-GPS

## 🚨 MANDATORY INITIAL STEPS

1. Read comprehensive handover:
   ```bash
   cat COMPREHENSIVE_HANDOVER_2025_08_01_1956_EEST.md
   ```

2. Read updated master plan:
   ```bash
   cat RESEARCHPROCESS_GPS_MASTER_PLAN_v4.0_2025_08_01_1956_EEST.md
   ```

## SESSION CONTEXT

**Project**: ResearchProcess-GPS - Research Process Management System
**Status**: Module System COMPLETE (Phase 4: 100%)
**Achievement**: All 21 tests passing, production-ready module architecture

## 🎯 SESSION OPTIONS

### Option 1: Begin Phase 5 - Collaboration Features
- Implement CRDT for real-time collaboration
- Design conflict resolution strategies
- Create collaboration protocol
- Enhance workspace management

### Option 2: Begin Phase 6 - Web Interface
- Choose web framework (Leptos/Yew/Dioxus)
- Design UI/UX system
- Implement workspace management UI
- Create entity browsers

### Option 3: Advanced Module Development
- Build GPS-specific analysis modules
- Create research methodology modules
- Implement specialized tools

## 🔧 CRITICAL REMINDERS

### Build Requirements
```bash
# MANDATORY for ALL cargo commands
export CARGO_BUILD_JOBS=1
```

### Database Connection
- PostgreSQL must be running
- Connection: `postgresql://postgres:postgres@localhost/researchprocess_gps`

### Key Context
- Module system uses message-based FFI (NOT trait objects)
- SDK provides `ffi_module!` macro for easy development
- All tests currently passing (maintain this!)
- NO_FALLBACK_POLICY: Zero tolerance for silent failures

## 📁 PROJECT STRUCTURE

```
ResearchProcess-GPS/
├── crates/
│   ├── rp-modules/        # Module system (COMPLETE)
│   ├── rp-module-sdk/     # SDK with message builders
│   └── ...
├── modules/
│   ├── research-log/      # Native module example
│   ├── research-log-wasm/ # WASM module example
│   └── example-module/    # New example with SDK
└── docs/
    └── development/
        ├── MODULE_DEVELOPMENT_GUIDE.md
        └── MODULE_FFI_QUICK_REFERENCE.md
```

## 🚀 QUICK START COMMANDS

```bash
# Switch to project
psw research

# Run all tests
export CARGO_BUILD_JOBS=1 && cargo test

# Build everything
export CARGO_BUILD_JOBS=1 && cargo build --release

# Start development
claude
```

## 💡 RECENT ACCOMPLISHMENTS

- ✅ Completed ModuleLoader FFI refactoring
- ✅ Fixed all thread safety issues with AtomicPtr
- ✅ Created comprehensive Module SDK
- ✅ Built example module with new architecture
- ✅ Wrote complete documentation

**The module system is production-ready. Choose your next major feature!**