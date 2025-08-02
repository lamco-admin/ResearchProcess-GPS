// Theoretical Meta-Model for Universal Genealogical Expression
// This model can express ANY genealogical data model through its primitives

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The four fundamental primitives of the meta-model
/// 1. Entity - Universal container for anything
/// 2. Relationship - Universal connection between entities
/// 3. Context - Universal qualifier/scope
/// 4. Certainty - Universal uncertainty expression

// === IDENTIFIERS ===
// Everything needs unique identification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelationshipId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextId(Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropertyId(Uuid);

// === PRIMITIVE 1: ENTITY ===
// The universal container that can represent anything
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,

    // Type is just a string - infinite types possible
    // Examples: "Identity.Named", "Event.Birth", "Theory.Migration", "Quantum.Superposition"
    pub entity_type: String,

    // State is just a string - infinite states possible
    // Examples: "Hypothetical", "Observed", "Documented", "Theoretical", "Disproven"
    pub state: String,

    // Properties can contain anything, including other entities
    pub properties: PropertyGraph,

    // Relationships this entity participates in
    pub relationships: Vec<RelationshipId>,

    // Contexts that qualify this entity
    pub contexts: Vec<Context>,

    // Meta-information
    pub meta: MetaInfo,
}

// === PRIMITIVE 2: RELATIONSHIP ===
// Universal connection between any number of entities
#[derive(Debug, Clone)]
pub struct Relationship {
    pub id: RelationshipId,

    // Type is open-ended
    // Examples: "Identity.PossibleSame", "Kinship.Parent", "Temporal.Before", "Causal.CausedBy"
    pub relationship_type: String,

    // N-ary relationships (not just binary)
    pub participants: Vec<Participant>,

    // Relationships can have properties too
    pub properties: PropertyGraph,

    // Contexts qualifying this relationship
    pub contexts: Vec<Context>,

    // Meta-information
    pub meta: MetaInfo,
}

#[derive(Debug, Clone)]
pub struct Participant {
    pub entity: EntityId,

    // Role is open-ended
    // Examples: "subject", "object", "witness", "source", "identity_a", "parent", "child"
    pub role: String,

    // Each participant can have different certainty
    pub certainty: Certainty,

    // Participant-specific contexts
    pub contexts: Vec<Context>,
}

// === PRIMITIVE 3: CONTEXT ===
// Universal qualifier that can scope/qualify anything
#[derive(Debug, Clone)]
pub struct Context {
    pub id: ContextId,

    // Context type is open-ended
    // Examples: "Temporal", "Spatial", "Cultural", "Theoretical", "Research", "Source"
    pub context_type: String,

    // What this context applies to
    pub scope: Scope,

    // Certainty of this context
    pub certainty: Certainty,

    // Contexts can have properties
    pub properties: PropertyGraph,
}

#[derive(Debug, Clone)]
pub enum Scope {
    // Time-related scope
    Temporal(TemporalScope),

    // Space-related scope
    Spatial(SpatialScope),

    // Culture/society scope
    Cultural(CulturalScope),

    // Theoretical/research scope
    Theoretical(TheoreticalScope),

    // Source/evidence scope
    Evidential(EvidentialScope),

    // Combination of scopes
    Composite(Vec<Scope>),

    // Open-ended scope
    Custom(String, PropertyGraph),
}

// === PRIMITIVE 4: CERTAINTY ===
// Universal uncertainty expression - NOT just a number
#[derive(Debug, Clone)]
pub enum Certainty {
    // Quantum superposition - multiple states with probabilities
    Quantum(Vec<(String, f64)>), // e.g., [("same_person", 0.75), ("different_person", 0.25)]

    // Fuzzy logic expression
    Fuzzy {
        membership: f64,      // 0.0 to 1.0
        confidence: f64,      // confidence in the membership value
    },

    // Bayesian probability network
    Bayesian {
        prior: f64,
        likelihood: f64,
        posterior: f64,
    },

    // Narrative explanation
    Narrative(String),

    // Logical expression
    Logical(LogicalExpression),

    // Explicitly unknown
    Unknown,

    // Composite certainty
    Composite(Vec<Certainty>),
}

// === PROPERTY SYSTEM ===
// The infinitely flexible property graph
#[derive(Debug, Clone)]
pub struct PropertyGraph {
    properties: HashMap<String, Property>,
}

#[derive(Debug, Clone)]
pub enum Property {
    // Simple values
    Value(Value),

    // Nested entity (infinite nesting possible)
    Entity(Box<Entity>),

