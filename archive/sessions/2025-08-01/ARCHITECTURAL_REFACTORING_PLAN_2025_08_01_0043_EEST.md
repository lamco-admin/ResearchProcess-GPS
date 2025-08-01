# ResearchProcess-GPS Architectural Refactoring Plan
## Fixing Layer Organization & Entity Type Inconsistencies
### Timestamp: 2025-08-01 00:43:16 EEST
### Priority: CRITICAL - Must Complete Before Further Development

---

## 🚨 EXECUTIVE SUMMARY

The ResearchProcess-GPS codebase has significant architectural inconsistencies that must be addressed:

1. **EntityType enum is incomplete** - Missing 7 entities, includes 1 non-entity
2. **entity_type_mapper violates NO_FALLBACK_POLICY** - Has dangerous fallback behavior
3. **Layer organization is confused** - Entities are misclassified between layers
4. **Naming inconsistencies** - EvidenceAnalysis should be AnalysisReport
5. **Documentation doesn't match implementation** - Claims 22 entities, actually 19

**This refactoring is now our #1 priority** and blocks all other development until resolved.

---

## 📊 CURRENT STATE ANALYSIS

### Entity Count Reality Check
- **Documentation claims**: 22 entities
- **Actual Entity trait implementations**: 19
- **ConfigEntity implementations**: 5 (different trait, not entities)
- **EntityType enum entries**: 13 (incomplete)

### Architectural Issues

#### 1. EntityType Enum Problems
```rust
// Current enum has only 13 entries:
pub enum EntityType {
    Theory, Evidence, Source, Repository, WorkProduct, 
    ProofStatement, Researcher, ResearchLog, Citation, 
    Fact, IdentityPersona, Relationship,
}
```

**Missing Entities**:
- Analysis, Confidence, EvidenceAnalysis, ResearchSession, 
- ResearchActivity, Location, Workspace

