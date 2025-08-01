//! WASM module support (for sandboxed execution)

use crate::prelude::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

pub mod macros;

/// Static module instance for WASM (single-threaded)
static mut MODULE_INSTANCE: Option<Box<dyn Module>> = None;

/// Convert a Rust string to a C string pointer
pub fn string_to_ptr(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Convert a C string pointer and length to a Rust string
pub fn ptr_to_string(ptr: *const c_char, len: c_int) -> Result<String> {
    unsafe {
        let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
        std::str::from_utf8(slice)
            .map(|s| s.to_string())
            .map_err(|e| ModuleError::Other(format!("Invalid UTF-8: {}", e)))
    }
}

/// Free a string allocated by this module
#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// Get module info as JSON string
#[no_mangle]
pub extern "C" fn get_module_info() -> *mut c_char {
    unsafe {
        match &MODULE_INSTANCE {
            Some(module) => {
                let info = module.info();
                match serde_json::to_string(&info) {
                    Ok(json) => string_to_ptr(json),
                    Err(_) => std::ptr::null_mut(),
                }
            }
            None => std::ptr::null_mut(),
        }
    }
}

/// Initialize the module
#[no_mangle]
pub extern "C" fn initialize(actor_id_ptr: *const c_char, actor_id_len: c_int) -> *mut c_char {
    let actor_id = match ptr_to_string(actor_id_ptr, actor_id_len) {
        Ok(s) => match Uuid::parse_str(&s) {
            Ok(uuid) => uuid,
            Err(e) => return string_to_ptr(json!({
                "error": format!("Invalid actor ID: {}", e)
            }).to_string()),
        },
        Err(e) => return string_to_ptr(json!({
            "error": format!("Invalid actor ID string: {}", e)
        }).to_string()),
    };
    
    let context = ModuleContext {
        instance_id: Uuid::new_v4(),
        actor_id,
        config: json!({}),
        resource_limits: ResourceLimits::default(),
        capabilities: vec![],
    };
    
    unsafe {
        if let Some(module) = &mut MODULE_INSTANCE {
            match module.initialize(context) {
                Ok(()) => string_to_ptr(json!({
                    "status": "initialized",
                    "version": SDK_VERSION
                }).to_string()),
                Err(e) => string_to_ptr(json!({
                    "error": format!("Initialization failed: {}", e)
                }).to_string()),
            }
        } else {
            string_to_ptr(json!({
                "error": "Module not created"
            }).to_string())
        }
    }
}

/// Execute a command
#[no_mangle]
pub extern "C" fn execute_command(
    command_ptr: *const c_char, 
    command_len: c_int,
    args_ptr: *const c_char,
    args_len: c_int
) -> *mut c_char {
    let command = match ptr_to_string(command_ptr, command_len) {
        Ok(s) => s,
        Err(e) => return string_to_ptr(json!({
            "error": format!("Invalid command string: {}", e)
        }).to_string()),
    };
    
    let args_str = match ptr_to_string(args_ptr, args_len) {
        Ok(s) => s,
        Err(e) => return string_to_ptr(json!({
            "error": format!("Invalid arguments string: {}", e)
        }).to_string()),
    };
    
    let args: Value = match serde_json::from_str(&args_str) {
        Ok(v) => v,
        Err(e) => return string_to_ptr(json!({
            "error": format!("Invalid arguments JSON: {}", e)
        }).to_string()),
    };
    
    unsafe {
        if let Some(module) = &mut MODULE_INSTANCE {
            match module.execute_command(&command, args) {
                Ok(result) => string_to_ptr(result.to_string()),
                Err(e) => string_to_ptr(json!({
                    "error": format!("Command failed: {}", e)
                }).to_string()),
            }
        } else {
            string_to_ptr(json!({
                "error": "Module not initialized"
            }).to_string())
        }
    }
}

/// Handle an event
#[no_mangle]
pub extern "C" fn handle_event(
    event_type_ptr: *const c_char,
    event_type_len: c_int,
    event_data_ptr: *const c_char,
    event_data_len: c_int
) -> *mut c_char {
    let event_type = match ptr_to_string(event_type_ptr, event_type_len) {
        Ok(s) => s,
        Err(e) => return string_to_ptr(json!({
            "error": format!("Invalid event type: {}", e)
        }).to_string()),
    };
    
    let event_data_str = match ptr_to_string(event_data_ptr, event_data_len) {
        Ok(s) => s,
        Err(e) => return string_to_ptr(json!({
            "error": format!("Invalid event data string: {}", e)
        }).to_string()),
    };
    
    let event_data: Value = match serde_json::from_str(&event_data_str) {
        Ok(v) => v,
        Err(e) => return string_to_ptr(json!({
            "error": format!("Invalid event data JSON: {}", e)
        }).to_string()),
    };
    
    unsafe {
        if let Some(module) = &mut MODULE_INSTANCE {
            match module.handle_event(&event_type, event_data) {
                Ok(()) => string_to_ptr(json!({
                    "status": "handled",
                    "event_type": event_type
                }).to_string()),
                Err(e) => string_to_ptr(json!({
                    "error": format!("Event handling failed: {}", e)
                }).to_string()),
            }
        } else {
            string_to_ptr(json!({
                "error": "Module not initialized"
            }).to_string())
        }
    }
}

/// Get module state
#[no_mangle]
pub extern "C" fn get_module_state() -> *mut c_char {
    unsafe {
        match &MODULE_INSTANCE {
            Some(module) => {
                let state = module.get_state();
                match serde_json::to_string(&state) {
                    Ok(json) => string_to_ptr(json),
                    Err(_) => std::ptr::null_mut(),
                }
            }
            None => std::ptr::null_mut(),
        }
    }
}

/// Shutdown the module
#[no_mangle]
pub extern "C" fn shutdown() -> *mut c_char {
    unsafe {
        if let Some(mut module) = MODULE_INSTANCE.take() {
            match module.shutdown() {
                Ok(()) => string_to_ptr(json!({
                    "status": "shutdown"
                }).to_string()),
                Err(e) => string_to_ptr(json!({
                    "error": format!("Shutdown failed: {}", e)
                }).to_string()),
            }
        } else {
            string_to_ptr(json!({
                "error": "Module not initialized"
            }).to_string())
        }
    }
}

/// Set the module instance (called by macros)
pub unsafe fn set_module_instance<M: Module + 'static>(module: M) {
    MODULE_INSTANCE = Some(Box::new(module));
}

/// Log a message (simplified for WASM)
pub fn log_message(level: &str, msg: &str) {
    println!("[{}] {}", level, msg);
}

/// Helper to emit events (simplified for WASM)
pub fn emit_event(event_type: &str, data: &Value) {
    log_message("event", &format!("{}: {}", event_type, data));
}