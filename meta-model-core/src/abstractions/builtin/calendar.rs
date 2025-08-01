// Calendar abstraction layer

use crate::layer1::*;
use crate::abstractions::{
    AbstractionLayer, TransformContext, ImportResult, ValidationReport,
    AbstractionResult, AbstractionError,
};
use async_trait::async_trait;
use chrono::{NaiveDate, Datelike};
use regex::Regex;
use once_cell::sync::Lazy;

/// Calendar system abstraction
pub struct CalendarAbstraction {
    calendar_type: CalendarType,
}

#[derive(Debug, Clone)]
enum CalendarType {
    Gregorian,
    Julian,
    FrenchRepublican,
    Hebrew,
    Islamic,
}

impl CalendarAbstraction {
    pub fn gregorian() -> Self {
        CalendarAbstraction {
            calendar_type: CalendarType::Gregorian,
        }
    }
    
    pub fn julian() -> Self {
        CalendarAbstraction {
            calendar_type: CalendarType::Julian,
        }
    }
    
    pub fn french_republican() -> Self {
        CalendarAbstraction {
            calendar_type: CalendarType::FrenchRepublican,
        }
    }
    
    /// Parse a date string into TemporalValue
    pub fn parse_date(&self, input: &str) -> AbstractionResult<TemporalValue> {
        match self.calendar_type {
            CalendarType::Gregorian => self.parse_gregorian(input),
            CalendarType::Julian => self.parse_julian(input),
            CalendarType::FrenchRepublican => self.parse_french_republican(input),
            _ => Err(AbstractionError::CalendarError("Calendar not implemented".to_string())),
        }
    }
    
