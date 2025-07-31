# ResearchProcess-GPS Next Session Prompt
### Generated: 2025-08-01 02:18:00 EEST

---

## 🚨 START HERE - CRITICAL CONTEXT

**MANDATORY READING**:
1. **Comprehensive Handover**: `COMPREHENSIVE_HANDOVER_2025_08_01_0217_EEST.md`
2. **Master Plan v2.5**: `RESEARCHPROCESS_GPS_MASTER_PLAN_v2.5_2025_08_01_0216_EEST.md`
3. **CLAUDE.md**: Project-specific instructions and NO_FALLBACK_POLICY

**Current Status**: NO_FALLBACK_POLICY 100% complete, NestableEntity updates pending

---

## 🎯 PRIORITY TASKS

### 1. Update 4 Entities to Implement NestableEntity (HIGH)

The following entities currently implement Entity directly but should implement NestableEntity:

1. **ProofStatement** (`crates/rp-core/src/proof_statement.rs`)
2. **ResearchLog** (`crates/rp-core/src/research_log.rs`)
3. **AnalysisReport** (`crates/rp-core/src/analysis_report.rs`)
4. **Note**: Workspace should remain Entity only (correct as-is)

**Implementation steps**:
```rust
// Change from:
impl Entity for ProofStatement { ... }

// To:
impl NestableEntity for ProofStatement { ... }
```

### 2. Update Documentation (MEDIUM)

After NestableEntity fixes:
- Update entity counts in all documentation
- Create NO_FALLBACK pattern guide
- Update API documentation for AnalysisReport

### 3. Complete Phase 3 (LOW)

- Implement OpenAPI/Swagger documentation with utoipa
- Optional: Add rate limiting with tower-governor

---

## 💻 ENVIRONMENT SETUP

```bash
# Database connection
export DB_NAME=researchprocess_gps
export DB_USER=researchprocess_gps
export DB_PASSWORD=researchprocess_gps
export DB_HOST=192.168.10.90
export DB_PORT=5432

# Run server
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps cargo run --bin rp-server

# Test API
curl -H "Authorization: Bearer test-token" \
     http://localhost:8080/api/v1/entities | jq .
```

---

## ⚠️ CRITICAL REMINDERS

1. **NO_FALLBACK_POLICY**: Maintain 100% compliance - we achieved it, don't break it!
2. **Entity naming**: Use AnalysisReport (not EvidenceAnalysis)
3. **Required fields**: Always return errors for missing data
4. **Test everything**: Run `cargo test` after changes

---

## 📊 PROJECT METRICS

- **Entities**: 19 implemented (100%)
- **NO_FALLBACK**: 100% compliant ✅
- **Phase 1**: 100% complete
- **Phase 2**: 100% complete
- **Phase 3**: 95% complete
- **Refactoring**: 80% complete

---

## 🔗 KEY REFERENCES

- **NO_FALLBACK_POLICY**: `/home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md`
- **Protocol Spec**: `docs/PROTOCOL_SPECIFICATION_v1.md`
- **Previous Session**: `COMPREHENSIVE_HANDOVER_2025_08_01_0136_EEST.md`

---

*Start by reading the comprehensive handover, then tackle NestableEntity updates.*