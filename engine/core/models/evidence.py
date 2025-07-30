"""
Evidence and Fact models for ResearchProcess-GPS.

Evidence is a first-class entity that can float between theories.
Facts are extracted from evidence and can be interpreted differently.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum

from .base import NestableBaseEntity
from .confidence import (
    ConfidenceContainer, EvidenceQuality, InformationType, EvidenceType
)


class SourceType(Enum):
    """GPS classification of source types"""
    ORIGINAL = "original"
    DERIVATIVE = "derivative"
    AUTHORED = "authored"
    UNKNOWN = "unknown"


class RecordType(Enum):
    """Common genealogical record types"""
    BIRTH_CERTIFICATE = "birth_certificate"
    DEATH_CERTIFICATE = "death_certificate"
    MARRIAGE_LICENSE = "marriage_license"
    CENSUS = "census"
    CHURCH_RECORD = "church_record"
    MILITARY_RECORD = "military_record"
    LAND_RECORD = "land_record"
    PROBATE = "probate"
    NEWSPAPER = "newspaper"
    CORRESPONDENCE = "correspondence"
    PHOTOGRAPH = "photograph"
    DNA_RESULTS = "dna_results"
    ORAL_HISTORY = "oral_history"
    OTHER = "other"


class FactType(Enum):
    """Types of facts that can be extracted from evidence"""
    NAME = "name"
    DATE = "date"
    PLACE = "place"
    AGE = "age"
    OCCUPATION = "occupation"
    RELATIONSHIP = "relationship"
    EVENT = "event"
    PHYSICAL_DESCRIPTION = "physical_description"
    PROPERTY = "property"
    MILITARY_SERVICE = "military_service"
    EDUCATION = "education"
    RELIGION = "religion"
    NATIONALITY = "nationality"
    RESIDENCE = "residence"
    BURIAL = "burial"
    DNA_MATCH = "dna_match"
    OTHER = "other"


class AnalysisType(Enum):
    """Types of analysis that can be performed on evidence"""
    TRANSCRIPTION = "transcription"
    TRANSLATION = "translation"
    CORRELATION = "correlation"
    CONFLICT_RESOLUTION = "conflict_resolution"
    DATE_ESTIMATION = "date_estimation"
    IDENTITY_CORRELATION = "identity_correlation"
    HANDWRITING_ANALYSIS = "handwriting_analysis"
    PHOTO_ANALYSIS = "photo_analysis"
    DNA_ANALYSIS = "dna_analysis"


@dataclass
class Repository(NestableBaseEntity['Repository']):
    """
    Where evidence is held.
    
    Now supports nesting for:
    - Repository hierarchies (National Archives → Branch → Collection)
    - Virtual repositories containing sub-repositories
    - Collection organization within repositories
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Repository"
        from ..abstractions.nesting import NestingType
        self.nesting_type = NestingType.HIERARCHICAL
    
    name: str = ""
    repository_type: str = ""  # "archive", "library", "online", "private"
    location: Optional[UUID] = None  # Reference to Location
    contact_info: Dict[str, str] = field(default_factory=dict)
    access_info: Dict[str, str] = field(default_factory=dict)
    visit_log: List[Dict[str, Any]] = field(default_factory=list)


@dataclass
class Citation:
    """Full citation information for evidence"""
    citation_id: UUID = field(default_factory=uuid4)
    full_citation: str = ""
    short_citation: str = ""
    
    # Structured elements
    author: Optional[str] = None
    title: Optional[str] = None
    publication_info: Optional[str] = None
    date_accessed: Optional[datetime] = None
    url: Optional[str] = None
    page_numbers: Optional[str] = None
    
    # Where to find it
    repository: Optional[Repository] = None
    call_number: Optional[str] = None
    microfilm_number: Optional[str] = None
    digital_ark: Optional[str] = None
    
    # Quality indicators
    is_complete: bool = False
    citation_standard: str = "Chicago"  # Chicago, MLA, etc.


