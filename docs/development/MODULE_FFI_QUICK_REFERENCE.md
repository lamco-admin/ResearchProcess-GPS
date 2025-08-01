# Module FFI Quick Reference

## Required FFI Exports

Every native module must export these C functions:

```c
// Create module instance
extern "C" fn create_module() -> *mut c_void;

// Destroy module instance  
extern "C" fn destroy_module(module: *mut c_void);

// Handle messages
extern "C" fn handle_message(
    module: *mut c_void,
    message_type: *const c_char,
    payload: *const c_char
) -> *mut c_char;

// Free allocated strings
extern "C" fn free_string(ptr: *mut c_char);

// Get module metadata (optional)
extern "C" fn get_module_metadata() -> *mut c_char;
```

## Using the SDK Macro

The easiest way to generate these exports:

```rust
use rp_module_sdk::prelude::*;

struct MyModule { /* ... */ }

impl FfiModule for MyModule { /* ... */ }

// This generates all required exports
rp_module_sdk::ffi_module!(MyModule, MODULE_HOLDER);
```

## Message Types & Expected Responses

### Init Message
```json
// Input
{
    "instance_id": "uuid",
    "workspace_id": "uuid",
    "actor_id": "uuid"
}

// Response
{
    "status": "success",
    "message": "Initialized"
}
```

### Command Message
```json
// Input
{
    "name": "command_name",
    "args": { /* command arguments */ }
}

// Response
{
    "status": "success|created|error",
    /* command-specific fields */
}
```

### Query Message
```json
// Input
{
    "entity": "entity_type",
    "filters": { /* filter conditions */ },
    "include": ["relation1", "relation2"],
    "limit": 10,
    "offset": 0
}

// Response
{
    "status": "success",
    "entities": [ /* results */ ],
    "count": 10
}
```

### Mutation Message
```json
// Input
{
    "entity": "entity_type",
    "operation": "create|update|delete",
    "data": { /* entity data */ }
}

// Response
{
    "status": "created|success|error",
    "id": "uuid",
    "message": "Entity created"
}
```

### Event Message
```json
// Input
{
    "type": "entity.created",
    "entity_id": "uuid",
    "entity_type": "research_log",
    "data": { /* event data */ }
}

// Response
{
    "status": "success",
    "message": "Event handled"
}
```

### Shutdown Message
```json
// Input
{}

// Response
{
    "status": "success",
    "message": "Shutdown complete"
}
```

## SDK Utilities

### Response Builders
```rust
ResponseBuilder::success().with("key", value).build()
ResponseBuilder::created().with("id", uuid).build()
ResponseBuilder::error("Error message").build()
```

### Query Builder
```rust
QueryBuilder::new()
    .entity("research_log")
    .filter("status", "active")
    .include("entries")
    .limit(10)
    .build()
```

### Mutation Builder
```rust
MutationBuilder::create("fact")
    .set("claim", "...")
    .set("confidence", 0.95)
    .build()

MutationBuilder::update("fact", id)
    .set("confidence", 0.98)
    .build()

MutationBuilder::delete("fact", id).build()
```

## Testing FFI Functions

```rust
#[test]
fn test_ffi_exports() {
    // Create module
    let module_ptr = create_module();
    assert!(!module_ptr.is_null());
    
    // Send init message
    let msg_type = CString::new("init").unwrap();
    let payload = CString::new(r#"{"instance_id":"..."}"#).unwrap();
    
    let response = handle_message(
        module_ptr,
        msg_type.as_ptr(),
        payload.as_ptr()
    );
    
    // Check response
    let response_str = unsafe {
        CStr::from_ptr(response).to_str().unwrap()
    };
    
    // Free response
    free_string(response);
    
    // Cleanup
    destroy_module(module_ptr);
}
```

## Common Patterns

### Safe String Conversion
```rust
// C string to Rust
unsafe fn c_str_to_string(ptr: *const c_char) -> Result<String, String> {
    if ptr.is_null() {
        return Err("Null pointer".to_string());
    }
    CStr::from_ptr(ptr)
        .to_str()
        .map(|s| s.to_string())
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}

// Rust string to C
fn string_to_c_str(s: String) -> *mut c_char {
    match CString::new(s) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
```

### Error Handling Pattern
```rust
fn handle_command(&mut self, name: &str, args: Value) -> Value {
    // Validate inputs
    let param = match args.get("param").and_then(|v| v.as_str()) {
        Some(p) => p,
        None => return ResponseBuilder::error("Missing param").build(),
    };
    
    // Process command
    match self.process(param) {
        Ok(result) => ResponseBuilder::success()
            .with("result", result)
            .build(),
        Err(e) => ResponseBuilder::error(e.to_string()).build(),
    }
}
```