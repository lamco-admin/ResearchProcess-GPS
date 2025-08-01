//! Test for minimal FFI module with new message-based architecture

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use libloading::{Library, Symbol};
use serde_json::json;

type CreateModuleFn = unsafe extern "C" fn() -> *mut std::ffi::c_void;
type DestroyModuleFn = unsafe extern "C" fn(*mut std::ffi::c_void);
type HandleMessageFn = unsafe extern "C" fn(
    *mut std::ffi::c_void,
    *const c_char,
    *const c_char,
) -> *mut c_char;
type FreeStringFn = unsafe extern "C" fn(*mut c_char);

#[test]
fn test_minimal_module_ffi() {
    // Load the module library
    let lib_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../modules/test-minimal/target/release/libtest_minimal.so");
    let library = unsafe { Library::new(lib_path).expect("Failed to load module library") };
    
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
    assert_eq!(response_json["message"], "Module initialized");
    
    unsafe { free_string(response) };
    
    // Test 2: Send a ping command
    let cmd_type = CString::new("command").unwrap();
    let ping_payload = CString::new(json!({
        "name": "ping"
    }).to_string()).unwrap();
    
    let response = unsafe {
        handle_message(module, cmd_type.as_ptr(), ping_payload.as_ptr())
    };
    
    assert!(!response.is_null(), "Ping response is null");
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "success");
    assert_eq!(response_json["response"], "pong");
    assert_eq!(response_json["message_count"], 2); // init + ping
    
    unsafe { free_string(response) };
    
    // Test 3: Get module status
    let status_payload = CString::new(json!({
        "name": "status"
    }).to_string()).unwrap();
    
    let response = unsafe {
        handle_message(module, cmd_type.as_ptr(), status_payload.as_ptr())
    };
    
    assert!(!response.is_null(), "Status response is null");
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["initialized"], true);
    assert_eq!(response_json["message_count"], 3); // init + ping + status
    
    unsafe { free_string(response) };
    
    // Test 4: Invalid message type
    let invalid_type = CString::new("invalid").unwrap();
    let empty_payload = CString::new("{}").unwrap();
    
    let response = unsafe {
        handle_message(module, invalid_type.as_ptr(), empty_payload.as_ptr())
    };
    
    assert!(!response.is_null(), "Invalid type response is null");
    
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().expect("Invalid UTF-8 in response")
    };
    
    let response_json: serde_json::Value = serde_json::from_str(response_str)
        .expect("Failed to parse response JSON");
    
    assert_eq!(response_json["status"], "error");
    assert!(response_json["error"].as_str().unwrap().contains("Unknown message type"));
    
    unsafe { free_string(response) };
    
    // Clean up
    unsafe { destroy_module(module) };
    
    println!("✅ All minimal FFI tests passed!");
}

#[test]
fn test_null_safety() {
    // This test verifies the module handles null pointers safely
    let lib_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../modules/test-minimal/target/release/libtest_minimal.so");
    let library = unsafe { Library::new(lib_path).expect("Failed to load module library") };
    
    let handle_message: Symbol<HandleMessageFn> = unsafe {
        library.get(b"handle_message").expect("Failed to find handle_message")
    };
    let destroy_module: Symbol<DestroyModuleFn> = unsafe {
        library.get(b"destroy_module").expect("Failed to find destroy_module")
    };
    
    // Test null module
    let response = unsafe {
        handle_message(std::ptr::null_mut(), std::ptr::null(), std::ptr::null())
    };
    assert!(response.is_null(), "Should return null for null module");
    
    // Test destroy with null (should not crash)
    unsafe { destroy_module(std::ptr::null_mut()) };
    
    println!("✅ Null safety tests passed!");
}