    fn parse_gregorian(&self, input: &str) -> AbstractionResult<TemporalValue> {
        // Date modifiers
        static DATE_MODIFIERS: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?i)^(abt|about|circa|c\.|ca\.|est\.|estimated|bef|before|aft|after|bet|between)\s+").unwrap()
        });
        
        // Check for modifiers
        if let Some(captures) = DATE_MODIFIERS.captures(input) {
            let modifier = captures.get(1).unwrap().as_str().to_lowercase();
            let date_part = &input[captures.get(0).unwrap().end()..];
            
            match modifier.as_str() {
                "abt" | "about" | "circa" | "c." | "ca." | "est." | "estimated" => {
                    if let Ok(base) = self.parse_simple_gregorian(date_part) {
                        return Ok(self.make_approximate(base));
                    }
                }
                "bef" | "before" => {
                    if let Ok(base) = self.parse_simple_gregorian(date_part) {
                        return Ok(TemporalValue::Bounded {
                            earliest: None,
                            latest: Some(self.extract_instant(base)),
                        });
                    }
                }
                "aft" | "after" => {
                    if let Ok(base) = self.parse_simple_gregorian(date_part) {
                        return Ok(TemporalValue::Bounded {
                            earliest: Some(self.extract_instant(base)),
                            latest: None,
                        });
                    }
                }
                "bet" | "between" => {
                    // Handle "between X and Y"
                    if let Some(and_pos) = date_part.find(" and ") {
                        let start_str = &date_part[..and_pos];
                        let end_str = &date_part[and_pos + 5..];
                        
                        if let (Ok(start), Ok(end)) = (
                            self.parse_simple_gregorian(start_str),
                            self.parse_simple_gregorian(end_str)
                        ) {
                            return Ok(TemporalValue::Range(
                                self.extract_instant(start),
                                self.extract_instant(end),
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
        
        // Try simple date parsing
        self.parse_simple_gregorian(input)
    }
    
    fn parse_simple_gregorian(&self, input: &str) -> AbstractionResult<TemporalValue> {
        let trimmed = input.trim();
        
        // Try different date formats
        // Year only: "1850"
        if let Ok(year) = trimmed.parse::<i32>() {
            return Ok(TemporalValue::Instant(TemporalInstant {
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
            }));
        }
        
        // Month Year: "Jan 1850" or "January 1850"
        static MONTH_YEAR: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?i)^(jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|september|oct|october|nov|november|dec|december)\s+(\d{4})$").unwrap()
        });
        
        if let Some(captures) = MONTH_YEAR.captures(trimmed) {
            let month_str = captures.get(1).unwrap().as_str();
            let year: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
            let month = self.month_from_name(month_str);
            
            return Ok(TemporalValue::Instant(TemporalInstant {
                expressions: vec![CalendarExpression::Gregorian {
                    year,
                    month: Some(month),
                    day: None,
                    hour: None,
                    minute: None,
                    second: None,
                }],
                precision: TemporalPrecision::Month,
                quality: TemporalQuality::Exact,
            }));
        }
        
        // Full date: "1 Jan 1850" or "25 December 1850"
        static FULL_DATE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?i)^(\d{1,2})\s+(jan|january|feb|february|mar|march|apr|april|may|jun|june|jul|july|aug|august|sep|september|oct|october|nov|november|dec|december)\s+(\d{4})$").unwrap()
        });
        
        if let Some(captures) = FULL_DATE.captures(trimmed) {
            let day: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
            let month_str = captures.get(2).unwrap().as_str();
            let year: i32 = captures.get(3).unwrap().as_str().parse().unwrap();
            let month = self.month_from_name(month_str);
            
            return Ok(TemporalValue::Instant(TemporalInstant {
                expressions: vec![CalendarExpression::Gregorian {
                    year,
                    month: Some(month),
                    day: Some(day),
                    hour: None,
                    minute: None,
                    second: None,
                }],
                precision: TemporalPrecision::Day,
                quality: TemporalQuality::Exact,
            }));
        }
        
        // ISO format: "1850-01-15"
        if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
            return Ok(TemporalValue::Instant(TemporalInstant {
                expressions: vec![CalendarExpression::Gregorian {
                    year: date.year(),
                    month: Some(date.month() as u8),
                    day: Some(date.day() as u8),
                    hour: None,
                    minute: None,
                    second: None,
                }],
                precision: TemporalPrecision::Day,
                quality: TemporalQuality::Exact,
            }));
        }
        
        Err(AbstractionError::CalendarError(format!("Cannot parse date: {}", input)))
    }
    
    fn parse_julian(&self, input: &str) -> AbstractionResult<TemporalValue> {
        // Julian calendar parsing
        // Handle dual dating: "25 March 1750/51"
        static DUAL_DATE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(.+?)\s+(\d{4})/(\d{2,4})$").unwrap()
        });
        
        if let Some(captures) = DUAL_DATE.captures(input) {
            let date_part = captures.get(1).unwrap().as_str();
            let year1: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
            let year2_str = captures.get(3).unwrap().as_str();
            
            // Handle abbreviated year (50 -> 1750)
            let year2 = if year2_str.len() == 2 {
                (year1 / 100) * 100 + year2_str.parse::<i32>().unwrap()
            } else {
                year2_str.parse().unwrap()
            };
            
            // Create uncertain temporal value
            return Ok(TemporalValue::Uncertain {
                possibilities: vec![
                    (self.parse_simple_julian(&format!("{} {}", date_part, year1))?, 0.5),
                    (self.parse_simple_julian(&format!("{} {}", date_part, year2))?, 0.5),
                ],
                constraints: vec![TemporalConstraint {
                    constraint_type: "DualDating".to_string(),
                    expression: "Old Style / New Style".to_string(),
                }],
            });
        }
        
        self.parse_simple_julian(input)
    }
    
    fn parse_simple_julian(&self, input: &str) -> AbstractionResult<TemporalValue> {
        // Similar to Gregorian but with Julian calendar expression
        // For now, parse as Gregorian and mark as Julian
        if let Ok(mut temporal) = self.parse_simple_gregorian(input) {
            // Convert to Julian expression
            if let TemporalValue::Instant(ref mut instant) = temporal {
                for expr in &mut instant.expressions {
                    if let CalendarExpression::Gregorian { year, month, day, .. } = expr {
                        *expr = CalendarExpression::Julian {
                            year: *year,
                            month: *month,
                            day: *day,
                        };
                    }
                }
            }
            return Ok(temporal);
        }
        
        Err(AbstractionError::CalendarError(format!("Cannot parse Julian date: {}", input)))
    }
    
    fn parse_french_republican(&self, input: &str) -> AbstractionResult<TemporalValue> {
        // Parse dates like "5 Thermidor An VII"
        static FRENCH_DATE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?i)^(\d{1,2})\s+(vendémiaire|brumaire|frimaire|nivôse|pluviôse|ventôse|germinal|floréal|prairial|messidor|thermidor|fructidor|sansculottides?)\s+an\s+([IVXLCDM]+|\d+)$").unwrap()
        });
        
        if let Some(captures) = FRENCH_DATE.captures(input.trim()) {
            let day: u8 = captures.get(1).unwrap().as_str().parse()
                .map_err(|_| AbstractionError::CalendarError("Invalid day".to_string()))?;
            let month_name = captures.get(2).unwrap().as_str();
            let year_str = captures.get(3).unwrap().as_str();
            
            // Convert month name to number
            let month = match month_name.to_lowercase().as_str() {
                "vendémiaire" => 1,
                "brumaire" => 2,
                "frimaire" => 3,
                "nivôse" => 4,
                "pluviôse" => 5,
                "ventôse" => 6,
                "germinal" => 7,
                "floréal" => 8,
                "prairial" => 9,
                "messidor" => 10,
                "thermidor" => 11,
                "fructidor" => 12,
                "sansculottides" => 13, // Complementary days
                _ => return Err(AbstractionError::CalendarError("Unknown month".to_string())),
            };
            
            // Parse year (Roman or Arabic numerals)
            let year = if year_str.chars().all(|c| "IVXLCDM".contains(c)) {
                self.parse_roman_numeral(year_str)?
            } else {
                year_str.parse()
                    .map_err(|_| AbstractionError::CalendarError("Invalid year".to_string()))?
            };
            
            return Ok(TemporalValue::Instant(TemporalInstant {
                expressions: vec![CalendarExpression::Generic {
                    calendar: "FrenchRepublican".to_string(),
                    expression: input.to_string(),
                    parsed: Some(serde_json::json!({
                        "day": day,
                        "month": month,
                        "year": year,
                    })),
                }],
                precision: TemporalPrecision::Day,
                quality: TemporalQuality::Exact,
            }));
        }
        
        Err(AbstractionError::CalendarError(format!("Cannot parse French Republican date: {}", input)))
    }
    
    fn month_from_name(&self, name: &str) -> u8 {
        match name.to_lowercase().as_str() {
            "jan" | "january" => 1,
            "feb" | "february" => 2,
            "mar" | "march" => 3,
            "apr" | "april" => 4,
            "may" => 5,
            "jun" | "june" => 6,
            "jul" | "july" => 7,
            "aug" | "august" => 8,
            "sep" | "september" => 9,
            "oct" | "october" => 10,
            "nov" | "november" => 11,
            "dec" | "december" => 12,
            _ => 0,
        }
    }
    
    fn parse_roman_numeral(&self, roman: &str) -> AbstractionResult<i32> {
        let mut result = 0;
        let mut prev_value = 0;
        
        for ch in roman.chars().rev() {
            let value = match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => return Err(AbstractionError::CalendarError("Invalid Roman numeral".to_string())),
            };
            
            if value < prev_value {
                result -= value;
            } else {
                result += value;
            }
            prev_value = value;
        }
        
        Ok(result)
    }
    
    fn make_approximate(&self, base: TemporalValue) -> TemporalValue {
        if let TemporalValue::Instant(mut instant) = base {
            instant.quality = TemporalQuality::Approximate;
            TemporalValue::Instant(instant)
        } else {
            base
        }
    }
    
    fn extract_instant(&self, temporal: TemporalValue) -> TemporalInstant {
        match temporal {
            TemporalValue::Instant(instant) => instant,
            _ => panic!("Expected instant"),
        }
    }
}

