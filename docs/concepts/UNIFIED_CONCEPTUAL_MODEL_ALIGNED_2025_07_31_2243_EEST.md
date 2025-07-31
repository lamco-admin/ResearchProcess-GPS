# Unified Conceptual Model - ResearchProcess-GPS (Aligned with Implementation)
## 2025-07-31 22:43:12 EEST
## Version 2.0 - Reflects Actual Implementation

---

## Executive Summary

ResearchProcess-GPS implements a revolutionary approach to genealogical data management where **methodologies, standards, and work products become metadata configurations** rather than hard-coded constraints. The system models the entire research process, not just conclusions.

**Implementation Status**: 22 of 23 planned entities implemented across three architectural layers.

---

## Core Philosophy

### Standards as Configuration, Not Code
```yaml
Principle: "Every standard becomes a configurable ruleset"
Implementation:
  - GPS elements → Compliance checklists
  - BCG standards → Competency metrics
  - Citation formats → Template configurations
  - Work products → Document schemas
```

---

## Implemented Entity Architecture

### Layer 1: Core Genealogical Entities (10 Implemented)

#### 1. Theory (Research Question)
```rust
// File: crates/rp-core/src/theory.rs
pub struct Theory {
    pub metadata: EntityMetadata,
    pub question: String,
    pub hypothesis: String,
    pub state: TheoryState,  // EXPLORING → TESTING → CONCLUDED → QUESTIONED
    pub parent_theory: Option<EntityId>,  // For branching
    pub child_theories: Vec<EntityId>,
    pub supporting_evidence: Vec<EntityId>,
    pub conflicting_evidence: Vec<EntityId>,
    pub current_confidence: Option<EntityId>,
    pub assigned_researchers: Vec<EntityId>,
    pub work_products: Vec<EntityId>,
    pub compliance_configs: HashMap<String, serde_json::Value>,
}
```

#### 2. IdentityPersona (Pre-Conclusion Identity)
```rust
// File: crates/rp-core/src/identity_persona.rs
pub struct IdentityPersona {
    pub metadata: EntityMetadata,
    pub label: String,  // Working name/label
    pub state: IdentityState,  // REFERENCE → WORKING → HYPOTHESIS → CANDIDATE
    pub evidence_references: Vec<EvidenceReference>,
    pub proposed_facts: Vec<EntityId>,
    pub confidence_assessments: Vec<EntityId>,
    pub variant_of: Option<EntityId>,  // Parent identity
    pub variants: Vec<EntityId>,       // Child identities
    pub promotion_eligible: bool,
    pub promotion_criteria: serde_json::Value,
}
```

#### 3. Source (Information Repository)
```rust
// File: crates/rp-core/src/source.rs
pub struct Source {
    pub metadata: EntityMetadata,
    pub title: String,
    pub source_type: SourceType,  // ITEM → SERIES → COLLECTION → REPOSITORY
    pub repository: Option<String>,
    pub call_number: Option<String>,
    pub url: Option<String>,
    pub accessed_date: Option<DateTime<Utc>>,
    pub quality_assessment: QualityAssessment,
    pub parent_source: Option<EntityId>,
    pub child_sources: Vec<EntityId>,
    pub evidence_extracted: Vec<EntityId>,
    pub citation_template: Option<String>,
    pub template_fields: HashMap<String, String>,
}
```

#### 4. Evidence (Extracted Information)
```rust
// File: crates/rp-core/src/evidence.rs
pub struct Evidence {
    pub metadata: EntityMetadata,
    pub evidence_type: EvidenceType,  // POSITIVE, NEGATIVE, DISPROVEN
    pub source_id: EntityId,
    pub extracted_text: String,
    pub interpreted_text: Option<String>,
    pub extraction_method: ExtractionMethod,  // MANUAL, OCR, AI_ASSISTED
    pub extraction_confidence: HashMap<String, f64>,
    pub applies_to_identities: Vec<EntityId>,
    pub supports_theories: Vec<EntityId>,
    pub contradicts_theories: Vec<EntityId>,
    pub extracted_by: EntityId,
    pub extraction_date: DateTime<Utc>,
}
```

#### 5. Citation (Source Reference)
```rust
// File: crates/rp-core/src/citation.rs
pub struct Citation {
    pub metadata: EntityMetadata,
    pub citation_type: CitationType,  // QUICK, FULL, ELEMENT, ANALYZED
    pub from_entity: EntityId,
    pub from_entity_type: String,
    pub to_source: EntityId,
    pub specific_location: Option<String>,  // Page, paragraph, etc.
    pub cited_elements: Vec<CitedElement>,
    pub formatted_citations: HashMap<String, String>,  // Style → Text
    pub validation_status: ValidationStatus,
}
```

