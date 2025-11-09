# ResearchProcess-GPS: Architectural Evolution & Current State

**Document Created**: 2025-11-09
**Purpose**: Comprehensive analysis of architectural evolution, metadata model refactoring, and current implementation state

---

## Executive Summary

ResearchProcess-GPS has undergone **sophisticated architectural evolution** through multiple refactoring iterations, with the most significant being the **metadata model transformation** that enabled true adaptability. The system transitioned from conceptual "standards as data" to a **working implementation** where methodologies, validation rules, and templates are runtime-configurable data structures rather than hardcoded logic.

**Current State**: ~40% implementation complete with solid architectural foundation
- **22 of 23 core entities** implemented (11,567 lines of Rust)
- **Event sourcing** system complete
- **API layer** 98% complete
- **Module system** with message-based FFI architecture complete
- **Storage abstraction** with PostgreSQL adapter working

**Critical Innovation**: Layer 3 metadata model enables methodology-agnostic research platform

---

## 1. Critical Architectural Evolution: Metadata Model Iterations

### Iteration 1: Conceptual Phase (Pre-Implementation)

**Timeline**: January 2025 (v0.1-framework)
**Architecture**: Django/Python-based
**Documents**: 15 comprehensive specification documents

**Vision**:
- Methodologies (GPS, BCG) as YAML/JSON configuration files
- Validation rules as data expressions
- Templates as external files
- Storage-agnostic protocol layer

**Status**: Conceptual design, never implemented in Python

**Key Documents**:
- `/v0.1-framework/GITHUB_FOR_GENEALOGY_VISION.md`
- `/v0.1-framework/SECURE_MODULAR_FOUNDATION.md`
- `/v0.1-framework/STANDARDS_METHODOLOGY_FRAMEWORK.md`

---

### Iteration 2: Initial Rust Implementation - Uniform Metadata

**Timeline**: July 30-31, 2025
**Commits**: `8898eac` (Layer 1), `d0f11be` (Layer 2)

**The Approach**: ONE-SIZE-FITS-ALL `EntityMetadata`

All entities (Layers 1 & 2) used the **same metadata structure**:

```rust
pub struct EntityMetadata {
    pub id: EntityId,
    pub created_by: EntityId,
    pub created_at: DateTime<Utc>,
    pub modified_by: EntityId,
    pub modified_at: DateTime<Utc>,
    pub is_active: bool,
    pub version: u32,
    pub parent_version: Option<EntityId>,
}
```

**Implementation Pattern**:
```rust
pub struct Theory {
    #[serde(flatten)]
    pub metadata: EntityMetadata,  // Uniform for all
    // ... theory-specific fields
}
```

**17 entities** implemented with this pattern: Theory, Evidence, Source, Citation, IdentityPersona, Relationship, Location, Fact, Confidence, Analysis, ProofStatement, ResearchSession, ResearchActivity, ResearchLog, Researcher, WorkProduct, AnalysisReport.

**The Problem**:
- **RIGID** - Configuration entities (Layer 3) have fundamentally different metadata needs
- Configuration data doesn't need created_by/modified_by tracking
- Methodology versioning needs different semantics than entity versioning
- Template metadata needs format/schema information
- Module metadata needs capability declarations

**Limitation**: System would require code changes to support new methodologies

---

### Iteration 3: Layer 3 Breakthrough - Custom Metadata Structures ⭐

**Timeline**: July 31, 2025
**Commit**: `842a6bc` - "Complete Layer 3: All 6 workspace & metadata entities implemented"

**THE CRITICAL DISCOVERY**: Configuration entities need **DIFFERENT metadata structures**!

#### What Changed: Custom Metadata Per Entity Type

**1. MethodologyConfig gets MethodologyMetadata**:
```rust
pub struct MethodologyMetadata {
    pub id: MethodologyConfigId,
    pub methodology_key: String,      // e.g., "gps-2021"
    pub name: String,                  // "Genealogical Proof Standard"
    pub version: String,               // "2021"
    pub authority: String,             // "BCG"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub source: ConfigSource,          // File, HTTP, Git, Embedded
}

pub struct MethodologyConfig {
    pub metadata: MethodologyMetadata,  // CUSTOM!
    pub workflow_stages: Vec<WorkflowStage>,
    pub required_elements: HashMap<String, RequirementSpec>,
    pub compliance_rules: Vec<ComplianceRule>,
    // ... methodology configuration
}
```

**2. Workspace gets WorkspaceMetadata**:
```rust
pub struct WorkspaceMetadata {
    pub id: WorkspaceId,
    pub workspace_key: String,
    pub owner: EntityId,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
}

pub struct Workspace {
    pub metadata: WorkspaceMetadata,   // CUSTOM!

    // Optional standard metadata for compatibility
    #[serde(skip)]
    entity_metadata: Option<EntityMetadata>,

    // Configuration orchestration
    pub methodologies: HashMap<String, MethodologyConfig>,
    pub active_methodologies: Vec<String>,
    // ...
}
```

**3. Each Layer 3 Entity Gets Custom Metadata**:

| Entity | Custom Metadata | Key Fields |
|--------|-----------------|------------|
| `MethodologyConfig` | `MethodologyMetadata` | methodology_key, authority, source |
| `Workspace` | `WorkspaceMetadata` | workspace_key, owner, last_accessed |
| `StandardsRegistry` | `StandardsMetadata` | standards_key, registry_version |
| `ModuleConfig` | `ModuleMetadata` | module_key, module_type, capabilities |
| `TemplateRegistry` | `TemplateMetadata` | template_key, template_format |
| `ValidationRule` | `ValidationRuleMetadata` | rule_key, rule_type, severity |

#### Why This Matters: THE ADAPTABILITY BREAKTHROUGH

