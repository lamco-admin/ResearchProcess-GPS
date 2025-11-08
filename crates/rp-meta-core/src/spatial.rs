//! Coordinate-agnostic spatial values
//!
//! Supports multiple coordinate systems, uncertain locations, relative positions,
//! and narrative location descriptions.

use serde::{Deserialize, Serialize};

use crate::PropertyGraph;

/// Coordinate-agnostic spatial value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum SpatialValue {
    /// Point in space
    Point(SpatialPoint),

    /// Area/region
    Region(SpatialRegion),

    /// Path/route
    Path(Vec<SpatialPoint>),

    /// Relative location
    Relative {
        anchor: SpatialReference,
        relation: String, // "north of", "near", "within"
        distance: Option<Quantity>,
    },

    /// Uncertain location with multiple possibilities
    Uncertain {
        possibilities: Vec<(SpatialValue, f64)>,
    },

    /// Narrative location
    Narrative(String),
}

/// A specific point in space
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpatialPoint {
    /// Coordinate expressions (can be multiple systems)
    pub expressions: Vec<CoordinateExpression>,

    /// Precision/accuracy
    pub precision: SpatialPrecision,
}

impl SpatialPoint {
    /// Create a point from latitude/longitude
    pub fn lat_long(latitude: f64, longitude: f64, datum: impl Into<String>) -> Self {
        Self {
            expressions: vec![CoordinateExpression::LatLong {
                latitude,
                longitude,
                datum: datum.into(),
            }],
            precision: SpatialPrecision {
                value: 10.0,
                unit: "meters".to_string(),
            },
        }
    }

    /// Create a named point
    pub fn named(name: impl Into<String>, authority: impl Into<String>) -> Self {
        Self {
            expressions: vec![CoordinateExpression::Named {
                name: name.into(),
                authority: authority.into(),
            }],
            precision: SpatialPrecision {
                value: 1000.0,
                unit: "meters".to_string(),
            },
        }
    }
}

/// Coordinate system expressions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "system", content = "coordinates")]
pub enum CoordinateExpression {
    /// Latitude/Longitude
    LatLong {
        latitude: f64,
        longitude: f64,
        datum: String, // "WGS84", "NAD83", etc.
    },

    /// Universal Transverse Mercator
    UTM {
        zone: u8,
        easting: f64,
        northing: f64,
    },

    /// Custom coordinate system
    Custom {
        system: String,
        coordinates: Vec<f64>,
    },

    /// Named location
    Named {
        name: String,
        authority: String, // "geonames", "historic-gazetteer", etc.
    },
}

/// Spatial region
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpatialRegion {
    pub region_type: String, // "polygon", "circle", "administrative"
    pub definition: PropertyGraph,
}

/// Spatial precision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpatialPrecision {
    pub value: f64,
    pub unit: String, // "meters", "miles", etc.
}

/// Spatial reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpatialReference {
    pub reference_type: String,
    pub reference: String,
}

/// Spatial scope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpatialScope {
    pub description: String,
    pub bounds: Option<SpatialRegion>,
}

/// Quantity with unit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub precision: Option<f64>,
}

impl SpatialValue {
    /// Create a point from lat/long
    pub fn point(latitude: f64, longitude: f64) -> Self {
        Self::Point(SpatialPoint::lat_long(latitude, longitude, "WGS84"))
    }

    /// Create a named location
    pub fn named(name: impl Into<String>) -> Self {
        Self::Point(SpatialPoint::named(name, "geonames"))
    }

    /// Create a narrative location
    pub fn narrative(description: impl Into<String>) -> Self {
        Self::Narrative(description.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_point_lat_long() {
        let point = SpatialPoint::lat_long(42.3601, -71.0589, "WGS84");
        assert_eq!(point.expressions.len(), 1);

        match &point.expressions[0] {
            CoordinateExpression::LatLong {
                latitude,
                longitude,
                datum,
            } => {
                assert_eq!(*latitude, 42.3601);
                assert_eq!(*longitude, -71.0589);
                assert_eq!(datum, "WGS84");
            }
            _ => panic!("Expected LatLong"),
        }
    }

    #[test]
    fn test_spatial_point_named() {
        let point = SpatialPoint::named("Boston, MA", "geonames");
        match &point.expressions[0] {
            CoordinateExpression::Named { name, authority } => {
                assert_eq!(name, "Boston, MA");
                assert_eq!(authority, "geonames");
            }
            _ => panic!("Expected Named"),
        }
    }

    #[test]
    fn test_spatial_value_point() {
        let value = SpatialValue::point(42.3601, -71.0589);
        assert!(matches!(value, SpatialValue::Point(_)));
    }

    #[test]
    fn test_spatial_value_named() {
        let value = SpatialValue::named("Boston");
        assert!(matches!(value, SpatialValue::Point(_)));
    }

    #[test]
    fn test_spatial_value_narrative() {
        let value = SpatialValue::narrative("where the old oak tree stood");
        assert!(matches!(value, SpatialValue::Narrative(_)));
    }

    #[test]
    fn test_spatial_value_uncertain() {
        let value = SpatialValue::Uncertain {
            possibilities: vec![
                (SpatialValue::named("Boston"), 0.6),
                (SpatialValue::named("Cambridge"), 0.4),
            ],
        };

        assert!(matches!(value, SpatialValue::Uncertain { .. }));
    }

    #[test]
    fn test_spatial_value_relative() {
        let value = SpatialValue::Relative {
            anchor: SpatialReference {
                reference_type: "named".to_string(),
                reference: "Boston".to_string(),
            },
            relation: "north of".to_string(),
            distance: Some(Quantity {
                value: 10.0,
                unit: "miles".to_string(),
                precision: Some(2.0),
            }),
        };

        assert!(matches!(value, SpatialValue::Relative { .. }));
    }
}
