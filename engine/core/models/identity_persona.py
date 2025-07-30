"""
Identity and Persona model for ResearchProcess-GPS.

An Identity is a reference to a person found in evidence. Each piece of evidence
may refer to people differently - "Bob", "Robert Jones", "Grandma's neighbor".
These are all Identities that may or may not refer to the same Person.

Identities can be:
- Nested together when believed to be the same person
- Promoted to Person status when research concludes
- Demoted back to Identity if new evidence challenges conclusions
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime, date
from uuid import UUID, uuid4
from enum import Enum

from .base import NestableBaseEntity
from .confidence import ConfidenceContainer, ConfidenceLevel
from ..abstractions.nesting import NestingType


class IdentityType(Enum):
    """Types of identity references"""
    NAMED = "named"                    # Has a name in evidence
    DESCRIBED = "described"            # Described but not named ("the widow")
    RELATIONSHIP = "relationship"      # Identified by relationship ("John's wife")
    OCCUPATIONAL = "occupational"      # Identified by occupation ("the blacksmith")
    PLACEHOLDER = "placeholder"        # Created by researcher
    INFERRED = "inferred"             # Inferred from evidence


class PersonaStatus(Enum):
    """Status of an identity/persona"""
    EVIDENCE_BASED = "evidence_based"  # Direct from evidence
    RESEARCHER_CREATED = "researcher_created"  # Created for organization
    MERGED = "merged"                  # Merged into another identity
    PROMOTED = "promoted"              # Promoted to Person
    ACTIVE = "active"                  # Currently being researched


@dataclass
class EvidenceReference:
    """How this identity appears in a specific piece of evidence"""
    evidence_id: UUID                  # The evidence containing reference
    reference_text: str                # Exact text in evidence ("Bob", "Robert Jones")
    reference_type: str                # How referenced ("name", "description", "relationship")
    context: str                       # Surrounding context
    confidence: ConfidenceLevel        # Confidence this refers to this identity
    
    # Location in evidence
    page: Optional[int] = None
    line: Optional[int] = None
    image_region: Optional[Dict[str, int]] = None  # For image evidence
    
    # Extracted attributes from this evidence
    attributes: Dict[str, Any] = field(default_factory=dict)
    # Examples: {"occupation": "farmer", "age": "about 40", "residence": "next door"}


@dataclass 
class IdentityPersona(NestableBaseEntity['IdentityPersona']):
    """
    An Identity/Persona is a reference to a person in evidence.
    
    Key concepts:
    - Each evidence reference creates an identity
    - Identities can be nested when believed to be same person
    - Can be promoted to Person when concluded
    - Can be demoted from Person if questioned
    
    Examples:
    - "Bob" in 1922 letter
    - "Robert Jones" in 1920 census
    - "Grandma's neighbor" in diary entry
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "IdentityPersona"
        self.nesting_type = NestingType.LOGICAL  # Logical grouping of references
    
    # Basic identity info
    identity_type: IdentityType = IdentityType.NAMED
    status: PersonaStatus = PersonaStatus.EVIDENCE_BASED
    
    # Primary identifier (how researcher refers to this identity)
    primary_name: str = ""  # e.g., "Bob (Grandma's neighbor)"
    
    # All evidence references to this identity
    evidence_references: List[EvidenceReference] = field(default_factory=list)
    
    # Aggregated attributes from all evidence
    aggregated_attributes: Dict[str, List[Tuple[Any, UUID]]] = field(default_factory=dict)
    # {"occupation": [("farmer", evidence_id1), ("blacksmith", evidence_id2)]}
    
    # Research notes
    researcher_notes: str = ""
    
    # If this was promoted to Person
    person_id: Optional[UUID] = None
    promotion_date: Optional[datetime] = None
    promotion_rationale: str = ""
    
    # If this was demoted from Person
    demoted_from_person_id: Optional[UUID] = None
    demotion_date: Optional[datetime] = None
    demotion_rationale: str = ""
    
    # Confidence this is a real person
    existence_confidence: Optional[ConfidenceContainer] = None
    
    def add_evidence_reference(self, evidence_id: UUID, reference_text: str,
                              reference_type: str = "name", context: str = "",
                              attributes: Dict[str, Any] = None) -> EvidenceReference:
        """Add a new evidence reference to this identity"""
        ref = EvidenceReference(
            evidence_id=evidence_id,
            reference_text=reference_text,
            reference_type=reference_type,
            context=context,
            confidence=ConfidenceLevel.MEDIUM,
            attributes=attributes or {}
        )
        self.evidence_references.append(ref)
        
        # Update aggregated attributes
        if attributes:
            for key, value in attributes.items():
                if key not in self.aggregated_attributes:
                    self.aggregated_attributes[key] = []
                self.aggregated_attributes[key].append((value, evidence_id))
        
        return ref
    
    def get_all_names(self) -> List[str]:
        """Get all name variations from evidence"""
        names = set()
        for ref in self.evidence_references:
            if ref.reference_type == "name":
                names.add(ref.reference_text)
        return list(names)
    
    def get_consistent_attributes(self) -> Dict[str, Any]:
        """Get attributes that are consistent across evidence"""
        consistent = {}
        for attr, values in self.aggregated_attributes.items():
            unique_values = set(v[0] for v in values)
            if len(unique_values) == 1:
                consistent[attr] = unique_values.pop()
        return consistent
    
    def get_conflicting_attributes(self) -> Dict[str, List[Any]]:
        """Get attributes that conflict across evidence"""
        conflicts = {}
        for attr, values in self.aggregated_attributes.items():
            unique_values = set(v[0] for v in values)
            if len(unique_values) > 1:
                conflicts[attr] = list(unique_values)
        return conflicts
    
    def merge_with(self, other: 'IdentityPersona') -> 'IdentityPersona':
        """Merge another identity into this one"""
        # Add all evidence references
        self.evidence_references.extend(other.evidence_references)
        
        # Merge aggregated attributes
        for attr, values in other.aggregated_attributes.items():
            if attr not in self.aggregated_attributes:
                self.aggregated_attributes[attr] = []
            self.aggregated_attributes[attr].extend(values)
        
        # Update other's status
        other.status = PersonaStatus.MERGED
        
        # Add other as nested child for provenance
        self.add_child(other)
        
        return self
    
    def promote_to_person(self) -> UUID:
        """Promote this identity to Person status"""
        if self.person_id:
            raise ValueError("Already promoted to Person")
        
        # In real implementation, would create Person entity
        self.person_id = uuid4()
        self.promotion_date = datetime.utcnow()
        self.status = PersonaStatus.PROMOTED
        
        return self.person_id
    
    def demote_from_person(self, person_id: UUID, reason: str) -> None:
        """Demote from Person back to Identity"""
        self.demoted_from_person_id = person_id
        self.demotion_date = datetime.utcnow()
        self.demotion_rationale = reason
        self.person_id = None
        self.status = PersonaStatus.ACTIVE
    
    def can_contain(self, child: Any) -> bool:
        """
        Identities can contain other identities when they might be same person.
        Very permissive to allow research flexibility.
        
        IMPORTANT: Nesting can be multi-level and recursive:
        - An identity can contain multiple other identities
        - Those identities can contain more identities
        - No depth limit - organize as complex as needed
        
        Example:
        "Robert Jones" contains:
          - "Bob" (which contains "Bobby" and "Rob")
          - "R. Jones" (which contains "R.J." and "Jones, R.")
          - "Mr. Jones"
        """
        if not isinstance(child, IdentityPersona):
            return True  # Can contain other types for research
        
        # Don't nest if already promoted to different persons
        if self.person_id and child.person_id and self.person_id != child.person_id:
            return False
        
        return super().can_contain(child)
    
    def get_all_nested_identities(self) -> List['IdentityPersona']:
        """Get all nested identities at any depth"""
        all_identities = []
        for child in self.get_children(recursive=True):
            if isinstance(child, IdentityPersona):
                all_identities.append(child)
        return all_identities
    
    def get_all_evidence_references(self) -> List[EvidenceReference]:
        """Get all evidence references from this identity and all nested ones"""
        all_refs = list(self.evidence_references)
        
        for nested in self.get_all_nested_identities():
            all_refs.extend(nested.evidence_references)
        
        return all_refs
    
    def consolidate_names(self) -> Set[str]:
        """Get all name variations from this identity and nested ones"""
        names = set(self.get_all_names())
        
        for nested in self.get_all_nested_identities():
            names.update(nested.get_all_names())
        
        return names
    
    def add_nested_identity(self, identity: 'IdentityPersona', 
                           confidence: float = 0.5,
                           rationale: str = "") -> bool:
        """
        Add a nested identity with confidence it's the same person.
        
        This creates a hypothesis that the nested identity is the same
        as this one, allowing complex identity hierarchies.
        """
        if self.add_child(identity):
            self.nesting_metadata[identity.id] = {
                "confidence": confidence,
                "rationale": rationale,
                "added_date": datetime.utcnow(),
                "relationship": "possible_same_person"
            }
            return True
        return False


