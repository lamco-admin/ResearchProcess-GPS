"""
ResearchProcess-GPS Core Domain Models

These models implement the RGPS protocols and represent the revolutionary
concepts that make genealogical research version-controlled and theory-based.
"""

from .identity import Identity, Persona
from .evidence import Evidence, ExtractedFact, EvidenceClassification
from .theory import Theory, TheoryBranch
from .event import Event, EventParticipation
from .relationship import Relationship, RelationshipParticipant
from .location import Location, TemporalLocation
from .confidence import ConfidenceContainer, ResearchCoverage

__all__ = [
    'Identity', 'Persona',
    'Evidence', 'ExtractedFact', 'EvidenceClassification',
    'Theory', 'TheoryBranch',
    'Event', 'EventParticipation',
    'Relationship', 'RelationshipParticipant',
    'Location', 'TemporalLocation',
    'ConfidenceContainer', 'ResearchCoverage'
]