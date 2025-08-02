// View - Ways of looking at data

use super::{ViewId, WorkspaceId};
use crate::layer1::{EntityId, PropertyGraph};
use crate::layer2::ProcessId;
use serde::{Serialize, Deserialize};

/// View primitive - ANY way of looking at data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    /// Unique identifier
    pub id: ViewId,

    /// Open-ended view type
    /// Examples: "List", "Graph", "Timeline", "Map", "Dashboard"
    pub view_type: String,

    /// What this view targets
    pub target: ViewTarget,

    /// How to project the data
    pub projection: Projection,

    /// Filters to apply
    pub filters: Vec<Filter>,

    /// How to present the data
    pub presentation: Presentation,
}

/// What the view targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewTarget {
    Entity(EntityId),
    Process(ProcessId),
    Workspace(WorkspaceId),
    Query(String),
    Multiple(Vec<ViewTarget>),
}

/// Data projection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projection {
    pub projection_type: String,
    pub fields: Vec<String>,
    pub transformations: Vec<Transformation>,
}

/// Data transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    pub transform_type: String,
    pub expression: String,
}

/// View filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub filter_type: String,
    pub expression: String,
}

/// Presentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presentation {
    pub presentation_type: String,
    pub layout: PropertyGraph,
    pub styling: PropertyGraph,
    pub interactions: Vec<Interaction>,
}

/// User interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub interaction_type: String,
    pub trigger: String,
    pub action: String,
}

impl View {
    /// Create a new view
    pub fn new(view_type: impl Into<String>, target: ViewTarget) -> Self {
        View {
            id: ViewId::new(),
            view_type: view_type.into(),
            target,
            projection: Projection {
                projection_type: "Default".to_string(),
                fields: Vec::new(),
                transformations: Vec::new(),
            },
            filters: Vec::new(),
            presentation: Presentation {
                presentation_type: "Default".to_string(),
                layout: PropertyGraph::new(),
                styling: PropertyGraph::new(),
                interactions: Vec::new(),
            },
        }
    }

    /// Add a filter
    pub fn add_filter(&mut self, filter: Filter) {
        self.filters.push(filter);
    }

    /// Add an interaction
    pub fn add_interaction(&mut self, interaction: Interaction) {
        self.presentation.interactions.push(interaction);
    }
}