"""
Theory model for ResearchProcess-GPS.

Theories are containers for different interpretations of genealogical data.
They can be branched, merged, and compared - like Git branches for genealogy.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum

from .base import NestableBaseEntity
from .confidence import ConfidenceContainer
from ...protocols.theory_protocol import TheoryState, TheoryComparison


class TheoryType(Enum):
    """Types of theories"""
    MAIN = "main"              # The main/primary theory
    EXPLORATION = "exploration" # Exploring "what if" scenarios  
    EVIDENCE = "evidence"      # Adding new evidence
    CORRECTION = "correction"  # Correcting errors
    MERGE = "merge"           # Result of merging theories
    ARCHIVE = "archive"       # Archived/historical theory


@dataclass
class TheoryMetrics:
    """Metrics for evaluating a theory"""
    entity_count: int = 0
    evidence_count: int = 0
    conflict_count: int = 0
    gap_count: int = 0
    gps_compliance_score: float = 0.0
    peer_review_score: float = 0.0
    last_calculated: Optional[datetime] = None


@dataclass
class ResearchGap:
    """A gap in research identified in a theory"""
    gap_id: UUID = field(default_factory=uuid4)
    description: str = ""
    gap_type: str = ""  # "missing_evidence", "temporal_gap", "geographic_gap", etc.
    affected_entities: List[UUID] = field(default_factory=list)
    suggested_actions: List[str] = field(default_factory=list)
    priority: str = "medium"  # low, medium, high
    identified_date: datetime = field(default_factory=datetime.utcnow)


@dataclass
class TheoryConflict:
    """A conflict within or between theories"""
    conflict_id: UUID = field(default_factory=uuid4)
    conflict_type: str = ""  # "date_conflict", "identity_conflict", "relationship_conflict"
    description: str = ""
    entities_involved: List[UUID] = field(default_factory=list)
    evidence_involved: List[UUID] = field(default_factory=list)
    proposed_resolutions: List[Dict[str, Any]] = field(default_factory=list)
    resolution_status: str = "unresolved"  # unresolved, partial, resolved


@dataclass
class Theory(NestableBaseEntity['Theory']):
    """
    A Theory is a container for a specific interpretation of genealogical data.
    It's like a Git branch - you can create multiple theories, test hypotheses,
    and merge the results back together.
    
    Now supports nesting for:
    - Theory hierarchies (main theory → sub-theories → experiments)
    - Theory evolution (v1 → v2 → v3)
    - Collaborative theories (combining multiple researchers' work)
    - Theory archives (historical theory development)
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Theory"
        from ..abstractions.nesting import NestingType
        self.nesting_type = NestingType.VERSIONED  # Theories evolve over time
    
    # Core theory properties
    hypothesis: str = ""
    theory_type: TheoryType = TheoryType.EXPLORATION
    state: TheoryState = TheoryState.DRAFT
    
    # Theory lineage (like Git branches)
    parent_theory: Optional[UUID] = None
    child_theories: List[UUID] = field(default_factory=list)
    merged_from: List[UUID] = field(default_factory=list)
    
    # Entities in this theory
    entities: Dict[UUID, str] = field(default_factory=dict)  # entity_id -> entity_type
    entity_versions: Dict[UUID, UUID] = field(default_factory=dict)  # entity_id -> version_id
    
    # Evidence evaluation
    supporting_evidence: Dict[UUID, float] = field(default_factory=dict)  # evidence_id -> support_level
    contradicting_evidence: Dict[UUID, str] = field(default_factory=dict)  # evidence_id -> contradiction_description
    
    # Research tracking
    research_gaps: List[ResearchGap] = field(default_factory=list)
    conflicts: List[TheoryConflict] = field(default_factory=list)
    assumptions: List[str] = field(default_factory=list)
    
    # Metrics and evaluation
    metrics: TheoryMetrics = field(default_factory=TheoryMetrics)
    
    # Collaboration
    owner: str = ""
    collaborators: List[str] = field(default_factory=list)
    peer_reviews: List[Dict[str, Any]] = field(default_factory=list)
    
    # Theory lifecycle
    created_date: datetime = field(default_factory=datetime.utcnow)
    last_modified: datetime = field(default_factory=datetime.utcnow)
    published_date: Optional[datetime] = None
    archived_date: Optional[datetime] = None
    
    def add_entity(self, entity_id: UUID, entity_type: str, version_id: UUID) -> None:
        """Add an entity to this theory"""
        self.entities[entity_id] = entity_type
        self.entity_versions[entity_id] = version_id
        self.last_modified = datetime.utcnow()
    
    def remove_entity(self, entity_id: UUID) -> bool:
        """Remove an entity from this theory"""
        if entity_id in self.entities:
            del self.entities[entity_id]
            del self.entity_versions[entity_id]
            self.last_modified = datetime.utcnow()
            return True
        return False
    
    def add_supporting_evidence(self, evidence_id: UUID, support_level: float) -> None:
        """Add evidence that supports this theory"""
        self.supporting_evidence[evidence_id] = support_level
        if evidence_id in self.contradicting_evidence:
            del self.contradicting_evidence[evidence_id]
    
    def add_contradicting_evidence(self, evidence_id: UUID, contradiction: str) -> None:
        """Add evidence that contradicts this theory"""
        self.contradicting_evidence[evidence_id] = contradiction
        if evidence_id in self.supporting_evidence:
            del self.supporting_evidence[evidence_id]
    
    def add_research_gap(self, gap: ResearchGap) -> None:
        """Identify a gap in research"""
        self.research_gaps.append(gap)
    
    def add_conflict(self, conflict: TheoryConflict) -> None:
        """Add a conflict that needs resolution"""
        self.conflicts.append(conflict)
    
    def calculate_confidence_score(self) -> float:
        """Calculate overall confidence in this theory"""
        if not self.metrics.last_calculated or \
           (datetime.utcnow() - self.metrics.last_calculated).seconds > 3600:
            self._recalculate_metrics()
        
        # Weight different factors
        weights = {
            'evidence_support': 0.3,
            'gps_compliance': 0.3,
            'conflict_resolution': 0.2,
            'peer_review': 0.2
        }
        
        # Calculate evidence support ratio
        total_evidence = len(self.supporting_evidence) + len(self.contradicting_evidence)
        evidence_ratio = len(self.supporting_evidence) / total_evidence if total_evidence > 0 else 0.5
        
        # Calculate conflict resolution ratio
        resolved_conflicts = sum(1 for c in self.conflicts if c.resolution_status == "resolved")
        conflict_ratio = resolved_conflicts / len(self.conflicts) if self.conflicts else 1.0
        
        # Combine scores
        score = (
            weights['evidence_support'] * evidence_ratio +
            weights['gps_compliance'] * self.metrics.gps_compliance_score +
            weights['conflict_resolution'] * conflict_ratio +
            weights['peer_review'] * self.metrics.peer_review_score
        )
        
        return min(max(score, 0.0), 1.0)
    
    def _recalculate_metrics(self) -> None:
        """Recalculate theory metrics"""
        self.metrics.entity_count = len(self.entities)
        self.metrics.evidence_count = len(self.supporting_evidence) + len(self.contradicting_evidence)
        self.metrics.conflict_count = len([c for c in self.conflicts if c.resolution_status != "resolved"])
        self.metrics.gap_count = len(self.research_gaps)
        self.metrics.last_calculated = datetime.utcnow()
        # TODO: Calculate GPS compliance score
        # TODO: Calculate peer review score
    
    def can_merge_with(self, other: 'Theory') -> Tuple[bool, List[str]]:
        """Check if this theory can be merged with another"""
        issues = []
        
        # Check for circular dependencies
        if other.id in self.child_theories:
            issues.append("Cannot merge with child theory")
        
        if self.id in other.child_theories:
            issues.append("Cannot merge parent into child")
        
        # Check for major conflicts
        conflicting_entities = set(self.entities.keys()) & set(other.entities.keys())
        for entity_id in conflicting_entities:
            if self.entity_versions[entity_id] != other.entity_versions[entity_id]:
                issues.append(f"Conflicting versions for entity {entity_id}")
        
        return len(issues) == 0, issues
    
    def branch(self, new_hypothesis: str) -> 'Theory':
        """Create a new theory branched from this one"""
        branch = Theory(
            hypothesis=new_hypothesis,
            theory_type=TheoryType.EXPLORATION,
            parent_theory=self.id,
            owner=self.owner
        )
        
        # Copy current state
        branch.entities = self.entities.copy()
        branch.entity_versions = self.entity_versions.copy()
        branch.assumptions = self.assumptions.copy()
        
        # Add to children
        self.child_theories.append(branch.id)
        
        return branch


@dataclass  
class TheoryBranch:
    """
    Represents a branch point in theory development.
    Used for tracking theory evolution and merges.
    """
    branch_id: UUID = field(default_factory=uuid4)
    parent_theory_id: UUID = field(default_factory=uuid4)
    child_theory_id: UUID = field(default_factory=uuid4)
    branch_date: datetime = field(default_factory=datetime.utcnow)
    branch_reason: str = ""
    branched_by: str = ""
    
    # What changed in the branch
    entities_added: List[UUID] = field(default_factory=list)
    entities_removed: List[UUID] = field(default_factory=list)
    entities_modified: List[UUID] = field(default_factory=list)