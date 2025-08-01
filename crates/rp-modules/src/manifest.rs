//! Module manifest parsing

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;

use crate::{ModuleCapabilities, ResourceLimits, Result, ModuleError};

/// Module manifest (module.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    /// Module information
    pub module: ModuleInfo,
    
    /// Dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    
    /// Required capabilities
    pub capabilities: CapabilityRequests,
    
    /// Resource requirements
    #[serde(default)]
    pub resources: ResourceRequests,
    
    /// Exported functionality
    #[serde(default)]
    pub exports: ModuleExports,
}

/// Module information section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    
    #[serde(rename = "type")]
    pub module_type: String, // "native" or "wasm"
}

/// Capability requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequests {
    #[serde(default)]
    pub entity_read: Vec<String>,
    
    #[serde(default)]
    pub entity_write: Vec<String>,
    
    #[serde(default)]
    pub entity_create: Vec<String>,
    
    #[serde(default)]
    pub entity_delete: Vec<String>,
    
    #[serde(default)]
    pub event_subscribe: Vec<String>,
    
    #[serde(default)]
    pub event_emit: Vec<String>,
    
    #[serde(default)]
    pub network_access: bool,
    
    #[serde(default)]
    pub filesystem_access: bool,
}

/// Resource requests
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceRequests {
    #[serde(default = "default_memory_limit")]
    pub memory_limit: String,
    
    #[serde(default = "default_storage_quota")]
    pub storage_quota: String,
    
    #[serde(default = "default_cpu_time_limit")]
    pub cpu_time_limit: String,
    
    #[serde(default)]
    pub max_concurrent_ops: Option<u32>,
}

/// Module exports
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModuleExports {
    #[serde(default)]
    pub commands: Vec<String>,
    
    #[serde(default)]
    pub ui_components: Vec<String>,
    
    #[serde(default)]
    pub api_endpoints: Vec<String>,
}

impl ModuleManifest {
    /// Load manifest from file
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| ModuleError::InvalidManifest(e.to_string()))?;
        
        Self::from_str(&content)
    }
    
    /// Parse manifest from string
    pub fn from_str(content: &str) -> Result<Self> {
        toml::from_str(content)
            .map_err(|e| ModuleError::InvalidManifest(e.to_string()))
    }
    
    /// Convert capability requests to ModuleCapabilities
    pub fn to_capabilities(&self) -> ModuleCapabilities {
        let mut caps = ModuleCapabilities::none();
        
        caps.entity_read.extend(self.capabilities.entity_read.iter().cloned());
        caps.entity_write.extend(self.capabilities.entity_write.iter().cloned());
        caps.entity_create.extend(self.capabilities.entity_create.iter().cloned());
        caps.entity_delete.extend(self.capabilities.entity_delete.iter().cloned());
        caps.event_subscribe.extend(self.capabilities.event_subscribe.iter().cloned());
        caps.event_emit.extend(self.capabilities.event_emit.iter().cloned());
        caps.network_access = self.capabilities.network_access;
        caps.filesystem_access = self.capabilities.filesystem_access;
        
        caps
    }
    
    /// Convert resource requests to ResourceLimits
    pub fn to_resource_limits(&self) -> Result<ResourceLimits> {
        let mut limits = ResourceLimits::default();
        
        limits.memory_bytes = parse_size(&self.resources.memory_limit)
            .ok_or_else(|| ModuleError::InvalidManifest(
                format!("Invalid memory limit: {}", self.resources.memory_limit)
            ))?;
        
        limits.storage_bytes = parse_size(&self.resources.storage_quota)
            .ok_or_else(|| ModuleError::InvalidManifest(
                format!("Invalid storage quota: {}", self.resources.storage_quota)
            ))?;
        
        limits.cpu_time_limit = parse_duration(&self.resources.cpu_time_limit)
            .ok_or_else(|| ModuleError::InvalidManifest(
                format!("Invalid CPU time limit: {}", self.resources.cpu_time_limit)
            ))?;
        
        if let Some(max_ops) = self.resources.max_concurrent_ops {
            limits.max_concurrent_ops = max_ops;
        }
        
        Ok(limits)
    }
}

fn default_memory_limit() -> String {
    "64MB".to_string()
}

fn default_storage_quota() -> String {
    "10MB".to_string()
}

fn default_cpu_time_limit() -> String {
    "100ms".to_string()
}

/// Parse size string (e.g., "64MB", "1GB")
fn parse_size(s: &str) -> Option<usize> {
    let s = s.trim();
    let (num_str, unit) = s.split_at(s.len() - 2);
    
    let num: usize = num_str.parse().ok()?;
    
    match unit.to_uppercase().as_str() {
        "KB" => Some(num * 1024),
        "MB" => Some(num * 1024 * 1024),
        "GB" => Some(num * 1024 * 1024 * 1024),
        _ => None,
    }
}

/// Parse duration string (e.g., "100ms", "5s")
fn parse_duration(s: &str) -> Option<std::time::Duration> {
    let s = s.trim();
    
    if s.ends_with("ms") {
        let num: u64 = s[..s.len()-2].parse().ok()?;
        Some(std::time::Duration::from_millis(num))
    } else if s.ends_with('s') {
        let num: u64 = s[..s.len()-1].parse().ok()?;
        Some(std::time::Duration::from_secs(num))
    } else {
        None
    }
}