//! Research Log Module - Advanced research logging and analysis
//! Now using message-based FFI architecture

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

/// Research log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub researcher_id: String,
    pub entry_type: LogEntryType,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub references: Vec<String>,
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
struct ResearchLogModule {
    initialized: bool,
    logs: Arc<Mutex<HashMap<Uuid, Vec<LogEntry>>>>,
    active_log: Option<Uuid>,
    module_id: Uuid,
}

impl ResearchLogModule {
    fn new() -> Self {
        Self {
            initialized: false,
            logs: Arc::new(Mutex::new(HashMap::new())),
            active_log: None,
            module_id: Uuid::new_v4(),
        }
    }

    fn handle_message(&mut self, msg_type: &str, payload: Value) -> Value {
        match msg_type {
            "init" => self.handle_init(payload),
            "command" => self.handle_command(payload),
            "query" => self.handle_query(payload),
            "mutation" => self.handle_mutation(payload),
            "event" => self.handle_event(payload),
            _ => json!({
                "status": "error",
                "error": format!("Unknown message type: {}", msg_type)
            })
        }
    }

    fn handle_init(&mut self, _payload: Value) -> Value {
        self.initialized = true;
        json!({
            "status": "success",
            "message": "Research Log module initialized",
            "module_id": self.module_id,
            "capabilities": [
                "create_log",
                "add_entry",
                "analyze_activity",
                "query_logs",
                "export_data"
            ]
        })
    }

    fn handle_command(&mut self, payload: Value) -> Value {
        if !self.initialized {
            return json!({
                "status": "error",
                "error": "Module not initialized"
            });
        }

        let command = match payload.get("name").and_then(|v| v.as_str()) {
            Some(cmd) => cmd,
            None => return json!({
                "status": "error",
                "error": "Missing command name"
            })
        };

        let args = payload.get("args").cloned().unwrap_or(json!({}));

        match command {
            "create_log" => self.cmd_create_log(args),
            "add_entry" => self.cmd_add_entry(args),
            "analyze_activity" => self.cmd_analyze_activity(args),
            "set_active_log" => self.cmd_set_active_log(args),
            _ => json!({
                "status": "error",
                "error": format!("Unknown command: {}", command)
            })
        }
    }

    fn handle_query(&self, payload: Value) -> Value {
        // In a real implementation, this would query the host for Layer 1 data
        json!({
            "status": "error",
            "error": "Query forwarding not implemented in this example"
        })
    }

    fn handle_mutation(&self, payload: Value) -> Value {
        // In a real implementation, this would send mutations to the host
        json!({
            "status": "error",
            "error": "Mutation forwarding not implemented in this example"
        })
    }

    fn handle_event(&mut self, payload: Value) -> Value {
        // Handle events from the host system
        if let Some(event_type) = payload.get("event").and_then(|v| v.as_str()) {
            match event_type {
                "research_log.created" => {
                    // Could react to logs created by other modules
                }
                "research_session.started" => {
                    // Could auto-create a log for the session
                }
                _ => {}
            }
        }
        json!({ "status": "acknowledged" })
    }

    fn cmd_create_log(&mut self, args: Value) -> Value {
        let title = match args.get("title").and_then(|v| v.as_str()) {
            Some(t) => t,
            None => return json!({
                "status": "error",
                "error": "Missing title parameter"
            })
        };

        let log_id = Uuid::new_v4();
        self.logs.lock().unwrap().insert(log_id, Vec::new());
        self.active_log = Some(log_id);

        // In a real implementation, we would emit an event through the host
        json!({
            "status": "created",
            "log_id": log_id,
            "title": title,
            "message": "Research log created",
            "emit_event": {
                "type": "research_log.created",
                "data": {
                    "log_id": log_id,
                    "title": title
                }
            }
        })
    }

    fn cmd_add_entry(&mut self, args: Value) -> Value {
        let log_id = self.active_log.or_else(|| {
            args.get("log_id")
                .and_then(|v| v.as_str())
                .and_then(|s| Uuid::parse_str(s).ok())
        });

        let log_id = match log_id {
            Some(id) => id,
            None => return json!({
                "status": "error",
                "error": "No active log or log_id specified"
            })
        };

        let entry_type: LogEntryType = match args.get("type") {
            Some(v) => match serde_json::from_value(v.clone()) {
                Ok(t) => t,
                Err(e) => return json!({
                    "status": "error",
                    "error": format!("Invalid entry type: {}", e)
                })
            },
            None => LogEntryType::Note
        };

        let title = match args.get("title").and_then(|v| v.as_str()) {
            Some(t) => t,
            None => return json!({
                "status": "error",
                "error": "Missing title"
            })
        };

        let content = match args.get("content").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => return json!({
                "status": "error",
                "error": "Missing content"
            })
        };