#[async_trait]
impl AbstractionLayer for CalendarAbstraction {
    fn name(&self) -> &str {
        match self.calendar_type {
            CalendarType::Gregorian => "Gregorian Calendar",
            CalendarType::Julian => "Julian Calendar",
            CalendarType::FrenchRepublican => "French Republican Calendar",
            CalendarType::Hebrew => "Hebrew Calendar",
            CalendarType::Islamic => "Islamic Calendar",
        }
    }
    
    fn format_id(&self) -> &str {
        match self.calendar_type {
            CalendarType::Gregorian => "CALENDAR.GREGORIAN",
            CalendarType::Julian => "CALENDAR.JULIAN",
            CalendarType::FrenchRepublican => "CALENDAR.FRENCH_REPUBLICAN",
            CalendarType::Hebrew => "CALENDAR.HEBREW",
            CalendarType::Islamic => "CALENDAR.ISLAMIC",
        }
    }
    
    fn format_version(&self) -> &str {
        "1.0"
    }
    
    async fn import(
        &self,
        data: &[u8],
        _context: &mut TransformContext,
    ) -> AbstractionResult<ImportResult> {
        // Calendar abstractions don't import entities, they parse dates
        Err(AbstractionError::UnsupportedFormat(
            "Calendar abstractions parse dates, not import entities".to_string()
        ))
    }
    
    async fn export(
        &self,
        _entities: &[Entity],
        _relationships: &[Relationship],
        _context: &mut TransformContext,
    ) -> AbstractionResult<Vec<u8>> {
        // Calendar abstractions don't export
        Err(AbstractionError::UnsupportedFormat(
            "Calendar abstractions format dates, not export entities".to_string()
        ))
    }
    
    async fn validate(&self, _data: &[u8]) -> AbstractionResult<ValidationReport> {
        // Calendar abstractions validate date strings, not files
        Ok(ValidationReport {
            valid: false,
            format_detected: None,
            version_detected: None,
            errors: vec![],
            warnings: vec![],
        })
    }
}