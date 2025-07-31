"""
Research Question model for ResearchProcess-GPS.

A Research Question is what drives genealogical research - the specific
question we're trying to answer with evidence and analysis.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum

from .base import NestableBaseEntity
from .confidence import ConfidenceContainer, GPSCompliance
from ..abstractions.nesting import NestingType


class ResearchQuestionType(Enum):
    """Types of genealogical research questions"""
    IDENTITY = "identity"              # Who was this person?
    PARENTAGE = "parentage"            # Who were their parents?
    RELATIONSHIP = "relationship"      # How were these people related?
    EVENT = "event"                    # What happened? When? Where?
    MIGRATION = "migration"            # Where did they come from/go?
    ORIGIN = "origin"                  # Where did family originate?
    VITAL = "vital"                    # Birth/Marriage/Death details
    CONFLICT = "conflict"              # Resolving conflicting evidence
    VERIFICATION = "verification"      # Confirming existing information
    EXPLORATION = "exploration"        # Open-ended research
    OTHER = "other"


class ResearchQuestionStatus(Enum):
    """Status of a research question"""
    PLANNING = "planning"              # Still planning approach
    ACTIVE = "active"                  # Actively researching
    ANALYZING = "analyzing"            # Analyzing gathered evidence
    TESTING = "testing"                # Testing hypotheses
    REVIEW = "review"                  # Under peer review
    CONCLUDED = "concluded"            # Reached conclusion
    ABANDONED = "abandoned"            # No longer pursuing
    BLOCKED = "blocked"                # Blocked by lack of evidence


class ResearchScope(Enum):
    """Scope of research"""
    NARROW = "narrow"                  # Single person/event
    FAMILY = "family"                  # Family group
    LINEAGE = "lineage"                # Single line
    SURNAME = "surname"                # All with surname
    LOCALITY = "locality"              # Geographic area
    BROAD = "broad"                    # Large scale


@dataclass
class ResearchPlan:
    """Plan for addressing a research question"""
    plan_id: UUID = field(default_factory=uuid4)
    
    # Strategy
    approach: str = ""                 # How we'll tackle this
    
    # Resources to check
    repositories: List[str] = field(default_factory=list)
    record_types: List[str] = field(default_factory=list)
    online_sources: List[str] = field(default_factory=list)
    
    # Geographic scope
    locations: List[UUID] = field(default_factory=list)
    
    # Temporal scope  
    time_period_start: Optional[int] = None
    time_period_end: Optional[int] = None
    
    # Specific searches
    planned_searches: List[Dict[str, Any]] = field(default_factory=list)
    
    # Expected evidence
    expected_evidence_types: List[str] = field(default_factory=list)
    
    # Constraints
    constraints: List[str] = field(default_factory=list)
    
    # Progress tracking
    searches_completed: List[Dict[str, Any]] = field(default_factory=list)
    repositories_visited: Set[str] = field(default_factory=set)
    
    def add_search(self, description: str, details: Dict[str, Any]) -> None:
        """Add a planned search"""
        self.planned_searches.append({
            "description": description,
            "details": details,
            "status": "planned",
            "results": None
        })
    
    def complete_search(self, search_index: int, results: Dict[str, Any]) -> None:
        """Mark a search as completed"""
        if 0 <= search_index < len(self.planned_searches):
            self.planned_searches[search_index]["status"] = "completed"
            self.planned_searches[search_index]["results"] = results


@dataclass
class WorkingHypothesis:
    """A tentative answer to a research question"""
    hypothesis_id: UUID = field(default_factory=uuid4)
    
    # The hypothesis
    statement: str = ""                # "John Smith's parents were James and Mary"
    
    # Key assertions
    assertions: List[str] = field(default_factory=list)
    # ["John born ~1820", "Parents married ~1818", "James died 1825"]
    
    # Supporting evidence
    supporting_evidence: List[UUID] = field(default_factory=list)
    evidence_interpretation: Dict[UUID, str] = field(default_factory=dict)
    
    # Contradicting evidence
    contradicting_evidence: List[UUID] = field(default_factory=list)
    contradiction_explanations: Dict[UUID, str] = field(default_factory=dict)
    
    # Assumptions made
    assumptions: List[str] = field(default_factory=list)
    
    # Testable predictions
    predictions: List[str] = field(default_factory=list)
    # ["Should find John in 1850 census with James", "Birth record should show James as father"]
    
    # Test results
    test_results: List[Dict[str, Any]] = field(default_factory=list)
    
    # Confidence
    confidence: Optional[ConfidenceContainer] = None
    
    # Status
    status: str = "active"  # "active", "testing", "supported", "refuted", "revised"
    
    # History
    created_date: datetime = field(default_factory=datetime.utcnow)
    last_revised: datetime = field(default_factory=datetime.utcnow)
    revision_history: List[Dict[str, Any]] = field(default_factory=list)
    
    def add_test_result(self, prediction: str, result: str, evidence: Optional[UUID] = None) -> None:
        """Add result of testing a prediction"""
        self.test_results.append({
            "prediction": prediction,
            "result": result,
            "evidence": evidence,
            "date": datetime.utcnow()
        })
    
    def revise(self, new_statement: str, reason: str) -> None:
        """Revise the hypothesis"""
        self.revision_history.append({
            "old_statement": self.statement,
            "new_statement": new_statement,
            "reason": reason,
            "date": datetime.utcnow()
        })
        self.statement = new_statement
        self.last_revised = datetime.utcnow()


@dataclass
class ResearchQuestion(NestableBaseEntity['ResearchQuestion']):
    """
    A Research Question drives genealogical research. It represents what
    we're trying to find out, tracks our progress, and eventually reaches
    a conclusion.
    
    Research questions naturally nest - complex questions break down into
    sub-questions, forming a research hierarchy.
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "ResearchQuestion"
        self.nesting_type = NestingType.HIERARCHICAL  # Questions form hierarchies
    
    # The question
    question: str = ""                 # "Who were John Smith's parents?"
    question_type: ResearchQuestionType = ResearchQuestionType.OTHER
    
    # Context
    background: str = ""               # Why we're asking this
    significance: str = ""             # Why it matters
    
    # Scope
    scope: ResearchScope = ResearchScope.NARROW
    primary_identities: List[UUID] = field(default_factory=list)
    related_identities: List[UUID] = field(default_factory=list)
    geographic_scope: List[UUID] = field(default_factory=list)  # Locations
    temporal_scope: Tuple[Optional[int], Optional[int]] = (None, None)
    
    # Related questions (beyond parent/child which is handled by nesting)
    related_questions: List[UUID] = field(default_factory=list)  # Lateral relationships
    
    # Research plan
    research_plan: Optional[ResearchPlan] = None
    
    # Working hypotheses
    working_hypotheses: List[WorkingHypothesis] = field(default_factory=list)
    active_hypothesis: Optional[UUID] = None
    
    # Evidence gathered
    evidence_examined: List[UUID] = field(default_factory=list)
    
    # Analyses performed
    analyses: List[UUID] = field(default_factory=list)  # References to Analysis entities
    
    # Status tracking
    status: ResearchQuestionStatus = ResearchQuestionStatus.PLANNING
    
    # GPS compliance
    gps_compliance: Optional[GPSCompliance] = None
    
    # Conclusion (when reached)
    conclusion: Optional['Conclusion'] = None
    conclusion_date: Optional[datetime] = None
    
    # Collaboration
    researchers: List[str] = field(default_factory=list)
    peer_reviews: List[Dict[str, Any]] = field(default_factory=list)
    
    # History
    created_date: datetime = field(default_factory=datetime.utcnow)
    status_history: List[Dict[str, Any]] = field(default_factory=list)
    
    def create_hypothesis(self, statement: str) -> WorkingHypothesis:
        """Create a new working hypothesis"""
        hypothesis = WorkingHypothesis(statement=statement)
        self.working_hypotheses.append(hypothesis)
        if not self.active_hypothesis:
            self.active_hypothesis = hypothesis.hypothesis_id
        return hypothesis
    
    def add_sub_question(self, question: str, question_type: ResearchQuestionType) -> 'ResearchQuestion':
        """Add a sub-question"""
        sub_question = ResearchQuestion()
        sub_question.question = question
        sub_question.question_type = question_type
        sub_question.scope = self.scope  # Inherit scope by default
        
        # Add as child using nesting
        self.add_child(sub_question)
        self.nesting_metadata[sub_question.id] = {
            "order": len(self.children),
            "created_date": datetime.utcnow()
        }
        
        return sub_question
    
    def update_status(self, new_status: ResearchQuestionStatus, notes: str = "") -> None:
        """Update the status with history tracking"""
        self.status_history.append({
            "old_status": self.status,
            "new_status": new_status,
            "date": datetime.utcnow(),
            "notes": notes
        })
        self.status = new_status
    
    def can_conclude(self) -> Tuple[bool, List[str]]:
        """Check if question can be concluded"""
        issues = []
        
        # Must have active hypothesis
        if not self.active_hypothesis:
            issues.append("No active hypothesis to conclude")
        
        # Must have evidence
        if not self.evidence_examined:
            issues.append("No evidence examined")
        
        # Must have GPS compliance
        if not self.gps_compliance or not self.gps_compliance.is_compliant():
            issues.append("Not GPS compliant")
        
        # Should have peer review
        if not self.peer_reviews:
            issues.append("No peer review completed")
        
        return len(issues) == 0, issues
    
    def get_sub_questions(self) -> List['ResearchQuestion']:
        """Get all direct sub-questions"""
        return [child for child in self.children if isinstance(child, ResearchQuestion)]
    
    def get_all_sub_questions(self) -> List['ResearchQuestion']:
        """Get all sub-questions recursively"""
        return [child for child in self.get_children(recursive=True) 
                if isinstance(child, ResearchQuestion)]
    
    def is_compound_question(self) -> bool:
        """Check if this question has sub-questions"""
        return len(self.get_sub_questions()) > 0
    
    def get_unanswered_questions(self) -> List['ResearchQuestion']:
        """Get all unanswered questions in the hierarchy"""
        unanswered = []
        if self.status not in [ResearchQuestionStatus.CONCLUDED, ResearchQuestionStatus.ABANDONED]:
            unanswered.append(self)
        
        for sub in self.get_all_sub_questions():
            if sub.status not in [ResearchQuestionStatus.CONCLUDED, ResearchQuestionStatus.ABANDONED]:
                unanswered.append(sub)
        
        return unanswered


