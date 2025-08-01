//! Research Log Module - WASM version for sandboxed execution
//! 
//! This is a simplified version of the Research Log module that runs in WASM.
//! Key differences from native version:
//! - No async/await (WASM limitations)
//! - No tokio runtime
//! - Simplified data structures
//! - Direct exports for WASI compatibility

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Module state stored globally (WASM is single-threaded)
static mut MODULE_STATE: Option<ResearchLogState> = None;

/// Safe accessor for module state that avoids creating mutable references
fn with_state<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce(&ResearchLogState) -> R,
{
    unsafe {
        let state_ptr = &raw const MODULE_STATE;
        match (*state_ptr).as_ref() {
            Some(state) => Ok(f(state)),
            None => Err("Module not initialized".to_string()),
        }
    }
}

/// Safe mutable accessor for module state
fn with_state_mut<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce(&mut ResearchLogState) -> R,
{
    unsafe {
        let state_ptr = &raw mut MODULE_STATE;
        match (*state_ptr).as_mut() {
            Some(state) => Ok(f(state)),
            None => Err("Module not initialized".to_string()),
        }
    }
}

/// Research log state
#[derive(Default)]
struct ResearchLogState {
    logs: HashMap<String, Vec<LogEntry>>,
    active_log: Option<String>,
    actor_id: Option<String>,
}

/// Research log entry (simplified for WASM)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: u64,  // Unix timestamp (simpler than DateTime)
    pub researcher_id: String,
    pub entry_type: LogEntryType,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub references: Vec<String>,
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

/// Host functions that WASM can call
extern "C" {
    /// Log a message to the host (for now just print)
    fn host_log(level: *const c_char, message: *const c_char);
}

/// Initialize the module
#[no_mangle]
pub extern "C" fn initialize(actor_id_ptr: *const c_char, actor_id_len: c_int) -> *mut c_char {
    unsafe {
        let actor_id_slice = std::slice::from_raw_parts(actor_id_ptr as *const u8, actor_id_len as usize);
        let actor_id = match std::str::from_utf8(actor_id_slice) {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        
        MODULE_STATE = Some(ResearchLogState {
            logs: HashMap::new(),
            active_log: None,
            actor_id: Some(actor_id.to_string()),
        });
    }
    
    log_message("info", "Research Log WASM module initialized");
    
    let result = json!({
        "status": "initialized",
        "version": "0.1.0"
    }).to_string();
    
    string_to_ptr(result)
}

/// Execute a command
#[no_mangle]
pub extern "C" fn execute_command(
    command_ptr: *const c_char, 
    command_len: c_int,
    args_ptr: *const c_char,
    args_len: c_int
) -> *mut c_char {
    let args_value: Value = match serde_json::from_str(args) {
        Ok(v) => v,
        Err(e) => {
            return json!({
                "error": format!("Invalid arguments: {}", e)
            }).to_string();
        }
    };
    
    let result = match command {
        "create_log" => handle_create_log(args_value),
        "add_entry" => handle_add_entry(args_value),
        "analyze_activity" => handle_analyze_activity(args_value),
        _ => {
            json!({
                "error": format!("Unknown command: {}", command)
            })
        }
    };
    
    result.to_string()
}

/// Handle create log command
fn handle_create_log(args: Value) -> Value {
    let title = match args.get("title").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => return json!({"error": "Missing title parameter"}),
    };
    
    let result = with_state_mut(|state| {
        let log_id = host_generate_uuid();
        state.logs.insert(log_id.clone(), Vec::new());
        state.active_log = Some(log_id.clone());
        
        // Emit event
        let event_data = json!({
            "log_type": "research",
            "researcher_id": state.actor_id.as_ref().unwrap_or(&String::new()),
            "title": title
        });
        host_emit_event("ResearchLog.Created", &event_data.to_string());
        
        json!({
            "log_id": log_id,
            "status": "created"
        })
    });
    
    match result {
        Ok(value) => value,
        Err(err) => json!({"error": err}),
    }
}

/// Handle add entry command
fn handle_add_entry(args: Value) -> Value {
    let entry_type: LogEntryType = match args.get("type") {
        Some(v) => match serde_json::from_value(v.clone()) {
            Ok(t) => t,
            Err(_) => LogEntryType::Note,
        },
        None => LogEntryType::Note,
    };
    
    let title = match args.get("title").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => return json!({"error": "Missing title"}),
    };
    
    let content = match args.get("content").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return json!({"error": "Missing content"}),
    };
    
    let tags = args.get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect())
        .unwrap_or_default();
    
    let result = with_state_mut(|state| {
        let log_id = match &state.active_log {
            Some(id) => id.clone(),
            None => return Err("No active log".to_string()),
        };
        
        let entry = LogEntry {
            id: host_generate_uuid(),
            timestamp: host_get_timestamp(),
            researcher_id: state.actor_id.clone().unwrap_or_default(),
            entry_type,
            title: title.to_string(),
            content: content.to_string(),
            tags,
            references: Vec::new(),
        };
        
        let entry_id = entry.id.clone();
        
        if let Some(log_entries) = state.logs.get_mut(&log_id) {
            log_entries.push(entry);
            
            Ok(json!({
                "entry_id": entry_id,
                "status": "added"
            }))
        } else {
            Err("Log not found".to_string())
        }
    });
    
    match result {
        Ok(Ok(value)) => value,
        Ok(Err(err)) => json!({"error": err}),
        Err(err) => json!({"error": err}),
    }
}

/// Analyze research activity
fn handle_analyze_activity(_args: Value) -> Value {
    let result = with_state(|state| {
        let mut total_entries = 0;
        let mut entry_types: HashMap<String, usize> = HashMap::new();
        let mut tags: HashMap<String, usize> = HashMap::new();
        
        for entries in state.logs.values() {
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
            "total_logs": state.logs.len(),
            "total_entries": total_entries,
            "entry_types": entry_types,
            "top_tags": tags,
        })
    });
    
    match result {
        Ok(value) => value,
        Err(err) => json!({"error": err}),
    }
}

/// Handle incoming events from the host
#[wasm_bindgen]
pub fn handle_event(event_type: &str, event_data: &str) -> String {
    host_log("debug", &format!("Received event: {} with data: {}", event_type, event_data));
    
    // Handle relevant events here
    // For now, just acknowledge
    json!({
        "status": "handled",
        "event_type": event_type
    }).to_string()
}

/// Shutdown the module
#[wasm_bindgen]
pub fn shutdown() -> String {
    host_log("info", "Shutting down Research Log WASM module");
    
    unsafe {
        MODULE_STATE = None;
    }
    
    json!({
        "status": "shutdown"
    }).to_string()
}

