# Workspace and Metadata Infrastructure - ResearchProcess-GPS
## 2025-07-30 21:10 EEST

## Overview

This document defines the metadata infrastructure that enables configurable standards, methodologies, and tool modules. This is the "configuration layer" that makes ResearchProcess-GPS adaptable to any genealogical methodology.

## Core Metadata Infrastructure

### 1. Workspace

```python
@dataclass
class Workspace(NestableBaseEntity['Workspace']):
    """
    User's configured research workspace.
    Defines available tools and active standards.
    Can nest for project/sub-project organization.
    """
    # Workspace identity
    name: str
    description: str
    workspace_type: WorkspaceType = WorkspaceType.PERSONAL
    # PERSONAL, TEAM, INSTITUTIONAL, PROJECT
    
    # Active configurations
    active_methodologies: List[str] = field(default_factory=list)
    # ["GPS-2025", "BCG-3.4", "FAN-PRINCIPLE"]
    
    active_standards: List[str] = field(default_factory=list)
    # ["EVIDENCE-EXPLAINED", "CHICAGO-MANUAL", "ISO-8601"]
    
    # Tool/module enablement
    enabled_modules: List[str] = field(default_factory=list)
    # ["AUTO_CAPTURE", "EVIDENCE_ANALYSIS", "PROOF_GENERATION"]
    
    module_configs: Dict[str, ModuleConfig] = field(default_factory=dict)
    
    # User preferences
    preferences: WorkspacePreferences = field(default_factory=WorkspacePreferences)
    
    # Default configurations
    default_templates: Dict[str, str] = field(default_factory=dict)
    # {
    #   "research_log": "BCG_STANDARD_LOG_v3",
    #   "proof_statement": "GPS_COMPLIANT_PROOF_v2"
    # }
    
    # Workspace state
    open_theories: List[UUID] = field(default_factory=list)
    active_research_logs: List[UUID] = field(default_factory=list)
    recent_work_products: List[UUID] = field(default_factory=list)
    
    # Ownership and access
    owner_id: UUID  # Researcher ID who owns workspace
    shared_with: List[UUID] = field(default_factory=list)  # Researcher IDs
    permissions: Dict[UUID, WorkspacePermission] = field(default_factory=dict)
    # {researcher_id: WorkspacePermission.READ_WRITE}
    
    # Validation rules
    validation_policies: Dict[str, ValidationPolicy] = field(default_factory=dict)
    
    # Export configurations
    export_configs: Dict[str, ExportConfig] = field(default_factory=dict)
    
    def apply_methodology(self, methodology: str) -> None:
        """Apply a methodology to workspace"""
        if methodology not in self.active_methodologies:
            self.active_methodologies.append(methodology)
            self._load_methodology_defaults(methodology)
    
    def validate_entity(self, entity: BaseEntity) -> ValidationResult:
        """Validate entity against workspace rules"""
        results = []
        for methodology in self.active_methodologies:
            result = self._validate_against_methodology(entity, methodology)
            results.append(result)
        return ValidationResult.combine(results)
```

### 2. MethodologyConfig

```python
@dataclass
class MethodologyConfig(BaseEntity):
    """
    Configuration for research methodologies.
    Loaded from YAML/JSON files.
    """
    # Identity
    methodology_name: str  # "GPS", "BCG", "FAN"
    version: str
    authority: str  # "Board for Certification of Genealogists"
    
    # Metadata
    description: str
    documentation_url: str
    last_updated: datetime
    
    # Workflow definition
    workflow_stages: List[WorkflowStage] = field(default_factory=list)
    
    # Requirements
    required_elements: Dict[str, RequirementSpec] = field(default_factory=dict)
    # {
    #   "reasonably_exhaustive_research": RequirementSpec(...),
    #   "complete_citations": RequirementSpec(...),
    #   "conflict_resolution": RequirementSpec(...)
    # }
    
    # Validation rules
    compliance_rules: List[ComplianceRule] = field(default_factory=list)
    
    # Work product requirements
    required_work_products: Dict[str, WorkProductRequirement] = field(default_factory=dict)
    # {
    #   "research_log": {"required": True, "template": "BCG_LOG"},
    #   "proof_statement": {"required": True, "when": "conclusion"}
    # }
    
    # Tool/feature enablement
    enabled_features: List[str] = field(default_factory=list)
    disabled_features: List[str] = field(default_factory=list)
    required_modules: List[str] = field(default_factory=list)
    
    # Templates and schemas
    templates: Dict[str, str] = field(default_factory=dict)  # type -> template_id
    schemas: Dict[str, str] = field(default_factory=dict)   # type -> schema_id
    
    # Scoring and metrics
    scoring_rubric: Optional[ScoringRubric] = None
    compliance_thresholds: Dict[str, float] = field(default_factory=dict)
```

### 3. StandardsRegistry

