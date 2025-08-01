# Next Session Prompt for ResearchProcess-GPS
### Timestamp: 2025-08-01 02:56:00 EEST

---

## 🚨 START HERE - CRITICAL CONTEXT

**MANDATORY READING**:
1. **Comprehensive Handover**: `COMPREHENSIVE_HANDOVER_2025_08_01_0254_EEST.md`
2. **Master Plan v2.7**: `RESEARCHPROCESS_GPS_MASTER_PLAN_v2.7_2025_08_01_0253_EEST.md`
3. **CLAUDE.md**: Project-specific instructions and NO_FALLBACK_POLICY

**Current Status**: 
- NestableEntity updates 100% complete
- Entity count corrected to 18 (Note never existed)
- OpenAPI implementation started but requires comprehensive work
- NO_FALLBACK_POLICY maintained at 100%

---

## 🎯 PRIORITY TASKS

### 1. Complete OpenAPI Implementation (HIGH)

The OpenAPI setup is partially done but needs comprehensive annotation work:

**Create missing types** in `rp-protocol/src/requests.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BranchTheoryRequest {
    // Check handlers/theories.rs for required fields
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MergePersonsRequest {
    // Check handlers/persons.rs for required fields
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InviteMemberRequest {
    // Check handlers/workspaces.rs for required fields
}
```

**Annotate ALL handlers** - Example pattern:
```rust
#[utoipa::path(
    post,
    path = "/api/v1/entities",
    request_body = CreateEntityRequest,
    responses(
        (status = 201, description = "Entity created", body = EntityResponse),
        (status = 400, description = "Invalid request"),
    ),
    tag = "entities"
)]
pub async fn create_entity(...) { ... }
```

**Add ToSchema derives** to all types referenced in API

### 2. Restore Phase 4-8 Details (MEDIUM)

After OpenAPI is working:
- Read `archive/deprecated/plans/ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md`
- Extract Phase 4-8 detailed descriptions
- Update Master Plan with full phase information

### 3. Update Documentation (MEDIUM)

- Remove all references to 19 entities (should be 18)
- Document the OpenAPI endpoints once working

---

## 💻 ENVIRONMENT SETUP

```bash
# Database connection
export DB_NAME=researchprocess_gps
export DB_USER=researchprocess_gps
export DB_PASSWORD=researchprocess_gps
export DB_HOST=192.168.10.90
export DB_PORT=5432

# Run server (currently won't compile - fix OpenAPI first)
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run --bin rp-server

# Once working, Swagger UI at:
http://localhost:8080/swagger-ui
```

---

## ⚠️ CRITICAL REMINDERS

1. **NO_FALLBACK_POLICY**: No shortcuts! Complete OpenAPI implementation required
2. **Entity count**: 18 entities, not 19 - Note is not an entity
3. **Compilation first**: Fix all utoipa compilation errors before testing
4. **Comprehensive approach**: Every handler needs annotation

---

## 📊 PROJECT METRICS

- Entities: 18 implemented (100%)
- NO_FALLBACK: 100% compliant ✅
- NestableEntity: 100% complete ✅
- Phase 1: 100% complete
- Phase 2: 100% complete
- Phase 3: 96% complete (OpenAPI remaining)
- Refactoring: 100% complete

---

## 🔗 KEY REFERENCES

- NO_FALLBACK_POLICY: /home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md
- Pattern Guide: docs/patterns/NO_FALLBACK_PATTERN_GUIDE_2025_08_01_0230_EEST.md
- Phase Details: archive/deprecated/plans/ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md
- Previous Session: COMPREHENSIVE_HANDOVER_2025_08_01_0254_EEST.md

---

Start by reading the comprehensive handover, then tackle the OpenAPI annotations systematically.