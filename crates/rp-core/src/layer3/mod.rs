//! Layer 3: Workspace & Metadata Model
//! 
//! This layer provides configuration infrastructure that makes ResearchProcess-GPS
//! adaptable to any research methodology without code changes.
//! 
//! Core principle: "Standards as Data, Not Code"

use crate::{EntityId, Result};
use serde::{Deserialize, Serialize};
use std::any::Any;

pub mod workspace;
pub mod methodology_config;
pub mod standards_registry;
pub mod module_config;
pub mod template_registry;
pub mod validation_rule;

pub use workspace::*;
pub use methodology_config::*;
pub use standards_registry::*;
pub use module_config::*;
pub use template_registry::*;
pub use validation_rule::*;

/// Type aliases for Layer 3 entity IDs
pub type WorkspaceId = EntityId;
pub type MethodologyConfigId = EntityId;
pub type StandardsRegistryId = EntityId;
pub type StandardsConfigId = EntityId;
pub type ModuleConfigId = EntityId;
pub type TemplateRegistryId = EntityId;
pub type TemplateConfigId = EntityId;
pub type ValidationRuleId = EntityId;

/// Base trait for Layer 3 configuration entities
/// These are simpler than full entities - they're configuration objects
pub trait ConfigEntity: Send + Sync + std::fmt::Debug {
    /// Get the configuration entity's ID
    fn id(&self) -> EntityId;
    
    /// Get the configuration type name
    fn config_type(&self) -> &'static str;
    
    /// Convert to Any for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Validate the configuration is well-formed
    fn validate(&self) -> Result<()>;
}

/// Configuration source types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigSource {
    /// Loaded from local file
    Local { 
        path: String,
        last_modified: chrono::DateTime<chrono::Utc>,
    },
    /// Loaded from remote repository
    Remote { 
        url: String,
        version: String,
        last_synced: chrono::DateTime<chrono::Utc>,
    },
    /// Embedded in code
    Embedded {
        version: String,
    },
    /// Created by user
    UserDefined {
        created_at: chrono::DateTime<chrono::Utc>,
    },
}

/// Module type categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleType {
    /// Data capture and import modules
    Capture,
    /// Analysis and correlation modules
    Analysis,
    /// Report and visualization generation
    Generation,
    /// Compliance and quality validation
    Validation,
    /// Integration with external systems
    Integration,
}

/// Template format types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateFormat {
    /// Markdown with variable substitution
    Markdown,
    /// HTML with template engine
    Html,
    /// LaTeX for academic output
    LaTeX,
    /// Microsoft Word via template
    Docx,
    /// Plain text
    Text,
}

/// Validation rule types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleType {
    /// Field must be present
    RequiredField,
    /// Field must match format
    Format,
    /// Entity must meet compliance rules
    Compliance,
    /// Custom validation logic
    Custom,
}

/// Severity levels for validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Must be fixed
    Error,
    /// Should be fixed
    Warning,
    /// Informational only
    Info,
}

/// Entity types that rules can apply to
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    // Layer 1 - Core Data Model (9 entities)
    Analysis,
    Citation,
    Confidence,
    Evidence,
    Fact,
    IdentityPersona,
    Location,
    Relationship,
    Source,
    
    // Layer 2 - Research Process & Products (9 entities)
    AnalysisReport,
    ProofStatement,
    ResearchActivity,
    Researcher,
    ResearchLog,
    ResearchSession,
    Theory,
    WorkProduct,
    
    // Layer 3 - Workflow & Configuration (1 entity)
    Workspace,
}