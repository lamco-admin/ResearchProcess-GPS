# ResearchProcess-GPS Rust Implementation - Session Continuation Prompt
## Date: July 31, 2025

I'm Greg Lamberson, continuing development of ResearchProcess-GPS, now architected as a full Rust implementation. In the previous session, we made a critical architectural pivot from a Python/Django application to a Rust-based protocol, engine, and platform for professional genealogical research.

## Project Evolution Summary

### Original Vision
ResearchProcess-GPS began as a revolutionary genealogical data model that treats research as a first-class concept, capturing not just conclusions but the entire research process, reasoning chains, and confidence narratives.

### Architectural Pivot (This Session)
We've reconceptualized ResearchProcess-GPS as:
1. **A Protocol** - Standardized way to represent, store, and exchange genealogical research
2. **An Engine** - High-performance core for analysis, reasoning, and state management  
3. **A Platform** - Extensible framework for building genealogical tools

## Current Project Status

### Conceptual Model (COMPLETE)
The data model includes these first-class entities:
- **Theory** (Research Question) - What we're investigating
- **Researcher** - Who's doing the research with full attribution
- **Analysis** (NEW) - Reasoning chains and analytical processes
- **IdentityPersona** - Evidence-based identity references
- **Person** - Concluded individuals
- **Evidence** - What we extract (positive/negative/disproven)
- **Confidence** - Full narrative containers, not just scores
- **Source/Citation** - Hierarchical source management
- **Fact** - Unified events/attributes/characteristics

### Rust Architecture (PLANNED)
We've designed a comprehensive Rust implementation with:
- 12+ crates in a workspace structure
- Compile-time validated state machines
- Hybrid WASM + Native module system
- PostgreSQL with JSONB + specialized tables
- CRDT + Event Sourcing for collaboration
- Multi-level caching architecture
- Zero-trust security model

## Key Design Decisions Made

1. **Full Rust Implementation** - Performance, safety, and concurrency
2. **Protocol-First Design** - Not an app but a standard
3. **Hybrid Storage Model** - PostgreSQL JSONB + binary + specialized tables
4. **State Machines via Macros** - Compile-time validation
5. **Capability-Based Modules** - WASM sandboxed + native performance
6. **GraphQL-Inspired Queries** - With genealogy extensions
7. **Multi-Level Caching** - Connection → Redis → Materialized views

## Critical Documents to Reference

### Conceptual Model Documents
- `/home/greg/genealogy-ai/ResearchProcess-GPS/engine/UNIFIED_CONCEPTUAL_MODEL_WITH_ANALYSIS_2025_07_31.md` - Complete data model with Analysis entity
- `/home/greg/genealogy-ai/ResearchProcess-GPS/engine/ENTITY_RELATIONSHIP_WITH_ANALYSIS_2025_07_31.md` - Entity relationships
- `/home/greg/genealogy-ai/ResearchProcess-GPS/COMPREHENSIVE_CONFIDENCE_FRAMEWORK.md` - Revolutionary confidence approach
- `/home/greg/genealogy-ai/ResearchProcess-GPS/UNLEASHED_CORE_DATA_MODEL.md` - Original vision

### Implementation Documents
- `/home/greg/genealogy-ai/ResearchProcess-GPS/engine/core/models/analysis.py` - Analysis entity implementation (Python reference)
- `/home/greg/genealogy-ai/ResearchProcess-GPS/engine/core/models/confidence_enhanced.py` - Enhanced confidence with Analysis support
- `/home/greg/genealogy-ai/ResearchProcess-GPS/engine/core/models/researcher.py` - Researcher entity with attribution

### Architecture & Planning
- `/home/greg/genealogy-ai/ResearchProcess-GPS/ULTRATHINK_PROJECT_PLAN_RUST_2025_07_31.md` - Comprehensive project plan
- `/home/greg/genealogy-ai/ResearchProcess-GPS/CRITICAL_ARCHITECTURAL_DECISIONS_2025_07_31.md` - Key technical decisions
- `/home/greg/genealogy-ai/ResearchProcess-GPS/IMPLEMENTATION_ROADMAP_WEEK_BY_WEEK_2025_07_31.md` - Weekly development plan
- `/home/greg/genealogy-ai/ResearchProcess-GPS/CONCEPTUAL_MODEL_STATUS_WITH_ANALYSIS_2025_07_31.md` - Architecture assessment

### Original Requirements
- `/home/greg/genealogy-ai/ResearchProcess-GPS/STANDARDS_METHODOLOGY_FRAMEWORK.md` - GPS as configuration
- `/home/greg/genealogy-ai/ResearchProcess-GPS/RESEARCH_PROCESS_FEATURE_COMPILATION.md` - User needs analysis

## Next Session Focus

### Week 1 Priorities (Development Start)
1. **Set up Rust workspace structure**
   ```bash
   cargo new researchprocess-gps
   cd researchprocess-gps
   # Create crate structure as defined in project plan
   ```

2. **Define core traits in `rp-core`**
   - Entity trait hierarchy
   - State machine traits  
   - Validation framework
   - Error types

3. **Implement Theory entity**
   - Full state machine
   - Serialization support
   - Basic validation

4. **PostgreSQL schema design**
   - Hybrid JSONB + binary storage
   - Change capture mechanism
   - Migration system

### Technical Decisions Needed Immediately

1. **Entity ID Strategy**
   - UUID v7 (time-ordered) vs ULID vs custom?
   - How to handle workspace partitioning?

2. **State Machine Macro Design**
   - Procedural macro syntax finalization
   - State transition validation rules

3. **Binary Protocol Format**
   - Message framing details
   - Compression strategy (zstd vs lz4?)

4. **Development Environment**
   - PostgreSQL 15 or 16?
   - Docker compose setup
   - CI/CD from day one?

## Key Architectural Principles to Maintain

1. **Protocol Over Application** - We're building a standard, not just software
2. **Process Over Conclusions** - Capture the journey, not just the destination
3. **Attribution Throughout** - Every action tracked to a Researcher
4. **Extensibility First** - Modules and plugins from the start
5. **Professional Grade** - This is for serious genealogical research

## Environment Setup for Next Session

```bash
# Rust toolchain
rustup toolchain install stable
rustup toolchain install nightly
rustup component add rustfmt clippy

# Database
docker run -d \
  --name rp-postgres \
  -e POSTGRES_PASSWORD=devpassword \
  -e POSTGRES_DB=researchprocess \
  -p 5432:5432 \
  postgres:15-alpine

# Development tools
cargo install cargo-watch cargo-nextest cargo-tarpaulin sqlx-cli

# Environment variables
export DATABASE_URL="postgres://postgres:devpassword@localhost/researchprocess"
export RUST_LOG="rp=debug"
```

## Questions to Address Next Session

1. Should we start with a minimal CLI tool to prove concepts?
2. How do we want to handle GRAMPS integration - read their PostgreSQL directly?
3. Do we implement Theory + Analysis first, or Theory + Evidence?
4. Should we use workspace inheritance for dependencies?
5. How early do we want to implement the module system?

## Remember

- This is a Rust-first implementation (no Python except maybe for tools later)
- We're building infrastructure, not a genealogy application
- The protocol enables an ecosystem of tools
- Performance and correctness are equally important
- Start simple but design for the complete vision

When starting the next session, begin by reviewing the ULTRATHINK project plan and the week-by-week roadmap, then start implementing Week 1 deliverables.