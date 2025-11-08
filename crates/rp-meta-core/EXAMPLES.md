# rp-meta-core Examples

This document provides examples of using the universal meta-model primitives.

## Table of Contents

1. [Basic Entity Creation](#basic-entity-creation)
2. [Genealogical Entities](#genealogical-entities)
3. [Relationships](#relationships)
4. [Contexts and Scoping](#contexts-and-scoping)
5. [Certainty Models](#certainty-models)
6. [Property Graphs](#property-graphs)
7. [Temporal Values](#temporal-values)
8. [Spatial Values](#spatial-values)
9. [Theory Versioning](#theory-versioning)

## Basic Entity Creation

```rust
use rp_meta_core::prelude::*;

// Create a simple entity
let person = Entity::new("Person")
    .with_state("Active")
    .with_property("name", Property::Value(Value::Text("John Smith".to_string())))
    .with_property("age", Property::Value(Value::Integer(42)));

assert_eq!(person.entity_type, "Person");
assert_eq!(person.get_property("name").unwrap(),
    &Property::Value(Value::Text("John Smith".to_string())));
```

## Genealogical Entities

```rust
use rp_meta_core::prelude::*;

// Create an IdentityPersona entity (genealogy domain)
let identity = Entity::new("IdentityPersona")
    .with_state("Hypothesis")
    .with_property("given_names", Property::Collection(vec![
        Property::Value(Value::Text("John".to_string())),
        Property::Value(Value::Text("William".to_string())),
    ]))
    .with_property("surname", Property::Value(Value::Text("Smith".to_string())))
    .with_property("birth_date", Property::Value(Value::Temporal(
        TemporalValue::date(1850, Some(3), Some(15))
    )))
    .with_property("birth_place", Property::Value(Value::Spatial(
        SpatialValue::named("Boston, MA")
    )));

// Create a Theory entity
let theory = Entity::new("Theory")
    .with_state("Active")
    .with_property("question", Property::Value(Value::Text(
        "Did John Smith migrate from Boston to Chicago in 1875?".to_string()
    )))
    .with_property("priority", Property::Value(Value::Integer(1)));

// Create an Evidence entity
let evidence = Entity::new("Evidence")
    .with_state("Verified")
    .with_property("title", Property::Value(Value::Text(
        "1850 U.S. Census - Boston, MA".to_string()
    )))
    .with_property("record_type", Property::Value(Value::Text("Census".to_string())))
    .with_property("source_quality", Property::Value(Value::Text("Primary".to_string())));
```

## Relationships

### Binary Relationship

```rust
use rp_meta_core::prelude::*;

let parent_id = EntityId::new();
let child_id = EntityId::new();

// Create a parent-child relationship
let relationship = Relationship::binary(
    "Kinship.Parent",
    parent_id,
    "parent",
    child_id,
    "child"
)
.with_property("biological", Property::Value(Value::Boolean(true)))
.with_property("adoptive", Property::Value(Value::Boolean(false)));
```

### N-ary Relationship

```rust
use rp_meta_core::prelude::*;

// DNA match involving person, sample, and laboratory (3-way relationship)
let person_id = EntityId::new();
let sample_id = EntityId::new();
let lab_id = EntityId::new();

let dna_match = Relationship::new("DNA.Match")
    .with_participant(Participant::new(person_id, "person"))
    .with_participant(Participant::new(sample_id, "sample"))
    .with_participant(Participant::new(lab_id, "laboratory"))
    .with_property("match_quality", Property::Value(Value::Float(98.5)))
    .with_property("centimorgans", Property::Value(Value::Float(1825.0)));

assert_eq!(dna_match.participants.len(), 3);
```

### Identity Correlation

```rust
use rp_meta_core::prelude::*;

// Two personas that might be the same person
let persona1_id = EntityId::new();
let persona2_id = EntityId::new();

let identity_link = Relationship::new("Identity.PossibleSame")
    .with_participant(
        Participant::new(persona1_id, "identity_a")
            .with_certainty(Certainty::quantum(vec![
                ("same_person", 0.75),
                ("different_person", 0.25),
            ]))
    )
    .with_participant(Participant::new(persona2_id, "identity_b"));
```

## Contexts and Scoping

### Temporal Context

```rust
use rp_meta_core::prelude::*;

let temporal_ctx = Context::temporal("19th century", None)
    .with_certainty(Certainty::probability(0.9));

let entity = Entity::new("Event")
    .with_context(temporal_ctx);
```

### Spatial Context

```rust
use rp_meta_core::prelude::*;

let spatial_ctx = Context::spatial("Boston, Massachusetts");

let entity = Entity::new("ResearchActivity")
    .with_context(spatial_ctx);
```

### Theoretical Context

```rust
use rp_meta_core::prelude::*;

// Same relationship exists in multiple theories
let theory_a_ctx = Context::theoretical("Migration Theory A", vec![
    "Family moved westward in 1875".to_string(),
]);

let theory_b_ctx = Context::theoretical("Migration Theory B", vec![
    "Family remained in Boston until 1890".to_string(),
]);

// In Theory A: John married Mary
let rel_a = Relationship::binary(
    "Kinship.Spouse",
    john_id,
    "husband",
    mary_id,
    "wife"
)
.with_context(theory_a_ctx);

// In Theory B: John married Sarah
let rel_b = Relationship::binary(
    "Kinship.Spouse",
    john_id,
    "husband",
    sarah_id,
    "wife"
)
.with_context(theory_b_ctx);

// Both relationships can coexist, scoped by different theories!
```

### Evidential Context

```rust
use rp_meta_core::prelude::*;

let evidential_ctx = Context::evidential("Census Record", "Primary")
    .with_property("repository", Property::Value(Value::Text(
        "National Archives".to_string()
    )));

let entity = Entity::new("Fact")
    .with_context(evidential_ctx);
```

## Certainty Models

### Quantum Superposition

```rust
use rp_meta_core::prelude::*;

// Multiple possible states with probabilities
let certainty = Certainty::quantum(vec![
    ("same_person", 0.75),
    ("different_person", 0.20),
    ("insufficient_evidence", 0.05),
]);

// Get most likely state
let (state, probability) = certainty.most_likely().unwrap();
assert_eq!(state, "same_person");
assert_eq!(probability, 0.75);
```

### Fuzzy Logic

```rust
use rp_meta_core::prelude::*;

let certainty = Certainty::fuzzy(
    0.8,  // membership (how much it belongs to the category)
    0.9   // confidence in the membership value
);
```

### Bayesian Probability

```rust
use rp_meta_core::prelude::*;

let certainty = Certainty::bayesian(
    0.5,  // prior probability
    0.8,  // likelihood
    0.75  // posterior probability
);
```

### Narrative Explanation

```rust
use rp_meta_core::prelude::*;

let certainty = Certainty::narrative(
    "Likely the same person based on matching birth location and timeframe, \
     but surname spelling differs slightly in records."
);
```

### Simple Probability

```rust
use rp_meta_core::prelude::*;

let certainty = Certainty::probability(0.85);

assert!(certainty.is_certain(0.8));  // true
assert!(!certainty.is_certain(0.9)); // false
```

## Property Graphs

### Basic Properties

```rust
use rp_meta_core::prelude::*;

let mut props = PropertyGraph::new();
props.set_text("name", "John Smith");
props.set_integer("age", 42);
props.set_bool("verified", true);
props.set_float("confidence", 0.85);

assert_eq!(props.get_text("name").unwrap(), "John Smith");
assert_eq!(props.get_integer("age").unwrap(), 42);
```

### Nested Properties

```rust
use rp_meta_core::prelude::*;

let mut address = PropertyGraph::new();
address.set_text("street", "123 Main St");
address.set_text("city", "Boston");
address.set_text("state", "MA");

let mut props = PropertyGraph::new();
props.set("address", Property::Map(address.properties));
```

### Collections

```rust
use rp_meta_core::prelude::*;

let tags = vec![
    Property::Value(Value::Text("genealogy".to_string())),
    Property::Value(Value::Text("migration".to_string())),
    Property::Value(Value::Text("19th-century".to_string())),
];

let mut props = PropertyGraph::new();
props.set_collection("tags", tags);
```

### Entity References

```rust
use rp_meta_core::prelude::*;

let theory_id = EntityId::new();
let evidence_id = EntityId::new();

let mut props = PropertyGraph::new();
props.set_reference("theory", theory_id);
props.set_reference("evidence", evidence_id);

assert_eq!(props.get_reference("theory").unwrap(), theory_id);
```

### Theoretical Values

```rust
use rp_meta_core::prelude::*;

let theoretical = TheoreticalValue {
    hypothesis: "John was born in Boston".to_string(),
    supporting_evidence: vec![evidence1_id, evidence2_id],
    contradicting_evidence: vec![],
    probability: 0.75,
};

let prop = Property::Theoretical(theoretical);
```

### Quantum Properties

```rust
use rp_meta_core::prelude::*;

// Birth location could be multiple places
let birth_location = Property::Quantum(vec![
    (Property::Value(Value::Text("Boston".to_string())), 0.6),
    (Property::Value(Value::Text("Cambridge".to_string())), 0.3),
    (Property::Value(Value::Text("Salem".to_string())), 0.1),
]);

let mut props = PropertyGraph::new();
props.set("birth_location", birth_location);
```

## Temporal Values

### Simple Date

```rust
use rp_meta_core::prelude::*;

let date = TemporalValue::date(1850, Some(3), Some(15));
```

### Approximate Date

```rust
use rp_meta_core::prelude::*;

let circa = TemporalValue::circa(1850);
```

### Date Range

```rust
use rp_meta_core::prelude::*;

let from = TemporalInstant::gregorian(1800, Some(1), Some(1));
let to = TemporalInstant::gregorian(1850, Some(12), Some(31));
let range = TemporalValue::range(from, to);
```

### Dual Dating (Calendar Transition)

```rust
use rp_meta_core::prelude::*;

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

let instant = TemporalInstant {
    expressions: vec![dual],
    precision: TemporalPrecision::Day,
    quality: TemporalQuality::Exact,
};
```

### Narrative Time

```rust
use rp_meta_core::prelude::*;

let time = TemporalValue::narrative("when the cherry blossoms bloomed");
```

### Hebrew Calendar

```rust
use rp_meta_core::prelude::*;

let hebrew = CalendarExpression::Hebrew {
    year: 5610,
    month: "Adar".to_string(),
    day: Some(15),
};

let instant = TemporalInstant {
    expressions: vec![hebrew],
    precision: TemporalPrecision::Day,
    quality: TemporalQuality::Exact,
};
```

## Spatial Values

### Lat/Long Coordinates

```rust
use rp_meta_core::prelude::*;

let location = SpatialValue::point(42.3601, -71.0589); // Boston
```

### Named Location

```rust
use rp_meta_core::prelude::*;

let location = SpatialValue::named("Boston, Massachusetts");
```

### Relative Location

```rust
use rp_meta_core::prelude::*;

let location = SpatialValue::Relative {
    anchor: SpatialReference {
        reference_type: "named".to_string(),
        reference: "Boston".to_string(),
    },
    relation: "10 miles north of".to_string(),
    distance: Some(Quantity {
        value: 10.0,
        unit: "miles".to_string(),
        precision: Some(2.0),
    }),
};
```

### Uncertain Location

```rust
use rp_meta_core::prelude::*;

let location = SpatialValue::Uncertain {
    possibilities: vec![
        (SpatialValue::named("Boston"), 0.6),
        (SpatialValue::named("Cambridge"), 0.3),
        (SpatialValue::named("Salem"), 0.1),
    ],
};
```

### Narrative Location

```rust
use rp_meta_core::prelude::*;

let location = SpatialValue::narrative("where the old oak tree stood");
```

## Theory Versioning

One of the most powerful features is the ability to have multiple theories coexist:

```rust
use rp_meta_core::prelude::*;

// Create two different theories about John's migration
let theory_a = Entity::new("Theory")
    .with_state("Active")
    .with_property("name", Property::Value(Value::Text(
        "Early Migration Theory".to_string()
    )));

let theory_b = Entity::new("Theory")
    .with_state("Active")
    .with_property("name", Property::Value(Value::Text(
        "Late Migration Theory".to_string()
    )));

// John's family structure in Theory A
let john_mary_marriage = Relationship::binary(
    "Kinship.Spouse",
    john_id,
    "husband",
    mary_id,
    "wife"
)
.with_context(Context::theoretical("Early Migration Theory", vec![
    "John married Mary in Boston in 1848".to_string(),
    "They had 3 children before migrating".to_string(),
]))
.with_property("marriage_date", Property::Value(Value::Temporal(
    TemporalValue::date(1848, Some(6), Some(12))
)))
.with_property("marriage_place", Property::Value(Value::Spatial(
    SpatialValue::named("Boston, MA")
)));

// John's family structure in Theory B
let john_sarah_marriage = Relationship::binary(
    "Kinship.Spouse",
    john_id,
    "husband",
    sarah_id,
    "wife"
)
.with_context(Context::theoretical("Late Migration Theory", vec![
    "John's first wife died, he remarried Sarah in Chicago".to_string(),
]))
.with_property("marriage_date", Property::Value(Value::Temporal(
    TemporalValue::date(1876, Some(9), Some(3))
)))
.with_property("marriage_place", Property::Value(Value::Spatial(
    SpatialValue::named("Chicago, IL")
)));

// Both relationships exist simultaneously!
// Query by context to see different theories
```

## Complete Example: Genealogical Research

```rust
use rp_meta_core::prelude::*;

// Create researcher
let researcher = Entity::new("Researcher")
    .with_property("name", Property::Value(Value::Text("Alice Johnson".to_string())))
    .with_property("certification", Property::Value(Value::Text("BCG Certified".to_string())));

// Create theory
let theory = Entity::new("Theory")
    .with_state("Active")
    .with_property("question", Property::Value(Value::Text(
        "Did John Smith and John Smythe refer to the same person?".to_string()
    )));

// Create two personas
let persona1 = Entity::new("IdentityPersona")
    .with_state("Hypothesis")
    .with_property("name", Property::Value(Value::Text("John Smith".to_string())))
    .with_property("birth_date", Property::Value(Value::Temporal(
        TemporalValue::circa(1850)
    )))
    .with_property("birth_place", Property::Value(Value::Spatial(
        SpatialValue::named("Boston, MA")
    )));

let persona2 = Entity::new("IdentityPersona")
    .with_state("Hypothesis")
    .with_property("name", Property::Value(Value::Text("John Smythe".to_string())))
    .with_property("birth_date", Property::Value(Value::Temporal(
        TemporalValue::date(1851, Some(3), Some(15))
    )))
    .with_property("birth_place", Property::Value(Value::Spatial(
        SpatialValue::named("Boston, MA")
    )));

// Create evidence
let census = Entity::new("Evidence")
    .with_state("Verified")
    .with_property("title", Property::Value(Value::Text(
        "1850 U.S. Census - Boston".to_string()
    )))
    .with_property("source_quality", Property::Value(Value::Text("Primary".to_string())));

// Create identity correlation with quantum certainty
let correlation = Relationship::new("Identity.PossibleSame")
    .with_participant(
        Participant::new(persona1.id, "identity_a")
            .with_certainty(Certainty::quantum(vec![
                ("same_person", 0.70),
                ("different_person", 0.25),
                ("uncertain", 0.05),
            ]))
            .with_property("supporting_evidence", Property::Reference(census.id))
    )
    .with_participant(
        Participant::new(persona2.id, "identity_b")
    )
    .with_context(Context::theoretical("Identity Correlation Analysis", vec![
        "Name variants common in period".to_string(),
        "Same location and approximate timeframe".to_string(),
    ]))
    .with_context(Context::evidential("Census Records", "Primary"));

// The correlation expresses:
// - 70% probability they're the same person
// - 25% probability they're different people
// - 5% insufficient evidence to determine
// - Evidence: 1850 Census
// - Context: Theoretical analysis + Primary source evidence
```

This example demonstrates the power of the meta-model: complex genealogical research
scenarios with uncertainty, multiple theories, and proper evidence tracking.
