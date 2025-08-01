// Common types used across all layers

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::layer1::TemporalValue;

/// Metadata for tracking changes and provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaInfo {
    pub created_at: TemporalValue,
    pub created_by: AgentId,
    pub updated_at: TemporalValue,
    pub updated_by: AgentId,
    pub version: u32,
    pub transaction_id: Option<Uuid>,
}

impl MetaInfo {
    pub fn new() -> Self {
        let now = TemporalValue::year(2025); // Simplified for now
        let system_agent = AgentId::system();
        
        MetaInfo {
            created_at: now.clone(),
            created_by: system_agent,
            updated_at: now,
            updated_by: system_agent,
            version: 1,
            transaction_id: None,
        }
    }
    
    pub fn update(&mut self, agent: AgentId) {
        self.updated_at = TemporalValue::year(2025); // Simplified
        self.updated_by = agent;
        self.version += 1;
    }
}

// Agent ID used across layers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub Uuid);

impl AgentId {
    pub fn new() -> Self {
        AgentId(Uuid::new_v4())
    }
    
    pub fn system() -> Self {
        AgentId(Uuid::nil())
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Agent:{}", self.0)
    }
}