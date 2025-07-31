# Layer 3 Implementation Guide - ResearchProcess-GPS
## Workspace & Metadata Model Architecture
### Created: 2025-07-31

---

## 🎯 Executive Summary

Layer 3 represents a fundamental paradigm shift in ResearchProcess-GPS architecture. Unlike Layers 1 and 2 which define **what** data to store and **how** to track research processes, Layer 3 defines **configuration infrastructure** that makes the entire system adaptable to any research methodology without code changes.

**Core Principle**: "Standards as Data, Not Code"

---

## 📐 Layer 3 Architecture Overview

### Fundamental Differences from Other Layers

| Aspect | Layer 1 | Layer 2 | Layer 3 |
|--------|---------|---------|---------|
| **Focus** | Genealogical data | Research process | Configuration & infrastructure |
| **Nature** | Data entities | Workflow entities | Meta-configuration entities |
| **Purpose** | Store research data | Track research activities | Control system behavior |
| **Examples** | Theory, Evidence, IdentityPersona | WorkProduct, ResearchLog | Workspace, MethodologyConfig |
| **State** | Research states (e.g., Exploring→Testing) | Process states (e.g., Draft→Published) | Configuration states (e.g., Active→Migrating) |

### Layer 3's Revolutionary Impact

1. **Methodology Agnostic**: GPS, BCG, FAN, or custom methodologies become configurations
2. **Dynamic Validation**: Rules loaded from YAML/JSON, not hard-coded
3. **Template-Driven**: Work products defined by templates, not code
4. **Module System**: Pluggable architecture for extensibility
5. **Multi-Workspace**: Parallel research with different methodologies

---

## 🏗️ Layer 3 Entity Specifications

### 1. Workspace Entity (✅ IMPLEMENTED)

**Purpose**: User's configured research environment

**Key Design Decisions**:
- **NOT** a simple container - it's an active configuration context
- **NOT** just user preferences - it defines methodology enforcement
- **HAS** state machine for migration and archival workflows
- **SUPPORTS** nested workspaces for complex projects

**Unique Characteristics**:
```rust
pub struct Workspace {
    // Configuration context
    active_methodologies: Vec<String>,      // e.g., ["GPS-2021", "BCG-2023"]
    active_standards: Vec<String>,          // e.g., ["ISO-15489", "NARA-2022"]
    
    // Module management
    enabled_modules: Vec<ModuleConfigRef>,  // Dynamic feature enabling
    
    // Template assignments
    default_templates: HashMap<String, String>, // WorkProductType → TemplateId
    
    // Collaboration
    owner: ResearcherId,
    collaborators: Vec<ResearcherId>,
}
```

**State Machine**: Active → Inactive → Archived → Migrating

---

### 2. MethodologyConfig Entity (📋 PLANNED)

**Purpose**: Defines a research methodology as configuration data

**Key Design Principle**: Methodologies are **data files**, not code

**Structure Design**:
```rust
pub struct MethodologyConfig {
    // Identity
    pub methodology_id: String,        // e.g., "gps-2021"
    pub name: String,                  // e.g., "Genealogical Proof Standard"
    pub version: String,               // e.g., "2021.1"
    pub authority: String,             // e.g., "Board for Certification of Genealogists"
    
    // Workflow definition
    pub workflow_stages: Vec<WorkflowStage>,
    pub required_elements: HashMap<String, RequirementSpec>,
    
    // Validation rules
    pub compliance_rules: Vec<ComplianceRule>,
    pub scoring_rubrics: HashMap<String, ScoringRubric>,
    
    // Work product requirements
    pub work_product_schemas: HashMap<WorkProductType, SchemaDefinition>,
    pub required_work_products: Vec<RequiredWorkProduct>,
    
    // Feature control
    pub enabled_features: HashSet<String>,
    pub disabled_features: HashSet<String>,
}
```

**Loading Mechanism**:
- Loaded from `~/.researchprocess-gps/methodologies/gps-2021.yaml`
- Can reference remote methodology repositories
- Supports inheritance from base methodologies

---

### 3. StandardsRegistry Entity (📋 PLANNED)

**Purpose**: Central registry of all available standards and methodologies

**Key Design Principle**: Discovery and version management

**Structure Design**:
```rust
pub struct StandardsRegistry {
    // Registry management
    pub registry_id: EntityId,
    pub registry_version: String,
    
    // Available standards
    pub standards: HashMap<String, StandardConfig>,
    pub methodologies: HashMap<String, MethodologyRef>,
    
    // Version management
    pub version_compatibility: HashMap<String, VersionSpec>,
    pub migration_paths: Vec<MigrationPath>,
    
    // Discovery
    pub local_sources: Vec<PathBuf>,
    pub remote_sources: Vec<RegistrySource>,
    
    // Active configurations
    pub active_standards: HashSet<String>,
    pub default_methodology: Option<String>,
}
```

**Key Features**:
- Auto-discovery of local methodology files
- Remote registry synchronization
- Version compatibility checking
- Migration path definitions

---

### 4. ModuleConfig Entity (📋 PLANNED)

