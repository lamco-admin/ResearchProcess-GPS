// Organization - How things are structured in a workspace

use super::{NodeId, WorkspaceItem};
use crate::layer1::{PropertyGraph, TemporalValue};
use indexmap::IndexMap;
use serde::{Serialize, Deserialize};

/// Organization structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub organization_type: String,
    pub structure: Structure,
    pub rules: Vec<OrganizationRule>,
}

/// Different organizational structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Structure {
    Hierarchical(HierarchyNode),
    Network(NetworkGraph),
    Tagged(TagSystem),
    Spatial(SpatialArrangement),
    Temporal(TimelineArrangement),
    Custom(PropertyGraph),
}

/// Hierarchical structure node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyNode {
    pub node_type: String,
    pub children: Vec<HierarchyNode>,
    pub properties: PropertyGraph,
}

/// Network graph structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkGraph {
    pub nodes: Vec<NodeId>,
    pub edges: Vec<Edge>,
}

/// Graph edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub edge_type: String,
    pub properties: PropertyGraph,
}

/// Tag-based organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSystem {
    pub tags: IndexMap<String, TagInfo>,
    pub relationships: Vec<TagRelationship>,
}

/// Tag information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    pub tag_type: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub rules: Vec<String>,
}

/// Tag relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagRelationship {
    pub relationship_type: String,
    pub from_tag: String,
    pub to_tag: String,
}

/// Spatial arrangement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialArrangement {
    pub space_type: String, // "2D", "3D", "Abstract"
    pub positions: IndexMap<String, Position>, // item_id -> position
}

/// Position in space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub coordinates: Vec<f64>,
    pub orientation: Option<Vec<f64>>,
    pub scale: Option<f64>,
}

/// Timeline arrangement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineArrangement {
    pub timeline_type: String,
    pub events: Vec<TimelineEvent>,
}

/// Timeline event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timestamp: TemporalValue,
    pub item: WorkspaceItem,
    pub event_type: String,
}

/// Organization rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationRule {
    pub rule_type: String,
    pub expression: String,
    pub action: String,
}

impl Default for Organization {
    fn default() -> Self {
        Organization {
            organization_type: "Flat".to_string(),
            structure: Structure::Custom(PropertyGraph::new()),
            rules: Vec::new(),
        }
    }
}

impl Organization {
    /// Create hierarchical organization
    pub fn hierarchical() -> Self {
        Organization {
            organization_type: "Hierarchical".to_string(),
            structure: Structure::Hierarchical(HierarchyNode {
                node_type: "Root".to_string(),
                children: Vec::new(),
                properties: PropertyGraph::new(),
            }),
            rules: Vec::new(),
        }
    }

    /// Create network organization
    pub fn network() -> Self {
        Organization {
            organization_type: "Network".to_string(),
            structure: Structure::Network(NetworkGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
            }),
            rules: Vec::new(),
        }
    }

    /// Create tag-based organization
    pub fn tagged() -> Self {
        Organization {
            organization_type: "Tagged".to_string(),
            structure: Structure::Tagged(TagSystem {
                tags: IndexMap::new(),
                relationships: Vec::new(),
            }),
            rules: Vec::new(),
        }
    }

    /// Add an organization rule
    pub fn add_rule(&mut self, rule: OrganizationRule) {
        self.rules.push(rule);
    }
}

impl HierarchyNode {
    /// Create a new hierarchy node
    pub fn new(node_type: impl Into<String>) -> Self {
        HierarchyNode {
            node_type: node_type.into(),
            children: Vec::new(),
            properties: PropertyGraph::new(),
        }
    }

    /// Add a child node
    pub fn add_child(&mut self, child: HierarchyNode) {
        self.children.push(child);
    }
}

impl TagSystem {
    /// Add a tag
    pub fn add_tag(&mut self, name: impl Into<String>, tag_info: TagInfo) {
        self.tags.insert(name.into(), tag_info);
    }

    /// Add a tag relationship
    pub fn add_relationship(&mut self, relationship: TagRelationship) {
        self.relationships.push(relationship);
    }
}