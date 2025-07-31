//! ResearchProcess-GPS Core Library
//! 
//! This crate defines the fundamental traits, types, and entities that form
//! the foundation of the ResearchProcess-GPS protocol and engine.

pub mod entity;
pub mod error;
pub mod id;
pub mod state;
pub mod validation;
pub mod researcher;
pub mod theory;
pub mod confidence;
pub mod evidence;
pub mod analysis;
pub mod identity_persona;
pub mod source;
pub mod citation;
pub mod fact;
pub mod relationship;
pub mod location;
pub mod work_product;
pub mod research_log;
pub mod research_session;
pub mod research_activity;
pub mod proof_statement;
pub mod evidence_analysis;

pub use entity::*;
pub use error::*;
pub use id::*;
pub use state::*;
pub use validation::*;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::entity::{Entity, NestableEntity, VersionedEntity};
    pub use crate::error::{Error, Result};
    pub use crate::id::{EntityId, generate_id};
    pub use crate::state::{StateMachine, State, StateTransition};
    pub use crate::validation::{Validatable, ValidationError};
    pub use crate::researcher::Researcher;
    pub use crate::theory::{Theory, TheoryState};
    pub use crate::confidence::{Confidence, ConfidenceLevel};
    pub use crate::evidence::{Evidence, ExtractedFact, RecordType};
    pub use crate::analysis::{Analysis, AnalysisType, AnalyticalPoint, ArgumentStrength};
    pub use crate::identity_persona::{IdentityPersona, IdentityState, IdentityType, EvidenceReference};
    pub use crate::source::{Source, SourceState, SourceType, SourceQuality, SourceClass, InformationClass};
    pub use crate::citation::{Citation, CitationState, CitationPurpose, CitationQuality, CitingEntityType};
    pub use crate::fact::{Fact, FactState, FactType, FactValue, DatePrecision, LocationReference};
    pub use crate::relationship::{Relationship, RelationshipState, RelationshipType, RelationshipPeriod};
    pub use crate::location::{Location, LocationType, Coordinates, AlternativeName};
    pub use crate::work_product::{WorkProduct, WorkProductState, WorkProductType, ValidationStatus, ComplianceResult, WorkProductReview};
    pub use crate::research_log::{ResearchLog, LogType, LogEntryType, ResearchLogEntry, ResearchCoverage};
    pub use crate::research_session::{ResearchSession, SessionType, SessionState};
    pub use crate::research_activity::{ResearchActivity, ActivityType, ActivityResult, ResultQuality};
    pub use crate::proof_statement::{ProofStatement, ProofType, ProofSection, GPSElement, ConflictDescription, ProofAudience};
    pub use crate::evidence_analysis::{EvidenceAnalysis, AnalysisMethod, EvidenceAnalysisItem, EvidenceType, Correlation, Pattern, Anomaly};
}