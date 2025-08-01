//! Test for Research Log module with new message-based FFI architecture

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use libloading::{Library, Symbol};
use serde_json::json;
use uuid::Uuid;

type CreateModuleFn = unsafe extern "C" fn() -> *mut std::ffi::c_void;
type DestroyModuleFn = unsafe extern "C" fn(*mut std::ffi::c_void);
type HandleMessageFn = unsafe extern "C" fn(
    *mut std::ffi::c_void,
    *const c_char,
    *const c_char,
) -> *mut c_char;
type FreeStringFn = unsafe extern "C" fn(*mut c_char);
type GetModuleMetadataFn = unsafe extern "C" fn() -> *mut c_char;

#[test]
fn test_research_log_module_ffi() {
    // Load the module library
    let lib_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/release/libresearch_log_module.so");
    let library = unsafe { Library::new(lib_path).expect("Failed to load research log module library") };
    
    // Get function pointers
    let create_module: Symbol<CreateModuleFn> = unsafe {
        library.get(b"create_module").expect("Failed to find create_module")
    };
    let destroy_module: Symbol<DestroyModuleFn> = unsafe {
        library.get(b"destroy_module").expect("Failed to find destroy_module")
    };
    let handle_message: Symbol<HandleMessageFn> = unsafe {
        library.get(b"handle_message").expect("Failed to find handle_message")
    };
    let free_string: Symbol<FreeStringFn> = unsafe {
        library.get(b"free_string").expect("Failed to find free_string")
    };
    let get_module_metadata: Symbol<GetModuleMetadataFn> = unsafe {
        library.get(b"get_module_metadata").expect("Failed to find get_module_metadata")
    };
    
    // Test metadata
    let metadata_ptr = unsafe { get_module_metadata() };
    assert!(!metadata_ptr.is_null(), "Metadata is null");
    
    let metadata_str = unsafe {
        CStr::from_ptr(metadata_ptr).to_str().expect("Invalid UTF-8 in metadata")
    };
    
    let metadata: serde_json::Value = serde_json::from_str(metadata_str)
        .expect("Failed to parse metadata JSON");
    
    assert_eq!(metadata["name"], "research-log");
    assert_eq!(metadata["version"], "0.2.0");
    assert_eq!(metadata["message_protocol_version"], "1.0");
    
    unsafe { free_string(metadata_ptr) };
    
    // Create module instance
    let module = unsafe { create_module() };
    assert!(!module.is_null(), "Module creation failed");
    
    // Test 1: Initialize the module
    let init_type = CString::new("init").unwrap();
    let init_payload = CString::new("{}").unwrap();
    
    let response = unsafe {
        handle_message(module, init_type.as_ptr(), init_payload.as_ptr())
    };
    
    assert!(!response.is_null(), "Init response is null");
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "success");
    assert!(response_json["module_id"].is_string());
    assert!(response_json["capabilities"].is_array());
    
    unsafe { free_string(response) };
    
    // Test 2: Create a research log
    let cmd_type = CString::new("command").unwrap();
    let create_log_payload = CString::new(json!({
        "name": "create_log",
        "args": {
            "title": "Test Research Log"
        }
    }).to_string()).unwrap();
    
    let response = unsafe {
        handle_message(module, cmd_type.as_ptr(), create_log_payload.as_ptr())
    };
    
    assert!(!response.is_null(), "Create log response is null");
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "success");
    let log_id = response_json["log_id"].as_str().expect("Missing log_id");
    let log_uuid = Uuid::parse_str(log_id).expect("Invalid log UUID");
    
    unsafe { free_string(response) };
    
    // Test 3: Add an entry to the log
    let add_entry_payload = CString::new(json!({
        "name": "add_entry",
        "args": {
            "type": "hypothesis",
            "title": "Initial Hypothesis",
            "content": "The FFI architecture will work correctly",
            "tags": ["test", "ffi", "architecture"],
            "researcher_id": "test-researcher"
        }
    }).to_string()).unwrap();
    
    let response = unsafe {
        handle_message(module, cmd_type.as_ptr(), add_entry_payload.as_ptr())
    };
    
    assert!(!response.is_null(), "Add entry response is null");
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "success");
    assert!(response_json["entry_id"].is_string());
    assert_eq!(response_json["log_id"], log_id);
    
    unsafe { free_string(response) };
    
    // Test 4: Analyze activity
    let analyze_payload = CString::new(json!({
        "name": "analyze_activity",
        "args": {}
    }).to_string()).unwrap();
    
    let response = unsafe {
        handle_message(module, cmd_type.as_ptr(), analyze_payload.as_ptr())
    };
    
    assert!(!response.is_null(), "Analyze response is null");
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "success");
    assert_eq!(response_json["analysis"]["total_logs"], 1);
    assert_eq!(response_json["analysis"]["total_entries"], 1);
    assert_eq!(response_json["analysis"]["entry_types"]["hypothesis"], 1);
    assert_eq!(response_json["analysis"]["active_log"], log_id);
    
    unsafe { free_string(response) };
    
    // Test 5: Handle events (should just acknowledge)
    let event_type = CString::new("event").unwrap();
    let event_payload = CString::new(json!({
        "event": "research_session.started",
        "data": {
            "session_id": "test-session"
        }
    }).to_string()).unwrap();
    
    let response = unsafe {
        handle_message(module, event_type.as_ptr(), event_payload.as_ptr())
    };
    
    assert!(!response.is_null(), "Event response is null");
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "acknowledged");
    
    unsafe { free_string(response) };
    
    // Clean up
    unsafe { destroy_module(module) };
    
    println!("✅ All Research Log FFI tests passed!");
}

