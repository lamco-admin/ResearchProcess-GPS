"""
ResearchProcess-GPS Theory & Evidence Protocol

Defines how theories are managed, branched, merged, and how evidence
floats between them. This is the core innovation of RGPS.
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime
from uuid import UUID
from enum import Enum
from dataclasses import dataclass

from .entity_protocol import EntityProtocol, ConfidenceContainer


class TheoryState(Enum):
    """States a theory can be in"""
    DRAFT = "draft"
    ACTIVE = "active"
    TESTING = "testing"
    VALIDATED = "validated"
    REFUTED = "refuted"
    MERGED = "merged"
    ARCHIVED = "archived"


class EvidenceSupport(Enum):
    """How evidence relates to a theory"""
    STRONGLY_SUPPORTS = 1.0
    SUPPORTS = 0.75
    NEUTRAL = 0.5
    CONTRADICTS = 0.25
    STRONGLY_CONTRADICTS = 0.0


@dataclass
class TheoryComparison:
    """Result of comparing two theories"""
    theory1_id: UUID
    theory2_id: UUID
    common_entities: Set[UUID]
    unique_to_theory1: Set[UUID]
    unique_to_theory2: Set[UUID]
    conflicting_entities: List[Tuple[UUID, str]]  # entity_id, conflict_description
    compatibility_score: float
    merge_difficulty: str  # easy, moderate, hard, impossible


@dataclass
class MergeResult:
    """Result of merging theories"""
    success: bool
    merged_theory_id: Optional[UUID]
    conflicts_resolved: int
    conflicts_remaining: List[Dict[str, Any]]
    entities_merged: int
    merge_log: List[str]


class TheoryEngineProtocol(ABC):
    """
    Core protocol for theory management - the revolutionary feature of RGPS.
    Theories are containers for different interpretations of genealogical data.
    """
    
    @abstractmethod
    def create_theory(self, hypothesis: str, parent_theory_id: Optional[UUID] = None) -> UUID:
        """
        Create a new theory, optionally branched from a parent.
        Returns the new theory ID.
        """
        pass
    
    @abstractmethod
    def get_theory(self, theory_id: UUID) -> Dict[str, Any]:
        """Get theory details"""
        pass
    
    @abstractmethod
    def list_theories(self, include_archived: bool = False) -> List[Dict[str, Any]]:
        """List all theories"""
        pass
    
    @abstractmethod
    def get_theory_state(self, theory_id: UUID) -> TheoryState:
        """Get current state of a theory"""
        pass
    
    @abstractmethod
    def set_theory_state(self, theory_id: UUID, state: TheoryState) -> bool:
        """Update theory state"""
        pass
    
    # Entity Management within Theories
    
    @abstractmethod
    def add_entity_to_theory(self, theory_id: UUID, entity: EntityProtocol) -> bool:
        """Add or update an entity in a theory"""
        pass
    
    @abstractmethod
    def remove_entity_from_theory(self, theory_id: UUID, entity_id: UUID) -> bool:
        """Remove an entity from a theory"""
        pass
    
    @abstractmethod
    def get_entity_in_theory(self, theory_id: UUID, entity_id: UUID) -> Optional[EntityProtocol]:
        """Get an entity as it exists in a specific theory"""
        pass
    
    @abstractmethod
    def list_theory_entities(self, theory_id: UUID, entity_type: Optional[str] = None) -> List[EntityProtocol]:
        """List all entities in a theory, optionally filtered by type"""
        pass
    
    # Theory Comparison and Merging
    
    @abstractmethod
    def compare_theories(self, theory1_id: UUID, theory2_id: UUID) -> TheoryComparison:
        """Compare two theories to understand differences"""
        pass
    
    @abstractmethod
    def preview_merge(self, source_theory_id: UUID, target_theory_id: UUID) -> Dict[str, Any]:
        """Preview what would happen in a merge without doing it"""
        pass
    
    @abstractmethod
    def merge_theories(self, source_theory_id: UUID, target_theory_id: UUID,
                      conflict_resolution: Optional[Dict[UUID, str]] = None) -> MergeResult:
        """
        Merge source theory into target theory.
        Conflict resolution maps entity IDs to resolution strategies.
        """
        pass
    
    @abstractmethod
    def branch_theory(self, parent_theory_id: UUID, new_hypothesis: str,
                     entity_filter: Optional[List[UUID]] = None) -> UUID:
        """
        Create a new theory branch from parent.
        Optionally include only specific entities.
        """
        pass
    
    # Evidence Management
    
    @abstractmethod
    def float_evidence(self, evidence_id: UUID, from_theory_id: UUID, to_theory_id: UUID) -> bool:
        """Move evidence from one theory to another"""
        pass
    
    @abstractmethod
    def share_evidence(self, evidence_id: UUID, theory_ids: List[UUID]) -> bool:
        """Share evidence across multiple theories"""
        pass
    
    @abstractmethod
    def get_evidence_theories(self, evidence_id: UUID) -> List[UUID]:
        """Get all theories that use this evidence"""
        pass
    
    # Theory Validation
    
    @abstractmethod
    def validate_theory(self, theory_id: UUID) -> Dict[str, Any]:
        """
        Validate theory for internal consistency, GPS compliance, etc.
        Returns validation results.
        """
        pass
    
    @abstractmethod
    def calculate_theory_confidence(self, theory_id: UUID) -> ConfidenceContainer:
        """Calculate overall confidence for a theory"""
        pass
    
    @abstractmethod
    def get_theory_gaps(self, theory_id: UUID) -> List[Dict[str, Any]]:
        """Identify research gaps in a theory"""
        pass


class EvidenceEngineProtocol(ABC):
    """
    Protocol for evidence management - evidence can float between theories
    and support or contradict different interpretations.
    """
    
    @abstractmethod
    def create_evidence(self, source_data: Dict[str, Any]) -> UUID:
        """Create new evidence from source data"""
        pass
    
    @abstractmethod
    def classify_evidence(self, evidence_id: UUID) -> Dict[str, str]:
        """
        Classify evidence according to GPS standards:
        - source_type: original, derivative, authored
        - information_type: primary, secondary, indeterminate
        - evidence_type: direct, indirect, negative
        """
        pass
    
    @abstractmethod
    def extract_facts(self, evidence_id: UUID) -> List[Dict[str, Any]]:
        """Extract facts from evidence"""
        pass
    
    @abstractmethod
    def link_fact_to_entity(self, fact_id: UUID, entity_id: UUID, theory_id: UUID) -> bool:
        """Link an extracted fact to an entity in a theory"""
        pass
    
    # Evidence-Theory Relationships
    
    @abstractmethod
    def evaluate_evidence_support(self, evidence_id: UUID, theory_id: UUID) -> EvidenceSupport:
        """Evaluate how well evidence supports a theory"""
        pass
    
    @abstractmethod
    def find_contradicting_evidence(self, theory_id: UUID) -> List[Tuple[UUID, str]]:
        """Find evidence that contradicts a theory"""
        pass
    
    @abstractmethod
    def find_supporting_evidence(self, theory_id: UUID) -> List[Tuple[UUID, float]]:
        """Find evidence that supports a theory with support strength"""
        pass
    
    # Evidence Quality
    
    @abstractmethod
    def assess_evidence_quality(self, evidence_id: UUID) -> Dict[str, float]:
        """
        Assess evidence quality metrics:
        - reliability: 0-1
        - completeness: 0-1
        - legibility: 0-1
        - provenance_chain: 0-1
        """
        pass
    
    @abstractmethod
    def track_evidence_usage(self, evidence_id: UUID) -> Dict[str, Any]:
        """Track how evidence is used across theories"""
        pass
    
    # Negative Evidence
    
    @abstractmethod
    def record_negative_evidence(self, search_parameters: Dict[str, Any], theory_id: UUID) -> UUID:
        """Record that a search found no results (negative evidence)"""
        pass
    
    @abstractmethod
    def get_negative_evidence(self, theory_id: UUID) -> List[Dict[str, Any]]:
        """Get all negative evidence for a theory"""
        pass


class TheoryMergeStrategy(ABC):
    """
    Abstract strategy for merging theories.
    Different strategies can be implemented for different merge scenarios.
    """
    
    @abstractmethod
    def can_merge(self, source_theory: Dict[str, Any], target_theory: Dict[str, Any]) -> bool:
        """Check if theories can be merged with this strategy"""
        pass
    
    @abstractmethod
    def identify_conflicts(self, source_theory: Dict[str, Any], 
                         target_theory: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Identify all conflicts between theories"""
        pass
    
    @abstractmethod
    def propose_resolutions(self, conflicts: List[Dict[str, Any]]) -> Dict[UUID, str]:
        """Propose automatic resolutions for conflicts"""
        pass
    
    @abstractmethod
    def execute_merge(self, source_theory: Dict[str, Any], target_theory: Dict[str, Any],
                     resolutions: Dict[UUID, str]) -> MergeResult:
        """Execute the merge with given resolutions"""
        pass