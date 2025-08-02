// Universal Research Meta-Model
// Layers 2 & 3 with same abstraction level as Layer 1

use crate::theoretical_meta_model::*;
use std::collections::HashMap;

// ============================================================================
// LAYER 2: UNIVERSAL RESEARCH PROCESS ABSTRACTION
// ============================================================================

/// Research Process primitive - can express ANY research methodology
#[derive(Debug, Clone)]
pub struct Process {
    pub id: ProcessId,

    // Process type is open-ended
    // Examples: "GPS.Research", "Scientific.Method", "Grounded.Theory", "Ethnographic.Study"
    pub process_type: String,

    // Process state is open-ended
    // Examples: "Initiated", "Active", "Suspended", "Complete", "Iterating"
    pub state: String,

    // Activities within this process
    pub activities: Vec<Activity>,

    // Products/outputs of this process
    pub products: Vec<ProductId>,

    // Methodologies being followed (can be multiple)
    pub methodologies: Vec<Methodology>,

    // Meta-properties
    pub properties: PropertyGraph,
    pub contexts: Vec<Context>,
    pub meta: MetaInfo,
}

/// Activity primitive - can express ANY research activity
#[derive(Debug, Clone)]
pub struct Activity {
    pub id: ActivityId,

    // Activity type is open-ended
    // Examples: "Search", "Analysis", "Synthesis", "Review", "Experiment", "Interview"
    pub activity_type: String,

    // Activity can have multiple states/phases
    pub states: Vec<ActivityState>,

    // Inputs and outputs (can be entities, processes, or products)
    pub inputs: Vec<ResourceReference>,
    pub outputs: Vec<ResourceReference>,

    // Who/what performs this activity
    pub agents: Vec<Agent>,

    // How the activity is performed
    pub methods: Vec<Method>,

    // Properties and contexts
    pub properties: PropertyGraph,
    pub contexts: Vec<Context>,
}

/// Agent primitive - can express ANY actor in research
#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,

    // Agent type is open-ended
    // Examples: "Human.Researcher", "AI.Assistant", "Community.Group", "Institution"
    pub agent_type: String,

    // Agent capabilities and roles
    pub capabilities: Vec<Capability>,
    pub roles: Vec<Role>,

    // Agent state and availability
    pub state: String,
    pub availability: Availability,

    pub properties: PropertyGraph,
    pub contexts: Vec<Context>,
}

/// Product primitive - can express ANY research output
#[derive(Debug, Clone)]
pub struct Product {
    pub id: ProductId,

    // Product type is open-ended
    // Examples: "Report", "Dataset", "Theory", "Model", "Artifact", "Recording"
    pub product_type: String,

    // Product state/maturity
    pub state: String,
    pub maturity: Maturity,

    // Provenance - how was this produced
    pub provenance: Provenance,

    // Validation/review state
    pub validation: Vec<Validation>,

    pub properties: PropertyGraph,
    pub contexts: Vec<Context>,
    pub meta: MetaInfo,
}

/// Methodology primitive - can express ANY research methodology
#[derive(Debug, Clone)]
pub struct Methodology {
    pub id: MethodologyId,

    // Open-ended methodology types
    // Examples: "GPS", "Scientific.Method", "Design.Thinking", "Agile.Research"
    pub methodology_type: String,

    // Rules/constraints of this methodology (but not hard-coded!)
    pub rules: Vec<Rule>,

    // Quality criteria
    pub quality_criteria: Vec<Criterion>,

    // Compliance checking
    pub compliance: ComplianceFramework,

    pub properties: PropertyGraph,
}

// Supporting abstractions