**Purpose**: Configuration for pluggable modules

**Key Design Principle**: Modules extend functionality without core changes

**Structure Design**:
```rust
pub struct ModuleConfig {
    // Module identity
    pub module_id: String,           // e.g., "com.example.dna-analysis"
    pub module_type: ModuleType,     // Capture, Analysis, Generation, Validation
    pub version: String,
    
    // Capabilities
    pub provides: Vec<Capability>,
    pub requires: Vec<Dependency>,
    
    // Configuration
    pub settings_schema: JsonSchema,
    pub default_settings: HashMap<String, Value>,
    pub user_settings: HashMap<String, Value>,
    
    // Resources
    pub templates: HashMap<String, TemplateRef>,
    pub validators: Vec<ValidatorRef>,
    
    // Lifecycle
    pub auto_load: bool,
    pub load_priority: i32,
}
```

**Module Types**:
- **Capture**: Data entry and import modules
- **Analysis**: Evidence analysis and correlation
- **Generation**: Report and visualization creation
- **Validation**: Compliance and quality checking

---

### 5. TemplateRegistry Entity (📋 PLANNED)

**Purpose**: Manages templates for work products and displays

**Key Design Principle**: Separation of content and presentation

**Structure Design**:
```rust
pub struct TemplateRegistry {
    // Registry metadata
    pub registry_id: EntityId,
    pub last_updated: DateTime<Utc>,
    
    // Template organization
    pub templates: HashMap<String, Template>,
    pub categories: HashMap<String, Vec<String>>,
    
    // Compatibility
    pub methodology_compatibility: HashMap<String, Vec<String>>,
    pub work_product_compatibility: HashMap<WorkProductType, Vec<String>>,
    
    // Sources
    pub template_sources: Vec<TemplateSource>,
    pub custom_templates: HashMap<String, CustomTemplate>,
}

pub struct Template {
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub format: TemplateFormat,      // Markdown, HTML, LaTeX, etc.
    pub schema: JsonSchema,          // Variables the template expects
    pub content: String,             // The actual template
    pub examples: Vec<TemplateExample>,
}
```

**Template Format Support**:
- Markdown with variable substitution
- HTML with Handlebars/Liquid
- LaTeX for academic output
- DOCX via template files

---

### 6. ValidationRule Entity (📋 PLANNED)

**Purpose**: Configurable validation rules as data

**Key Design Principle**: Rules are data expressions, not code

**Structure Design**:
```rust
pub struct ValidationRule {
    // Rule identity
    pub rule_id: String,
    pub rule_type: RuleType,         // RequiredField, Format, Compliance, Custom
    pub category: String,            // GPS, BCG, DataQuality, etc.
    
    // Rule definition
    pub condition: RuleExpression,   // JSON-based rule language
    pub parameters: HashMap<String, ParameterDef>,
    
    // Execution context
    pub applies_to: Vec<EntityType>,
    pub methodology_scope: Vec<String>,
    
    // Response
    pub severity: Severity,          // Error, Warning, Info
    pub message_template: String,
    pub remediation_hint: Option<String>,
}

pub enum RuleExpression {
    // Simple field checks
    Required { field: String },
    Pattern { field: String, regex: String },
    
    // Complex conditions
    And(Vec<RuleExpression>),
    Or(Vec<RuleExpression>),
    Not(Box<RuleExpression>),
    
    // Comparative rules
    GreaterThan { field: String, value: Value },
    Between { field: String, min: Value, max: Value },
    
    // Cross-entity rules
    Exists { entity_type: String, condition: Box<RuleExpression> },
    Count { entity_type: String, condition: Box<RuleExpression>, operator: CompOp, value: i32 },
}
```

**Rule Language Examples**:
```json
{
  "rule_type": "compliance",
  "condition": {
    "and": [
      {"exists": {"entity": "Evidence", "where": {"type": "primary_source"}}},
      {"count": {"entity": "Citation", "min": 3}},
      {"required": {"field": "proof_statement.gps_elements"}}
    ]
  }
}
```

---

## 🔧 Implementation Patterns for Layer 3

### 1. Configuration Loading Pattern

```rust
impl MethodologyConfig {
    /// Load from YAML file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
    
    /// Load from registry
    pub fn from_registry(registry: &StandardsRegistry, id: &str) -> Result<Self> {
        registry.get_methodology(id)
            .ok_or_else(|| Error::MethodologyNotFound(id.to_string()))
    }
}
```

### 2. Dynamic Validation Pattern

```rust
impl ValidationRule {
    /// Execute rule against an entity
    pub fn evaluate(&self, entity: &dyn Entity, context: &ValidationContext) -> ValidationResult {
        match &self.condition {
            RuleExpression::Required { field } => {
                // Use reflection or visitor pattern to check field
            },
            RuleExpression::And(rules) => {
                // Evaluate all sub-rules
            },
            // ... other rule types
        }
    }
}
```

### 3. Template Rendering Pattern