    // Computed/derived property
    Computation(Computation),

    // Theoretical/possible values
    Theoretical(TheoreticalValue),

    // Quantum superposition of properties
    Quantum(Vec<(Property, f64)>),

    // Reference to another entity
    Reference(EntityId),

    // Collection of properties
    Collection(Vec<Property>),

    // Property map (nested properties)
    Map(HashMap<String, Property>),
}

#[derive(Debug, Clone)]
pub enum Value {
    // Basic types
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    Binary(Vec<u8>),

    // Temporal values (calendar-agnostic)
    Temporal(TemporalValue),

    // Spatial values (coordinate-system-agnostic)
    Spatial(SpatialValue),

    // Identifiers
    Identifier(String, String), // (system, id)

    // Quantities with units
    Quantity(f64, String), // (value, unit)

    // Structured data
    Json(serde_json::Value),

    // Function/expression that produces a value
    Expression(String), // Expression language TBD
}

// === TEMPORAL ABSTRACTION ===
// Not tied to any specific calendar system
#[derive(Debug, Clone)]
pub enum TemporalValue {
    // Single moment in time
    Instant(TemporalInstant),

    // Time range
    Range(TemporalInstant, TemporalInstant),

    // Before/after constraints
    Before(TemporalInstant),
    After(TemporalInstant),

    // Relative to another time
    Relative {
        anchor: TemporalReference,
        offset: Duration,
    },

    // Recurring time pattern
    Recurring(TemporalPattern),

    // Uncertain time
    Uncertain {
        possibilities: Vec<(TemporalValue, f64)>,
        constraints: Vec<TemporalConstraint>,
    },

    // Narrative time description
    Narrative(String), // "when the cherry blossoms bloomed"
}

#[derive(Debug, Clone)]
pub struct TemporalInstant {
    // Can be expressed in multiple calendar systems simultaneously
    expressions: Vec<CalendarExpression>,

    // Precision of this instant
    precision: TemporalPrecision,

    // Quality of the temporal information
    quality: TemporalQuality,
}

#[derive(Debug, Clone)]
pub enum CalendarExpression {
    // Standard calendars
    Gregorian { year: i32, month: Option<u8>, day: Option<u8> },
    Julian { year: i32, month: Option<u8>, day: Option<u8> },

    // Dual dating for calendar transitions
    DualDated {
        julian: Box<CalendarExpression>,
        gregorian: Box<CalendarExpression>,
    },

    // Other calendar systems
    Hebrew { year: i32, month: String, day: Option<u8> },
    Islamic { year: i32, month: String, day: Option<u8> },
    FrenchRepublican { year: i32, month: String, day: Option<u8> },

    // Relative expression
    Relative { description: String }, // "third year of the reign of..."

    // Custom calendar
    Custom {
        calendar_system: String,
        expression: String,
    },
}

// === SPATIAL ABSTRACTION ===
// Not tied to any specific coordinate system
#[derive(Debug, Clone)]
pub enum SpatialValue {
    // Point in space
    Point(SpatialPoint),

    // Area/region
    Region(SpatialRegion),

    // Path/route
    Path(Vec<SpatialPoint>),

    // Relative location
    Relative {
        anchor: SpatialReference,
        relation: String, // "north of", "near", "within"
        distance: Option<Quantity>,
    },

    // Uncertain location
    Uncertain {
        possibilities: Vec<(SpatialValue, f64)>,
    },

    // Narrative location
    Narrative(String), // "where the old oak tree stood"
}

#[derive(Debug, Clone)]
pub struct SpatialPoint {
    // Can be expressed in multiple coordinate systems
    expressions: Vec<CoordinateExpression>,

    // Precision/accuracy
    precision: SpatialPrecision,
}

#[derive(Debug, Clone)]
pub enum CoordinateExpression {
    // Standard systems
    LatLong { latitude: f64, longitude: f64, datum: String },
    UTM { zone: u8, easting: f64, northing: f64 },

    // Historical/custom systems
    Custom {
        system: String,
        coordinates: Vec<f64>,
    },

    // Named location
    Named {
        name: String,
        authority: String, // "geonames", "historic-gazetteer", etc.
    },
}

// === META INFORMATION ===
// Tracks the meta-aspects of data
#[derive(Debug, Clone)]
pub struct MetaInfo {
    // When this was created/modified
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,

    // Who created/modified (can be entity IDs)
    pub created_by: Option<EntityId>,
    pub modified_by: Option<EntityId>,

    // Version/change tracking
    pub version: u64,
    pub previous_version: Option<EntityId>,

