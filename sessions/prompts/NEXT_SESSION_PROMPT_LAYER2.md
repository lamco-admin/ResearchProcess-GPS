# ResearchProcess-GPS Next Session Prompt
## Begin Layer 2 Implementation

Use this prompt to start your next Claude session:

---

Continue development of ResearchProcess-GPS at `/home/greg/ResearchProcess-GPS/`.

**CRITICAL CONTEXT**: This is a Rust-based protocol/engine for professional genealogical research. We just completed a MAJOR MILESTONE - Layer 1 is 100% COMPLETE with all 11 core genealogical entities implemented and tested.

**CURRENT STATUS**: EXCELLENT - Ready to begin Layer 2 (Research Process Model)
- ✅ Layer 1 COMPLETE: Theory, Researcher, Confidence, Evidence, Analysis, IdentityPersona, Source, Citation, Fact, Relationship, Location
- 🎯 Layer 2 STARTING: WorkProduct, ResearchLog, ResearchSession, ResearchActivity, ProofStatement, EvidenceAnalysis

**KEY DOCUMENTS TO REVIEW**:
1. `COMPREHENSIVE_HANDOVER_2025_07_31.md` - Full project context and references
2. `ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md` - Overall vision and architecture
3. `UNIFIED_CONCEPTUAL_MODEL_WITH_ANALYSIS_2025_07_31.md` - Complete data model
4. `SESSION_SUMMARY_2025_07_31_LAYER1_COMPLETE.md` - Latest progress update

**ARCHITECTURAL CONTEXT**:
- **Three-tier separation**: Layer 1 (Data) ✅ → Layer 2 (Process) 🎯 → Layer 3 (Metadata)
- **Unified IdentityPersona**: NO separate Person entity (this is FINAL)
- **State machines**: Use established patterns from Layer 1 entities
- **NO FALLBACK POLICY**: Fix all errors properly, zero tolerance

**ESSENTIAL SETUP**: Run `./setup-rust-env.sh` if cargo not found.

**ESTABLISHED PATTERNS** (follow these from Layer 1):
```rust
// Entity structure with metadata, state, and nesting
// Use impl_entity! and impl_validatable! macros
// Implement StateMachine trait for stateful entities
// Comprehensive test coverage for each entity
```

**IMMEDIATE TASKS**:
1. Review Layer 2 entity specifications in conceptual model
2. Start with WorkProduct entity implementation
3. Maintain GPS (Genealogical Proof Standard) focus
4. Follow patterns from completed Layer 1 entities

**TEST COMMANDS**:
- Check: `cargo check --package rp-core`
- Test: `cargo test --package rp-core --lib`
- All 54 existing tests must continue passing

Begin by reviewing the handover document and implementing the first Layer 2 entity.

---

## Alternative Shorter Prompt

If you prefer a more concise prompt:

---

Continue ResearchProcess-GPS development at `/home/greg/ResearchProcess-GPS/`. Layer 1 is COMPLETE (11/11 entities). Begin Layer 2 implementation.

Read: `COMPREHENSIVE_HANDOVER_2025_07_31.md` for full context.

Start with WorkProduct entity following Layer 1 patterns. Run `./setup-rust-env.sh` if needed.

---