```python
@dataclass
class StandardsRegistry(BaseEntity):
    """
    Central registry of available standards and methodologies.
    """
    # Available standards
    standards: Dict[str, StandardConfig] = field(default_factory=dict)
    methodologies: Dict[str, MethodologyConfig] = field(default_factory=dict)
    
    # Currently loaded
    loaded_standards: Set[str] = field(default_factory=set)
    loaded_methodologies: Set[str] = field(default_factory=set)
    
    # Version management
    version_compatibility: Dict[str, VersionSpec] = field(default_factory=dict)
    # {
    #   "GPS-2025": {"min_engine": "1.0", "max_engine": "2.0"},
    #   "BCG-3.4": {"min_engine": "1.2"}
    # }
    
    # Migration rules
    migration_rules: Dict[str, MigrationRule] = field(default_factory=dict)
    # {"GPS-2020->GPS-2025": MigrationRule(...)}
    
    # Discovery
    standard_sources: List[StandardSource] = field(default_factory=list)
    # [
    #   {"type": "local", "path": "config/standards"},
    #   {"type": "remote", "url": "https://standards.genealogy.org"}
    # ]
    
    def load_standard(self, standard_id: str, version: Optional[str] = None) -> StandardConfig:
        """Load a standard configuration"""
        pass
    
    def check_compatibility(self, standard_id: str) -> bool:
        """Check if standard is compatible with engine"""
        pass
    
    def migrate_data(self, from_standard: str, to_standard: str, data: Any) -> Any:
        """Migrate data between standard versions"""
        pass
```

### 4. ModuleConfig

```python
@dataclass
class ModuleConfig(BaseEntity):
    """
    Configuration for pluggable tool modules.
    """
    # Module identity
    module_id: str
    module_name: str
    module_type: ModuleType
    # CAPTURE, ANALYSIS, GENERATION, VALIDATION, EXPORT
    
    version: str
    author: str
    
    # Module metadata
    description: str
    documentation: str
    license: str
    
    # Capabilities
    capabilities: List[str] = field(default_factory=list)
    # ["AUTO_CAPTURE_BROWSER", "EVIDENCE_CORRELATION", "GPS_VALIDATION"]
    
    supported_entities: List[str] = field(default_factory=list)
    # ["Evidence", "ResearchLog", "ProofStatement"]
    
    # Dependencies
    dependencies: List[ModuleDependency] = field(default_factory=list)
    required_modules: List[str] = field(default_factory=list)
    
    # Configuration
    settings: Dict[str, ModuleSetting] = field(default_factory=dict)
    # {
    #   "auto_capture_interval": ModuleSetting(type="int", default=30),
    #   "capture_screenshots": ModuleSetting(type="bool", default=False)
    # }
    
    # Templates and resources
    templates: Dict[str, TemplateRef] = field(default_factory=dict)
    resources: Dict[str, ResourceRef] = field(default_factory=dict)
    
    # Permissions required
    required_permissions: List[str] = field(default_factory=list)
    # ["READ_EVIDENCE", "CREATE_WORKPRODUCT", "MODIFY_RESEARCHLOG"]
    
    # State
    enabled: bool = True
    load_on_startup: bool = True
    
    def validate_settings(self, settings: Dict[str, Any]) -> ValidationResult:
        """Validate module settings"""
        pass
```

### 5. TemplateRegistry

```python
@dataclass
class TemplateRegistry(BaseEntity):
    """
    Registry of templates for work products and displays.
    """
    # Available templates
    templates: Dict[str, Template] = field(default_factory=dict)
    
    # Organization
    template_categories: Dict[str, List[str]] = field(default_factory=dict)
    # {
    #   "research_logs": ["BCG_STANDARD", "NGS_FORMAT", "CUSTOM_LOG"],
    #   "proof_statements": ["GPS_COMPLIANT", "NARRATIVE_STYLE"]
    # }
    
    # Compatibility
    compatibility_matrix: Dict[str, List[str]] = field(default_factory=dict)
    # {"template_id": ["methodology1", "methodology2"]}
    
    # Template sources
    template_sources: List[TemplateSource] = field(default_factory=list)
    
    def get_templates_for_type(self, 
                              work_product_type: str,
                              methodology: Optional[str] = None) -> List[Template]:
        """Get compatible templates"""
        pass
    
    def validate_template(self, template_id: str, data: Dict[str, Any]) -> ValidationResult:
        """Validate data against template schema"""
        pass
```

### 6. Template

```python
@dataclass
class Template(BaseEntity):
    """
    Template definition for work products.
    """
    # Identity
    template_id: str
    template_name: str
    template_type: str  # "research_log", "proof_statement", etc.
    
    version: str
    author: str
    
    # Metadata
    description: str
    methodology_compliance: List[str] = field(default_factory=list)
    # ["GPS-2025", "BCG-3.4"]
    
    # Schema definition
    schema: Dict[str, Any] = field(default_factory=dict)
    # JSON Schema format
    
    # Sections
    sections: List[TemplateSection] = field(default_factory=list)
    required_sections: List[str] = field(default_factory=list)
    optional_sections: List[str] = field(default_factory=list)
    
    # Field definitions
    fields: Dict[str, FieldDefinition] = field(default_factory=dict)
    
    # Validation
    validation_rules: List[ValidationRule] = field(default_factory=list)
    
    # Formatting
    default_format: str = "markdown"
    supported_formats: List[str] = field(default_factory=list)
    # ["markdown", "html", "pdf", "docx"]
    
    # Examples
    example_data: Optional[Dict[str, Any]] = None
    example_output: Optional[str] = None
    
    def instantiate(self, data: Dict[str, Any]) -> WorkProduct:
        """Create work product from template"""
        pass
```