@dataclass
class ExtractedFact:
    """A fact extracted from evidence"""
    fact_id: UUID = field(default_factory=uuid4)
    fact_type: FactType = FactType.OTHER
    
    # What was found
    extracted_text: str = ""  # Exact text from source
    normalized_value: str = ""  # Standardized version
    interpretation: str = ""  # What we think it means
    
    # Context
    location_in_source: str = ""  # "Page 3, line 15"
    surrounding_context: str = ""  # Text around the fact
    
    # What it applies to
    applies_to_entities: List[UUID] = field(default_factory=list)
    applies_to_theories: Set[UUID] = field(default_factory=set)
    
    # Confidence in extraction
    extraction_confidence: float = 0.0  # 0-1
    extraction_method: str = ""  # "manual", "OCR", "AI"
    extracted_by: str = ""
    extracted_date: datetime = field(default_factory=datetime.utcnow)
    
    # Analysis
    supports_theories: Dict[UUID, float] = field(default_factory=dict)  # theory_id -> support_level
    contradicts_theories: Dict[UUID, str] = field(default_factory=dict)  # theory_id -> reason
    
    def interpret_for_theory(self, theory_id: UUID) -> Optional[str]:
        """Get interpretation specific to a theory"""
        # Different theories might interpret the same fact differently
        # TODO: Implement theory-specific interpretations
        return self.interpretation


@dataclass
class EvidenceAnalysis:
    """Analysis performed on evidence"""
    analysis_id: UUID = field(default_factory=uuid4)
    analysis_type: AnalysisType = AnalysisType.TRANSCRIPTION
    
    # Analysis details
    performed_by: str = ""
    performed_date: datetime = field(default_factory=datetime.utcnow)
    methodology: str = ""
    
    # Results
    results: Dict[str, Any] = field(default_factory=dict)
    conclusions: List[str] = field(default_factory=list)
    
    # For correlation analysis
    correlated_evidence: List[UUID] = field(default_factory=list)
    correlation_strength: Optional[float] = None
    
    # For conflict analysis
    conflicts_identified: List[Dict[str, Any]] = field(default_factory=list)
    resolution_proposed: Optional[str] = None
    
    # Quality
    confidence: Optional[ConfidenceContainer] = None
    peer_reviewed: bool = False
    review_notes: List[str] = field(default_factory=list)


@dataclass
class EvidenceClassification:
    """GPS classification of evidence"""
    source_type: SourceType = SourceType.UNKNOWN
    information_type: InformationType = InformationType.INDETERMINATE
    evidence_type: EvidenceType = EvidenceType.INDIRECT
    
    # Additional classification
    record_type: RecordType = RecordType.OTHER
    original_purpose: str = ""  # Why the record was created
    known_errors: List[str] = field(default_factory=list)
    reliability_issues: List[str] = field(default_factory=list)