**Invalid Entry**:
- Repository (it's a SourceType variant, not an entity)

#### 2. NO_FALLBACK_POLICY Violations
```rust
// entity_type_mapper.rs - MULTIPLE VIOLATIONS:
"analysis" => EntityType::Theory,        // WRONG mapping!
"workspace" => EntityType::WorkProduct,  // WRONG mapping!
_ => {
    warn!("Unknown entity type: {}, defaulting to Theory", s);
    EntityType::Theory  // VIOLATION: Silent fallback!
}
```

#### 3. Layer Misclassification
- **Theory** is in Layer 1 but belongs in Layer 2 (it's a process orchestrator)
- **ProofStatement** is in Layer 1 but belongs in Layer 2 (it's a work product)
- **Analysis** vs **EvidenceAnalysis** confusion (data entity vs work product)

---

## 🎯 TARGET ARCHITECTURE

### Refined Layer Model

#### Layer 1 - Core Data Model
**Purpose**: The expanded genealogical data model that compresses to GEDCOM-like formats

```
Source Management Triad:
├── Source (hierarchical: System→Repository→Collection→Series→Item)
├── Citation (flexible connector between any entity and sources)
└── Evidence (extracted information with GPS classifications)

Identity & Relations:
├── IdentityPersona (people with state progression)
├── Relationship (connections between entities)
└── Location (places with temporal periods and hierarchy)

Atomic Data:
├── Fact (atomic claims extracted from evidence)
├── Confidence (assessment narratives)
└── Analysis (flexible analytical reasoning container)
```

#### Layer 2 - Research Process & Products
**Purpose**: Process orchestration and deliverable outputs

```
Process Orchestration:
├── Theory (main research driver with state machine)
├── ResearchSession (work session tracking)
├── ResearchActivity (atomic research tasks)
├── ResearchLog (process documentation)
└── Researcher (agents performing research)

Work Products (Deliverables):
├── WorkProduct (base for all outputs)
├── ProofStatement (GPS proof argument document)
├── AnalysisReport (renamed from EvidenceAnalysis)
└── (future: Report, Article, Book, Chart, etc.)
```

#### Layer 3 - Workflow & Configuration
**Purpose**: System configuration and templates

```
Special Container:
└── Workspace (research environment container)

Configuration Entities (ConfigEntity trait):
├── MethodologyConfig (GPS, BCG, custom standards)
├── ModuleConfig (plugin configurations)
├── StandardsRegistry (available methodologies)
├── TemplateRegistry (document templates)
└── ValidationRule (compliance rules)
```

---

## 📋 REFACTORING TASKS

### Phase 1: Fix Critical Violations (Immediate)

#### Task 1.1: Fix EntityType Enum ✅
```rust
// crates/rp-core/src/layer3/mod.rs
pub enum EntityType {
    // Layer 1 - Core Data (9 entities)
    Source,
    Citation,
    Evidence,
    IdentityPersona,
    Relationship,
    Location,
    Fact,
    Confidence,
    Analysis,
    
    // Layer 2 - Process & Products (9 entities)
    Theory,
    ResearchSession,
    ResearchActivity,
    ResearchLog,
    Researcher,
    WorkProduct,
    ProofStatement,
    EvidenceAnalysis, // Will rename to AnalysisReport
    
    // Layer 3 - Special (1 entity)
    Workspace,
}
```

#### Task 1.2: Remove NO_FALLBACK_POLICY Violations ✅
```rust
// crates/rp-server/src/entity_type_mapper.rs
pub fn parse_entity_type(s: &str) -> Result<EntityType> {
    match s.to_lowercase().as_str() {
        "theory" => Ok(EntityType::Theory),
        "source" => Ok(EntityType::Source),
        // ... all 19 mappings ...
        "analysis" => Ok(EntityType::Analysis), // FIX
        "workspace" => Ok(EntityType::Workspace), // FIX
        
        // Aliases for backward compatibility
        "person" => Ok(EntityType::IdentityPersona),
        "document" => Ok(EntityType::WorkProduct),
        
        // NO FALLBACK - Return error for unknown
        _ => Err(ApiError::InvalidEntityType(s.to_string()))
    }
}
```

### Phase 2: Structural Refactoring

#### Task 2.1: Rename EvidenceAnalysis → AnalysisReport
1. Rename file: `evidence_analysis.rs` → `analysis_report.rs`
2. Update struct name and all references
3. Update EntityType enum
4. Update entity_type_mapper
5. Update API endpoints if needed

#### Task 2.2: Reorganize Module Structure (Optional)
```
crates/rp-core/src/
├── layer1/           # Core data entities
│   ├── mod.rs
│   ├── source.rs
│   ├── citation.rs
│   ├── evidence.rs
│   ├── identity_persona.rs
│   ├── relationship.rs
│   ├── location.rs
│   ├── fact.rs
│   ├── confidence.rs
│   └── analysis.rs
├── layer2/           # Process & products
│   ├── mod.rs
│   ├── theory.rs
│   ├── research_session.rs
│   ├── research_activity.rs
│   ├── research_log.rs
│   ├── researcher.rs
│   ├── work_product.rs
│   ├── proof_statement.rs
│   └── analysis_report.rs
└── layer3/           # Configuration
    ├── mod.rs
    ├── workspace.rs
    └── ... (config entities)
```

### Phase 3: Documentation Updates

#### Task 3.1: Update Master Plan
- Correct entity count from 22 to 19
- Document the Entity vs ConfigEntity distinction
- Update layer organization
- Add architectural decision log entry

#### Task 3.2: Update API Documentation
- Document that "persons" endpoints use IdentityPersona
- Add migration notes for EvidenceAnalysis → AnalysisReport
- Document removal of fallback behavior

### Phase 4: Testing & Validation

#### Task 4.1: Test Entity Type Resolution
- Verify all 19 entity types resolve correctly
- Ensure unknown types return errors (no fallback)
- Test backward compatibility aliases

#### Task 4.2: API Endpoint Testing
- Test CRUD operations for all entity types
- Verify search filters work with new enum
- Check entity-specific endpoints

---

## 🚀 IMPLEMENTATION SEQUENCE

### Day 1 (Immediate - High Priority)
1. [ ] Fix EntityType enum - Add missing entries, remove Repository
2. [ ] Fix entity_type_mapper - Remove fallback, fix mappings
3. [ ] Test basic API functionality
4. [ ] Update this plan with results

### Day 2 (Refactoring)
5. [ ] Rename EvidenceAnalysis → AnalysisReport
6. [ ] Update all references in codebase
7. [ ] Consider module reorganization
8. [ ] Run full test suite

### Day 3 (Documentation)
9. [ ] Update master plan with correct counts
10. [ ] Document architectural decisions
11. [ ] Update API documentation
12. [ ] Create migration guide if needed

---

## 🎓 KEY ARCHITECTURAL PRINCIPLES

### 1. Layer Separation
- **Layer 1**: Pure data entities (the "what")
- **Layer 2**: Process and products (the "how" and "deliverables")
- **Layer 3**: Configuration (the "settings")

### 2. Entity vs ConfigEntity
- **Entity**: Full lifecycle, audit trail, business logic
- **ConfigEntity**: Lightweight configuration objects

### 3. NO_FALLBACK_POLICY
- Every operation must succeed or fail explicitly
- No silent defaults or degraded operation
- Better to fail fast than hide problems

### 4. Theory as Process Orchestrator
Theory/ResearchQuestion is the central process driver in Layer 2:
- Has state machine (Draft → Active → Proven/Disproven)
- Orchestrates relationships between Layer 1 entities
- Drives research methodology

---

## 📊 SUCCESS METRICS

1. **EntityType enum has exactly 19 entries** matching all Entity implementations
2. **No fallback behavior** in entity type resolution
3. **All API tests pass** with new enum
4. **Documentation matches implementation** exactly
5. **Clear layer boundaries** in code organization

---

## 🔄 CHANGE LOG

### 2025-08-01 00:43 EEST - Initial Plan
- Created comprehensive refactoring plan
- Identified all architectural inconsistencies
- Defined clear layer model
- Established implementation sequence

---

*This plan supersedes all previous architectural decisions and establishes the canonical layer model for ResearchProcess-GPS.*