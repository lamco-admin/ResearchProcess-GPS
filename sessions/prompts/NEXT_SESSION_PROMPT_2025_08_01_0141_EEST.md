# ResearchProcess-GPS Next Session Prompt
## Critical NO_FALLBACK_POLICY Enforcement - Phase 2
### Timestamp: 2025-08-01 01:41:11 EEST

---

## 🚨 CRITICAL CONTEXT - START HERE

**Previous Session**: Major architectural refactoring completed ~60% of NO_FALLBACK_POLICY enforcement
**Commit**: bb0a579 - Fixed EntityType enum, renamed EvidenceAnalysis, partial NO_FALLBACK fixes
**Handover**: COMPREHENSIVE_HANDOVER_2025_08_01_0136_EEST.md

### READ THESE DOCUMENTS FIRST:
1. `COMPREHENSIVE_HANDOVER_2025_08_01_0136_EEST.md` - Complete session context
2. `NO_FALLBACK_POLICY_VIOLATIONS_2025_08_01_0140_EEST.md` - All violations found
3. `NO_FALLBACK_FIXES_SUMMARY_2025_08_01_0200_EEST.md` - What was fixed, what remains

---

## 🎯 TODAY'S PRIORITIES

### 🔴 CRITICAL: Complete NO_FALLBACK_POLICY Fixes (Priority #1)

The previous session fixed ~60% of violations. Your IMMEDIATE task is to complete the remaining 40%.

**Start with store_runtime.rs** - Multiple critical violations remain:
```rust
// Line 162-163: Version fallbacks
let from_version = from_version.unwrap_or(1);
let to_version = to_version.unwrap_or(i64::MAX);

// Line 223: Current version fallback
let current_version = events.last().map(|e| e.aggregate_version).unwrap_or(0);

// Line 239-240: Pagination defaults
let limit = limit.unwrap_or(100);
let offset = offset.unwrap_or(0);

// Line 307: Aggregate version fallback
Ok(result.0.unwrap_or(0))

// Line 342: Max version fallback
let max_version = max_version.unwrap_or(i64::MAX);
```

**Then continue with**:
- Protocol layer violations (filters.rs, etc.)
- Query builder pagination defaults
- Any remaining `unwrap_or` patterns

### ⚠️ HIGH: Complete Architectural Refactoring

After NO_FALLBACK fixes are complete:

1. **Update 4 entities to implement NestableEntity**:
   - ProofStatement (currently implements Entity directly)
   - ResearchLog (currently implements Entity directly)  
   - AnalysisReport (formerly EvidenceAnalysis, currently implements Entity directly)
   - Note: Workspace should remain Entity only

2. **Update all documentation** to reflect changes

### 📋 MEDIUM: Complete Phase 3
- Implement OpenAPI/Swagger docs with utoipa
- Add rate limiting with tower-governor (optional)

---

## 💡 KEY TECHNICAL CONTEXT

### NO_FALLBACK_POLICY Rules
1. **No silent failures** - Every error must be handled explicitly
2. **No degraded operation** - Fail fast rather than continue with bad data
3. **No assumptions** - Missing data is an error, not a default
4. **Exceptions**: PostgreSQL port 5432, empty passwords are OK

### Entity Architecture
- **19 entities total** (not 22 as originally documented)
- **EntityType enum**: Now correctly has all 19 entries
- **AnalysisReport**: New name for EvidenceAnalysis
- **NestableEntity**: Should be implemented by all entities except Workspace

### What Works Well
- EntityType enum is now complete and correct
- Entity renaming is complete
- Critical event handling no longer uses fallbacks
- Database configuration fails fast on bad URLs

---

## 🛠️ DEVELOPMENT SETUP

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

# Find remaining violations
cargo clippy --all-targets -- -W clippy::all | grep unwrap_or
rg "unwrap_or" --type rust
```

---

## ⚡ QUICK WINS

1. **Constants for acceptable defaults**:
   ```rust
   const DEFAULT_PAGE_SIZE: i32 = 100;
   const DEFAULT_OFFSET: i32 = 0;
   const MAX_VERSION: i64 = i64::MAX;
   ```

2. **Make parameters required where possible**:
   ```rust
   // Instead of Option<i32> for limit/offset
   // Use required parameters with sensible defaults at API layer
   ```

3. **Use Result for uncertain operations**:
   ```rust
   // Instead of: events.last().map(|e| e.aggregate_version).unwrap_or(0)
   // Use: events.last().map(|e| e.aggregate_version)
   //      .ok_or_else(|| EventError::EmptyEventStream)?
   ```

---

## 📚 REFERENCE

- **NO_FALLBACK_POLICY**: `/home/greg/ai-tools/docs/standards/NO_FALLBACK_POLICY.md`
- **Master Plan**: `RESEARCHPROCESS_GPS_MASTER_PLAN_v2.4_2025_08_01_0043_EEST.md`
- **API Protocol**: `docs/PROTOCOL_SPECIFICATION_v1.md`

---

## 🎯 SUCCESS CRITERIA

1. **Zero `unwrap_or` patterns** in production code
2. **All 4 entities** updated to implement NestableEntity
3. **All tests passing** with new error handling
4. **Documentation updated** to reflect all changes

---

*Start with store_runtime.rs fixes. Every violation fixed makes the system more reliable.*