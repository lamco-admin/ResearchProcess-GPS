"""
Identity nesting capabilities for ResearchProcess-GPS.

Identities can nest in various ways to represent collective identities,
identity evolution, uncertain identities, and theoretical groupings.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set
from datetime import datetime, date
from uuid import UUID, uuid4
from enum import Enum

from .identity import Identity, Persona, ExistenceStatus
from ..abstractions.nesting import NestableEntity, NestingType, NestingConstraints


class IdentityNestingType(Enum):
    """Types of identity nesting relationships"""
    COLLECTIVE = "collective"              # Group identity containing individuals
    TEMPORAL_PHASE = "temporal_phase"      # Life phases of same person
    ROLE_BASED = "role_based"             # Professional/social roles
    UNCERTAIN_GROUP = "uncertain_group"    # Possible identities for unknown person
    FAMILY_PLACEHOLDER = "family_placeholder"  # Unknown family members
    THEORETICAL_SET = "theoretical_set"    # Set of theoretical identities
    SPLIT_IDENTITY = "split_identity"      # Identity that may be multiple people
    MERGED_IDENTITY = "merged_identity"    # Multiple people who may be one


@dataclass
class NestableIdentity(Identity, NestableEntity['NestableIdentity']):
    """
    An Identity that can contain other identities, enabling representation
    of complex identity relationships in genealogical research.
    """
    
    def __init__(self, **kwargs):
        Identity.__init__(self, **kwargs)
        NestableEntity.__init__(self)
        
        # Nesting-specific attributes
        self.identity_nesting_type: IdentityNestingType = IdentityNestingType.COLLECTIVE
        self.nesting_rationale: str = ""
        
        # Constraints for identity nesting
        self.constraints = NestingConstraints(
            max_depth=3,  # Don't nest too deeply
            circular_reference_allowed=False
        )
    
    def can_contain(self, child: 'NestableIdentity') -> bool:
        """Determine if this identity can contain another"""
        
        # Collective identities can contain individuals
        if self.identity_nesting_type == IdentityNestingType.COLLECTIVE:
            return child.identity_nesting_type not in [
                IdentityNestingType.COLLECTIVE,
                IdentityNestingType.THEORETICAL_SET
            ]
        
        # Temporal phases must be of same person (would need to verify)
        elif self.identity_nesting_type == IdentityNestingType.TEMPORAL_PHASE:
            # In practice, would check if they represent same person
            return child.identity_nesting_type == IdentityNestingType.TEMPORAL_PHASE
        
        # Uncertain groups can contain any candidate
        elif self.identity_nesting_type == IdentityNestingType.UNCERTAIN_GROUP:
            return True
        
        # Family placeholders can contain family members
        elif self.identity_nesting_type == IdentityNestingType.FAMILY_PLACEHOLDER:
            return True
        
        # Theoretical sets can contain any identity
        elif self.identity_nesting_type == IdentityNestingType.THEORETICAL_SET:
            return True
        
        return False
    
    def validate_nesting(self) -> List[str]:
        """Validate identity nesting structure"""
        issues = []
        
        # Check basic nesting constraints
        issues.extend(super().validate_nesting())
        
        # Collective identities should have multiple children
        if (self.identity_nesting_type == IdentityNestingType.COLLECTIVE and 
            len(self.children) < 2):
            issues.append("Collective identity should contain multiple individuals")
        
        # Temporal phases should be chronological
        if self.identity_nesting_type == IdentityNestingType.TEMPORAL_PHASE:
            # Would need to check chronological order of children
            pass
        
        return issues


@dataclass
class CollectiveIdentity(NestableIdentity):
    """
    Represents a group of people treated as a unit.
    Examples: "The Smith Brothers", "Unknown Miller Children"
    """
    
    def __init__(self, collective_name: str, identity_type: str = "siblings"):
        super().__init__()
        self.identity_nesting_type = IdentityNestingType.COLLECTIVE
        self.collective_name = collective_name
        self.collective_type = identity_type  # "siblings", "partners", "group"
        
        # Set primary name as collective
        from .identity import NameForm, NameType
        self.add_name(NameForm(
            given_names=[collective_name],
            name_type=NameType.UNKNOWN
        ))
    
    def add_member(self, member: Identity, role: str = "member") -> bool:
        """Add a member to the collective"""
        if self.add_child(member):
            self.nesting_metadata[member.id] = {
                "role": role,
                "added_date": datetime.utcnow()
            }
            return True
        return False
    
    def get_members_by_role(self, role: str) -> List[Identity]:
        """Get all members with a specific role"""
        return [
            child for child in self.children
            if self.nesting_metadata.get(child.id, {}).get("role") == role
        ]


@dataclass
class TemporalIdentityPhases(NestableIdentity):
    """
    Represents different life phases of the same person.
    Example: "John Smith (youth)", "John Smith (adult)", "John Smith (elder)"
    """
    
    def __init__(self, base_name: str):
        super().__init__()
        self.identity_nesting_type = IdentityNestingType.TEMPORAL_PHASE
        self.base_name = base_name
    
    def add_phase(self, phase_identity: Identity, 
                  start_year: Optional[int] = None,
                  end_year: Optional[int] = None,
                  phase_name: str = "") -> bool:
        """Add a life phase"""
        if self.add_child(phase_identity):
            self.nesting_metadata[phase_identity.id] = {
                "phase_name": phase_name,
                "start_year": start_year,
                "end_year": end_year
            }
            return True
        return False
    
    def get_phase_at_year(self, year: int) -> Optional[Identity]:
        """Get the identity phase active in a given year"""
        for child in self.children:
            metadata = self.nesting_metadata.get(child.id, {})
            start = metadata.get("start_year")
            end = metadata.get("end_year")
            
            if start and year < start:
                continue
            if end and year > end:
                continue
                
            return child
        
        return None


@dataclass
class UncertainIdentityGroup(NestableIdentity):
    """
    Represents multiple possible identities for an unknown person.
    Example: "Person who married Mary Jones" with candidates
    """
    
    def __init__(self, description: str, research_question_id: Optional[UUID] = None):
        super().__init__()
        self.identity_nesting_type = IdentityNestingType.UNCERTAIN_GROUP
        self.description = description
        self.research_question_id = research_question_id
        self.existence_status = ExistenceStatus.HYPOTHETICAL
    
    def add_candidate(self, candidate: Identity, 
                     confidence: float = 0.5,
                     evidence_for: List[UUID] = None,
                     evidence_against: List[UUID] = None) -> bool:
        """Add a candidate identity"""
        if self.add_child(candidate):
            self.nesting_metadata[candidate.id] = {
                "confidence": confidence,
                "evidence_for": evidence_for or [],
                "evidence_against": evidence_against or [],
                "added_date": datetime.utcnow()
            }
            return True
        return False
    
    def get_best_candidate(self) -> Optional[Identity]:
        """Get the candidate with highest confidence"""
        best = None
        best_confidence = 0.0
        
        for child in self.children:
            confidence = self.nesting_metadata.get(child.id, {}).get("confidence", 0)
            if confidence > best_confidence:
                best = child
                best_confidence = confidence
        
        return best
    
    def resolve_to_candidate(self, chosen: Identity) -> Identity:
        """Resolve uncertainty by choosing a candidate"""
        if chosen not in self.children:
            raise ValueError("Chosen identity must be a candidate")
        
        # Mark this as resolved
        self.existence_status = ExistenceStatus.MERGED
        self.nesting_metadata["resolution"] = {
            "chosen_id": chosen.id,
            "resolution_date": datetime.utcnow(),
            "other_candidates": [c.id for c in self.children if c.id != chosen.id]
        }
        
        return chosen


@dataclass
class FamilyPlaceholderGroup(NestableIdentity):
    """
    Represents unknown family members as a group.
    Example: "Unknown children of John and Mary Smith"
    """
    
    def __init__(self, description: str, family_context: Dict[str, Any]):
        super().__init__()
        self.identity_nesting_type = IdentityNestingType.FAMILY_PLACEHOLDER
        self.description = description
        self.family_context = family_context  # {"parents": [...], "family_id": ...}
        self.existence_status = ExistenceStatus.HYPOTHETICAL
    
    def add_discovered_member(self, member: Identity, 
                            birth_order: Optional[int] = None) -> bool:
        """Add a discovered family member"""
        if self.add_child(member):
            self.nesting_metadata[member.id] = {
                "birth_order": birth_order,
                "discovery_date": datetime.utcnow(),
                "status": "confirmed"
            }
            member.existence_status = ExistenceStatus.EVIDENCED
            return True
        return False
    
    def add_hypothetical_member(self, placeholder_name: str,
                              estimated_birth: Optional[int] = None) -> Identity:
        """Add a hypothetical family member"""
        member = NestableIdentity()
        member.existence_status = ExistenceStatus.HYPOTHETICAL
        
        from .identity import NameForm, NameType
        member.add_name(NameForm(
            given_names=[placeholder_name],
            name_type=NameType.UNKNOWN
        ))
        
        if self.add_child(member):
            self.nesting_metadata[member.id] = {
                "estimated_birth": estimated_birth,
                "status": "hypothetical"
            }
        
        return member


@dataclass 
class TheoreticalIdentitySet(NestableIdentity):
    """
    A set of identities being analyzed together for research purposes.
    Example: "All John Smiths in County X between 1820-1850"
    """
    
    def __init__(self, set_description: str, selection_criteria: Dict[str, Any]):
        super().__init__()
        self.identity_nesting_type = IdentityNestingType.THEORETICAL_SET
        self.set_description = set_description
        self.selection_criteria = selection_criteria
    
    def add_identity(self, identity: Identity, 
                    inclusion_reason: str,
                    match_score: float = 0.0) -> bool:
        """Add an identity to the theoretical set"""
        if self.add_child(identity):
            self.nesting_metadata[identity.id] = {
                "inclusion_reason": inclusion_reason,
                "match_score": match_score,
                "analysis_status": "pending"
            }
            return True
        return False
    
    def mark_analyzed(self, identity: Identity, 
                     analysis_result: str,
                     is_match: bool) -> None:
        """Mark an identity as analyzed"""
        if identity.id in self.nesting_metadata:
            self.nesting_metadata[identity.id].update({
                "analysis_status": "complete",
                "analysis_result": analysis_result,
                "is_match": is_match,
                "analysis_date": datetime.utcnow()
            })
    
    def get_matches(self) -> List[Identity]:
        """Get identities that matched the research criteria"""
        return [
            child for child in self.children
            if self.nesting_metadata.get(child.id, {}).get("is_match", False)
        ]


class IdentityNestingAnalyzer:
    """Analyzes nested identity structures"""
    
    @staticmethod
    def find_identity_overlaps(set1: TheoreticalIdentitySet, 
                              set2: TheoreticalIdentitySet) -> List[Identity]:
        """Find identities that appear in both sets"""
        ids1 = {child.id for child in set1.children}
        ids2 = {child.id for child in set2.children}
        
        overlap_ids = ids1.intersection(ids2)
        
        return [child for child in set1.children if child.id in overlap_ids]
    
    @staticmethod
    def merge_collective_identities(collective1: CollectiveIdentity,
                                  collective2: CollectiveIdentity) -> CollectiveIdentity:
        """Merge two collective identities"""
        merged = CollectiveIdentity(
            f"{collective1.collective_name} + {collective2.collective_name}",
            collective1.collective_type
        )
        
        # Add all members from both
        for member in collective1.children + collective2.children:
            role1 = collective1.nesting_metadata.get(member.id, {}).get("role", "member")
            role2 = collective2.nesting_metadata.get(member.id, {}).get("role", "member") 
            merged.add_member(member, role1)  # Would need to handle role conflicts
        
        return merged
    
    @staticmethod
    def extract_individuals_from_collective(collective: CollectiveIdentity) -> List[Identity]:
        """Extract individual identities from a collective"""
        individuals = []
        
        for child in collective.children:
            # Create standalone copy
            individual = Identity()
            # Copy relevant attributes from child
            # ... implementation
            individuals.append(individual)
        
        return individuals