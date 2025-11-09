# Koha Integration Exploration: Comprehensive Handover

**Document Created**: 2025-11-09 15:35:00 UTC
**Session**: Phase 1 (Weeks 1-2) - Koha Authority Control Research
**Branch**: `claude/repository-setup-011CUxVk9NQfSeNoJUrhDTH4`
**Next Branch**: `claude/koha-integration-exploration` (to be created)

---

## Executive Summary

This handover documents the completion of **Phase 1, Week 2** research into Koha's authority control architecture for potential integration with ResearchProcess-GPS. The investigation has revealed that Koha's 25-year-old authority control system provides excellent architectural patterns for implementing canonical person/place records in genealogical research management.

### Research Completed

✅ **Week 1**: Strategic analysis of Koha for genealogy (KOHA_GENEALOGY_STRATEGIC_ANALYSIS.md - already existed)
✅ **Week 2**: Deep technical analysis of authority control architecture
✅ **Week 2**: Complete Person entity design specification for RP-GPS
✅ **Week 2**: Architectural pattern extraction and adaptation

### Deliverables

1. **KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md** (13,500+ words)
   - Complete technical analysis of Koha's authority control system
   - Database schema documentation (auth_header, auth_types, borrower_relationships)
   - MARC21 authority format breakdown
   - Code architecture analysis (C4::AuthoritiesMarc, Koha::Authority)
   - Authority linking system (C4::Linker)
   - Search architecture (Elasticsearch + EmbedSeeFromHeadings)
   - Architectural patterns for RP-GPS adaptation

2. **RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md** (15,000+ words)
   - Complete Person entity specification
   - Database schemas (persons, person_variant_names, person_relationships, source_persons)
   - Rust data structures with full implementations
   - Merge operations design
   - Search & query strategies
   - Elasticsearch index mapping
   - REST API specification
   - Testing strategy
   - 12-week implementation plan

### Key Strategic Recommendation

**✅ CONFIRMED: Option 4 + Elements of Option 3**

- Study Koha's authority patterns (DONE ✅)
- Enhance RP-GPS with authority patterns (READY FOR IMPLEMENTATION)
- Build MARC adapter for interoperability (SPECIFICATION NEEDED)
- Optional: Koha plugin if demand exists (FUTURE)

---

## Research Findings Summary

### 1. Koha Authority Control Architecture

**Core Strengths Identified**:

1. **Dual Storage Format** (marc + marcxml):
   - Binary MARC for original format preservation
   - XML MARC for modern tooling and XPath queries
   - **Lesson**: Maintain native format + export format for interoperability

2. **Variant Name Handling** (MARC 4XX fields):
   - Systematic storage of all name forms
   - Source attribution for each variant
   - Automatic search embedding (IncludeSeeFromInSearches)
   - **Lesson**: Make variants first-class citizens, not afterthoughts

3. **Authority Linking** (Subfield $9):
   - Bidirectional links: authorities ↔ bibliographic records
   - Automatic linking via C4::Linker module
   - Usage tracking (get_usage_count, linked_biblionumbers)
   - **Lesson**: Automate linking, don't burden users

4. **Relationship Modeling** (MARC 5XX + borrower_relationships):
   - Hierarchical authority trees (authtrees field)
   - Directional relationships with typed links
   - **Lesson**: Support both hierarchical and peer relationships

5. **Merge Operations**:
   - Progressive merging (immediate vs. deferred based on usage count)
   - Complete audit logging
   - Merge request queue for high-volume authorities
   - **Lesson**: Production systems always have duplicates - plan for merging from day 1

6. **Search with Authority Embedding**:
   - EmbedSeeFromHeadings filter embeds variants into search index
   - User searches "Johann Schmidt" → finds "John Smith" sources
   - Transparent to end users
   - **Lesson**: Index all variants for comprehensive search

### 2. MARC21 Authority Format Structure

**Personal Name Authority Record Anatomy**:

```
Field 100: Established Heading
  - Main authoritative form: "Smith, John, 1820-1891"

Fields 400: See From Tracing (Variants)
  - "Schmidt, Johann" (immigration name)
  - "Smith, J." (abbreviated)
  - "Smith, Johnny" (nickname)

Fields 500: See Also From Tracing (Relationships)
  - Spouse: "Smith, Mary, 1822-1895"
  - Father: "Smith, William, 1790-1870"
  - Mother: "Jones, Elizabeth, 1795-1875"

Fields 670: Source Data Found (Evidence Citations)
  - "1850 U.S. Census, Ohio (John Smith, age 30)"
  - "Death certificate, 1891 (d. March 15, 1891)"
```

