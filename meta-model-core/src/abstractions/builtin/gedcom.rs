// GEDCOM abstraction layer (placeholder)

use crate::layer1::*;
use crate::abstractions::base::{
    AbstractionLayer, TransformContext, ImportResult, ValidationReport,
};
use crate::abstractions::error::{AbstractionError, AbstractionResult};
use async_trait::async_trait;

/// GEDCOM abstraction
pub struct GedcomAbstraction {
    version: String,
}

impl GedcomAbstraction {
    pub fn new() -> Self {
        GedcomAbstraction {
            version: "5.5.1".to_string(),
        }
    }
}

#[async_trait]
impl AbstractionLayer for GedcomAbstraction {
    fn name(&self) -> &str {
        "GEDCOM"
    }

    fn format_id(&self) -> &str {
        "GEDCOM"
    }

    fn format_version(&self) -> &str {
        &self.version
    }

    async fn import(
        &self,
        _data: &[u8],
        _context: &mut TransformContext,
    ) -> AbstractionResult<ImportResult> {
        // TODO: Implement GEDCOM import
        Err(AbstractionError::UnsupportedFormat(
            "GEDCOM import not yet implemented".to_string()
        ))
    }

    async fn export(
        &self,
        _entities: &[Entity],
        _relationships: &[Relationship],
        _context: &mut TransformContext,
    ) -> AbstractionResult<Vec<u8>> {
        // TODO: Implement GEDCOM export
        Err(AbstractionError::UnsupportedFormat(
            "GEDCOM export not yet implemented".to_string()
        ))
    }

    async fn validate(&self, _data: &[u8]) -> AbstractionResult<ValidationReport> {
        // TODO: Implement GEDCOM validation
        Ok(ValidationReport {
            valid: false,
            format_detected: None,
            version_detected: None,
            errors: vec![],
            warnings: vec![],
        })
    }
}