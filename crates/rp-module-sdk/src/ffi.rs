//! FFI utilities for native module development
//! 
//! This module provides helpers for implementing the required FFI exports
//! and handling the message-based protocol.

use std::ffi::{CStr, CString, c_char};
use std::sync::Mutex;
use serde_json::Value;
use crate::messages::{MessageType, ResponseBuilder};

/// Trait for modules implementing the FFI protocol
pub trait FfiModule: Send + Sync {
    /// Handle initialization
    fn init(&mut self, payload: Value) -> Value;
    
    /// Handle commands
    fn handle_command(&mut self, name: &str, args: Value) -> Value;
    
    /// Handle queries
    fn handle_query(&self, payload: Value) -> Value;
    
    /// Handle mutations
    fn handle_mutation(&mut self, payload: Value) -> Value;
    
    /// Handle events
    fn handle_event(&mut self, event_type: &str, payload: Value) -> Value;
    
    /// Handle shutdown
    fn shutdown(&mut self) -> Value;
}

/// Global module instance holder
pub struct ModuleHolder<M: FfiModule> {
    instance: Mutex<Option<M>>,
}

impl<M: FfiModule> ModuleHolder<M> {
    /// Create a new holder
    pub const fn new() -> Self {
        Self {
            instance: Mutex::new(None),
        }
    }
    
    /// Set the module instance
    pub fn set(&self, module: M) {
        *self.instance.lock().unwrap() = Some(module);
    }
    
    /// Handle a message
    pub fn handle_message(&self, message_type: &str, payload: Value) -> Value {
        let mut guard = self.instance.lock().unwrap();
        let module = match guard.as_mut() {
            Some(m) => m,
            None => return ResponseBuilder::error("Module not initialized").build(),
        };
        
        match MessageType::from_str(message_type) {
            Some(MessageType::Init) => module.init(payload),
            Some(MessageType::Command) => {
                match crate::messages::parse_command(&payload) {
                    Ok((name, args)) => module.handle_command(name, args.clone()),
                    Err(e) => ResponseBuilder::error(e).build(),
                }
            },
            Some(MessageType::Query) => module.handle_query(payload),
            Some(MessageType::Mutation) => module.handle_mutation(payload),
            Some(MessageType::Event) => {
                let event_type = payload.get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                module.handle_event(event_type, payload.clone())
            },
            Some(MessageType::Shutdown) => module.shutdown(),
            None => ResponseBuilder::error(format!("Unknown message type: {}", message_type)).build(),
        }
    }
    
    /// Clear the module instance
    pub fn clear(&self) {
        *self.instance.lock().unwrap() = None;
    }
}

/// Convert a C string to Rust string
pub unsafe fn c_str_to_string(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Err("Null pointer".to_string());
    }
    
    CStr::from_ptr(ptr)
        .to_str()
        .map(|s| s.to_string())
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}

/// Convert a Rust string to C string
pub fn string_to_c_str(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Macro to generate FFI exports for a module
#[macro_export]
macro_rules! ffi_module {
    ($module_type:ty, $holder:ident) => {
        static $holder: $crate::ffi::ModuleHolder<$module_type> = 
            $crate::ffi::ModuleHolder::new();
        
        #[no_mangle]
        pub extern "C" fn create_module() -> *mut std::ffi::c_void {
            let module = <$module_type>::new();
            $holder.set(module);
            1 as *mut std::ffi::c_void // Non-null pointer
        }
        
        #[no_mangle]
        pub extern "C" fn destroy_module(_module: *mut std::ffi::c_void) {
            $holder.clear();
        }
        
        #[no_mangle]
        pub extern "C" fn handle_message(
            _module: *mut std::ffi::c_void,
            message_type: *const std::ffi::c_char,
            payload: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_char {
            unsafe {
                let message_type_str = match $crate::ffi::c_str_to_string(message_type) {
                    Ok(s) => s,
                    Err(e) => return $crate::ffi::string_to_c_str(
                        $crate::messages::ResponseBuilder::error(e).build_string()
                    ),
                };
                
                let payload_str = match $crate::ffi::c_str_to_string(payload) {
                    Ok(s) => s,
                    Err(e) => return $crate::ffi::string_to_c_str(
                        $crate::messages::ResponseBuilder::error(e).build_string()
                    ),
                };
                
                let payload_value: serde_json::Value = match serde_json::from_str(&payload_str) {
                    Ok(v) => v,
                    Err(e) => return $crate::ffi::string_to_c_str(
                        $crate::messages::ResponseBuilder::error(
                            format!("Invalid JSON: {}", e)
                        ).build_string()
                    ),
                };
                
                let response = $holder.handle_message(&message_type_str, payload_value);
                $crate::ffi::string_to_c_str(response.to_string())
            }
        }
        
        #[no_mangle]
        pub extern "C" fn free_string(ptr: *mut std::ffi::c_char) {
            if !ptr.is_null() {
                unsafe {
                    let _ = std::ffi::CString::from_raw(ptr);
                }
            }
        }
        
        #[no_mangle]
        pub extern "C" fn get_module_metadata() -> *mut std::ffi::c_char {
            let metadata = serde_json::json!({
                "name": stringify!($module_type),
                "version": env!("CARGO_PKG_VERSION"),
                "sdk_version": $crate::SDK_VERSION,
            });
            $crate::ffi::string_to_c_str(metadata.to_string())
        }
    };
}