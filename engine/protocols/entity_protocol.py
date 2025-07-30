"""
ResearchProcess-GPS Entity Protocol

Defines the core protocol that all entities in the system must implement.
This is the foundation for the entire RGPS ecosystem.
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Any, Set
from datetime import datetime
from enum import Enum
from dataclasses import dataclass
from uuid import UUID


class PublicationState(Enum):
    """Multi-state publication model"""
    DRAFT = "draft"
    WORKING = "working"
    REVIEW = "review"
    PUBLISHED = "published"
    ARCHIVED = "archived"


class ConfidenceLevel(Enum):
    """GPS-compliant confidence levels"""
    SPECULATIVE = 0
    LOW = 25
    MODERATE = 50
    HIGH = 75
    CERTAIN = 100


@dataclass
class Version:
    """Version information for an entity"""
    version_id: UUID
    parent_version: Optional[UUID]
    created_at: datetime
    created_by: str
    change_description: str
    theory_context: Optional[UUID]


@dataclass
class Attribution:
    """Attribution information for research work"""
    contributor_id: str
    contribution_type: str  # created, edited, reviewed, verified
    timestamp: datetime
    description: str


@dataclass
class ConfidenceContainer:
    """Rich confidence tracking with GPS compliance"""
    summary_confidence: ConfidenceLevel
    evidence_quality: Dict[str, float]
    research_coverage: Dict[str, Any]
    gps_compliance: Dict[str, bool]
    peer_assessments: List[Dict[str, Any]]


class EntityProtocol(ABC):
    """
    Core protocol that all ResearchProcess-GPS entities must implement.
    This enables version control, theory management, and collaboration.
    """
    
    @abstractmethod
    def get_id(self) -> UUID:
        """Unique identifier for this entity"""
        pass
    
    @abstractmethod
    def get_type(self) -> str:
        """Entity type (Identity, Event, Evidence, etc.)"""
        pass
    
    @abstractmethod
    def get_version(self) -> Version:
        """Current version information"""
        pass
    
    @abstractmethod
    def get_publication_state(self) -> PublicationState:
        """Current publication state"""
        pass
    
    @abstractmethod
    def get_attributions(self) -> List[Attribution]:
        """All attributions for this entity"""
        pass
    
    @abstractmethod
    def get_confidence(self) -> ConfidenceContainer:
        """Confidence assessment for this entity"""
        pass
    
    @abstractmethod
    def get_data(self) -> Dict[str, Any]:
        """Entity-specific data"""
        pass
    
    @abstractmethod
    def get_references(self) -> Dict[str, List[UUID]]:
        """References to other entities"""
        pass
    
    @abstractmethod
    def applies_to_theories(self) -> Set[UUID]:
        """Which theories this entity exists in"""
        pass
    
    @abstractmethod
    def branch_for_theory(self, theory_id: UUID) -> 'EntityProtocol':
        """Create a version of this entity for a specific theory"""
        pass
    
    @abstractmethod
    def merge_from(self, other: 'EntityProtocol') -> 'EntityProtocol':
        """Merge another version of this entity"""
        pass
    
    @abstractmethod
    def validate(self) -> List[str]:
        """Validate entity data, return list of issues"""
        pass
    
    @abstractmethod
    def to_dict(self) -> Dict[str, Any]:
        """Serialize to dictionary for storage/transport"""
        pass
    
    @classmethod
    @abstractmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'EntityProtocol':
        """Deserialize from dictionary"""
        pass


class TheoryProtocol(ABC):
    """
    Protocol for theory management - the key innovation of ResearchProcess-GPS
    """
    
    @abstractmethod
    def get_id(self) -> UUID:
        """Theory identifier"""
        pass
    
    @abstractmethod
    def get_hypothesis(self) -> str:
        """What this theory proposes"""
        pass
    
    @abstractmethod
    def get_parent_theory(self) -> Optional[UUID]:
        """Theory this was branched from"""
        pass
    
    @abstractmethod
    def get_entities(self) -> Set[UUID]:
        """All entities in this theory"""
        pass
    
    @abstractmethod
    def add_entity(self, entity: EntityProtocol) -> None:
        """Add or update entity in this theory"""
        pass
    
    @abstractmethod
    def remove_entity(self, entity_id: UUID) -> None:
        """Remove entity from this theory"""
        pass
    
    @abstractmethod
    def branch(self, new_hypothesis: str) -> 'TheoryProtocol':
        """Create a new theory branch"""
        pass
    
    @abstractmethod
    def merge(self, other: 'TheoryProtocol') -> 'TheoryProtocol':
        """Merge another theory into this one"""
        pass
    
    @abstractmethod
    def compare_with(self, other: 'TheoryProtocol') -> Dict[str, Any]:
        """Compare this theory with another"""
        pass
    
    @abstractmethod
    def calculate_confidence(self) -> ConfidenceContainer:
        """Calculate overall theory confidence"""
        pass
    
    @abstractmethod
    def validate_gps_compliance(self) -> Dict[str, bool]:
        """Check GPS compliance elements"""
        pass


class EvidenceProtocol(ABC):
    """
    Protocol for evidence - can float between theories
    """
    
    @abstractmethod
    def supports_theory(self, theory: TheoryProtocol) -> float:
        """How strongly this evidence supports a theory (0-1)"""
        pass
    
    @abstractmethod
    def contradicts_theory(self, theory: TheoryProtocol) -> float:
        """How strongly this evidence contradicts a theory (0-1)"""
        pass
    
    @abstractmethod
    def get_extracted_facts(self) -> List[Dict[str, Any]]:
        """Facts extracted from this evidence"""
        pass
    
    @abstractmethod
    def applies_to_entities(self) -> Set[UUID]:
        """Which entities this evidence relates to"""
        pass
    
    @abstractmethod
    def get_source_classification(self) -> Dict[str, str]:
        """Source type, info type, evidence type per GPS"""
        pass