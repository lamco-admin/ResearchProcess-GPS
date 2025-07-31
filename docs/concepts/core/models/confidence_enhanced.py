"""
Enhanced Confidence Container for ResearchProcess-GPS.

This revolutionary implementation treats confidence not as a simple score
but as a complete research narrative with assessments, methodologies,
and full audit trails.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Union
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum

from .base import NestableBaseEntity
from ...protocols.entity_protocol import ConfidenceLevel


class EvidenceType(Enum):
    """Types of evidence classification"""
    POSITIVE = "positive"      # Supports the claim
    NEGATIVE = "negative"      # Absence of expected evidence
    DISPROVEN = "disproven"   # Contradicts the claim
    NEUTRAL = "neutral"       # Neither supports nor contradicts


class AssessmentMethodology(Enum):
    """Standard assessment methodologies"""
    GPS_2025 = "GPS-2025"
    BCG_3_4 = "BCG-3.4"
    NGS_2024 = "NGS-2024"
    CUSTOM = "custom"


class ConfidenceState(Enum):
    """States for confidence assessment lifecycle"""
    BUILDING = "building"          # Still gathering assessments
    REVIEWING = "reviewing"        # Under peer review
    ACCEPTED = "accepted"         # Peer reviewed and accepted
    CHALLENGED = "challenged"     # Being questioned
    SUPERSEDED = "superseded"     # Replaced by newer assessment


@dataclass
class PeerReview:
    """Peer review of a confidence assessment"""
    review_id: UUID = field(default_factory=uuid4)
    reviewer_id: UUID = field(default_factory=uuid4)  # Researcher ID
    reviewer_name: str = ""  # For display
    reviewer_credentials: str = ""  # "CG", "PhD", etc.
    review_date: datetime = field(default_factory=datetime.utcnow)
    
    # Review components
    methodology_review: str = ""
    evidence_review: str = ""
    analysis_review: str = ""
    conclusion_review: str = ""
    
    # Scores
    methodology_score: float = 0.0  # 0-1
    evidence_score: float = 0.0     # 0-1
    analysis_score: float = 0.0     # 0-1
    conclusion_score: float = 0.0   # 0-1
    
    # Overall assessment
    recommendation: str = ""  # "accept", "revise", "reject"
    required_changes: List[str] = field(default_factory=list)
    
    def overall_score(self) -> float:
        """Calculate overall review score"""
        scores = [
            self.methodology_score,
            self.evidence_score,
            self.analysis_score,
            self.conclusion_score
        ]
        return sum(scores) / len(scores) if scores else 0.0


@dataclass
class ConfidenceAssessment(NestableBaseEntity['ConfidenceAssessment']):
    """
    Individual assessment within a confidence container.
    Can be nested for complex multi-part assessments.
    """
    
    # Methodology used
    methodology: AssessmentMethodology = AssessmentMethodology.CUSTOM
    methodology_config: Dict[str, Any] = field(default_factory=dict)
    methodology_version: str = ""
    
    # Link to detailed Analysis entity (NEW)
    detailed_analysis_id: Optional[UUID] = None  # Analysis entity with full reasoning
    
    # Multi-dimensional analysis
    dimensions: Dict[str, float] = field(default_factory=dict)
    # Example dimensions:
    # {
    #   "source_reliability": 0.85,
    #   "information_credibility": 0.90,
    #   "evidence_directness": 0.75,
    #   "research_exhaustiveness": 0.80,
    #   "analysis_thoroughness": 0.95,
    #   "temporal_proximity": 0.70,
    #   "geographic_relevance": 0.85,
    #   "cultural_understanding": 0.60
    # }
    
    # Narrative explanation (the "why")
    narrative: str = ""
    narrative_sections: Dict[str, str] = field(default_factory=dict)
    # {
    #   "source_analysis": "The 1850 census is an original source...",
    #   "information_quality": "Information provided by head of household...",
    #   "evidence_evaluation": "Direct evidence of residence but indirect for birth...",
    #   "research_gaps": "Church records not yet examined due to...",
    #   "conclusions": "High confidence in residence, moderate for relationships..."
    # }
    
    # Evidence analysis
    supporting_evidence: List[UUID] = field(default_factory=list)
    contradicting_evidence: List[UUID] = field(default_factory=list)
    evidence_weight: Dict[UUID, float] = field(default_factory=dict)  # How much each evidence contributes
    
    # For negative/disproven evidence
    evidence_type: EvidenceType = EvidenceType.POSITIVE
    absence_explanation: Optional[str] = None
    disproof_mechanism: Optional[str] = None
    negative_searches: List[Dict[str, Any]] = field(default_factory=list)
    # [
    #   {
    #     "repository": "County Courthouse",
    #     "collection": "Birth Records 1820-1830",
    #     "search_date": "2025-07-30",
    #     "result": "No record found for John Smith"
    #   }
    # ]
    
    # Professional metadata
    assessor_id: UUID = field(default_factory=uuid4)  # Researcher ID
    assessor_name: str = ""  # For display
    assessor_credentials: str = ""  # "CG", "AG", etc.
    assessment_date: datetime = field(default_factory=datetime.utcnow)
    assessment_hours: float = 0.0  # Time invested
    
    # Peer review
    peer_reviewed: bool = False
    peer_reviews: List[PeerReview] = field(default_factory=list)
    peer_reviewer_ids: List[UUID] = field(default_factory=list)  # Researcher IDs
    
    # GPS compliance for this assessment
    gps_elements: Dict[str, bool] = field(default_factory=dict)
    # {
    #   "reasonably_exhaustive": True,
    #   "complete_citations": True,
    #   "thorough_analysis": True,
    #   "conflicts_resolved": False,
    #   "written_conclusion": True
    # }
    
    def calculate_summary_score(self) -> float:
        """Calculate overall assessment score from dimensions"""
        if not self.dimensions:
            return 0.0
        return sum(self.dimensions.values()) / len(self.dimensions)
    
    def add_peer_review(self, review: PeerReview) -> None:
        """Add a peer review to this assessment"""
        self.peer_reviews.append(review)
        self.peer_reviewed = True
        self.updated_at = datetime.utcnow()


@dataclass
class ResearchCoverage:
    """Enhanced research coverage tracking"""
    
    # Geographic coverage with granularity
    geographic_coverage: Dict[str, Dict[str, Any]] = field(default_factory=dict)
    # {
    #   "Boston, MA": {
    #     "coverage": 0.85,
    #     "repositories_checked": ["City Hall", "Public Library"],
    #     "repositories_pending": ["Catholic Archives"],
    #     "years_covered": "1820-1880"
    #   }
    # }
    
    # Temporal coverage by location
    temporal_coverage: Dict[str, Dict[str, float]] = field(default_factory=dict)
    # {
    #   "Boston, MA": {
    #     "1820-1830": 0.90,
    #     "1830-1840": 0.85,
    #     "1840-1850": 0.95
    #   }
    # }
    
    # Repository tracking with collections
    repositories_searched: Dict[str, Dict[str, Any]] = field(default_factory=dict)
    # {
    #   "Massachusetts State Archives": {
    #     "date_searched": "2025-07-30",
    #     "collections": {
    #       "Vital Records": {"years": "1820-1880", "completeness": 0.95},
    #       "Probate Records": {"status": "not searched"}
    #     }
    #   }
    # }
    
    # Record type coverage
    record_type_coverage: Dict[str, Dict[str, Any]] = field(default_factory=dict)
    # {
    #   "vital_records": {
    #     "births": 0.90,
    #     "marriages": 0.85,
    #     "deaths": 0.80
    #   },
    #   "census": {
    #     "federal": 1.0,
    #     "state": 0.50
    #   }
    # }
    
    # Negative searches documented
    negative_searches: List[Dict[str, Any]] = field(default_factory=list)
    
    def calculate_overall_coverage(self) -> float:
        """Calculate weighted overall research coverage"""
        weights = {
            'geographic': 0.25,
            'temporal': 0.25,
            'repository': 0.25,
            'record_type': 0.25
        }
        
        scores = {}
        
        # Geographic score
        if self.geographic_coverage:
            geo_scores = [loc['coverage'] for loc in self.geographic_coverage.values() 
                         if 'coverage' in loc]
            scores['geographic'] = sum(geo_scores) / len(geo_scores) if geo_scores else 0
        
        # Temporal score
        if self.temporal_coverage:
            temp_scores = []
            for location_coverage in self.temporal_coverage.values():
                temp_scores.extend(location_coverage.values())
            scores['temporal'] = sum(temp_scores) / len(temp_scores) if temp_scores else 0
        
        # Repository score
        if self.repositories_searched:
            # Count repositories with substantial searching
            searched = sum(1 for repo in self.repositories_searched.values()
                         if repo.get('collections'))
            total = len(self.repositories_searched) + len(self.repositories_searched)
            scores['repository'] = searched / total if total > 0 else 0
        
        # Record type score
        if self.record_type_coverage:
            type_scores = []
            for category in self.record_type_coverage.values():
                type_scores.extend(category.values())
            scores['record_type'] = sum(type_scores) / len(type_scores) if type_scores else 0
        
        # Calculate weighted average
        total_score = sum(scores.get(key, 0) * weight 
                         for key, weight in weights.items())
        return total_score


@dataclass
class AuditCheckItem:
    """Enhanced audit checklist item with research tracking"""
    check_id: UUID = field(default_factory=uuid4)
    category: str = ""  # "Repository", "Record Type", "Analysis", etc.
    description: str = ""
    
    # Planning
    priority: str = "medium"  # "high", "medium", "low"
    estimated_hours: float = 0.0
    dependencies: List[UUID] = field(default_factory=list)  # Other checks that must complete first
    assigned_to: Optional[UUID] = None  # Researcher ID
    
    # Execution
    completed: bool = False
    completed_by_id: Optional[UUID] = None  # Researcher ID
    completed_by_name: Optional[str] = None  # For display
    completed_date: Optional[datetime] = None
    actual_hours: float = 0.0
    
    # Results
    result: str = ""
    evidence_found: List[UUID] = field(default_factory=list)
    evidence_quality: Optional[str] = None
    
    # Follow-up
    next_steps: List[str] = field(default_factory=list)
    spawned_checks: List[UUID] = field(default_factory=list)  # New checks created from this one
    
    # GPS compliance impact
    gps_element: Optional[str] = None  # Which GPS element this supports
    compliance_impact: Optional[str] = None  # How this affects GPS compliance


@dataclass
class Confidence(NestableBaseEntity['Confidence']):
    """
    Revolutionary confidence container - not just a score but a complete
    research narrative. This is what separates professional genealogy
    from simple family trees.
    """
    
    # Multiple assessments over time (the journey)
    assessments: List[ConfidenceAssessment] = field(default_factory=list)
    
    # Supporting analyses (NEW)
    supporting_analyses: List[UUID] = field(default_factory=list)  # Analysis entity IDs
    # Can contain multiple Analysis objects that build confidence:
    # - Correlation analyses
    # - Conflict resolution analyses  
    # - Pattern analyses
    # - Timeline analyses
    
    # Current state
    state: ConfidenceState = ConfidenceState.BUILDING
    state_history: List[Dict[str, Any]] = field(default_factory=list)
    
    # Computed summary (for quick reference)
    current_confidence: ConfidenceLevel = ConfidenceLevel.SPECULATIVE
    summary_score: float = 0.0  # 0-1
    
    # Research coverage tracking
    coverage: ResearchCoverage = field(default_factory=ResearchCoverage)
    
    # Detailed audit trail
    audit_checklist: List[AuditCheckItem] = field(default_factory=list)
    audit_categories: Dict[str, List[UUID]] = field(default_factory=dict)  # Category -> check IDs
    
    # GPS compliance tracking
    gps_compliance: Dict[str, Any] = field(default_factory=dict)
    # {
    #   "standard_version": "GPS-2025",
    #   "elements": {
    #     "reasonably_exhaustive": {
    #       "status": True,
    #       "evidence": "Searched all available...",
    #       "gaps": ["Church records unavailable"]
    #     },
    #     ...
    #   },
    #   "overall_compliance": 0.85,
    #   "certification_eligible": False
    # }
    
    # Methodology tracking
    methodologies_used: Set[str] = field(default_factory=set)
    primary_methodology: Optional[str] = None
    
    # For complex confidence (nested assessments)
    sub_assessments: Dict[str, 'Confidence'] = field(default_factory=dict)
    # {
    #   "identity_confidence": Confidence(),
    #   "relationship_confidence": Confidence(),
    #   "event_confidence": Confidence()
    # }
    
    # Professional metadata
    lead_researcher_id: UUID = field(default_factory=uuid4)  # Researcher ID
    lead_researcher_name: str = ""  # For display
    research_team_ids: List[UUID] = field(default_factory=list)  # Researcher IDs
    research_team_names: List[str] = field(default_factory=list)  # For display
    total_hours: float = 0.0
    date_range: Optional[tuple[datetime, datetime]] = None
    
    # Publication/sharing
    publishable: bool = False
    embargo_until: Optional[datetime] = None
    citation_text: str = ""
    
    def add_assessment(self, assessment: ConfidenceAssessment) -> None:
        """Add new assessment and recompute confidence"""
        self.assessments.append(assessment)
        self.methodologies_used.add(assessment.methodology.value)
        self._recompute_confidence()
        self._check_state_transition()
        self.updated_at = datetime.utcnow()
    
    def add_supporting_analysis(self, analysis_id: UUID) -> None:
        """Add a supporting Analysis entity"""
        if analysis_id not in self.supporting_analyses:
            self.supporting_analyses.append(analysis_id)
            self._recompute_confidence()
            self.updated_at = datetime.utcnow()
    
    def add_audit_check(self, 
                       description: str, 
                       category: str,
                       priority: str = "medium",
                       gps_element: Optional[str] = None) -> AuditCheckItem:
        """Add a new audit check item"""
        item = AuditCheckItem(
            description=description,
            category=category,
            priority=priority,
            gps_element=gps_element
        )
        self.audit_checklist.append(item)
        
        # Track by category
        if category not in self.audit_categories:
            self.audit_categories[category] = []
        self.audit_categories[category].append(item.check_id)
        
        return item
    
    def complete_audit_check(self, 
                           check_id: UUID,
                           result: str,
                           completed_by_id: UUID,  # Researcher ID
                           completed_by_name: str,  # For display
                           evidence_found: List[UUID] = None,
                           next_steps: List[str] = None) -> bool:
        """Complete an audit check with results"""
        for item in self.audit_checklist:
            if item.check_id == check_id:
                item.completed = True
                item.completed_by_id = completed_by_id
                item.completed_by_name = completed_by_name
                item.completed_date = datetime.utcnow()
                item.result = result
                if evidence_found:
                    item.evidence_found.extend(evidence_found)
                if next_steps:
                    item.next_steps.extend(next_steps)
                
                self._recompute_confidence()
                self._update_gps_compliance()
                return True
        return False
    
    def _recompute_confidence(self) -> None:
        """Recompute summary confidence from all assessments"""
        if not self.assessments and not self.supporting_analyses:
            self.summary_score = 0.0
            self.current_confidence = ConfidenceLevel.SPECULATIVE
            return
        
        # Weight recent assessments more heavily
        weights = []
        scores = []
        
        for i, assessment in enumerate(self.assessments):
            age_days = (datetime.utcnow() - assessment.assessment_date).days
            weight = 1.0 / (1.0 + age_days / 365.0)  # Decay over time
            
            # Boost weight for peer-reviewed assessments
            if assessment.peer_reviewed:
                weight *= 1.5
            
            weights.append(weight)
            scores.append(assessment.calculate_summary_score())
        
        # Calculate weighted average
        if weights:
            total_weight = sum(weights)
            weighted_score = sum(s * w for s, w in zip(scores, weights)) / total_weight
        else:
            weighted_score = 0.0
        
        # Factor in research coverage
        coverage_score = self.coverage.calculate_overall_coverage()
        
        # Factor in audit completion
        if self.audit_checklist:
            completed = sum(1 for item in self.audit_checklist if item.completed)
            audit_score = completed / len(self.audit_checklist)
        else:
            audit_score = 0.0
        
        # Factor in supporting analyses
        analysis_boost = 0.0
        if self.supporting_analyses:
            # Each supporting analysis adds confidence
            analysis_boost = min(0.1 * len(self.supporting_analyses), 0.3)
        
        # Combine factors
        base_score = (
            weighted_score * 0.5 +
            coverage_score * 0.3 +
            audit_score * 0.2
        )
        
        # Apply analysis boost
        self.summary_score = min(base_score + analysis_boost, 1.0)
        
        # Map to confidence level
        if self.summary_score >= 0.9:
            self.current_confidence = ConfidenceLevel.CERTAIN
        elif self.summary_score >= 0.75:
            self.current_confidence = ConfidenceLevel.HIGH
        elif self.summary_score >= 0.5:
            self.current_confidence = ConfidenceLevel.MODERATE
        elif self.summary_score >= 0.25:
            self.current_confidence = ConfidenceLevel.LOW
        else:
            self.current_confidence = ConfidenceLevel.SPECULATIVE
    
    def _check_state_transition(self) -> None:
        """Check if confidence should transition states"""
        if self.state == ConfidenceState.BUILDING:
            # Check if ready for review
            if (len(self.assessments) >= 1 and 
                self.coverage.calculate_overall_coverage() >= 0.7):
                self.transition_to(ConfidenceState.REVIEWING)
        
        elif self.state == ConfidenceState.REVIEWING:
            # Check if peer reviewed
            peer_reviewed = any(a.peer_reviewed for a in self.assessments)
            if peer_reviewed:
                self.transition_to(ConfidenceState.ACCEPTED)
    
    def transition_to(self, new_state: ConfidenceState) -> None:
        """Transition to a new state"""
        transition = {
            'from_state': self.state.value,
            'to_state': new_state.value,
            'timestamp': datetime.utcnow(),
            'reason': f"Automatic transition based on confidence metrics"
        }
        self.state_history.append(transition)
        self.state = new_state
    
    def _update_gps_compliance(self) -> None:
        """Update GPS compliance based on current state"""
        # This would load GPS configuration and check compliance
        # For now, a simplified version:
        elements = {
            'reasonably_exhaustive': self.coverage.calculate_overall_coverage() >= 0.8,
            'complete_citations': True,  # Would check all evidence has citations
            'thorough_analysis': len(self.assessments) > 0,
            'conflicts_resolved': True,  # Would check for unresolved conflicts
            'written_conclusion': any(a.narrative for a in self.assessments)
        }
        
        self.gps_compliance = {
            'standard_version': 'GPS-2025',
            'elements': elements,
            'overall_compliance': sum(elements.values()) / len(elements),
            'certification_eligible': all(elements.values())
        }
    
    def get_missing_research(self) -> List[str]:
        """Get list of research that still needs to be done"""
        missing = []
        
        # Uncompleted audit items
        for item in self.audit_checklist:
            if not item.completed:
                missing.append(f"{item.category}: {item.description}")
        
        # Geographic gaps
        for location, data in self.coverage.geographic_coverage.items():
            if data.get('coverage', 0) < 0.8:
                missing.append(f"Geographic: Improve coverage for {location}")
            for repo in data.get('repositories_pending', []):
                missing.append(f"Repository: Search {repo} for {location}")
        
        # Record type gaps
        for category, types in self.coverage.record_type_coverage.items():
            for record_type, coverage in types.items():
                if coverage < 0.8:
                    missing.append(f"Records: Improve {record_type} coverage")
        
        # GPS compliance gaps
        if self.gps_compliance:
            elements = self.gps_compliance.get('elements', {})
            if not elements.get('reasonably_exhaustive'):
                missing.append("GPS: Complete reasonably exhaustive research")
            if not elements.get('written_conclusion'):
                missing.append("GPS: Write formal conclusion")
        
        return missing
    
    def export_narrative(self) -> str:
        """Export confidence as a narrative document"""
        sections = []
        
        # Header
        sections.append(f"# Confidence Assessment")
        sections.append(f"**Current Level**: {self.current_confidence.value}")
        sections.append(f"**Score**: {self.summary_score:.1%}")
        sections.append(f"**State**: {self.state.value}")
        sections.append("")
        
        # Research Coverage
        sections.append("## Research Coverage")
        sections.append(f"Overall Coverage: {self.coverage.calculate_overall_coverage():.1%}")
        sections.append("")
        
        # Individual Assessments
        sections.append("## Assessments")
        for i, assessment in enumerate(self.assessments, 1):
            sections.append(f"### Assessment {i}")
            sections.append(f"**Date**: {assessment.assessment_date}")
            sections.append(f"**Assessor**: {assessment.assessor_name} ({assessment.assessor_credentials})")
            sections.append(f"**Methodology**: {assessment.methodology.value}")
            if assessment.detailed_analysis_id:
                sections.append(f"**Detailed Analysis**: See Analysis {assessment.detailed_analysis_id}")
            sections.append("")
            sections.append("**Narrative**:")
            sections.append(assessment.narrative)
            sections.append("")
        
        # Supporting Analyses
        if self.supporting_analyses:
            sections.append("## Supporting Analyses")
            sections.append(f"This confidence assessment is supported by {len(self.supporting_analyses)} detailed analyses:")
            for analysis_id in self.supporting_analyses:
                sections.append(f"- Analysis {analysis_id}")
            sections.append("")
        
        # GPS Compliance
        if self.gps_compliance:
            sections.append("## GPS Compliance")
            elements = self.gps_compliance.get('elements', {})
            for element, status in elements.items():
                sections.append(f"- {element}: {'✓' if status else '✗'}")
            sections.append("")
        
        # Missing Research
        missing = self.get_missing_research()
        if missing:
            sections.append("## Research Needed")
            for item in missing:
                sections.append(f"- {item}")
        
        return "\n".join(sections)