"""
Confidence tracking models for ResearchProcess-GPS.

Instead of simple numeric confidence scores, RGPS tracks comprehensive
confidence with audit trails, GPS compliance, and research coverage.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum

from ...protocols.entity_protocol import ConfidenceLevel


class EvidenceQuality(Enum):
    """Quality assessment for evidence"""
    ORIGINAL = "original"
    HIGH_QUALITY_DERIVATIVE = "high_quality_derivative"
    DERIVATIVE = "derivative"
    QUESTIONABLE = "questionable"
    UNRELIABLE = "unreliable"


class InformationType(Enum):
    """GPS classification of information type"""
    PRIMARY = "primary"      # Created at time of event
    SECONDARY = "secondary"  # Created after event from primary
    INDETERMINATE = "indeterminate"  # Cannot determine


class EvidenceType(Enum):
    """GPS classification of evidence type"""
    DIRECT = "direct"        # Directly states conclusion
    INDIRECT = "indirect"    # Requires inference
    NEGATIVE = "negative"    # Absence of expected evidence


@dataclass
class ResearchCoverage:
    """Tracks what research has been conducted"""
    
    # Geographic coverage
    geographic_coverage: Dict[str, float] = field(default_factory=dict)  # location -> coverage %
    
    # Temporal coverage  
    temporal_coverage: Dict[str, float] = field(default_factory=dict)  # time_period -> coverage %
    
    # Repository coverage
    repositories_searched: Set[str] = field(default_factory=set)
    repositories_not_searched: Set[str] = field(default_factory=set)
    
    # Record type coverage
    record_types_searched: Set[str] = field(default_factory=set)
    record_types_not_searched: Set[str] = field(default_factory=set)
    
    # Online database coverage
    databases_searched: Set[str] = field(default_factory=set)
    databases_not_searched: Set[str] = field(default_factory=set)
    
    def calculate_overall_coverage(self) -> float:
        """Calculate overall research coverage percentage"""
        total_items = 0
        covered_items = 0
        
        # Geographic
        for coverage in self.geographic_coverage.values():
            total_items += 1
            covered_items += coverage
        
        # Temporal
        for coverage in self.temporal_coverage.values():
            total_items += 1
            covered_items += coverage
        
        # Repositories (binary: searched or not)
        total_repos = len(self.repositories_searched) + len(self.repositories_not_searched)
        if total_repos > 0:
            total_items += total_repos
            covered_items += len(self.repositories_searched)
        
        # Record types
        total_types = len(self.record_types_searched) + len(self.record_types_not_searched)
        if total_types > 0:
            total_items += total_types
            covered_items += len(self.record_types_searched)
        
        return covered_items / total_items if total_items > 0 else 0.0


@dataclass
class AuditCheckItem:
    """A specific item that was checked during research"""
    check_id: UUID = field(default_factory=uuid4)
    description: str = ""  # "Checked Smith County birth records 1850-1860"
    completed: bool = False
    completed_by: Optional[str] = None
    completed_date: Optional[datetime] = None
    result: str = ""  # What was found or not found
    evidence_references: List[UUID] = field(default_factory=list)
    next_steps: str = ""  # Suggested follow-up


@dataclass
class GPSCompliance:
    """Track compliance with Genealogical Proof Standard elements"""
    
    # Element 1: Reasonably exhaustive research
    reasonably_exhaustive: bool = False
    exhaustive_notes: str = ""
    research_coverage: Optional[ResearchCoverage] = None
    
    # Element 2: Complete and accurate citation
    complete_citations: bool = False
    citation_issues: List[str] = field(default_factory=list)
    
    # Element 3: Thorough analysis and correlation
    thorough_analysis: bool = False
    analysis_description: str = ""
    correlation_performed: bool = False
    
    # Element 4: Resolution of conflicts
    conflicts_identified: List[str] = field(default_factory=list)
    conflicts_resolved: Dict[str, str] = field(default_factory=dict)  # conflict -> resolution
    unresolved_conflicts: List[str] = field(default_factory=list)
    
    # Element 5: Sound written conclusion
    written_conclusion: bool = False
    conclusion_location: str = ""  # Where the conclusion is documented
    
    def is_compliant(self) -> bool:
        """Check if all GPS elements are satisfied"""
        return all([
            self.reasonably_exhaustive,
            self.complete_citations,
            self.thorough_analysis,
            len(self.unresolved_conflicts) == 0,
            self.written_conclusion
        ])
    
    def get_compliance_score(self) -> float:
        """Get a percentage score for GPS compliance"""
        elements = [
            self.reasonably_exhaustive,
            self.complete_citations,
            self.thorough_analysis,
            len(self.unresolved_conflicts) == 0,
            self.written_conclusion
        ]
        return sum(1 for e in elements if e) / len(elements)


@dataclass
class PeerAssessment:
    """Assessment by another researcher"""
    assessment_id: UUID = field(default_factory=uuid4)
    reviewer_id: str = ""
    review_date: datetime = field(default_factory=datetime.utcnow)
    confidence_given: ConfidenceLevel = ConfidenceLevel.MODERATE
    
    # Detailed feedback
    strengths: List[str] = field(default_factory=list)
    weaknesses: List[str] = field(default_factory=list)
    suggestions: List[str] = field(default_factory=list)
    
    # Specific assessments
    evidence_quality_score: float = 0.5  # 0-1
    analysis_quality_score: float = 0.5  # 0-1
    citation_quality_score: float = 0.5  # 0-1
    
    # Overall recommendation
    recommendation: str = ""  # "accept", "revise", "reject"
    
    def overall_score(self) -> float:
        """Calculate overall peer assessment score"""
        return (self.evidence_quality_score + 
                self.analysis_quality_score + 
                self.citation_quality_score) / 3


@dataclass
class ConfidenceContainer:
    """
    Comprehensive confidence tracking that goes beyond simple numbers.
    This is what makes RGPS revolutionary for genealogical research.
    """
    
    # Summary (for compatibility and quick reference)
    summary_confidence: ConfidenceLevel = ConfidenceLevel.SPECULATIVE
    
    # Evidence quality breakdown
    evidence_quality: Dict[str, float] = field(default_factory=dict)
    source_reliability: float = 0.0  # 0-1
    information_credibility: float = 0.0  # 0-1
    evidence_directness: float = 0.0  # 0-1
    
    # Research completeness
    research_coverage: ResearchCoverage = field(default_factory=ResearchCoverage)
    
    # Detailed audit trail
    audit_checklist: List[AuditCheckItem] = field(default_factory=list)
    
    # GPS compliance tracking
    gps_compliance: GPSCompliance = field(default_factory=GPSCompliance)
    
    # Peer review
    peer_assessments: List[PeerAssessment] = field(default_factory=list)
    
    # History
    confidence_history: List[Dict[str, Any]] = field(default_factory=list)
    last_updated: datetime = field(default_factory=datetime.utcnow)
    
    def add_audit_check(self, description: str) -> AuditCheckItem:
        """Add a new audit check item"""
        item = AuditCheckItem(description=description)
        self.audit_checklist.append(item)
        return item
    
    def complete_audit_check(self, check_id: UUID, result: str, 
                           completed_by: str, evidence_refs: List[UUID] = None) -> bool:
        """Mark an audit check as complete"""
        for item in self.audit_checklist:
            if item.check_id == check_id:
                item.completed = True
                item.completed_by = completed_by
                item.completed_date = datetime.utcnow()
                item.result = result
                if evidence_refs:
                    item.evidence_references.extend(evidence_refs)
                self._update_confidence()
                return True
        return False
    
    def add_peer_assessment(self, assessment: PeerAssessment) -> None:
        """Add a peer review assessment"""
        self.peer_assessments.append(assessment)
        self._update_confidence()
    
    def _update_confidence(self) -> None:
        """Recalculate summary confidence based on all factors"""
        scores = []
        
        # Evidence quality score
        if self.evidence_quality:
            avg_quality = sum(self.evidence_quality.values()) / len(self.evidence_quality)
            scores.append(avg_quality)
        
        # Research coverage score
        coverage_score = self.research_coverage.calculate_overall_coverage()
        scores.append(coverage_score)
        
        # Audit completion score
        if self.audit_checklist:
            completed = sum(1 for item in self.audit_checklist if item.completed)
            audit_score = completed / len(self.audit_checklist)
            scores.append(audit_score)
        
        # GPS compliance score
        gps_score = self.gps_compliance.get_compliance_score()
        scores.append(gps_score)
        
        # Peer review score
        if self.peer_assessments:
            peer_score = sum(a.overall_score() for a in self.peer_assessments) / len(self.peer_assessments)
            scores.append(peer_score)
        
        # Calculate overall score
        if scores:
            overall = sum(scores) / len(scores)
            
            # Map to confidence level
            if overall >= 0.9:
                self.summary_confidence = ConfidenceLevel.CERTAIN
            elif overall >= 0.75:
                self.summary_confidence = ConfidenceLevel.HIGH
            elif overall >= 0.5:
                self.summary_confidence = ConfidenceLevel.MODERATE
            elif overall >= 0.25:
                self.summary_confidence = ConfidenceLevel.LOW
            else:
                self.summary_confidence = ConfidenceLevel.SPECULATIVE
        
        # Record in history
        self.confidence_history.append({
            'timestamp': datetime.utcnow(),
            'confidence_level': self.summary_confidence,
            'scores': scores,
            'factors_considered': len(scores)
        })
        
        self.last_updated = datetime.utcnow()
    
    def get_missing_research(self) -> List[str]:
        """Get list of research that still needs to be done"""
        missing = []
        
        # Check audit items
        for item in self.audit_checklist:
            if not item.completed:
                missing.append(f"Audit: {item.description}")
        
        # Check repositories
        for repo in self.research_coverage.repositories_not_searched:
            missing.append(f"Repository: {repo}")
        
        # Check record types
        for record_type in self.research_coverage.record_types_not_searched:
            missing.append(f"Record Type: {record_type}")
        
        # Check GPS compliance
        if not self.gps_compliance.reasonably_exhaustive:
            missing.append("GPS: Complete reasonably exhaustive research")
        if not self.gps_compliance.complete_citations:
            missing.append("GPS: Ensure all citations are complete")
        if self.gps_compliance.unresolved_conflicts:
            missing.append(f"GPS: Resolve {len(self.gps_compliance.unresolved_conflicts)} conflicts")
        
        return missing
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization"""
        return {
            'summary_confidence': self.summary_confidence.value,
            'evidence_quality': self.evidence_quality,
            'source_reliability': self.source_reliability,
            'information_credibility': self.information_credibility,
            'evidence_directness': self.evidence_directness,
            'research_coverage': {
                'overall': self.research_coverage.calculate_overall_coverage(),
                'geographic': dict(self.research_coverage.geographic_coverage),
                'temporal': dict(self.research_coverage.temporal_coverage),
                'repositories_searched': list(self.research_coverage.repositories_searched),
                'repositories_not_searched': list(self.research_coverage.repositories_not_searched)
            },
            'gps_compliance': {
                'compliant': self.gps_compliance.is_compliant(),
                'score': self.gps_compliance.get_compliance_score(),
                'elements': {
                    'reasonably_exhaustive': self.gps_compliance.reasonably_exhaustive,
                    'complete_citations': self.gps_compliance.complete_citations,
                    'thorough_analysis': self.gps_compliance.thorough_analysis,
                    'conflicts_resolved': len(self.gps_compliance.unresolved_conflicts) == 0,
                    'written_conclusion': self.gps_compliance.written_conclusion
                }
            },
            'audit_items_completed': sum(1 for item in self.audit_checklist if item.completed),
            'audit_items_total': len(self.audit_checklist),
            'peer_assessments': len(self.peer_assessments),
            'last_updated': self.last_updated.isoformat()
        }