**Key Insight**: MARC21 authority format maps remarkably well to genealogical needs:
- 100 = Canonical person name
- 4XX = Name variants
- 5XX = Family relationships
- 670 = Source citations

### 3. RP-GPS Person Entity Design

**Core Architecture**:

```
persons (canonical individuals)
  ├── person_variant_names (all name forms)
  ├── person_relationships (family structure)
  ├── source_persons (source mentions/citations)
  └── person_merges (duplicate consolidation log)
```

**Key Design Decisions**:

1. **Canonical vs. Mentions Separation**:
   - `persons` table = THE person (canonical)
   - `source_persons` table = mentions in sources
   - Clear linking: `source_persons.person_id → persons.person_id`

2. **Variant Names as First-Class Entities**:
   - Dedicated `person_variant_names` table
   - Type classification (birth, married, immigration, spelling, etc.)
   - Source attribution for each variant
   - Full-text search indexing

3. **Relationships with Certainty Tracking**:
   - Directional relationships (person_1 → person_2)
   - Typed (parent, child, spouse, sibling, etc.)
   - Certainty levels (1=Speculative → 5=Definite)
   - Source evidence required

4. **Merge Operations with Audit Trail**:
   - `person_merges` table logs all merges
   - Stores backup of merged person (JSON)
   - Supports reversal
   - Transfers all source mentions, variants, relationships

5. **Search Integration**:
   - PostgreSQL full-text search for basic queries
   - Elasticsearch for advanced search with variant embedding
   - Phonetic matching (Metaphone encoder)
   - Fuzzy matching for duplicate detection

---

## Implementation Roadmap

### Phase 1: Core Entity (Weeks 5-6) - NEXT IMMEDIATE STEPS

**Week 5 Tasks**:
- [ ] Create database migrations:
  - `persons` table
  - `person_variant_names` table
  - Triggers (updated_at, audit logging)
- [ ] Implement Rust structs:
  - `Person` with CRUD operations
  - `VariantName` with CRUD
  - `GenealogyDate` type
- [ ] Unit tests for all data structures
- [ ] Basic PostgreSQL queries

**Week 6 Tasks**:
- [ ] Create additional migrations:
  - `person_relationships` table
  - `source_persons` table
  - `person_merges` table
- [ ] Implement relationship operations
- [ ] Implement source linking
- [ ] Integration tests
- [ ] Database indices optimization

### Phase 2: Operations (Weeks 7-8)

**Week 7**:
- [ ] Search functions (name, year range, full-text)
- [ ] Elasticsearch index mapping
- [ ] Indexing pipeline
- [ ] Duplicate detection algorithm

**Week 8**:
- [ ] Person merge implementation
- [ ] Merge reversal
- [ ] Audit logging system
- [ ] Performance optimization

### Phase 3: API & UI (Weeks 9-10)

**Week 9**:
- [ ] REST API endpoints (see specification)
- [ ] OpenAPI/Swagger documentation
- [ ] API integration tests

**Week 10**:
- [ ] Web UI components
- [ ] Person detail view
- [ ] Relationship graph visualization
- [ ] Merge wizard

### Phase 4: MARC Integration (Weeks 11-12) - OPTIONAL

**Week 11**:
- [ ] MARC21 authority export (Person → MARC)
- [ ] MARC library integration (marc-rs crate)
- [ ] MARC caching system

**Week 12**:
- [ ] MARC21 authority import (MARC → Person)
- [ ] Round-trip testing
- [ ] Koha plugin prototype (if needed)

---

## Technical Specifications Ready

### Database Schemas

All table schemas are fully specified in **RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md**:

1. **persons** table (16 columns):
   - Core name components (given, surname, middle, prefix, suffix)
   - Vital events (birth/death dates and places)
   - Biographical data (sex, occupation, religion)
   - Research notes and conclusion confidence
   - MARC export cache
   - Metadata (created, updated, archived)

2. **person_variant_names** table (13 columns):
   - Name components
   - Variant type classification (14 types)
   - Source attribution
   - Language tracking

3. **person_relationships** table (10 columns):
   - Bidirectional relationships
   - 17 relationship types
   - Certainty tracking (1-5 scale)
   - Source evidence

4. **source_persons** table (16 columns):
   - Source mentions
   - Linking to canonical persons
   - Extracted data (age, occupation, etc.)
   - Page/line citations

5. **person_merges** table (10 columns):
   - Merge audit log
   - Backup data for reversal
   - Strategy tracking

### Rust Implementations

