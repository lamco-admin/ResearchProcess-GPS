# Unified Conceptual Model - ResearchProcess-GPS
## 2025-07-30 20:00 EEST

## Executive Summary

ResearchProcess-GPS represents a revolutionary approach to genealogical data management where **methodologies, standards, and work products become metadata configurations** rather than hard-coded constraints. The system models the entire research process, not just conclusions.

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

## Unified Entity Architecture

### 1. Researcher (Agent/Actor)
```python
class Researcher(NestableBaseEntity):
    """
    Person or organization conducting research.
    Can nest for teams, departments, or hierarchical organizations.
    Similar to GEDCOM X Agent but focused on research attribution.
    """
    # Identity
    name: str
    researcher_type: ResearcherType
    # INDIVIDUAL, TEAM, ORGANIZATION, SYSTEM
    
    # Professional credentials
    credentials: List[Credential] = field(default_factory=list)
    # [
    #   {"type": "CG", "number": "123", "issued": "2020-01-01"},
    #   {"type": "AG", "number": "456", "issued": "2019-01-01"}
    # ]
    
    # Contact information
    emails: List[str] = field(default_factory=list)
    addresses: List[Address] = field(default_factory=list)
    identifiers: Dict[str, str] = field(default_factory=dict)
    # {"ORCID": "0000-0000-0000-0000", "FamilySearch": "XXXX-XXX"}
    
    # Organizational structure (nesting)
    parent_researcher: Optional[UUID] = None  # For team members
    team_members: List[UUID] = field(default_factory=list)
    
    # Roles and permissions
    roles: List[ResearcherRole] = field(default_factory=list)
    # ["LEAD_RESEARCHER", "REVIEWER", "CONTRIBUTOR"]
    
    permissions: Dict[str, PermissionLevel] = field(default_factory=dict)
    # {"Theory.create": "ALLOWED", "Person.delete": "DENIED"}
    
    # Activity tracking
    theories_created: List[UUID] = field(default_factory=list)
    evidence_discovered: List[UUID] = field(default_factory=list)
    assessments_performed: List[UUID] = field(default_factory=list)
    reviews_conducted: List[UUID] = field(default_factory=list)
    
    # Expertise areas
    specializations: List[str] = field(default_factory=list)
    # ["Irish genealogy", "DNA analysis", "18th century records"]
    
    geographic_expertise: List[str] = field(default_factory=list)
    temporal_expertise: List[str] = field(default_factory=list)
    
    # State tracking
    state: ResearcherState = ResearcherState.ACTIVE
    # ACTIVE, INACTIVE, RETIRED, SUSPENDED
```

### 2. Theory (User Term: "Research Question")
```python
class Theory(NestableBaseEntity):
    """
    The central organizing principle - what we're investigating
    Git-like branching for different hypotheses
    """
    # Core question/hypothesis
    question: str
    hypothesis: str
    
    # State progression
    state: TheoryState  # EXPLORING → TESTING → CONCLUDED → QUESTIONED
    
    # Standards compliance tracking
    compliance_configs: Dict[str, ComplianceConfig]
    # {
    #   "GPS-2025": GPSComplianceConfig,
    #   "BCG-3.4": BCGComplianceConfig,
    #   "NGS-RL-2024": ResearchLogConfig
    # }
    
    # Work products generated
    work_products: Dict[str, WorkProduct]
    # {
    #   "research_log": ResearchLog,
    #   "proof_statement": ProofStatement,
    #   "analysis": EvidenceAnalysis
    # }
    
    # Research attribution
    created_by: UUID  # Researcher ID
    created_date: datetime
    contributors: List[UUID] = field(default_factory=list)  # Researcher IDs
    last_modified_by: UUID
    last_modified_date: datetime
```

