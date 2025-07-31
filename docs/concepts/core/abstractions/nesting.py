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
    RESEARCH = "research"              # Research-based organization
    THEMATIC = "thematic"              # Theme-based grouping
    WORKFLOW = "workflow"              # Process/workflow grouping
    ARBITRARY = "arbitrary"            # User-defined grouping
    MIXED = "mixed"                    # Mixed-type collection


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
        
    def can_contain(self, child: T) -> bool:
        """
        Check if this entity can contain the given child.
        By default, allows any nesting - override for specific restrictions.
        """
        # Permissive by default - only check for circular references
        if hasattr(child, 'is_ancestor_of') and child.is_ancestor_of(self):
            return False
        return True
    
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
    """
    Constraints on nesting behavior.
    By default, maximally permissive - override as needed.
    """
    max_depth: Optional[int] = None              # Maximum nesting depth (None = unlimited)
    max_children: Optional[int] = None           # Maximum children per node (None = unlimited)
    allowed_child_types: Set[str] = field(default_factory=set)  # Empty = allow all
    forbidden_child_types: Set[str] = field(default_factory=set)
    circular_reference_allowed: bool = False     # Safety default
    multiple_parents_allowed: bool = False       # For DAG structures
    allow_mixed_types: bool = True              # Can contain different entity types
    allow_external_references: bool = True       # Can reference entities outside hierarchy


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
    Used for grouping related entities in any way that makes sense to the researcher.
    
    Collections are maximally flexible:
    - Can contain any entity type
    - Can mix entity types
    - Can have arbitrary metadata
    - Can represent any organizational structure
    """
    
    def __init__(self, name: str, collection_type: str = "general", 
                 description: str = "", purpose: str = ""):
        super().__init__()
        self.name = name
        self.collection_type = collection_type
        self.description = description
        self.purpose = purpose
        self.nesting_type = NestingType.ARBITRARY
        self.metadata: Dict[str, Any] = {}
        self.item_metadata: Dict[UUID, Dict[str, Any]] = {}  # Per-item metadata
        self.tags: Set[str] = set()  # Flexible tagging
        self.created_date = datetime.utcnow()
        
        # Maximally permissive constraints
        self.constraints = NestingConstraints(
            allow_mixed_types=True,
            allow_external_references=True
        )
        
    def add_item(self, item: Any, metadata: Dict[str, Any] = None, 
                 tags: Set[str] = None, notes: str = "") -> bool:
        """
        Add any item to collection with optional metadata.
        Accepts ANY object - maximally permissive.
        """
        if self.add_child(item):
            item_data = {
                "added_date": datetime.utcnow(),
                "notes": notes,
                "entity_type": type(item).__name__
            }
            if metadata:
                item_data.update(metadata)
            self.item_metadata[item.id] = item_data
            
            if tags:
                self.tags.update(tags)
                item_data["tags"] = list(tags)
            
            return True
        return False
    
    def add_items(self, items: List[Any], common_metadata: Dict[str, Any] = None) -> int:
        """Add multiple items at once"""
        added = 0
        for item in items:
            if self.add_item(item, metadata=common_metadata):
                added += 1
        return added
    
    def get_items_by_metadata(self, key: str, value: Any) -> List[Any]:
        """Get items that have specific metadata"""
        results = []
        for child in self.children:
            child_metadata = self.item_metadata.get(child.id, {})
            if child_metadata.get(key) == value:
                results.append(child)
        return results
    
    def get_items_by_type(self, entity_type: Type) -> List[Any]:
        """Get all items of a specific type"""
        return [child for child in self.children if isinstance(child, entity_type)]
    
    def get_items_by_tag(self, tag: str) -> List[Any]:
        """Get items that have a specific tag"""
        results = []
        for child in self.children:
            child_tags = self.item_metadata.get(child.id, {}).get("tags", [])
            if tag in child_tags:
                results.append(child)
        return results
    
    def organize_by_metadata(self, key: str) -> Dict[Any, List[Any]]:
        """Organize collection items by metadata key"""
        organized = {}
        for child in self.children:
            child_metadata = self.item_metadata.get(child.id, {})
            value = child_metadata.get(key, "Unknown")
            if value not in organized:
                organized[value] = []
            organized[value].append(child)
        return organized
    
    def organize_by_type(self) -> Dict[str, List[Any]]:
        """Organize collection by entity type"""
        organized = {}
        for child in self.children:
            entity_type = type(child).__name__
            if entity_type not in organized:
                organized[entity_type] = []
            organized[entity_type].append(child)
        return organized
    
    def filter_items(self, condition: callable) -> List[Any]:
        """Filter items by arbitrary condition"""
        return [child for child in self.children if condition(child)]
    
    def apply_to_all(self, operation: callable) -> None:
        """Apply an operation to all items in collection"""
        for child in self.children:
            operation(child)
    
    def create_subcollection(self, name: str, condition: callable) -> 'CollectionEntity':
        """Create a subcollection based on a condition"""
        subcollection = CollectionEntity(
            name=name,
            collection_type=f"subset_of_{self.name}",
            description=f"Subset of {self.name}"
        )
        
        for child in self.children:
            if condition(child):
                child_metadata = self.item_metadata.get(child.id, {})
                subcollection.add_item(child, metadata=child_metadata.copy())
        
        return subcollection


@dataclass
class UniversalCollection(CollectionEntity):
    """
    The most flexible collection type - can contain literally anything.
    No restrictions whatsoever on what can be grouped together.
    """
    
    def __init__(self, name: str, researcher_notes: str = ""):
        super().__init__(
            name=name,
            collection_type="universal",
            description="Unrestricted collection",
            purpose=researcher_notes
        )
        self.nesting_type = NestingType.ARBITRARY
        
    def can_contain(self, child: Any) -> bool:
        """Universal collections can contain anything"""
        return True


@dataclass 
class ResearchCollection(CollectionEntity):
    """Collection specifically for research organization"""
    
    def __init__(self, research_topic: str, research_question_id: Optional[UUID] = None):
        super().__init__(
            name=f"Research: {research_topic}",
            collection_type="research",
            description=f"Research materials for {research_topic}"
        )
        self.research_topic = research_topic
        self.research_question_id = research_question_id
        self.nesting_type = NestingType.RESEARCH


@dataclass
class TemporalCollection(CollectionEntity):
    """Collection organized by time period"""
    
    def __init__(self, time_period: str, start_year: Optional[int] = None, 
                 end_year: Optional[int] = None):
        super().__init__(
            name=f"Period: {time_period}",
            collection_type="temporal",
            description=f"Items from {time_period}"
        )
        self.time_period = time_period
        self.start_year = start_year
        self.end_year = end_year
        self.nesting_type = NestingType.TEMPORAL


@dataclass
class GeographicCollection(CollectionEntity):
    """Collection organized by location/geography"""
    
    def __init__(self, location_name: str, location_id: Optional[UUID] = None):
        super().__init__(
            name=f"Location: {location_name}",
            collection_type="geographic",
            description=f"Items related to {location_name}"
        )
        self.location_name = location_name
        self.location_id = location_id
        self.nesting_type = NestingType.SPATIAL


@dataclass
class WorkflowCollection(CollectionEntity):
    """Collection for organizing workflow/process items"""
    
    def __init__(self, workflow_name: str, stage: str = ""):
        super().__init__(
            name=f"Workflow: {workflow_name}",
            collection_type="workflow",
            description=f"Items in {workflow_name} workflow"
        )
        self.workflow_name = workflow_name
        self.stage = stage
        self.nesting_type = NestingType.WORKFLOW


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