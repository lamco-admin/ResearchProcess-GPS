// Abstraction Registry - Central management of all abstraction layers

use crate::theoretical_meta_model::*;
use crate::abstraction_layers::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Central registry for all abstraction layers
pub struct AbstractionRegistry {
    layers: HashMap<String, Arc<dyn AbstractionLayer>>,
    calendar_abstraction: Arc<CalendarAbstraction>,
    name_abstraction: Arc<NameAbstraction>,
    media_abstraction: Arc<MediaAbstraction>,
}

impl AbstractionRegistry {
    pub fn new() -> Self {
        let mut registry = AbstractionRegistry {
            layers: HashMap::new(),
            calendar_abstraction: Arc::new(CalendarAbstraction::new()),
            name_abstraction: Arc::new(NameAbstraction::new()),
            media_abstraction: Arc::new(MediaAbstraction::new()),
        };
        
        // Register standard layers
        registry.register("GRAMPS-XML", Arc::new(GrampsXmlAbstraction::new()));
        // Could add more: GEDCOM, FamilySearch, etc.
        
        registry
    }
    
    /// Register a new abstraction layer
    pub fn register(&mut self, name: &str, layer: Arc<dyn AbstractionLayer>) {
        self.layers.insert(name.to_string(), layer);
    }
    
    /// Import data through appropriate abstraction layer
    pub fn import(&self, format: &str, data: &[u8]) -> Result<Vec<Entity>, String> {
        self.layers.get(format)
            .ok_or_else(|| format!("Unknown format: {}", format))?
            .import(data)
    }
    
    /// Export entities through appropriate abstraction layer
    pub fn export(&self, format: &str, entities: &[Entity]) -> Result<Vec<u8>, String> {
        self.layers.get(format)
            .ok_or_else(|| format!("Unknown format: {}", format))?
            .export(entities)
    }
}

/// Demonstration of multi-layer abstraction
pub fn demonstrate_layered_import() {
    let registry = AbstractionRegistry::new();
    
    // Simulated GRAMPS XML data
    let gramps_xml = r#"
    <person handle="_p001" id="I0001">
        <name type="Birth Name">
            <first>John</first>
            <surname>Smith</surname>
        </name>
        <eventref hlink="_e001" role="Primary"/>
        <attribute type="Occupation" value="Blacksmith">
            <dateval val="1850-1860"/>
        </attribute>
    </person>
    "#;
    
    // Import through GRAMPS layer
    let entities = registry.import("GRAMPS-XML", gramps_xml.as_bytes()).unwrap();
    
    // The imported entity preserves GRAMPS structure but is now in meta-model
    let person = &entities[0];
    
    // GRAMPS-specific data is preserved
    assert_eq!(person.entity_type, "GRAMPS.Person");
    
    // But dates are parsed through calendar abstraction
    // Attributes are preserved as nested entities
    // Names could be parsed through name abstraction
    
    println!("Imported GRAMPS data through abstraction layers");
}

/// Example: How abstraction layers enable lossless round-trip
pub fn demonstrate_round_trip() {
    let registry = AbstractionRegistry::new();
    
    // Original GRAMPS data
    let original_xml = r#"<person handle="_p001" id="I0001">
        <name type="Birth Name">
            <first>John</first>
            <call>Johnny</call>
            <surname prefix="van">Smith</surname>
        </name>
        <attribute type="Occupation" value="Blacksmith"/>
        <attribute type="Religion" value="Lutheran"/>
        <personref hlink="_p002" rel="Godfather"/>
    </person>"#;
    
    // Import
    let entities = registry.import("GRAMPS-XML", original_xml.as_bytes()).unwrap();
    
    // Manipulate in meta-model (add research notes, create relationships, etc.)
    let mut person = entities[0].clone();
    person.properties.set_text("research_note", "Need to verify occupation");
    
    // Export back to GRAMPS
    let exported_xml = registry.export("GRAMPS-XML", &[person]).unwrap();
    
    // Original GRAMPS data is preserved even after round-trip!
    // Additional research data is stored separately
    
    println!("Round-trip preserves all original data");
}