@dataclass
class Person(NestableBaseEntity['Person']):
    """
    A Person is a concluded, validated individual.
    Created from one or more IdentityPersona when research concludes they're the same person.
    
    Key differences from IdentityPersona:
    - Represents concluded research
    - Has validated attributes
    - Participates in Relationships (not just theoretical ones)
    - Maps to traditional genealogy
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Person"
        # Persons can nest for family groups but NOT to imply relationships
        self.nesting_type = NestingType.CATEGORICAL
    
    # Concluded identity
    primary_name: str = ""
    all_names: List[str] = field(default_factory=list)
    
    # Source identities that were merged to create this Person
    source_identities: List[UUID] = field(default_factory=list)
    
    # Concluded attributes
    birth_date: Optional[date] = None
    birth_place: Optional[UUID] = None  # Location ID
    death_date: Optional[date] = None
    death_place: Optional[UUID] = None
    
    # Validated attributes from research
    validated_attributes: Dict[str, Any] = field(default_factory=dict)
    
    # GPS compliance
    gps_compliant: bool = False
    validation_notes: str = ""
    
    # Relationships (actual genealogical connections)
    relationships: List[UUID] = field(default_factory=list)
    
    # Research conclusion
    conclusion_date: datetime = field(default_factory=datetime.utcnow)
    conclusion_rationale: str = ""
    
    @classmethod
    def from_identity(cls, identity: IdentityPersona, rationale: str = "") -> 'Person':
        """Create a Person from a single Identity"""
        person = cls()
        person.primary_name = identity.primary_name
        person.all_names = identity.get_all_names()
        person.source_identities = [identity.id]
        person.validated_attributes = identity.get_consistent_attributes()
        person.conclusion_rationale = rationale
        
        # Link back
        identity.promote_to_person()
        
        return person
    
    @classmethod
    def from_identities(cls, identities: List[IdentityPersona], 
                       primary: IdentityPersona, rationale: str = "") -> 'Person':
        """Create a Person from multiple Identities"""
        person = cls()
        person.primary_name = primary.primary_name
        
        # Collect all names
        all_names = set()
        for identity in identities:
            all_names.update(identity.get_all_names())
        person.all_names = list(all_names)
        
        # Source identities
        person.source_identities = [i.id for i in identities]
        
        # Find consistent attributes across all
        # TODO: Implement attribute reconciliation
        
        person.conclusion_rationale = rationale
        
        # Link back to identities
        for identity in identities:
            identity.promote_to_person()
        
        return person
    
    def demote_to_identities(self, reason: str) -> List[IdentityPersona]:
        """Demote this Person back to constituent Identities"""
        # In real implementation, would restore original identities
        # and update their status
        return []


@dataclass
class IdentityGroup(NestableBaseEntity['IdentityGroup']):
    """
    A group of IdentityPersona that might be the same person.
    Used during research to track possibilities.
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "IdentityGroup"
        self.nesting_type = NestingType.RESEARCH
    
    group_name: str = ""  # e.g., "Possible Bob/Robert Jones matches"
    research_question_id: Optional[UUID] = None
    
    # Why these might be same person
    grouping_rationale: str = ""
    
    # Confidence they're same person
    same_person_confidence: ConfidenceLevel = ConfidenceLevel.POSSIBLE
    
    def add_identity(self, identity: IdentityPersona, rationale: str = "") -> bool:
        """Add an identity to this group"""
        if self.add_child(identity):
            self.nesting_metadata[identity.id] = {
                "rationale": rationale,
                "added_date": datetime.utcnow()
            }
            return True
        return False
    
    def promote_to_person(self, primary_identity: IdentityPersona) -> Person:
        """Promote this group to a single Person"""
        identities = [child for child in self.children 
                     if isinstance(child, IdentityPersona)]
        return Person.from_identities(identities, primary_identity, 
                                    self.grouping_rationale)