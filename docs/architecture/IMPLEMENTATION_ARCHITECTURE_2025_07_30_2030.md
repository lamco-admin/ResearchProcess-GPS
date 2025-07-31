# Implementation Architecture - ResearchProcess-GPS
## 2025-07-30 20:30 EEST

## Executive Summary

This document transforms the unified conceptual model into concrete implementation steps. ResearchProcess-GPS treats **methodologies as metadata**, **standards as configurations**, and **work products as templates**. Every entity has states, supports promotion/demotion, and maintains complete history.

## Core Architecture Principles

### 1. Everything is Configurable
```yaml
principle: "No hard-coded genealogy rules"
implementation:
  - Standards loaded from YAML/JSON configs
  - Methodologies as pluggable modules
  - Work products as template definitions
  - Validation rules as data, not code
```

### 2. State Machines Everywhere
```python
class EntityStateMachine:
    """Every entity follows state progression"""
    
    states = {
        'DRAFT': ['WORKING', 'ABANDONED'],
        'WORKING': ['TESTING', 'DRAFT', 'ABANDONED'],
        'TESTING': ['CONCLUDED', 'WORKING', 'QUESTIONED'],
        'CONCLUDED': ['PUBLISHED', 'QUESTIONED'],
        'PUBLISHED': ['CHALLENGED', 'ARCHIVED'],
        'QUESTIONED': ['WORKING', 'ABANDONED'],
        'CHALLENGED': ['WORKING', 'DEFENDED'],
    }
    
    def can_transition(self, from_state: str, to_state: str) -> bool:
        return to_state in self.states.get(from_state, [])
```

## Enhanced Entity Implementations

### 1. Revolutionary Confidence Container

Building on existing `confidence.py`, expand to full container:

```python
@dataclass
class ConfidenceAssessment(NestableBaseEntity):
    """Individual assessment within confidence container"""
    
    # Methodology used
    methodology: str  # "GPS-2025", "BCG-3.4", "Custom"
    methodology_config: Dict[str, Any]  # Loaded from YAML
    
    # Multi-dimensional analysis
    dimensions: Dict[str, float] = field(default_factory=dict)
    # {
    #   "source_reliability": 0.85,
    #   "information_credibility": 0.90,
    #   "evidence_directness": 0.75,
    #   "research_exhaustiveness": 0.80,
    #   "analysis_thoroughness": 0.95
    # }
    
    # Narrative explanation
    narrative: str  # Full text explanation
    
    # Evidence for this assessment
    supporting_evidence: List[UUID]
    contradicting_evidence: List[UUID]
    
    # For negative/disproven
    evidence_type: EvidenceType
    absence_explanation: Optional[str]
    disproof_mechanism: Optional[str]
    
    # Audit trail
    assessor: str
    assessment_date: datetime
    peer_reviewed: bool
    peer_reviews: List[PeerReview]

@dataclass
class Confidence(NestableBaseEntity):
    """
    Not a score but a complete research narrative.
    Can nest for complex multi-part assessments.
    """
    
    # Multiple assessments over time
    assessments: List[ConfidenceAssessment]
    
    # Current summary (computed)
    current_confidence: ConfidenceLevel
    
    # GPS compliance tracking
    gps_compliance: GPSCompliance
    
    # Research coverage
    coverage: ResearchCoverage
    
    # Audit checklist
    audit_items: List[AuditCheckItem]
    
    # State management
    state: ConfidenceState  # BUILDING → REVIEWING → ACCEPTED → CHALLENGED
    
    def add_assessment(self, assessment: ConfidenceAssessment) -> None:
        """Add new assessment and recompute confidence"""
        self.assessments.append(assessment)
        self._recompute_confidence()
        self._check_state_transition()
```

### 2. Theory (Research Question) Implementation

```python
@dataclass  
class Theory(NestableBaseEntity):
    """
    Central organizing principle - what we're investigating.
    User-facing term: "Research Question"
    """
    
    # The question/hypothesis
    question: str  # "Who were the parents of John Smith?"
    hypothesis: str  # "John Smith was son of William & Mary"
    
    # State progression
    state: TheoryState
    
    # Standards compliance tracking
    compliance_configs: Dict[str, ComplianceConfig] = field(default_factory=dict)
    compliance_status: Dict[str, ComplianceStatus] = field(default_factory=dict)
    
    # Work products generated
    work_products: Dict[str, WorkProduct] = field(default_factory=dict)
    # {
    #   "research_log": ResearchLog instance,
    #   "proof_statement": ProofStatement instance,
    #   "analysis": EvidenceAnalysis instance
    # }
    
    # Git-like branching
    parent_theory: Optional[UUID] = None
    child_theories: List[UUID] = field(default_factory=list)
    merge_history: List[TheoryMerge] = field(default_factory=list)
    
    def branch(self, new_hypothesis: str) -> 'Theory':
        """Create alternative theory branch"""
        child = Theory(
            question=self.question,
            hypothesis=new_hypothesis,
            parent_theory=self.id,
            state=TheoryState.EXPLORING
        )
        self.child_theories.append(child.id)
        return child
    
    def check_gps_compliance(self) -> GPSCompliance:
        """Check theory against GPS standards"""
        config = self.compliance_configs.get('GPS-2025')
        return config.check_compliance(self) if config else None
```

