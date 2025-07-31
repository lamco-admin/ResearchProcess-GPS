"""
Researcher entity for ResearchProcess-GPS.

Represents individuals, teams, or organizations conducting genealogical research.
Tracks attribution, permissions, and research activities.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set
from datetime import datetime
from uuid import UUID, uuid4
from enum import Enum

from .base import NestableBaseEntity


class ResearcherType(Enum):
    """Types of researchers"""
    INDIVIDUAL = "individual"
    TEAM = "team"
    ORGANIZATION = "organization"
    SYSTEM = "system"  # For automated processes


class ResearcherState(Enum):
    """States for researcher lifecycle"""
    ACTIVE = "active"
    INACTIVE = "inactive"
    RETIRED = "retired"
    SUSPENDED = "suspended"


class ResearcherRole(Enum):
    """Roles a researcher can have"""
    LEAD_RESEARCHER = "lead_researcher"
    RESEARCHER = "researcher"
    REVIEWER = "reviewer"
    CONTRIBUTOR = "contributor"
    OBSERVER = "observer"
    ADMINISTRATOR = "administrator"


class PermissionLevel(Enum):
    """Permission levels for actions"""
    DENIED = "denied"
    READ = "read"
    CREATE = "create"
    MODIFY = "modify"
    DELETE = "delete"
    ADMIN = "admin"


@dataclass
class Credential:
    """Professional credential"""
    credential_type: str  # "CG", "AG", "CGSM", "PhD"
    credential_number: Optional[str] = None
    issuing_authority: str = ""
    issue_date: Optional[datetime] = None
    expiry_date: Optional[datetime] = None
    status: str = "active"  # "active", "expired", "revoked"


@dataclass
class Address:
    """Physical or mailing address"""
    address_type: str = "mailing"  # "physical", "mailing", "billing"
    street_lines: List[str] = field(default_factory=list)
    city: str = ""
    state_province: str = ""
    postal_code: str = ""
    country: str = ""
    
    # Temporal aspect
    valid_from: Optional[datetime] = None
    valid_to: Optional[datetime] = None


@dataclass
class ResearchMetrics:
    """Track researcher's activities and contributions"""
    theories_created: int = 0
    theories_concluded: int = 0
    evidence_items_discovered: int = 0
    evidence_items_analyzed: int = 0
    identities_created: int = 0
    persons_concluded: int = 0
    assessments_performed: int = 0
    peer_reviews_conducted: int = 0
    work_products_created: int = 0
    
    # Time tracking
    total_research_hours: float = 0.0
    hours_by_year: Dict[int, float] = field(default_factory=dict)
    
    # Quality metrics
    gps_compliance_rate: float = 0.0
    peer_review_approval_rate: float = 0.0
    
    # Last activity
    last_activity_date: Optional[datetime] = None
    last_activity_type: Optional[str] = None