#[derive(Debug, Clone)]
pub struct ActivityState {
    pub state_type: String,
    pub timestamp: TemporalValue,
    pub agent: AgentId,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub enum ResourceReference {
    Entity(EntityId),
    Process(ProcessId),
    Product(ProductId),
    Activity(ActivityId),
    External(String), // URI or other reference
}

#[derive(Debug, Clone)]
pub struct Capability {
    pub capability_type: String,
    pub level: String,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub role_type: String,
    pub scope: String,
    pub authority: String,
}

#[derive(Debug, Clone)]
pub enum Availability {
    Available,
    Busy(ActivityId),
    Scheduled(Vec<TimeSlot>),
    Unavailable(String),
}

#[derive(Debug, Clone)]
pub struct TimeSlot {
    pub start: TemporalValue,
    pub end: TemporalValue,
    pub activity: Option<ActivityId>,
}

#[derive(Debug, Clone)]
pub enum Maturity {
    Draft,
    Review,
    Revised,
    Final,
    Published,
    Deprecated,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Provenance {
    pub created_by: Vec<AgentId>,
    pub created_from: Vec<ResourceReference>,
    pub methods_used: Vec<Method>,
    pub timeline: Vec<ProvenanceEvent>,
}

#[derive(Debug, Clone)]
pub struct ProvenanceEvent {
    pub event_type: String,
    pub timestamp: TemporalValue,
    pub agent: AgentId,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub method_type: String,
    pub parameters: PropertyGraph,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Validation {
    pub validation_type: String,
    pub validator: AgentId,
    pub result: ValidationResult,
    pub feedback: String,
    pub timestamp: TemporalValue,
}

#[derive(Debug, Clone)]
pub enum ValidationResult {
    Passed,
    Failed(Vec<String>),
    Conditional(Vec<String>),
    InProgress,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub rule_type: String,
    pub expression: String, // Could be logical expression, natural language, etc.
    pub enforcement: Enforcement,
}

#[derive(Debug, Clone)]
pub enum Enforcement {
    Required,
    Recommended,
    Optional,
    Conditional(String),
}

#[derive(Debug, Clone)]
pub struct Criterion {
    pub criterion_type: String,
    pub measurement: String,
    pub threshold: String,
}

#[derive(Debug, Clone)]
pub struct ComplianceFramework {
    pub framework_type: String,
    pub checks: Vec<ComplianceCheck>,
}

#[derive(Debug, Clone)]
pub struct ComplianceCheck {
    pub check_type: String,
    pub expression: String,
    pub severity: String,
}

// ============================================================================
// LAYER 3: UNIVERSAL WORKFLOW & CONFIGURATION ABSTRACTION
// ============================================================================

/// Workspace primitive - can express ANY organizational structure
#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: WorkspaceId,

    // Workspace type is open-ended
    // Examples: "Project", "Investigation", "Collection", "Laboratory", "Archive"
    pub workspace_type: String,

    // Workspace state
    pub state: String,

    // What's in this workspace (completely flexible)
    pub contents: Vec<WorkspaceItem>,

    // How this workspace is organized
    pub organization: Organization,

    // Who can access/modify
    pub governance: Governance,

    // Workspace behaviors
    pub behaviors: Vec<Behavior>,