#### 6. Confidence (Assessment Container)
```rust
// File: crates/rp-core/src/confidence.rs
pub struct Confidence {
    pub metadata: EntityMetadata,
    pub target_entity: EntityId,
    pub target_entity_type: String,
    pub assessments: Vec<ConfidenceAssessment>,
    pub current_score: Option<f64>,  // Calculated from assessments
    pub methodology_used: String,
    pub peer_reviewed: bool,
    pub peer_reviewers: Vec<EntityId>,
}
```

#### 7. Relationship (Entity Connection)
```rust
// File: crates/rp-core/src/relationship.rs
pub struct Relationship {
    pub metadata: EntityMetadata,
    pub relationship_type: RelationshipType,
    pub from_entity: EntityId,
    pub from_entity_type: String,
    pub to_entity: EntityId,
    pub to_entity_type: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub evidence_support: Vec<EntityId>,
    pub confidence: Option<EntityId>,
    pub temporal_scope: Option<TemporalScope>,
}
```

#### 8. EvidenceAnalysis (Analysis Work Product)
```rust
// File: crates/rp-core/src/evidence_analysis.rs
pub struct EvidenceAnalysis {
    pub metadata: EntityMetadata,
    pub analysis_type: AnalysisType,
    pub target_theory: EntityId,
    pub evidence_examined: Vec<EntityId>,
    pub methodology: String,
    pub findings: String,
    pub evidence_matrix: Option<serde_json::Value>,
    pub conflicts_identified: Vec<ConflictRecord>,
    pub conflicts_resolved: Vec<ResolutionRecord>,
    pub conclusion: Option<String>,
}
```

#### 9. ProofStatement (Proof Argument)
```rust
// File: crates/rp-core/src/proof_statement.rs
pub struct ProofStatement {
    pub metadata: EntityMetadata,
    pub statement_type: ProofStatementType,  // ARGUMENT, SUMMARY
    pub target_theory: EntityId,
    pub gps_compliance: GPSCompliance,
    pub research_question: String,
    pub evidence_presented: Vec<EntityId>,
    pub analysis_performed: Vec<EntityId>,
    pub reasoning_chain: Vec<ReasoningStep>,
    pub conclusion: String,
    pub peer_reviews: Vec<EntityId>,
}
```

#### 10. Fact (Atomic Claim)
```rust
// File: crates/rp-core/src/fact.rs
pub struct Fact {
    pub metadata: EntityMetadata,
    pub fact_type: FactType,
    pub subject_entity: EntityId,
    pub subject_entity_type: String,
    pub predicate: String,
    pub object_value: serde_json::Value,
    pub temporal_scope: Option<TemporalScope>,
    pub spatial_scope: Option<SpatialScope>,
    pub evidence_basis: Vec<EntityId>,
    pub confidence: Option<EntityId>,
}
```

### Layer 2: Research Process Entities (6 Implemented)

#### 11. Researcher (Agent/Actor)
```rust
// File: crates/rp-core/src/researcher.rs
pub struct Researcher {
    pub metadata: EntityMetadata,
    pub name: String,
    pub researcher_type: ResearcherType,  // INDIVIDUAL, TEAM, ORGANIZATION
    pub credentials: Vec<Credential>,
    pub contact_info: ContactInfo,
    pub specializations: Vec<String>,
    pub active_theories: Vec<EntityId>,
    pub contributions: HashMap<String, Vec<EntityId>>,  // Type → IDs
    pub permissions: HashMap<String, PermissionLevel>,
}
```

#### 12. ResearchLog (Process Documentation)
```rust
// File: crates/rp-core/src/research_log.rs
pub struct ResearchLog {
    pub metadata: EntityMetadata,
    pub log_type: LogType,  // SESSION, PROJECT, REPOSITORY_VISIT
    pub owner_researcher: EntityId,
    pub associated_theory: Option<EntityId>,
    pub entries: Vec<LogEntry>,
    pub schema_config: serde_json::Value,
    pub auto_captured: bool,
    pub export_formats: Vec<String>,
}
```

#### 13. ResearchSession (Work Session)
```rust
// File: crates/rp-core/src/research_session.rs
pub struct ResearchSession {
    pub metadata: EntityMetadata,
    pub session_type: SessionType,
    pub researcher: EntityId,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub objectives: Vec<String>,
    pub activities: Vec<EntityId>,
    pub outputs: Vec<EntityId>,
    pub notes: Option<String>,
}
```

