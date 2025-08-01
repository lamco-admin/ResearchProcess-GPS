//! Research Log Module - Advanced research logging and analysis

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use rp_core::EntityId;
use rp_events::{DomainEvent, ResearchLogEvent};
use rp_modules::{
    ModuleContext, ModuleMessage, ModuleMetadata,
    module::{ResearchModule, ModuleType, ModuleStatus},
    Result as ModuleResult, ModuleError,
};

/// Research log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub researcher_id: EntityId,
    pub entry_type: LogEntryType,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub references: Vec<EntityId>,
    pub metadata: HashMap<String, Value>,
}

/// Types of log entries
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogEntryType {
    Note,
    Observation,
    Question,
    Hypothesis,
    Conclusion,
    Task,
    Progress,
}

/// Research log module implementation
pub struct ResearchLogModule {
    metadata: ModuleMetadata,
    context: Option<ModuleContext>,
    logs: Arc<RwLock<HashMap<Uuid, Vec<LogEntry>>>>,
    active_log: Option<Uuid>,
}

impl ResearchLogModule {
    /// Create a new research log module
    pub fn new() -> Self {
        let metadata = ModuleMetadata {
            id: Uuid::new_v4(),
            name: "research-log".to_string(),
            version: "0.1.0".to_string(),
            description: "Advanced research logging and analysis module".to_string(),
            author: "ResearchProcess-GPS Team".to_string(),
            license: "MIT".to_string(),
            module_type: ModuleType::Native,
            loaded_at: Utc::now(),
            status: ModuleStatus::Loaded,
        };

        Self {
            metadata,
            context: None,
            logs: Arc::new(RwLock::new(HashMap::new())),
            active_log: None,
        }
    }

    /// Handle create log command
    async fn handle_create_log(&mut self, args: Value) -> ModuleResult<Value> {
        let _title = args.get("title")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ModuleError::ExecutionError("Missing title parameter".to_string()))?;

        let log_id = Uuid::new_v4();
        self.logs.write().await.insert(log_id, Vec::new());
        self.active_log = Some(log_id);

        // Send log creation event
        if let Some(context) = &self.context {
            let event = DomainEvent::ResearchLog(ResearchLogEvent::Created {
                log_type: "research".to_string(),
                researcher_id: *context.actor_id.as_uuid(),
            });

            let _ = context.send_to_host(ModuleMessage::EmitEvent { event }).await;
        }

        Ok(json!({
            "log_id": log_id,
            "status": "created"
        }))
    }

    /// Handle add entry command
    async fn handle_add_entry(&mut self, args: Value) -> ModuleResult<Value> {
        let log_id = self.active_log
            .ok_or_else(|| ModuleError::ExecutionError("No active log".to_string()))?;

        let entry_type: LogEntryType = serde_json::from_value(
            args.get("type").cloned().unwrap_or(json!("note"))
        ).map_err(|e| ModuleError::ExecutionError(format!("Invalid entry type: {}", e)))?;

        let title = args.get("title")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ModuleError::ExecutionError("Missing title".to_string()))?;

        let content = args.get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ModuleError::ExecutionError("Missing content".to_string()))?;

        let entry = LogEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            researcher_id: self.context.as_ref()
                .map(|c| c.actor_id)
                .unwrap_or_else(EntityId::new),
            entry_type,
            title: title.to_string(),
            content: content.to_string(),
            tags: args.get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect())
                .unwrap_or_default(),
            references: Vec::new(),
            metadata: HashMap::new(),
        };

        let entry_id = entry.id;
        self.logs.write().await
            .get_mut(&log_id)
            .ok_or_else(|| ModuleError::ExecutionError("Log not found".to_string()))?
            .push(entry);

        Ok(json!({
            "entry_id": entry_id,
            "status": "added"
        }))
    }

    /// Analyze research activity
    async fn handle_analyze_activity(&self, _args: Value) -> ModuleResult<Value> {
        let logs = self.logs.read().await;
        
        let mut total_entries = 0;
        let mut entry_types: HashMap<String, usize> = HashMap::new();
        let mut tags: HashMap<String, usize> = HashMap::new();

        for entries in logs.values() {
            total_entries += entries.len();
            
            for entry in entries {
                let type_name = match &entry.entry_type {
                    LogEntryType::Note => "note",
                    LogEntryType::Observation => "observation",
                    LogEntryType::Question => "question",
                    LogEntryType::Hypothesis => "hypothesis",
                    LogEntryType::Conclusion => "conclusion",
                    LogEntryType::Task => "task",
                    LogEntryType::Progress => "progress",
                };
                *entry_types.entry(type_name.to_string()).or_insert(0) += 1;

                for tag in &entry.tags {
                    *tags.entry(tag.clone()).or_insert(0) += 1;
                }
            }
        }

        Ok(json!({
            "total_logs": logs.len(),
            "total_entries": total_entries,
            "entry_types": entry_types,
            "top_tags": tags,
        }))
    }
}

#[async_trait]
impl ResearchModule for ResearchLogModule {
    fn metadata(&self) -> &ModuleMetadata {
        &self.metadata
    }

    async fn initialize(&mut self, context: ModuleContext) -> ModuleResult<()> {
        tracing::info!("Initializing Research Log module");
        self.context = Some(context);
        Ok(())
    }

    async fn handle_message(&mut self, message: ModuleMessage) -> ModuleResult<()> {
        match message {
            ModuleMessage::Event(event) => {
                tracing::debug!("Received event: {}", event.event_type());
                // Handle relevant events
                Ok(())
            }
            ModuleMessage::ConfigUpdate(config) => {
                tracing::info!("Configuration updated: {:?}", config);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn execute_command(
        &mut self,
        command: &str,
        args: Value
    ) -> ModuleResult<Value> {
        match command {
            "create_log" => self.handle_create_log(args).await,
            "add_entry" => self.handle_add_entry(args).await,
            "analyze_activity" => self.handle_analyze_activity(args).await,
            _ => Err(ModuleError::ExecutionError(
                format!("Unknown command: {}", command)
            )),
        }
    }

    async fn shutdown(&mut self) -> ModuleResult<()> {
        tracing::info!("Shutting down Research Log module");
        Ok(())
    }
}

/// Module creation function for dynamic loading
#[no_mangle]
pub extern "C" fn _create_module() -> *mut std::ffi::c_void {
    let module = Box::new(ResearchLogModule::new());
    let module_box: Box<dyn ResearchModule> = module;
    Box::into_raw(Box::new(module_box)) as *mut std::ffi::c_void
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_module_creation() {
        let module = ResearchLogModule::new();
        assert_eq!(module.metadata().name, "research-log");
        assert_eq!(module.metadata().version, "0.1.0");
    }

    #[tokio::test]
    async fn test_create_log() {
        let mut module = ResearchLogModule::new();
        let args = json!({ "title": "Test Research Log" });
        
        let result = module.handle_create_log(args).await.unwrap();
        assert!(result.get("log_id").is_some());
        assert_eq!(result.get("status").unwrap(), "created");
    }

    #[tokio::test]
    async fn test_add_entry() {
        let mut module = ResearchLogModule::new();
        
        // First create a log
        let create_args = json!({ "title": "Test Log" });
        module.handle_create_log(create_args).await.unwrap();
        
        // Then add an entry
        let entry_args = json!({
            "type": "note",
            "title": "Test Entry",
            "content": "This is a test entry",
            "tags": ["test", "example"]
        });
        
        let result = module.handle_add_entry(entry_args).await.unwrap();
        assert!(result.get("entry_id").is_some());
        assert_eq!(result.get("status").unwrap(), "added");
    }
}