// Agent - Actor in research processes

use crate::common::AgentId;
use crate::layer1::{PropertyGraph, Context};
use crate::layer2::ActivityId;
use serde::{Serialize, Deserialize};

/// Agent primitive - can express ANY actor in research
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier
    pub id: AgentId,

    /// Open-ended agent type
    /// Examples: "Human.Researcher", "AI.Assistant", "Software.Tool", "Organization"
    pub agent_type: String,

    /// Agent capabilities
    pub capabilities: Vec<Capability>,

    /// Agent roles
    pub roles: Vec<Role>,

    /// Current state
    pub state: String,

    /// Availability
    pub availability: Availability,

    /// Agent properties
    pub properties: PropertyGraph,

    /// Contexts for this agent
    pub contexts: Vec<Context>,
}

/// Agent capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub capability_type: String,
    pub level: String,
    pub constraints: Vec<String>,
}

/// Agent role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub role_type: String,
    pub scope: String,
    pub authority: String,
}

/// Agent availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Availability {
    Available,
    Busy(ActivityId),
    Scheduled(Vec<TimeSlot>),
    Unavailable(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start: crate::layer1::TemporalValue,
    pub end: crate::layer1::TemporalValue,
    pub activity: Option<ActivityId>,
}

impl Agent {
    /// Create a new agent
    pub fn new(agent_type: impl Into<String>) -> Self {
        Agent {
            id: AgentId::new(),
            agent_type: agent_type.into(),
            capabilities: Vec::new(),
            roles: Vec::new(),
            state: "Active".to_string(),
            availability: Availability::Available,
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
        }
    }

    /// Create with specific ID
    pub fn with_id(id: AgentId, agent_type: impl Into<String>) -> Self {
        Agent {
            id,
            agent_type: agent_type.into(),
            capabilities: Vec::new(),
            roles: Vec::new(),
            state: "Active".to_string(),
            availability: Availability::Available,
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
        }
    }

    /// Add a capability
    pub fn add_capability(&mut self, capability: Capability) {
        self.capabilities.push(capability);
    }

    /// Add a role
    pub fn add_role(&mut self, role: Role) {
        self.roles.push(role);
    }

    /// Set availability
    pub fn set_availability(&mut self, availability: Availability) {
        self.availability = availability;
    }

    /// Check if agent has capability
    pub fn has_capability(&self, capability_type: &str) -> bool {
        self.capabilities
            .iter()
            .any(|c| c.capability_type == capability_type)
    }

    /// Check if agent has role
    pub fn has_role(&self, role_type: &str) -> bool {
        self.roles
            .iter()
            .any(|r| r.role_type == role_type)
    }
}

/// Common agent builders
impl Agent {
    /// Create a human researcher
    pub fn researcher(name: impl Into<String>) -> Self {
        let mut agent = Agent::new("Human.Researcher");
        agent.properties.set_text("name", name);
        agent.add_capability(Capability {
            capability_type: "Research.Genealogical".to_string(),
            level: "Professional".to_string(),
            constraints: vec![],
        });
        agent
    }

    /// Create an AI assistant
    pub fn ai_assistant(model: impl Into<String>) -> Self {
        let mut agent = Agent::new("AI.Assistant");
        agent.properties.set_text("model", model);
        agent.add_capability(Capability {
            capability_type: "Analysis.Pattern".to_string(),
            level: "Advanced".to_string(),
            constraints: vec!["Requires human validation".to_string()],
        });
        agent
    }

    /// Create a software tool
    pub fn software_tool(tool_name: impl Into<String>) -> Self {
        let mut agent = Agent::new("Software.Tool");
        agent.properties.set_text("tool_name", tool_name);
        agent
    }

    /// Create the system agent
    pub fn system() -> Self {
        Agent::with_id(AgentId::system(), "System")
    }
}

impl Capability {
    /// Create a new capability
    pub fn new(capability_type: impl Into<String>, level: impl Into<String>) -> Self {
        Capability {
            capability_type: capability_type.into(),
            level: level.into(),
            constraints: Vec::new(),
        }
    }
}

impl Role {
    /// Create a new role
    pub fn new(role_type: impl Into<String>, scope: impl Into<String>) -> Self {
        Role {
            role_type: role_type.into(),
            scope: scope.into(),
            authority: "Standard".to_string(),
        }
    }
}