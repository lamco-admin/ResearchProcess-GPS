"""
Event model for ResearchProcess-GPS.

Events are independent entities that float between theories and can have
multiple participants. Unlike traditional genealogy software where events
are "owned" by persons or families.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime, date, timedelta
from uuid import UUID, uuid4
from enum import Enum

from .base import BaseEntity
from .confidence import ConfidenceContainer


class EventCategory(Enum):
    """High-level event categories"""
    VITAL = "vital"              # Birth, death, marriage
    CIVIC = "civic"              # Census, voter registration, naturalization
    RELIGIOUS = "religious"      # Baptism, confirmation, burial
    MILITARY = "military"        # Enlistment, discharge, battle
    EDUCATIONAL = "educational"  # Graduation, enrollment
    OCCUPATIONAL = "occupational" # Employment, retirement, promotion
    RESIDENTIAL = "residential"   # Moving, residence
    LEGAL = "legal"              # Court cases, wills, contracts
    MEDICAL = "medical"          # Illness, treatment, vaccination
    SOCIAL = "social"            # Club membership, social events
    TRAVEL = "travel"            # Immigration, trips
    OTHER = "other"


class EventType(Enum):
    """Specific event types"""
    # Vital events
    BIRTH = "birth"
    DEATH = "death"
    MARRIAGE = "marriage"
    DIVORCE = "divorce"
    
    # Religious events
    BAPTISM = "baptism"
    CHRISTENING = "christening"
    CONFIRMATION = "confirmation"
    BAR_MITZVAH = "bar_mitzvah"
    BURIAL = "burial"
    
    # Civic events
    CENSUS = "census"
    VOTER_REGISTRATION = "voter_registration"
    NATURALIZATION = "naturalization"
    
    # Military events
    ENLISTMENT = "enlistment"
    DISCHARGE = "discharge"
    PROMOTION = "promotion"
    BATTLE = "battle"
    
    # Life events
    GRADUATION = "graduation"
    OCCUPATION = "occupation"
    RESIDENCE = "residence"
    IMMIGRATION = "immigration"
    EMIGRATION = "emigration"
    
    # Legal events
    WILL = "will"
    PROBATE = "probate"
    LAND_PURCHASE = "land_purchase"
    COURT_CASE = "court_case"
    
    # Other
    CUSTOM = "custom"


class ParticipantRole(Enum):
    """Roles that participants can have in events"""
    # Primary roles
    PRINCIPAL = "principal"      # Main person(s) the event is about
    SUBJECT = "subject"          # Subject of the event
    
    # Official roles
    OFFICIANT = "officiant"      # Person performing ceremony
    WITNESS = "witness"          # Official witness
    INFORMANT = "informant"      # Person providing information
    
    # Family roles
    PARENT = "parent"
    CHILD = "child"
    SPOUSE = "spouse"
    
    # Legal roles
    EXECUTOR = "executor"
    BENEFICIARY = "beneficiary"
    GUARDIAN = "guardian"
    
    # Other roles
    ATTENDEE = "attendee"
    SPONSOR = "sponsor"
    EMPLOYER = "employer"
    OWNER = "owner"
    BUYER = "buyer"
    SELLER = "seller"
    
    # Generic
    PARTICIPANT = "participant"
    OTHER = "other"


class DatePrecision(Enum):
    """How precise a date is"""
    EXACT = "exact"              # Know the exact date
    DAY = "day"                  # Know day, month, year
    MONTH = "month"              # Know month and year
    YEAR = "year"                # Know only year
    DECADE = "decade"            # Know approximate decade
    CENTURY = "century"          # Know only century
    UNKNOWN = "unknown"


class CalendarSystem(Enum):
    """Calendar systems for dates"""
    GREGORIAN = "gregorian"
    JULIAN = "julian"
    HEBREW = "hebrew"
    ISLAMIC = "islamic"
    FRENCH_REVOLUTIONARY = "french_revolutionary"
    CHINESE = "chinese"
    OTHER = "other"


@dataclass
class TemporalPoint:
    """A point in time with uncertainty"""
    # Best estimate
    date_value: Optional[date] = None
    time_value: Optional[datetime] = None
    
    # Original text
    date_as_recorded: str = ""
    
    # Precision
    precision: DatePrecision = DatePrecision.UNKNOWN
    
    # Calendar
    calendar_system: CalendarSystem = CalendarSystem.GREGORIAN
    
    # Modifiers
    modifiers: List[str] = field(default_factory=list)  # "about", "before", "after", "between"
    
    # Range (if approximate)
    earliest_possible: Optional[date] = None
    latest_possible: Optional[date] = None
    
    # Confidence
    confidence: float = 0.0  # 0-1
    
    def is_precise(self) -> bool:
        """Check if this is a precise date"""
        return self.precision in [DatePrecision.EXACT, DatePrecision.DAY]
    
    def overlaps_with(self, other: 'TemporalPoint') -> bool:
        """Check if two temporal points could be the same time"""
        if self.date_value and other.date_value:
            # If we have exact dates, simple comparison
            if self.is_precise() and other.is_precise():
                return self.date_value == other.date_value
            
            # Otherwise check ranges
            self_earliest = self.earliest_possible or self.date_value
            self_latest = self.latest_possible or self.date_value
            other_earliest = other.earliest_possible or other.date_value
            other_latest = other.latest_possible or other.date_value
            
            return not (self_latest < other_earliest or other_latest < self_earliest)
        
        return True  # Unknown dates might overlap


@dataclass
class EventParticipation:
    """How an identity participates in an event"""
    participation_id: UUID = field(default_factory=uuid4)
    
    # Who participated
    identity_id: UUID = field(default_factory=uuid4)
    
    # Their role
    role: ParticipantRole = ParticipantRole.PARTICIPANT
    role_description: str = ""  # Additional details
    
    # Age at event (if known)
    age_at_event: Optional[str] = None  # "25", "infant", "about 30"
    calculated_birth_year: Optional[int] = None
    
    # Presence
    presence_type: str = "physical"  # "physical", "legal", "proxy", "mentioned"
    
    # Theory-specific
    applicable_theories: Set[UUID] = field(default_factory=set)
    theory_notes: Dict[UUID, str] = field(default_factory=dict)
    
    # Confidence
    confidence: Optional[ConfidenceContainer] = None


@dataclass
class Event(BaseEntity):
    """
    An Event in ResearchProcess-GPS is independent - not owned by any person or family.
    Multiple people can participate in various roles, and the same event can be
    interpreted differently in different theories.
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Event"
    
    # Event classification
    category: EventCategory = EventCategory.OTHER
    event_type: EventType = EventType.CUSTOM
    custom_type: str = ""  # For non-standard event types
    
    # Description
    title: str = ""
    description: str = ""
    
    # When it happened
    temporal_data: TemporalPoint = field(default_factory=TemporalPoint)
    duration: Optional[timedelta] = None  # For events that span time
    
    # Where it happened  
    location_id: Optional[UUID] = None  # Reference to Location
    location_as_recorded: str = ""
    location_type: str = "occurrence"  # "occurrence", "registration", "reporting"
    
    # Jurisdiction at time
    jurisdiction_id: Optional[UUID] = None
    jurisdiction_as_recorded: str = ""
    
    # Participants
    participations: List[EventParticipation] = field(default_factory=list)
    
    # Cultural context
    cultural_context: str = ""
    language: str = "English"
    
    # Evidence
    evidence_links: List[UUID] = field(default_factory=list)
    primary_evidence: Optional[UUID] = None
    
    # Sequence and relationships to other events
    preceded_by: List[UUID] = field(default_factory=list)  # Events that came before
    followed_by: List[UUID] = field(default_factory=list)  # Events that came after
    caused_by: List[UUID] = field(default_factory=list)    # Causal relationships
    resulted_in: List[UUID] = field(default_factory=list)  # Results of this event
    
    # Theory-specific interpretations
    theory_interpretations: Dict[UUID, Dict[str, Any]] = field(default_factory=dict)
    
    # Research notes
    research_notes: List[str] = field(default_factory=list)
    
    def add_participant(self, identity_id: UUID, role: ParticipantRole,
                       age: Optional[str] = None, theories: Set[UUID] = None) -> EventParticipation:
        """Add a participant to this event"""
        participation = EventParticipation(
            identity_id=identity_id,
            role=role,
            age_at_event=age,
            applicable_theories=theories or set()
        )
        
        # Calculate birth year if age is given
        if age and self.temporal_data.date_value:
            try:
                age_value = int(age)
                participation.calculated_birth_year = self.temporal_data.date_value.year - age_value
            except ValueError:
                pass  # Age might be "infant", "child", etc.
        
        self.participations.append(participation)
        return participation
    
    def get_participants_by_role(self, role: ParticipantRole) -> List[EventParticipation]:
        """Get all participants with a specific role"""
        return [p for p in self.participations if p.role == role]
    
    def get_participant_for_theory(self, identity_id: UUID, theory_id: UUID) -> Optional[EventParticipation]:
        """Get how an identity participates in this event in a specific theory"""
        for participation in self.participations:
            if (participation.identity_id == identity_id and 
                theory_id in participation.applicable_theories):
                return participation
        return None
    
    def interpret_for_theory(self, theory_id: UUID) -> Dict[str, Any]:
        """Get theory-specific interpretation of this event"""
        base_interpretation = {
            'event_type': self.event_type.value,
            'date': self.temporal_data.date_as_recorded,
            'place': self.location_as_recorded,
            'participants': []
        }
        
        # Add theory-specific participants
        for participation in self.participations:
            if theory_id in participation.applicable_theories:
                base_interpretation['participants'].append({
                    'identity_id': str(participation.identity_id),
                    'role': participation.role.value,
                    'age': participation.age_at_event
                })
        
        # Merge with any stored theory-specific interpretation
        if theory_id in self.theory_interpretations:
            base_interpretation.update(self.theory_interpretations[theory_id])
        
        return base_interpretation
    
    def conflicts_with(self, other: 'Event') -> Optional[str]:
        """Check if this event conflicts with another"""
        # Same type events at different times for same people
        if self.event_type == other.event_type:
            # Check if they have common participants
            self_participants = {p.identity_id for p in self.participations}
            other_participants = {p.identity_id for p in other.participations}
            common = self_participants & other_participants
            
            if common and not self.temporal_data.overlaps_with(other.temporal_data):
                return f"Same {self.event_type.value} event at different times for same people"
        
        # Birth/death conflicts
        if self.event_type == EventType.BIRTH and other.event_type == EventType.DEATH:
            if self.temporal_data.date_value and other.temporal_data.date_value:
                if self.temporal_data.date_value > other.temporal_data.date_value:
                    return "Birth after death"
        
        # TODO: Add more conflict detection
        
        return None
    
    def calculate_derivative_facts(self) -> List[Dict[str, Any]]:
        """Calculate facts that can be derived from this event"""
        facts = []
        
        # Age calculations
        for participation in self.participations:
            if participation.calculated_birth_year:
                facts.append({
                    'fact_type': 'calculated_birth_year',
                    'identity_id': participation.identity_id,
                    'value': participation.calculated_birth_year,
                    'confidence': 0.8 if participation.age_at_event.isdigit() else 0.5
                })
        
        # Relationship implications
        if self.event_type == EventType.MARRIAGE:
            spouses = self.get_participants_by_role(ParticipantRole.PRINCIPAL)
            if len(spouses) >= 2:
                facts.append({
                    'fact_type': 'relationship',
                    'relationship_type': 'marriage',
                    'participants': [p.identity_id for p in spouses],
                    'start_date': self.temporal_data.date_value
                })
        
        # Residence implications
        if self.event_type in [EventType.CENSUS, EventType.VOTER_REGISTRATION]:
            for participation in self.participations:
                if participation.presence_type == "physical":
                    facts.append({
                        'fact_type': 'residence',
                        'identity_id': participation.identity_id,
                        'location_id': self.location_id,
                        'date': self.temporal_data.date_value
                    })
        
        return facts


@dataclass
class EventSequence:
    """A sequence of related events forming a narrative"""
    sequence_id: UUID = field(default_factory=uuid4)
    
    # What kind of sequence
    sequence_type: str = ""  # "life_events", "migration", "military_service"
    title: str = ""
    description: str = ""
    
    # Events in order
    events: List[UUID] = field(default_factory=list)
    
    # Participants
    primary_participants: Set[UUID] = field(default_factory=set)
    
    # Time span
    start_date: Optional[TemporalPoint] = None
    end_date: Optional[TemporalPoint] = None
    
    # Geographic span
    locations: List[UUID] = field(default_factory=list)
    
    # Theory-specific
    applicable_theories: Set[UUID] = field(default_factory=set)
    
    def add_event(self, event_id: UUID, position: Optional[int] = None) -> None:
        """Add an event to the sequence"""
        if position is None:
            self.events.append(event_id)
        else:
            self.events.insert(position, event_id)
    
    def get_timeline(self) -> List[Tuple[UUID, Optional[date]]]:
        """Get events with their dates in chronological order"""
        # TODO: Would need access to event objects to properly implement
        return [(event_id, None) for event_id in self.events]