"""
Naming system abstraction layer for ResearchProcess-GPS.

Handles different cultural naming systems with their unique structures,
rules, and conventions.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import date
from enum import Enum
from uuid import UUID, uuid4


class NamingSystemType(Enum):
    """Types of naming systems"""
    WESTERN = "western"
    PATRONYMIC = "patronymic"
    ISLAMIC = "islamic"
    CHINESE = "chinese"
    JAPANESE = "japanese"
    KOREAN = "korean"
    SPANISH = "spanish"
    PORTUGUESE = "portuguese"
    ICELANDIC = "icelandic"
    RUSSIAN = "russian"
    INDIAN = "indian"
    AFRICAN = "african"
    INDIGENOUS = "indigenous"
    CUSTOM = "custom"


class NameComponentType(Enum):
    """Types of name components across cultures"""
    # Universal
    GIVEN = "given"              # Personal name
    FAMILY = "family"            # Family/clan name
    
    # Western
    MIDDLE = "middle"            # Middle name(s)
    SURNAME = "surname"          # Last name
    SUFFIX = "suffix"            # Jr, III, etc.
    PREFIX = "prefix"            # Mr, Dr, etc.
    
    # Patronymic
    PATRONYM = "patronym"        # Father's name derivative
    MATRONYM = "matronym"        # Mother's name derivative
    
    # Islamic
    ISM = "ism"                  # Given name
    NASAB = "nasab"              # Lineage (ibn/bint + father)
    LAQAB = "laqab"              # Epithet/title
    NISBA = "nisba"              # Origin/tribe/profession
    KUNYA = "kunya"              # Teknonym (Abu/Umm + child)
    
    # Chinese/Japanese/Korean
    GENERATION = "generation"     # Generation name
    COURTESY = "courtesy"        # Courtesy/art name
    
    # Spanish/Portuguese
    PATERNAL_SURNAME = "paternal_surname"
    MATERNAL_SURNAME = "maternal_surname"
    
    # Other
    CLAN = "clan"                # Clan name
    TRIBE = "tribe"              # Tribal name
    CASTE = "caste"              # Caste identifier
    NICKNAME = "nickname"        # Informal name
    ALIAS = "alias"              # Alternative name
    TITLE = "title"              # Noble/professional title
    RELIGIOUS = "religious"      # Religious name


@dataclass
class NameComponent:
    """A single component of a name"""
    component_type: NameComponentType
    value: str
    
    # Optional metadata
    language: str = ""
    transliteration: str = ""  # If original is in different script
    meaning: str = ""
    
    # Context
    usage_context: str = ""  # "formal", "informal", "legal", "religious"
    cultural_notes: str = ""


@dataclass
class NameStructure:
    """Defines how names are structured in a naming system"""
    # Required components
    required_components: List[NameComponentType] = field(default_factory=list)
    
    # Optional components
    optional_components: List[NameComponentType] = field(default_factory=list)
    
    # Display order
    display_order: List[NameComponentType] = field(default_factory=list)
    
    # Sorting rules
    sort_by_component: NameComponentType = NameComponentType.FAMILY
    
    # Special rules
    rules: Dict[str, Any] = field(default_factory=dict)


class NamingSystem(ABC):
    """Abstract base for naming systems"""
    
    @abstractmethod
    def get_type(self) -> NamingSystemType:
        """Get naming system type"""
        pass
    
    @abstractmethod
    def get_structure(self) -> NameStructure:
        """Get the structure of names in this system"""
        pass
    
    @abstractmethod
    def parse_name(self, full_name: str, context: Dict[str, Any] = None) -> List[NameComponent]:
        """Parse a full name into components"""
        pass
    
    @abstractmethod
    def format_name(self, components: List[NameComponent], 
                   style: str = "full") -> str:
        """Format name components into a string"""
        pass
    
    @abstractmethod
    def generate_variants(self, components: List[NameComponent]) -> List[str]:
        """Generate possible name variants"""
        pass
    
    @abstractmethod
    def validate_name(self, components: List[NameComponent]) -> List[str]:
        """Validate name structure, return issues"""
        pass
    
    @abstractmethod
    def get_sort_key(self, components: List[NameComponent]) -> str:
        """Get sort key for alphabetizing"""
        pass


class WesternNamingSystem(NamingSystem):
    """Western naming system implementation"""
    
    def get_type(self) -> NamingSystemType:
        return NamingSystemType.WESTERN
    
    def get_structure(self) -> NameStructure:
        return NameStructure(
            required_components=[NameComponentType.GIVEN, NameComponentType.SURNAME],
            optional_components=[
                NameComponentType.MIDDLE, 
                NameComponentType.PREFIX,
                NameComponentType.SUFFIX,
                NameComponentType.NICKNAME
            ],
            display_order=[
                NameComponentType.PREFIX,
                NameComponentType.GIVEN,
                NameComponentType.MIDDLE,
                NameComponentType.SURNAME,
                NameComponentType.SUFFIX
            ],
            sort_by_component=NameComponentType.SURNAME
        )
    
    def parse_name(self, full_name: str, context: Dict[str, Any] = None) -> List[NameComponent]:
        """Parse Western name"""
        components = []
        
        # Simple parsing - real implementation would be more sophisticated
        parts = full_name.strip().split()
        
        if parts:
            # Check for prefix
            if parts[0].lower() in ['mr', 'mrs', 'ms', 'dr', 'prof']:
                components.append(NameComponent(NameComponentType.PREFIX, parts[0]))
                parts = parts[1:]
            
            # Check for suffix
            if parts and parts[-1].lower() in ['jr', 'sr', 'ii', 'iii', 'iv', 'phd', 'md']:
                components.append(NameComponent(NameComponentType.SUFFIX, parts[-1]))
                parts = parts[:-1]
            
            # Remaining parts
            if parts:
                components.append(NameComponent(NameComponentType.GIVEN, parts[0]))
                if len(parts) > 2:
                    for middle in parts[1:-1]:
                        components.append(NameComponent(NameComponentType.MIDDLE, middle))
                if len(parts) > 1:
                    components.append(NameComponent(NameComponentType.SURNAME, parts[-1]))
        
        return components
    
    def format_name(self, components: List[NameComponent], style: str = "full") -> str:
        """Format Western name"""
        if style == "full":
            ordered = []
            order = self.get_structure().display_order
            
            for comp_type in order:
                for comp in components:
                    if comp.component_type == comp_type:
                        ordered.append(comp.value)
            
            return " ".join(ordered)
        
        elif style == "formal":
            # Last, First Middle
            surname = next((c.value for c in components if c.component_type == NameComponentType.SURNAME), "")
            given = next((c.value for c in components if c.component_type == NameComponentType.GIVEN), "")
            middle = " ".join(c.value for c in components if c.component_type == NameComponentType.MIDDLE)
            
            parts = [surname]
            if given:
                parts.extend([",", given])
            if middle:
                parts.append(middle)
            
            return " ".join(parts)
        
        return self.format_name(components, "full")
    
    def generate_variants(self, components: List[NameComponent]) -> List[str]:
        """Generate Western name variants"""
        variants = []
        
        # Full name
        variants.append(self.format_name(components, "full"))
        
        # Formal (Last, First)
        variants.append(self.format_name(components, "formal"))
        
        # First Last (no middle)
        given = next((c.value for c in components if c.component_type == NameComponentType.GIVEN), "")
        surname = next((c.value for c in components if c.component_type == NameComponentType.SURNAME), "")
        if given and surname:
            variants.append(f"{given} {surname}")
        
        # With initials
        middles = [c.value for c in components if c.component_type == NameComponentType.MIDDLE]
        if middles and given and surname:
            initials = " ".join(m[0] + "." for m in middles)
            variants.append(f"{given} {initials} {surname}")
        
        return list(set(variants))  # Remove duplicates
    
    def validate_name(self, components: List[NameComponent]) -> List[str]:
        """Validate Western name"""
        issues = []
        
        # Check required components
        has_given = any(c.component_type == NameComponentType.GIVEN for c in components)
        has_surname = any(c.component_type == NameComponentType.SURNAME for c in components)
        
        if not has_given:
            issues.append("Western name requires a given name")
        if not has_surname:
            issues.append("Western name requires a surname")
        
        return issues
    
    def get_sort_key(self, components: List[NameComponent]) -> str:
        """Get sort key for Western name"""
        surname = next((c.value for c in components if c.component_type == NameComponentType.SURNAME), "")
        given = next((c.value for c in components if c.component_type == NameComponentType.GIVEN), "")
        return f"{surname}, {given}".lower()


class IslamicNamingSystem(NamingSystem):
    """Islamic naming system implementation"""
    
    def get_type(self) -> NamingSystemType:
        return NamingSystemType.ISLAMIC
    
    def get_structure(self) -> NameStructure:
        return NameStructure(
            required_components=[NameComponentType.ISM],
            optional_components=[
                NameComponentType.NASAB,
                NameComponentType.LAQAB,
                NameComponentType.NISBA,
                NameComponentType.KUNYA
            ],
            display_order=[
                NameComponentType.KUNYA,
                NameComponentType.ISM,
                NameComponentType.NASAB,
                NameComponentType.LAQAB,
                NameComponentType.NISBA
            ],
            sort_by_component=NameComponentType.ISM,
            rules={
                "nasab_prefix_male": "ibn",  # son of
                "nasab_prefix_female": "bint",  # daughter of
                "kunya_prefix_male": "Abu",  # father of
                "kunya_prefix_female": "Umm"  # mother of
            }
        )
    
    def parse_name(self, full_name: str, context: Dict[str, Any] = None) -> List[NameComponent]:
        """Parse Islamic name"""
        components = []
        parts = full_name.split()
        
        i = 0
        while i < len(parts):
            part = parts[i]
            
            # Check for Kunya
            if part in ["Abu", "Umm"] and i + 1 < len(parts):
                components.append(NameComponent(
                    NameComponentType.KUNYA,
                    f"{part} {parts[i + 1]}"
                ))
                i += 2
                continue
            
            # Check for Nasab
            elif part in ["ibn", "bin", "bint", "binti"] and i + 1 < len(parts):
                # Collect the full nasab chain
                nasab_parts = [part, parts[i + 1]]
                j = i + 2
                while j < len(parts) and parts[j] in ["ibn", "bin", "bint", "binti"]:
                    if j + 1 < len(parts):
                        nasab_parts.extend([parts[j], parts[j + 1]])
                        j += 2
                    else:
                        break
                
                components.append(NameComponent(
                    NameComponentType.NASAB,
                    " ".join(nasab_parts)
                ))
                i = j
                continue
            
            # Check for Nisba (starts with al-)
            elif part.startswith("al-") or part.startswith("Al-"):
                components.append(NameComponent(NameComponentType.NISBA, part))
                i += 1
                continue
            
            # Otherwise it's likely an Ism
            else:
                components.append(NameComponent(NameComponentType.ISM, part))
                i += 1
        
        return components
    
    def format_name(self, components: List[NameComponent], style: str = "full") -> str:
        """Format Islamic name"""
        if style == "full":
            ordered = []
            for comp_type in self.get_structure().display_order:
                for comp in components:
                    if comp.component_type == comp_type:
                        ordered.append(comp.value)
            return " ".join(ordered)
        
        elif style == "common":
            # Just Ism and Nisba
            ism = next((c.value for c in components if c.component_type == NameComponentType.ISM), "")
            nisba = next((c.value for c in components if c.component_type == NameComponentType.NISBA), "")
            return f"{ism} {nisba}".strip()
        
        return self.format_name(components, "full")
    
    def generate_variants(self, components: List[NameComponent]) -> List[str]:
        """Generate Islamic name variants"""
        variants = []
        
        # Full name
        variants.append(self.format_name(components, "full"))
        
        # Common name
        variants.append(self.format_name(components, "common"))
        
        # Just Ism
        ism = next((c.value for c in components if c.component_type == NameComponentType.ISM), "")
        if ism:
            variants.append(ism)
        
        # Kunya + Ism
        kunya = next((c.value for c in components if c.component_type == NameComponentType.KUNYA), "")
        if kunya and ism:
            variants.append(f"{kunya} {ism}")
        
        return list(set(variants))
    
    def validate_name(self, components: List[NameComponent]) -> List[str]:
        """Validate Islamic name"""
        issues = []
        
        # Must have an Ism
        has_ism = any(c.component_type == NameComponentType.ISM for c in components)
        if not has_ism:
            issues.append("Islamic name requires an Ism (given name)")
        
        return issues
    
    def get_sort_key(self, components: List[NameComponent]) -> str:
        """Get sort key for Islamic name"""
        ism = next((c.value for c in components if c.component_type == NameComponentType.ISM), "")
        return ism.lower()


@dataclass
class CulturalName:
    """A culturally-aware name representation"""
    name_id: UUID = field(default_factory=uuid4)
    
    # Naming system
    naming_system: NamingSystemType
    
    # Components
    components: List[NameComponent] = field(default_factory=list)
    
    # Full name as recorded
    full_name_as_recorded: str = ""
    
    # Formatted versions
    formatted_versions: Dict[str, str] = field(default_factory=dict)
    
    # Cultural context
    cultural_context: Dict[str, Any] = field(default_factory=dict)
    # gender, social_status, time_period, region
    
    # Temporal validity
    valid_from: Optional[date] = None
    valid_to: Optional[date] = None
    
    # Usage context
    usage_contexts: List[str] = field(default_factory=list)
    # ["legal", "religious", "informal", "professional"]
    
    def get_component(self, component_type: NameComponentType) -> Optional[str]:
        """Get a specific component value"""
        for comp in self.components:
            if comp.component_type == component_type:
                return comp.value
        return None
    
    def format(self, style: str = "full", system: Optional[NamingSystem] = None) -> str:
        """Format the name in a specific style"""
        if style in self.formatted_versions:
            return self.formatted_versions[style]
        
        if system:
            formatted = system.format_name(self.components, style)
            self.formatted_versions[style] = formatted
            return formatted
        
        return self.full_name_as_recorded


class NameAnalyzer:
    """Analyzes and compares names across cultures"""
    
    def __init__(self):
        self.systems: Dict[NamingSystemType, NamingSystem] = {
            NamingSystemType.WESTERN: WesternNamingSystem(),
            NamingSystemType.ISLAMIC: IslamicNamingSystem(),
            # Add more systems
        }
    
    def identify_system(self, name: str) -> NamingSystemType:
        """Try to identify the naming system"""
        # Look for Islamic markers
        if any(marker in name.split() for marker in ["ibn", "bin", "bint", "Abu", "Umm", "al-"]):
            return NamingSystemType.ISLAMIC
        
        # Default to Western for now
        return NamingSystemType.WESTERN
    
    def parse_name(self, name: str, system_type: Optional[NamingSystemType] = None) -> CulturalName:
        """Parse a name with cultural awareness"""
        if not system_type:
            system_type = self.identify_system(name)
        
        system = self.systems.get(system_type)
        if not system:
            raise ValueError(f"Unsupported naming system: {system_type}")
        
        components = system.parse_name(name)
        
        return CulturalName(
            naming_system=system_type,
            components=components,
            full_name_as_recorded=name
        )
    
    def compare_names(self, name1: CulturalName, name2: CulturalName) -> float:
        """Compare two names for similarity (0-1)"""
        # Simple comparison - real implementation would be sophisticated
        score = 0.0
        matches = 0
        total = 0
        
        # Compare each component type
        component_types = set()
        for comp in name1.components + name2.components:
            component_types.add(comp.component_type)
        
        for comp_type in component_types:
            val1 = name1.get_component(comp_type)
            val2 = name2.get_component(comp_type)
            
            if val1 and val2:
                total += 1
                if val1.lower() == val2.lower():
                    matches += 1
                elif val1.lower() in val2.lower() or val2.lower() in val1.lower():
                    matches += 0.5
        
        return matches / total if total > 0 else 0.0
    
    def transliterate_name(self, name: CulturalName, 
                          target_system: NamingSystemType) -> CulturalName:
        """Transliterate a name to a different system"""
        # This would need proper transliteration rules
        # For now, just copy
        new_name = CulturalName(
            naming_system=target_system,
            components=name.components.copy(),
            full_name_as_recorded=name.full_name_as_recorded
        )
        
        return new_name