Complete Rust structs with methods:

- `Person` (50+ methods)
- `VariantName`
- `PersonRelationship` with bidirectional handling
- `SourcePerson` with linking operations
- `PersonMerge` with reversal capability
- `GenealogyDate` with uncertainty modeling
- `Confidence` enum (1-5 scale)
- `RelationshipType` enum (17 types)
- `VariantNameType` enum (14 types)

### API Specification

Complete REST API endpoints specified:
- `GET /api/v1/persons` - List with pagination
- `GET /api/v1/persons/:id` - Get single person
- `POST /api/v1/persons` - Create
- `PUT /api/v1/persons/:id` - Update
- `DELETE /api/v1/persons/:id` - Archive
- `GET /api/v1/persons/:id/variants` - Get variants
- `POST /api/v1/persons/:id/variants` - Add variant
- `GET /api/v1/persons/:id/relationships` - Get relationships
- `POST /api/v1/persons/:id/relationships` - Add relationship
- `GET /api/v1/persons/:id/sources` - Get linked sources
- `POST /api/v1/persons/:id/merge` - Merge persons

### Elasticsearch Mapping

Complete index mapping with:
- Name analyzers with phonetic matching
- Variant name embedding
- Date range queries
- Faceted search capabilities

---

## Architectural Patterns Applied

### Pattern 1: Canonical vs. Mentions

**Koha**: Authority record vs. Bibliographic field
**RP-GPS**: Person vs. SourcePerson
**Benefit**: Clear separation of "the truth" vs. "what sources say"

### Pattern 2: Variant Handling

**Koha**: MARC 4XX fields + EmbedSeeFromHeadings
**RP-GPS**: person_variant_names + Elasticsearch embedding
**Benefit**: Search "Johann Schmidt" → find "John Smith" automatically

### Pattern 3: Authority Linking

**Koha**: MARC $9 subfield
**RP-GPS**: source_persons.person_id
**Benefit**: Bidirectional navigation (person → sources, source → persons)

### Pattern 4: Relationship Modeling

**Koha**: MARC 5XX + authtrees + borrower_relationships
**RP-GPS**: person_relationships with typed links
**Benefit**: Complex family structures with certainty tracking

### Pattern 5: Merge Operations

**Koha**: merge() function with deferred processing
**RP-GPS**: Person::merge_from() with audit trail
**Benefit**: Production-grade duplicate handling

### Pattern 6: Search Embedding

**Koha**: EmbedSeeFromHeadings filter
**RP-GPS**: Elasticsearch variant_names field
**Benefit**: Comprehensive search without user knowledge of variants

---

## What We Did NOT Complete (Future Work)

### Week 3-4 Tasks (Deferred)

Due to time constraints, we completed Weeks 2 research early but did not fully complete Week 3-4 tasks:

**Week 3** (Originally Planned):
- [ ] Study Koha plugin lifecycle management in depth
- [ ] Analyze Elasticsearch integration architecture details
- [ ] Document specific plugin hooks and patterns

**Week 4** (Originally Planned):
- [ ] Create Person entity specification (**DONE** ✅)
- [ ] Design MARC adapter architecture (**PARTIALLY DONE** - basic patterns documented)
- [ ] Plan implementation phases (**DONE** ✅ - 12-week roadmap created)

**Recommendation**: Week 3 research (plugin system, Elasticsearch details) can be done on-demand during implementation if needed. The core Person entity design is complete and ready for implementation.

### MARC Adapter Specification

While we documented the MARC21 format and conversion patterns, a dedicated **MARC_ADAPTER_SPECIFICATION.md** document was not created. This should include:

- Detailed Person → MARC21 conversion algorithm
- MARC21 → Person parsing logic
- Field mapping tables (RP-GPS fields ↔ MARC fields)
- Round-trip conversion testing
- Rust implementation using `marc-rs` crate

**Priority**: MEDIUM (only needed if library integration is required)

### Koha Plugin Development

We did not create actual Koha plugin code. This was intentional - the strategy is to:

1. **First**: Implement Person entity in RP-GPS (Weeks 5-10)
2. **Second**: Add MARC export capability (Weeks 11-12)
3. **Third**: IF institutional demand exists, develop Koha plugin (future)

---

## Files Created This Session

### Main Documents

1. **KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md**
   - Location: `/home/user/ResearchProcess-GPS/`
   - Size: 13,500+ words
   - Status: ✅ Complete

2. **RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md**
   - Location: `/home/user/ResearchProcess-GPS/`
   - Size: 15,000+ words
   - Status: ✅ Complete

