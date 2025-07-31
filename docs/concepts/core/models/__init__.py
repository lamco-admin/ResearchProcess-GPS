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
from .research_question import (
    ResearchQuestion, WorkingHypothesis, ResearchPlan, Conclusion,
    ResearchQuestionType, ResearchQuestionStatus, ResearchScope
)
from .analysis import (
    Analysis, AnalyticalPoint, CorrelationSet, ConflictResolution,
    AnalysisFramework, AnalysisType, AnalysisMethodology, ArgumentStrength
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
from .theory import (
    Theory, TheoryType, TheoryMetrics,
    ResearchGap, TheoryConflict, TheoryBranch
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
    
    # Research Question
    'ResearchQuestion', 'WorkingHypothesis', 'ResearchPlan', 'Conclusion',
    'ResearchQuestionType', 'ResearchQuestionStatus', 'ResearchScope',
    
    # Analysis
    'Analysis', 'AnalyticalPoint', 'CorrelationSet', 'ConflictResolution',
    'AnalysisFramework', 'AnalysisType', 'AnalysisMethodology', 'ArgumentStrength',
    
    # Event
    'Event', 'EventParticipation', 'EventSequence', 'TemporalPoint',
    'EventCategory', 'EventType', 'ParticipantRole', 'DatePrecision',
    
    # Relationship
    'Relationship', 'RelationshipParticipant', 'RelationshipNetwork',
    'RelationshipCategory', 'RelationshipType', 'ParticipantRoleType',
    
    # Location
    'Location', 'TemporalLocation', 'PlaceName', 'Coordinates',
    'Boundary', 'Jurisdiction', 'LocationType', 'JurisdictionType',
    
    # Theory
    'Theory', 'TheoryType', 'TheoryMetrics',
    'ResearchGap', 'TheoryConflict', 'TheoryBranch'
]