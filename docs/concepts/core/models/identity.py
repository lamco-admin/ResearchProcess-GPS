"""
Identity and Persona models for ResearchProcess-GPS.

==============================================================================
DEPRECATED: This theory-based Identity model is deprecated in favor of the 
evidence-based IdentityPersona model in identity_persona.py.

The new model better reflects real genealogical research where:
- Each evidence reference creates an IdentityPersona
- These can be nested/grouped when possibly the same person
- They can be promoted to Person status when research concludes
- They can be demoted back if new evidence emerges

Please use identity_persona.py for all new development.
==============================================================================

Original theory-based approach kept for historical reference only.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum

from .base import BaseEntity
from .confidence import ConfidenceContainer


class ExistenceStatus(Enum):
    """Status of an identity's existence"""
    HYPOTHETICAL = "hypothetical"  # Theoretical person
    EVIDENCED = "evidenced"        # Some evidence exists
    CONFIRMED = "confirmed"        # Strong evidence
    DISPROVEN = "disproven"       # Proven not to exist
    MERGED = "merged"             # Merged into another identity
    DUPLICATE = "duplicate"       # Duplicate of another identity


class NameType(Enum):
    """Types of names an identity might have"""
    BIRTH = "birth"
    MARRIED = "married"
    PROFESSIONAL = "professional"
    RELIGIOUS = "religious"
    NICKNAME = "nickname"
    ALIAS = "alias"
    UNKNOWN = "unknown"


class BiologicalSex(Enum):
    """Biological sex options"""
    MALE = "male"
    FEMALE = "female"
    INTERSEX = "intersex"
    UNKNOWN = "unknown"


@dataclass
class NameForm:
    """A name form with cultural context and time period"""
    name_id: UUID = field(default_factory=uuid4)
    given_names: List[str] = field(default_factory=list)
    surnames: List[str] = field(default_factory=list)
    titles: List[str] = field(default_factory=list)
    suffixes: List[str] = field(default_factory=list)
    nicknames: List[str] = field(default_factory=list)
    cultural_context: str = ""
    name_type: NameType = NameType.UNKNOWN
    usage_start: Optional[datetime] = None
    usage_end: Optional[datetime] = None
    confidence: Optional[ConfidenceContainer] = None
    source_evidence: List[UUID] = field(default_factory=list)
    
    def full_name(self) -> str:
        """Construct full name from components"""
        parts = []
        if self.titles:
            parts.extend(self.titles)
        if self.given_names:
            parts.extend(self.given_names)
        if self.surnames:
            parts.extend(self.surnames)
        if self.suffixes:
            parts.extend(self.suffixes)
        return " ".join(parts)


@dataclass
class BiologicalProfile:
    """Biological characteristics of an identity"""
    biological_sex: BiologicalSex = BiologicalSex.UNKNOWN
    chromosomal_sex: Optional[str] = None  # From DNA testing
    phenotypical_sex: Optional[str] = None  # As observed/recorded
    
    # Genetic information
    y_dna_haplogroup: Optional[str] = None
    mt_dna_haplogroup: Optional[str] = None
    genetic_markers: Dict[str, Any] = field(default_factory=dict)
    
    # Physical characteristics (if known)
    physical_traits: Dict[str, str] = field(default_factory=dict)
    
    confidence: Optional[ConfidenceContainer] = None
    evidence_references: List[UUID] = field(default_factory=list)


@dataclass
class SocialIdentity:
    """Social aspects of identity - profession, religion, culture, etc."""
    identity_id: UUID = field(default_factory=uuid4)
    identity_type: str = ""  # "legal", "professional", "religious", "cultural"
    attributes: Dict[str, Any] = field(default_factory=dict)
    valid_from: Optional[datetime] = None
    valid_to: Optional[datetime] = None
    geographic_scope: Optional[UUID] = None  # Reference to Location
    confidence: Optional[ConfidenceContainer] = None
    evidence_references: List[UUID] = field(default_factory=list)


