"""
ResearchProcess-GPS Core Domain Models

These models implement the RGPS protocols and represent the revolutionary
concepts that make genealogical research version-controlled and theory-based.
"""

from .base import BaseEntity
from .confidence import (
    ConfidenceContainer, ResearchCoverage, AuditCheckItem, 
    GPSCompliance, PeerAssessment
)
from .identity import (
    Identity, Persona, NameForm, BiologicalProfile, 
    SocialIdentity, ExistenceStatus, NameType
)
from .evidence import (
    Evidence, ExtractedFact, EvidenceClassification, EvidenceAnalysis,
    Repository, Citation, NegativeEvidence, SourceType, RecordType, 
    FactType, AnalysisType
)
from .theory import (
    Theory, TheoryBranch, TheoryMetrics, ResearchGap, 
    TheoryConflict, TheoryType
)
from .event import (
    Event, EventParticipation, EventSequence, TemporalPoint,
    EventCategory, EventType, ParticipantRole, DatePrecision
)
from .relationship import (
    Relationship, RelationshipParticipant, RelationshipNetwork,
    RelationshipCategory, RelationshipType, ParticipantRoleType
)
from .location import (
    Location, TemporalLocation, PlaceName, Coordinates, 
    Boundary, Jurisdiction, LocationType, JurisdictionType
)

__all__ = [
    # Base
    'BaseEntity',
    
    # Confidence
    'ConfidenceContainer', 'ResearchCoverage', 'AuditCheckItem',
    'GPSCompliance', 'PeerAssessment',
    
    # Identity
    'Identity', 'Persona', 'NameForm', 'BiologicalProfile',
    'SocialIdentity', 'ExistenceStatus', 'NameType',
    
    # Evidence  
    'Evidence', 'ExtractedFact', 'EvidenceClassification', 'EvidenceAnalysis',
    'Repository', 'Citation', 'NegativeEvidence', 'SourceType', 'RecordType',
    'FactType', 'AnalysisType',
    
    # Theory
    'Theory', 'TheoryBranch', 'TheoryMetrics', 'ResearchGap',
    'TheoryConflict', 'TheoryType',
    
    # Event
    'Event', 'EventParticipation', 'EventSequence', 'TemporalPoint',
    'EventCategory', 'EventType', 'ParticipantRole', 'DatePrecision',
    
    # Relationship
    'Relationship', 'RelationshipParticipant', 'RelationshipNetwork',
    'RelationshipCategory', 'RelationshipType', 'ParticipantRoleType',
    
    # Location
    'Location', 'TemporalLocation', 'PlaceName', 'Coordinates',
    'Boundary', 'Jurisdiction', 'LocationType', 'JurisdictionType'
]