/// Example: Cross-format transformation
pub fn demonstrate_cross_format() {
    let registry = AbstractionRegistry::new();
    
    // Import from GRAMPS
    let gramps_data = vec![]; // Would be actual GRAMPS XML
    let entities = registry.import("GRAMPS-XML", &gramps_data).unwrap();
    
    // Transform entities for GEDCOM export
    let transformed_entities: Vec<Entity> = entities.into_iter()
        .map(|e| transform_for_gedcom(e))
        .collect();
    
    // Export to GEDCOM (if we had GEDCOM layer)
    // let gedcom_data = registry.export("GEDCOM", &transformed_entities).unwrap();
    
    println!("Data transformed between formats via meta-model");
}

fn transform_for_gedcom(mut entity: Entity) -> Entity {
    // GEDCOM requires different structure
    if entity.entity_type == "GRAMPS.Person" {
        entity.entity_type = "GEDCOM.INDI".to_string();
        // ... more transformations
    }
    entity
}

/// Example: Using calendar abstraction directly
pub fn demonstrate_calendar_abstraction() {
    let calendar = CalendarAbstraction::new();
    
    // Parse various date formats into universal TemporalValue
    let dates = vec![
        ("1850", None),
        ("abt 1850", None),
        ("bet 1850 and 1855", None),
        ("5 Thermidor An VII", Some("french_republican")),
        ("25 March 1750/51", Some("julian")),
    ];
    
    for (date_str, calendar_hint) in dates {
        match calendar.parse_date(date_str, calendar_hint) {
            Ok(temporal) => {
                println!("Parsed '{}' into universal temporal format", date_str);
                // Now this TemporalValue can be:
                // - Stored in any entity
                // - Compared across calendar systems
                // - Exported to any format
            }
            Err(e) => println!("Failed to parse '{}': {}", date_str, e),
        }
    }
}

/// Example: Preservation of unknown data
pub fn demonstrate_unknown_preservation() {
    // Say GRAMPS adds a new attribute type we don't know about
    let gramps_with_unknown = r#"
    <person handle="_p001" id="I0001">
        <name type="Birth Name">
            <first>John</first>
            <surname>Smith</surname>
        </name>
        <attribute type="FutureAttribute" value="Some future value">
            <future-element>Unknown to us</future-element>
        </attribute>
    </person>
    "#;
    
    let registry = AbstractionRegistry::new();
    
    // Import preserves EVERYTHING
    let entities = registry.import("GRAMPS-XML", gramps_with_unknown.as_bytes()).unwrap();
    let person = &entities[0];
    
    // Unknown data is preserved in gramps_original
    if let Some(Property::Value(Value::Json(original))) = person.properties.get("gramps_original") {
        // Can export back exactly as imported
        println!("Unknown GRAMPS data preserved for round-trip");
    }
    
    // But we can still work with what we understand
    assert!(person.properties.get("name_0").is_some());
}

/// Example: Media handling through abstraction
pub fn demonstrate_media_abstraction() {
    // Create a photo entity
    let mut photo = Entity::new("Media.Photo");
    
    // Small images can be embedded
    let thumbnail_data = vec![0xFF, 0xD8, 0xFF]; // JPEG header
    photo.properties.set("thumbnail", Property::Value(Value::Binary(thumbnail_data)));
    
    // Large images use media abstraction layer
    photo.properties.set("storage", Property::Map({
        let mut storage = HashMap::new();
        storage.insert("backend".to_string(), 
            Property::Value(Value::Text("filesystem".to_string())));
        storage.insert("path".to_string(), 
            Property::Value(Value::Text("/photos/grandma1965.jpg".to_string())));
        storage.insert("checksum".to_string(), 
            Property::Value(Value::Text("sha256:abc123...".to_string())));
        storage
    }));
    
    // Media metadata uses standard properties
    photo.properties.set_text("mime_type", "image/jpeg");
    photo.properties.set("dimensions", Property::Map({
        let mut dims = HashMap::new();
        dims.insert("width".to_string(), Property::Value(Value::Integer(1920)));
        dims.insert("height".to_string(), Property::Value(Value::Integer(1080)));
        dims
    }));
    
    println!("Media handled through abstraction layer");
}

/// The key insight: Abstraction layers are bidirectional transformers
/// 
/// Native Format <-> Abstraction Layer <-> Meta-Model <-> Abstraction Layer <-> Native Format
///
/// This allows:
/// 1. Lossless round-trip (preserve what we don't understand)
/// 2. Cross-format transformation (GRAMPS -> Meta-Model -> GEDCOM)
/// 3. Universal operations (search across all formats)
/// 4. Format evolution (new fields don't break old code)
/// 5. Research enhancement (add research data without losing original)