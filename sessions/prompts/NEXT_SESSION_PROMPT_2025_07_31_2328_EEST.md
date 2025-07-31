# Next Session Prompt - ResearchProcess-GPS
### Created: 2025-07-31 23:28:00 EEST

---

## 🚨 CRITICAL: READ THESE DOCUMENTS FIRST

1. **COMPREHENSIVE_HANDOVER_2025_07_31_2327_EEST.md** - Complete context from this session
2. **RESEARCHPROCESS_GPS_MASTER_PLAN_v2.3_2025_07_31_2327_EEST.md** - Current project state

---

## 📋 SESSION CONTEXT

You are continuing work on ResearchProcess-GPS, a genealogical research management system implementing standards as configuration, not code.

### Current Status
- **Phase 1**: ✅ 100% Complete - 22 entities implemented (evolved design)
- **Phase 2**: ✅ 100% Complete - Event sourcing with PostgreSQL NOTIFY
- **Phase 3**: 🔧 95% Complete - Only OpenAPI docs and rate limiting remain

### Key Architecture Points
- **Unified IdentityPersona**: No separate Person entity (intentional design)
- **Event Sourced**: Every change creates events
- **Standards as Data**: GPS/BCG are configurations
- **NO_FALLBACK_POLICY**: Zero tolerance for workarounds

### Technical Environment
- **Database**: PostgreSQL on 192.168.10.90 (NOT localhost)
- **API Port**: 8080 (NOT 3000)
- **Auth Token**: "test-token" for testing
- **Language**: Rust

---

## 🎯 SPECIFIC NEXT TASKS

### 1. Fix EntityType Enum (High Priority)
**File**: `crates/rp-core/src/layer3/mod.rs`
- Currently has 13 entries, needs 22
- Add: Analysis, Confidence, EvidenceAnalysis, ResearchSession, ResearchActivity
- Add: Workspace, MethodologyConfig, ModuleConfig, StandardsRegistry
- Add: TemplateRegistry, ValidationRule
- Remove: Repository (it's a SourceType, not entity)

### 2. Complete Phase 3 (Medium Priority)
- Add OpenAPI documentation using utoipa
- Implement rate limiting with tower-governor (optional)

### 3. Begin Phase 4: Query DSL (High Priority)
- Design ResearchQL syntax
- Choose parser (pest or nom)
- Read ULTRATHINK plan for Phase 4 requirements

---

## 🔧 QUICK START COMMANDS

```bash
# Switch to project
psw research

# Set up database environment
export DB_NAME=researchprocess_gps
export DB_USER=researchprocess_gps
export DB_PASSWORD=researchprocess_gps
export DB_GREEN=192.168.10.90

# Run server
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run --bin rp-server

# Quick test
curl -H "Authorization: Bearer test-token" \
     http://localhost:8080/api/v1/entities | jq .
```

---

## 📚 KEY DOCUMENTS

- **Master Plan**: RESEARCHPROCESS_GPS_MASTER_PLAN_v2.3_2025_07_31_2327_EEST.md
- **Conceptual Model**: docs/concepts/UNIFIED_CONCEPTUAL_MODEL_ALIGNED_2025_07_31_2243_EEST.md
- **Protocol Spec**: docs/PROTOCOL_SPECIFICATION_v1.md
- **Phase 3 Tasks**: docs/concepts/PHASE3_REMAINING_TASKS_2025_07_31_2229_EEST.md

---

## ⚠️ IMPORTANT REMINDERS

1. **Person Entity**: Merged into IdentityPersona - don't create separate Person
2. **API Endpoints**: /api/v1/persons/* work with IdentityPersona
3. **Clean Root**: Only 5 files should be in root directory
4. **Living Document**: Update Master Plan with any changes
5. **NO_FALLBACK_POLICY**: No workarounds, handle all errors explicitly

---

Begin by reading the comprehensive handover, then tackle the EntityType enum fix as the first priority.