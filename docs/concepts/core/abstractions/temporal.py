"""
Temporal abstraction layer for ResearchProcess-GPS.

Handles all date/time complexities including multiple calendar systems,
approximate dates, ranges, and complex temporal expressions.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any, Set, Tuple, Union
from datetime import date, datetime, timedelta
from enum import Enum
from uuid import UUID, uuid4


class CalendarType(Enum):
    """Supported calendar systems"""
    GREGORIAN = "gregorian"
    JULIAN = "julian"
    HEBREW = "hebrew"
    ISLAMIC = "islamic"
    CHINESE = "chinese"
    PERSIAN = "persian"
    FRENCH_REVOLUTIONARY = "french_revolutionary"
    COPTIC = "coptic"
    ETHIOPIAN = "ethiopian"
    INDIAN_CIVIL = "indian_civil"
    JAPANESE = "japanese"
    CUSTOM = "custom"


class DatePrecision(Enum):
    """How precise a date is"""
    EXACT = "exact"              # Know exact date and time
    DAY = "day"                  # Know the day
    MONTH = "month"              # Know month and year
    SEASON = "season"            # Know season and year
    YEAR = "year"                # Know only year
    DECADE = "decade"            # Know approximate decade
    CENTURY = "century"          # Know only century
    ERA = "era"                  # Know only era/period
    UNKNOWN = "unknown"


class DateModifier(Enum):
    """Modifiers for dates"""
    EXACT = "exact"              # Exactly this date
    ABOUT = "about"              # Approximately
    BEFORE = "before"            # Before this date
    AFTER = "after"              # After this date
    OR_BEFORE = "or_before"      # On or before
    OR_AFTER = "or_after"        # On or after
    CALCULATED = "calculated"     # Calculated from other info
    ESTIMATED = "estimated"       # Estimated
    INTERPRETED = "interpreted"   # Interpreted from source


class TemporalExpressionType(Enum):
    """Types of temporal expressions"""
    EXACT_DATE = "exact_date"
    DATE_RANGE = "date_range"
    MULTIPLE_DATES = "multiple_dates"
    NOT_DATE = "not_date"
    RELATIVE_DATE = "relative_date"
    RECURRING_DATE = "recurring_date"
    DURATION = "duration"
    SEQUENCE = "sequence"
    UNKNOWN = "unknown"


@dataclass
class CalendarDate:
    """A date in a specific calendar system"""
    calendar_type: CalendarType
    year: Optional[int] = None
    month: Optional[int] = None
    day: Optional[int] = None
    
    # Calendar-specific fields
    era: Optional[str] = None  # BCE/CE, AH, etc.
    cycle: Optional[int] = None  # For Chinese calendar
    
    # Original representation
    original_text: str = ""
    
    def to_gregorian(self) -> Optional[date]:
        """Convert to Gregorian calendar if possible"""
        # This would need calendar conversion libraries
        # For now, return None - real implementation would convert
        return None


class CalendarSystem(ABC):
    """Abstract base for calendar systems"""
    
    @abstractmethod
    def get_type(self) -> CalendarType:
        """Get calendar type"""
        pass
    
    @abstractmethod
    def parse_date(self, date_string: str) -> CalendarDate:
        """Parse a date string in this calendar"""
        pass
    
    @abstractmethod
    def format_date(self, cal_date: CalendarDate) -> str:
        """Format a date in this calendar"""
        pass
    
    @abstractmethod
    def to_gregorian(self, cal_date: CalendarDate) -> Optional[date]:
        """Convert to Gregorian date"""
        pass
    
    @abstractmethod
    def from_gregorian(self, greg_date: date) -> CalendarDate:
        """Convert from Gregorian date"""
        pass
    
    @abstractmethod
    def validate_date(self, cal_date: CalendarDate) -> bool:
        """Check if date is valid in this calendar"""
        pass


@dataclass
class DateComponent:
    """A single component of a complex date expression"""
    component_id: UUID = field(default_factory=uuid4)
    
    # The date itself
    calendar_date: Optional[CalendarDate] = None
    gregorian_equivalent: Optional[date] = None
    
    # Precision and modifiers
    precision: DatePrecision = DatePrecision.UNKNOWN
    modifier: DateModifier = DateModifier.EXACT
    
    # Confidence
    confidence: float = 0.0  # 0-1
    
    # Source
    source_text: str = ""
    interpretation_notes: str = ""


@dataclass
class TemporalExpression:
    """
    A complex temporal expression that can represent:
    - Exact dates
    - Date ranges
    - Multiple possible dates
    - NOT dates (explicitly not this date)
    - Approximate dates
    - Relative dates
    - Recurring dates
    - Unknown dates with constraints
    """
    expression_id: UUID = field(default_factory=uuid4)
    expression_type: TemporalExpressionType = TemporalExpressionType.UNKNOWN
    
    # Components
    components: List[DateComponent] = field(default_factory=list)
    
    # For ranges
    start_component: Optional[DateComponent] = None
    end_component: Optional[DateComponent] = None
    
    # For NOT dates
    excluded_dates: List[DateComponent] = field(default_factory=list)
    
    # For relative dates
    relative_to: Optional[UUID] = None  # Reference to another event
    relative_offset: Optional[timedelta] = None
    relative_description: str = ""  # "3 months after", "before"
    
    # For recurring dates
    recurrence_pattern: Optional[str] = None  # "annually", "every 7 years"
    
    # Constraints
    constraints: List[str] = field(default_factory=list)
    # ["must be Tuesday", "during harvest", "full moon"]
    
    # Display
    display_text: str = ""  # How to show to user
    sort_date: Optional[date] = None  # For sorting
    
    def add_exact_date(self, cal_date: CalendarDate, 
                      modifier: DateModifier = DateModifier.EXACT) -> DateComponent:
        """Add an exact date component"""
        component = DateComponent(
            calendar_date=cal_date,
            modifier=modifier,
            precision=DatePrecision.DAY
        )
        self.components.append(component)
        self.expression_type = TemporalExpressionType.EXACT_DATE
        return component
    
    def add_date_range(self, start: CalendarDate, end: CalendarDate) -> None:
        """Add a date range"""
        self.start_component = DateComponent(calendar_date=start)
        self.end_component = DateComponent(calendar_date=end)
        self.expression_type = TemporalExpressionType.DATE_RANGE
    
    def add_not_date(self, cal_date: CalendarDate) -> None:
        """Add a NOT date (explicitly not this date)"""
        component = DateComponent(calendar_date=cal_date)
        self.excluded_dates.append(component)
        if self.expression_type == TemporalExpressionType.UNKNOWN:
            self.expression_type = TemporalExpressionType.NOT_DATE
    
    def set_relative_date(self, relative_to_event: UUID, 
                         description: str, offset: Optional[timedelta] = None) -> None:
        """Set as relative to another event"""
        self.relative_to = relative_to_event
        self.relative_description = description
        self.relative_offset = offset
        self.expression_type = TemporalExpressionType.RELATIVE_DATE
    
    def get_date_range(self) -> Tuple[Optional[date], Optional[date]]:
        """Get earliest and latest possible dates"""
        earliest = None
        latest = None
        
        if self.expression_type == TemporalExpressionType.DATE_RANGE:
            if self.start_component and self.start_component.gregorian_equivalent:
                earliest = self.start_component.gregorian_equivalent
            if self.end_component and self.end_component.gregorian_equivalent:
                latest = self.end_component.gregorian_equivalent
        
        elif self.expression_type == TemporalExpressionType.EXACT_DATE:
            if self.components and self.components[0].gregorian_equivalent:
                date_val = self.components[0].gregorian_equivalent
                modifier = self.components[0].modifier
                
                if modifier == DateModifier.BEFORE:
                    latest = date_val
                elif modifier == DateModifier.AFTER:
                    earliest = date_val
                elif modifier == DateModifier.ABOUT:
                    # Add uncertainty based on precision
                    days = {
                        DatePrecision.DAY: 7,
                        DatePrecision.MONTH: 30,
                        DatePrecision.YEAR: 365,
                        DatePrecision.DECADE: 3650,
                        DatePrecision.CENTURY: 36500
                    }.get(self.components[0].precision, 0)
                    
                    earliest = date_val - timedelta(days=days)
                    latest = date_val + timedelta(days=days)
                else:
                    earliest = latest = date_val
        
        return earliest, latest
    
    def could_be_date(self, check_date: date) -> bool:
        """Check if this expression could represent the given date"""
        # Check NOT dates first
        for excluded in self.excluded_dates:
            if excluded.gregorian_equivalent == check_date:
                return False
        
        # Check if in range
        earliest, latest = self.get_date_range()
        if earliest and check_date < earliest:
            return False
        if latest and check_date > latest:
            return False
        
        # Check constraints
        # TODO: Implement constraint checking
        
        return True
    
    def format_display(self, calendar_type: CalendarType = CalendarType.GREGORIAN) -> str:
        """Format for display"""
        if self.display_text:
            return self.display_text
        
        if self.expression_type == TemporalExpressionType.EXACT_DATE:
            if self.components:
                comp = self.components[0]
                prefix = {
                    DateModifier.ABOUT: "about ",
                    DateModifier.BEFORE: "before ",
                    DateModifier.AFTER: "after ",
                    DateModifier.CALCULATED: "calculated ",
                    DateModifier.ESTIMATED: "estimated "
                }.get(comp.modifier, "")
                
                return prefix + comp.source_text
        
        elif self.expression_type == TemporalExpressionType.DATE_RANGE:
            start = self.start_component.source_text if self.start_component else "?"
            end = self.end_component.source_text if self.end_component else "?"
            return f"between {start} and {end}"
        
        elif self.expression_type == TemporalExpressionType.RELATIVE_DATE:
            return self.relative_description
        
        return "unknown date"


@dataclass
class TemporalConstraint:
    """Constraints on when something could have happened"""
    constraint_id: UUID = field(default_factory=uuid4)
    
    # Type of constraint
    constraint_type: str = ""  # "age_based", "sequence", "cultural", "logical"
    description: str = ""
    
    # Bounds
    earliest_possible: Optional[TemporalExpression] = None
    latest_possible: Optional[TemporalExpression] = None
    
    # Related entities
    based_on_entities: List[UUID] = field(default_factory=list)
    based_on_events: List[UUID] = field(default_factory=list)
    
    # Reasoning
    reasoning: str = ""
    confidence: float = 0.0
    
    def apply_to_expression(self, expr: TemporalExpression) -> TemporalExpression:
        """Apply this constraint to a temporal expression"""
        # TODO: Implement constraint application
        return expr


class TemporalAnalyzer:
    """Analyzes temporal relationships and constraints"""
    
    def calculate_age_at_event(self, birth_expr: TemporalExpression, 
                              event_expr: TemporalExpression) -> Optional[Tuple[int, int]]:
        """Calculate age range at event"""
        birth_earliest, birth_latest = birth_expr.get_date_range()
        event_earliest, event_latest = event_expr.get_date_range()
        
        if not all([birth_earliest, birth_latest, event_earliest, event_latest]):
            return None
        
        min_age = (event_earliest - birth_latest).days // 365
        max_age = (event_latest - birth_earliest).days // 365
        
        return max(0, min_age), max_age
    
    def check_temporal_consistency(self, events: List[Tuple[UUID, TemporalExpression]]) -> List[str]:
        """Check if a set of events is temporally consistent"""
        issues = []
        
        # Sort by earliest possible date
        sorted_events = sorted(events, 
                             key=lambda x: x[1].get_date_range()[0] or date.min)
        
        # Check for impossible sequences
        # TODO: Implement sequence checking
        
        return issues
    
    def infer_date_from_age(self, age_text: str, reference_expr: TemporalExpression,
                           is_birth: bool = True) -> TemporalExpression:
        """Infer a date from age and reference date"""
        # Parse age
        try:
            age = int(age_text)
        except ValueError:
            # Handle "infant", "child", etc.
            age_ranges = {
                "infant": (0, 1),
                "child": (1, 12),
                "youth": (12, 18),
                "adult": (18, 100)
            }
            if age_text.lower() in age_ranges:
                min_age, max_age = age_ranges[age_text.lower()]
            else:
                return TemporalExpression(expression_type=TemporalExpressionType.UNKNOWN)
        else:
            min_age = max_age = age
        
        # Calculate date range
        ref_earliest, ref_latest = reference_expr.get_date_range()
        
        result = TemporalExpression(expression_type=TemporalExpressionType.DATE_RANGE)
        
        if ref_earliest:
            if is_birth:
                # Birth = reference - age
                earliest_birth = ref_earliest.year - max_age
                latest_birth = ref_latest.year - min_age if ref_latest else ref_earliest.year - min_age
            else:
                # Death = reference + age  
                earliest_death = ref_earliest.year + min_age
                latest_death = ref_latest.year + max_age if ref_latest else ref_earliest.year + max_age
        
        # TODO: Create proper CalendarDate objects
        result.display_text = f"{'born' if is_birth else 'died'} {age_text} at reference"
        
        return result