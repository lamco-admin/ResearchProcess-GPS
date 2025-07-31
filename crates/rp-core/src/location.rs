//! Location entity - hierarchical geographic data management

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    entity::{EntityMetadata, NestableEntity},
    EntityId, Result, impl_entity, impl_validatable,
};

/// Types of locations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationType {
    /// Continents
    Continent,
    
    /// Countries and nations
    Country,
    Nation,
    
    /// Administrative divisions
    State,
    Province,
    Territory,
    Department,
    Prefecture,
    
    /// Regional divisions
    Region,
    County,
    Parish,
    District,
    Canton,
    
    /// Urban divisions
    City,
    Town,
    Village,
    Township,
    Borough,
    
    /// Local divisions
    Neighborhood,
    Ward,
    Precinct,
    
    /// Specific locations
    Address,
    Building,
    Cemetery,
    Church,
    Hospital,
    School,
    
    /// Geographic features
    Mountain,
    River,
    Lake,
    Island,
    
    /// Custom location type
    Custom(String),
}

/// Geographic coordinates
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub precision: f64, // in meters
}

/// Time period when location existed with specific attributes
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LocationPeriod {
    /// Start of this period
    pub start_date: Option<DateTime<Utc>>,
    
    /// End of this period
    pub end_date: Option<DateTime<Utc>>,
    
    /// Name during this period
    #[validate(length(min = 1))]
    pub name: String,
    
    /// Jurisdiction during this period
    pub jurisdiction: Option<String>,
    
    /// Notes about this period
    pub notes: Option<String>,
}

/// Alternative names for a location
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AlternativeName {
    /// The alternative name
    #[validate(length(min = 1))]
    pub name: String,
    
    /// Language of the name
    pub language: Option<String>,
    
    /// Type of name (official, colloquial, historical, etc.)
    pub name_type: String,
    
    /// Time period when this name was used
    pub period: Option<LocationPeriod>,
    
    /// Source of this name
    pub source: Option<String>,
}

/// Boundary information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boundary {
    /// Type of boundary (political, ecclesiastical, etc.)
    pub boundary_type: String,
    
    /// Description of the boundary
    pub description: String,
    
    /// Neighboring locations
    pub neighbors: Vec<EntityId>,
    
    /// GeoJSON or other boundary data
    pub boundary_data: Option<serde_json::Value>,
}

