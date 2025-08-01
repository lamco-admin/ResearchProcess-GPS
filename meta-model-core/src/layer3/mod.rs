// Layer 3: Universal Workflow
//
// Abstract primitives for ANY workflow system

mod workspace;
mod configuration;
mod view;
mod tool;
mod organization;
mod governance;

pub use workspace::{Workspace, WorkspaceItem};
pub use configuration::{Configuration, Schema, ValidationRule};
pub use view::{View, ViewTarget, Projection, Filter, Presentation};
pub use tool::{Tool, ToolCapability, Interface};
pub use organization::{Organization, Structure, OrganizationRule};
pub use governance::{Governance, Policy, PermissionSystem, Permission};

// Re-export nested types
pub use workspace::Behavior;
pub use organization::{HierarchyNode, NetworkGraph, TagSystem};
pub use governance::Enforcement;

// ID types
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConfigurationId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ViewId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ToolId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub Uuid);

// Constructors
impl WorkspaceId {
    pub fn new() -> Self {
        WorkspaceId(Uuid::new_v4())
    }
}

impl ConfigurationId {
    pub fn new() -> Self {
        ConfigurationId(Uuid::new_v4())
    }
}

impl ViewId {
    pub fn new() -> Self {
        ViewId(Uuid::new_v4())
    }
}

impl ToolId {
    pub fn new() -> Self {
        ToolId(Uuid::new_v4())
    }
}

impl NodeId {
    pub fn new() -> Self {
        NodeId(Uuid::new_v4())
    }
}

// Display implementations
impl std::fmt::Display for WorkspaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Workspace:{}", self.0)
    }
}

impl std::fmt::Display for ConfigurationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration:{}", self.0)
    }
}

impl std::fmt::Display for ViewId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "View:{}", self.0)
    }
}

impl std::fmt::Display for ToolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tool:{}", self.0)
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node:{}", self.0)
    }
}