#### 14. ResearchActivity (Atomic Activity)
```rust
// File: crates/rp-core/src/research_activity.rs
pub struct ResearchActivity {
    pub metadata: EntityMetadata,
    pub activity_type: ActivityType,  // SEARCH, EXTRACT, ANALYZE, DOCUMENT
    pub session_id: EntityId,
    pub timestamp: DateTime<Utc>,
    pub duration: Option<Duration>,
    pub target_entity: Option<EntityId>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub results: HashMap<String, serde_json::Value>,
}
```

#### 15. WorkProduct (Generated Output)
```rust
// File: crates/rp-core/src/work_product.rs
pub struct WorkProduct {
    pub metadata: EntityMetadata,
    pub product_type: WorkProductType,
    pub template_id: String,
    pub schema_version: String,
    pub generated_from: Vec<EntityId>,
    pub generation_metadata: HashMap<String, serde_json::Value>,
    pub validation_rules: Vec<String>,
    pub export_formats: Vec<String>,
    pub published: bool,
}
```

#### 16. Analysis (Analysis Result)
```rust
// File: crates/rp-core/src/analysis.rs
pub struct Analysis {
    pub metadata: EntityMetadata,
    pub analysis_type: String,
    pub scope: AnalysisScope,
    pub inputs: Vec<EntityId>,
    pub methodology: String,
    pub execution_time: Duration,
    pub results: serde_json::Value,
    pub visualizations: Vec<String>,
    pub cached: bool,
}
```

### Layer 3: Infrastructure & Configuration (6 Implemented)

#### 17. Workspace (User Environment)
```rust
// File: crates/rp-core/src/layer3/workspace.rs
pub struct Workspace {
    pub metadata: EntityMetadata,
    pub name: String,
    pub owner: EntityId,
    pub members: Vec<WorkspaceMember>,
    pub active_methodologies: Vec<String>,
    pub active_standards: Vec<String>,
    pub enabled_modules: Vec<String>,
    pub default_templates: HashMap<String, String>,
    pub open_theories: Vec<EntityId>,
}
```

#### 18. MethodologyConfig (Standards as Data)
```rust
// File: crates/rp-core/src/layer3/methodology_config.rs
pub struct MethodologyConfig {
    pub id: EntityId,
    pub methodology_name: String,  // GPS, BCG, FAN
    pub version: String,
    pub workflow_stages: Vec<WorkflowStage>,
    pub required_elements: HashMap<String, RequirementSpec>,
    pub compliance_rules: Vec<ComplianceRule>,
    pub enabled_features: Vec<String>,
    pub validation_schema: serde_json::Value,
}
```

#### 19. ModuleConfig (Module Settings)
```rust
// File: crates/rp-core/src/layer3/module_config.rs
pub struct ModuleConfig {
    pub id: EntityId,
    pub module_type: ModuleType,  // CAPTURE, ANALYSIS, GENERATION
    pub module_id: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub settings: HashMap<String, serde_json::Value>,
    pub resource_limits: ResourceLimits,
}
```

#### 20. StandardsRegistry (Available Standards)
```rust
// File: crates/rp-core/src/layer3/standards_registry.rs
pub struct StandardsRegistry {
    pub id: EntityId,
    pub standards: HashMap<String, StandardConfig>,
    pub active_standards: Vec<String>,
    pub version_compatibility: HashMap<String, VersionSpec>,
    pub migration_rules: HashMap<String, MigrationRule>,
    pub last_updated: DateTime<Utc>,
}
```

#### 21. TemplateRegistry (Document Templates)
```rust
// File: crates/rp-core/src/layer3/template_registry.rs
pub struct TemplateRegistry {
    pub id: EntityId,
    pub templates: HashMap<String, Template>,
    pub template_categories: HashMap<String, Vec<String>>,
    pub compatibility_matrix: HashMap<String, Vec<String>>,
    pub custom_templates: HashMap<String, CustomTemplate>,
}
```

#### 22. ValidationRule (Configurable Rules)
```rust
// File: crates/rp-core/src/layer3/validation_rule.rs
pub struct ValidationRule {
    pub id: EntityId,
    pub rule_type: RuleType,  // REQUIRED_FIELD, FORMAT_CHECK, COMPLIANCE
    pub applies_to: Vec<String>,  // Entity types
    pub rule_spec: serde_json::Value,
    pub error_message: String,
    pub severity: Severity,  // ERROR, WARNING, INFO
    pub active: bool,
}
```

