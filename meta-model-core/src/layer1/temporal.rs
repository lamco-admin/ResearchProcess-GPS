// Temporal System - Universal time representation

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Universal temporal value that can express any time concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalValue {
    /// Single point in time
    Instant(TemporalInstant),
    
    /// Range between two instants
    Range(TemporalInstant, TemporalInstant),
    
    /// Duration without specific start/end
    Duration(TemporalDuration),
    
    /// Recurring temporal pattern
    Recurring(RecurringPattern),
    
    /// Unknown but bounded
    Bounded {
        earliest: Option<TemporalInstant>,
        latest: Option<TemporalInstant>,
    },
    
    /// Uncertain with possibilities
    Uncertain {
        possibilities: Vec<(TemporalValue, f64)>, // value + probability
        constraints: Vec<TemporalConstraint>,
    },
    
    /// Relative to another temporal value
    Relative {
        anchor: Box<TemporalValue>,
        offset: TemporalDuration,
        relation: TemporalRelation,
    },
}

/// A moment in time expressed in multiple calendar systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalInstant {
    /// Multiple calendar expressions for the same instant
    pub expressions: Vec<CalendarExpression>,
    
    /// Precision of this instant
    pub precision: TemporalPrecision,
    
    /// Quality of the temporal information
    pub quality: TemporalQuality,
}

/// Calendar-specific expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalendarExpression {
    /// Gregorian calendar (most common)
    Gregorian {
        year: i32,
        month: Option<u8>,
        day: Option<u8>,
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
    },
    
    /// Julian calendar
    Julian {
        year: i32,
        month: Option<u8>,
        day: Option<u8>,
    },
    
    /// ISO 8601 timestamp
    ISO8601(DateTime<Utc>),
    
    /// Generic calendar expression
    Generic {
        calendar: String,
        expression: String,
        parsed: Option<serde_json::Value>,
    },
}

/// Precision of temporal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalPrecision {
    Millennium,
    Century,
    Decade,
    Year,
    Quarter,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
}

/// Quality of temporal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalQuality {
    Exact,
    Approximate,
    Estimated,
    Calculated,
    Interpreted,
}

/// Duration of time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDuration {
    pub years: Option<i32>,
    pub months: Option<i32>,
    pub days: Option<i32>,
    pub hours: Option<i32>,
    pub minutes: Option<i32>,
    pub seconds: Option<i32>,
}

/// Recurring temporal pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringPattern {
    pub pattern_type: String, // "Daily", "Weekly", "Monthly", "Yearly", "Custom"
    pub interval: i32,
    pub parameters: serde_json::Value,
}

/// Temporal constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalConstraint {
    pub constraint_type: String,
    pub expression: String,
}

/// Temporal relation for relative times
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalRelation {
    Before,
    After,
    During,
    Overlapping,
}

impl TemporalValue {
    /// Create a simple year
    pub fn year(year: i32) -> Self {
        TemporalValue::Instant(TemporalInstant {
            expressions: vec![CalendarExpression::Gregorian {
                year,
                month: None,
                day: None,
                hour: None,
                minute: None,
                second: None,
            }],
            precision: TemporalPrecision::Year,
            quality: TemporalQuality::Exact,
        })
    }
    
    /// Create an approximate year
    pub fn approximate_year(year: i32) -> Self {
        TemporalValue::Instant(TemporalInstant {
            expressions: vec![CalendarExpression::Gregorian {
                year,
                month: None,
                day: None,
                hour: None,
                minute: None,
                second: None,
            }],
            precision: TemporalPrecision::Year,
            quality: TemporalQuality::Approximate,
        })
    }
    
    /// Create a year range
    pub fn year_range(start: i32, end: i32) -> Self {
        TemporalValue::Range(
            TemporalInstant {
                expressions: vec![CalendarExpression::Gregorian {
                    year: start,
                    month: None,
                    day: None,
                    hour: None,
                    minute: None,
                    second: None,
                }],
                precision: TemporalPrecision::Year,
                quality: TemporalQuality::Exact,
            },
            TemporalInstant {
                expressions: vec![CalendarExpression::Gregorian {
                    year: end,
                    month: None,
                    day: None,
                    hour: None,
                    minute: None,
                    second: None,
                }],
                precision: TemporalPrecision::Year,
                quality: TemporalQuality::Exact,
            },
        )
    }
    
    /// Create "before year"
    pub fn before_year(year: i32) -> Self {
        TemporalValue::Bounded {
            earliest: None,
            latest: Some(TemporalInstant {
                expressions: vec![CalendarExpression::Gregorian {
                    year,
                    month: None,
                    day: None,
                    hour: None,
                    minute: None,
                    second: None,
                }],
                precision: TemporalPrecision::Year,
                quality: TemporalQuality::Exact,
            }),
        }
    }
    
    /// Create "after year"
    pub fn after_year(year: i32) -> Self {
        TemporalValue::Bounded {
            earliest: Some(TemporalInstant {
                expressions: vec![CalendarExpression::Gregorian {
                    year,
                    month: None,
                    day: None,
                    hour: None,
                    minute: None,
                    second: None,
                }],
                precision: TemporalPrecision::Year,
                quality: TemporalQuality::Exact,
            }),
            latest: None,
        }
    }
}