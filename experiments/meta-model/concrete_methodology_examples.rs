// Concrete Examples: How real methodologies map to abstract Layer 2 & 3
// Shows GPS, Scientific Method, Agile, etc. expressed in universal meta-model

use crate::universal_research_meta_model::*;
use crate::theoretical_meta_model::*;
use std::collections::HashMap;

// ============================================================================
// LAYER 2 EXAMPLES: RESEARCH METHODOLOGIES
// ============================================================================

/// Example 1: Genealogical Proof Standard (GPS) as Layer 2 Process
pub mod gps_methodology {
    use super::*;
    
    pub fn create_gps_process() -> Process {
        let mut gps = Process {
            id: ProcessId::new(),
            process_type: "GPS.Research".to_string(),
            state: "Active".to_string(),
            activities: vec![],
            products: vec![],
            methodologies: vec![],
            properties: PropertyGraph::new(),
            contexts: vec![],
            meta: MetaInfo::now(),
        };
        
        // GPS Methodology definition
        let gps_methodology = create_gps_methodology();
        gps.methodologies.push(gps_methodology);
        
        // GPS Activities (5 elements)
        gps.activities = vec![
            create_exhaustive_search_activity(),
            create_source_citation_activity(),
            create_evidence_analysis_activity(),
            create_conflict_resolution_activity(),
            create_written_conclusion_activity(),
        ];
        
        // GPS-specific properties
        gps.properties.set_text("research_question", "Who were the parents of John Smith (1850-1920)?");
        gps.properties.set("quality_score", Property::Map({
            let mut scores = HashMap::new();
            scores.insert("exhaustiveness".to_string(), 
                Property::Value(Value::Float(0.85)));
            scores.insert("source_reliability".to_string(), 
                Property::Value(Value::Float(0.90)));
            scores.insert("evidence_correlation".to_string(), 
                Property::Value(Value::Float(0.75)));
            scores
        }));
        
        gps
    }
    
    fn create_gps_methodology() -> Methodology {
        Methodology {
            id: MethodologyId::new(),
            methodology_type: "Genealogical.ProofStandard".to_string(),
            rules: vec![
                Rule {
                    rule_type: "Requirement.ExhaustiveSearch".to_string(),
                    expression: "search_coverage >= 0.9 AND sources_checked >= minimum_sources".to_string(),
                    enforcement: Enforcement::Required,
                },
                Rule {
                    rule_type: "Requirement.CompleteCitations".to_string(),
                    expression: "all_sources.have_citation AND citations.are_complete".to_string(),
                    enforcement: Enforcement::Required,
                },
                Rule {
                    rule_type: "Requirement.AnalysisQuality".to_string(),
                    expression: "evidence.is_analyzed AND correlations.are_identified".to_string(),
                    enforcement: Enforcement::Required,
                },
                Rule {
                    rule_type: "Requirement.ConflictResolution".to_string(),
                    expression: "conflicts.are_identified AND resolutions.are_reasoned".to_string(),
                    enforcement: Enforcement::Required,
                },
                Rule {
                    rule_type: "Requirement.WrittenConclusion".to_string(),
                    expression: "conclusion.is_written AND reasoning.is_sound".to_string(),
                    enforcement: Enforcement::Required,
                },
            ],
            quality_criteria: vec![
                Criterion {
                    criterion_type: "SearchCompleteness".to_string(),
                    measurement: "percentage_sources_searched".to_string(),
                    threshold: "90%".to_string(),
                },
                Criterion {
                    criterion_type: "EvidenceReliability".to_string(),
                    measurement: "weighted_source_quality".to_string(),
                    threshold: "High".to_string(),
                },
            ],
            compliance: ComplianceFramework {
                framework_type: "GPS.Compliance".to_string(),
                checks: vec![
                    ComplianceCheck {
                        check_type: "ExhaustiveSearch.Verify".to_string(),
                        expression: "repositories_searched.includes(all_likely_repositories)".to_string(),
                        severity: "Critical".to_string(),
                    },
                ],
            },
            properties: PropertyGraph::new(),
        }
    }
    
