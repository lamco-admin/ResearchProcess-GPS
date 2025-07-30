"""
Location model for ResearchProcess-GPS.

Locations are temporal-aware, meaning they understand that places change
names, jurisdictions, and boundaries over time.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime, date
from uuid import UUID, uuid4
from enum import Enum

from .base import BaseEntity
from .confidence import ConfidenceContainer


class LocationType(Enum):
    """Types of locations"""
    # Administrative
    COUNTRY = "country"
    STATE = "state"
    PROVINCE = "province"
    COUNTY = "county"
    CITY = "city"
    TOWN = "town"
    VILLAGE = "village"
    TOWNSHIP = "township"
    DISTRICT = "district"
    
    # Geographic
    REGION = "region"
    AREA = "area"
    NEIGHBORHOOD = "neighborhood"
    
    # Specific places
    ADDRESS = "address"
    BUILDING = "building"
    CEMETERY = "cemetery"
    CHURCH = "church"
    HOSPITAL = "hospital"
    SCHOOL = "school"
    FARM = "farm"
    BUSINESS = "business"
    
    # Natural features
    RIVER = "river"
    LAKE = "lake"
    MOUNTAIN = "mountain"
    VALLEY = "valley"
    
    # Other
    UNKNOWN = "unknown"
    OTHER = "other"


class JurisdictionType(Enum):
    """Types of jurisdictions"""
    CIVIL = "civil"
    ECCLESIASTICAL = "ecclesiastical"
    MILITARY = "military"
    JUDICIAL = "judicial"
    POSTAL = "postal"
    CENSUS = "census"
    ELECTORAL = "electoral"
    TRADITIONAL = "traditional"  # Indigenous, tribal
    OTHER = "other"


class CoordinateSystem(Enum):
    """Coordinate systems for spatial data"""
    WGS84 = "WGS84"  # Standard GPS
    NAD83 = "NAD83"  # North American
    OSGB36 = "OSGB36"  # British National Grid
    ED50 = "ED50"  # European
    LOCAL = "local"  # Local survey
    HISTORICAL = "historical"  # Historical maps
    OTHER = "other"


@dataclass
class PlaceName:
    """A name for a place with temporal validity"""
    name_id: UUID = field(default_factory=uuid4)
    
    # The name
    name: str = ""
    language: str = "English"
    name_type: str = "official"  # "official", "colloquial", "historical", "indigenous"
    
    # When this name was used
    valid_from: Optional[date] = None
    valid_to: Optional[date] = None
    
    # Context
    cultural_group: str = ""  # Who used this name
    administrative_level: str = ""  # At what level was it official
    
    # Source
    source_evidence: List[UUID] = field(default_factory=list)
    confidence: float = 1.0
    
    def was_valid_on(self, check_date: date) -> bool:
        """Check if this name was valid on a given date"""
        if self.valid_from and check_date < self.valid_from:
            return False
        if self.valid_to and check_date > self.valid_to:
            return False
        return True


@dataclass
class Coordinates:
    """Geographic coordinates with precision"""
    latitude: float = 0.0
    longitude: float = 0.0
    altitude: Optional[float] = None
    
    # Precision
    precision_meters: float = 1.0  # How accurate are these coordinates
    coordinate_system: CoordinateSystem = CoordinateSystem.WGS84
    
    # Source
    source: str = ""  # "GPS", "map", "geocoded", "estimated"
    determination_date: Optional[date] = None
    
    def distance_to(self, other: 'Coordinates') -> float:
        """Calculate distance to another coordinate (simplified)"""
        # Simplified distance calculation
        # Real implementation would use proper geodesic calculations
        import math
        dlat = math.radians(other.latitude - self.latitude)
        dlon = math.radians(other.longitude - self.longitude)
        a = (math.sin(dlat/2)**2 + 
             math.cos(math.radians(self.latitude)) * 
             math.cos(math.radians(other.latitude)) * 
             math.sin(dlon/2)**2)
        c = 2 * math.asin(math.sqrt(a))
        return 6371000 * c  # Earth radius in meters


@dataclass
class Boundary:
    """Boundary definition with temporal validity"""
    boundary_id: UUID = field(default_factory=uuid4)
    
    # Boundary type
    boundary_type: str = ""  # "administrative", "property", "parish"
    
    # Definition (simplified - could be complex polygon)
    boundary_points: List[Coordinates] = field(default_factory=list)
    
    # When these boundaries were valid
    valid_from: Optional[date] = None
    valid_to: Optional[date] = None
    
    # Why it changed
    change_reason: str = ""  # "redistricting", "annexation", "survey correction"
    
    # Source
    source_evidence: List[UUID] = field(default_factory=list)


@dataclass
class Jurisdiction:
    """A jurisdiction that governs a location"""
    jurisdiction_id: UUID = field(default_factory=uuid4)
    
    # Jurisdiction info
    name: str = ""
    jurisdiction_type: JurisdictionType = JurisdictionType.CIVIL
    level: int = 0  # 0=country, 1=state, 2=county, etc.
    
    # Parent jurisdiction
    parent_jurisdiction: Optional[UUID] = None  # Reference to parent Location
    
    # Temporal validity
    valid_from: Optional[date] = None
    valid_to: Optional[date] = None
    
    # Governing body
    governing_body: str = ""
    legal_framework: str = ""
    
    def contains_date(self, check_date: date) -> bool:
        """Check if jurisdiction was active on date"""
        if self.valid_from and check_date < self.valid_from:
            return False
        if self.valid_to and check_date > self.valid_to:
            return False
        return True


@dataclass
class TemporalLocation:
    """A location at a specific point in time"""
    snapshot_id: UUID = field(default_factory=uuid4)
    
    # When
    snapshot_date: date = field(default_factory=date.today)
    
    # Names at this time
    active_names: List[PlaceName] = field(default_factory=list)
    primary_name: str = ""
    
    # Jurisdiction at this time
    jurisdictions: List[Jurisdiction] = field(default_factory=list)
    
    # Boundaries at this time
    boundaries: Optional[Boundary] = None
    
    # Population/demographics at this time (if known)
    population: Optional[int] = None
    demographic_notes: str = ""


@dataclass
class Location(BaseEntity):
    """
    A Location in ResearchProcess-GPS understands that places change over time.
    The same physical location might have different names, jurisdictions, and
    boundaries at different points in history.
    """
    
    def __post_init__(self):
        super().__post_init__()
        self.type = "Location"
    
    # Location classification
    location_type: LocationType = LocationType.UNKNOWN
    
    # Current/modern identification
    current_name: str = ""
    current_country: str = ""
    
    # All historical names
    place_names: List[PlaceName] = field(default_factory=list)
    
    # Geographic position (if known)
    coordinates: Optional[Coordinates] = None
    
    # All historical boundaries
    boundaries: List[Boundary] = field(default_factory=list)
    
    # Jurisdictional history
    jurisdictions: List[Jurisdiction] = field(default_factory=list)
    
    # Parent/child locations
    parent_location: Optional[UUID] = None  # e.g., city -> county
    child_locations: List[UUID] = field(default_factory=list)  # e.g., county -> cities
    
    # Temporal snapshots
    temporal_snapshots: Dict[date, TemporalLocation] = field(default_factory=dict)
    
    # Associated features
    nearby_features: List[Dict[str, Any]] = field(default_factory=list)
    # [{"type": "river", "name": "Smith River", "relationship": "eastern boundary"}]
    
    # Cultural associations
    cultural_significance: Dict[str, List[str]] = field(default_factory=dict)
    # {"Cherokee": ["ancestral homeland"], "European": ["colonial settlement"]}
    
    # Evidence
    evidence_links: List[UUID] = field(default_factory=list)
    
    # Research notes
    research_notes: List[str] = field(default_factory=list)
    
    def add_place_name(self, name: str, valid_from: Optional[date] = None,
                      valid_to: Optional[date] = None, language: str = "English") -> PlaceName:
        """Add a historical name for this location"""
        place_name = PlaceName(
            name=name,
            language=language,
            valid_from=valid_from,
            valid_to=valid_to
        )
        self.place_names.append(place_name)
        return place_name
    
    def get_name_on_date(self, check_date: date, language: str = None) -> Optional[str]:
        """Get the name of this location on a specific date"""
        valid_names = []
        
        for place_name in self.place_names:
            if place_name.was_valid_on(check_date):
                if language is None or place_name.language == language:
                    valid_names.append(place_name)
        
        if valid_names:
            # Prefer official names
            official = [n for n in valid_names if n.name_type == "official"]
            if official:
                return official[0].name
            return valid_names[0].name
        
        return self.current_name  # Fallback
    
    def add_jurisdiction(self, name: str, jurisdiction_type: JurisdictionType,
                        valid_from: Optional[date] = None,
                        valid_to: Optional[date] = None) -> Jurisdiction:
        """Add a jurisdiction that governed this location"""
        jurisdiction = Jurisdiction(
            name=name,
            jurisdiction_type=jurisdiction_type,
            valid_from=valid_from,
            valid_to=valid_to
        )
        self.jurisdictions.append(jurisdiction)
        return jurisdiction
    
    def get_jurisdictions_on_date(self, check_date: date, 
                                 jurisdiction_type: Optional[JurisdictionType] = None) -> List[Jurisdiction]:
        """Get all jurisdictions active on a specific date"""
        active = []
        
        for jurisdiction in self.jurisdictions:
            if jurisdiction.contains_date(check_date):
                if jurisdiction_type is None or jurisdiction.jurisdiction_type == jurisdiction_type:
                    active.append(jurisdiction)
        
        return active
    
    def get_temporal_snapshot(self, snapshot_date: date) -> TemporalLocation:
        """Get or create a temporal snapshot for a specific date"""
        if snapshot_date not in self.temporal_snapshots:
            # Create snapshot
            snapshot = TemporalLocation(snapshot_date=snapshot_date)
            
            # Find active names
            for name in self.place_names:
                if name.was_valid_on(snapshot_date):
                    snapshot.active_names.append(name)
            
            # Set primary name
            if snapshot.active_names:
                official = [n for n in snapshot.active_names if n.name_type == "official"]
                snapshot.primary_name = official[0].name if official else snapshot.active_names[0].name
            
            # Find active jurisdictions
            snapshot.jurisdictions = self.get_jurisdictions_on_date(snapshot_date)
            
            # Find active boundaries
            for boundary in self.boundaries:
                if boundary.valid_from and boundary.valid_to:
                    if boundary.valid_from <= snapshot_date <= boundary.valid_to:
                        snapshot.boundaries = boundary
                        break
            
            self.temporal_snapshots[snapshot_date] = snapshot
        
        return self.temporal_snapshots[snapshot_date]
    
    def merge_duplicate(self, other: 'Location') -> 'Location':
        """Merge another location that represents the same place"""
        # Merge names
        existing_names = {(n.name, n.language) for n in self.place_names}
        for name in other.place_names:
            if (name.name, name.language) not in existing_names:
                self.place_names.append(name)
        
        # Merge jurisdictions
        self.jurisdictions.extend(other.jurisdictions)
        
        # Merge boundaries
        self.boundaries.extend(other.boundaries)
        
        # Merge evidence
        self.evidence_links.extend(other.evidence_links)
        
        # Update coordinates if better precision
        if other.coordinates and (not self.coordinates or 
                                 other.coordinates.precision_meters < self.coordinates.precision_meters):
            self.coordinates = other.coordinates
        
        return self
    
    def standardize_name(self, name: str, date: Optional[date] = None) -> str:
        """
        Standardize a place name for this location.
        Useful for matching historical spellings to modern places.
        """
        # Simple implementation - real one would be more sophisticated
        normalized = name.lower().strip()
        
        # Check if it matches any historical names
        for place_name in self.place_names:
            if place_name.name.lower() == normalized:
                # Return the official/modern version
                return self.current_name or place_name.name
        
        # Check for common variations
        # TODO: Implement fuzzy matching, abbreviation expansion, etc.
        
        return name
    
    def format_for_date(self, check_date: date, include_jurisdiction: bool = True) -> str:
        """Format location name as it would have appeared on a specific date"""
        parts = []
        
        # Get name on date
        name = self.get_name_on_date(check_date)
        if name:
            parts.append(name)
        
        # Get jurisdictions if requested
        if include_jurisdiction:
            jurisdictions = self.get_jurisdictions_on_date(check_date, JurisdictionType.CIVIL)
            # Sort by level (local to national)
            jurisdictions.sort(key=lambda j: j.level)
            
            for jurisdiction in jurisdictions:
                if jurisdiction.name and jurisdiction.name not in parts:
                    parts.append(jurisdiction.name)
        
        return ", ".join(parts)