### 2. IdentityPersona (Dual-Named: Identity OR Persona)
```python
class IdentityPersona(NestableBaseEntity):
    """
    Non-conclusive state: evidence references
    Unlimited nesting for variants and research organization
    """
    state: IdentityState
    # REFERENCE → WORKING → HYPOTHESIS → CANDIDATE
    
    evidence_references: List[EvidenceReference]
    
    # Can become Person when concluded
    promotion_eligible: bool
    promotion_criteria: PromotionConfig  # Configurable standards
    
    # Attribution
    discovered_by: UUID  # Researcher ID
    discovery_date: datetime
```

### 3. Person (Concluded Identity)
```python
class Person(NestableBaseEntity):
    """
    Promoted from IdentityPersona when research concludes
    Additional properties only available after conclusion
    """
    state: PersonState
    # CONCLUDED → ACCEPTED → PUBLISHED → CHALLENGED
    
    source_identities: List[UUID]  # IdentityPersonas that created this
    
    # Only Persons can have confirmed relationships
    confirmed_relationships: List[UUID]
    
    # Can be demoted if new evidence emerges
    demotion_triggers: DemotionConfig
    
    # Attribution
    concluded_by: UUID  # Researcher ID
    conclusion_date: datetime
    reviewed_by: List[UUID] = field(default_factory=list)  # Researcher IDs
```

### 4. Source (Hierarchical)
```python
class Source(NestableBaseEntity):
    """
    Flexible hierarchy supporting all source types
    """
    source_type: SourceType
    # ITEM → SERIES → COLLECTION → REPOSITORY → SYSTEM
    
    # Multi-dimensional quality
    quality_dimensions: SourceQuality
    # {
    #   "originality": Scale,
    #   "contemporaneity": Scale,
    #   "officiality": Scale,
    #   "completeness": float,
    #   "accessibility": Scale
    # }
    
    # Template-based citations
    citation_template: Optional[SourceTemplate]
    template_fields: Dict[str, str]
    
    # Attribution
    added_by: UUID  # Researcher ID
    added_date: datetime
```

### 5. Evidence (First-Class Research Object)
```python
class Evidence(NestableBaseEntity):
    """
    What we extract from sources
    Can be positive, negative, or disproven
    """
    evidence_type: EvidenceType
    # POSITIVE - normal evidence
    # NEGATIVE - absence of expected
    # DISPROVEN - contradicts claims
    
    # Extraction tracking
    extraction: ExtractionRecord
    # {
    #   "method": "manual|OCR|AI",
    #   "original_text": str,
    #   "interpreted_text": str,
    #   "confidence_per_element": Dict
    # }
    
    # Attribution
    extracted_by: UUID  # Researcher ID
    extraction_date: datetime
    verified_by: Optional[UUID] = None  # Researcher ID
```

### 6. Citation (Flexible Relationship)
```python
class Citation(NestableBaseEntity):
    """
    Connects entities to sources with precision
    """
    citation_state: CitationState
    # QUICK → FULL → ELEMENT → ANALYZED
    
    # Flexible connections
    from_entity: UUID
    from_type: str
    to_source: UUID
    
    # Element-level when needed
    cited_elements: List[CitedElement]
    # Each element has original, interpreted, confidence
    
    # Template-driven formatting
    output_formats: Dict[str, str]
    # Multiple styles from same data
    
    # Attribution
    created_by: UUID  # Researcher ID
    creation_date: datetime
```

### 7. Confidence (Revolutionary Container)
```python
class Confidence(NestableBaseEntity):
    """
    Not a score but a complete research narrative
    """
    # Multiple assessments over time
    assessments: List[ConfidenceAssessment]
    
    # Each assessment contains
    class ConfidenceAssessment:
        methodology: str
        analysis: str  # Full explanation
        dimensions: Dict[str, float]  # Multi-dimensional
        assessor_id: UUID  # Researcher ID
        assessor_name: str  # For display
        date: datetime
        peer_reviewed: bool
        peer_reviewers: List[UUID] = field(default_factory=list)  # Researcher IDs
        
        # For negative/disproven
        evidence_type: EvidenceType
        absence_explanation: Optional[str]
        disproof_mechanism: Optional[str]
```

