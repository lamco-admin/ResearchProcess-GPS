# Research Process Entities - ResearchProcess-GPS
## 2025-07-30 21:05 EEST

## Overview

These entities form the research process infrastructure, separate from but linked to the genealogical data model. They enable configurable methodologies, standards compliance tracking, and work product generation.

## Core Research Process Entities

### 1. WorkProduct

```python
@dataclass
class WorkProduct(NestableBaseEntity['WorkProduct']):
    """
    Base class for all research work products.
    Enables tools to generate standard outputs.
    """
    # Type and template
    product_type: WorkProductType
    # RESEARCH_LOG, PROOF_STATEMENT, PROOF_ARGUMENT, 
    # EVIDENCE_ANALYSIS, RESEARCH_REPORT, TIMELINE,
    # CORRELATION_MATRIX, SOURCE_ANALYSIS
    
    template_id: str  # References template in registry
    template_version: str
    schema_version: str
    
    # Links to genealogical data
    theory_id: UUID  # Parent research question
    evidence_ids: List[UUID] = field(default_factory=list)
    identity_ids: List[UUID] = field(default_factory=list)
    
    # Generation metadata
    generation_metadata: Dict[str, Any] = field(default_factory=dict)
    # {
    #   "methodology": "GPS-2025",
    #   "generator": "ProofStatementBuilder v1.0",
    #   "generation_date": "2025-07-30T21:00:00Z",
    #   "parameters": {...}
    # }
    
    # Content (structured based on template)
    content: Dict[str, Any] = field(default_factory=dict)
    
    # Validation and compliance
    validation_rules: List[ValidationRule] = field(default_factory=list)
    validation_status: ValidationStatus = ValidationStatus.DRAFT
    compliance_checks: Dict[str, ComplianceResult] = field(default_factory=dict)
    
    # Export capabilities
    export_formats: List[str] = field(default_factory=list)
    # ["markdown", "pdf", "docx", "gedcom-extension"]
    
    # State management
    state: WorkProductState = WorkProductState.DRAFT
    # DRAFT → REVIEW → FINAL → PUBLISHED → ARCHIVED
    
    # Peer review
    reviews: List[WorkProductReview] = field(default_factory=list)
    
    # Attribution
    created_by: UUID  # Researcher ID
    created_date: datetime
    last_modified_by: UUID  # Researcher ID
    last_modified_date: datetime
    contributors: List[UUID] = field(default_factory=list)  # All Researcher IDs
    
    def validate(self) -> ValidationResult:
        """Validate against template and methodology rules"""
        pass
    
    def export(self, format: str) -> bytes:
        """Export in specified format"""
        pass
```

### 2. ResearchLog

```python
@dataclass
class ResearchLog(WorkProduct):
    """
    Specialized work product for research documentation.
    Supports auto-capture and manual entries.
    """
    # Log configuration
    log_type: LogType = LogType.PROJECT
    # SESSION, PROJECT, REPOSITORY_VISIT, ONLINE_SEARCH
    
    schema_config: str = "BCG_STANDARD"
    # References methodology-specific schema
    
    # Entries
    entries: List[ResearchLogEntry] = field(default_factory=list)
    
    # Sessions that contributed
    session_ids: List[UUID] = field(default_factory=list)
    
    # Coverage tracking
    coverage_summary: ResearchCoverage = field(default_factory=ResearchCoverage)
    
    # Auto-capture settings
    auto_capture_enabled: bool = True
    capture_sources: List[CaptureSource] = field(default_factory=list)
    
    # Attribution tracking
    primary_researcher: UUID  # Main researcher maintaining log
    contributing_researchers: List[UUID] = field(default_factory=list)
    
    def add_entry(self, entry: ResearchLogEntry) -> None:
        """Add manual or auto-captured entry"""
        self.entries.append(entry)
        self._update_coverage(entry)
        # Track contributor
        if entry.researcher_id not in self.contributing_researchers:
            self.contributing_researchers.append(entry.researcher_id)
    
    def add_session(self, session: ResearchSession) -> None:
        """Import entries from research session"""
        for activity in session.activities:
            entry = self._activity_to_entry(activity)
            self.add_entry(entry)
        self.session_ids.append(session.id)
```

### 3. ResearchLogEntry

```python
@dataclass
class ResearchLogEntry(BaseEntity):
    """
    Individual entry in a research log.
    """
    # Entry metadata
    entry_type: LogEntryType
    # SEARCH, ANALYSIS, DISCOVERY, NOTE, NEGATIVE_RESULT
    
    timestamp: datetime
    duration: Optional[timedelta] = None
    
    # What was searched/analyzed
    target_description: str  # "1850 Census, Boston Ward 3"
    repository: Optional[str] = None
    collection: Optional[str] = None
    
    # Search parameters
    search_params: Dict[str, Any] = field(default_factory=dict)
    # {"surname": "Smith", "given": "John", "year_range": "1845-1855"}
    
    # Results
    result_type: ResultType = ResultType.NO_RESULT
    # FOUND, NO_RESULT, PARTIAL, ACCESS_DENIED
    
    results: List[UUID] = field(default_factory=list)  # Evidence IDs
    result_count: int = 0
    
    # For negative results
    negative_result: bool = False
    absence_reason: Optional[str] = None
    # "Records destroyed in 1871 fire"
    
    # Notes and analysis
    notes: str = ""
    follow_up_needed: List[str] = field(default_factory=list)
    
    # Attribution
    researcher_id: UUID  # Researcher ID
    researcher_name: str = ""  # For display
    capture_method: CaptureMethod = CaptureMethod.MANUAL
    # MANUAL, BROWSER_EXTENSION, API_INTEGRATION
```

