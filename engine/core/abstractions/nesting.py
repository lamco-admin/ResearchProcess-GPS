"""
Nesting abstraction for ResearchProcess-GPS.

Most genealogical entities can contain other entities of the same type,
creating hierarchical structures. This module provides the base functionality
for nestable entities.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Generic, TypeVar, Type
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum


T = TypeVar('T', bound='NestableEntity')


class NestingType(Enum):
    """Types of nesting relationships"""
    HIERARCHICAL = "hierarchical"      # Strict parent-child hierarchy
    COMPOSITIONAL = "compositional"    # Part of a whole
    CATEGORICAL = "categorical"        # Categorical grouping
    TEMPORAL = "temporal"              # Time-based nesting
    SPATIAL = "spatial"                # Space-based nesting
    LOGICAL = "logical"                # Logical grouping
    ADMINISTRATIVE = "administrative"  # Administrative hierarchy


class NestableEntity(ABC, Generic[T]):
    """
    Base class for entities that can nest within each other.
    Provides common functionality for hierarchical structures.
    """
    
    def __init__(self):
        self.id: UUID = uuid4()
        self.parent_id: Optional[UUID] = None
        self.children: List[T] = []
        self.nesting_type: NestingType = NestingType.HIERARCHICAL
        self.nesting_metadata: Dict[str, Any] = {}
        
    @abstractmethod
    def can_contain(self, child: T) -> bool:
        """Check if this entity can contain the given child"""
        pass
    
    @abstractmethod
    def validate_nesting(self) -> List[str]:
        """Validate the nesting structure, return issues"""
        pass
    
    def add_child(self, child: T) -> bool:
        """Add a child entity"""
        if not self.can_contain(child):
            return False
            
        # Remove from previous parent if exists
        if child.parent_id and child.parent_id != self.id:
            # Would need to look up previous parent and remove
            pass
            
        child.parent_id = self.id
        if child not in self.children:
            self.children.append(child)
        return True
    
    def remove_child(self, child: T) -> bool:
        """Remove a child entity"""
        if child in self.children:
            self.children.remove(child)
            child.parent_id = None
            return True
        return False
    
    def get_children(self, recursive: bool = False) -> List[T]:
        """Get children, optionally recursive"""
        if not recursive:
            return self.children.copy()
            
        result = []
        for child in self.children:
            result.append(child)
            if isinstance(child, NestableEntity):
                result.extend(child.get_children(recursive=True))
        return result
    
    def get_parent(self) -> Optional[T]:
        """Get parent entity (would need entity registry in real implementation)"""
        # In real implementation, would look up parent by ID
        return None
    
    def get_ancestors(self) -> List[T]:
        """Get all ancestors up to root"""
        ancestors = []
        current = self.get_parent()
        while current:
            ancestors.append(current)
            current = current.get_parent()
        return ancestors
    
    def get_root(self) -> T:
        """Get root of the hierarchy"""
        ancestors = self.get_ancestors()
        return ancestors[-1] if ancestors else self
    
    def get_depth(self) -> int:
        """Get depth in hierarchy (0 for root)"""
        return len(self.get_ancestors())
    
    def get_siblings(self) -> List[T]:
        """Get sibling entities"""
        parent = self.get_parent()
        if parent:
            return [child for child in parent.children if child.id != self.id]
        return []
    
    def is_ancestor_of(self, other: T) -> bool:
        """Check if this entity is an ancestor of another"""
        current = other.get_parent()
        while current:
            if current.id == self.id:
                return True
            current = current.get_parent()
        return False
    
    def is_descendant_of(self, other: T) -> bool:
        """Check if this entity is a descendant of another"""
        return other.is_ancestor_of(self)
    
    def find_descendants(self, condition: callable) -> List[T]:
        """Find all descendants matching a condition"""
        results = []
        for child in self.get_children(recursive=True):
            if condition(child):
                results.append(child)
        return results
    
    def get_path_to_root(self) -> List[UUID]:
        """Get path of IDs from this entity to root"""
        path = [self.id]
        path.extend(ancestor.id for ancestor in self.get_ancestors())
        return path
    
    def move_to_parent(self, new_parent: Optional[T]) -> bool:
        """Move this entity to a new parent"""
        # Check for circular references
        if new_parent and (new_parent.id == self.id or self.is_ancestor_of(new_parent)):
            return False
            
        # Remove from old parent
        old_parent = self.get_parent()
        if old_parent:
            old_parent.remove_child(self)
            
        # Add to new parent
        if new_parent:
            return new_parent.add_child(self)
        else:
            self.parent_id = None
            return True


@dataclass
class NestingConstraints:
    """Constraints on nesting behavior"""
    max_depth: Optional[int] = None              # Maximum nesting depth
    max_children: Optional[int] = None           # Maximum children per node
    allowed_child_types: Set[str] = field(default_factory=set)
    forbidden_child_types: Set[str] = field(default_factory=set)
    circular_reference_allowed: bool = False
    multiple_parents_allowed: bool = False       # For DAG structures


@dataclass
class NestableResearchQuestion:
    """Example: Research Questions that can nest"""
    
    def __init__(self, question: str):
        super().__init__()
        self.question = question
        self.constraints = NestingConstraints(
            max_depth=5,  # Don't go too deep
            allowed_child_types={"ResearchQuestion"}
        )
    
    def can_contain(self, child: 'NestableResearchQuestion') -> bool:
        """Research questions can contain sub-questions"""
        # Check depth
        if self.constraints.max_depth:
            if child.get_depth() + self.get_depth() + 1 > self.constraints.max_depth:
                return False
                
        # Check for circular references
        if not self.constraints.circular_reference_allowed:
            if child.is_ancestor_of(self):
                return False
                
        return True
    
    def validate_nesting(self) -> List[str]:
        """Validate research question nesting"""
        issues = []
        
        # Check depth
        if self.constraints.max_depth and self.get_depth() > self.constraints.max_depth:
            issues.append(f"Exceeds maximum depth of {self.constraints.max_depth}")
            
        # Check children count
        if self.constraints.max_children and len(self.children) > self.constraints.max_children:
            issues.append(f"Too many sub-questions: {len(self.children)}")
            
        return issues


@dataclass
class NestableEvent:
    """Example: Events that can contain sub-events"""
    
    def __init__(self, event_type: str, description: str):
        super().__init__()
        self.event_type = event_type
        self.description = description
        self.nesting_type = NestingType.TEMPORAL
        
    def can_contain(self, child: 'NestableEvent') -> bool:
        """Events can contain sub-events of appropriate types"""
        # War can contain battles
        if self.event_type == "war" and child.event_type in ["battle", "siege", "campaign"]:
            return True
            
        # Migration can contain stops
        if self.event_type == "migration" and child.event_type in ["departure", "arrival", "stop"]:
            return True
            
        # Conference can contain sessions
        if self.event_type == "conference" and child.event_type in ["session", "presentation"]:
            return True
            
        # Generic containment for same type
        if self.event_type == child.event_type:
            return True
            
        return False


@dataclass
class NestableLocation:
    """Example: Locations with spatial hierarchy"""
    
    def __init__(self, name: str, location_type: str):
        super().__init__()
        self.name = name
        self.location_type = location_type
        self.nesting_type = NestingType.SPATIAL
        
    def can_contain(self, child: 'NestableLocation') -> bool:
        """Locations follow spatial hierarchy"""
        hierarchy = {
            "world": ["continent"],
            "continent": ["country"],
            "country": ["state", "province"],
            "state": ["county", "district"],
            "county": ["city", "town"],
            "city": ["neighborhood", "district"],
            "neighborhood": ["street"],
            "street": ["address"],
            "building": ["floor", "unit"]
        }
        
        allowed_children = hierarchy.get(self.location_type, [])
        return child.location_type in allowed_children


@dataclass
class CollectionEntity(NestableEntity[T]):
    """
    Special type of nestable entity that represents a collection.
    Used for grouping related entities.
    """
    
    def __init__(self, name: str, collection_type: str):
        super().__init__()
        self.name = name
        self.collection_type = collection_type
        self.nesting_type = NestingType.CATEGORICAL
        self.metadata: Dict[str, Any] = {}
        
    def add_item(self, item: T, metadata: Dict[str, Any] = None) -> bool:
        """Add item to collection with optional metadata"""
        if self.add_child(item):
            if metadata:
                self.metadata[item.id] = metadata
            return True
        return False
    
    def get_items_by_metadata(self, key: str, value: Any) -> List[T]:
        """Get items that have specific metadata"""
        results = []
        for child in self.children:
            child_metadata = self.metadata.get(child.id, {})
            if child_metadata.get(key) == value:
                results.append(child)
        return results
    
    def organize_by_metadata(self, key: str) -> Dict[Any, List[T]]:
        """Organize collection items by metadata key"""
        organized = {}
        for child in self.children:
            child_metadata = self.metadata.get(child.id, {})
            value = child_metadata.get(key, "Unknown")
            if value not in organized:
                organized[value] = []
            organized[value].append(child)
        return organized


class NestingAnalyzer:
    """Analyzes nesting structures"""
    
    @staticmethod
    def find_common_ancestor(entity1: NestableEntity, entity2: NestableEntity) -> Optional[NestableEntity]:
        """Find lowest common ancestor of two entities"""
        ancestors1 = set(entity1.get_path_to_root())
        ancestors2 = set(entity2.get_path_to_root())
        
        common = ancestors1.intersection(ancestors2)
        if not common:
            return None
            
        # Find the lowest (closest to entities)
        for ancestor_id in entity1.get_path_to_root():
            if ancestor_id in common:
                # Would need to look up entity by ID
                return None
                
        return None
    
    @staticmethod
    def calculate_relationship_path(entity1: NestableEntity, entity2: NestableEntity) -> List[UUID]:
        """Calculate path between two entities through their common ancestor"""
        common_ancestor = NestingAnalyzer.find_common_ancestor(entity1, entity2)
        if not common_ancestor:
            return []
            
        # Path: entity1 → ancestor → entity2
        path1 = entity1.get_path_to_root()
        path2 = entity2.get_path_to_root()
        
        # Find where they meet
        # ... implementation
        
        return []
    
    @staticmethod
    def flatten_hierarchy(root: NestableEntity, include_metadata: bool = True) -> List[Dict[str, Any]]:
        """Flatten a hierarchy into a list with level information"""
        result = []
        
        def process_entity(entity: NestableEntity, level: int):
            item = {
                "id": entity.id,
                "level": level,
                "parent_id": entity.parent_id
            }
            
            if include_metadata:
                item["metadata"] = entity.nesting_metadata
                
            result.append(item)
            
            for child in entity.children:
                process_entity(child, level + 1)
                
        process_entity(root, 0)
        return result