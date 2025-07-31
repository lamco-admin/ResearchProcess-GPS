"""
Spatial abstraction layer for ResearchProcess-GPS.

Handles multiple coordinate systems, addressing formats, and geographic
representations across different cultures and time periods.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple, Union
from datetime import date
from enum import Enum
from uuid import UUID, uuid4


class SpatialSystemType(Enum):
    """Types of spatial reference systems"""
    COORDINATE = "coordinate"          # Lat/long, grid systems
    ADDRESS = "address"                # Street addresses
    DESCRIPTIVE = "descriptive"        # Narrative descriptions
    CADASTRAL = "cadastral"            # Land survey systems
    TRADITIONAL = "traditional"        # Indigenous/traditional systems
    RELATIVE = "relative"              # Relative to landmarks
    ADMINISTRATIVE = "administrative"  # Political divisions
    ECCLESIASTICAL = "ecclesiastical"  # Church jurisdictions
    MILITARY = "military"              # Military districts
    POSTAL = "postal"                  # Postal codes/zones


class CoordinateSystemType(Enum):
    """Specific coordinate systems"""
    WGS84 = "WGS84"                   # GPS standard
    NAD83 = "NAD83"                   # North American Datum
    NAD27 = "NAD27"                   # Older North American
    OSGB36 = "OSGB36"                 # British National Grid
    ED50 = "ED50"                     # European Datum
    GDA94 = "GDA94"                   # Australian
    NZGD2000 = "NZGD2000"            # New Zealand
    JGD2011 = "JGD2011"              # Japanese
    DHDN = "DHDN"                     # German
    RD = "RD"                         # Dutch
    SWEREF99 = "SWEREF99"            # Swedish
    LOCAL_GRID = "LOCAL_GRID"         # Local survey grid
    HISTORICAL = "HISTORICAL"         # From historical maps
    CUSTOM = "CUSTOM"


class AddressSystemType(Enum):
    """Types of addressing systems"""
    STREET = "street"                  # Modern street addresses
    RURAL_ROUTE = "rural_route"       # Rural delivery routes
    POST_OFFICE_BOX = "po_box"        # PO boxes
    MILITARY_APO = "military_apo"      # Military addresses
    TRADITIONAL = "traditional"        # Pre-modern addressing
    DESCRIPTIVE = "descriptive"        # "Third house past church"
    GRID = "grid"                      # Grid-based (some US cities)
    JAPANESE = "japanese"              # Block-based system
    KOREAN = "korean"                  # Dong/Gu system
    CUSTOM = "custom"


class CadastralSystemType(Enum):
    """Land survey and property systems"""
    PLSS = "PLSS"                     # US Public Land Survey System
    METES_BOUNDS = "metes_bounds"     # Metes and bounds
    LOT_CONCESSION = "lot_concession" # Canadian system
    SEIGNEURIAL = "seigneurial"       # French colonial
    SPANISH_COLONIAL = "spanish_colonial"
    TORRENS = "torrens"               # Torrens title system
    DOOMSDAY = "doomsday"            # English historical
    CUSTOM = "custom"


@dataclass
class SpatialPrecision:
    """Precision/accuracy of spatial data"""
    precision_meters: float = 1.0      # Spatial precision
    confidence: float = 1.0            # Confidence in location
    determination_method: str = ""     # How location was determined
    notes: str = ""


@dataclass
class CoordinatePoint:
    """A point in a specific coordinate system"""
    system: CoordinateSystemType
    
    # Primary coordinates
    x: float = 0.0                     # Longitude or easting
    y: float = 0.0                     # Latitude or northing
    z: Optional[float] = None          # Elevation/altitude
    
    # Additional parameters for some systems
    zone: Optional[str] = None         # UTM zone, state plane zone
    hemisphere: Optional[str] = None   # N/S, E/W
    
    # Original representation
    original_format: str = ""          # How it was originally written
    
    # Precision
    precision: Optional[SpatialPrecision] = None
    
    def to_wgs84(self) -> Optional['CoordinatePoint']:
        """Convert to WGS84 (GPS) coordinates"""
        # Would need coordinate transformation library
        # For now, return None
        return None


@dataclass
class AddressComponents:
    """Components of a street address"""
    # Building/location
    building_number: Optional[str] = None
    building_name: Optional[str] = None
    unit_number: Optional[str] = None
    
    # Street
    street_name: Optional[str] = None
    street_type: Optional[str] = None  # Road, Street, Avenue
    street_direction: Optional[str] = None  # N, S, E, W
    
    # Area
    neighborhood: Optional[str] = None
    district: Optional[str] = None
    
    # City/Town
    city: Optional[str] = None
    
    # Larger divisions
    county: Optional[str] = None
    state_province: Optional[str] = None
    
    # Country
    country: Optional[str] = None
    
    # Postal
    postal_code: Optional[str] = None
    
    # Full formatted address
    formatted_address: str = ""
    
    # Address system
    system_type: AddressSystemType = AddressSystemType.STREET


@dataclass
class CadastralDescription:
    """Land survey description"""
    system: CadastralSystemType
    
    # PLSS (US Public Land Survey)
    township: Optional[str] = None
    range: Optional[str] = None
    section: Optional[int] = None
    quarter_section: Optional[str] = None
    
    # Metes and bounds
    metes_bounds_description: Optional[str] = None
    
    # Lot and concession (Canadian)
    lot: Optional[str] = None
    concession: Optional[str] = None
    
    # General
    parcel_id: Optional[str] = None
    survey_date: Optional[date] = None
    surveyor: Optional[str] = None
    
    # Original text
    original_description: str = ""


@dataclass
class TraditionalLocation:
    """Traditional/indigenous location description"""
    cultural_group: str = ""           # Which culture's system
    
    # Traditional place name
    place_name: str = ""
    place_meaning: Optional[str] = None
    
    # Relationship to landmarks
    landmarks: List[str] = field(default_factory=list)
    directions: List[str] = field(default_factory=list)
    
    # Seasonal variations
    seasonal_location: bool = False
    season_descriptions: Dict[str, str] = field(default_factory=dict)
    
    # Territory/region
    territory_name: Optional[str] = None
    
    # Sacred/cultural significance
    cultural_significance: Optional[str] = None
    access_restrictions: Optional[str] = None


class SpatialSystem(ABC):
    """Abstract base for spatial systems"""
    
    @abstractmethod
    def get_type(self) -> SpatialSystemType:
        """Get spatial system type"""
        pass
    
    @abstractmethod
    def parse_location(self, location_string: str, 
                      context: Dict[str, Any] = None) -> Any:
        """Parse a location string in this system"""
        pass
    
    @abstractmethod
    def format_location(self, location_data: Any, 
                       style: str = "full") -> str:
        """Format location data as string"""
        pass
    
    @abstractmethod
    def validate_location(self, location_data: Any) -> List[str]:
        """Validate location data, return issues"""
        pass
    
    @abstractmethod
    def get_bounding_box(self, location_data: Any) -> Optional[Tuple[float, float, float, float]]:
        """Get bounding box as (min_lon, min_lat, max_lon, max_lat)"""
        pass


class CoordinateSystem(SpatialSystem):
    """Coordinate-based spatial system"""
    
    def __init__(self, system_type: CoordinateSystemType):
        self.system_type = system_type
    
    def get_type(self) -> SpatialSystemType:
        return SpatialSystemType.COORDINATE
    
    def parse_location(self, location_string: str, 
                      context: Dict[str, Any] = None) -> CoordinatePoint:
        """Parse coordinate string"""
        # Simple parsing - real implementation would be more sophisticated
        parts = location_string.replace(',', ' ').split()
        
        if len(parts) >= 2:
            try:
                x = float(parts[0])
                y = float(parts[1])
                z = float(parts[2]) if len(parts) > 2 else None
                
                return CoordinatePoint(
                    system=self.system_type,
                    x=x, y=y, z=z,
                    original_format=location_string
                )
            except ValueError:
                pass
        
        return CoordinatePoint(system=self.system_type)
    
    def format_location(self, location_data: CoordinatePoint, 
                       style: str = "full") -> str:
        """Format coordinates"""
        if style == "decimal":
            if location_data.z:
                return f"{location_data.x:.6f}, {location_data.y:.6f}, {location_data.z:.1f}m"
            return f"{location_data.x:.6f}, {location_data.y:.6f}"
        
        elif style == "dms":  # Degrees Minutes Seconds
            # Convert decimal to DMS
            def decimal_to_dms(decimal: float, is_latitude: bool) -> str:
                direction = ""
                if is_latitude:
                    direction = "N" if decimal >= 0 else "S"
                else:
                    direction = "E" if decimal >= 0 else "W"
                
                decimal = abs(decimal)
                degrees = int(decimal)
                minutes = int((decimal - degrees) * 60)
                seconds = ((decimal - degrees) * 60 - minutes) * 60
                
                return f"{degrees}°{minutes}'{seconds:.1f}\"{direction}"
            
            lat = decimal_to_dms(location_data.y, True)
            lon = decimal_to_dms(location_data.x, False)
            return f"{lat} {lon}"
        
        return self.format_location(location_data, "decimal")
    
    def validate_location(self, location_data: CoordinatePoint) -> List[str]:
        """Validate coordinates"""
        issues = []
        
        if self.system_type == CoordinateSystemType.WGS84:
            if not -180 <= location_data.x <= 180:
                issues.append("Longitude must be between -180 and 180")
            if not -90 <= location_data.y <= 90:
                issues.append("Latitude must be between -90 and 90")
        
        return issues
    
    def get_bounding_box(self, location_data: CoordinatePoint) -> Optional[Tuple[float, float, float, float]]:
        """Get bounding box for point"""
        # Account for precision
        precision = location_data.precision.precision_meters if location_data.precision else 10
        
        # Rough conversion (1 degree ≈ 111km at equator)
        degree_offset = precision / 111000
        
        return (
            location_data.x - degree_offset,
            location_data.y - degree_offset,
            location_data.x + degree_offset,
            location_data.y + degree_offset
        )


@dataclass
class SpatialReference:
    """A complete spatial reference combining multiple systems"""
    reference_id: UUID = field(default_factory=uuid4)
    
    # Primary location (best available)
    primary_system: SpatialSystemType = SpatialSystemType.DESCRIPTIVE
    
    # Different representations
    coordinate: Optional[CoordinatePoint] = None
    address: Optional[AddressComponents] = None
    cadastral: Optional[CadastralDescription] = None
    traditional: Optional[TraditionalLocation] = None
    descriptive: Optional[str] = None
    
    # Historical names/descriptions
    historical_descriptions: List[Tuple[date, str]] = field(default_factory=list)
    
    # Precision and confidence
    precision: Optional[SpatialPrecision] = None
    
    # Which representation to prefer
    preferred_representation: str = "coordinate"  # or "address", "descriptive"
    
    # Notes
    location_notes: str = ""
    
    def add_coordinate(self, coord: CoordinatePoint) -> None:
        """Add coordinate representation"""
        self.coordinate = coord
        if self.primary_system == SpatialSystemType.DESCRIPTIVE:
            self.primary_system = SpatialSystemType.COORDINATE
    
    def add_address(self, addr: AddressComponents) -> None:
        """Add address representation"""
        self.address = addr
        if self.primary_system == SpatialSystemType.DESCRIPTIVE:
            self.primary_system = SpatialSystemType.ADDRESS
    
    def get_display_location(self, prefer_system: Optional[SpatialSystemType] = None) -> str:
        """Get location for display"""
        if prefer_system == SpatialSystemType.COORDINATE and self.coordinate:
            return CoordinateSystem(self.coordinate.system).format_location(self.coordinate)
        
        if prefer_system == SpatialSystemType.ADDRESS and self.address:
            return self.address.formatted_address
        
        # Return whatever we have
        if self.address and self.address.formatted_address:
            return self.address.formatted_address
        
        if self.coordinate:
            return CoordinateSystem(self.coordinate.system).format_location(self.coordinate)
        
        if self.descriptive:
            return self.descriptive
        
        return "Location unknown"
    
    def merge_with(self, other: 'SpatialReference') -> 'SpatialReference':
        """Merge another spatial reference"""
        # Take best precision
        if other.coordinate and (not self.coordinate or 
                                 (other.precision and self.precision and 
                                  other.precision.precision_meters < self.precision.precision_meters)):
            self.coordinate = other.coordinate
        
        if other.address and not self.address:
            self.address = other.address
        
        if other.cadastral and not self.cadastral:
            self.cadastral = other.cadastral
        
        # Merge historical descriptions
        self.historical_descriptions.extend(other.historical_descriptions)
        
        return self


class SpatialAnalyzer:
    """Analyzes and correlates spatial data"""
    
    def calculate_distance(self, ref1: SpatialReference, ref2: SpatialReference) -> Optional[float]:
        """Calculate distance between two locations in meters"""
        if ref1.coordinate and ref2.coordinate:
            # Convert to WGS84 if needed
            coord1 = ref1.coordinate.to_wgs84() or ref1.coordinate
            coord2 = ref2.coordinate.to_wgs84() or ref2.coordinate
            
            if coord1.system == CoordinateSystemType.WGS84 and coord2.system == CoordinateSystemType.WGS84:
                # Haversine formula
                import math
                R = 6371000  # Earth radius in meters
                
                lat1, lon1 = math.radians(coord1.y), math.radians(coord1.x)
                lat2, lon2 = math.radians(coord2.y), math.radians(coord2.x)
                
                dlat = lat2 - lat1
                dlon = lon2 - lon1
                
                a = math.sin(dlat/2)**2 + math.cos(lat1) * math.cos(lat2) * math.sin(dlon/2)**2
                c = 2 * math.asin(math.sqrt(a))
                
                return R * c
        
        return None
    
    def locations_could_be_same(self, ref1: SpatialReference, ref2: SpatialReference,
                               max_distance_meters: float = 1000) -> bool:
        """Check if two locations could be the same place"""
        # Try distance
        distance = self.calculate_distance(ref1, ref2)
        if distance is not None:
            return distance <= max_distance_meters
        
        # Try address matching
        if ref1.address and ref2.address:
            # Simple comparison - real implementation would be sophisticated
            if (ref1.address.street_name == ref2.address.street_name and
                ref1.address.city == ref2.address.city):
                return True
        
        # Try descriptive matching
        if ref1.descriptive and ref2.descriptive:
            # Would need fuzzy matching
            if ref1.descriptive.lower() in ref2.descriptive.lower() or vice_versa:
                return True
        
        return False