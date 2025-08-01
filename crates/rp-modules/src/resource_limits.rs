//! Resource limiting for modules

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Type alias for the test
pub type ModuleResourceLimits = ResourceLimits;

/// Resource limits for a module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory in bytes
    pub memory_bytes: usize,
    
    /// Maximum storage quota in bytes
    pub storage_bytes: usize,
    
    /// Maximum CPU time per call (in milliseconds for serialization)
    #[serde(rename = "cpu_time_ms")]
    pub cpu_time_ms: u64,
    
    /// Maximum number of concurrent operations
    pub max_concurrent_ops: u32,
    
    /// Maximum number of entity operations per minute
    pub entity_ops_per_minute: u32,
    
    /// Maximum number of events per minute
    pub events_per_minute: u32,
    
    /// For WASM: Maximum table elements
    #[serde(default = "default_table_elements")]
    pub table_elements: u32,
    
    /// For WASM: Maximum instances
    #[serde(default = "default_instances")]
    pub instances: u32,
}

fn default_table_elements() -> u32 {
    10000
}

fn default_instances() -> u32 {
    1
}

impl ResourceLimits {
    /// Get CPU time limit as Duration
    pub fn cpu_time_limit(&self) -> Duration {
        Duration::from_millis(self.cpu_time_ms)
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_bytes: 64 * 1024 * 1024, // 64 MB
            storage_bytes: 10 * 1024 * 1024, // 10 MB
            cpu_time_ms: 100,
            max_concurrent_ops: 10,
            entity_ops_per_minute: 1000,
            events_per_minute: 1000,
            table_elements: 10000,
            instances: 1,
        }
    }
}

/// Resource limiter for tracking usage
#[async_trait::async_trait]
pub trait ResourceLimiter: Send + Sync {
    /// Check if memory allocation is allowed
    async fn check_memory(&self, bytes: usize) -> bool;
    
    /// Check if storage operation is allowed
    async fn check_storage(&self, bytes: usize) -> bool;
    
    /// Check if CPU time is available
    async fn check_cpu_time(&self, duration: Duration) -> bool;
    
    /// Check if entity operation is allowed
    async fn check_entity_op(&self) -> bool;
    
    /// Check if event emission is allowed
    async fn check_event(&self) -> bool;
    
    /// Record memory usage
    async fn record_memory_usage(&self, bytes: usize);
    
    /// Record storage usage
    async fn record_storage_usage(&self, bytes: usize);
    
    /// Record CPU time usage
    async fn record_cpu_time(&self, duration: Duration);
    
    /// Record entity operation
    async fn record_entity_op(&self);
    
    /// Record event emission
    async fn record_event(&self);
}

/// Wasmtime resource limiter implementation
pub struct WasmtimeResourceLimiter {
    limits: ResourceLimits,
    current_memory: std::sync::atomic::AtomicUsize,
}

impl WasmtimeResourceLimiter {
    pub fn new(limits: ResourceLimits) -> Self {
        Self {
            limits,
            current_memory: std::sync::atomic::AtomicUsize::new(0),
        }
    }
}

impl wasmtime::ResourceLimiter for WasmtimeResourceLimiter {
    fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> anyhow::Result<bool> {
        let growth = desired.saturating_sub(current);
        let new_total = self.current_memory.load(std::sync::atomic::Ordering::Relaxed) + growth;
        
        if new_total > self.limits.memory_bytes {
            return Ok(false);
        }
        
        self.current_memory.store(new_total, std::sync::atomic::Ordering::Relaxed);
        Ok(true)
    }
    
    fn table_growing(
        &mut self,
        _current: u32,
        desired: u32,
        _maximum: Option<u32>,
    ) -> anyhow::Result<bool> {
        Ok(desired <= self.limits.table_elements)
    }
}