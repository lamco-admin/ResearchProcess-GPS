// Workspace - Container for organizing work

use super::{WorkspaceId, ConfigurationId, ViewId, ToolId, Organization, Governance};
use crate::layer1::{EntityId, PropertyGraph, Context};
use crate::layer2::{ProcessId, ProductId};
use crate::common::MetaInfo;
use serde::{Serialize, Deserialize};

/// Workspace primitive - can express ANY organizational structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Unique identifier
    pub id: WorkspaceId,
    
    /// Open-ended workspace type
    /// Examples: "Project", "Repository", "Investigation", "Collection"
    pub workspace_type: String,
    
    /// Workspace state
    pub state: String,
    
    /// What's in this workspace
    pub contents: Vec<WorkspaceItem>,
    
    /// How this workspace is organized
    pub organization: Organization,
    
    /// Who can access/modify
    pub governance: Governance,
    
    /// Workspace behaviors
    pub behaviors: Vec<Behavior>,
    
    /// Workspace properties
    pub properties: PropertyGraph,
    
    /// Contexts for this workspace
    pub contexts: Vec<Context>,
    
    /// Metadata
    pub meta: MetaInfo,
}

/// Items that can be in a workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkspaceItem {
    Entity(EntityId),              // Layer 1 entity
    Process(ProcessId),            // Layer 2 process
    Product(ProductId),            // Layer 2 product
    Workspace(WorkspaceId),        // Nested workspace
    Configuration(ConfigurationId), // Configuration
    View(ViewId),                  // View
    Tool(ToolId),                  // Tool
    Reference(String),             // External reference
}

/// Workspace behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    pub behavior_type: String,
    pub triggers: Vec<Trigger>,
    pub actions: Vec<Action>,
    pub conditions: Vec<Condition>,
}

/// Behavior trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub trigger_type: String,
    pub expression: String,
}

/// Behavior action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: String,
    pub parameters: PropertyGraph,
}

/// Behavior condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: String,
    pub expression: String,
}

impl Workspace {
    /// Create a new workspace
    pub fn new(workspace_type: impl Into<String>) -> Self {
        Workspace {
            id: WorkspaceId::new(),
            workspace_type: workspace_type.into(),
            state: "Active".to_string(),
            contents: Vec::new(),
            organization: Organization::default(),
            governance: Governance::default(),
            behaviors: Vec::new(),
            properties: PropertyGraph::new(),
            contexts: Vec::new(),
            meta: MetaInfo::new(),
        }
    }
    
    /// Add an item to the workspace
    pub fn add_item(&mut self, item: WorkspaceItem) {
        self.contents.push(item);
    }
    
    /// Add a behavior
    pub fn add_behavior(&mut self, behavior: Behavior) {
        self.behaviors.push(behavior);
    }
    
    /// Get items of a specific type
    pub fn items_of_type<T>(&self, filter: impl Fn(&WorkspaceItem) -> Option<T>) -> Vec<T> {
        self.contents
            .iter()
            .filter_map(filter)
            .collect()
    }
    
    /// Get all entities
    pub fn entities(&self) -> Vec<EntityId> {
        self.items_of_type(|item| match item {
            WorkspaceItem::Entity(id) => Some(*id),
            _ => None,
        })
    }
    
    /// Get all processes
    pub fn processes(&self) -> Vec<ProcessId> {
        self.items_of_type(|item| match item {
            WorkspaceItem::Process(id) => Some(*id),
            _ => None,
        })
    }
}

/// Common workspace builders
impl Workspace {
    /// Create a project workspace
    pub fn project(name: impl Into<String>) -> Self {
        let mut workspace = Workspace::new("Project");
        workspace.properties.set_text("name", name);
        workspace
    }
    
    /// Create a GitHub repository workspace
    pub fn github_repo(repo_name: impl Into<String>) -> Self {
        let mut workspace = Workspace::new("GitHub.Repository");
        workspace.properties.set_text("repository", repo_name);
        workspace
    }
    
    /// Create a research investigation workspace
    pub fn investigation(topic: impl Into<String>) -> Self {
        let mut workspace = Workspace::new("Research.Investigation");
        workspace.properties.set_text("topic", topic);
        workspace
    }
    
    /// Create a file system directory workspace
    pub fn directory(path: impl Into<String>) -> Self {
        let mut workspace = Workspace::new("FileSystem.Directory");
        workspace.properties.set_text("path", path);
        workspace
    }
}

impl Behavior {
    /// Create a new behavior
    pub fn new(behavior_type: impl Into<String>) -> Self {
        Behavior {
            behavior_type: behavior_type.into(),
            triggers: Vec::new(),
            actions: Vec::new(),
            conditions: Vec::new(),
        }
    }
    
    /// Add a trigger
    pub fn add_trigger(&mut self, trigger_type: impl Into<String>, expression: impl Into<String>) {
        self.triggers.push(Trigger {
            trigger_type: trigger_type.into(),
            expression: expression.into(),
        });
    }
    
    /// Add an action
    pub fn add_action(&mut self, action_type: impl Into<String>) {
        self.actions.push(Action {
            action_type: action_type.into(),
            parameters: PropertyGraph::new(),
        });
    }
    
    /// Add a condition
    pub fn add_condition(&mut self, condition_type: impl Into<String>, expression: impl Into<String>) {
        self.conditions.push(Condition {
            condition_type: condition_type.into(),
            expression: expression.into(),
        });
    }
}