//! Research Log Module - WASM version for sandboxed execution
//! 
//! This is a simplified version of the Research Log module that runs in WASM.
//! Key differences from native version:
//! - No async/await (WASM limitations)
//! - No tokio runtime
//! - Simplified data structures
//! - Direct exports for WASI compatibility
//! - Message-based communication protocol matching native modules

use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Convert a Rust string to a C string pointer
fn string_to_ptr(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Log a message (since we can't call host functions with strings directly)
fn log_message(level: &str, msg: &str) {
    println!("[{}] {}", level, msg);
}

/// Module state stored globally (WASM is single-threaded)
static mut MODULE_STATE: Option<ResearchLogModule> = None;

/// Safe mutable accessor for module state
fn with_state_mut<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce(&mut ResearchLogModule) -> R,
{
    unsafe {
        let state_ptr = &raw mut MODULE_STATE;
        
        // Auto-create module state if not exists
        if (*state_ptr).is_none() {
            *state_ptr = Some(ResearchLogModule::new());
        }
        
        match (*state_ptr).as_mut() {
            Some(state) => Ok(f(state)),
            None => Err("Module not initialized".to_string()),
        }
    }
}

/// Research log module implementation
struct ResearchLogModule {
    initialized: bool,
    logs: HashMap<String, Vec<LogEntry>>,
    active_log: Option<String>,
    module_id: String,
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

/// Simple function to generate UUIDs (simplified for WASM)
fn host_generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{:x}", timestamp)
}

/// Get current timestamp
fn host_get_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Emit an event (simplified for WASM - just log it)
fn host_emit_event(event_type: &str, event_data: &str) {
    log_message("event", &format!("{}: {}", event_type, event_data));
}

impl ResearchLogModule {
    fn new() -> Self {
        Self {
            initialized: false,
            logs: HashMap::new(),
            active_log: None,
            module_id: host_generate_uuid(),
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
            "message": "Research Log WASM module initialized",
            "module_id": &self.module_id,
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

    fn handle_query(&self, _payload: Value) -> Value {
        // In a real implementation, this would query the host for Layer 1 data
        json!({
            "status": "error",
            "error": "Query forwarding not implemented in WASM environment"
        })
    }

    fn handle_mutation(&self, _payload: Value) -> Value {
        // In a real implementation, this would send mutations to the host
        json!({
            "status": "error",
            "error": "Mutation forwarding not implemented in WASM environment"
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

        let log_id = host_generate_uuid();
        self.logs.insert(log_id.clone(), Vec::new());
        self.active_log = Some(log_id.clone());

        // In a real implementation, we would emit an event through the host
        let event_data = json!({
            "log_id": &log_id,
            "title": title
        });
        host_emit_event("ResearchLog.Created", &event_data.to_string());

        json!({
            "status": "success",
            "log_id": &log_id,
            "title": title,
            "message": "Research log created",
            "emit_event": {
                "type": "research_log.created",
                "data": {
                    "log_id": &log_id,
                    "title": title
                }
            }
        })
    }

    fn cmd_add_entry(&mut self, args: Value) -> Value {
        let log_id = self.active_log.clone().or_else(|| {
            args.get("log_id").and_then(|v| v.as_str()).map(String::from)
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
            id: host_generate_uuid(),
            timestamp: host_get_timestamp(),
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
        };

        let entry_id = entry.id.clone();
        
        match self.logs.get_mut(&log_id) {
            Some(entries) => {
                entries.push(entry);
                json!({
                    "status": "success",
                    "entry_id": &entry_id,
                    "log_id": &log_id,
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
        let mut total_entries = 0;
        let mut entry_types: HashMap<String, usize> = HashMap::new();
        let mut tags: HashMap<String, usize> = HashMap::new();

        for entries in self.logs.values() {
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
                "total_logs": self.logs.len(),
                "total_entries": total_entries,
                "entry_types": entry_types,
                "top_tags": tags,
                "active_log": &self.active_log
            }
        })
    }

    fn cmd_set_active_log(&mut self, args: Value) -> Value {
        match args.get("log_id").and_then(|v| v.as_str()) {
            Some(id_str) => {
                if self.logs.contains_key(id_str) {
                    self.active_log = Some(id_str.to_string());
                    json!({
                        "status": "success",
                        "active_log": id_str
                    })
                } else {
                    json!({
                        "status": "error",
                        "error": "Log not found"
                    })
                }
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
    unsafe {
        MODULE_STATE = Some(ResearchLogModule::new());
        // Return a dummy pointer since WASM uses global state
        1 as *mut std::ffi::c_void
    }
}

/// Destroy a module instance
#[no_mangle]
pub extern "C" fn destroy_module(_module: *mut std::ffi::c_void) {
    unsafe {
        MODULE_STATE = None;
    }
}

/// Handle a message - main communication interface
#[no_mangle]
pub extern "C" fn handle_message(
    _module: *mut std::ffi::c_void,
    message_type: *const c_char,
    payload: *const c_char,
) -> *mut c_char {
    // Parse message type
    let msg_type = unsafe {
        match std::ffi::CStr::from_ptr(message_type).to_str() {
            Ok(s) => s,
            Err(_) => return string_to_ptr(json!({
                "status": "error",
                "error": "Invalid message type"
            }).to_string()),
        }
    };
    
    // Parse payload
    let payload_str = unsafe {
        match std::ffi::CStr::from_ptr(payload).to_str() {
            Ok(s) => s,
            Err(_) => return string_to_ptr(json!({
                "status": "error",
                "error": "Invalid payload"
            }).to_string()),
        }
    };
    
    let payload_json: Value = match serde_json::from_str(&payload_str) {
        Ok(v) => v,
        Err(e) => {
            return string_to_ptr(json!({
                "status": "error",
                "error": format!("Invalid JSON payload: {}", e)
            }).to_string());
        }
    };
    
    // Handle the message
    let result = with_state_mut(|module| {
        module.handle_message(&msg_type, payload_json)
    });
    
    match result {
        Ok(response) => string_to_ptr(response.to_string()),
        Err(err) => string_to_ptr(json!({
            "status": "error",
            "error": err
        }).to_string())
    }
}

/// Free a string allocated by this module
#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            // Reconstruct the CString and drop it to free memory
            let _ = CString::from_raw(ptr);
        }
    }
}

/// Get module metadata
#[no_mangle]
pub extern "C" fn get_module_metadata() -> *mut c_char {
    let metadata = json!({
        "name": "research-log-wasm",
        "version": "0.2.0",
        "description": "Advanced research logging and analysis module (WASM version)",
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
    
    string_to_ptr(metadata.to_string())
}

/// Legacy execute_command for backward compatibility
#[no_mangle]
pub extern "C" fn execute_command(
    command_ptr: *const u8, 
    command_len: i32,
    args_ptr: *const u8,
    args_len: i32
) -> *mut c_char {
    // Convert command string from memory
    let command = unsafe {
        let slice = std::slice::from_raw_parts(command_ptr, command_len as usize);
        match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return string_to_ptr(json!({
                "error": "Invalid command string"
            }).to_string()),
        }
    };
    
    // Convert args string from memory
    let args_str = unsafe {
        let slice = std::slice::from_raw_parts(args_ptr, args_len as usize);
        match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return string_to_ptr(json!({
                "error": "Invalid arguments string"
            }).to_string()),
        }
    };
    
    // Parse args as JSON
    let args_value: Value = match serde_json::from_str(&args_str) {
        Ok(v) => v,
        Err(e) => {
            return string_to_ptr(json!({
                "error": format!("Invalid arguments: {}", e)
            }).to_string());
        }
    };
    
    // Create command message
    let message = json!({
        "name": command,
        "args": args_value
    });
    
    // Use the unified message handler
    let result = with_state_mut(|module| {
        module.handle_message("command", message)
    });
    
    match result {
        Ok(response) => {
            // For backward compatibility with tests
            if let Some(error) = response.get("error") {
                string_to_ptr(json!({ "error": error }).to_string())
            } else if response.get("status") == Some(&json!("success")) {
                // Transform the response to match expected format
                if command == "create_log" {
                    // Extract just the fields the test expects
                    string_to_ptr(json!({
                        "log_id": response.get("log_id"),
                        "status": "created"
                    }).to_string())
                } else {
                    // For other commands, keep the full response but change status
                    let mut result = response.clone();
                    result["status"] = json!("success");
                    string_to_ptr(result.to_string())
                }
            } else {
                string_to_ptr(response.to_string())
            }
        },
        Err(err) => string_to_ptr(json!({
            "error": err
        }).to_string())
    }
}

/// Initialize function for backward compatibility
#[no_mangle]
pub extern "C" fn initialize(_actor_id_ptr: *const u8, _actor_id_len: i32) -> *mut c_char {
    // Initialize is now handled through handle_message
    let _ = with_state_mut(|module| {
        module.handle_message("init", json!({}))
    });
    
    string_to_ptr(json!({
        "status": "initialized",
        "version": "0.2.0"
    }).to_string())
}

