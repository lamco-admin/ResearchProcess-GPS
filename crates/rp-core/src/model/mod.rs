use serde::{Deserialize, Serialize};

// Re-export the meta-model
pub use meta_model_core::*;

/// Temporal constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalConstraint {
    pub constraint_type: String,
    pub expression: String,
}

/// Temporal quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalQuality {
    pub quality_type: String,
    pub value: String,
    pub unit: String,
}