### 3. IdentityPersona (Dual-Named Entity)

```python
@dataclass
class IdentityPersona(NestableBaseEntity):
    """
    Non-conclusive identity/persona representation.
    Created from evidence references.
    Can be promoted to Person when concluded.
    """
    
    # Display name (user preference)
    display_as: str = "Identity"  # or "Persona"
    
    # State tracking
    state: IdentityState
    # REFERENCE → WORKING → HYPOTHESIS → CANDIDATE → PROMOTED
    
    # Evidence that created this identity
    evidence_references: List[EvidenceReference] = field(default_factory=list)
    
    # Extracted attributes (with confidence)
    attributes: Dict[str, AttributeValue] = field(default_factory=dict)
    # {
    #   "name": AttributeValue(value="John Smith", confidence=0.85),
    #   "birth_date": AttributeValue(value="1825", confidence=0.75)
    # }
    
    # Can nest for variants
    variants: List['IdentityPersona'] = field(default_factory=list)
    
    # Promotion tracking
    promotion_eligible: bool = False
    promotion_criteria: PromotionConfig = field(default_factory=PromotionConfig)
    promotion_blockers: List[str] = field(default_factory=list)
    
    # If promoted
    person_id: Optional[UUID] = None
    promotion_date: Optional[datetime] = None
    
    def check_promotion_eligibility(self) -> bool:
        """Check if ready for Person promotion"""
        return self.promotion_criteria.evaluate(self)
    
    def promote_to_person(self) -> UUID:
        """Promote to concluded Person"""
        if not self.promotion_eligible:
            raise ValueError("Not eligible for promotion")
        
        person = Person.from_identity_persona(self)
        self.person_id = person.id
        self.promotion_date = datetime.utcnow()
        self.state = IdentityState.PROMOTED
        return person.id
```

### 4. ResearchLog (Process Documentation)

```python
@dataclass
class ResearchLog(NestableBaseEntity):
    """
    Documents the research journey.
    Auto-captured + manual entries.
    """
    
    # Log configuration
    log_type: LogType  # SESSION, PROJECT, REPOSITORY_VISIT
    schema_config: ResearchLogConfig  # NGS, BCG, custom
    
    # Parent theory/question
    theory_id: UUID
    
    # Entries
    entries: List[LogEntry] = field(default_factory=list)
    
    # Auto-capture data
    browser_sessions: List[BrowserSession] = field(default_factory=list)
    api_calls: List[APICall] = field(default_factory=list)
    
    # State tracking
    state: LogState  # ACTIVE → CLOSED → ARCHIVED
    
    def add_search(self, 
                   repository: str,
                   collection: str,
                   search_params: Dict,
                   results: List[UUID],
                   negative: bool = False) -> LogEntry:
        """Add repository search to log"""
        entry = LogEntry(
            entry_type="SEARCH",
            timestamp=datetime.utcnow(),
            repository=repository,
            collection=collection,
            search_params=search_params,
            results=results,
            negative_result=negative,
            researcher=self.current_researcher
        )
        self.entries.append(entry)
        return entry

@dataclass
class LogEntry:
    """Individual research log entry"""
    entry_type: str  # SEARCH, NOTE, ANALYSIS, DISCOVERY
    timestamp: datetime
    
    # Search details
    repository: Optional[str] = None
    collection: Optional[str] = None
    search_params: Dict[str, Any] = field(default_factory=dict)
    
    # Results
    results: List[UUID] = field(default_factory=list)  # Evidence IDs
    negative_result: bool = False
    
    # Notes
    notes: str = ""
    
    # Attribution
    researcher: str = ""
```

## Standards as Configuration

