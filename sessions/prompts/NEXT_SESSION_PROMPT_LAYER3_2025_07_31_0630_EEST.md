# Next Session Prompt - ResearchProcess-GPS Layer 3 Continuation
## Generated: 2025-07-31 06:30:00 EEST

---

## 🚀 PROMPT FOR NEXT SESSION

Continue development of ResearchProcess-GPS Layer 3 (Workspace & Metadata Model) at `/home/greg/ResearchProcess-GPS/`.

**CRITICAL READING ORDER** - Read these documents in this exact sequence:

1. **`COMPREHENSIVE_SESSION_HANDOVER_2025_07_31_0629_EEST.md`**
   - Complete session context and Layer 3 progress (50% complete)
   - 3/6 Layer 3 entities implemented and tested

2. **`LAYER_3_IMPLEMENTATION_GUIDE_2025_07_31.md`**
   - PRIMARY REFERENCE for Layer 3 architecture
   - Explains "Standards as Data" philosophy
   - Contains entity specifications and patterns

3. **`LAYER_3_WORKSPACE_ANALYSIS_2025_07_31.md`**
   - Critical analysis of Layer 3's unique patterns
   - Explains why Layer 3 differs from Layers 1 & 2
   - Contains implementation issue resolutions

**CURRENT STATUS**:
- Layer 1: 100% Complete (11/11 entities)
- Layer 2: 100% Complete (6/6 entities)
- Layer 3: 50% Complete (3/6 entities)
- Total: 20/23 entities (87%)
- All 127 tests passing

**COMPLETED LAYER 3 ENTITIES**:
1. ✅ Workspace (615 lines) - Environment controller
2. ✅ MethodologyConfig (559 lines) - GPS/BCG as data
3. ✅ StandardsRegistry (802 lines) - Standards discovery

**PENDING LAYER 3 ENTITIES**:
4. ⏳ ModuleConfig - Pluggable tool modules
5. ⏳ TemplateRegistry - Work product templates
6. ⏳ ValidationRule - Configurable validation

**KEY ARCHITECTURAL PRINCIPLES**:
- NO state machines in Layer 3 (configuration != domain state)
- Custom metadata per entity type (not standard EntityMetadata)
- Configuration as data philosophy
- Type aliases for clarity (WorkspaceId = EntityId)
- ConfigEntity trait (not Entity trait)
- No enforcement logic (modules enforce, Layer 3 stores)

**NEXT STEPS**:
1. Implement remaining 3 Layer 3 entities
2. Follow patterns from completed entities
3. Run tests: `cargo test --package rp-core --lib layer3`
4. Prepare for PostgreSQL schema design
5. Plan module system architecture

**REMEMBER**: Layer 3 is fundamentally different - it's configuration infrastructure that makes methodologies like GPS become data files rather than code.

---

## 📋 Quick Reference Commands

```bash
# Check compilation
cargo check --package rp-core

# Run Layer 3 tests
cargo test --package rp-core --lib layer3

# Run all tests
cargo test --package rp-core --lib

# Review Layer 3 implementation
bat crates/rp-core/src/layer3/mod.rs
bat crates/rp-core/src/layer3/workspace.rs
bat crates/rp-core/src/layer3/methodology_config.rs
bat crates/rp-core/src/layer3/standards_registry.rs
```

---

*Use this prompt to continue Layer 3 implementation with full context.*