    pub properties: PropertyGraph,
    pub contexts: Vec<Context>,
    pub meta: MetaInfo,
}

/// WorkspaceItem - anything that can be in a workspace
#[derive(Debug, Clone)]
pub enum WorkspaceItem {
    Entity(EntityId),
    Process(ProcessId),
    Product(ProductId),
    Workspace(WorkspaceId), // Nested workspaces
    Configuration(ConfigurationId),
    View(ViewId),
    Tool(ToolId),
    Reference(String), // External reference
}

/// Organization primitive - how things are structured
#[derive(Debug, Clone)]
pub struct Organization {
    pub organization_type: String,
    pub structure: Structure,
    pub rules: Vec<OrganizationRule>,
}

#[derive(Debug, Clone)]
pub enum Structure {
    Hierarchical(HierarchyNode),
    Network(NetworkGraph),
    Tagged(TagSystem),
    Spatial(SpatialArrangement),
    Temporal(TimelineArrangement),
    Custom(PropertyGraph),
}

#[derive(Debug, Clone)]
pub struct HierarchyNode {
    pub node_type: String,
    pub children: Vec<HierarchyNode>,
    pub properties: PropertyGraph,
}

#[derive(Debug, Clone)]
pub struct NetworkGraph {
    pub nodes: Vec<NodeId>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub edge_type: String,
    pub properties: PropertyGraph,
}

#[derive(Debug, Clone)]
pub struct TagSystem {
    pub tags: HashMap<String, TagInfo>,
    pub relationships: Vec<TagRelationship>,
}

#[derive(Debug, Clone)]
pub struct TagInfo {
    pub tag_type: String,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TagRelationship {
    pub relationship_type: String,
    pub from_tag: String,
    pub to_tag: String,
}

#[derive(Debug, Clone)]
pub struct SpatialArrangement {
    pub space_type: String, // "2D", "3D", "Abstract"
    pub positions: HashMap<WorkspaceItem, Position>,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub coordinates: Vec<f64>,
    pub orientation: Option<Vec<f64>>,
    pub scale: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TimelineArrangement {
    pub timeline_type: String,
    pub events: Vec<TimelineEvent>,
}

#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub timestamp: TemporalValue,
    pub item: WorkspaceItem,
    pub event_type: String,
}

#[derive(Debug, Clone)]
pub struct OrganizationRule {
    pub rule_type: String,
    pub expression: String,
    pub action: String,
}

/// Governance primitive - who can do what
#[derive(Debug, Clone)]
pub struct Governance {
    pub governance_type: String,
    pub policies: Vec<Policy>,
    pub permissions: PermissionSystem,
}

#[derive(Debug, Clone)]
pub struct Policy {
    pub policy_type: String,
    pub rules: Vec<String>,
    pub enforcement: Enforcement,
}

#[derive(Debug, Clone)]
pub struct PermissionSystem {
    pub system_type: String,
    pub permissions: HashMap<AgentId, Vec<Permission>>,
    pub roles: HashMap<String, Vec<Permission>>,
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub permission_type: String,
    pub resource: String,
    pub actions: Vec<String>,
    pub constraints: Vec<String>,
}

/// Behavior primitive - what the workspace does
#[derive(Debug, Clone)]
pub struct Behavior {
    pub behavior_type: String,
    pub triggers: Vec<Trigger>,
    pub actions: Vec<Action>,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub trigger_type: String,
    pub expression: String,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub action_type: String,
    pub parameters: PropertyGraph,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub condition_type: String,
    pub expression: String,
}

/// Configuration primitive - ANY configuration/settings
#[derive(Debug, Clone)]
pub struct Configuration {
    pub id: ConfigurationId,
    pub config_type: String,
    pub schema: Schema,
    pub values: PropertyGraph,
    pub validation: Vec<ValidationRule>,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub schema_type: String,
    pub definition: PropertyGraph,
}

#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub rule_type: String,
    pub expression: String,
    pub message: String,
}

/// View primitive - ANY way of looking at data
#[derive(Debug, Clone)]
pub struct View {
    pub id: ViewId,
    pub view_type: String,
    pub target: ViewTarget,
    pub projection: Projection,
    pub filters: Vec<Filter>,
    pub presentation: Presentation,
}

#[derive(Debug, Clone)]
pub enum ViewTarget {
    Entity(EntityId),
    Process(ProcessId),
    Workspace(WorkspaceId),
    Query(String),
    Multiple(Vec<ViewTarget>),
}

#[derive(Debug, Clone)]
pub struct Projection {
    pub projection_type: String,
    pub fields: Vec<String>,
    pub transformations: Vec<Transformation>,
}

#[derive(Debug, Clone)]
pub struct Transformation {
    pub transform_type: String,
    pub expression: String,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub filter_type: String,
    pub expression: String,
}

#[derive(Debug, Clone)]
pub struct Presentation {
    pub presentation_type: String,
    pub layout: PropertyGraph,
    pub styling: PropertyGraph,
    pub interactions: Vec<Interaction>,
}

#[derive(Debug, Clone)]
pub struct Interaction {
    pub interaction_type: String,
    pub trigger: String,
    pub action: String,
}

/// Tool primitive - ANY tool/capability
#[derive(Debug, Clone)]
pub struct Tool {
    pub id: ToolId,
    pub tool_type: String,
    pub capabilities: Vec<ToolCapability>,
    pub interface: Interface,
    pub configuration: Configuration,
}

#[derive(Debug, Clone)]
pub struct ToolCapability {
    pub capability_type: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub interface_type: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub endpoint_type: String,
    pub path: String,
    pub methods: Vec<String>,
    pub schema: Schema,
}

// ID types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActivityId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProductId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MethodologyId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkspaceId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigurationId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToolId(Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(Uuid);