### 8. ResearchLog (Process Documentation)
```python
class ResearchLog(NestableBaseEntity):
    """
    Documents the journey, not the destination
    Auto-captured + manual entries
    """
    log_type: LogType
    # SESSION, PROJECT, REPOSITORY_VISIT
    
    # Configurable schema based on standards
    schema_config: ResearchLogConfig
    # NGS, BCG, custom formats
    
    entries: List[LogEntry]
    # Each tracks what, where, when, results
    
    # Auto-capture integration
    browser_sessions: List[BrowserSession]
    extracted_data: List[ExtractedData]
    
    # Attribution
    maintained_by: UUID  # Primary researcher
    contributors: List[UUID] = field(default_factory=list)  # All who added entries
```

## Methodology as Configuration

### GPS (Genealogical Proof Standard)
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

### Work Product Templates
```yaml
WorkProductTemplates:
  research_log:
    schema: "BCG_Standard"
    auto_capture: enabled
    required_fields: [date, repository, search_params, results]
    
  proof_statement:
    structure:
      - research_question
      - evidence_presentation
      - analysis
      - conflict_resolution
      - conclusion
    gps_compliance_check: automatic
    
  evidence_analysis:
    matrix_format: "Evidence_Explained"
    quality_dimensions: [source, information, evidence]
    correlation_required: true
```

## Research Process Infrastructure

### Core Research Process Entities

#### 1. WorkProduct
```python
class WorkProduct(NestableBaseEntity):
    """
    Base class for all research work products
    Enables tools to generate standard outputs
    """
    product_type: WorkProductType
    # RESEARCH_LOG, PROOF_STATEMENT, ANALYSIS_MATRIX, REPORT
    
    template_id: str  # References configuration
    schema_version: str
    
    # Metadata for tool generation
    generation_metadata: Dict[str, Any]
    validation_rules: List[ValidationRule]
    export_formats: List[str]
```

#### 2. ResearchSession
```python
class ResearchSession(NestableBaseEntity):
    """
    Captures a research session for auto-logging
    """
    session_type: SessionType
    # REPOSITORY_VISIT, ONLINE_SEARCH, ANALYSIS_SESSION
    
    start_time: datetime
    end_time: Optional[datetime]
    
    # Capture metadata
    capture_sources: List[CaptureSource]
    # BROWSER_EXTENSION, API_INTEGRATION, MANUAL_ENTRY
    
    activities: List[ResearchActivity]
    outputs: List[UUID]  # Generated entities
```

#### 3. MethodologyConfig
```python
class MethodologyConfig(BaseEntity):
    """
    Configuration for research methodologies
    Loaded from YAML/JSON, enables tool behavior
    """
    methodology_name: str  # GPS, BCG, FAN, etc.
    version: str
    
    # Workflow definition
    workflow_stages: List[WorkflowStage]
    required_elements: Dict[str, RequirementSpec]
    
    # Validation rules
    compliance_rules: List[ComplianceRule]
    
    # Tool enablement
    enabled_features: List[str]
    disabled_features: List[str]
```

#### 4. StandardsRegistry
```python
class StandardsRegistry(BaseEntity):
    """
    Registry of available standards configurations
    """
    standards: Dict[str, StandardConfig]
    active_standards: List[str]
    
    # Version management
    version_compatibility: Dict[str, VersionSpec]
    migration_rules: Dict[str, MigrationRule]
```

### Workspace Configuration Model

#### 1. Workspace
```python
class Workspace(NestableBaseEntity):
    """
    User's configured research workspace
    Defines available tools and active standards
    """
    # Active configurations
    active_methodologies: List[str]
    active_standards: List[str]
    
    # Tool enablement
    enabled_modules: List[str]
    module_configs: Dict[str, ModuleConfig]
    
    # User preferences
    default_templates: Dict[str, str]
    export_preferences: Dict[str, Any]
    
    # Workspace state
    open_theories: List[UUID]
    active_research_logs: List[UUID]
```