@dataclass
class Conclusion:
    """
    A Conclusion is the resolved state of a Research Question.
    It represents defensible answers that can be mapped to traditional
    genealogical entities (persons, events, relationships).
    """
    conclusion_id: UUID = field(default_factory=uuid4)
    
    # Link to research question
    research_question_id: UUID = field(default_factory=uuid4)
    
    # The conclusion statement
    statement: str = ""
    
    # Key findings
    findings: List[str] = field(default_factory=list)
    
    # Final hypothesis that was validated
    validated_hypothesis: Optional[WorkingHypothesis] = None
    
    # Evidence summary
    key_evidence: List[UUID] = field(default_factory=list)
    evidence_summary: str = ""
    
    # GPS compliance documentation
    gps_compliance: Optional[GPSCompliance] = None
    
    # Confidence in conclusion
    confidence: Optional[ConfidenceContainer] = None
    
    # Limitations and caveats
    limitations: List[str] = field(default_factory=list)
    caveats: List[str] = field(default_factory=list)
    
    # Future research needed
    future_research: List[str] = field(default_factory=list)
    
    # Mappings to traditional genealogy
    entity_mappings: Dict[str, List[UUID]] = field(default_factory=dict)
    # {"persons": [...], "events": [...], "relationships": [...]}
    
    # Publication info
    publication_date: datetime = field(default_factory=datetime.utcnow)
    published_by: str = ""
    peer_reviewers: List[str] = field(default_factory=list)
    
    # Citation for this conclusion
    citation: str = ""
    
    def add_entity_mapping(self, entity_type: str, entity_id: UUID) -> None:
        """Add mapping to traditional genealogy entity"""
        if entity_type not in self.entity_mappings:
            self.entity_mappings[entity_type] = []
        self.entity_mappings[entity_type].append(entity_id)
    
    def create_traditional_mapping(self) -> Dict[str, Any]:
        """Create mapping to traditional genealogy format"""
        return {
            "persons": self.entity_mappings.get("persons", []),
            "events": self.entity_mappings.get("events", []),
            "relationships": self.entity_mappings.get("relationships", []),
            "sources": self.key_evidence,
            "notes": self.statement
        }