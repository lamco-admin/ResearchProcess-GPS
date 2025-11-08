//! Calendar-agnostic temporal values
//!
//! Supports multiple calendar systems, uncertain dates, relative dates,
//! and narrative time descriptions.

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

use crate::{EntityId, PropertyGraph};

/// Calendar-agnostic temporal value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum TemporalValue {
    /// Single moment in time
    Instant(TemporalInstant),

    /// Time range
    Range(Box<TemporalInstant>, Box<TemporalInstant>),

    /// Before a specific time
    Before(Box<TemporalInstant>),

    /// After a specific time
    After(Box<TemporalInstant>),

    /// Relative to another time
    Relative {
        anchor: TemporalReference,
        offset: Duration,
    },

    /// Recurring time pattern
    Recurring(TemporalPattern),

    /// Uncertain time with multiple possibilities
    Uncertain {
        possibilities: Vec<(TemporalValue, f64)>,
        constraints: Vec<TemporalConstraint>,
    },

    /// Narrative time description
    Narrative(String),
}

/// A specific moment in time
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemporalInstant {
    /// Calendar expressions for this instant (can be multiple)
    pub expressions: Vec<CalendarExpression>,

    /// Precision of this instant
    pub precision: TemporalPrecision,

    /// Quality of the temporal information
    pub quality: TemporalQuality,
}

impl TemporalInstant {
    /// Create a new instant with a Gregorian date
    pub fn gregorian(year: i32, month: Option<u8>, day: Option<u8>) -> Self {
        Self {
            expressions: vec![CalendarExpression::Gregorian { year, month, day }],
            precision: match (month, day) {
                (None, None) => TemporalPrecision::Year,
                (Some(_), None) => TemporalPrecision::Month,
                (Some(_), Some(_)) => TemporalPrecision::Day,
                _ => TemporalPrecision::Unknown,
            },
            quality: TemporalQuality::Exact,
        }
    }

    /// Create an instant from UTC datetime
    pub fn from_utc(dt: DateTime<Utc>) -> Self {
        Self {
            expressions: vec![CalendarExpression::Gregorian {
                year: dt.year(),
                month: Some(dt.month() as u8),
                day: Some(dt.day() as u8),
            }],
            precision: TemporalPrecision::Day,
            quality: TemporalQuality::Exact,
        }
    }

    /// Create an approximate instant
    pub fn approximate(year: i32, month: Option<u8>, day: Option<u8>) -> Self {
        let mut instant = Self::gregorian(year, month, day);
        instant.quality = TemporalQuality::Approximate;
        instant
    }
}

/// Calendar system expressions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "calendar", content = "date")]
pub enum CalendarExpression {
    /// Gregorian calendar
    Gregorian {
        year: i32,
        month: Option<u8>,
        day: Option<u8>,
    },

    /// Julian calendar
    Julian {
        year: i32,
        month: Option<u8>,
        day: Option<u8>,
    },

    /// Dual dating for calendar transitions
    DualDated {
        julian: Box<CalendarExpression>,
        gregorian: Box<CalendarExpression>,
    },

    /// Hebrew calendar
    Hebrew {
        year: i32,
        month: String,
        day: Option<u8>,
    },

    /// Islamic calendar
    Islamic {
        year: i32,
        month: String,
        day: Option<u8>,
    },

    /// French Republican calendar
    FrenchRepublican {
        year: i32,
        month: String,
        day: Option<u8>,
    },

    /// Relative expression (e.g., "third year of the reign of...")
    Relative { description: String },

    /// Custom calendar system
    Custom {
        calendar_system: String,
        expression: String,
    },
}

/// Temporal precision levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TemporalPrecision {
    Exact,
    Day,
    Month,
    Year,
    Decade,
    Century,
    Era,
    Unknown,
}

/// Temporal quality indicators
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TemporalQuality {
    Exact,
    Approximate,
    Estimated,
    Calculated,
    Interpreted,
    Unknown,
}

/// Duration with unit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Duration {
    pub value: f64,
    pub unit: String, // "days", "years", "generations", etc.
}

/// Reference to a temporal anchor point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum TemporalReference {
    Entity(EntityId),
    Instant(Box<TemporalInstant>),
    Narrative(String),
}

/// Recurring temporal pattern
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemporalPattern {
    pub pattern_type: String, // "annual", "seasonal", "generational"
    pub parameters: PropertyGraph,
}

/// Temporal constraint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemporalConstraint {
    pub constraint_type: String, // "must_be_before", "must_be_after"
    pub reference: TemporalReference,
}

/// Temporal scope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemporalScope {
    pub description: String,
    pub bounds: Option<(TemporalValue, TemporalValue)>,
}

impl TemporalValue {
    /// Create a simple date
    pub fn date(year: i32, month: Option<u8>, day: Option<u8>) -> Self {
        Self::Instant(TemporalInstant::gregorian(year, month, day))
    }

    /// Create an approximate date
    pub fn circa(year: i32) -> Self {
        Self::Instant(TemporalInstant::approximate(year, None, None))
    }

    /// Create a date range
    pub fn range(from: TemporalInstant, to: TemporalInstant) -> Self {
        Self::Range(Box::new(from), Box::new(to))
    }

    /// Create a narrative time
    pub fn narrative(description: impl Into<String>) -> Self {
        Self::Narrative(description.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gregorian_date() {
        let instant = TemporalInstant::gregorian(1850, Some(3), Some(15));
        assert_eq!(instant.precision, TemporalPrecision::Day);
        assert_eq!(instant.quality, TemporalQuality::Exact);

        match &instant.expressions[0] {
            CalendarExpression::Gregorian { year, month, day } => {
                assert_eq!(*year, 1850);
                assert_eq!(*month, Some(3));
                assert_eq!(*day, Some(15));
            }
            _ => panic!("Expected Gregorian calendar"),
        }
    }

    #[test]
    fn test_approximate_date() {
        let instant = TemporalInstant::approximate(1850, None, None);
        assert_eq!(instant.precision, TemporalPrecision::Year);
        assert_eq!(instant.quality, TemporalQuality::Approximate);
    }

    #[test]
    fn test_temporal_value_date() {
        let value = TemporalValue::date(1850, Some(3), Some(15));
        assert!(matches!(value, TemporalValue::Instant(_)));
    }

    #[test]
    fn test_temporal_value_circa() {
        let value = TemporalValue::circa(1850);
        match value {
            TemporalValue::Instant(instant) => {
                assert_eq!(instant.quality, TemporalQuality::Approximate);
            }
            _ => panic!("Expected Instant"),
        }
    }

    #[test]
    fn test_temporal_value_narrative() {
        let value = TemporalValue::narrative("when the cherry blossoms bloomed");
        assert!(matches!(value, TemporalValue::Narrative(_)));
    }

    #[test]
    fn test_dual_dated() {
        let julian = CalendarExpression::Julian {
            year: 1752,
            month: Some(2),
            day: Some(11),
        };
        let gregorian = CalendarExpression::Gregorian {
            year: 1752,
            month: Some(2),
            day: Some(22),
        };

        let dual = CalendarExpression::DualDated {
            julian: Box::new(julian),
            gregorian: Box::new(gregorian),
        };

        assert!(matches!(dual, CalendarExpression::DualDated { .. }));
    }

    #[test]
    fn test_temporal_range() {
        let from = TemporalInstant::gregorian(1800, Some(1), Some(1));
        let to = TemporalInstant::gregorian(1850, Some(12), Some(31));
        let range = TemporalValue::range(from, to);

        assert!(matches!(range, TemporalValue::Range(_, _)));
    }
}