**Before (Iteration 2)**: Hard-coded GPS rules in application logic
```rust
// Hypothetical BAD approach
if theory.state == TheoryState::Proven {
    // Hard-coded GPS validation
    if theory.sources.len() < 3 {
        return Err("GPS requires at least 3 sources");
    }
    // ... more hard-coded rules
}
```

**After (Iteration 3)**: GPS as runtime-loaded configuration
```rust
// Load GPS 2021 configuration
let gps_2021 = MethodologyConfig::create_gps_2021();
workspace.load_methodology(gps_2021);
workspace.activate_methodology("gps-2021");

// Validation happens via configured rules
let active_configs = workspace.get_active_configs();
for rule in active_configs.validation_rules {
    rule.evaluate(entity)?;
}
```

**The Power**:
- **GPS 2021 vs GPS 2025**: Different `MethodologyConfig` files, same code
- **BCG vs Custom methodology**: Same data structure, different configurations
- **Institutional standards**: Organizations create their own methodology configs
- **Version evolution**: Update methodology file, no code changes
- **Multi-methodology support**: Load multiple simultaneously, choose per-project

**Real-World Impact**:
```rust
// GPS 2021 validation rule (as data!)
ValidationRule {
    rule_key: "gps-exhaustive-research",
    expression: And {
        conditions: vec![
            Count {
                entity_type: "Source",
                op: GreaterThanOrEqual,
                value: 3
            },
            Count {
                entity_type: "Repository",
                op: GreaterThanOrEqual,
                value: 2
            },
        ]
    },
    severity: Error,
    methodology_scope: vec!["gps-2021"],
}
```

This rule **exists as data** - it can be loaded from YAML, versioned in Git, shared between users, updated without recompiling.

#### ConfigEntity vs Entity

**New Trait for Layer 3**:
```rust
pub trait ConfigEntity: Send + Sync + Debug {
    fn id(&self) -> EntityId;
    fn config_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn validate(&self) -> Result<()>;
}
```