```rust
impl Template {
    /// Render template with data
    pub fn render(&self, data: &TemplateData) -> Result<String> {
        match self.format {
            TemplateFormat::Markdown => {
                // Use handlebars or similar
            },
            TemplateFormat::Html => {
                // Use liquid or similar
            },
            // ... other formats
        }
    }
}
```

### 4. Module Loading Pattern

```rust
impl ModuleConfig {
    /// Load and initialize module
    pub async fn load_module(&self, context: &ModuleContext) -> Result<Box<dyn Module>> {
        match self.module_type {
            ModuleType::Native => {
                // Load Rust dynamic library
            },
            ModuleType::Wasm => {
                // Load WebAssembly module
            },
            ModuleType::Script => {
                // Load script-based module
            },
        }
    }
}
```

---

## 🔄 Integration with Layers 1 & 2

### How Layer 3 Controls Other Layers

1. **Workspace Context**
   ```rust
   impl WorkProduct {
       fn validate_in_workspace(&self, workspace: &Workspace) -> Result<()> {
           for methodology in &workspace.active_methodologies {
               let config = MethodologyConfig::load(methodology)?;
               config.validate_work_product(self)?;
           }
           Ok(())
       }
   }
   ```

2. **Dynamic Validation**
   ```rust
   impl Theory {
       fn apply_validation_rules(&self, rules: &[ValidationRule]) -> ValidationResult {
           let mut results = Vec::new();
           for rule in rules {
               if rule.applies_to.contains(&EntityType::Theory) {
                   results.push(rule.evaluate(self));
               }
           }
           ValidationResult::aggregate(results)
       }
   }
   ```

3. **Template-Based Generation**
   ```rust
   impl ProofStatement {
       fn generate_document(&self, workspace: &Workspace) -> Result<String> {
           let template_id = workspace.default_templates
               .get("proof_statement")
               .ok_or(Error::NoDefaultTemplate)?;
           
           let template = TemplateRegistry::get(template_id)?;
           template.render(&self.to_template_data())
       }
   }
   ```

---

## 📊 Layer 3 State Machines

### Workspace State Machine
```
┌─────────┐     ┌──────────┐     ┌──────────┐
│ Active  │────▶│ Inactive │────▶│ Archived │
└─────────┘     └──────────┘     └──────────┘
     │                                  │
     └──────────────────────────────────┼─────┐
                                        ▼     │
                                  ┌───────────┐│
                                  │ Migrating ││
                                  └───────────┘▼
                                        │ Active
                                        └──────┘
```

### Module State Machine
```
┌─────────────┐     ┌────────┐     ┌────────┐
│ Uninstalled │────▶│ Loaded │────▶│ Active │
└─────────────┘     └────────┘     └────────┘
                         │               │
                         ▼               ▼
                    ┌─────────┐     ┌────────┐
                    │ Failed  │     │ Paused │
                    └─────────┘     └────────┘
```

---

## 🚀 Implementation Priority

### Phase 1: Core Infrastructure (Current)
1. ✅ Workspace - Configuration context
2. 🎯 MethodologyConfig - Standards as data
3. 📋 StandardsRegistry - Discovery and management

### Phase 2: Extension Infrastructure
4. 📋 ModuleConfig - Plugin system
5. 📋 TemplateRegistry - Document generation
6. 📋 ValidationRule - Dynamic validation

### Phase 3: Integration
- Cross-layer validation
- Template rendering pipeline
- Module loading system
- Configuration UI

---

## 💡 Key Implementation Insights

### 1. Entity Structure Differences

**Layers 1 & 2**: Focus on domain data
```rust
pub struct Theory {
    pub question: String,        // Domain data
    pub evidence: Vec<EntityId>, // Domain relationships
    pub state: TheoryState,      // Domain state
}
```

**Layer 3**: Focus on configuration
```rust
pub struct MethodologyConfig {
    pub rules: Vec<ValidationRule>,      // Behavior configuration
    pub templates: Vec<TemplateRef>,     // Display configuration
    pub workflows: Vec<WorkflowStage>,   // Process configuration
}
```

### 2. State Machine Differences

- **Layer 1**: Research states (Exploring → Testing → Concluded)
- **Layer 2**: Process states (Draft → Review → Published)
- **Layer 3**: Configuration states (Active → Migrating → Updated)

### 3. Validation Approach

- **Layers 1 & 2**: Validate data integrity
- **Layer 3**: Validate configuration consistency AND enforce configured rules on Layers 1 & 2

---

## 🎯 Success Criteria

1. **Configuration-Driven**: All GPS/BCG rules loadable from YAML/JSON
2. **Multi-Methodology**: Support multiple active methodologies
3. **Extensible**: Third-party modules without core changes
4. **Backwards Compatible**: Existing Layer 1 & 2 code continues working
5. **Performance**: Configuration caching for efficiency

---

## 📝 Next Steps

1. Fix Workspace entity to properly implement Layer 3 patterns
2. Implement MethodologyConfig with YAML loading
3. Create example GPS and BCG configuration files
4. Implement StandardsRegistry with discovery
5. Design module loading architecture
6. Create template examples

---

*This guide ensures future sessions understand Layer 3's unique architecture and implementation requirements.*