@dataclass
class Researcher(NestableBaseEntity['Researcher']):
    """
    Person or organization conducting research.
    Can nest for teams, departments, or hierarchical organizations.
    Similar to GEDCOM X Agent but focused on research attribution.
    """
    
    # Identity
    name: str
    researcher_type: ResearcherType = ResearcherType.INDIVIDUAL
    
    # Professional credentials
    credentials: List[Credential] = field(default_factory=list)
    
    # Contact information
    emails: List[str] = field(default_factory=list)
    phones: List[str] = field(default_factory=list)
    addresses: List[Address] = field(default_factory=list)
    
    # External identifiers
    identifiers: Dict[str, str] = field(default_factory=dict)
    # {
    #   "ORCID": "0000-0000-0000-0000",
    #   "FamilySearch": "XXXX-XXX",
    #   "Ancestry": "username",
    #   "WikiTree": "Smith-12345"
    # }
    
    # Organizational structure (nesting)
    parent_researcher: Optional[UUID] = None  # For team members
    team_members: List[UUID] = field(default_factory=list)  # For teams
    department: Optional[str] = None
    organization: Optional[str] = None
    
    # Roles and permissions
    roles: List[ResearcherRole] = field(default_factory=list)
    
    # Granular permissions
    permissions: Dict[str, PermissionLevel] = field(default_factory=dict)
    # {
    #   "Theory.create": PermissionLevel.CREATE,
    #   "Person.delete": PermissionLevel.DENIED,
    #   "Evidence.modify": PermissionLevel.MODIFY
    # }
    
    # Default permissions for entity types
    default_permissions: Dict[str, PermissionLevel] = field(default_factory=dict)
    # {
    #   "Theory": PermissionLevel.CREATE,
    #   "Evidence": PermissionLevel.CREATE,
    #   "Person": PermissionLevel.READ
    # }
    
    # Expertise areas
    specializations: List[str] = field(default_factory=list)
    # ["Irish genealogy", "DNA analysis", "18th century paleography"]
    
    geographic_expertise: List[str] = field(default_factory=list)
    # ["Ireland", "New England", "Virginia"]
    
    temporal_expertise: List[str] = field(default_factory=list)
    # ["1700-1800", "Colonial America", "Irish Famine era"]
    
    language_skills: List[str] = field(default_factory=list)
    # ["English", "German:read", "Latin:read"]
    
    # Research interests
    research_interests: List[str] = field(default_factory=list)
    current_projects: List[str] = field(default_factory=list)
    
    # Activity tracking
    theories_owned: List[UUID] = field(default_factory=list)
    active_research_logs: List[UUID] = field(default_factory=list)
    
    # Metrics
    metrics: ResearchMetrics = field(default_factory=ResearchMetrics)
    
    # State tracking
    state: ResearcherState = ResearcherState.ACTIVE
    state_reason: Optional[str] = None  # Why inactive/suspended
    
    # Professional information
    bio: str = ""
    website: Optional[str] = None
    social_media: Dict[str, str] = field(default_factory=dict)
    # {"Twitter": "@researcher", "LinkedIn": "profile-url"}
    
    # Preferences
    preferences: Dict[str, Any] = field(default_factory=dict)
    # {
    #   "default_methodology": "GPS-2025",
    #   "citation_style": "Evidence_Explained",
    #   "date_format": "ISO-8601"
    # }
    
    # Privacy settings
    privacy_settings: Dict[str, bool] = field(default_factory=dict)
    # {
    #   "show_email": False,
    #   "show_real_name": True,
    #   "show_credentials": True
    # }
    
    def add_credential(self, credential: Credential) -> None:
        """Add a professional credential"""
        self.credentials.append(credential)
        self.updated_at = datetime.utcnow()
    
    def add_team_member(self, member_id: UUID) -> None:
        """Add a team member (for team/organization researchers)"""
        if self.researcher_type not in [ResearcherType.TEAM, ResearcherType.ORGANIZATION]:
            raise ValueError("Only team/organization researchers can have members")
        
        if member_id not in self.team_members:
            self.team_members.append(member_id)
            self.updated_at = datetime.utcnow()
    
    def check_permission(self, entity_type: str, action: str) -> PermissionLevel:
        """Check permission for a specific action"""
        # Check specific permission first
        specific_key = f"{entity_type}.{action}"
        if specific_key in self.permissions:
            return self.permissions[specific_key]
        
        # Check default permission for entity type
        if entity_type in self.default_permissions:
            return self.default_permissions[entity_type]
        
        # Default based on role
        if ResearcherRole.ADMINISTRATOR in self.roles:
            return PermissionLevel.ADMIN
        elif ResearcherRole.LEAD_RESEARCHER in self.roles:
            return PermissionLevel.MODIFY
        elif ResearcherRole.RESEARCHER in self.roles:
            return PermissionLevel.CREATE
        elif ResearcherRole.REVIEWER in self.roles:
            return PermissionLevel.READ
        else:
            return PermissionLevel.DENIED
    
    def update_metrics(self, activity_type: str, **kwargs) -> None:
        """Update researcher metrics based on activity"""
        if activity_type == "theory_created":
            self.metrics.theories_created += 1
        elif activity_type == "evidence_discovered":
            self.metrics.evidence_items_discovered += 1
        elif activity_type == "assessment_performed":
            self.metrics.assessments_performed += 1
        # ... other activity types
        
        self.metrics.last_activity_date = datetime.utcnow()
        self.metrics.last_activity_type = activity_type
        
        # Update hours if provided
        if 'hours' in kwargs:
            self.metrics.total_research_hours += kwargs['hours']
            year = datetime.utcnow().year
            if year not in self.metrics.hours_by_year:
                self.metrics.hours_by_year[year] = 0
            self.metrics.hours_by_year[year] += kwargs['hours']
    
    def get_display_name(self) -> str:
        """Get display name based on privacy settings"""
        if self.privacy_settings.get('show_real_name', True):
            return self.name
        elif self.identifiers:
            # Use first available identifier
            for system, identifier in self.identifiers.items():
                return f"{system}:{identifier}"
        return f"Researcher {str(self.id)[:8]}"
    
    def get_credentials_display(self) -> str:
        """Get formatted credentials for display"""
        if not self.privacy_settings.get('show_credentials', True):
            return ""
        
        active_creds = [c for c in self.credentials if c.status == "active"]
        if active_creds:
            return ", ".join(c.credential_type for c in active_creds)
        return ""
    
    def to_attribution_dict(self) -> Dict[str, Any]:
        """Convert to attribution dictionary for display"""
        return {
            'id': self.id,
            'name': self.get_display_name(),
            'credentials': self.get_credentials_display(),
            'type': self.researcher_type.value,
            'roles': [r.value for r in self.roles]
        }