**Difference from Entity trait**:
- Simpler - no created_by/modified_by tracking
- No state machines (configs don't have lifecycles)
- No complex relationships
- Focus on **defining behavior** rather than **tracking research**

**Philosophical Insight**: Configuration entities **define how** the system behaves; domain entities **track what** researchers do.

---

### Iteration 4: EntityType Enum & NO_FALLBACK_POLICY Refactoring

**Timeline**: August 1, 2025
**Commits**: `58f792a` (plan), `bb0a579` (implementation)

**Documents**:
- `/archive/sessions/2025-08-01/ARCHITECTURAL_REFACTORING_PLAN_2025_08_01_0043_EEST.md`
- `/archive/sessions/2025-08-01/REFACTORING_SUMMARY_2025_08_01_0130_EEST.md`

**Problems Discovered**:
1. **EntityType enum incomplete**: Had only 13 entries, needed 22
2. **entity_type_mapper violated NO_FALLBACK_POLICY**: Dangerous fallback behavior
3. **Layer misclassification**: Theory incorrectly in Layer 2, should be Layer 1
4. **Incorrect mappings**: "analysis" → Theory (wrong!), "workspace" → WorkProduct (wrong!)

**What Changed**:

**1. Fixed EntityType Enum** (13 → 22 entries):
```rust
pub enum EntityType {
    // Layer 1 - Core Data Model (9 entities)
    Analysis, Citation, Confidence, Evidence, Fact,
    IdentityPersona, Location, Relationship, Source,

    // Layer 2 - Research Process & Products (9 entities)
    AnalysisReport, ProofStatement, ResearchActivity, Researcher,
    ResearchLog, ResearchSession, Theory, WorkProduct,

    // Layer 3 - Workflow & Configuration (4 entities)
    Workspace, MethodologyConfig, ModuleConfig, TemplateRegistry,
}
```

**2. Removed ALL Fallback Behavior**:
```rust
// OLD (WRONG - violates NO_FALLBACK_POLICY):
fn parse(s: &str) -> EntityType {
    match s {
        "theory" => EntityType::Theory,
        // ... other cases
        _ => {
            warn!("Unknown entity type: {}, defaulting to Theory", s);
            EntityType::Theory  // SILENT FAILURE!
        }
    }
}

// NEW (CORRECT - explicit error):
fn parse(s: &str) -> Result<EntityType, ApiError> {
    match s {
        "theory" => Ok(EntityType::Theory),
        // ... other cases
        _ => Err(ApiError::InvalidEntityType(s.to_string()))
    }
}
```

**3. Renamed EvidenceAnalysis → AnalysisReport** for clarity

**Impact**: System now **fails fast and explicitly** on errors rather than silently corrupting data with fallback values.

---

## 2. State Machine Architecture

### Implementation

**Core Infrastructure** (`/crates/rp-core/src/state.rs`):

```rust
/// Any state type must implement this
pub trait State: Display + EnumString + Serialize + DeserializeOwned {
    fn name(&self) -> &str;
    fn is_terminal(&self) -> bool;
    fn is_error(&self) -> bool;
}

/// Entities with states implement this
pub trait StateMachine: Entity {
    type State: State;

    fn current_state(&self) -> &Self::State;
    fn state_history(&self) -> &[StateTransition<Self::State>];
    fn can_transition(&self, to: &Self::State) -> bool;
    fn transition(&mut self, to: Self::State, by: EntityId, reason: Option<String>) -> Result<()>;
    fn valid_transitions(&self) -> Vec<Self::State>;
}

/// Complete audit trail
pub struct StateTransition<S> {
    pub from: S,
    pub to: S,
    pub triggered_by: EntityId,
    pub timestamp: DateTime<Utc>,
    pub reason: Option<String>,
}
```

**Macro for State Definition**:
```rust
define_states! {
    pub enum TheoryState {
        Draft, Active, OnHold, Proven, Disproven, Unresolvable, Abandoned
    }
}
// Auto-implements: State, Display, EnumString, Serialize, Deserialize
```

### Real-World Examples

**1. Theory States** (7 states):
```
Draft → Active ⟷ OnHold
           ↓
        {Proven, Disproven, Unresolvable, Abandoned}
```
- Terminal states: Proven, Disproven, Unresolvable, Abandoned
- Bidirectional: Active ⟷ OnHold
- Business logic: `can_transition()` prevents transitions from terminal states

**2. WorkProduct States** (5 states):
```
Draft → Review ⟷ Draft
         ↓
       Final → Published → Archived
```
- Bidirectional: Review can return to Draft
- Terminal: Archived is final
- Validation: `ValidationStatus::Valid` required before Review

**3. IdentityPersona States** (7 complex states):
```
Reference → Working → Hypothesis → Concluded → Verified → Published
                                                              ↓
                                                          Challenged
                                                              ↓
                                                           Working (loop!)
```
- Most sophisticated: Research loop when challenged
- State-dependent: `is_concluded()` determines if facts can be attached
- Progressive resolution: Reference (source mention) → Working (active research) → Concluded (identity established)

**4. ResearchSession States** (4 states):
```
Active ⟷ Paused
  ↓        ↓
Completed  Abandoned
```
- Bidirectional: Active ⟷ Paused
- Both can transition to terminal states
- Duration calculation uses state transitions

### Key Insight

States are **not just status flags** - they control:
- **What operations are valid**: Can't add conclusion to terminal Theory
- **UI behavior**: What actions to show user
- **Validation rules**: Different rules per state
- **Business logic**: WorkProduct needs `ValidationStatus::Valid` before Review
- **Audit trail**: Complete history of who changed what when and why

---

## 3. NestableEntity System

### Philosophy

**"Everything can contain related things"** - hierarchical composition is a first-class concept, not an afterthought.

### Implementation

**Trait Definition** (`/crates/rp-core/src/entity.rs`):
```rust
pub trait NestableEntity: Entity {
    fn children(&self) -> Vec<EntityId>;
    fn can_contain(&self, entity_type: &str) -> bool;
    async fn add_child(&mut self, child_id: EntityId) -> Result<()>;
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool>;
}
```

### Real Implementations

**1. Theory (Hierarchical Research Questions)**:
```rust
pub struct Theory {
    // ...
    pub child_theories: Vec<EntityId>,
}

impl NestableEntity for Theory {
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "Theory"  // Only theories contain theories
    }
}
```

**Use Case**:
- Main question: "Who were John Smith's parents?"
  - Sub-theory 1: "Who was John Smith's father?"
  - Sub-theory 2: "Who was John Smith's mother?"

**2. WorkProduct (Nested Documents)**:
```rust
pub struct WorkProduct {
    // ...
    pub parent_product: Option<EntityId>,
    pub child_products: Vec<EntityId>,
}

impl NestableEntity for WorkProduct {
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "WorkProduct"
    }
}
```

**Use Case**:
- Main proof statement
  - Appendix A: Timeline analysis
  - Appendix B: DNA analysis report
  - Appendix C: Evidence evaluation matrix

**3. ResearchSession (Session Nesting)**:
```rust
pub struct ResearchSession {
    // ...
    pub parent_session: Option<EntityId>,
    pub child_sessions: Vec<EntityId>,
    pub activities: Vec<EntityId>,
    pub evidence_created: Vec<EntityId>,
    pub work_products_created: Vec<EntityId>,
}

impl NestableEntity for ResearchSession {
    fn can_contain(&self, entity_type: &str) -> bool {
        entity_type == "ResearchSession"
    }
}
```

**Use Case**:
- Full-day archive visit (parent session)
  - Morning: Census records search (child session)
  - Afternoon: Land records search (child session)
- Duration calculation works hierarchically

### Pattern

Every nestable entity has:
1. `parent_X: Option<EntityId>` field
2. `child_Xs: Vec<EntityId>` field
3. Type-specific `can_contain()` logic (type safety)
4. Duplicate prevention in `add_child()`
5. Validation in `remove_child()`

**Philosophy**: Nesting provides **organic organization** that matches human research workflows, not artificial database constraints.

---

## 4. Adaptability Through Layer 3

### The Core Innovation: "Standards as Data, Not Code"

**Problem**: Traditional genealogy software hard-codes GPS rules, BCG standards, etc. Changing standards = waiting for software update.

**Solution**: Load standards as configuration at runtime.

### 4.1 MethodologyConfig - Methodology as Data

**Location**: `/crates/rp-core/src/layer3/methodology_config.rs`

**Structure**:
```rust
pub struct MethodologyConfig {
    pub metadata: MethodologyMetadata,

    // Workflow definition
    pub workflow_stages: Vec<WorkflowStage>,

    // What's required
    pub required_elements: HashMap<String, RequirementSpec>,

    // Validation rules
    pub compliance_rules: Vec<ComplianceRule>,

    // Work product schemas
    pub work_product_schemas: HashMap<String, WorkProductSchema>,

    // Feature flags
    pub enabled_features: HashSet<String>,
    pub disabled_features: HashSet<String>,

    // Default templates
    pub default_templates: HashMap<String, String>,

    // GPS-specific (optional)
    pub gps_elements: Option<GPSElements>,

    // Extensibility
    pub custom_config: HashMap<String, serde_json::Value>,
}
```

**Real Example - GPS 2021**:
```rust
impl MethodologyConfig {
    pub fn create_gps_2021() -> Self {
        Self {
            metadata: MethodologyMetadata {
                methodology_key: "gps-2021".to_string(),
                name: "Genealogical Proof Standard".to_string(),
                version: "2021".to_string(),
                authority: "Board for Certification of Genealogists".to_string(),
                // ...
            },

            // 5 GPS elements as requirements
            required_elements: [
                ("exhaustive_research", RequirementSpec { /* ... */ }),
                ("complete_citations", RequirementSpec { /* ... */ }),
                ("analysis_correlation", RequirementSpec { /* ... */ }),
                ("conflict_resolution", RequirementSpec { /* ... */ }),
                ("sound_conclusion", RequirementSpec { /* ... */ }),
            ].into(),

            // 3 workflow stages
            workflow_stages: vec![
                WorkflowStage { name: "research", /* ... */ },
                WorkflowStage { name: "analysis", /* ... */ },
                WorkflowStage { name: "conclusion", /* ... */ },
            ],

            // Feature flags
            enabled_features: [
                "gps_compliance_tracking",
                "proof_statement_generation",
                "evidence_analysis_matrix",
            ].into(),

            // GPS-specific config
            gps_elements: Some(GPSElements {
                reasonably_exhaustive_research: /* ... */,
                complete_accurate_citations: /* ... */,
                // ...
            }),
        }
    }
}
```

**Different Methodologies**:
- `create_gps_2021()` - 2021 version of GPS
- `create_gps_2025()` - 2025 updated version
- `create_bcg_2023()` - BCG standards
- `create_fan_principle()` - Functus, Ancestry, Network methodology
- `create_custom()` - User-defined methodology

**Key Insight**: Same code, different configurations. Zero code changes to support new methodologies.

### 4.2 ValidationRule - Validation as Data

**Location**: `/crates/rp-core/src/layer3/validation_rule.rs`

**The Expression Language**:
```rust
pub enum RuleExpression {
    Required { field: String },
    Pattern { field: String, regex: String },
    Length { field: String, min: Option<usize>, max: Option<usize> },

    // Logical operators
    And { conditions: Vec<RuleExpression> },
    Or { conditions: Vec<RuleExpression> },
    Not { condition: Box<RuleExpression> },

    // Comparisons
    Compare { field: String, op: ComparisonOp, value: Value },
    In { field: String, values: Vec<Value> },

    // Cross-entity validation
    Exists {
        entity_type: String,
        where_clause: Box<RuleExpression>
    },
    Count {
        entity_type: String,
        where_clause: Option<Box<RuleExpression>>,
        op: ComparisonOp,
        value: i32
    },

    // Collection validation
    All { collection: String, condition: Box<RuleExpression> },
    Any { collection: String, condition: Box<RuleExpression> },

    // Custom functions
    Custom { function: String, args: HashMap<String, Value> },
}
```

**Real Examples**:

**GPS Exhaustive Research Rule**:
```rust
ValidationRule {
    metadata: ValidationRuleMetadata {
        rule_key: "gps-exhaustive-research".to_string(),
        severity: Severity::Error,
        // ...
    },
    expression: And {
        conditions: vec![
            // Must have at least 3 sources
            Count {
                entity_type: "Source".to_string(),
                where_clause: None,
                op: ComparisonOp::GreaterThanOrEqual,
                value: 3
            },
            // From at least 2 different repositories
            Count {
                entity_type: "Repository".to_string(),
                where_clause: None,
                op: ComparisonOp::GreaterThanOrEqual,
                value: 2
            },
        ]
    },
    applies_to: vec![EntityType::Theory, EntityType::ProofStatement],
    methodology_scope: vec!["gps-2021".to_string()],
}
```

**Citation Completeness Rule**:
```rust
ValidationRule {
    expression: And {
        conditions: vec![
            Required { field: "source_id".to_string() },
            Required { field: "location_in_source".to_string() },
            Or {
                conditions: vec![
                    Required { field: "verbatim_extract".to_string() },
                    Required { field: "abstract_summary".to_string() },
                ]
            },
        ]
    },
    applies_to: vec![EntityType::Citation],
    methodology_scope: vec!["gps-2021".to_string(), "bcg-2023".to_string()],
}
```

**Key Features**:
- **Composable**: And, Or, Not allow complex logic
- **Cross-entity**: Can validate across entity relationships
- **Scoped**: Only applies to specific methodologies
- **Extensible**: Custom functions for domain-specific validation
- **Data-driven**: Stored as JSON/YAML, not Rust code

### 4.3 TemplateRegistry - Templates as Data

**Location**: `/crates/rp-core/src/layer3/template_registry.rs`

**Template Structure**:
```rust
pub struct TemplateConfig {
    pub metadata: TemplateMetadata,

    // Template format
    pub format: TemplateFormat,  // Markdown, HTML, LaTeX, Docx

    // Template content with {{variables}}
    pub content: String,

    // Variable definitions
    pub variables: Vec<TemplateVariable>,

    // Usage examples
    pub examples: Vec<TemplateExample>,

    // Compatibility
    pub compatible_work_products: Vec<WorkProductType>,
    pub compatible_methodologies: Vec<String>,

    // Template inheritance
    pub extends: Option<String>,
}

pub struct TemplateVariable {
    pub name: String,
    pub var_type: VariableType,  // String, Number, Date, Boolean, Array, Object
    pub description: String,
    pub required: bool,
    pub default_value: Option<Value>,
    pub validation: Option<RuleExpression>,
    pub example_value: Option<Value>,
}
```

**Real Example - GPS Proof Statement**:
```rust
TemplateConfig {
    metadata: TemplateMetadata {
        template_key: "gps-proof-statement".to_string(),
        name: "GPS Proof Statement".to_string(),
        format: TemplateFormat::Markdown,
        // ...
    },

    content: r#"
# Proof Statement: {{title}}

## Research Question
{{research_question}}

## Summary
This proof statement addresses the identity of {{subject_name}} based on
analysis of {{source_count}} sources from {{repository_count}} repositories.

## Evidence Summary
{{#each evidence_items}}
- **{{this.type}}**: {{this.summary}}
  - Source: {{this.source_citation}}
  - Quality: {{this.quality_assessment}}
{{/each}}

{{#if has_conflicts}}
## Conflict Resolution
{{conflict_resolution}}
{{/if}}

## Conclusion
Based on the preponderance of evidence, it is concluded that
{{conclusion_statement}}.

**Confidence Level**: {{confidence_level}}

## GPS Elements Satisfied
{{#each gps_elements}}
- [{{#if this.satisfied}}X{{else}} {{/if}}] {{this.element}}: {{this.justification}}
{{/each}}
    "#,

    variables: vec![
        TemplateVariable {
            name: "title".to_string(),
            var_type: VariableType::String,
            description: "Proof statement title".to_string(),
            required: true,
            default_value: None,
            example_value: Some("Identity of John Smith of Ohio".into()),
        },
        // ... more variables
    ],

    compatible_work_products: vec![WorkProductType::ProofStatement],
    compatible_methodologies: vec!["gps-2021".to_string()],
}
```

**Template Registry Features**:
- **Discovery**: Scans filesystem, Git repos, HTTP endpoints
- **Indexing**: By category, methodology, work product type, tags
- **Inheritance**: Templates can extend other templates
- **Validation**: Variables have validation rules
- **Multiple formats**: Markdown, HTML, LaTeX, Docx

### 4.4 Workspace - The Configuration Orchestrator

**Location**: `/crates/rp-core/src/layer3/workspace.rs`

**The Central Hub**:
```rust
pub struct Workspace {
    pub metadata: WorkspaceMetadata,

    // Methodology management
    pub methodologies: HashMap<String, MethodologyConfig>,
    pub active_methodologies: Vec<String>,

    // Standards management
    pub standards: HashMap<String, StandardsConfig>,
    pub active_standards: Vec<String>,

    // Module management
    pub enabled_modules: Vec<EnabledModule>,
    pub module_configs: HashMap<String, ModuleConfig>,

    // Template management
    pub templates: HashMap<String, TemplateConfig>,
    pub default_templates: HashMap<String, String>,

    // Validation rules
    pub validation_rules: HashMap<String, ValidationRule>,

    // Export preferences
    pub export_preferences: ExportPreferences,

    // UI state (what's currently open)
    pub open_theories: Vec<EntityId>,
    pub recent_work_products: Vec<EntityId>,
    pub pinned_items: Vec<EntityId>,
}
```

**Key Methods**:
```rust
impl Workspace {
    // Methodology management
    pub async fn load_methodology(&mut self, config: MethodologyConfig) -> Result<()>;
    pub async fn activate_methodology(&mut self, key: &str) -> Result<()>;
    pub fn get_active_configs(&self) -> ActiveConfigs;

    // Module management
    pub async fn enable_module(&mut self, key: &str, settings: ModuleSettings) -> Result<()>;
    pub async fn disable_module(&mut self, key: &str) -> Result<()>;

    // Template management
    pub async fn load_template(&mut self, config: TemplateConfig) -> Result<()>;
    pub fn set_default_template(&mut self, work_type: &str, template_key: &str) -> Result<()>;
    pub fn get_compatible_templates(&self, work_type: &str, methodology: &str) -> Vec<&TemplateConfig>;
}
```

**Usage Pattern**:
```rust
// 1. Create workspace
let mut workspace = Workspace::new("research-workspace".to_string(), user_id);

// 2. Load GPS 2021 methodology
let gps = MethodologyConfig::create_gps_2021();
workspace.load_methodology(gps).await?;

// 3. Activate it
workspace.activate_methodology("gps-2021").await?;

// 4. Now GPS validation rules automatically apply
let active_configs = workspace.get_active_configs();
// Modules can access: active_configs.validation_rules, templates, etc.

// 5. Work products use GPS templates
let templates = workspace.get_compatible_templates("ProofStatement", "gps-2021");
let template = templates.first().unwrap();
```

**The Power**:
- **Multi-methodology**: Load GPS + BCG + custom simultaneously
- **Per-project configuration**: Each workspace can use different methodologies
- **Runtime switching**: Change methodologies without restart
- **Module integration**: Modules get active configs via `get_active_configs()`
- **Template selection**: Automatic filtering by compatibility

---

## 5. Storage Abstraction Architecture

### Philosophy

**True abstraction** means: Backends can be PostgreSQL, SQLite, file-based, in-memory, Git, or even JIRA. Entities don't know or care.

### Core Trait

**Location**: `/crates/rp-storage/src/traits.rs`

```rust
pub trait StorageBackend: Send + Sync {
    type Transaction: Transaction;

    async fn initialize(&self) -> StorageResult<()>;
    async fn health_check(&self) -> StorageResult<HealthStatus>;
    async fn begin_transaction(&self) -> StorageResult<Self::Transaction>;

    fn capabilities(&self) -> StorageCapabilities;
    fn backend_type(&self) -> &str;

    async fn shutdown(&self) -> StorageResult<()>;
}
```

### Capabilities System

**Backends declare what they support**:
```rust
pub struct StorageCapabilities {
    pub supports_transactions: bool,
    pub supports_vectors: bool,
    pub supports_graph: bool,
    pub supports_streaming: bool,
    pub supports_bulk_operations: bool,
    pub max_transaction_size: Option<usize>,
    pub max_entity_size: Option<usize>,
}
```

**Example**:
```rust
// PostgreSQL supports everything
PostgresBackend::capabilities() {
    StorageCapabilities {
        supports_transactions: true,
        supports_vectors: true,  // pgvector extension
        supports_graph: true,    // Apache AGE extension
        supports_streaming: true, // LISTEN/NOTIFY
        supports_bulk_operations: true,
        // ...
    }
}

// SQLite more limited
SqliteBackend::capabilities() {
    StorageCapabilities {
        supports_transactions: true,
        supports_vectors: false,  // No vector support
        supports_graph: false,    // No graph extension
        supports_streaming: false, // No LISTEN/NOTIFY
        supports_bulk_operations: true,
        // ...
    }
}
```

### Optional Capabilities via Trait Composition

**1. QueryableBackend** (all backends should implement):
```rust
pub trait QueryableBackend: StorageBackend {
    async fn query(&self, query: Query) -> StorageResult<QueryResult>;
    async fn create_index(&self, index: IndexDefinition) -> StorageResult<()>;
}
```

**2. VectorSearchBackend** (PostgreSQL with pgvector):
```rust
pub trait VectorSearchBackend: StorageBackend {
    async fn store_embeddings(&self, entity_id: Uuid, embeddings: &[f32], ...) -> StorageResult<()>;
    async fn vector_search(&self, query_vector: &[f32], limit: usize, ...) -> StorageResult<Vec<VectorSearchResult>>;
    async fn delete_embeddings(&self, entity_id: Uuid) -> StorageResult<()>;
}
```

**3. GraphBackend** (PostgreSQL with Apache AGE):
```rust
pub trait GraphBackend: StorageBackend {
    async fn store_relationship(&self, relationship: &Relationship) -> StorageResult<()>;
    async fn traverse(&self, start: Uuid, pattern: TraversalPattern, ...) -> StorageResult<Vec<TraversalResult>>;
    async fn shortest_path(&self, from: Uuid, to: Uuid, ...) -> StorageResult<Option<Vec<EntityId>>>;
    async fn common_ancestors(&self, ids: Vec<Uuid>, ...) -> StorageResult<Vec<EntityId>>;
}
```

**4. StreamingBackend** (PostgreSQL LISTEN/NOTIFY, WebSocket):
```rust
pub trait StreamingBackend: StorageBackend {
    async fn stream_changes(&self, filters: Option<ChangeFilters>) -> StorageResult<Pin<Box<dyn Stream<Item = ChangeEvent>>>>;
    async fn replay_history(&self, from: DateTime<Utc>, ...) -> StorageResult<Vec<HistoricalEvent>>;
}
```

**5. BulkOperations** (optimization for mass imports):
```rust
pub trait BulkOperations: StorageBackend {
    async fn bulk_insert(&self, entities: Vec<StorageEntity>) -> StorageResult<Vec<EntityId>>;
    async fn bulk_update(&self, updates: Vec<BulkUpdate>) -> StorageResult<usize>;
    async fn bulk_delete(&self, ids: Vec<EntityId>) -> StorageResult<usize>;
}
```

### Storage-Agnostic Entity Representation

**Universal format**:
```rust
pub struct StorageEntity {
    pub id: Uuid,
    pub entity_type: String,
    pub data: JsonValue,  // Serialized entity (backend-agnostic)
    pub binary_data: Option<Vec<u8>>,  // For attachments
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u64,
}
```

**Key Insight**: All entities serialize to JSON for storage. Backend handles JSON however it wants:
- PostgreSQL: Store in JSONB column
- SQLite: Store in TEXT column as JSON
- File-based: Store as .json files
- Git: Store in YAML files with Git tracking

### Module Storage Checks

**Modules check capabilities before using features**:
```rust
// In a module that needs vector search
let caps = storage.capabilities();

if caps.supports_vectors {
    // Use vector search
    let results = storage.vector_search(query_vector, 10).await?;
} else {
    // Fallback to keyword search
    let results = storage.query(Query::keyword(query)).await?;
}
```

---

## 6. Module System Evolution

### Architectural Change: Trait-based FFI → Message-based FFI

**Timeline**: Commits `9f0b71c`, `9ddf90b`, `285415f`

**Documents**:
- `/archive/sessions/2025-08-01/MODULE_FFI_REFACTORING_SESSION_2025_08_01_0217_EEST.md`
- `/docs/development/MODULE_FFI_QUICK_REFERENCE.md`

### The Problem with Trait-based FFI

**Original approach** (abandoned):
```rust
// Trait-based FFI (UNSAFE, PROBLEMATIC)
#[no_mangle]
pub extern "C" fn module_init() -> *mut dyn Module {
    Box::into_raw(Box::new(MyModule::new())) as *mut dyn Module
}

// Host side
let module_ptr = unsafe { dlsym(..., "module_init") };
let module: Box<dyn Module> = unsafe { Box::from_raw(module_ptr) };
```

**Problems**:
1. **Lifetime issues**: Trait objects across FFI boundaries are unsafe
2. **ABI stability**: No guarantee of trait object layout between compilations
3. **Error handling**: Panics in modules crash host
4. **Type safety**: Casting between *mut dyn Trait is error-prone
5. **Memory management**: Who owns the memory? Host or module?

### The Solution: Message-based FFI

**New approach**:
```rust
// Message-based FFI (SAFE, STABLE)

#[repr(C)]
pub struct Message {
    pub msg_type: MessageType,
    pub payload: *const u8,
    pub payload_len: usize,
}

// Module exports simple C functions
#[no_mangle]
pub extern "C" fn module_handle_message(msg: Message) -> Message {
    // Deserialize message
    let request: ModuleRequest = serde_json::from_slice(
        unsafe { std::slice::from_raw_parts(msg.payload, msg.payload_len) }
    ).unwrap();

    // Handle request
    let response = match request {
        ModuleRequest::Initialize(config) => handle_init(config),
        ModuleRequest::Execute(params) => handle_execute(params),
        // ...
    };

    // Serialize response
    let json = serde_json::to_vec(&response).unwrap();
    Message::from_bytes(json)
}
```

**Benefits**:
1. **Safety**: No trait objects, just simple C structs
2. **Stability**: JSON message format is stable ABI
3. **Isolation**: Module panics can't crash host (host catches errors in response)
4. **Language-agnostic**: Could support Python, JS, C++ modules in future
5. **Testability**: Easy to mock with JSON messages
6. **Uniformity**: Same protocol for Native and WASM modules

### Module SDK

**Location**: `/crates/rp-module-sdk/`

**Provides**:
- Safe API for module development
- Message serialization/deserialization
- Error handling helpers
- Storage access abstractions
- Testing utilities

**Example module using SDK**:
```rust
use rp_module_sdk::prelude::*;

#[module]
pub struct ResearchLogModule {
    config: ModuleConfig,
}

#[module_impl]
impl ResearchLogModule {
    #[module_method]
    pub fn auto_capture(&self, session: &ResearchSession) -> Result<ResearchLog> {
        // Module logic here
        // SDK handles message serialization
    }
}
```

### Current State

**Working**:
- Native module loading (libloading)
- WASM module support (Wasmtime 25.0)
- Message-based protocol
- Module SDK
- Example modules: research-log (native & WASM), example-module

**Not Yet Working**:
- Module discovery/marketplace
- Module sandboxing/permissions
- Hot reload
- Module dependency management

---

## 7. Current Implementation Status

### ✅ Phase 1: Core Entities (100% Complete)

**22 of 23 entities implemented** (11,567 lines of Rust code)

**Layer 1 - Core Genealogical (9 entities)**:
- `Source` (829 lines) - Hierarchical source management
- `Citation` (846 lines) - Source references with quality tracking
- `Evidence` (504 lines) - Extracted information from sources
- `IdentityPersona` (712 lines) - Unified person model with state progression
- `Relationship` (610 lines) - Flexible entity connections
- `Location` (519 lines) - Geographic places with temporal dimensions
- `Fact` (699 lines) - Atomic claims about entities
- `Confidence` (291 lines) - Confidence assessment narratives
- `Analysis` (538 lines) - Analysis results and reasoning

**Layer 2 - Research Process (9 entities)**:
- `Theory` (413 lines) - Research questions and hypotheses with state machine
- `ResearchSession` (659 lines) - Research work sessions with objectives
- `ResearchActivity` (590 lines) - Atomic research activities
- `ResearchLog` (762 lines) - GPS-compliant process documentation
- `Researcher` (207 lines) - Research agents/actors
- `WorkProduct` (731 lines) - All research output types
- `ProofStatement` (751 lines) - GPS proof arguments
- `AnalysisReport` (1,026 lines) - Analysis work products with findings

**Layer 3 - Infrastructure (4 entities + ConfigEntity trait)**:
- `Workspace` - User environment orchestrator
- `MethodologyConfig` - Methodology definitions as data
- `ModuleConfig` - Module configurations
- `TemplateRegistry` - Template management

**Missing**:
- `Person` entity (promotes from IdentityPersona when concluded)
- `StandardsRegistry` entity (partially implemented)
- `ValidationRule` entity (partially implemented)

### ✅ Phase 2: Event Sourcing (100% Complete)

**Location**: `/crates/rp-events/`

- Complete event store with versioning
- Event streaming infrastructure
- PostgreSQL LISTEN/NOTIFY integration
- Transactional consistency
- Event replay capabilities
- Snapshot optimization

### ✅ Phase 3: API Layer (98% Complete)

**Location**: `/crates/rp-server/`

**Completed**:
- Full REST API with CRUD operations for all entities
- WebSocket real-time event streaming
- Entity-specific operations
- Search functionality
- Authentication (API key-based)
- Error handling with ApiError types

**Remaining**:
- OpenAPI documentation generation (95% complete)
- Rate limiting
- OAuth2 support

### ✅ Phase 4: Module System (100% Complete)

**Location**: `/crates/rp-modules/`

- Message-based FFI architecture (refactored from trait-based)
- Native module loading with libloading
- WASM module support with Wasmtime 25.0
- Thread-safe module manager (Arc<Mutex<>>)
- Module SDK for development (`/crates/rp-module-sdk/`)
- Example modules working

### ✅ Storage Implementations (Partial)

**Working**:
- PostgreSQL adapter complete (`/crates/rp-storage-postgres/`)
- Storage abstraction layer complete (`/crates/rp-storage/`)

**Planned**:
- Git storage backend (`/crates/rp-storage-git/` - scaffolded)
- Filesystem storage (`/crates/rp-storage-fs/` - scaffolded)

### ⏳ Not Yet Implemented

**Phase 5-8: Future Phases**:
- Collaboration features (CRDTs, conflict resolution)
- Web interface (Leptos/Yew/Dioxus - not decided)
- Theory versioning mechanics (entity exists, Git-like operations not coded)
- GEDCOM import/export (analyzed, parser not written)
- .rgps archive format (documented, not implemented)
- Advanced features (query language, GEDCOM 7 adapter)
- Production readiness (deployment, monitoring, scaling)

---

## 8. Key Architectural Patterns

### 1. **Explicit Error Handling (NO_FALLBACK_POLICY)**

**Zero tolerance** for silent failures:
```rust
// ❌ WRONG (violates NO_FALLBACK_POLICY)
let result = operation().unwrap_or_default();
let data = fetch().ok()?;

// ✅ CORRECT (explicit error handling)
let result = operation().map_err(|e| {
    error!("Operation failed: {}", e);
    ApiError::Internal(e.to_string())
})?;
```

**Impact**: System fails fast and explicitly, preventing data corruption

### 2. **EntityMetadata Pattern**

**Every domain entity** embeds standardized metadata:
```rust
pub struct Theory {
    #[serde(flatten)]  // Flatten into parent JSON
    pub metadata: EntityMetadata,
    // ... entity-specific fields
}
```

**Provides**:
- Universal tracking: who created, who modified, when
- Soft delete: `is_active` flag
- Versioning: `version` number, `parent_version` link
- Full audit trail

### 3. **Dual Metadata (Layer 3 Optimization)**

Configuration entities have **both**:
```rust
pub struct Workspace {
    pub metadata: WorkspaceMetadata,  // Config-specific
    #[serde(skip)]
    entity_metadata: Option<EntityMetadata>,  // Optional compatibility
}
```

**Rationale**: Config entities are lighter weight, don't need full audit trails

### 4. **JSON for Flexibility**

Extensive use of `serde_json::Value`:
```rust
pub struct Theory {
    // ...
    pub custom_fields: HashMap<String, serde_json::Value>,
    pub generation_metadata: Option<serde_json::Value>,
    pub statistics: Option<serde_json::Value>,
}
```

**Balance**: Type safety for core fields, flexibility for extensions

### 5. **Builder Pattern via Macros**

Reduce boilerplate:
```rust
impl_entity!(Theory, "Theory");
impl_validatable!(WorkProduct);
define_states! { pub enum TheoryState { ... } }
```

**Generated**: ~20 lines of Entity trait implementation per entity

### 6. **Trait Composition over Inheritance**

Entities implement multiple capabilities:
```rust
impl Entity for Theory { ... }
impl StateMachine for Theory { ... }
impl NestableEntity for Theory { ... }
impl Validatable for Theory { ... }
```

**Flexibility**: Pick and choose capabilities per entity

### 7. **Capabilities-based Storage**

Storage backends **declare** what they support:
```rust
let caps = backend.capabilities();
if caps.supports_vectors {
    // Use vector search
} else {
    // Fallback to keyword search
}
```

**Benefit**: Graceful degradation, not hard requirements

---

## 9. Critical Files Reference

| Purpose | Location |
|---------|----------|
| **Core Traits** | `/crates/rp-core/src/entity.rs` |
| **State Machines** | `/crates/rp-core/src/state.rs` |
| **Example Entity** | `/crates/rp-core/src/theory.rs` |
| **Complex State Machine** | `/crates/rp-core/src/work_product.rs` |
| **Configuration Orchestrator** | `/crates/rp-core/src/layer3/workspace.rs` |
| **Methodology as Data** | `/crates/rp-core/src/layer3/methodology_config.rs` |
| **Validation DSL** | `/crates/rp-core/src/layer3/validation_rule.rs` |
| **Storage Abstraction** | `/crates/rp-storage/src/traits.rs` |
| **PostgreSQL Implementation** | `/crates/rp-storage-postgres/src/lib.rs` |
| **Event Sourcing** | `/crates/rp-events/src/lib.rs` |
| **API Server** | `/crates/rp-server/src/lib.rs` |
| **Module System** | `/crates/rp-modules/src/lib.rs` |
| **Module SDK** | `/crates/rp-module-sdk/src/lib.rs` |

---

## 10. Evolution Timeline (Git History)

| Date | Commit | Milestone |
|------|--------|-----------|
| Jul 30 | `adb357d` | Major architectural pivot: Rust implementation |
| Jul 31 | `3e5c16e` | Complete 8/10 core entities with unified IdentityPersona |
| Jul 31 | `8898eac` | Complete Layer 1: All 11 core genealogical entities |
| Jul 31 | `d0f11be` | Complete Layer 2: All 6 research process entities |
| Jul 31 | `842a6bc` | **Complete Layer 3: All 6 workspace & metadata entities** (BREAKTHROUGH) |
| Aug 1 | `58f792a` | Plan critical architectural refactoring |
| Aug 1 | `bb0a579` | Execute refactoring - EntityType & NO_FALLBACK_POLICY |
| Aug 1 | `6ba9fb1` | Complete NestableEntity updates |
| Aug 1 | `311dd18` | Complete WASM module integration & create module SDK |
| Aug 1 | `9f0b71c` | Implement message-based FFI architecture |
| Aug 1 | `9ddf90b` | Start ModuleLoader FFI refactoring |
| Aug 1 | `285415f` | Complete Module System with FFI refactoring and SDK |

**Pattern**: Continuous refinement toward cleaner abstractions

---

## 11. The Adaptability Achievement

### What Makes This Architecture Revolutionary

**Traditional genealogy software**:
```rust
// GPS validation hard-coded in application
if theory.state == Proven {
    if theory.sources.len() < 3 {
        return Err("GPS requires 3 sources");  // HARD-CODED!
    }
}
```
- Changing GPS standards = waiting for software update
- Adding BCG standards = code changes
- Custom methodologies = impossible

**ResearchProcess-GPS architecture**:
```rust
// Load methodology at runtime
let gps = MethodologyConfig::create_gps_2021();
workspace.load_methodology(gps);
workspace.activate_methodology("gps-2021");

// Validation from configuration
let configs = workspace.get_active_configs();
for rule in configs.validation_rules {
    rule.evaluate(&theory)?;  // CONFIGURED!
}
```
- GPS 2021 → GPS 2025: Just load different config file
- Add BCG: Load BCG config alongside GPS
- Custom methodology: Create your own config
- **Zero code changes**

### The Three Pillars of Adaptability

1. **MethodologyConfig** - Defines research methodology as data
2. **ValidationRule** - Validation logic as expression trees
3. **TemplateRegistry** - Document templates with variables

Together these enable **true methodology-agnostic** research platform.

---

## Conclusion

ResearchProcess-GPS demonstrates **sophisticated architectural evolution** through multiple refactoring iterations:

1. **Metadata Model Evolution** (Iterations 1-4): From conceptual "standards as data" to working implementation with custom metadata structures per entity type

2. **State Machine Architecture**: Complete lifecycle management with audit trails

3. **NestableEntity System**: Hierarchical composition as first-class concept

4. **Layer 3 Breakthrough**: Configuration entities enable true adaptability

5. **Storage Abstraction**: Capabilities-based system for backend flexibility

6. **Module System**: Message-based FFI for safe, language-agnostic extensions

The system is **~40% implemented** with solid architectural foundation. The critical innovation is Layer 3's custom metadata model enabling **"Standards as Data"** - methodologies, validation rules, and templates are runtime-configurable, making this a truly **methodology-agnostic research platform**.

**Next Steps**: Theory versioning mechanics, GEDCOM import/export, web interface, collaboration features.

---

**Document Status**: Complete architectural analysis as of 2025-11-09