    fn create_exhaustive_search_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "GPS.ExhaustiveSearch".to_string(),
            states: vec![
                ActivityState {
                    state_type: "Planning".to_string(),
                    timestamp: TemporalValue::now(),
                    agent: AgentId::new(),
                    notes: "Identifying all potential repositories".to_string(),
                },
                ActivityState {
                    state_type: "Executing".to_string(),
                    timestamp: TemporalValue::now(),
                    agent: AgentId::new(),
                    notes: "Searching identified repositories".to_string(),
                },
            ],
            inputs: vec![
                ResourceReference::External("Research Question".to_string()),
                ResourceReference::External("Known Information".to_string()),
            ],
            outputs: vec![
                ResourceReference::Product(ProductId::new()), // Search log
                ResourceReference::Product(ProductId::new()), // Source list
            ],
            agents: vec![create_genealogist_agent()],
            methods: vec![
                Method {
                    method_type: "Repository.Survey".to_string(),
                    parameters: {
                        let mut params = PropertyGraph::new();
                        params.set_text("scope", "All repositories likely to have records");
                        params
                    },
                    constraints: vec!["Must include online and offline sources".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_source_citation_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "GPS.SourceCitation".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Sources Found".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Citations
            agents: vec![create_genealogist_agent()],
            methods: vec![
                Method {
                    method_type: "Citation.Chicago".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Must enable source retrieval".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_evidence_analysis_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "GPS.EvidenceAnalysis".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Cited Sources".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Analysis
            agents: vec![create_genealogist_agent()],
            methods: vec![
                Method {
                    method_type: "Evidence.Correlation".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Consider all evidence pieces".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_conflict_resolution_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "GPS.ConflictResolution".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Conflicting Evidence".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Resolution
            agents: vec![create_genealogist_agent()],
            methods: vec![
                Method {
                    method_type: "Conflict.ReasonedAnalysis".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Must explain resolution reasoning".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_written_conclusion_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "GPS.WrittenConclusion".to_string(),
            states: vec![],
            inputs: vec![
                ResourceReference::External("All Analysis".to_string()),
                ResourceReference::External("Resolutions".to_string()),
            ],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Proof argument
            agents: vec![create_genealogist_agent()],
            methods: vec![
                Method {
                    method_type: "Writing.ProofArgument".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Must be coherent and complete".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_genealogist_agent() -> Agent {
        Agent {
            id: AgentId::new(),
            agent_type: "Human.Genealogist".to_string(),
            capabilities: vec![
                Capability {
                    capability_type: "Research.Genealogical".to_string(),
                    level: "Professional".to_string(),
                    constraints: vec![],
                },
                Capability {
                    capability_type: "Analysis.Evidence".to_string(),
                    level: "Expert".to_string(),
                    constraints: vec![],
                },
            ],
            roles: vec![
                Role {
                    role_type: "Lead.Researcher".to_string(),
                    scope: "Full project".to_string(),
                    authority: "Decision making".to_string(),
                },
            ],
            state: "Active".to_string(),
            availability: Availability::Available,
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
}

/// Example 2: Scientific Method as Layer 2 Process
pub mod scientific_method {
    use super::*;
    
    pub fn create_scientific_process() -> Process {
        let mut scientific = Process {
            id: ProcessId::new(),
            process_type: "Scientific.Method".to_string(),
            state: "Experimentation".to_string(),
            activities: vec![],
            products: vec![],
            methodologies: vec![],
            properties: PropertyGraph::new(),
            contexts: vec![],
            meta: MetaInfo::now(),
        };
        
        // Scientific Method activities
        scientific.activities = vec![
            create_observation_activity(),
            create_hypothesis_activity(),
            create_experiment_activity(),
            create_analysis_activity(),
            create_conclusion_activity(),
        ];
        
        // Scientific methodology
        let methodology = Methodology {
            id: MethodologyId::new(),
            methodology_type: "Scientific.Empirical".to_string(),
            rules: vec![
                Rule {
                    rule_type: "Falsifiability".to_string(),
                    expression: "hypothesis.can_be_disproven".to_string(),
                    enforcement: Enforcement::Required,
                },
                Rule {
                    rule_type: "Reproducibility".to_string(),
                    expression: "experiment.can_be_repeated".to_string(),
                    enforcement: Enforcement::Required,
                },
                Rule {
                    rule_type: "ControlledVariables".to_string(),
                    expression: "variables.are_controlled".to_string(),
                    enforcement: Enforcement::Required,
                },
            ],
            quality_criteria: vec![
                Criterion {
                    criterion_type: "StatisticalSignificance".to_string(),
                    measurement: "p_value".to_string(),
                    threshold: "< 0.05".to_string(),
                },
            ],
            compliance: ComplianceFramework {
                framework_type: "Scientific.Standards".to_string(),
                checks: vec![],
            },
            properties: PropertyGraph::new(),
        };
        
        scientific.methodologies.push(methodology);
        scientific
    }
    
    fn create_observation_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scientific.Observation".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Natural Phenomenon".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Observations
            agents: vec![create_scientist_agent()],
            methods: vec![
                Method {
                    method_type: "Observation.Systematic".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Unbiased recording".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_hypothesis_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scientific.Hypothesis".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Observations".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Hypothesis
            agents: vec![create_scientist_agent()],
            methods: vec![
                Method {
                    method_type: "Hypothesis.Formation".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Must be testable".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_experiment_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scientific.Experiment".to_string(),
            states: vec![
                ActivityState {
                    state_type: "Design".to_string(),
                    timestamp: TemporalValue::now(),
                    agent: AgentId::new(),
                    notes: "Designing controlled experiment".to_string(),
                },
                ActivityState {
                    state_type: "Execute".to_string(),
                    timestamp: TemporalValue::now(),
                    agent: AgentId::new(),
                    notes: "Running experiment with controls".to_string(),
                },
            ],
            inputs: vec![ResourceReference::External("Hypothesis".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Data
            agents: vec![create_scientist_agent()],
            methods: vec![
                Method {
                    method_type: "Experiment.Controlled".to_string(),
                    parameters: {
                        let mut params = PropertyGraph::new();
                        params.set_text("design", "Double-blind randomized");
                        params
                    },
                    constraints: vec!["Control all variables".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_analysis_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scientific.Analysis".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Experimental Data".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Results
            agents: vec![create_scientist_agent()],
            methods: vec![
                Method {
                    method_type: "Analysis.Statistical".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Appropriate statistical tests".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_conclusion_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scientific.Conclusion".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Analysis Results".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Paper
            agents: vec![create_scientist_agent()],
            methods: vec![
                Method {
                    method_type: "Conclusion.Evidence-Based".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Based solely on evidence".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_scientist_agent() -> Agent {
        Agent {
            id: AgentId::new(),
            agent_type: "Human.Scientist".to_string(),
            capabilities: vec![
                Capability {
                    capability_type: "Research.Scientific".to_string(),
                    level: "PhD".to_string(),
                    constraints: vec![],
                },
            ],
            roles: vec![
                Role {
                    role_type: "Principal.Investigator".to_string(),
                    scope: "Research project".to_string(),
                    authority: "Full".to_string(),
                },
            ],
            state: "Active".to_string(),
            availability: Availability::Available,
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
}

/// Example 3: Agile/Scrum as Layer 2 Process
pub mod agile_methodology {
    use super::*;
    
    pub fn create_agile_process() -> Process {
        let mut agile = Process {
            id: ProcessId::new(),
            process_type: "Agile.Scrum".to_string(),
            state: "Sprint.Active".to_string(),
            activities: vec![],
            products: vec![],
            methodologies: vec![],
            properties: PropertyGraph::new(),
            contexts: vec![],
            meta: MetaInfo::now(),
        };
        
        // Sprint activities
        agile.activities = vec![
            create_sprint_planning_activity(),
            create_daily_standup_activity(),
            create_development_activity(),
            create_sprint_review_activity(),
            create_retrospective_activity(),
        ];
        
        // Agile methodology
        let methodology = Methodology {
            id: MethodologyId::new(),
            methodology_type: "Agile.Framework".to_string(),
            rules: vec![
                Rule {
                    rule_type: "Iteration.TimeBoxed".to_string(),
                    expression: "sprint.duration <= 4.weeks".to_string(),
                    enforcement: Enforcement::Required,
                },
                Rule {
                    rule_type: "Team.SelfOrganizing".to_string(),
                    expression: "team.makes_decisions".to_string(),
                    enforcement: Enforcement::Recommended,
                },
            ],
            quality_criteria: vec![
                Criterion {
                    criterion_type: "Velocity".to_string(),
                    measurement: "story_points_per_sprint".to_string(),
                    threshold: "Consistent or improving".to_string(),
                },
            ],
            compliance: ComplianceFramework {
                framework_type: "Scrum.Guide".to_string(),
                checks: vec![],
            },
            properties: PropertyGraph::new(),
        };
        
        agile.methodologies.push(methodology);
        agile
    }
    
    fn create_sprint_planning_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scrum.SprintPlanning".to_string(),
            states: vec![],
            inputs: vec![
                ResourceReference::External("Product Backlog".to_string()),
                ResourceReference::External("Team Capacity".to_string()),
            ],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Sprint backlog
            agents: vec![
                create_scrum_master_agent(),
                create_product_owner_agent(),
                create_developer_agent(),
            ],
            methods: vec![
                Method {
                    method_type: "Planning.Poker".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Whole team participates".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_daily_standup_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scrum.DailyStandup".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Yesterday's Progress".to_string())],
            outputs: vec![ResourceReference::External("Today's Plan".to_string())],
            agents: vec![create_developer_agent()],
            methods: vec![
                Method {
                    method_type: "Meeting.Standup".to_string(),
                    parameters: {
                        let mut params = PropertyGraph::new();
                        params.set_text("duration", "15 minutes");
                        params
                    },
                    constraints: vec!["Time-boxed".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_development_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scrum.Development".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("User Stories".to_string())],
            outputs: vec![ResourceReference::Product(ProductId::new())], // Working software
            agents: vec![create_developer_agent()],
            methods: vec![
                Method {
                    method_type: "Development.Iterative".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Definition of Done".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_sprint_review_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scrum.SprintReview".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Sprint Increment".to_string())],
            outputs: vec![ResourceReference::External("Stakeholder Feedback".to_string())],
            agents: vec![
                create_product_owner_agent(),
                create_developer_agent(),
            ],
            methods: vec![
                Method {
                    method_type: "Demo.Live".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Working software only".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_retrospective_activity() -> Activity {
        Activity {
            id: ActivityId::new(),
            activity_type: "Scrum.Retrospective".to_string(),
            states: vec![],
            inputs: vec![ResourceReference::External("Sprint Metrics".to_string())],
            outputs: vec![ResourceReference::External("Improvement Actions".to_string())],
            agents: vec![
                create_scrum_master_agent(),
                create_developer_agent(),
            ],
            methods: vec![
                Method {
                    method_type: "Retrospective.StartStopContinue".to_string(),
                    parameters: PropertyGraph::new(),
                    constraints: vec!["Safe environment".to_string()],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_scrum_master_agent() -> Agent {
        Agent {
            id: AgentId::new(),
            agent_type: "Human.ScrumMaster".to_string(),
            capabilities: vec![
                Capability {
                    capability_type: "Facilitation".to_string(),
                    level: "Certified".to_string(),
                    constraints: vec![],
                },
            ],
            roles: vec![
                Role {
                    role_type: "Process.Facilitator".to_string(),
                    scope: "Scrum Team".to_string(),
                    authority: "Process guidance".to_string(),
                },
            ],
            state: "Active".to_string(),
            availability: Availability::Available,
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_product_owner_agent() -> Agent {
        Agent {
            id: AgentId::new(),
            agent_type: "Human.ProductOwner".to_string(),
            capabilities: vec![
                Capability {
                    capability_type: "Product.Vision".to_string(),
                    level: "Expert".to_string(),
                    constraints: vec![],
                },
            ],
            roles: vec![
                Role {
                    role_type: "Product.Decision".to_string(),
                    scope: "Product Backlog".to_string(),
                    authority: "Prioritization".to_string(),
                },
            ],
            state: "Active".to_string(),
            availability: Availability::Available,
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
    
    fn create_developer_agent() -> Agent {
        Agent {
            id: AgentId::new(),
            agent_type: "Human.Developer".to_string(),
            capabilities: vec![
                Capability {
                    capability_type: "Software.Development".to_string(),
                    level: "Senior".to_string(),
                    constraints: vec![],
                },
            ],
            roles: vec![
                Role {
                    role_type: "Team.Member".to_string(),
                    scope: "Development".to_string(),
                    authority: "Technical decisions".to_string(),
                },
            ],
            state: "Active".to_string(),
            availability: Availability::Available,
            properties: PropertyGraph::new(),
            contexts: vec![],
        }
    }
}

// ============================================================================
// LAYER 3 EXAMPLES: WORKFLOW SYSTEMS
// ============================================================================

/// Example 1: GitHub workflow as Layer 3 Workspace
pub mod github_workflow {
    use super::*;
    
    pub fn create_github_workspace() -> Workspace {
        let mut github = Workspace {
            id: WorkspaceId::new(),
            workspace_type: "GitHub.Repository".to_string(),
            state: "Active".to_string(),
            contents: vec![],
            organization: Organization {
                organization_type: "Repository.Structure".to_string(),
                structure: Structure::Hierarchical(HierarchyNode {
                    node_type: "Root".to_string(),
                    children: vec![
                        HierarchyNode {
                            node_type: "Branches".to_string(),
                            children: vec![],
                            properties: PropertyGraph::new(),
                        },
                        HierarchyNode {
                            node_type: "Issues".to_string(),
                            children: vec![],
                            properties: PropertyGraph::new(),
                        },
                        HierarchyNode {
                            node_type: "PullRequests".to_string(),
                            children: vec![],
                            properties: PropertyGraph::new(),
                        },
                    ],
                    properties: PropertyGraph::new(),
                }),
                rules: vec![
                    OrganizationRule {
                        rule_type: "Branch.Protection".to_string(),
                        expression: "main.requires_review".to_string(),
                        action: "Block direct push".to_string(),
                    },
                ],
            },
            governance: Governance {
                governance_type: "Repository.Permissions".to_string(),
                policies: vec![
                    Policy {
                        policy_type: "Access.Control".to_string(),
                        rules: vec!["Contributors can create branches".to_string()],
                        enforcement: Enforcement::Required,
                    },
                ],
                permissions: PermissionSystem {
                    system_type: "GitHub.RBAC".to_string(),
                    permissions: HashMap::new(),
                    roles: {
                        let mut roles = HashMap::new();
                        roles.insert("Owner".to_string(), vec![
                            Permission {
                                permission_type: "Full".to_string(),
                                resource: "*".to_string(),
                                actions: vec!["*".to_string()],
                                constraints: vec![],
                            },
                        ]);
                        roles.insert("Contributor".to_string(), vec![
                            Permission {
                                permission_type: "Limited".to_string(),
                                resource: "branches/*".to_string(),
                                actions: vec!["create", "push"].into_iter().map(String::from).collect(),
                                constraints: vec!["Not to main".to_string()],
                            },
                        ]);
                        roles
                    },
                },
            },
            behaviors: vec![
                Behavior {
                    behavior_type: "CI.Pipeline".to_string(),
                    triggers: vec![
                        Trigger {
                            trigger_type: "Push".to_string(),
                            expression: "branch != 'main'".to_string(),
                        },
                    ],
                    actions: vec![
                        Action {
                            action_type: "Run.Tests".to_string(),
                            parameters: PropertyGraph::new(),
                        },
                    ],
                    conditions: vec![],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
            meta: MetaInfo::now(),
        };
        
        // Add GitHub-specific tools
        github.contents.push(WorkspaceItem::Tool(create_github_cli_tool().id));
        github.contents.push(WorkspaceItem::View(create_pr_view().id));
        
        github
    }
    
    fn create_github_cli_tool() -> Tool {
        Tool {
            id: ToolId::new(),
            tool_type: "CLI.GitHub".to_string(),
            capabilities: vec![
                ToolCapability {
                    capability_type: "Repository.Management".to_string(),
                    inputs: vec!["commands".to_string()],
                    outputs: vec!["repository state".to_string()],
                    constraints: vec![],
                },
            ],
            interface: Interface {
                interface_type: "CommandLine".to_string(),
                endpoints: vec![
                    Endpoint {
                        endpoint_type: "Command".to_string(),
                        path: "gh".to_string(),
                        methods: vec!["pr", "issue", "repo"].into_iter().map(String::from).collect(),
                        schema: Schema {
                            schema_type: "CLI.Arguments".to_string(),
                            definition: PropertyGraph::new(),
                        },
                    },
                ],
            },
            configuration: Configuration {
                id: ConfigurationId::new(),
                config_type: "GitHub.Settings".to_string(),
                schema: Schema {
                    schema_type: "JSON".to_string(),
                    definition: PropertyGraph::new(),
                },
                values: PropertyGraph::new(),
                validation: vec![],
            },
        }
    }
    
    fn create_pr_view() -> View {
        View {
            id: ViewId::new(),
            view_type: "GitHub.PullRequestList".to_string(),
            target: ViewTarget::Query("type:pr state:open".to_string()),
            projection: Projection {
                projection_type: "List".to_string(),
                fields: vec!["title", "author", "status", "reviews"].into_iter().map(String::from).collect(),
                transformations: vec![],
            },
            filters: vec![
                Filter {
                    filter_type: "State".to_string(),
                    expression: "state == 'open'".to_string(),
                },
            ],
            presentation: Presentation {
                presentation_type: "WebUI.List".to_string(),
                layout: PropertyGraph::new(),
                styling: PropertyGraph::new(),
                interactions: vec![
                    Interaction {
                        interaction_type: "Click".to_string(),
                        trigger: "row".to_string(),
                        action: "navigate_to_pr".to_string(),
                    },
                ],
            },
        }
    }
}

/// Example 2: JIRA workflow as Layer 3 Workspace
pub mod jira_workflow {
    use super::*;
    
    pub fn create_jira_workspace() -> Workspace {
        let mut jira = Workspace {
            id: WorkspaceId::new(),
            workspace_type: "JIRA.Project".to_string(),
            state: "Active".to_string(),
            contents: vec![],
            organization: Organization {
                organization_type: "Issue.Tracking".to_string(),
                structure: Structure::Network(NetworkGraph {
                    nodes: vec![], // Would contain issue IDs
                    edges: vec![], // Links between issues
                }),
                rules: vec![
                    OrganizationRule {
                        rule_type: "Workflow.Transition".to_string(),
                        expression: "status_transitions.follow_workflow".to_string(),
                        action: "Enforce workflow rules".to_string(),
                    },
                ],
            },
            governance: create_jira_governance(),
            behaviors: create_jira_behaviors(),
            properties: PropertyGraph::new(),
            contexts: vec![],
            meta: MetaInfo::now(),
        };
        
        // Add JIRA boards as views
        jira.contents.push(WorkspaceItem::View(create_kanban_board().id));
        jira.contents.push(WorkspaceItem::Configuration(create_jira_workflow_config().id));
        
        jira
    }
    
    fn create_jira_governance() -> Governance {
        Governance {
            governance_type: "JIRA.Permissions".to_string(),
            policies: vec![
                Policy {
                    policy_type: "Issue.Creation".to_string(),
                    rules: vec!["Users can create issues".to_string()],
                    enforcement: Enforcement::Required,
                },
            ],
            permissions: PermissionSystem {
                system_type: "JIRA.PermissionScheme".to_string(),
                permissions: HashMap::new(),
                roles: HashMap::new(),
            },
        }
    }
    
    fn create_jira_behaviors() -> Vec<Behavior> {
        vec![
            Behavior {
                behavior_type: "Workflow.StateTransition".to_string(),
                triggers: vec![
                    Trigger {
                        trigger_type: "Status.Change".to_string(),
                        expression: "from:InProgress to:Done".to_string(),
                    },
                ],
                actions: vec![
                    Action {
                        action_type: "Validate.Requirements".to_string(),
                        parameters: PropertyGraph::new(),
                    },
                ],
                conditions: vec![
                    Condition {
                        condition_type: "Field.Required".to_string(),
                        expression: "resolution != null".to_string(),
                    },
                ],
            },
        ]
    }
    
    fn create_kanban_board() -> View {
        View {
            id: ViewId::new(),
            view_type: "JIRA.KanbanBoard".to_string(),
            target: ViewTarget::Query("project = PROJ".to_string()),
            projection: Projection {
                projection_type: "Board.Columns".to_string(),
                fields: vec!["status", "summary", "assignee"].into_iter().map(String::from).collect(),
                transformations: vec![
                    Transformation {
                        transform_type: "GroupBy".to_string(),
                        expression: "status".to_string(),
                    },
                ],
            },
            filters: vec![],
            presentation: Presentation {
                presentation_type: "Kanban.Swimlanes".to_string(),
                layout: {
                    let mut layout = PropertyGraph::new();
                    layout.set_text("columns", "To Do,In Progress,Done");
                    layout
                },
                styling: PropertyGraph::new(),
                interactions: vec![
                    Interaction {
                        interaction_type: "Drag".to_string(),
                        trigger: "card".to_string(),
                        action: "change_status".to_string(),
                    },
                ],
            },
        }
    }
    
    fn create_jira_workflow_config() -> Configuration {
        Configuration {
            id: ConfigurationId::new(),
            config_type: "JIRA.WorkflowDefinition".to_string(),
            schema: Schema {
                schema_type: "Workflow.States".to_string(),
                definition: PropertyGraph::new(),
            },
            values: {
                let mut values = PropertyGraph::new();
                values.set("states", Property::Collection(vec![
                    Property::Value(Value::Text("To Do".to_string())),
                    Property::Value(Value::Text("In Progress".to_string())),
                    Property::Value(Value::Text("Done".to_string())),
                ]));
                values
            },
            validation: vec![],
        }
    }
}

/// Example 3: Local filesystem as Layer 3 Workspace
pub mod filesystem_workflow {
    use super::*;
    
    pub fn create_filesystem_workspace() -> Workspace {
        Workspace {
            id: WorkspaceId::new(),
            workspace_type: "FileSystem.Directory".to_string(),
            state: "Mounted".to_string(),
            contents: vec![
                WorkspaceItem::Entity(EntityId::new()), // Files as entities
                WorkspaceItem::Workspace(WorkspaceId::new()), // Subdirectories
            ],
            organization: Organization {
                organization_type: "Hierarchical.FileSystem".to_string(),
                structure: Structure::Hierarchical(HierarchyNode {
                    node_type: "Directory".to_string(),
                    children: vec![],
                    properties: {
                        let mut props = PropertyGraph::new();
                        props.set_text("path", "/research/genealogy");
                        props
                    },
                }),
                rules: vec![
                    OrganizationRule {
                        rule_type: "Naming.Convention".to_string(),
                        expression: "filename.matches('[A-Za-z0-9_-]+')".to_string(),
                        action: "Suggest rename".to_string(),
                    },
                ],
            },
            governance: Governance {
                governance_type: "OS.Permissions".to_string(),
                policies: vec![],
                permissions: PermissionSystem {
                    system_type: "POSIX".to_string(),
                    permissions: HashMap::new(),
                    roles: HashMap::new(),
                },
            },
            behaviors: vec![
                Behavior {
                    behavior_type: "FileWatch".to_string(),
                    triggers: vec![
                        Trigger {
                            trigger_type: "File.Modified".to_string(),
                            expression: "*.md".to_string(),
                        },
                    ],
                    actions: vec![
                        Action {
                            action_type: "Index.Update".to_string(),
                            parameters: PropertyGraph::new(),
                        },
                    ],
                    conditions: vec![],
                },
            ],
            properties: PropertyGraph::new(),
            contexts: vec![],
            meta: MetaInfo::now(),
        }
    }
}

/// Example 4: Research workspace combining multiple systems
pub mod integrated_research_workspace {
    use super::*;
    
    pub fn create_integrated_workspace() -> Workspace {
        let mut integrated = Workspace {
            id: WorkspaceId::new(),
            workspace_type: "Research.Integrated".to_string(),
            state: "Active".to_string(),
            contents: vec![],
            organization: Organization {
                organization_type: "Multi.System".to_string(),
                structure: Structure::Tagged(TagSystem {
                    tags: {
                        let mut tags = HashMap::new();
                        tags.insert("source".to_string(), TagInfo {
                            tag_type: "Category".to_string(),
                            color: Some("#blue".to_string()),
                            icon: Some("📄".to_string()),
                            rules: vec![],
                        });
                        tags.insert("hypothesis".to_string(), TagInfo {
                            tag_type: "Status".to_string(),
                            color: Some("#yellow".to_string()),
                            icon: Some("💡".to_string()),
                            rules: vec![],
                        });
                        tags
                    },
                    relationships: vec![],
                }),
                rules: vec![],
            },
            governance: create_research_governance(),
            behaviors: create_research_behaviors(),
            properties: PropertyGraph::new(),
            contexts: vec![],
            meta: MetaInfo::now(),
        };
        
        // Integrate multiple workspaces
        integrated.contents.push(WorkspaceItem::Workspace(WorkspaceId::new())); // GitHub
        integrated.contents.push(WorkspaceItem::Workspace(WorkspaceId::new())); // JIRA
        integrated.contents.push(WorkspaceItem::Workspace(WorkspaceId::new())); // Local files
        
        // Add research-specific tools
        integrated.contents.push(WorkspaceItem::Tool(create_evidence_analyzer_tool().id));
        integrated.contents.push(WorkspaceItem::View(create_research_dashboard().id));
        
        integrated
    }
    
    fn create_research_governance() -> Governance {
        Governance {
            governance_type: "Research.Ethics".to_string(),
            policies: vec![
                Policy {
                    policy_type: "Privacy".to_string(),
                    rules: vec!["Living person data requires consent".to_string()],
                    enforcement: Enforcement::Required,
                },
                Policy {
                    policy_type: "Attribution".to_string(),
                    rules: vec!["All sources must be cited".to_string()],
                    enforcement: Enforcement::Required,
                },
            ],
            permissions: PermissionSystem {
                system_type: "Research.Access".to_string(),
                permissions: HashMap::new(),
                roles: HashMap::new(),
            },
        }
    }
    
    fn create_research_behaviors() -> Vec<Behavior> {
        vec![
            Behavior {
                behavior_type: "Evidence.Correlation".to_string(),
                triggers: vec![
                    Trigger {
                        trigger_type: "Evidence.Added".to_string(),
                        expression: "type:document".to_string(),
                    },
                ],
                actions: vec![
                    Action {
                        action_type: "Analyze.Correlations".to_string(),
                        parameters: PropertyGraph::new(),
                    },
                    Action {
                        action_type: "Suggest.Links".to_string(),
                        parameters: PropertyGraph::new(),
                    },
                ],
                conditions: vec![],
            },
        ]
    }
    
    fn create_evidence_analyzer_tool() -> Tool {
        Tool {
            id: ToolId::new(),
            tool_type: "Analysis.Evidence".to_string(),
            capabilities: vec![
                ToolCapability {
                    capability_type: "Correlation.Detection".to_string(),
                    inputs: vec!["evidence_set".to_string()],
                    outputs: vec!["correlations".to_string()],
                    constraints: vec![],
                },
                ToolCapability {
                    capability_type: "Conflict.Detection".to_string(),
                    inputs: vec!["evidence_set".to_string()],
                    outputs: vec!["conflicts".to_string()],
                    constraints: vec![],
                },
            ],
            interface: Interface {
                interface_type: "API.REST".to_string(),
                endpoints: vec![],
            },
            configuration: Configuration {
                id: ConfigurationId::new(),
                config_type: "Analysis.Settings".to_string(),
                schema: Schema {
                    schema_type: "JSON".to_string(),
                    definition: PropertyGraph::new(),
                },
                values: PropertyGraph::new(),
                validation: vec![],
            },
        }
    }
    
    fn create_research_dashboard() -> View {
        View {
            id: ViewId::new(),
            view_type: "Dashboard.Research".to_string(),
            target: ViewTarget::Multiple(vec![
                ViewTarget::Process(ProcessId::new()), // GPS process
                ViewTarget::Workspace(WorkspaceId::new()), // Evidence
            ]),
            projection: Projection {
                projection_type: "Dashboard.Widgets".to_string(),
                fields: vec![],
                transformations: vec![],
            },
            filters: vec![],
            presentation: Presentation {
                presentation_type: "Web.Dashboard".to_string(),
                layout: {
                    let mut layout = PropertyGraph::new();
                    layout.set("widgets", Property::Collection(vec![
                        Property::Value(Value::Text("GPS Progress".to_string())),
                        Property::Value(Value::Text("Evidence Map".to_string())),
                        Property::Value(Value::Text("Active Hypotheses".to_string())),
                    ]));
                    layout
                },
                styling: PropertyGraph::new(),
                interactions: vec![],
            },
        }
    }
}

// Helper to create temporal value for "now"
impl TemporalValue {
    fn now() -> Self {
        TemporalValue::Instant(TemporalInstant {
            expressions: vec![CalendarExpression::Gregorian {
                year: 2025,
                month: Some(8),
                day: Some(1),
            }],
            precision: TemporalPrecision::Day,
            quality: TemporalQuality::Exact,
        })
    }
}

impl MetaInfo {
    fn now() -> Self {
        MetaInfo {
            created_at: TemporalValue::now(),
            created_by: AgentId::new(),
            updated_at: TemporalValue::now(),
            updated_by: AgentId::new(),
            version: 1,
            transaction_id: Some(Uuid::new_v4()),
        }
    }
}

// Placeholder ID generators
impl ProcessId {
    fn new() -> Self { ProcessId(Uuid::new_v4()) }
}
impl ActivityId {
    fn new() -> Self { ActivityId(Uuid::new_v4()) }
}
impl AgentId {
    fn new() -> Self { AgentId(Uuid::new_v4()) }
}
impl ProductId {
    fn new() -> Self { ProductId(Uuid::new_v4()) }
}
impl MethodologyId {
    fn new() -> Self { MethodologyId(Uuid::new_v4()) }
}
impl WorkspaceId {
    fn new() -> Self { WorkspaceId(Uuid::new_v4()) }
}
impl ConfigurationId {
    fn new() -> Self { ConfigurationId(Uuid::new_v4()) }
}
impl ViewId {
    fn new() -> Self { ViewId(Uuid::new_v4()) }
}
impl ToolId {
    fn new() -> Self { ToolId(Uuid::new_v4()) }
}
impl EntityId {
    fn new() -> Self { EntityId(Uuid::new_v4()) }
}

use uuid::Uuid;