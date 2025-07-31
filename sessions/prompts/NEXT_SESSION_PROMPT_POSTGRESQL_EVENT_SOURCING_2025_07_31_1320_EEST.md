# ResearchProcess-GPS: PostgreSQL Event Sourcing Implementation Session

## 🎯 Session Goal
Implement the Event Sourcing layer on top of the completed PostgreSQL CRUD foundation for the ResearchProcess-GPS project.

## 📚 CRITICAL: Read These Documents First (IN ORDER)

### 1. **Master Architecture Document**
```bash
cd ~/ResearchProcess-GPS
bat ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md
```
Pay special attention to:
- Line 17: Core Architecture overview
- Line 209: Storage Abstraction Layer design
- Line 401: Development Phases (we're entering Phase 2)
- Event Sourcing architecture section

### 2. **Conceptual Model**
```bash
bat engine/UNIFIED_CONCEPTUAL_MODEL_2025_07_30_2000.md
```
Focus on:
- Entity specifications and relationships
- State machine definitions
- Event patterns for each entity type

### 3. **Latest Handover Document**
```bash
bat COMPREHENSIVE_HANDOVER_2025_07_31_1310_EEST.md
```
This contains:
- Current project status (CRUD complete)
- Database infrastructure details
- Technical patterns established
- Next phase planning (lines 167-199)

### 4. **PostgreSQL Implementation Summary**
```bash
bat POSTGRESQL_CRUD_IMPLEMENTATION_SUMMARY_2025_07_31_1305_EEST.md
```
Critical technical details:
- JSONB architecture (lines 77-100)
- Connection patterns (lines 106-127)
- Existing schema ready for event sourcing

## 🏗️ Current State Summary

### ✅ Completed
- All 23 core entities implemented
- PostgreSQL storage adapter with full CRUD
- Database migrations applied (including trigger fixes)
- Comprehensive test suite passing
- JSONB-based flexible storage
- Change data capture via triggers

### 🎯 This Session Focus
Implement Event Sourcing layer:
1. Enhanced event store table structure
2. Domain event definitions for all entities
3. Event publishing from mutations
4. Basic projection handlers
5. Event replay capability

## 🔧 Technical Context

### Database Connection
```bash
# PostgreSQL server details
Host: 192.168.10.90
Port: 5432
Database: researchprocess_gps
User: researchprocess_gps
Password: researchprocess_gps

# Environment override pattern (IMPORTANT!)
env DB_NAME=researchprocess_gps DB_USER=researchprocess_gps \
    DB_PASSWORD=researchprocess_gps ./command
```

### Code Locations
```
/crates/rp-storage-postgres/  # PostgreSQL implementation
/crates/rp-storage/          # Storage traits
/crates/rp-core/             # Entity definitions
/schemas/postgres/           # Database schema
```

## 📋 Immediate Tasks

### 1. Create Event Store Schema
Design robust event table beyond the existing `changes` table:
- Aggregate tracking
- Event versioning
- Event metadata
- Efficient indexes

### 2. Define Domain Events
Create event types for all 23 entities:
- TheoryCreated, TheoryUpdated, TheoryStateChanged
- EvidenceAdded, EvidenceQualityChanged
- ... for all entities

### 3. Implement Event Publishing
Modify existing CRUD to publish events:
- Keep CRUD as write model
- Publish events on mutations
- Maintain consistency

### 4. Create First Projections
Start with simple read models:
- Entity count projection
- State distribution projection
- Recent activity projection

## ⚠️ Critical Reminders

1. **NO FALLBACK POLICY**: Fix all errors before proceeding
2. **JSONB First**: Leverage PostgreSQL's JSONB capabilities
3. **Test Everything**: Integration tests against real database
4. **Read Docs First**: Don't skip the documentation reading

## 🚀 Start Commands

```bash
# 1. Switch to project
cd ~/ResearchProcess-GPS

# 2. Read all documentation (MANDATORY)
bat ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md
bat engine/UNIFIED_CONCEPTUAL_MODEL_2025_07_30_2000.md
bat COMPREHENSIVE_HANDOVER_2025_07_31_1310_EEST.md
bat POSTGRESQL_CRUD_IMPLEMENTATION_SUMMARY_2025_07_31_1305_EEST.md

# 3. Review current implementation
fd . crates/rp-storage-postgres/src/ -e rs -x bat {}

# 4. Check database state
env DB_NAME=researchprocess_gps psql -h 192.168.10.90 -U researchprocess_gps

# 5. Run existing tests to ensure baseline
cargo test --package rp-storage-postgres
```

## 📊 Success Metrics

By end of session:
- [ ] Event store schema designed and migrated
- [ ] Core domain events defined
- [ ] Event publishing integrated with CRUD
- [ ] At least one projection implemented
- [ ] Tests for event sourcing layer
- [ ] Documentation updated

---

**Remember**: The foundation is solid. The CRUD layer is complete and tested. Now we're adding event sourcing on top without breaking existing functionality. Start by reading ALL documentation to understand the full context before making any changes.

*Generated: 2025-07-31 13:20:00 EEST*
*Purpose: Enable event sourcing implementation with full project context*