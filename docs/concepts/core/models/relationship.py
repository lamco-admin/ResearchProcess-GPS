"""
Relationship model for ResearchProcess-GPS.

Relationships are flexible, multi-party connections between identities
that can represent any type of human relationship, not just nuclear families.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime, date
from uuid import UUID, uuid4
from enum import Enum

from .base import NestableBaseEntity
from .confidence import ConfidenceContainer
from ..abstractions.nesting import NestingType


class RelationshipCategory(Enum):
    """High-level relationship categories"""
    BIOLOGICAL = "biological"    # Blood relationships
    LEGAL = "legal"              # Legal relationships (marriage, adoption)
    SOCIAL = "social"            # Social relationships
    PROFESSIONAL = "professional" # Work relationships
    SPIRITUAL = "spiritual"      # Godparents, religious
    ECONOMIC = "economic"        # Business partnerships
    RESIDENTIAL = "residential"  # Household members
    OTHER = "other"


class RelationshipType(Enum):
    """Specific relationship types"""
    # Biological
    PARENT_CHILD = "parent_child"
    SIBLING = "sibling"
    GRANDPARENT = "grandparent"
    COUSIN = "cousin"
    ANCESTOR = "ancestor"
    DESCENDANT = "descendant"
    
    # Legal
    MARRIAGE = "marriage"
    CIVIL_UNION = "civil_union"
    COMMON_LAW = "common_law"
    DIVORCE = "divorce"
    ADOPTION = "adoption"
    GUARDIANSHIP = "guardianship"
    WARD = "ward"
    
    # Social
    GODPARENT = "godparent"
    GODCHILD = "godchild"
    FRIEND = "friend"
    NEIGHBOR = "neighbor"
    HOUSEHOLD = "household"
    
    # Professional
    EMPLOYER_EMPLOYEE = "employer_employee"
    MASTER_APPRENTICE = "master_apprentice"
    BUSINESS_PARTNER = "business_partner"
    
    # Historical
    ENSLAVER_ENSLAVED = "enslaver_enslaved"
    OWNER_OWNED = "owner_owned"  # For historical accuracy
    FEUDAL = "feudal"
    
    # Other
    UNKNOWN = "unknown"
    CUSTOM = "custom"


class ParticipantRoleType(Enum):
    """Roles in relationships"""
    # Parent-child roles
    PARENT = "parent"
    CHILD = "child"
    FATHER = "father"
    MOTHER = "mother"
    SON = "son"
    DAUGHTER = "daughter"
    
    # Spousal roles
    SPOUSE = "spouse"
    HUSBAND = "husband"
    WIFE = "wife"
    PARTNER = "partner"
    
    # Sibling roles
    SIBLING = "sibling"
    BROTHER = "brother"
    SISTER = "sister"
    
    # Extended family
    GRANDPARENT = "grandparent"
    GRANDCHILD = "grandchild"
    UNCLE = "uncle"
    AUNT = "aunt"
    NEPHEW = "nephew"
    NIECE = "niece"
    COUSIN = "cousin"
    
    # Legal roles
    GUARDIAN = "guardian"
    WARD = "ward"
    ADOPTIVE_PARENT = "adoptive_parent"
    ADOPTED_CHILD = "adopted_child"
    
    # Social roles
    GODPARENT = "godparent"
    GODCHILD = "godchild"
    SPONSOR = "sponsor"
    
    # Professional roles
    EMPLOYER = "employer"
    EMPLOYEE = "employee"
    MASTER = "master"
    APPRENTICE = "apprentice"
    PARTNER = "partner"
    
    # Historical roles
    ENSLAVER = "enslaver"
    ENSLAVED = "enslaved"
    OWNER = "owner"
    OWNED = "owned"
    
    # Generic
    PARTICIPANT = "participant"
    OTHER = "other"


@dataclass
class RelationshipParticipant:
    """A participant in a relationship with their role"""
    participant_id: UUID = field(default_factory=uuid4)
    
    # Identity in the relationship
    identity_id: UUID = field(default_factory=uuid4)
    
    # Their role
    role: ParticipantRoleType = ParticipantRoleType.PARTICIPANT
    role_description: str = ""  # Additional details
    
    # When they participated
    start_date: Optional[date] = None
    end_date: Optional[date] = None
    date_precision: str = "exact"  # "exact", "approximate", "unknown"
    
    # Why it ended (if applicable)
    end_reason: Optional[str] = None  # "death", "divorce", "moved", etc.
    
    # Theory-specific
    applicable_theories: Set[UUID] = field(default_factory=set)
    
    # Confidence
    confidence: Optional[ConfidenceContainer] = None
    
    def is_active_on_date(self, check_date: date) -> bool:
        """Check if participant was active in relationship on given date"""
        if self.start_date and check_date < self.start_date:
            return False
        if self.end_date and check_date > self.end_date:
            return False
        return True


class RelationshipStatus(Enum):
    """Status of a relationship - theoretical vs concluded"""
    THEORETICAL = "theoretical"      # Research phase, uncertain
    PROBABLE = "probable"            # High confidence but not concluded
    CONCLUDED = "concluded"          # GPS-compliant, ready for persons
    DISPROVEN = "disproven"          # Researched and found false


@dataclass
class Relationship(NestableBaseEntity['Relationship']):
    """
    A Relationship in ResearchProcess-GPS represents connections between identities.
    
    IMPORTANT: Relationships are for actual human connections (biological, legal, social).
    They are NOT for organizational nesting (use collections for that).
    
    Can be theoretical (during research) or concluded (for persons).
    
    Examples:
    - Marriage: 2+ participants as spouses
    - Parent-child: 1+ parents, 1+ children  
    - Household: All members with various roles
    - Business: Multiple partners
    - Blended family: Complex step-relationships
    
    Note: Relationships CAN nest to represent complex social structures,
    but this is different from using nesting to imply the relationship itself.
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Relationship"
        # Relationships can nest for complex social structures
        self.nesting_type = NestingType.LOGICAL
    
    # Relationship classification
    category: RelationshipCategory = RelationshipCategory.OTHER
    relationship_type: RelationshipType = RelationshipType.UNKNOWN
    custom_type: str = ""  # For non-standard relationships
    
    # Research status
    status: RelationshipStatus = RelationshipStatus.THEORETICAL
    
    # Relationship subtype for nuanced distinctions
    subtype: str = ""  # e.g., "biological_confirmed", "adoptive_legal", "foster_temporary"
    
    # Description
    title: str = ""  # "Smith-Jones Marriage", "Johnson Household 1850"
    description: str = ""
    
    # Cultural and legal context
    cultural_context: str = ""  # Important for interpreting relationships
    legal_framework: str = ""   # Jurisdiction and time period
    social_context: str = ""    # Social norms that apply
    
    # Participants
    participants: List[RelationshipParticipant] = field(default_factory=list)
    
    # Relationship timeline
    start_date: Optional[date] = None
    end_date: Optional[date] = None
    date_notes: str = ""
    
    # Events that define/modify the relationship
    defining_events: List[UUID] = field(default_factory=list)  # Marriage, adoption decree
    
    # Evidence
    evidence_links: List[UUID] = field(default_factory=list)
    
    # Related relationships
    supersedes: Optional[UUID] = None  # Previous relationship this replaces
    superseded_by: Optional[UUID] = None  # Relationship that replaces this
    related_relationships: List[UUID] = field(default_factory=list)
    
    # Theory-specific interpretations
    theory_interpretations: Dict[UUID, Dict[str, Any]] = field(default_factory=dict)
    
    # Research notes
    research_notes: List[str] = field(default_factory=list)
    
    def add_participant(self, identity_id: UUID, role: ParticipantRoleType,
                       start_date: Optional[date] = None,
                       theories: Set[UUID] = None) -> RelationshipParticipant:
        """Add a participant to this relationship"""
        participant = RelationshipParticipant(
            identity_id=identity_id,
            role=role,
            start_date=start_date or self.start_date,
            applicable_theories=theories or set()
        )
        self.participants.append(participant)
        return participant
    
    def get_participants_by_role(self, role: ParticipantRoleType) -> List[RelationshipParticipant]:
        """Get all participants with a specific role"""
        return [p for p in self.participants if p.role == role]
    
    def get_participant_role(self, identity_id: UUID, theory_id: Optional[UUID] = None) -> Optional[ParticipantRoleType]:
        """Get the role of a specific identity in this relationship"""
        for participant in self.participants:
            if participant.identity_id == identity_id:
                if theory_id is None or theory_id in participant.applicable_theories:
                    return participant.role
        return None
    
    def get_counterparts(self, identity_id: UUID, theory_id: Optional[UUID] = None) -> List[Tuple[UUID, ParticipantRoleType]]:
        """Get other participants in relationship with their roles"""
        counterparts = []
        for participant in self.participants:
            if participant.identity_id != identity_id:
                if theory_id is None or theory_id in participant.applicable_theories:
                    counterparts.append((participant.identity_id, participant.role))
        return counterparts
    
    def is_active_on_date(self, check_date: date) -> bool:
        """Check if relationship was active on given date"""
        if self.start_date and check_date < self.start_date:
            return False
        if self.end_date and check_date > self.end_date:
            return False
        return True
    
    def validate_consistency(self) -> List[str]:
        """Validate internal consistency of the relationship"""
        issues = []
        
        # Check for appropriate roles
        if self.relationship_type == RelationshipType.MARRIAGE:
            spouses = self.get_participants_by_role(ParticipantRoleType.SPOUSE)
            if len(spouses) < 2:
                issues.append("Marriage requires at least 2 spouses")
        
        elif self.relationship_type == RelationshipType.PARENT_CHILD:
            parents = (self.get_participants_by_role(ParticipantRoleType.PARENT) +
                      self.get_participants_by_role(ParticipantRoleType.FATHER) +
                      self.get_participants_by_role(ParticipantRoleType.MOTHER))
            children = (self.get_participants_by_role(ParticipantRoleType.CHILD) +
                       self.get_participants_by_role(ParticipantRoleType.SON) +
                       self.get_participants_by_role(ParticipantRoleType.DAUGHTER))
            
            if not parents:
                issues.append("Parent-child relationship needs at least one parent")
            if not children:
                issues.append("Parent-child relationship needs at least one child")
        
        # Check date consistency
        for participant in self.participants:
            if participant.start_date and self.start_date:
                if participant.start_date < self.start_date:
                    issues.append(f"Participant {participant.identity_id} starts before relationship")
            
            if participant.end_date and self.end_date:
                if participant.end_date > self.end_date:
                    issues.append(f"Participant {participant.identity_id} ends after relationship")
        
        return issues
    
    def calculate_kinship_chain(self, from_id: UUID, to_id: UUID) -> Optional[List[Tuple[UUID, str]]]:
        """
        Calculate kinship chain between two identities through this relationship.
        Returns list of (identity_id, relationship_step) or None if not connected.
        """
        # Simple implementation for parent-child
        if self.relationship_type == RelationshipType.PARENT_CHILD:
            from_role = self.get_participant_role(from_id)
            to_role = self.get_participant_role(to_id)
            
            if from_role in [ParticipantRoleType.PARENT, ParticipantRoleType.FATHER, ParticipantRoleType.MOTHER]:
                if to_role in [ParticipantRoleType.CHILD, ParticipantRoleType.SON, ParticipantRoleType.DAUGHTER]:
                    return [(from_id, "parent"), (to_id, "child")]
            
            elif from_role in [ParticipantRoleType.CHILD, ParticipantRoleType.SON, ParticipantRoleType.DAUGHTER]:
                if to_role in [ParticipantRoleType.PARENT, ParticipantRoleType.FATHER, ParticipantRoleType.MOTHER]:
                    return [(from_id, "child"), (to_id, "parent")]
        
        # TODO: Implement for other relationship types
        
        return None


