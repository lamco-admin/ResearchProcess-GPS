// Governance - Who can do what in a workspace

use crate::common::AgentId;
use indexmap::IndexMap;
use serde::{Serialize, Deserialize};

// Re-export Enforcement from layer2
pub use crate::layer2::methodology::Enforcement;

/// Governance system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Governance {
    pub governance_type: String,
    pub policies: Vec<Policy>,
    pub permissions: PermissionSystem,
}

/// Policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub policy_type: String,
    pub rules: Vec<String>,
    pub enforcement: Enforcement,
}

/// Permission system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSystem {
    pub system_type: String,
    pub permissions: IndexMap<AgentId, Vec<Permission>>,
    pub roles: IndexMap<String, Vec<Permission>>,
}

/// Permission definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub permission_type: String,
    pub resource: String,
    pub actions: Vec<String>,
    pub constraints: Vec<String>,
}

impl Default for Governance {
    fn default() -> Self {
        Governance {
            governance_type: "Open".to_string(),
            policies: Vec::new(),
            permissions: PermissionSystem {
                system_type: "Default".to_string(),
                permissions: IndexMap::new(),
                roles: IndexMap::new(),
            },
        }
    }
}

impl Governance {
    /// Create a new governance system
    pub fn new(governance_type: impl Into<String>) -> Self {
        Governance {
            governance_type: governance_type.into(),
            policies: Vec::new(),
            permissions: PermissionSystem {
                system_type: "Custom".to_string(),
                permissions: IndexMap::new(),
                roles: IndexMap::new(),
            },
        }
    }
    
    /// Add a policy
    pub fn add_policy(&mut self, policy: Policy) {
        self.policies.push(policy);
    }
    
    /// Grant permission to an agent
    pub fn grant_permission(&mut self, agent: AgentId, permission: Permission) {
        self.permissions
            .permissions
            .entry(agent)
            .or_insert_with(Vec::new)
            .push(permission);
    }
    
    /// Define a role with permissions
    pub fn define_role(&mut self, role: impl Into<String>, permissions: Vec<Permission>) {
        self.permissions.roles.insert(role.into(), permissions);
    }
    
    /// Check if agent has permission
    pub fn has_permission(&self, agent: AgentId, resource: &str, action: &str) -> bool {
        // Check direct permissions
        if let Some(perms) = self.permissions.permissions.get(&agent) {
            if perms.iter().any(|p| p.allows(resource, action)) {
                return true;
            }
        }
        
        // In real implementation, would also check role-based permissions
        false
    }
}

impl Policy {
    /// Create a new policy
    pub fn new(policy_type: impl Into<String>) -> Self {
        Policy {
            policy_type: policy_type.into(),
            rules: Vec::new(),
            enforcement: Enforcement::Required,
        }
    }
    
    /// Add a rule
    pub fn add_rule(&mut self, rule: impl Into<String>) {
        self.rules.push(rule.into());
    }
}

impl Permission {
    /// Create a new permission
    pub fn new(permission_type: impl Into<String>, resource: impl Into<String>) -> Self {
        Permission {
            permission_type: permission_type.into(),
            resource: resource.into(),
            actions: Vec::new(),
            constraints: Vec::new(),
        }
    }
    
    /// Add allowed action
    pub fn allow_action(&mut self, action: impl Into<String>) {
        self.actions.push(action.into());
    }
    
    /// Add constraint
    pub fn add_constraint(&mut self, constraint: impl Into<String>) {
        self.constraints.push(constraint.into());
    }
    
    /// Check if permission allows action on resource
    pub fn allows(&self, resource: &str, action: &str) -> bool {
        (self.resource == "*" || self.resource == resource) &&
        (self.actions.contains(&"*".to_string()) || self.actions.contains(&action.to_string()))
    }
}

/// Common governance patterns
impl Governance {
    /// Create GitHub-style permissions
    pub fn github_style() -> Self {
        let mut gov = Governance::new("GitHub.Permissions");
        
        // Define roles
        gov.define_role("Owner", vec![
            Permission {
                permission_type: "Full".to_string(),
                resource: "*".to_string(),
                actions: vec!["*".to_string()],
                constraints: vec![],
            },
        ]);
        
        gov.define_role("Contributor", vec![
            Permission {
                permission_type: "Write".to_string(),
                resource: "branches/*".to_string(),
                actions: vec!["create".to_string(), "push".to_string()],
                constraints: vec!["Not to protected branches".to_string()],
            },
            Permission {
                permission_type: "Read".to_string(),
                resource: "*".to_string(),
                actions: vec!["read".to_string()],
                constraints: vec![],
            },
        ]);
        
        gov
    }
    
    /// Create research ethics governance
    pub fn research_ethics() -> Self {
        let mut gov = Governance::new("Research.Ethics");
        
        gov.add_policy(Policy {
            policy_type: "Privacy".to_string(),
            rules: vec![
                "Living person data requires consent".to_string(),
                "Sensitive data must be anonymized".to_string(),
            ],
            enforcement: Enforcement::Required,
        });
        
        gov.add_policy(Policy {
            policy_type: "Attribution".to_string(),
            rules: vec![
                "All sources must be cited".to_string(),
                "Contributors must be acknowledged".to_string(),
            ],
            enforcement: Enforcement::Required,
        });
        
        gov
    }
}