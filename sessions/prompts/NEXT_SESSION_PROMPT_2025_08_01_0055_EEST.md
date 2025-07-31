# Next Session Prompt - ResearchProcess-GPS
## Architectural Refactoring Focus
### Generated: 2025-08-01 00:55 EEST

---

I need to continue work on the ResearchProcess-GPS project.

**CRITICAL: You MUST start by reading these documents in order:**
1. `COMPREHENSIVE_HANDOVER_2025_07_31_2327_EEST.md`
2. `RESEARCHPROCESS_GPS_MASTER_PLAN_v2.4_2025_08_01_0043_EEST.md` (See Addendum A)
3. `ARCHITECTURAL_REFACTORING_PLAN_2025_08_01_0043_EEST.md`

## 🚨 URGENT: Architectural Refactoring Required

**Current Status**: Major architectural inconsistencies discovered that block all other work:
- Entity count is 19 (not 22) + 5 ConfigEntities
- EntityType enum incomplete (13 entries, needs 19)
- NO_FALLBACK_POLICY violations in entity_type_mapper
- Layer organization needs implementation

## Priority Tasks (In Order):

### 1. Fix EntityType Enum
- **File**: `crates/rp-core/src/layer3/mod.rs`
- Add missing 7 entities: Analysis, Confidence, EvidenceAnalysis, ResearchSession, ResearchActivity, Location, Workspace
- Remove Repository (it's a SourceType, not an entity)
- Result: 19 entries matching all Entity implementations

### 2. Fix entity_type_mapper.rs
- **File**: `crates/rp-server/src/entity_type_mapper.rs`
- Remove ALL fallback behavior (NO_FALLBACK_POLICY violation)
- Fix incorrect mappings: "analysis" and "workspace"
- Return explicit errors for unknown types

### 3. Test and Verify
- Ensure all API endpoints work with corrected enum
- Verify no breaking changes to existing functionality
- Check that unknown entity types properly return errors

### 4. Consider Renaming (if time permits)
- EvidenceAnalysis → AnalysisReport (it's a WorkProduct)

## Technical Context:
- **Database**: PostgreSQL on 192.168.10.90
- **API Port**: 8080
- **Test token**: "test-token"
- **NO_FALLBACK_POLICY**: Zero tolerance for workarounds

## Layer Organization (Corrected):
- **Layer 1**: Core Data (9 entities) - Source, Citation, Evidence, IdentityPersona, Relationship, Location, Fact, Confidence, Analysis
- **Layer 2**: Process & Products (9 entities) - Theory, ResearchSession, ResearchActivity, ResearchLog, Researcher, WorkProduct, ProofStatement, EvidenceAnalysis
- **Layer 3**: Config (1 entity + 5 ConfigEntities) - Workspace + configs

## Remember:
- Theory is Layer 2 (process orchestrator), not Layer 1
- Entity vs ConfigEntity are different traits
- This refactoring is Priority #1 - no other work until complete

The architectural refactoring plan has all the details you need to proceed.