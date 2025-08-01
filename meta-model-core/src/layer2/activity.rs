// Activity - A step or action within a research process

use super::{ActivityId, ProcessId, ProductId, Agent};
use crate::layer1::{EntityId, PropertyGraph, Context, TemporalValue};
use crate::common::AgentId;
use serde::{Serialize, Deserialize};

/// Activity primitive - can express ANY research activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    /// Unique identifier
    pub id: ActivityId,
    
    /// Open-ended activity type
    /// Examples: "Search", "Analysis", "Interview", "Experiment", "Review"
    pub activity_type: String,
    
    /// Activity can have multiple states/phases
    pub states: Vec<ActivityState>,
    
    /// Inputs to this activity
    pub inputs: Vec<ResourceReference>,
    
    /// Outputs from this activity
    pub outputs: Vec<ResourceReference>,
    
    /// Who/what performs this activity
    pub agents: Vec<Agent>,
    
    /// How the activity is performed
    pub methods: Vec<Method>,
    
    /// Activity properties
    pub properties: PropertyGraph,
    
    /// Contexts scoping this activity
    pub contexts: Vec<Context>,
}

/// State of an activity at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityState {
    pub state_type: String,
    pub timestamp: TemporalValue,
    pub agent: AgentId,
    pub notes: String,
}

/// Reference to resources (can cross layers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceReference {
    Entity(EntityId),         // Layer 1 entity
    Process(ProcessId),       // Another process
    Product(ProductId),       // A product
    Activity(ActivityId),     // Another activity
    External(String),         // External reference (URI, etc.)
}

/// Method for performing an activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub method_type: String,
    pub parameters: PropertyGraph,
    pub constraints: Vec<String>,
}

impl Activity {
    /// Create a new activity
    pub fn new(activity_type: impl Into<String>) -> Self {
        Activity {
            id: ActivityId::new(),
            activity_type: activity_type.into(),
            states: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            agents: Vec::new(),
            methods: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
        }
    }
    
    /// Add a state transition
    pub fn add_state(&mut self, state_type: impl Into<String>, agent: AgentId, notes: impl Into<String>) {
        self.states.push(ActivityState {
            state_type: state_type.into(),
            timestamp: TemporalValue::year(2025), // Simplified
            agent,
            notes: notes.into(),
        });
    }
    
    /// Add an input
    pub fn add_input(&mut self, input: ResourceReference) {
        self.inputs.push(input);
    }
    
    /// Add an output
    pub fn add_output(&mut self, output: ResourceReference) {
        self.outputs.push(output);
    }
    
    /// Add an agent
    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }
    
    /// Add a method
    pub fn add_method(&mut self, method: Method) {
        self.methods.push(method);
    }
    
    /// Get current state
    pub fn current_state(&self) -> Option<&ActivityState> {
        self.states.last()
    }
    
    /// Check if activity is in a specific state
    pub fn is_in_state(&self, state_type: &str) -> bool {
        self.current_state()
            .map(|s| s.state_type == state_type)
            .unwrap_or(false)
    }
}

/// Common activity builders
impl Activity {
    /// Create a search activity
    pub fn search(scope: impl Into<String>) -> Self {
        let mut activity = Activity::new("Search");
        activity.properties.set_text("scope", scope);
        activity
    }
    
    /// Create an analysis activity
    pub fn analysis(analysis_type: impl Into<String>) -> Self {
        let mut activity = Activity::new("Analysis");
        activity.properties.set_text("analysis_type", analysis_type);
        activity
    }
    
    /// Create an evidence correlation activity
    pub fn evidence_correlation() -> Self {
        Activity::new("Evidence.Correlation")
    }
    
    /// Create a documentation activity
    pub fn documentation(document_type: impl Into<String>) -> Self {
        let mut activity = Activity::new("Documentation");
        activity.properties.set_text("document_type", document_type);
        activity
    }
}

impl Method {
    /// Create a new method
    pub fn new(method_type: impl Into<String>) -> Self {
        Method {
            method_type: method_type.into(),
            parameters: PropertyGraph::new(),
            constraints: Vec::new(),
        }
    }
    
    /// Add a constraint
    pub fn add_constraint(&mut self, constraint: impl Into<String>) {
        self.constraints.push(constraint.into());
    }
}