### GPS Configuration Example
```yaml
# config/standards/GPS-2025.yaml
standard:
  name: "Genealogical Proof Standard"
  version: "2025"
  authority: "BCG"
  
elements:
  reasonably_exhaustive_research:
    description: "Thorough research in reliable sources"
    requirements:
      repository_coverage:
        vital_records:
          minimum: 0.90
          recommended: 1.0
        census_records:
          minimum: 0.80
          recommended: 0.95
        probate_records:
          minimum: 0.70
          recommended: 0.90
      
      negative_search_documentation:
        required: true
        format: "research_log_entry"
      
      geographic_coverage:
        primary_location: 0.95
        adjacent_locations: 0.80
        migration_paths: 0.70
  
  complete_citations:
    description: "Complete and accurate source citations"
    template_system: "Evidence_Explained"
    required_elements:
      - author
      - title
      - publication_info
      - where_found
      - specific_content
    validation_rules:
      - "all_sources_cited"
      - "citations_testable"
      - "format_consistent"
  
  thorough_analysis:
    description: "Analysis and correlation of evidence"
    requirements:
      evidence_matrix: 
        required: true
        format: "evidence_analysis_worksheet"
      correlation_tracking:
        required: true
      quality_assessment:
        source: required
        information: required
        evidence: required
  
  conflict_resolution:
    description: "Resolution of conflicting evidence"
    documentation:
      required: true
      format: "proof_statement"
    resolution_types:
      - explained
      - reconciled
      - prioritized
    unresolved_conflicts:
      allowed: false
  
  written_conclusion:
    description: "Sound written conclusion"
    formats:
      - proof_argument
      - proof_summary
      - research_report
    peer_review:
      recommended: true
      required_for_publication: true

compliance_checking:
  automated_checks:
    - citation_completeness
    - repository_coverage
    - conflict_identification
  manual_verification:
    - analysis_quality
    - reasoning_soundness
    - conclusion_support
```

### Work Product Templates
```yaml
# config/work_products/research_log.yaml
work_product:
  name: "Research Log"
  standard: "BCG"
  
schema:
  required_fields:
    - date
    - researcher
    - objective
    - repository
    - sources_searched
    - results
    
  optional_fields:
    - call_numbers
    - search_strategy
    - time_spent
    - costs
    - next_steps
    
formatting:
  date_format: "YYYY-MM-DD"
  structure: "chronological"
  
auto_capture:
  enabled: true
  sources:
    - browser_extension
    - api_integration
    - manual_entry
    
export_formats:
  - markdown
  - pdf
  - excel
  - gedcom_extension
```

## Toolbox Architecture

### 1. Planning Tools
```python
class ResearchPlanTool:
    """Generate research plans from theories"""
    
    def create_plan(self, theory: Theory) -> ResearchPlan:
        # Load template based on configuration
        template = self.load_template(theory.compliance_configs)
        
        # Generate plan sections
        plan = ResearchPlan(
            theory_id=theory.id,
            objectives=self.extract_objectives(theory),
            repositories=self.suggest_repositories(theory),
            record_types=self.suggest_records(theory),
            priority_order=self.calculate_priorities(theory)
        )
        
        return plan
```

### 2. Collection Tools
```python
class AutoCaptureExtension:
    """Browser extension for automatic research capture"""
    
    def capture_search(self, url: str, params: Dict) -> LogEntry:
        # Identify repository
        repository = self.identify_repository(url)
        
        # Extract search parameters
        search_params = self.extract_params(url, params)
        
        # Create log entry
        return LogEntry(
            entry_type="SEARCH",
            repository=repository,
            search_params=search_params,
            timestamp=datetime.utcnow()
        )
```

### 3. Analysis Tools
```python
class EvidenceAnalyzer:
    """Analyze evidence according to configured standards"""
    
    def analyze(self, evidence: List[Evidence], 
                standard: str = "GPS-2025") -> EvidenceAnalysis:
        config = self.load_standard(standard)
        
        analysis = EvidenceAnalysis()
        
        # Quality assessment
        for e in evidence:
            analysis.add_quality_assessment(
                e.id,
                self.assess_source_quality(e, config),
                self.assess_information_quality(e, config),
                self.assess_evidence_quality(e, config)
            )
        
        # Correlation matrix
        analysis.correlation_matrix = self.build_correlations(evidence)
        
        # Conflict detection
        analysis.conflicts = self.detect_conflicts(evidence)
        
        return analysis
```

### 4. Documentation Tools
```python
class ProofStatementGenerator:
    """Generate proof statements from theories and evidence"""
    
    def generate(self, theory: Theory, 
                 template: str = "BCG_Standard") -> ProofStatement:
        # Load template
        template_config = self.load_template(template)
        
        # Build sections
        statement = ProofStatement(
            theory_id=theory.id,
            research_question=theory.question,
            evidence_presentation=self.format_evidence(theory),
            analysis=self.format_analysis(theory),
            conflicts=self.format_conflicts(theory),
            conclusion=self.format_conclusion(theory)
        )
        
        # Check GPS compliance
        statement.gps_compliance = theory.check_gps_compliance()
        
        return statement
```

