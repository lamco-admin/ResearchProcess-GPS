// GRAMPS XML abstraction layer

use crate::layer1::*;
use crate::abstractions::{
    AbstractionLayer, TransformContext, ImportResult, ValidationReport,
    AbstractionResult, AbstractionError, ImportWarning, WarningSeverity,
    ImportStatistics,
};
use async_trait::async_trait;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::io::BufRead;

/// GRAMPS XML abstraction
pub struct GrampsAbstraction {
    version: String,
}

impl GrampsAbstraction {
    pub fn new() -> Self {
        GrampsAbstraction {
            version: "1.7.2".to_string(),
        }
    }
    
    /// Parse a GRAMPS person element
    fn parse_person(
        &self,
        reader: &mut Reader<&[u8]>,
        context: &mut TransformContext,
    ) -> AbstractionResult<Entity> {
        let mut entity = Entity::new("GRAMPS.Person");
        let mut attributes = HashMap::new();
        
        // Parse person attributes
        // This is simplified - real implementation would handle all GRAMPS elements
        
        // Store original XML if requested
        if context.options.preserve_original {
            // In real implementation, would store the actual XML
            entity.properties.set(
                "gramps_original",
                Property::Value(Value::Text("<!-- original XML -->".to_string()))
            );
        }
        
        Ok(entity)
    }
}

#[async_trait]
impl AbstractionLayer for GrampsAbstraction {
    fn name(&self) -> &str {
        "GRAMPS XML"
    }
    
    fn format_id(&self) -> &str {
        "GRAMPS-XML"
    }
    
    fn format_version(&self) -> &str {
        &self.version
    }
    
