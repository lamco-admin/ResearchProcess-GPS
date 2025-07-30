"""
Analysis model for ResearchProcess-GPS.

Analysis is the work of interpreting evidence, correlating information,
identifying conflicts, and building arguments. It's a first-class entity
that tracks the reasoning process.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum

from .base import BaseEntity
from .confidence import ConfidenceContainer


class AnalysisType(Enum):
    """Types of genealogical analysis"""
    EVIDENCE_CORRELATION = "evidence_correlation"    # Connecting related evidence
    IDENTITY_RESOLUTION = "identity_resolution"      # Same person in different records?
    CONFLICT_RESOLUTION = "conflict_resolution"      # Resolving contradictions
    TIMELINE_CONSTRUCTION = "timeline_construction"  # Building chronology
    RELATIONSHIP_ANALYSIS = "relationship_analysis"  # Determining relationships
    LOCATION_ANALYSIS = "location_analysis"          # Place identification
    NAME_ANALYSIS = "name_analysis"                  # Name variations/changes
    DATE_ANALYSIS = "date_analysis"                  # Dating undated records
    HANDWRITING_ANALYSIS = "handwriting_analysis"    # Paleography
    DNA_ANALYSIS = "dna_analysis"                    # Genetic genealogy
    SOURCE_CRITICISM = "source_criticism"            # Evaluating source reliability
    NEGATIVE_EVIDENCE = "negative_evidence"          # Why something is missing
    CLUSTER_ANALYSIS = "cluster_analysis"            # FAN club analysis
    MIGRATION_ANALYSIS = "migration_analysis"        # Movement patterns
    CUSTOM = "custom"


class AnalysisMethodology(Enum):
    """Methodologies used in analysis"""
    DIRECT_EVIDENCE = "direct_evidence"              # Evidence directly states
    INDIRECT_EVIDENCE = "indirect_evidence"          # Requires inference
    CORRELATION = "correlation"                      # Comparing multiple sources
    ELIMINATION = "elimination"                      # Process of elimination
    TRIANGULATION = "triangulation"                  # Three+ sources agree
    PREPONDERANCE = "preponderance"                  # Weight of evidence
    CHRONOLOGICAL = "chronological"                  # Timeline-based
    GEOGRAPHICAL = "geographical"                    # Location-based
    SOCIAL_NETWORK = "social_network"                # FAN principle
    STATISTICAL = "statistical"                      # Statistical analysis
    COMPARATIVE = "comparative"                      # Comparing similar cases


class ArgumentStrength(Enum):
    """Strength of analytical arguments"""
    PROOF = "proof"                    # Meets GPS standard
    STRONG = "strong"                  # Very likely correct
    MODERATE = "moderate"              # More likely than not
    WEAK = "weak"                      # Possible but uncertain
    SPECULATIVE = "speculative"        # Educated guess
    DISPROVEN = "disproven"           # Shown to be false


@dataclass
class AnalyticalPoint:
    """A single point in an analysis"""
    point_id: UUID = field(default_factory=uuid4)
    
    # The assertion
    assertion: str = ""                # "John in 1850 census is same as 1860"
    
    # Evidence used
    evidence_refs: List[UUID] = field(default_factory=list)
    
    # Reasoning
    reasoning: str = ""                # Why we think this
    methodology: AnalysisMethodology = AnalysisMethodology.DIRECT_EVIDENCE
    
    # Strength
    strength: ArgumentStrength = ArgumentStrength.MODERATE
    
    # Alternative interpretations
    alternatives: List[str] = field(default_factory=list)
    why_preferred: str = ""            # Why this interpretation over others
    
    # Dependencies
    depends_on: List[UUID] = field(default_factory=list)  # Other points
    
    # Confidence
    confidence: float = 0.5            # 0-1


@dataclass
class CorrelationSet:
    """A set of correlated evidence"""
    correlation_id: UUID = field(default_factory=uuid4)
    
    # Evidence being correlated
    evidence_items: List[UUID] = field(default_factory=list)
    
    # What they have in common
    correlation_points: List[str] = field(default_factory=list)
    # ["Same name", "Same age", "Same location"]
    
    # Differences noted
    differences: List[str] = field(default_factory=list)
    # ["Middle initial different", "Age off by 2 years"]
    
    # Conclusion
    conclusion: str = ""               # "Same person" or "Different people"
    confidence: float = 0.0


@dataclass
class ConflictResolution:
    """Resolution of conflicting evidence"""
    conflict_id: UUID = field(default_factory=uuid4)
    
    # Conflicting evidence
    evidence_a: UUID = field(default_factory=uuid4)
    evidence_b: UUID = field(default_factory=uuid4)
    
    # Nature of conflict
    conflict_type: str = ""            # "date", "name", "relationship", etc.
    conflict_description: str = ""
    
    # Possible explanations
    explanations: List[str] = field(default_factory=list)
    
    # Preferred resolution
    resolution: str = ""
    resolution_reasoning: str = ""
    
    # Which evidence is preferred
    preferred_evidence: Optional[UUID] = None
    
    # Confidence in resolution
    confidence: float = 0.0


@dataclass
class Analysis(BaseEntity):
    """
    Analysis is the intellectual work of genealogical research.
    It documents how we interpret evidence and reach conclusions.
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Analysis"
    
    # Basic properties
    title: str = ""
    analysis_type: AnalysisType = AnalysisType.CUSTOM
    
    # What's being analyzed
    research_question: Optional[UUID] = None  # Link to research question
    scope_description: str = ""
    
    # Evidence analyzed
    evidence_analyzed: List[UUID] = field(default_factory=list)
    
    # Methodology
    methodology: List[AnalysisMethodology] = field(default_factory=list)
    methodology_notes: str = ""
    
    # The analysis itself
    analytical_points: List[AnalyticalPoint] = field(default_factory=list)
    
    # Correlations found
    correlations: List[CorrelationSet] = field(default_factory=list)
    
    # Conflicts resolved
    conflict_resolutions: List[ConflictResolution] = field(default_factory=list)
    
    # Building the argument
    argument_structure: List[str] = field(default_factory=list)
    # ["First, establish identity", "Then, show relationships", "Finally, resolve conflicts"]
    
    # Conclusions reached
    conclusions: List[str] = field(default_factory=list)
    overall_strength: ArgumentStrength = ArgumentStrength.MODERATE
    
    # Limitations
    limitations: List[str] = field(default_factory=list)
    assumptions: List[str] = field(default_factory=list)
    
    # What's still needed
    gaps_identified: List[str] = field(default_factory=list)
    further_research: List[str] = field(default_factory=list)
    
    # Visual aids
    charts: List[Dict[str, Any]] = field(default_factory=list)  # Timelines, maps, etc.
    
    # Peer review
    peer_reviews: List[Dict[str, Any]] = field(default_factory=list)
    
    # Metadata
    analyst: str = ""
    analysis_date: datetime = field(default_factory=datetime.utcnow)
    last_revised: datetime = field(default_factory=datetime.utcnow)
    
    def add_point(self, assertion: str, evidence: List[UUID], 
                  reasoning: str) -> AnalyticalPoint:
        """Add an analytical point"""
        point = AnalyticalPoint(
            assertion=assertion,
            evidence_refs=evidence,
            reasoning=reasoning
        )
        self.analytical_points.append(point)
        return point
    
    def add_correlation(self, evidence_items: List[UUID], 
                       common_points: List[str]) -> CorrelationSet:
        """Add a correlation between evidence items"""
        correlation = CorrelationSet(
            evidence_items=evidence_items,
            correlation_points=common_points
        )
        self.correlations.append(correlation)
        return correlation
    
    def resolve_conflict(self, evidence_a: UUID, evidence_b: UUID,
                        conflict_type: str, resolution: str) -> ConflictResolution:
        """Document resolution of a conflict"""
        conflict = ConflictResolution(
            evidence_a=evidence_a,
            evidence_b=evidence_b,
            conflict_type=conflict_type,
            resolution=resolution
        )
        self.conflict_resolutions.append(conflict)
        return conflict
    
    def build_timeline(self, events: List[Tuple[UUID, str, Any]]) -> Dict[str, Any]:
        """Build a timeline from analyzed events"""
        timeline = {
            "id": uuid4(),
            "title": f"Timeline for {self.title}",
            "events": []
        }
        
        for event_id, description, date_info in events:
            timeline["events"].append({
                "event_id": event_id,
                "description": description,
                "date": date_info,
                "evidence": []  # Would be filled with supporting evidence
            })
        
        # Sort by date
        # Real implementation would handle complex dates
        
        self.charts.append(timeline)
        return timeline
    
    def calculate_overall_strength(self) -> ArgumentStrength:
        """Calculate overall strength of the analysis"""
        if not self.analytical_points:
            return ArgumentStrength.SPECULATIVE
        
        # Average the strength of all points
        strength_values = {
            ArgumentStrength.PROOF: 5,
            ArgumentStrength.STRONG: 4,
            ArgumentStrength.MODERATE: 3,
            ArgumentStrength.WEAK: 2,
            ArgumentStrength.SPECULATIVE: 1,
            ArgumentStrength.DISPROVEN: 0
        }
        
        total = sum(strength_values.get(point.strength, 0) 
                   for point in self.analytical_points)
        average = total / len(self.analytical_points)
        
        # Map back to strength
        if average >= 4.5:
            self.overall_strength = ArgumentStrength.PROOF
        elif average >= 3.5:
            self.overall_strength = ArgumentStrength.STRONG
        elif average >= 2.5:
            self.overall_strength = ArgumentStrength.MODERATE
        elif average >= 1.5:
            self.overall_strength = ArgumentStrength.WEAK
        else:
            self.overall_strength = ArgumentStrength.SPECULATIVE
        
        return self.overall_strength
    
    def export_narrative(self) -> str:
        """Export analysis as narrative text"""
        narrative = [f"# {self.title}\n"]
        
        # Introduction
        narrative.append(f"## Scope\n{self.scope_description}\n")
        
        # Methodology
        narrative.append("## Methodology\n")
        for method in self.methodology:
            narrative.append(f"- {method.value}\n")
        if self.methodology_notes:
            narrative.append(f"\n{self.methodology_notes}\n")
        
        # Analysis
        narrative.append("\n## Analysis\n")
        for i, point in enumerate(self.analytical_points, 1):
            narrative.append(f"\n### Point {i}: {point.assertion}\n")
            narrative.append(f"{point.reasoning}\n")
            if point.alternatives:
                narrative.append("\nAlternative interpretations considered:\n")
                for alt in point.alternatives:
                    narrative.append(f"- {alt}\n")
                narrative.append(f"\n{point.why_preferred}\n")
        
        # Correlations
        if self.correlations:
            narrative.append("\n## Correlations\n")
            for corr in self.correlations:
                narrative.append(f"\n{corr.conclusion}\n")
                narrative.append("Common points:\n")
                for point in corr.correlation_points:
                    narrative.append(f"- {point}\n")
        
        # Conflicts
        if self.conflict_resolutions:
            narrative.append("\n## Conflict Resolutions\n")
            for conf in self.conflict_resolutions:
                narrative.append(f"\n**{conf.conflict_type}**: {conf.conflict_description}\n")
                narrative.append(f"Resolution: {conf.resolution}\n")
        
        # Conclusions
        narrative.append("\n## Conclusions\n")
        for conclusion in self.conclusions:
            narrative.append(f"- {conclusion}\n")
        
        narrative.append(f"\nOverall strength: {self.overall_strength.value}\n")
        
        # Limitations
        if self.limitations:
            narrative.append("\n## Limitations\n")
            for limitation in self.limitations:
                narrative.append(f"- {limitation}\n")
        
        return "".join(narrative)


@dataclass
class AnalysisFramework:
    """
    Framework for conducting specific types of analysis.
    These are reusable templates for common analytical tasks.
    """
    framework_id: UUID = field(default_factory=uuid4)
    name: str = ""
    analysis_type: AnalysisType = AnalysisType.CUSTOM
    
    # Steps to follow
    steps: List[Dict[str, str]] = field(default_factory=list)
    # [{"step": "1", "action": "Gather all name variants", "output": "name_list"}]
    
    # Required evidence types
    required_evidence: List[str] = field(default_factory=list)
    
    # Checklist
    checklist: List[str] = field(default_factory=list)
    
    # Common pitfalls
    warnings: List[str] = field(default_factory=list)
    
    # Example
    example: Optional[str] = None
    
    def create_analysis(self, title: str) -> Analysis:
        """Create a new analysis using this framework"""
        analysis = Analysis(
            title=title,
            analysis_type=self.analysis_type
        )
        
        # Pre-populate with framework guidance
        analysis.argument_structure = [step["action"] for step in self.steps]
        
        return analysis