### 4. ResearchSession

```python
@dataclass
class ResearchSession(NestableBaseEntity['ResearchSession']):
    """
    Captures a research session for auto-logging.
    Can be nested for sub-sessions.
    """
    # Session metadata
    session_type: SessionType
    # REPOSITORY_VISIT, ONLINE_SEARCH, ANALYSIS_SESSION,
    # DOCUMENT_REVIEW, CORRELATION_WORK
    
    start_time: datetime
    end_time: Optional[datetime] = None
    
    # Location/context
    location: Optional[str] = None  # Physical or virtual
    repository: Optional[str] = None
    purpose: str = ""  # "Find Smith family in 1850 census"
    
    # Capture metadata
    capture_sources: List[CaptureSource] = field(default_factory=list)
    # [
    #   {"type": "BROWSER_EXTENSION", "version": "1.0"},
    #   {"type": "API_INTEGRATION", "api": "FamilySearch"}
    # ]
    
    # Activities performed
    activities: List[ResearchActivity] = field(default_factory=list)
    
    # Outputs generated
    evidence_created: List[UUID] = field(default_factory=list)
    identities_created: List[UUID] = field(default_factory=list)
    work_products_created: List[UUID] = field(default_factory=list)
    
    # Session state
    state: SessionState = SessionState.ACTIVE
    # ACTIVE, PAUSED, COMPLETED, ABANDONED
    
    # Parent theory context
    theory_id: Optional[UUID] = None
    
    # Attribution
    researcher_id: UUID  # Primary researcher for session
    additional_researchers: List[UUID] = field(default_factory=list)
    
    def add_activity(self, activity: ResearchActivity) -> None:
        """Add activity to session"""
        self.activities.append(activity)
        
    def pause(self) -> None:
        """Pause session for later resumption"""
        self.state = SessionState.PAUSED
        
    def complete(self) -> None:
        """Mark session as complete"""
        self.end_time = datetime.utcnow()
        self.state = SessionState.COMPLETED
```

### 5. ResearchActivity

```python
@dataclass
class ResearchActivity(BaseEntity):
    """
    Atomic research activity within a session.
    """
    # Activity classification
    activity_type: ActivityType
    # SEARCH, EXTRACT, ANALYZE, CORRELATE, DOCUMENT,
    # REVIEW, TRANSCRIBE, TRANSLATE
    
    timestamp: datetime
    duration: Optional[timedelta] = None
    
    # Activity details
    description: str  # "Search for John Smith birth record"
    
    # Target of activity
    target_entity: Optional[UUID] = None
    target_type: Optional[str] = None  # "Source", "Evidence", etc.
    
    # Parameters and context
    parameters: Dict[str, Any] = field(default_factory=dict)
    tools_used: List[str] = field(default_factory=list)
    
    # Results
    success: bool = True
    results: Dict[str, Any] = field(default_factory=dict)
    # {
    #   "found_count": 3,
    #   "quality": "high",
    #   "confidence": 0.85
    # }
    
    # Links to created entities
    created_entities: List[UUID] = field(default_factory=list)
    modified_entities: List[UUID] = field(default_factory=list)
    
    # Errors or issues
    errors: List[str] = field(default_factory=list)
    warnings: List[str] = field(default_factory=list)
    
    # Attribution
    performed_by: UUID  # Researcher ID
```

### 6. ProofStatement

```python
@dataclass
class ProofStatement(WorkProduct):
    """
    Formal proof statement or argument work product.
    """
    # Proof type
    proof_type: ProofType = ProofType.STATEMENT
    # STATEMENT, ARGUMENT, SUMMARY
    
    # Structure (following template)
    sections: Dict[str, ProofSection] = field(default_factory=dict)
    # {
    #   "question": ProofSection(...),
    #   "evidence": ProofSection(...),
    #   "analysis": ProofSection(...),
    #   "conclusion": ProofSection(...)
    # }
    
    # GPS compliance
    gps_elements: Dict[str, bool] = field(default_factory=dict)
    gps_compliant: bool = False
    
    # Conflicts addressed
    conflicts_identified: List[ConflictDescription] = field(default_factory=list)
    resolutions: Dict[str, str] = field(default_factory=dict)
    
    # Supporting materials
    evidence_matrix_id: Optional[UUID] = None
    timeline_id: Optional[UUID] = None
    
    # Professional metadata
    intended_audience: str = ""  # "peer_review", "client", "publication"
    publication_status: Optional[str] = None
```

### 7. EvidenceAnalysis