@dataclass
class RelationshipNetwork:
    """
    A network of relationships forming a family or social group.
    Useful for analyzing complex family structures.
    """
    network_id: UUID = field(default_factory=uuid4)
    
    # Network properties
    name: str = ""  # "Smith Extended Family", "1850 Household"
    network_type: str = ""  # "family", "household", "clan", "community"
    
    # Relationships in the network
    relationships: Set[UUID] = field(default_factory=set)
    
    # All identities involved
    identities: Set[UUID] = field(default_factory=set)
    
    # Central figures (if any)
    central_identities: Set[UUID] = field(default_factory=set)
    
    # Time period
    active_from: Optional[date] = None
    active_to: Optional[date] = None
    
    # Geographic scope
    primary_location: Optional[UUID] = None
    
    # Theory-specific
    applicable_theories: Set[UUID] = field(default_factory=set)
    
    def add_relationship(self, relationship_id: UUID, participant_ids: List[UUID]) -> None:
        """Add a relationship to the network"""
        self.relationships.add(relationship_id)
        self.identities.update(participant_ids)
    
    def find_path(self, from_id: UUID, to_id: UUID) -> Optional[List[UUID]]:
        """
        Find relationship path between two identities.
        Returns list of relationship IDs forming the path.
        """
        # TODO: Implement graph traversal
        return None
    
    def get_relationship_degree(self, identity1: UUID, identity2: UUID) -> Optional[int]:
        """
        Calculate degree of relationship between two identities.
        1 = parent/child/sibling, 2 = grandparent/grandchild, etc.
        """
        path = self.find_path(identity1, identity2)
        return len(path) if path else None