// Tool - Capabilities and tools in workspaces

use super::{ToolId, ConfigurationId};
use crate::layer1::PropertyGraph;
use serde::{Serialize, Deserialize};

/// Tool primitive - ANY tool/capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Unique identifier
    pub id: ToolId,
    
    /// Open-ended tool type
    /// Examples: "Analyzer", "Importer", "Validator", "Generator"
    pub tool_type: String,
    
    /// Tool capabilities
    pub capabilities: Vec<ToolCapability>,
    
    /// Tool interface
    pub interface: Interface,
    
    /// Tool configuration
    pub configuration: Configuration,
}

/// Tool capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    pub capability_type: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub constraints: Vec<String>,
}

/// Tool interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub interface_type: String,
    pub endpoints: Vec<Endpoint>,
}

/// Interface endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub endpoint_type: String,
    pub path: String,
    pub methods: Vec<String>,
    pub schema: Schema,
}

// Re-use Configuration and Schema from configuration module
pub use super::configuration::{Configuration, Schema};

impl Tool {
    /// Create a new tool
    pub fn new(tool_type: impl Into<String>) -> Self {
        Tool {
            id: ToolId::new(),
            tool_type: tool_type.into(),
            capabilities: Vec::new(),
            interface: Interface {
                interface_type: "Default".to_string(),
                endpoints: Vec::new(),
            },
            configuration: Configuration::new("ToolConfig"),
        }
    }
    
    /// Add a capability
    pub fn add_capability(&mut self, capability: ToolCapability) {
        self.capabilities.push(capability);
    }
    
    /// Add an endpoint
    pub fn add_endpoint(&mut self, endpoint: Endpoint) {
        self.interface.endpoints.push(endpoint);
    }
    
    /// Check if tool has capability
    pub fn has_capability(&self, capability_type: &str) -> bool {
        self.capabilities
            .iter()
            .any(|c| c.capability_type == capability_type)
    }
}