#### 2. ModuleConfig
```python
class ModuleConfig(BaseEntity):
    """
    Configuration for pluggable tool modules
    """
    module_type: ModuleType
    # CAPTURE, ANALYSIS, GENERATION, VALIDATION
    
    # Module metadata
    capabilities: List[str]
    dependencies: List[str]
    
    # Configuration
    settings: Dict[str, Any]
    templates: Dict[str, TemplateRef]
```

### Metadata Infrastructure

#### 1. TemplateRegistry
```python
class TemplateRegistry(BaseEntity):
    """
    Registry of available templates for work products
    """
    templates: Dict[str, Template]
    
    # Template metadata
    template_categories: Dict[str, List[str]]
    compatibility_matrix: Dict[str, List[str]]
```

#### 2. ValidationRule
```python
class ValidationRule(BaseEntity):
    """
    Configurable validation rules
    """
    rule_type: RuleType
    # REQUIRED_FIELD, FORMAT_CHECK, COMPLIANCE_CHECK
    
    rule_spec: Dict[str, Any]
    error_message: str
    severity: str  # ERROR, WARNING, INFO
```

### Process Tracking Infrastructure

#### 1. ResearchActivity
```python
class ResearchActivity(BaseEntity):
    """
    Atomic research activity within a session
    """
    activity_type: ActivityType
    # SEARCH, EXTRACT, ANALYZE, DOCUMENT
    
    timestamp: datetime
    duration: Optional[timedelta]
    
    # Activity details
    target_entity: Optional[UUID]
    parameters: Dict[str, Any]
    results: Dict[str, Any]
```

#### 2. ComplianceStatus
```python
class ComplianceStatus(BaseEntity):
    """
    Tracks compliance with configured standards
    """
    entity_id: UUID
    standard: str
    
    # Compliance details
    elements: Dict[str, ElementStatus]
    overall_score: float
    
    # Validation results
    validation_date: datetime
    validation_details: List[ValidationResult]
```

## The Revolutionary Aspects

### 1. Everything Has States
- Research progresses through defined states
- Can be promoted or demoted
- History maintained
- Reversible decisions

### 2. Standards as Data
- No hard-coded genealogy rules
- Standards loaded as configurations
- Multiple standards simultaneously
- Easy updates as standards evolve

### 3. Process-First Design
- Research journey documented
- Not just conclusions
- Negative evidence explicit
- Thinking captured

### 4. True Flexibility
- Unlimited nesting where sensible
- No artificial relationship constraints
- Cultural adaptability built-in
- Future-proof extensibility

### 5. Professional-Grade Confidence
- Not simple scores
- Complete narratives
- Multi-dimensional assessment
- Peer review integrated

## Data Model Architecture

### Separation of Concerns

#### 1. Genealogical Data Model
- Core entities: Theory, IdentityPersona, Person, Source, Evidence, Citation, Confidence
- Relationships and events
- Nesting and versioning infrastructure

#### 2. Research Process Model
- WorkProduct, ResearchLog, ResearchSession
- Methodology and standards configurations
- Compliance tracking infrastructure

#### 3. Workspace Model
- User preferences and active configurations
- Module enablement and settings
- Tool infrastructure (not tools themselves)

### Core Entity Patterns
```python
# Genealogical entities extend NestableBaseEntity
class GenealogyEntity(NestableBaseEntity):
    # State management
    state: EntityState
    state_history: List[StateTransition]
    
    # Confidence tracking
    confidence: Confidence  # Full container
    
    # Citations
    citations: List[UUID]  # Citation entities
    
    # Research process linkage
    theory_context: Optional[UUID]
    work_products: List[UUID]
    
    # Attribution (every entity tracks who did what)
    created_by: UUID  # Researcher ID
    created_date: datetime
    modified_by: UUID  # Researcher ID  
    modified_date: datetime
    contributors: List[UUID] = field(default_factory=list)  # All researchers

# Research process entities
class ResearchEntity(BaseEntity):
    # Links to genealogical data
    genealogy_refs: List[UUID]
    
    # Methodology compliance
    methodology: str
    compliance_status: ComplianceStatus
    
    # Workspace context
    workspace_id: UUID
    
    # Attribution
    created_by: UUID  # Researcher ID
    created_date: datetime
    last_modified_by: UUID
    last_modified_date: datetime
```