    async fn import(
        &self,
        data: &[u8],
        context: &mut TransformContext,
    ) -> AbstractionResult<ImportResult> {
        let mut reader = Reader::from_reader(data);
        reader.trim_text(true);
        
        let mut entities = Vec::new();
        let mut relationships = Vec::new();
        let mut warnings = Vec::new();
        let mut statistics = ImportStatistics::default();
        
        let start_time = std::time::Instant::now();
        
        let mut buf = Vec::new();
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name().as_ref() {
                        b"person" => {
                            statistics.total_records += 1;
                            match self.parse_person(&mut reader, context) {
                                Ok(entity) => {
                                    entities.push(entity);
                                    statistics.entities_created += 1;
                                }
                                Err(err) => {
                                    warnings.push(ImportWarning {
                                        severity: WarningSeverity::Warning,
                                        code: "PARSE_ERROR".to_string(),
                                        message: err.to_string(),
                                        context: Some("person".to_string()),
                                    });
                                    statistics.skipped_records += 1;
                                }
                            }
                        }
                        b"family" => {
                            // Parse family as relationship
                            statistics.total_records += 1;
                            // Simplified - would create relationships
                        }
                        b"event" => {
                            // Parse event as entity
                            statistics.total_records += 1;
                            // Simplified
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(AbstractionError::XmlError(format!(
                        "Error at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    )));
                }
                _ => {}
            }
            buf.clear();
        }
        
        statistics.processing_time_ms = start_time.elapsed().as_millis() as u64;
        
        Ok(ImportResult {
            entities,
            relationships,
            warnings,
            statistics,
        })
    }
    
    async fn export(
        &self,
        entities: &[Entity],
        relationships: &[Relationship],
        context: &mut TransformContext,
    ) -> AbstractionResult<Vec<u8>> {
        use quick_xml::Writer;
        use quick_xml::events::{BytesStart, BytesEnd, BytesText};
        
        let mut writer = Writer::new(Vec::new());
        
        // Start GRAMPS database
        let mut db_elem = BytesStart::new("database");
        db_elem.push_attribute(("xmlns", "http://gramps-project.org/xml/1.7.2/"));
        writer.write_event(Event::Start(db_elem))?;
        
        // Export entities
        for entity in entities {
            if entity.entity_type.starts_with("GRAMPS.") {
                match entity.entity_type.as_str() {
                    "GRAMPS.Person" => {
                        self.export_person(&mut writer, entity, context)?;
                    }
                    "GRAMPS.Event" => {
                        self.export_event(&mut writer, entity, context)?;
                    }
                    _ => {
                        // Skip unknown GRAMPS types
                    }
                }
            }
        }
        
        // Export relationships as families
        for relationship in relationships {
            if relationship.relationship_type.starts_with("GRAMPS.") {
                self.export_family(&mut writer, relationship, context)?;
            }
        }
        
        // End database
        writer.write_event(Event::End(BytesEnd::new("database")))?;
        
        Ok(writer.into_inner())
    }
    
    async fn validate(&self, data: &[u8]) -> AbstractionResult<ValidationReport> {
        let mut reader = Reader::from_reader(data);
        reader.trim_text(true);
        
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut format_detected = None;
        let mut version_detected = None;
        
        let mut buf = Vec::new();
        
        // Check for GRAMPS XML structure
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"database" {
                        // Check for GRAMPS namespace
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"xmlns" {
                                    let value = String::from_utf8_lossy(&attr.value);
                                    if value.contains("gramps-project.org") {
                                        format_detected = Some("GRAMPS-XML".to_string());
                                        // Extract version from namespace
                                        if let Some(version) = value.split('/').last() {
                                            version_detected = Some(version.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    errors.push(crate::abstractions::base::ValidationError {
                        line: None,
                        column: None,
                        message: format!("XML parse error: {:?}", e),
                    });
                    break;
                }
                _ => {}
            }
            buf.clear();
        }
        
        Ok(ValidationReport {
            valid: format_detected.is_some() && errors.is_empty(),
            format_detected,
            version_detected,
            errors,
            warnings,
        })
    }
}

impl GrampsAbstraction {
    fn export_person(
        &self,
        writer: &mut quick_xml::Writer<Vec<u8>>,
        entity: &Entity,
        _context: &TransformContext,
    ) -> AbstractionResult<()> {
        use quick_xml::events::{BytesStart, BytesEnd, BytesText, Event};
        
        // Check if we have original GRAMPS data
        if let Some(Property::Value(Value::Text(original))) = entity.properties.get("gramps_original") {
            // In real implementation, would parse and write original XML
            // For now, create basic person element
        }
        
        let mut person = BytesStart::new("person");
        
        // Add handle and id if available
        if let Some(handle) = entity.properties.get_text("gramps_handle") {
            person.push_attribute(("handle", handle));
        }
        if let Some(id) = entity.properties.get_text("gramps_id") {
            person.push_attribute(("id", id));
        }
        
        writer.write_event(Event::Start(person.to_owned()))?;
        
        // Export name if present
        if let Some(Property::Entity(name_entity)) = entity.properties.get("name_0") {
            self.export_name(writer, name_entity)?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("person")))?;
        
        Ok(())
    }
    
    fn export_name(
        &self,
        writer: &mut quick_xml::Writer<Vec<u8>>,
        name_entity: &Entity,
    ) -> AbstractionResult<()> {
        use quick_xml::events::{BytesStart, BytesEnd, BytesText, Event};
        
        let mut name = BytesStart::new("name");
        if let Some(name_type) = name_entity.properties.get_text("type") {
            name.push_attribute(("type", name_type));
        }
        
        writer.write_event(Event::Start(name.to_owned()))?;
        
        // First name
        if let Some(first) = name_entity.properties.get_text("first") {
            writer.write_event(Event::Start(BytesStart::new("first")))?;
            writer.write_event(Event::Text(BytesText::new(first)))?;
            writer.write_event(Event::End(BytesEnd::new("first")))?;
        }
        
        // Surname
        if let Some(surname) = name_entity.properties.get_text("surname") {
            let mut surname_elem = BytesStart::new("surname");
            if let Some(prefix) = name_entity.properties.get_text("surname_prefix") {
                surname_elem.push_attribute(("prefix", prefix));
            }
            writer.write_event(Event::Start(surname_elem))?;
            writer.write_event(Event::Text(BytesText::new(surname)))?;
            writer.write_event(Event::End(BytesEnd::new("surname")))?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("name")))?;
        
        Ok(())
    }
    
    fn export_event(
        &self,
        writer: &mut quick_xml::Writer<Vec<u8>>,
        entity: &Entity,
        _context: &TransformContext,
    ) -> AbstractionResult<()> {
        // Similar to export_person
        Ok(())
    }
    
    fn export_family(
        &self,
        writer: &mut quick_xml::Writer<Vec<u8>>,
        relationship: &Relationship,
        _context: &TransformContext,
    ) -> AbstractionResult<()> {
        // Export relationship as GRAMPS family
        Ok(())
    }
}