3. **KOHA_INTEGRATION_EXPLORATION_HANDOVER_2025_11_09_1535_UTC.md** (this document)
   - Location: `/home/user/ResearchProcess-GPS/`
   - Status: ✅ Complete

### Existing Reference Document

- **KOHA_GENEALOGY_STRATEGIC_ANALYSIS.md**
  - Location: Found in `claude/researchprocess-gps-exploration-011CUxKTKG62QrHVV1vmVTUa` branch
  - Already existed from previous session
  - Provides strategic analysis and 5 integration options

---

## Next Session Instructions

### Immediate Actions

1. **Review Documents**:
   - Read KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md
   - Read RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md
   - Review architectural patterns and design decisions

2. **Decision Point**: Choose implementation approach:
   - **Option A**: Begin Phase 1 implementation (create Person entity)
   - **Option B**: Create MARC adapter specification first
   - **Option C**: Continue research into plugin system / Elasticsearch

3. **Recommended**: Option A (Begin Implementation)
   - All specifications are ready
   - Week 5 tasks are well-defined
   - Can build MARC adapter later if needed

### Creating New Branch (As Recommended in Strategy Doc)

```bash
# Create the exploration branch
git checkout -b claude/koha-integration-exploration

# Add the new documents
git add KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md
git add RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md
git add KOHA_INTEGRATION_EXPLORATION_HANDOVER_2025_11_09_1535_UTC.md

# Commit
git commit -m "docs: Complete Phase 1 Koha authority control research

Deep technical analysis of Koha's 25-year-old authority control architecture
and complete Person entity design specification for ResearchProcess-GPS.

Key Deliverables:
- KOHA_AUTHORITY_ARCHITECTURE_STUDY (13,500 words)
  * Database schema analysis (auth_header, auth_types)
  * MARC21 authority format breakdown
  * Code architecture (C4::AuthoritiesMarc, Koha::Authority)
  * Authority linking system (C4::Linker)
  * Search architecture (Elasticsearch + EmbedSeeFromHeadings)
  * Architectural patterns for RP-GPS

- RP_GPS_PERSON_ENTITY_DESIGN (15,000 words)
  * Complete database schemas (5 tables)
  * Full Rust implementations with 50+ methods
  * Variant name handling
  * Relationship modeling with certainty
  * Merge operations with audit trail
  * REST API specification
  * Elasticsearch mapping
  * 12-week implementation plan

- KOHA_INTEGRATION_EXPLORATION_HANDOVER
  * Research summary and findings
  * Implementation roadmap
  * Next session instructions

Strategic Outcome:
Confirmed Option 4 approach - learn from Koha's authority patterns,
enhance RP-GPS with proven architectural patterns, build optional
MARC adapter for library interoperability.

Ready for Phase 1 implementation (Weeks 5-6): Core Person entity."

# Push to remote
git push -u origin claude/koha-integration-exploration
```

### Week 5 Implementation Tasks

If proceeding with implementation:

1. **Database Migrations**:
   ```sql
   -- Create persons table (see specification)
   -- Create person_variant_names table
   -- Create triggers (updated_at, audit logging)
   -- Create indexes
   ```

2. **Rust Structs**:
   ```rust
   // Implement Person struct
   // Implement VariantName struct
   // Implement GenealogyDate type
   // Implement Confidence enum
   ```

3. **CRUD Operations**:
   ```rust
   impl Person {
       async fn create(...) -> Result<Person>
       async fn find(...) -> Result<Person>
       async fn update(...) -> Result<Person>
       async fn archive(...) -> Result<()>
   }
   ```

4. **Tests**:
   ```rust
   #[test] fn test_canonical_name() { ... }
   #[test] fn test_life_span() { ... }
   #[tokio::test] async fn test_person_crud() { ... }
   ```

---

## Success Criteria Met

### Phase 1, Week 2 Goals ✅

- [x] Deep dive into Koha authority control architecture
- [x] Study authority linking patterns and implementation
- [x] Design RP-GPS Person entity
- [x] Document architectural patterns
- [x] Create implementation roadmap

### Deliverables Complete ✅

- [x] KOHA_AUTHORITY_ARCHITECTURE_STUDY.md
- [x] RP_GPS_PERSON_ENTITY_DESIGN.md
- [x] Comprehensive handover document
- [x] 12-week implementation plan

### Knowledge Gained ✅

