// Base abstraction layer trait and types

use crate::layer1::{Entity, Relationship};
use crate::abstractions::{AbstractionResult, AbstractionError};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Base trait for all abstraction layers
#[async_trait]
pub trait AbstractionLayer: Send + Sync {
    /// Name of this abstraction layer
    fn name(&self) -> &str;
    
    /// Supported format identifier (e.g., "GRAMPS-XML", "GEDCOM-5.5.1")
    fn format_id(&self) -> &str;
    
    /// Version of the format this abstraction supports
    fn format_version(&self) -> &str;
    
    /// Import from native format to meta-model
    async fn import(
        &self, 
        data: &[u8], 
        context: &mut TransformContext
    ) -> AbstractionResult<ImportResult>;
    
    /// Export from meta-model to native format
    async fn export(
        &self,
        entities: &[Entity],
        relationships: &[Relationship],
        context: &mut TransformContext
    ) -> AbstractionResult<Vec<u8>>;
    
    /// Validate that data is in the expected format
    async fn validate(&self, data: &[u8]) -> AbstractionResult<ValidationReport>;
    
    /// Check if an entity was created by this abstraction
    fn owns_entity(&self, entity: &Entity) -> bool {
        entity.entity_type.starts_with(&format!("{}.", self.format_id()))
    }
    
    /// Get metadata about this abstraction
    fn metadata(&self) -> AbstractionMetadata {
        AbstractionMetadata {
            name: self.name().to_string(),
            format_id: self.format_id().to_string(),
            format_version: self.format_version().to_string(),
            capabilities: vec![],
            limitations: vec![],
        }
    }
}

/// Result of an import operation
#[derive(Debug, Clone)]
pub struct ImportResult {
    /// Entities created
    pub entities: Vec<Entity>,
    
    /// Relationships created
    pub relationships: Vec<Relationship>,
    
    /// Warnings generated during import
    pub warnings: Vec<ImportWarning>,
    
    /// Statistics about the import
    pub statistics: ImportStatistics,
}

/// Warning generated during import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportWarning {
    pub severity: WarningSeverity,
    pub code: String,
    pub message: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WarningSeverity {
    Info,
    Warning,
    Error,
}

/// Import statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportStatistics {
    pub total_records: usize,
    pub entities_created: usize,
    pub relationships_created: usize,
    pub skipped_records: usize,
    pub processing_time_ms: u64,
}

/// Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub valid: bool,
    pub format_detected: Option<String>,
    pub version_detected: Option<String>,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub message: String,
}

/// Context for transformations
#[derive(Debug, Clone)]
pub struct TransformContext {
    /// Options for the transformation
    pub options: TransformOptions,
    
    /// Mapping of external IDs to entity IDs
    pub id_map: HashMap<String, crate::layer1::EntityId>,
    
    /// Calendar system to use
    pub calendar_system: String,
    
    /// Language/locale for text processing
    pub locale: String,
    
    /// Custom properties to preserve
    pub preserve_unknown: bool,
}

/// Options for transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformOptions {
    /// Preserve original data in entity properties
    pub preserve_original: bool,
    
    /// Create audit trail of transformation
    pub audit_trail: bool,
    
    /// Strict mode - fail on any warnings
    pub strict: bool,
    
    /// Custom options
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for TransformContext {
    fn default() -> Self {
        TransformContext {
            options: TransformOptions::default(),
            id_map: HashMap::new(),
            calendar_system: "gregorian".to_string(),
            locale: "en-US".to_string(),
            preserve_unknown: true,
        }
    }
}

impl Default for TransformOptions {
    fn default() -> Self {
        TransformOptions {
            preserve_original: true,
            audit_trail: false,
            strict: false,
            custom: HashMap::new(),
        }
    }
}

/// Metadata about an abstraction layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractionMetadata {
    pub name: String,
    pub format_id: String,
    pub format_version: String,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}