/// A hierarchical location entity
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Location {
    /// Entity metadata
    #[serde(flatten)]
    pub metadata: EntityMetadata,
    
    /// Type of location
    pub location_type: LocationType,
    
    /// Current official name
    #[validate(length(min = 1))]
    pub name: String,
    
    /// Full hierarchical name (e.g., "Boston, Suffolk, Massachusetts, USA")
    pub full_name: String,
    
    /// Alternative names
    #[validate(nested)]
    pub alternative_names: Vec<AlternativeName>,
    
    /// Geographic coordinates
    pub coordinates: Option<Coordinates>,
    
    /// Parent location (hierarchical)
    pub parent_location: Option<EntityId>,
    
    /// Child locations
    pub child_locations: Vec<EntityId>,
    
    /// Historical periods with different attributes
    #[validate(nested)]
    pub historical_periods: Vec<LocationPeriod>,
    
    /// Current jurisdiction
    pub jurisdiction: Option<String>,
    
    /// Boundary information
    pub boundaries: Vec<Boundary>,
    
    /// Population data
    pub population: Option<u64>,
    pub population_date: Option<DateTime<Utc>>,
    
    /// Area in square kilometers
    pub area_sq_km: Option<f64>,
    
    /// Elevation in meters
    pub elevation_m: Option<f64>,
    
    /// Time zone
    pub timezone: Option<String>,
    
    /// ISO codes (country, subdivision)
    pub iso_codes: Vec<String>,
    
    /// FIPS codes
    pub fips_codes: Vec<String>,
    
    /// Geonames ID
    pub geonames_id: Option<u64>,
    
    /// Wikidata ID
    pub wikidata_id: Option<String>,
    
    /// Sources for this location data
    pub source_ids: Vec<EntityId>,
    
    /// Notes
    pub notes: Option<String>,
    
    /// Custom attributes
    pub custom_fields: serde_json::Value,
    
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Location {
    /// Create a new location
    pub fn new(
        location_type: LocationType,
        name: impl Into<String>,
        created_by: EntityId,
    ) -> Self {
        let name = name.into();
        let now = Utc::now();
        
        Self {
            metadata: EntityMetadata {
                id: EntityId::new(),
                created_by,
                created_at: now,
                modified_by: created_by,
                modified_at: now,
                is_active: true,
                version: 1,
                parent_version: None,
            },
            location_type,
            name: name.clone(),
            full_name: name,
            alternative_names: vec![],
            coordinates: None,
            parent_location: None,
            child_locations: vec![],
            historical_periods: vec![],
            jurisdiction: None,
            boundaries: vec![],
            population: None,
            population_date: None,
            area_sq_km: None,
            elevation_m: None,
            timezone: None,
            iso_codes: vec![],
            fips_codes: vec![],
            geonames_id: None,
            wikidata_id: None,
            source_ids: vec![],
            notes: None,
            custom_fields: serde_json::Value::Object(serde_json::Map::new()),
            tags: vec![],
        }
    }
    
    /// Create a location with coordinates
    pub fn with_coordinates(
        location_type: LocationType,
        name: impl Into<String>,
        coordinates: Coordinates,
        created_by: EntityId,
    ) -> Self {
        let mut location = Self::new(location_type, name, created_by);
        location.coordinates = Some(coordinates);
        location
    }
    
    /// Add an alternative name
    pub fn add_alternative_name(&mut self, alt_name: AlternativeName) {
        self.alternative_names.push(alt_name);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Add a historical period
    pub fn add_historical_period(&mut self, period: LocationPeriod) {
        self.historical_periods.push(period);
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Set parent location and update full name
    pub fn set_parent_location(&mut self, parent_id: EntityId, parent_name: &str) {
        self.parent_location = Some(parent_id);
        self.update_full_name(Some(parent_name));
        self.metadata.update(self.metadata.modified_by);
    }
    
    /// Update the full hierarchical name
    fn update_full_name(&mut self, parent_name: Option<&str>) {
        if let Some(parent) = parent_name {
            self.full_name = format!("{}, {}", self.name, parent);
        } else {
            self.full_name = self.name.clone();
        }
    }
    
    /// Check if this location contains another (hierarchically)
    pub fn contains(&self, location_id: &EntityId) -> bool {
        self.child_locations.contains(location_id)
    }
    
    /// Get all names (current + alternatives)
    pub fn all_names(&self) -> Vec<&str> {
        let mut names = vec![self.name.as_str()];
        names.extend(self.alternative_names.iter().map(|an| an.name.as_str()));
        names
    }
    
    /// Check if location existed at a specific date
    pub fn existed_at(&self, date: &DateTime<Utc>) -> bool {
        if self.historical_periods.is_empty() {
            return true; // Assume always existed if no periods defined
        }
        
        self.historical_periods.iter().any(|period| {
            let after_start = period.start_date.map_or(true, |start| date >= &start);
            let before_end = period.end_date.map_or(true, |end| date <= &end);
            after_start && before_end
        })
    }
    
    /// Get name at a specific date
    pub fn name_at(&self, date: &DateTime<Utc>) -> Option<&str> {
        for period in &self.historical_periods {
            let after_start = period.start_date.map_or(true, |start| date >= &start);
            let before_end = period.end_date.map_or(true, |end| date <= &end);
            if after_start && before_end {
                return Some(&period.name);
            }
        }
        Some(&self.name) // Default to current name
    }
}

impl_entity!(Location, "Location");
impl_validatable!(Location);

#[async_trait]
impl NestableEntity for Location {
    fn children(&self) -> Vec<EntityId> {
        self.child_locations.clone()
    }
    
    fn can_contain(&self, entity_type: &str) -> bool {
        // Locations can contain other locations
        entity_type == "Location"
    }
    
    async fn add_child(&mut self, child_id: EntityId) -> Result<()> {
        if !self.child_locations.contains(&child_id) {
            self.child_locations.push(child_id);
            self.metadata.update(self.metadata.modified_by);
        }
        Ok(())
    }
    
    async fn remove_child(&mut self, child_id: EntityId) -> Result<bool> {
        let initial_len = self.child_locations.len();
        self.child_locations.retain(|id| id != &child_id);
        if self.child_locations.len() < initial_len {
            self.metadata.update(self.metadata.modified_by);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_location_creation() {
        let creator_id = EntityId::new();
        let location = Location::new(
            LocationType::City,
            "Boston",
            creator_id,
        );
        
        assert_eq!(location.name, "Boston");
        assert_eq!(location.full_name, "Boston");
        assert_eq!(location.location_type, LocationType::City);
    }
    
    #[test]
    fn test_location_with_coordinates() {
        let creator_id = EntityId::new();
        let coords = Coordinates {
            latitude: 42.3601,
            longitude: -71.0589,
            altitude: Some(43.0),
            precision: 10.0,
        };
        
        let location = Location::with_coordinates(
            LocationType::City,
            "Boston",
            coords,
            creator_id,
        );
        
        assert!(location.coordinates.is_some());
        assert_eq!(location.coordinates.unwrap().latitude, 42.3601);
    }
    
    #[test]
    fn test_alternative_names() {
        let creator_id = EntityId::new();
        let mut location = Location::new(
            LocationType::City,
            "New York",
            creator_id,
        );
        
        let alt_name = AlternativeName {
            name: "New Amsterdam".to_string(),
            language: Some("English".to_string()),
            name_type: "Historical".to_string(),
            period: None,
            source: Some("Historical records".to_string()),
        };
        
        location.add_alternative_name(alt_name);
        assert_eq!(location.alternative_names.len(), 1);
        assert_eq!(location.all_names().len(), 2);
    }
    
    #[test]
    fn test_hierarchical_names() {
        let creator_id = EntityId::new();
        let parent_id = EntityId::new();
        let mut location = Location::new(
            LocationType::City,
            "Boston",
            creator_id,
        );
        
        location.set_parent_location(parent_id, "Massachusetts, USA");
        assert_eq!(location.full_name, "Boston, Massachusetts, USA");
    }
    
    #[test]
    fn test_historical_periods() {
        let creator_id = EntityId::new();
        let mut location = Location::new(
            LocationType::City,
            "Istanbul",
            creator_id,
        );
        
        let byzantine_period = LocationPeriod {
            start_date: None,
            end_date: DateTime::parse_from_rfc3339("1453-05-29T00:00:00Z")
                .ok()
                .map(|dt| dt.with_timezone(&Utc)),
            name: "Constantinople".to_string(),
            jurisdiction: Some("Byzantine Empire".to_string()),
            notes: None,
        };
        
        let ottoman_period = LocationPeriod {
            start_date: DateTime::parse_from_rfc3339("1453-05-29T00:00:00Z")
                .ok()
                .map(|dt| dt.with_timezone(&Utc)),
            end_date: DateTime::parse_from_rfc3339("1923-10-29T00:00:00Z")
                .ok()
                .map(|dt| dt.with_timezone(&Utc)),
            name: "Konstantiniyye".to_string(),
            jurisdiction: Some("Ottoman Empire".to_string()),
            notes: None,
        };
        
        location.add_historical_period(byzantine_period);
        location.add_historical_period(ottoman_period);
        
        assert_eq!(location.historical_periods.len(), 2);
        
        // Test name at different dates
        let date_1450 = DateTime::parse_from_rfc3339("1450-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(location.name_at(&date_1450), Some("Constantinople"));
        
        let date_1500 = DateTime::parse_from_rfc3339("1500-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        assert_eq!(location.name_at(&date_1500), Some("Konstantiniyye"));
    }
    
    #[tokio::test]
    async fn test_location_nesting() {
        let creator_id = EntityId::new();
        let mut country = Location::new(
            LocationType::Country,
            "United States",
            creator_id,
        );
        
        let state_id = EntityId::new();
        assert!(country.add_child(state_id).await.is_ok());
        assert!(country.contains(&state_id));
        
        assert!(country.remove_child(state_id).await.unwrap());
        assert!(!country.contains(&state_id));
    }
    
    #[test]
    fn test_location_validation() {
        let creator_id = EntityId::new();
        let location = Location::new(
            LocationType::City,
            "Boston",
            creator_id,
        );
        
        assert!(validator::Validate::validate(&location).is_ok());
    }
}