    // Why this exists
    pub rationale: Option<String>,

    // Quality/review status
    pub review_status: Option<String>,

    // Custom metadata
    pub custom: HashMap<String, Value>,
}

// === HELPER ENUMS ===

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum TemporalQuality {
    Exact,
    Approximate,
    Estimated,
    Calculated,
    Interpreted,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Duration {
    pub value: f64,
    pub unit: String, // "days", "years", "generations", etc.
}

#[derive(Debug, Clone)]
pub enum TemporalReference {
    Entity(EntityId),
    Instant(Box<TemporalInstant>),
    Narrative(String),
}

#[derive(Debug, Clone)]
pub struct TemporalPattern {
    pub pattern_type: String, // "annual", "seasonal", "generational"
    pub parameters: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct TemporalConstraint {
    pub constraint_type: String, // "must_be_before", "must_be_after"
    pub reference: TemporalReference,
}

#[derive(Debug, Clone)]
pub struct TemporalScope {
    pub description: String,
    pub bounds: Option<(TemporalValue, TemporalValue)>,
}

#[derive(Debug, Clone)]
pub struct SpatialScope {
    pub description: String,
    pub bounds: Option<SpatialRegion>,
}

#[derive(Debug, Clone)]
pub struct CulturalScope {
    pub culture: String,
    pub period: Option<TemporalScope>,
    pub region: Option<SpatialScope>,
}

#[derive(Debug, Clone)]
pub struct TheoreticalScope {
    pub theory: String,
    pub assumptions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EvidentialScope {
    pub source_type: String,
    pub quality: String,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SpatialRegion {
    pub region_type: String, // "polygon", "circle", "administrative"
    pub definition: PropertyGraph,
}

#[derive(Debug, Clone)]
pub struct SpatialReference {
    pub reference_type: String,
    pub reference: String,
}

#[derive(Debug, Clone)]
pub struct SpatialPrecision {
    pub value: f64,
    pub unit: String, // "meters", "miles", etc.
}

#[derive(Debug, Clone)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub precision: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum LogicalExpression {
    And(Vec<LogicalExpression>),
    Or(Vec<LogicalExpression>),
    Not(Box<LogicalExpression>),
    Implies(Box<LogicalExpression>, Box<LogicalExpression>),
    Predicate(String, Vec<Value>),
}

#[derive(Debug, Clone)]
pub struct Computation {
    pub expression: String,
    pub language: String, // "javascript", "python", "prolog", etc.
    pub dependencies: Vec<PropertyId>,
}

#[derive(Debug, Clone)]
pub struct TheoreticalValue {
    pub hypothesis: String,
    pub supporting_evidence: Vec<EntityId>,
    pub contradicting_evidence: Vec<EntityId>,
    pub probability: f64,
}

// === IMPLEMENTATION HELPERS ===

impl EntityId {
    pub fn new() -> Self {
        EntityId(Uuid::new_v4())
    }
}

impl RelationshipId {
    pub fn new() -> Self {
        RelationshipId(Uuid::new_v4())
    }
}

impl ContextId {
    pub fn new() -> Self {
        ContextId(Uuid::new_v4())
    }
}

impl PropertyId {
    pub fn new() -> Self {
        PropertyId(Uuid::new_v4())
    }
}

impl PropertyGraph {
    pub fn new() -> Self {
        PropertyGraph {
            properties: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: Property) {
        self.properties.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Property> {
        self.properties.get(key)
    }

    // Helper methods for common operations
    pub fn set_text(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.set(key, Property::Value(Value::Text(value.into())));
    }

    pub fn set_reference(&mut self, key: impl Into<String>, entity_id: EntityId) {
        self.set(key, Property::Reference(entity_id));
    }

    pub fn set_entity(&mut self, key: impl Into<String>, entity: Entity) {
        self.set(key, Property::Entity(Box::new(entity)));
    }
}

impl Entity {
    pub fn new(entity_type: impl Into<String>) -> Self {
        Entity {
            id: EntityId::new(),
            entity_type: entity_type.into(),
            state: "Initial".to_string(),
            properties: PropertyGraph::new(),
            relationships: Vec::new(),
            contexts: Vec::new(),
            meta: MetaInfo::now(),
        }
    }
}

impl MetaInfo {
    pub fn now() -> Self {
        let now = Utc::now();
        MetaInfo {
            created: now,
            modified: now,
            created_by: None,
            modified_by: None,
            version: 1,
            previous_version: None,
            rationale: None,
            review_status: None,
            custom: HashMap::new(),
        }
    }
}