1. **Technical Understanding**:
   - Koha database schema (auth_header, auth_types, borrower_relationships)
   - MARC21 authority format (1XX, 4XX, 5XX fields)
   - Authority control code architecture (C4::AuthoritiesMarc, Koha::Authority)
   - Linking system implementation (C4::Linker, subfield $9)
   - Search architecture (Elasticsearch + EmbedSeeFromHeadings)
   - Merge operations (merge(), usage tracking, deferred merges)

2. **Architectural Patterns**:
   - Canonical vs. mentions separation
   - Variant name handling as first-class
   - Authority linking with bidirectional navigation
   - Relationship modeling with certainty
   - Merge operations with audit trail
   - Search with variant embedding

3. **Design Decisions**:
   - Database schema for 5 core tables
   - Rust struct implementations
   - API endpoint specifications
   - Search strategy
   - Testing approach

---

## Risk Assessment

### Low Risk ✅

1. **Technical Feasibility**: All patterns proven in Koha (25 years production)
2. **Database Design**: Well-normalized, follows PostgreSQL best practices
3. **Rust Implementation**: Straightforward, using standard libraries
4. **Testing Strategy**: Comprehensive unit + integration tests

### Medium Risk ⚠️

1. **Duplicate Detection Algorithm**: Needs tuning for genealogical data
2. **Elasticsearch Performance**: May need optimization for large datasets
3. **MARC Conversion**: Complex if round-trip fidelity is required

### Mitigations

1. **Start Simple**: Implement basic Person CRUD first, add complexity incrementally
2. **Defer MARC**: Only implement if institutional integration is needed
3. **Test Early**: Integration tests from Week 6 onward
4. **User Feedback**: Validate merge UI/UX early with genealogists

---

## Questions for Next Session

1. **Implementation Priority**: Begin Person entity implementation or continue research?
2. **MARC Adapter**: Create detailed specification now or defer until needed?
3. **Plugin System**: Study Koha plugin patterns now or on-demand later?
4. **Branch Strategy**: Merge work into main or keep exploration branch separate?

---

## Conclusion

**Phase 1 Research: COMPLETE ✅**

The investigation into Koha's authority control architecture has been highly successful. We've identified proven patterns from 25 years of library science that map remarkably well to genealogical research needs. The complete Person entity design is ready for implementation, with full database schemas, Rust implementations, API specifications, and a 12-week roadmap.

**Strategic Decision: CONFIRMED ✅**

- Learn from Koha's authority patterns (DONE)
- Enhance RP-GPS with these patterns (READY)
- Build optional MARC adapter (SPEC DEFERRED)
- Optional Koha plugin if demand exists (FUTURE)

**Recommendation**: Proceed to **Phase 1 Implementation (Weeks 5-6)** - Create core Person entity with database migrations, Rust structs, CRUD operations, and comprehensive tests. The specifications are complete and ready to code.

---

**Session Status**: Research Phase Complete
**Next Phase**: Implementation
**Documents**: Ready for Git Commit
**Branch**: Ready to Push

**🤖 Generated with Claude Code**

---

## Appendix: Document Cross-References

### This Session's Documents

1. **KOHA_AUTHORITY_ARCHITECTURE_STUDY_2025_11_09_1535_UTC.md**
   - Part 1: Database Architecture
   - Part 2: MARC21 Authority Format
   - Part 3: Code Architecture & Implementation
   - Part 4: Authority Linking System
   - Part 5: Search Architecture
   - Part 6: Architectural Patterns for RP-GPS
   - Part 7: Key Lessons
   - Part 8: Recommendations

2. **RP_GPS_PERSON_ENTITY_DESIGN_2025_11_09_1535_UTC.md**
   - Part 1: Core Person Entity
   - Part 2: Variant Names
   - Part 3: Person Relationships
   - Part 4: Source-Person Linking
   - Part 5: Person Merge Operations
   - Part 6: Search & Query Operations
   - Part 7: Elasticsearch Index Mapping
   - Part 8: API Endpoints
   - Part 9: Testing Strategy
   - Part 10: Migration & Implementation Plan

### Previous Session's Documents

3. **KOHA_GENEALOGY_STRATEGIC_ANALYSIS.md** (found in exploration branch)
   - Part 1: What is Koha?
   - Part 2: Architectural Alignments
   - Part 3: Mapping to Genealogical Concepts
   - Part 4: Comparison with RP-GPS
   - Part 5: Strategic Options (5 approaches)
   - Part 6: Recommendation (Option 4)
   - Part 7: Branch Strategy
   - Part 8: Deliverables & Timeline
   - Part 9: Risk Assessment
   - Part 10: Success Criteria

---

*Comprehensive handover complete. All research documented, specifications ready, next steps clear. Ready for implementation.*
