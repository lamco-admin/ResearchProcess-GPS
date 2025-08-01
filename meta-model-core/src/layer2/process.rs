// Process - The fundamental unit of research methodology

use super::{ProcessId, Activity, ProductId, Methodology};
use crate::layer1::{PropertyGraph, Context};
use crate::common::MetaInfo;
use serde::{Serialize, Deserialize};

/// Research Process primitive - can express ANY research methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    /// Unique identifier
    pub id: ProcessId,
    
    /// Open-ended process type
    /// Examples: "GPS.Research", "Scientific.Method", "Design.Thinking", "Agile.Sprint"
    pub process_type: String,
    
    /// Open-ended process state
    /// Examples: "Planning", "Active", "Review", "Complete", "Suspended"
    pub state: String,
    
    /// Activities within this process
    pub activities: Vec<Activity>,
    
    /// Products/outputs of this process
    pub products: Vec<ProductId>,
    
    /// Methodologies being followed (can be multiple)
    pub methodologies: Vec<Methodology>,
    
    /// Process properties
    pub properties: PropertyGraph,
    
    /// Contexts scoping this process
    pub contexts: Vec<Context>,
    
    /// Metadata
    pub meta: MetaInfo,
}

impl Process {
    /// Create a new process
    pub fn new(process_type: impl Into<String>) -> Self {
        Process {
            id: ProcessId::new(),
            process_type: process_type.into(),
            state: "Planning".to_string(),
            activities: Vec::new(),
            products: Vec::new(),
            methodologies: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::new(),
        }
    }
    
    /// Add an activity to the process
    pub fn add_activity(&mut self, activity: Activity) {
        self.activities.push(activity);
    }
    
    /// Add a product reference
    pub fn add_product(&mut self, product_id: ProductId) {
        if !self.products.contains(&product_id) {
            self.products.push(product_id);
        }
    }
    
    /// Add a methodology
    pub fn add_methodology(&mut self, methodology: Methodology) {
        self.methodologies.push(methodology);
    }
    
    /// Change process state
    pub fn set_state(&mut self, state: impl Into<String>) {
        self.state = state.into();
        self.meta.update(crate::common::AgentId::system());
    }
    
    /// Get activities by state
    pub fn activities_by_state(&self, state: &str) -> Vec<&Activity> {
        self.activities
            .iter()
            .filter(|a| a.states.iter().any(|s| s.state_type == state))
            .collect()
    }
    
    /// Check if process follows a specific methodology
    pub fn follows_methodology(&self, methodology_type: &str) -> bool {
        self.methodologies
            .iter()
            .any(|m| m.methodology_type == methodology_type)
    }
}

/// Common process builders
impl Process {
    /// Create a GPS research process
    pub fn gps_research(research_question: impl Into<String>) -> Self {
        let mut process = Process::new("GPS.Research");
        process.properties.set_text("research_question", research_question);
        process.state = "Active".to_string();
        process
    }
    
    /// Create a scientific method process
    pub fn scientific_method(hypothesis: impl Into<String>) -> Self {
        let mut process = Process::new("Scientific.Method");
        process.properties.set_text("hypothesis", hypothesis);
        process.state = "Observation".to_string();
        process
    }
    
    /// Create an agile sprint
    pub fn agile_sprint(sprint_goal: impl Into<String>, duration_days: i32) -> Self {
        let mut process = Process::new("Agile.Sprint");
        process.properties.set_text("sprint_goal", sprint_goal);
        process.properties.set_integer("duration_days", duration_days as i64);
        process.state = "Planning".to_string();
        process
    }
}