### Not Yet Implemented (2 Entities)

#### 23. Person (Promoted Identity)
**Planned Location**: `crates/rp-core/src/person.rs`
```rust
// Conceptual - Not Yet Implemented
pub struct Person {
    pub metadata: EntityMetadata,
    pub state: PersonState,  // CONCLUDED → ACCEPTED → PUBLISHED → CHALLENGED
    pub source_identities: Vec<EntityId>,  // IdentityPersonas
    pub confirmed_facts: Vec<EntityId>,
    pub confirmed_relationships: Vec<EntityId>,
    pub vital_events: Vec<EntityId>,
    pub conclusion_basis: EntityId,  // ProofStatement
    pub demotion_triggers: Vec<DemotionTrigger>,
}
```

#### 24. ComplianceStatus (Standards Tracking)
**Planned Location**: `crates/rp-core/src/compliance_status.rs`
```rust
// Conceptual - Not Yet Implemented
pub struct ComplianceStatus {
    pub metadata: EntityMetadata,
    pub entity_id: EntityId,
    pub entity_type: String,
    pub standard: String,  // GPS-2025, BCG-3.4
    pub elements: HashMap<String, ElementStatus>,
    pub overall_score: f64,
    pub validation_date: DateTime<Utc>,
    pub validation_details: Vec<ValidationResult>,
    pub recommendations: Vec<String>,
}
```

---

## Entity Type Enum Alignment Issue

### Current EntityType Enum (13 entries)
```rust
// File: crates/rp-core/src/layer3/mod.rs
pub enum EntityType {
    Theory,
    Evidence,
    Source,
    Repository,      // Not a separate entity, part of Source
    WorkProduct,
    ProofStatement,
    Researcher,
    ResearchLog,
    Citation,
    Fact,
    IdentityPersona,
    Relationship,
}
```

### Required Updates
The EntityType enum needs to be updated to include all 22 implemented entities:
- Add: Analysis, Confidence, EvidenceAnalysis, ResearchSession, ResearchActivity
- Add: Workspace, MethodologyConfig, ModuleConfig, StandardsRegistry
- Add: TemplateRegistry, ValidationRule
- Remove: Repository (it's a SourceType, not separate entity)
- Future: Person, ComplianceStatus (when implemented)

---

## Standards as Configuration Examples

### GPS Configuration
```yaml
GPS_2025_Config:
  elements:
    reasonably_exhaustive_research:
      requirements:
        - repository_coverage: 
            vital_records: 0.90
            census_records: 0.80
        - negative_search_documentation: required
        
    complete_citations:
      template_system: "Evidence_Explained"
      required_elements: [author, title, date, location]
      
    thorough_analysis:
      evidence_matrix: required
      correlation_tracking: required
      
    conflict_resolution:
      documentation: required
      resolution_types: [explained, reconciled, prioritized]
      
    written_conclusion:
      formats: [proof_argument, proof_summary]
      peer_review: recommended
```

---

## State Machines

### Theory States
```
EXPLORING → HYPOTHESIZED → TESTING → CONCLUDED
                ↓            ↓          ↓
            ABANDONED    REVISED   QUESTIONED
```

### IdentityPersona States
```
REFERENCE → WORKING → HYPOTHESIS → CANDIDATE → [Person]
     ↓         ↓          ↓           ↓
  MERGED   ABANDONED  DISPROVEN   REJECTED
```

---

## Event Sourcing Implementation

Every entity operation generates events:
```rust
pub enum EventType {
    Created,
    Updated,
    StateChanged { from: String, to: String },
    Deleted,
    Merged { into: EntityId },
    Branched { new_id: EntityId },
    // ... entity-specific events
}
```

---

## API Integration Points

### REST Endpoints (Implemented)
- Generic CRUD for all entities
- Entity-specific operations (9 endpoints)
- Full-text search across entities
- WebSocket real-time events

### Authentication (Implemented)
- API key based (Bearer tokens)
- Per-operation permissions
- Workspace-based access control

---

## Next Implementation Priorities

1. **Fix EntityType Enum** - Add missing 10 entity types
2. **Implement Person Entity** - Complete Layer 1
3. **Implement ComplianceStatus** - GPS/BCG tracking
4. **Update entity_type_mapper.rs** - Remove workarounds
5. **Complete API Documentation** - OpenAPI/Swagger

---

*This model represents the actual implementation state as of 2025-07-31 22:43 EEST*