        let entry = LogEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            researcher_id: args.get("researcher_id")
                .and_then(|v| v.as_str())
                .unwrap_or("system")
                .to_string(),
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
        
        let mut logs = self.logs.lock().unwrap();
        match logs.get_mut(&log_id) {
            Some(entries) => {
                entries.push(entry);
                json!({
                    "status": "success",
                    "entry_id": entry_id,
                    "log_id": log_id,
                    "message": "Entry added to log"
                })
            }
            None => json!({
                "status": "error",
                "error": "Log not found"
            })
        }
    }

    fn cmd_analyze_activity(&self, _args: Value) -> Value {
        let logs = self.logs.lock().unwrap();
        
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

        json!({
            "status": "success",
            "analysis": {
                "total_logs": logs.len(),
                "total_entries": total_entries,
                "entry_types": entry_types,
                "top_tags": tags,
                "active_log": self.active_log
            }
        })
    }

    fn cmd_set_active_log(&mut self, args: Value) -> Value {
        match args.get("log_id").and_then(|v| v.as_str()) {
            Some(id_str) => match Uuid::parse_str(id_str) {
                Ok(id) => {
                    if self.logs.lock().unwrap().contains_key(&id) {
                        self.active_log = Some(id);
                        json!({
                            "status": "success",
                            "active_log": id
                        })
                    } else {
                        json!({
                            "status": "error",
                            "error": "Log not found"
                        })
                    }
                }
                Err(e) => json!({
                    "status": "error",
                    "error": format!("Invalid UUID: {}", e)
                })
            },
            None => json!({
                "status": "error",
                "error": "Missing log_id"
            })
        }
    }
}

/// Create a new module instance
#[no_mangle]
pub extern "C" fn create_module() -> *mut std::ffi::c_void {
    let module = Box::new(ResearchLogModule::new());
    Box::into_raw(module) as *mut std::ffi::c_void
}

/// Destroy a module instance
#[no_mangle]
pub extern "C" fn destroy_module(module: *mut std::ffi::c_void) {
    if module.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(module as *mut ResearchLogModule);
        // Module is dropped here
    }
}

/// Handle a message - main communication interface
#[no_mangle]
pub extern "C" fn handle_message(
    module: *mut std::ffi::c_void,
    message_type: *const c_char,
    payload: *const c_char,
) -> *mut c_char {
    if module.is_null() || message_type.is_null() || payload.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        // Get module reference
        let module = &mut *(module as *mut ResearchLogModule);
        
        // Parse message type
        let msg_type = match CStr::from_ptr(message_type).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        
        // Parse payload
        let payload_str = match CStr::from_ptr(payload).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        
        let payload_json: Value = match serde_json::from_str(payload_str) {
            Ok(v) => v,
            Err(e) => {
                let error_response = json!({
                    "status": "error",
                    "error": format!("Invalid JSON payload: {}", e)
                });
                return CString::new(error_response.to_string())
                    .unwrap_or_default()
                    .into_raw();
            }
        };
        
        // Handle the message
        let response = module.handle_message(msg_type, payload_json);
        
        // Convert response to C string
        CString::new(response.to_string())
            .unwrap_or_default()
            .into_raw()
    }
}

/// Free a string allocated by this module
#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
        // String is dropped here
    }
}

/// Get module metadata
#[no_mangle]
pub extern "C" fn get_module_metadata() -> *mut c_char {
    let metadata = json!({
        "name": "research-log",
        "version": "0.2.0",
        "description": "Advanced research logging and analysis module",
        "author": "ResearchProcess-GPS Team",
        "capabilities": [
            "create_log",
            "add_entry", 
            "analyze_activity",
            "query_logs",
            "export_data"
        ],
        "message_protocol_version": "1.0"
    });
    
    CString::new(metadata.to_string())
        .unwrap_or_default()
        .into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = ResearchLogModule::new();
        assert!(!module.initialized);
        assert!(module.active_log.is_none());
    }

    #[test]
    fn test_initialization() {
        let mut module = ResearchLogModule::new();
        let response = module.handle_message("init", json!({}));
        assert_eq!(response["status"], "success");
        assert!(module.initialized);
    }

    #[test]
    fn test_create_log() {
        let mut module = ResearchLogModule::new();
        module.handle_message("init", json!({}));
        
        let response = module.handle_message("command", json!({
            "name": "create_log",
            "args": { "title": "Test Research Log" }
        }));
        
        assert_eq!(response["status"], "success");
        assert!(response["log_id"].is_string());
        assert!(module.active_log.is_some());
    }
}