@dataclass
class Identity(BaseEntity):
    """
    An Identity represents a potentially real person.
    Unlike traditional genealogy software that uses fixed "Person" objects,
    an Identity can have multiple interpretations (Personas) across different theories.
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Identity"
    
    # Core identity fields
    existence_status: ExistenceStatus = ExistenceStatus.HYPOTHETICAL
    
    # Names - can have multiple across time and culture
    names: List[NameForm] = field(default_factory=list)
    primary_name: Optional[UUID] = None  # Reference to primary name in names list
    
    # Biological information
    biological_profile: Optional[BiologicalProfile] = None
    
    # Social identities
    social_identities: List[SocialIdentity] = field(default_factory=list)
    
    # Life span (may be uncertain)
    birth_event: Optional[UUID] = None  # Reference to birth Event
    death_event: Optional[UUID] = None  # Reference to death Event
    
    # Personas - different interpretations in different theories
    personas: Dict[UUID, 'Persona'] = field(default_factory=dict)  # theory_id -> Persona
    
    # Research tracking
    first_evidenced: Optional[datetime] = None
    last_updated: Optional[datetime] = None
    research_notes: List[str] = field(default_factory=list)
    
    def add_name(self, name_form: NameForm) -> None:
        """Add a name form to this identity"""
        self.names.append(name_form)
        if self.primary_name is None:
            self.primary_name = name_form.name_id
    
    def get_primary_name(self) -> Optional[NameForm]:
        """Get the primary name form"""
        if self.primary_name:
            for name in self.names:
                if name.name_id == self.primary_name:
                    return name
        return self.names[0] if self.names else None
    
    def add_persona(self, theory_id: UUID, persona: 'Persona') -> None:
        """Add a persona for a specific theory"""
        self.personas[theory_id] = persona
        persona.identity_id = self.id
    
    def get_persona_for_theory(self, theory_id: UUID) -> Optional['Persona']:
        """Get the persona for a specific theory"""
        return self.personas.get(theory_id)
    
    def merge_with(self, other: 'Identity') -> 'Identity':
        """Merge another identity into this one"""
        # This is a complex operation that would involve:
        # 1. Merging names (deduplicating)
        # 2. Merging biological profiles
        # 3. Merging social identities
        # 4. Updating all references
        # 5. Creating appropriate audit trail
        # For now, we'll just mark the other as merged
        other.existence_status = ExistenceStatus.MERGED
        # TODO: Implement full merge logic
        return self


@dataclass
class Persona(BaseEntity):
    """
    A Persona is a specific interpretation of an Identity within a Theory.
    This allows the same Identity to exist differently in different theories.
    For example, "John Smith born 1820" vs "John Smith born 1823".
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Persona"
    
    # Link to parent identity
    identity_id: UUID = field(default_factory=uuid4)
    
    # Theory this persona exists in
    theory_id: UUID = field(default_factory=uuid4)
    
    # Persona-specific interpretations
    interpreted_names: List[NameForm] = field(default_factory=list)
    interpreted_birth_date: Optional[datetime] = None
    interpreted_death_date: Optional[datetime] = None
    interpreted_birth_place: Optional[UUID] = None  # Location reference
    interpreted_death_place: Optional[UUID] = None  # Location reference
    
    # Relationships in this theory
    relationship_participations: List[UUID] = field(default_factory=list)
    
    # Events in this theory
    event_participations: List[UUID] = field(default_factory=list)
    
    # Theory-specific notes
    theory_notes: str = ""
    
    # How confident are we in this interpretation?
    interpretation_confidence: Optional[ConfidenceContainer] = None
    
    def differs_from(self, other: 'Persona') -> Dict[str, Any]:
        """Compare with another persona to find differences"""
        differences = {}
        
        # Compare birth dates
        if self.interpreted_birth_date != other.interpreted_birth_date:
            differences['birth_date'] = {
                'self': self.interpreted_birth_date,
                'other': other.interpreted_birth_date
            }
        
        # Compare death dates
        if self.interpreted_death_date != other.interpreted_death_date:
            differences['death_date'] = {
                'self': self.interpreted_death_date,
                'other': other.interpreted_death_date
            }
        
        # Compare names
        self_names = {n.full_name() for n in self.interpreted_names}
        other_names = {n.full_name() for n in other.interpreted_names}
        if self_names != other_names:
            differences['names'] = {
                'self': list(self_names),
                'other': list(other_names)
            }
        
        # TODO: Add more comparisons
        
        return differences