@dataclass
class Evidence(NestableBaseEntity['Evidence']):
    """
    Evidence is a first-class entity in ResearchProcess-GPS.
    It can float between theories and be interpreted differently in each.
    
    Now supports nesting for:
    - Document bundles (multiple pages/items from same source)
    - Evidence collections (related evidence grouped for analysis)
    - Derivative evidence (evidence derived from other evidence)
    - Evidence hierarchies (original → transcription → translation)
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Evidence"
        from ..abstractions.nesting import NestingType
        self.nesting_type = NestingType.COMPOSITIONAL  # Evidence bundles
    
    # Classification
    classification: EvidenceClassification = field(default_factory=EvidenceClassification)
    
    # Citation
    citation: Optional[Citation] = None
    
    # Content
    title: str = ""
    description: str = ""
    transcription: Optional[str] = None
    abstract: Optional[str] = None
    
    # Language and translation
    original_language: str = "English"
    translations: Dict[str, str] = field(default_factory=dict)  # language -> translation
    
    # Digital representation
    files: List[Dict[str, Any]] = field(default_factory=list)  # file references
    ocr_text: Optional[str] = None
    
    # Provenance
    creator: str = ""  # Who created the original record
    creation_date: Optional[datetime] = None
    creation_purpose: str = ""
    chain_of_custody: List[Dict[str, Any]] = field(default_factory=list)
    
    # Repository information
    repository: Optional[Repository] = None
    acquisition_info: Dict[str, str] = field(default_factory=dict)
    
    # Extracted facts
    extracted_facts: List[ExtractedFact] = field(default_factory=list)
    
    # Analysis performed
    analyses: List[EvidenceAnalysis] = field(default_factory=list)
    
    # Research process
    found_date: datetime = field(default_factory=datetime.utcnow)
    found_by: str = ""
    search_context: str = ""  # What they were looking for
    negative_evidence: bool = False  # True if this is absence of expected evidence
    
    # Quality assessment
    quality: EvidenceQuality = EvidenceQuality.DERIVATIVE
    legibility: float = 1.0  # 0-1
    completeness: float = 1.0  # 0-1
    
    # Theory relationships
    applicable_theories: Set[UUID] = field(default_factory=set)
    theory_interpretations: Dict[UUID, str] = field(default_factory=dict)
    
    def extract_fact(self, fact_type: FactType, text: str, 
                     normalized: str = None, interpretation: str = None) -> ExtractedFact:
        """Extract a fact from this evidence"""
        fact = ExtractedFact(
            fact_type=fact_type,
            extracted_text=text,
            normalized_value=normalized or text,
            interpretation=interpretation or text,
            extracted_by="system",
            extraction_confidence=0.8
        )
        self.extracted_facts.append(fact)
        return fact
    
    def add_analysis(self, analysis_type: AnalysisType, 
                    results: Dict[str, Any], conclusions: List[str]) -> EvidenceAnalysis:
        """Add an analysis of this evidence"""
        analysis = EvidenceAnalysis(
            analysis_type=analysis_type,
            performed_by="system",
            results=results,
            conclusions=conclusions
        )
        self.analyses.append(analysis)
        return analysis
    
    def correlate_with(self, other_evidence: 'Evidence') -> EvidenceAnalysis:
        """Correlate this evidence with another piece of evidence"""
        analysis = EvidenceAnalysis(
            analysis_type=AnalysisType.CORRELATION,
            correlated_evidence=[other_evidence.id],
            performed_by="system"
        )
        
        # Find common facts
        common_entities = set()
        for fact in self.extracted_facts:
            for other_fact in other_evidence.extracted_facts:
                common = set(fact.applies_to_entities) & set(other_fact.applies_to_entities)
                common_entities.update(common)
        
        analysis.results = {
            'common_entities': list(common_entities),
            'correlation_points': len(common_entities)
        }
        
        if common_entities:
            analysis.conclusions.append(
                f"Evidence correlates on {len(common_entities)} entities"
            )
            analysis.correlation_strength = min(len(common_entities) / 10.0, 1.0)
        
        self.analyses.append(analysis)
        return analysis
    
    def evaluate_for_theory(self, theory_id: UUID) -> Tuple[float, str]:
        """
        Evaluate how this evidence supports or contradicts a theory.
        Returns (support_level, explanation)
        """
        support_score = 0.0
        explanations = []
        
        # Check if any facts support this theory
        supporting_facts = 0
        contradicting_facts = 0
        
        for fact in self.extracted_facts:
            if theory_id in fact.supports_theories:
                supporting_facts += 1
                support_score += fact.supports_theories[theory_id]
            elif theory_id in fact.contradicts_theories:
                contradicting_facts += 1
                explanations.append(fact.contradicts_theories[theory_id])
        
        if supporting_facts + contradicting_facts > 0:
            support_score = support_score / (supporting_facts + contradicting_facts)
        else:
            support_score = 0.5  # Neutral if no facts apply
        
        if supporting_facts > contradicting_facts:
            explanation = f"Supports with {supporting_facts} facts"
        elif contradicting_facts > supporting_facts:
            explanation = f"Contradicts with {contradicting_facts} facts: " + "; ".join(explanations[:2])
        else:
            explanation = "Neutral or ambiguous"
        
        return support_score, explanation
    
    def get_gps_classification(self) -> Dict[str, str]:
        """Get GPS-standard classification of this evidence"""
        return {
            'source_type': self.classification.source_type.value,
            'information_type': self.classification.information_type.value,
            'evidence_type': self.classification.evidence_type.value
        }


@dataclass
class NegativeEvidence(Evidence):
    """
    Special type of evidence representing the absence of expected records.
    Critical for GPS compliance.
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.negative_evidence = True
    
    # What was searched for
    search_parameters: Dict[str, Any] = field(default_factory=dict)
    search_strategy: str = ""
    
    # What was expected but not found
    expected_record_type: RecordType = RecordType.OTHER
    expected_content: str = ""
    
    # Why it matters
    significance: str = ""
    implications: List[str] = field(default_factory=list)
    
    # Alternative explanations
    possible_reasons: List[str] = field(default_factory=list)
    # ["Record destroyed", "Person not there", "Different name used", etc.]