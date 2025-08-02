// Layer 2: Universal Research Process
//
// Abstract primitives that can express ANY research methodology

mod process;
mod activity;
mod agent;
mod product;
pub mod methodology;

pub use process::Process;
pub use activity::{Activity, ActivityState, Method};
pub use agent::{Agent, Capability, Role, Availability};
pub use product::{Product, Maturity, Provenance, Validation};
pub use methodology::{Methodology, Rule, Criterion, ComplianceFramework};

// Resource references that can cross layers
pub use activity::ResourceReference;

// ID types
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActivityId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProductId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MethodologyId(pub Uuid);

// Re-export AgentId from common
pub use crate::common::AgentId;

// Constructors
impl ProcessId {
    pub fn new() -> Self {
        ProcessId(Uuid::new_v4())
    }
}

impl ActivityId {
    pub fn new() -> Self {
        ActivityId(Uuid::new_v4())
    }
}

impl ProductId {
    pub fn new() -> Self {
        ProductId(Uuid::new_v4())
    }
}

impl MethodologyId {
    pub fn new() -> Self {
        MethodologyId(Uuid::new_v4())
    }
}

// Display implementations
impl std::fmt::Display for ProcessId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Process:{}", self.0)
    }
}

impl std::fmt::Display for ActivityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Activity:{}", self.0)
    }
}

impl std::fmt::Display for ProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Product:{}", self.0)
    }
}

impl std::fmt::Display for MethodologyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Methodology:{}", self.0)
    }
}