```python
@dataclass
class EvidenceAnalysis(WorkProduct):
    """
    Evidence analysis matrix or worksheet.
    """
    # Analysis methodology
    analysis_method: str = "EVIDENCE_EXPLAINED"
    
    # Evidence evaluated
    evidence_items: List[EvidenceAnalysisItem] = field(default_factory=list)
    
    # Correlation results
    correlations: Dict[str, List[Correlation]] = field(default_factory=dict)
    # {
    #   "name_variants": [...],
    #   "date_conflicts": [...],
    #   "place_variations": [...]
    # }
    
    # Quality assessments
    source_quality_matrix: Dict[UUID, SourceQualityAssessment] = field(default_factory=dict)
    information_quality_matrix: Dict[UUID, InformationQualityAssessment] = field(default_factory=dict)
    evidence_quality_matrix: Dict[UUID, EvidenceQualityAssessment] = field(default_factory=dict)
    
    # Patterns identified
    patterns: List[Pattern] = field(default_factory=list)
    anomalies: List[Anomaly] = field(default_factory=list)
    
    # Conclusions
    synthesis: str = ""
    confidence_assessment: Optional[UUID] = None  # Links to Confidence entity
```

## Supporting Types

### Enumerations

```python
class WorkProductType(Enum):
    RESEARCH_LOG = "research_log"
    PROOF_STATEMENT = "proof_statement"
    PROOF_ARGUMENT = "proof_argument"
    EVIDENCE_ANALYSIS = "evidence_analysis"
    RESEARCH_REPORT = "research_report"
    TIMELINE = "timeline"
    CORRELATION_MATRIX = "correlation_matrix"
    SOURCE_ANALYSIS = "source_analysis"
    FAMILY_GROUP_SHEET = "family_group_sheet"
    PEDIGREE_CHART = "pedigree_chart"

class WorkProductState(Enum):
    DRAFT = "draft"
    REVIEW = "review"
    FINAL = "final"
    PUBLISHED = "published"
    ARCHIVED = "archived"
    DEPRECATED = "deprecated"

class SessionType(Enum):
    REPOSITORY_VISIT = "repository_visit"
    ONLINE_SEARCH = "online_search"
    ANALYSIS_SESSION = "analysis_session"
    DOCUMENT_REVIEW = "document_review"
    CORRELATION_WORK = "correlation_work"
    CLIENT_MEETING = "client_meeting"

class ActivityType(Enum):
    SEARCH = "search"
    EXTRACT = "extract"
    ANALYZE = "analyze"
    CORRELATE = "correlate"
    DOCUMENT = "document"
    REVIEW = "review"
    TRANSCRIBE = "transcribe"
    TRANSLATE = "translate"
    PHOTOGRAPH = "photograph"
    INDEX = "index"

class LogEntryType(Enum):
    SEARCH = "search"
    ANALYSIS = "analysis"
    DISCOVERY = "discovery"
    NOTE = "note"
    NEGATIVE_RESULT = "negative_result"
    ACCESS_ISSUE = "access_issue"
    FOLLOW_UP = "follow_up"

class ResultType(Enum):
    FOUND = "found"
    NO_RESULT = "no_result"
    PARTIAL = "partial"
    ACCESS_DENIED = "access_denied"
    PENDING = "pending"
```

### Supporting Classes

```python
@dataclass
class ProofSection:
    """Section of a proof statement"""
    title: str
    content: str
    citations: List[UUID] = field(default_factory=list)
    subsections: List['ProofSection'] = field(default_factory=list)

@dataclass
class ConflictDescription:
    """Description of conflicting evidence"""
    conflict_type: str  # "date", "name", "relationship"
    evidence_a: UUID
    evidence_b: UUID
    description: str
    resolution_strategy: str

@dataclass
class EvidenceAnalysisItem:
    """Individual item in evidence analysis"""
    evidence_id: UUID
    source_citation: str
    information_extracted: str
    analysis_notes: str
    quality_assessment: Dict[str, str]

@dataclass
class WorkProductReview:
    """Peer review of work product"""
    reviewer_id: UUID  # Researcher ID
    reviewer_name: str  # For display
    review_date: datetime
    review_type: str  # "technical", "methodology", "completeness"
    comments: List[str]
    required_changes: List[str]
    approval_status: str  # "approved", "conditional", "rejected"
```

## Integration Points

### With Genealogical Model
1. WorkProduct links to Theory via theory_id
2. ResearchLog documents Evidence discovery
3. EvidenceAnalysis evaluates Evidence entities
4. ProofStatement synthesizes IdentityPersona conclusions

### With Methodology Infrastructure
1. WorkProduct validates against MethodologyConfig
2. Templates loaded from TemplateRegistry
3. Compliance checked against StandardsRegistry
4. Export formats determined by workspace preferences

### With Workspace Model
1. Active methodologies determine available templates
2. Module configuration enables/disables features
3. User preferences set defaults
4. Export preferences control output formats

## Next Steps

1. Define WorkProduct template schemas
2. Create validation rule specifications
3. Design auto-capture integration points
4. Specify compliance checking algorithms
5. Build export format transformations