#[test]
fn test_research_log_error_handling() {
    let lib_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/release/libresearch_log_module.so");
    let library = unsafe { Library::new(lib_path).expect("Failed to load research log module library") };
    
    let create_module: Symbol<CreateModuleFn> = unsafe {
        library.get(b"create_module").expect("Failed to find create_module")
    };
    let destroy_module: Symbol<DestroyModuleFn> = unsafe {
        library.get(b"destroy_module").expect("Failed to find destroy_module")
    };
    let handle_message: Symbol<HandleMessageFn> = unsafe {
        library.get(b"handle_message").expect("Failed to find handle_message")
    };
    let free_string: Symbol<FreeStringFn> = unsafe {
        library.get(b"free_string").expect("Failed to find free_string")
    };
    
    let module = unsafe { create_module() };
    
    // Test command before initialization
    let cmd_type = CString::new("command").unwrap();
    let cmd_payload = CString::new(json!({
        "name": "create_log",
        "args": { "title": "Test" }
    }).to_string()).unwrap();
    
    let response = unsafe {
        handle_message(module, cmd_type.as_ptr(), cmd_payload.as_ptr())
    };
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "error");
    assert_eq!(response_json["error"], "Module not initialized");
    
    unsafe { free_string(response) };
    
    // Initialize module
    let init_type = CString::new("init").unwrap();
    let init_payload = CString::new("{}").unwrap();
    let response = unsafe {
        handle_message(module, init_type.as_ptr(), init_payload.as_ptr())
    };
    unsafe { free_string(response) };
    
    // Test invalid command
    let invalid_cmd_payload = CString::new(json!({
        "name": "invalid_command",
        "args": {}
    }).to_string()).unwrap();
    
    let response = unsafe {
        handle_message(module, cmd_type.as_ptr(), invalid_cmd_payload.as_ptr())
    };
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "error");
    assert!(response_json["error"].as_str().unwrap().contains("Unknown command"));
    
    unsafe { free_string(response) };
    unsafe { destroy_module(module) };
    
    println!("✅ Research Log error handling tests passed!");
}