## Revolutionary Features Implementation

### 1. Everything Has States
```python
class StateManager:
    """Universal state management for all entities"""
    
    def transition(self, entity: BaseEntity, 
                   new_state: str,
                   reason: str) -> bool:
        # Check if transition allowed
        if not entity.can_transition_to(new_state):
            return False
        
        # Record transition
        transition = StateTransition(
            from_state=entity.state,
            to_state=new_state,
            timestamp=datetime.utcnow(),
            reason=reason,
            actor=self.current_user
        )
        
        entity.state_history.append(transition)
        entity.state = new_state
        
        # Trigger workflows
        self.trigger_workflows(entity, transition)
        
        return True
```

### 2. Standards as Data
```python
class StandardsLoader:
    """Load and manage genealogy standards"""
    
    def load_standard(self, standard_name: str) -> ComplianceConfig:
        # Load from YAML
        config_path = f"config/standards/{standard_name}.yaml"
        with open(config_path) as f:
            config_data = yaml.safe_load(f)
        
        # Create compliance checker
        return ComplianceConfig(
            name=config_data['standard']['name'],
            version=config_data['standard']['version'],
            elements=config_data['elements'],
            compliance_rules=config_data['compliance_checking']
        )
    
    def check_compliance(self, entity: BaseEntity, 
                        standard: str) -> ComplianceReport:
        config = self.load_standard(standard)
        return config.check_entity(entity)
```

### 3. Process-First Design
```python
class ResearchProcess:
    """Central process orchestrator"""
    
    def start_research(self, question: str) -> Theory:
        # Create theory
        theory = Theory(
            question=question,
            state=TheoryState.EXPLORING
        )
        
        # Create research log
        log = ResearchLog(
            theory_id=theory.id,
            log_type=LogType.PROJECT
        )
        
        # Link work products
        theory.work_products['research_log'] = log
        
        # Load applicable standards
        for standard in self.user_preferences.standards:
            theory.compliance_configs[standard] = self.load_standard(standard)
        
        return theory
```

## Migration Path

### Phase 1: Core Infrastructure (Weeks 1-4)
1. Implement enhanced NestableBaseEntity with states
2. Create standards loading system
3. Build Confidence container entity
4. Implement Theory (Research Question) entity

### Phase 2: Identity System (Weeks 5-8)
1. Build IdentityPersona with promotion/demotion
2. Enhance Person for concluded identities
3. Create identity merging/splitting tools
4. Implement evidence reference system

### Phase 3: Research Process (Weeks 9-12)
1. Implement ResearchLog entity
2. Build auto-capture integrations
3. Create work product generators
4. Implement GPS compliance checking

### Phase 4: Analysis Tools (Weeks 13-16)
1. Build evidence analyzer
2. Create correlation matrices
3. Implement conflict detection
4. Build proof statement generator

### Phase 5: Collaboration (Weeks 17-20)
1. Add multi-user support
2. Implement theory branching/merging
3. Build peer review workflows
4. Create attribution tracking

## Storage Architecture

### Git-Based Storage
```python
class GitStorageAdapter:
    """Store entities in Git with full history"""
    
    def save_entity(self, entity: BaseEntity) -> str:
        # Serialize to YAML
        yaml_content = self.serialize_to_yaml(entity)
        
        # Create file path
        file_path = f"{entity.__class__.__name__.lower()}/{entity.id}.yaml"
        
        # Write and commit
        self.repo.index.add([file_path])
        commit = self.repo.index.commit(
            f"Update {entity.__class__.__name__} {entity.id}",
            author=self.get_author()
        )
        
        return commit.hexsha
    
    def load_entity_history(self, entity_id: UUID) -> List[EntityVersion]:
        # Get all commits for this entity
        file_path = self.get_entity_path(entity_id)
        commits = list(self.repo.iter_commits(paths=file_path))
        
        # Load each version
        versions = []
        for commit in commits:
            content = self.repo.oid_to_object(commit.tree[file_path].binsha).data
            entity = self.deserialize_from_yaml(content)
            versions.append(EntityVersion(entity, commit))
        
        return versions
```

## The Complete Revolution

This architecture enables:

1. **True Research Process Modeling** - Not just conclusions but the entire journey
2. **Standards Without Hard-Coding** - GPS, BCG, custom standards all configurable
3. **Professional-Grade Confidence** - Complete narratives, not simple scores
4. **Unlimited Flexibility** - Nest anything, relate anything, configure anything
5. **Full Attribution** - Every change tracked, every decision documented
6. **Time Travel** - See any entity at any point in its history
7. **Collaborative Research** - Branch, merge, review like software development

This is genealogy software designed for **how genealogists actually work**, not how programmers think they should work.