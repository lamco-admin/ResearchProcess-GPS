//! Minimal FFI test module - validates message-based architecture

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Internal module state
struct MinimalModule {
    initialized: bool,
    message_count: u32,
}

impl MinimalModule {
    fn new() -> Self {
        Self {
            initialized: false,
            message_count: 0,
        }
    }

    fn handle_message(&mut self, msg_type: &str, payload: Value) -> Value {
        self.message_count += 1;
        
        match msg_type {
            "init" => {
                self.initialized = true;
                json!({
                    "status": "success",
                    "message": "Module initialized"
                })
            }
            "command" => {
                let cmd = payload.get("name").and_then(|v| v.as_str()).unwrap_or("unknown");
                match cmd {
                    "ping" => json!({
                        "status": "success",
                        "response": "pong",
                        "message_count": self.message_count
                    }),
                    "status" => json!({
                        "initialized": self.initialized,
                        "message_count": self.message_count
                    }),
                    _ => json!({
                        "status": "error",
                        "error": format!("Unknown command: {}", cmd)
                    })
                }
            }
            "query" => json!({
                "status": "error",
                "error": "Queries not supported in minimal module"
            }),
            "mutation" => json!({
                "status": "error", 
                "error": "Mutations not supported in minimal module"
            }),
            _ => json!({
                "status": "error",
                "error": format!("Unknown message type: {}", msg_type)
            })
        }
    }
}

/// Create a new module instance
#[no_mangle]
pub extern "C" fn create_module() -> *mut std::ffi::c_void {
    let module = Box::new(MinimalModule::new());
    Box::into_raw(module) as *mut std::ffi::c_void
}

/// Destroy a module instance
#[no_mangle]
pub extern "C" fn destroy_module(module: *mut std::ffi::c_void) {
    if module.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(module as *mut MinimalModule);
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
        let module = &mut *(module as *mut MinimalModule);
        
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

/// Get the module's function table (alternative approach)
#[repr(C)]
pub struct ModuleFunctions {
    pub create: extern "C" fn() -> *mut std::ffi::c_void,
    pub destroy: extern "C" fn(*mut std::ffi::c_void),
    pub handle_message: extern "C" fn(*mut std::ffi::c_void, *const c_char, *const c_char) -> *mut c_char,
    pub free_string: extern "C" fn(*mut c_char),
}

#[no_mangle]
pub extern "C" fn get_module_functions() -> ModuleFunctions {
    ModuleFunctions {
        create: create_module,
        destroy: destroy_module,
        handle_message,
        free_string,
    }
}