### State Machine Infrastructure
```python
class StateTransitionConfig:
    """
    Configurable state transitions loaded from metadata
    """
    entity_type: str
    transitions: Dict[str, TransitionSpec]
    
    # Loaded from YAML like:
    # Theory:
    #   EXPLORING:
    #     allowed_next: [TESTING, ABANDONED]
    #     requirements: []
    #     validators: []

class TransitionSpec:
    allowed_next: List[str]
    requirements: List[str]
    validators: List[str]
    side_effects: List[str]  # e.g., create_work_product
```

### Metadata-Driven Behavior
```yaml
# Example: config/entities/theory_behaviors.yaml
TheoryBehaviors:
  states:
    EXPLORING:
      description: "Initial research phase"
      allowed_actions: [add_evidence, create_hypothesis]
      required_products: []
      
    TESTING:
      description: "Hypothesis testing phase"
      allowed_actions: [add_evidence, modify_hypothesis]
      required_products: [research_log]
      
    CONCLUDED:
      description: "Research concluded"
      allowed_actions: [publish, revise]
      required_products: [proof_statement, research_log]
      
  transitions:
    EXPLORING_to_TESTING:
      requirements:
        - min_evidence_count: 3
        - hypothesis_defined: true
      validators:
        - validate_evidence_quality
        - check_research_log_started
        
  promotion_rules:
    create_person_from_identity:
      source_entity: IdentityPersona
      target_entity: Person
      requirements:
        - confidence_level: 0.85
        - gps_compliance: true
        - state: CANDIDATE
```

## Infrastructure Capabilities

### What the Data Model Enables

1. **Configurable Standards Support**
   - Load any genealogy standard from configuration
   - Track compliance without hard-coding rules
   - Update standards without changing code
   - Support multiple standards simultaneously

2. **Research Process Tracking**
   - Capture research sessions and activities
   - Generate work products from templates
   - Validate against configured standards
   - Maintain complete audit trails

3. **Flexible Tool Development**
   - Tools can query active configurations
   - Generate appropriate work products
   - Validate compliance automatically
   - Adapt to user's methodology preferences

4. **Workspace Modularity**
   - Enable/disable tool modules
   - Configure methodology preferences
   - Set default templates and formats
   - Maintain multiple workspace configurations

### Key Design Principles

1. **Separation of Data and Tools**
   - Data model provides infrastructure
   - Tools are built on top of model
   - Configuration drives behavior
   - No tool logic in core entities

2. **Metadata-Driven Everything**
   - Standards as YAML/JSON files
   - Templates as configurations
   - Workflows as data
   - Validation rules as specifications

3. **State-Based Progression**
   - Every entity has defined states
   - Transitions are configurable
   - Side effects are declarative
   - History is maintained

4. **Research-First Design**
   - Process entities are first-class
   - Conclusions linked to evidence
   - Journey documented automatically
   - Compliance tracked continuously

### Next Refinements Needed

1. **Entity Relationship Refinement**
   - Clarify Theory ↔ WorkProduct relationships
   - Define Evidence ↔ IdentityPersona linkage
   - Specify Citation state transitions
   - Detail Confidence container nesting

2. **Metadata Schema Definition**
   - YAML schema for standards
   - Template specification format
   - Validation rule language
   - Workflow definition structure

3. **State Machine Formalization**
   - Complete state diagrams for each entity
   - Transition requirement specifications
   - Side effect definitions
   - Rollback/undo capabilities

4. **Infrastructure Boundaries**
   - Clear separation from tool logic
   - API surface for tool developers
   - Extension points for custom entities
   - Plugin architecture for modules

This unified model provides the **infrastructure** for research-driven genealogy, where methodology configurations drive system behavior, not hard-coded rules.