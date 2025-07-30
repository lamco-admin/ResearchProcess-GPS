"""
Base model classes for ResearchProcess-GPS entities.
All domain models inherit from these base classes.
"""

from abc import ABC
from typing import Dict, List, Optional, Any, Set
from datetime import datetime
from uuid import UUID, uuid4
from dataclasses import dataclass, field

from ...protocols.entity_protocol import (
    EntityProtocol, PublicationState, Version, Attribution,
    ConfidenceContainer
)


@dataclass
class BaseEntity(EntityProtocol):
    """
    Base class for all RGPS entities.
    Implements the EntityProtocol with common functionality.
    """
    
    # Core fields
    id: UUID = field(default_factory=uuid4)
    type: str = field(init=False)  # Set by subclasses
    created_at: datetime = field(default_factory=datetime.utcnow)
    updated_at: datetime = field(default_factory=datetime.utcnow)
    
    # Versioning
    version: Version = field(default_factory=lambda: Version(
        version_id=uuid4(),
        parent_version=None,
        created_at=datetime.utcnow(),
        created_by="system",
        change_description="Initial creation",
        theory_context=None
    ))
    
    # Publication and collaboration
    publication_state: PublicationState = PublicationState.DRAFT
    attributions: List[Attribution] = field(default_factory=list)
    
    # Confidence tracking
    confidence: Optional[ConfidenceContainer] = None
    
    # Theory management
    theory_contexts: Set[UUID] = field(default_factory=set)
    
    # Entity-specific data
    data: Dict[str, Any] = field(default_factory=dict)
    
    # References to other entities
    references: Dict[str, List[UUID]] = field(default_factory=dict)
    
    # Privacy and security
    privacy_level: str = "private"
    encryption_key_id: Optional[str] = None
    
    def __post_init__(self):
        """Set the entity type based on class name"""
        if not hasattr(self, '_type_set'):
            self.type = self.__class__.__name__
            self._type_set = True
    
    # EntityProtocol implementation
    
    def get_id(self) -> UUID:
        return self.id
    
    def get_type(self) -> str:
        return self.type
    
    def get_version(self) -> Version:
        return self.version
    
    def get_publication_state(self) -> PublicationState:
        return self.publication_state
    
    def get_attributions(self) -> List[Attribution]:
        return self.attributions
    
    def get_confidence(self) -> ConfidenceContainer:
        if self.confidence is None:
            # Return a default confidence container
            from .confidence import ConfidenceContainer, ConfidenceLevel
            self.confidence = ConfidenceContainer(
                summary_confidence=ConfidenceLevel.SPECULATIVE,
                evidence_quality={},
                research_coverage={},
                gps_compliance={
                    "reasonably_exhaustive": False,
                    "complete_citations": False,
                    "thorough_analysis": False,
                    "conflict_resolution": False,
                    "sound_conclusion": False
                },
                peer_assessments=[]
            )
        return self.confidence
    
    def get_data(self) -> Dict[str, Any]:
        return self.data
    
    def get_references(self) -> Dict[str, List[UUID]]:
        return self.references
    
    def applies_to_theories(self) -> Set[UUID]:
        return self.theory_contexts
    
    def branch_for_theory(self, theory_id: UUID) -> 'BaseEntity':
        """Create a version of this entity for a specific theory"""
        import copy
        branched = copy.deepcopy(self)
        branched.id = uuid4()
        branched.version = Version(
            version_id=uuid4(),
            parent_version=self.version.version_id,
            created_at=datetime.utcnow(),
            created_by="system",
            change_description=f"Branched for theory {theory_id}",
            theory_context=theory_id
        )
        branched.theory_contexts = {theory_id}
        return branched
    
    def merge_from(self, other: 'BaseEntity') -> 'BaseEntity':
        """Merge another version of this entity"""
        if other.type != self.type:
            raise ValueError(f"Cannot merge different entity types: {self.type} and {other.type}")
        
        # This is a simplified merge - real implementation would be more sophisticated
        import copy
        merged = copy.deepcopy(self)
        merged.id = uuid4()
        merged.version = Version(
            version_id=uuid4(),
            parent_version=self.version.version_id,
            created_at=datetime.utcnow(),
            created_by="system",
            change_description=f"Merged from {other.id}",
            theory_context=None
        )
        
        # Merge theory contexts
        merged.theory_contexts = self.theory_contexts.union(other.theory_contexts)
        
        # Merge attributions
        merged.attributions.extend(other.attributions)
        
        # TODO: Implement sophisticated merge strategies for data and references
        
        return merged
    
    def validate(self) -> List[str]:
        """Validate entity data, return list of issues"""
        issues = []
        
        if not self.id:
            issues.append("Entity must have an ID")
        
        if not self.type:
            issues.append("Entity must have a type")
        
        if not self.version:
            issues.append("Entity must have version information")
        
        # Subclasses should override to add specific validation
        
        return issues
    
    def to_dict(self) -> Dict[str, Any]:
        """Serialize to dictionary for storage/transport"""
        return {
            "id": str(self.id),
            "type": self.type,
            "created_at": self.created_at.isoformat(),
            "updated_at": self.updated_at.isoformat(),
            "version": {
                "version_id": str(self.version.version_id),
                "parent_version": str(self.version.parent_version) if self.version.parent_version else None,
                "created_at": self.version.created_at.isoformat(),
                "created_by": self.version.created_by,
                "change_description": self.version.change_description,
                "theory_context": str(self.version.theory_context) if self.version.theory_context else None
            },
            "publication_state": self.publication_state.value,
            "attributions": [
                {
                    "contributor_id": attr.contributor_id,
                    "contribution_type": attr.contribution_type,
                    "timestamp": attr.timestamp.isoformat(),
                    "description": attr.description
                }
                for attr in self.attributions
            ],
            "confidence": self.confidence.to_dict() if self.confidence else None,
            "theory_contexts": [str(tid) for tid in self.theory_contexts],
            "data": self.data,
            "references": {
                ref_type: [str(ref_id) for ref_id in ref_ids]
                for ref_type, ref_ids in self.references.items()
            },
            "privacy_level": self.privacy_level,
            "encryption_key_id": self.encryption_key_id
        }
    
    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'BaseEntity':
        """Deserialize from dictionary"""
        # This is a base implementation - subclasses should override
        instance = cls()
        
        instance.id = UUID(data["id"])
        instance.type = data["type"]
        instance.created_at = datetime.fromisoformat(data["created_at"])
        instance.updated_at = datetime.fromisoformat(data["updated_at"])
        
        # Reconstruct version
        version_data = data["version"]
        instance.version = Version(
            version_id=UUID(version_data["version_id"]),
            parent_version=UUID(version_data["parent_version"]) if version_data["parent_version"] else None,
            created_at=datetime.fromisoformat(version_data["created_at"]),
            created_by=version_data["created_by"],
            change_description=version_data["change_description"],
            theory_context=UUID(version_data["theory_context"]) if version_data["theory_context"] else None
        )
        
        instance.publication_state = PublicationState(data["publication_state"])
        
        # Reconstruct attributions
        instance.attributions = [
            Attribution(
                contributor_id=attr["contributor_id"],
                contribution_type=attr["contribution_type"],
                timestamp=datetime.fromisoformat(attr["timestamp"]),
                description=attr["description"]
            )
            for attr in data.get("attributions", [])
        ]
        
        # TODO: Reconstruct confidence container
        
        instance.theory_contexts = {UUID(tid) for tid in data.get("theory_contexts", [])}
        instance.data = data.get("data", {})
        instance.references = {
            ref_type: [UUID(ref_id) for ref_id in ref_ids]
            for ref_type, ref_ids in data.get("references", {}).items()
        }
        
        instance.privacy_level = data.get("privacy_level", "private")
        instance.encryption_key_id = data.get("encryption_key_id")
        
        return instance