## Configuration Schemas

### Methodology YAML Example

```yaml
# config/methodologies/GPS-2025.yaml
methodology:
  name: "Genealogical Proof Standard"
  version: "2025"
  authority: "Board for Certification of Genealogists"
  description: "Five elements required for genealogical proof"
  
workflow_stages:
  - name: "research_planning"
    description: "Define research question and plan"
    required_products: ["research_plan"]
    
  - name: "evidence_gathering"
    description: "Reasonably exhaustive research"
    required_products: ["research_log"]
    validation_rules:
      - "check_repository_coverage"
      - "verify_negative_searches"
      
  - name: "evidence_analysis"
    description: "Analyze and correlate evidence"
    required_products: ["evidence_analysis"]
    
  - name: "conclusion"
    description: "Written conclusion with proof"
    required_products: ["proof_statement"]

required_elements:
  reasonably_exhaustive_research:
    description: "Thorough research in all relevant sources"
    validation_rules:
      - rule_type: "coverage"
        parameters:
          vital_records: 0.90
          census_records: 0.85
          
  complete_citations:
    description: "Complete and accurate source citations"
    template_system: "Evidence_Explained"
    required_fields: ["author", "title", "publication", "location"]
    
compliance_rules:
  - rule_id: "gps_element_1"
    element: "reasonably_exhaustive_research"
    validator: "validate_research_coverage"
    
  - rule_id: "gps_element_5"
    element: "written_conclusion"
    validator: "validate_proof_statement"
```

### Module Configuration Example

```yaml
# config/modules/auto_capture.yaml
module:
  id: "auto_capture_browser"
  name: "Browser Auto-Capture"
  type: "CAPTURE"
  version: "1.0.0"
  
capabilities:
  - "CAPTURE_SEARCHES"
  - "CAPTURE_SCREENSHOTS"
  - "EXTRACT_METADATA"
  - "SESSION_RECORDING"
  
supported_sites:
  - pattern: "*.familysearch.org/*"
    extractor: "familysearch_extractor"
  - pattern: "*.ancestry.com/*"
    extractor: "ancestry_extractor"
    
settings:
  capture_interval:
    type: "integer"
    default: 30
    min: 10
    max: 300
    description: "Seconds between capture checks"
    
  auto_extract:
    type: "boolean"
    default: true
    description: "Automatically extract data from pages"
    
dependencies:
  - module: "research_session_manager"
    version: ">=1.0"
```

## Integration Architecture

### Loading Process

```python
class MetadataLoader:
    """Loads and manages metadata configurations"""
    
    def load_workspace(self, workspace_id: str) -> Workspace:
        """Load workspace with all configurations"""
        workspace = self._load_workspace_config(workspace_id)
        
        # Load methodologies
        for methodology_id in workspace.active_methodologies:
            methodology = self.standards_registry.load_methodology(methodology_id)
            workspace.apply_methodology(methodology)
        
        # Load modules
        for module_id in workspace.enabled_modules:
            module = self._load_module_config(module_id)
            workspace.module_configs[module_id] = module
        
        return workspace
    
    def validate_configuration(self, workspace: Workspace) -> ValidationResult:
        """Validate workspace configuration consistency"""
        # Check methodology compatibility
        # Verify module dependencies
        # Validate template availability
        pass
```

### Runtime Usage

```python
class WorkspaceContext:
    """Runtime context for workspace operations"""
    
    def __init__(self, workspace: Workspace):
        self.workspace = workspace
        self.validators = self._load_validators()
        self.generators = self._load_generators()
    
    def create_work_product(self, 
                           product_type: str,
                           theory: Theory) -> WorkProduct:
        """Create work product using workspace configuration"""
        # Get appropriate template
        template_id = self.workspace.default_templates.get(product_type)
        template = self.template_registry.get_template(template_id)
        
        # Generate product
        product = template.instantiate({
            'theory': theory,
            'methodology': self.workspace.active_methodologies[0]
        })
        
        # Validate against methodologies
        for methodology in self.workspace.active_methodologies:
            self.validate_against_methodology(product, methodology)
        
        return product
```

## Benefits of This Architecture

1. **Complete Flexibility**: Any standard or methodology can be supported
2. **No Hard-Coding**: All genealogy-specific rules in configuration
3. **Version Management**: Standards can evolve without breaking data
4. **User Choice**: Researchers choose their preferred methodologies
5. **Institutional Support**: Organizations can mandate specific standards
6. **Tool Modularity**: Features can be enabled/disabled per workspace
7. **Template Variety**: Multiple templates for same work product type
8. **Validation Layers**: Automatic compliance checking

## Next Steps

1. Define JSON schemas for configuration files
2. Create validation rule language specification
3. Design template field definition format
4. Build module dependency resolution
5. Implement configuration migration tools