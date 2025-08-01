# Module Development Guide

## Overview

ResearchProcess-GPS supports extensibility through a message-based FFI (Foreign Function Interface) module system. This guide explains how to develop modules using the new architecture.

## Architecture Overview

The module system uses a **unified message-based protocol** for both native (Rust) and WASM modules. Instead of trait objects or complex interfaces, all communication happens through JSON messages passed via FFI functions.

### Key Principles

1. **Message-Based Communication**: All interactions use JSON messages
2. **Type Safety**: The SDK provides builders and parsers for messages  
3. **Memory Safety**: No direct memory sharing between host and modules
4. **Language Agnostic**: The protocol can be implemented in any language

## Getting Started

### 1. Create Your Module Project

```toml
# Cargo.toml
[package]
name = "my-module"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Required for dynamic loading

[dependencies]
rp-module-sdk = { path = "../../crates/rp-module-sdk" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
```

### 2. Implement the FfiModule Trait

```rust
use rp_module_sdk::prelude::*;
use serde_json::Value;

pub struct MyModule {
    initialized: bool,
    // Your module state
}

impl MyModule {
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
}

impl FfiModule for MyModule {
    fn init(&mut self, payload: Value) -> Value {
        // Initialize your module
        self.initialized = true;
        ResponseBuilder::success()
            .with("message", "Module initialized")
            .build()
    }
    
    fn handle_command(&mut self, name: &str, args: Value) -> Value {
        // Handle commands
        match name {
            "my_command" => {
                // Your command logic
                ResponseBuilder::success()
                    .with("result", "Command executed")
                    .build()
            },
            _ => ResponseBuilder::error("Unknown command").build(),
        }
    }
    
    // Implement other required methods...
}

// Generate FFI exports
rp_module_sdk::ffi_module!(MyModule, MODULE_HOLDER);
```

## Message Protocol

### Message Types

The protocol supports six message types:

1. **init** - Initialize the module with context
2. **command** - Execute a module-specific command
3. **query** - Query Layer 1 data
4. **mutation** - Mutate Layer 1 data
5. **event** - Handle system events
6. **shutdown** - Clean shutdown

### Message Format

All messages follow this structure:

```json
{
    "type": "command",
    "name": "create_log",
    "args": {
        "title": "My Research Log"
    }
}
```

### Response Format

Responses use a standard format:

```json
{
    "status": "success|created|error",
    "field1": "value1",
    "field2": "value2"
}
```

## Using the SDK

### Response Builders

The SDK provides fluent builders for responses:

```rust
// Success response
ResponseBuilder::success()
    .with("count", 42)
    .with("message", "Items processed")
    .build()

// Created response (for resource creation)
ResponseBuilder::created()
    .with("id", uuid)
    .with("title", title)
    .build()

// Error response
ResponseBuilder::error("Invalid parameters").build()
```

### Query Builder

Build queries for Layer 1 data:

```rust
let query = QueryBuilder::new()
    .entity("research_log")
    .filter("status", "active")
    .include("entries")
    .limit(10)
    .build();
```

### Mutation Builder

Create mutations for Layer 1 data:

```rust
// Create a new entity
let mutation = MutationBuilder::create("fact")
    .set("claim", "The sky is blue")
    .set("confidence", 0.95)
    .build();

// Update an existing entity
let mutation = MutationBuilder::update("fact", fact_id)
    .set("confidence", 0.98)
    .build();
```

## Accessing Layer 1 Data

Modules can access ResearchProcess-GPS entities through queries and mutations:

### Querying Data

```rust
fn handle_query(&self, payload: Value) -> Value {
    // The host will execute this query and return results
    ResponseBuilder::success()
        .with("query_forwarded", true)
        .build()
}
```

### Creating Entities

```rust
fn handle_mutation(&mut self, payload: Value) -> Value {
    // The host will execute this mutation
    ResponseBuilder::success()
        .with("mutation_forwarded", true)
        .build()
}
```

## Module Manifest

Create a `module.toml` file to describe your module:

```toml
[module]
name = "my-module"
version = "0.1.0"
description = "My custom research module"
author = "Your Name"
license = "MIT"
module_type = "native"  # or "wasm"

[capabilities]
can_query = true
can_mutate = true
can_emit_events = true
can_handle_events = true

[resources]
max_memory_mb = 128
max_cpu_percent = 50
```

## Building Your Module

### Native Modules

```bash
cargo build --release
```

The output will be in `target/release/lib<module_name>.so` (Linux) or `.dylib` (macOS).

### WASM Modules

```bash
cargo build --target wasm32-wasip1 --release
```

## Testing Your Module

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initialization() {
        let mut module = MyModule::new();
        let response = module.init(json!({
            "instance_id": Uuid::new_v4()
        }));
        assert_eq!(response["status"], "success");
    }
}
```

### Integration Testing

Place your module in the `modules/` directory and use the ModuleLoader:

```rust
let loader = ModuleLoader::new()?;
let module_id = loader.load_module("modules/my-module", context).await?;
loader.initialize_module(&module_id).await?;

let result = loader.execute_command(
    &module_id, 
    "my_command",
    json!({ "param": "value" })
).await?;
```

## Best Practices

1. **Error Handling**: Always return proper error responses instead of panicking
2. **Validation**: Validate all input parameters before processing
3. **Resource Management**: Clean up resources in the shutdown handler
4. **Logging**: Use structured logging for debugging
5. **Documentation**: Document your commands and expected parameters

## Common Patterns

### State Management

```rust
pub struct MyModule {
    state: HashMap<String, Value>,
    config: ModuleConfig,
}
```

### Command Routing

```rust
fn handle_command(&mut self, name: &str, args: Value) -> Value {
    match name {
        "cmd1" => self.cmd_handler_1(args),
        "cmd2" => self.cmd_handler_2(args),
        _ => ResponseBuilder::error("Unknown command").build(),
    }
}
```

### Event Handling

```rust
fn handle_event(&mut self, event_type: &str, payload: Value) -> Value {
    match event_type {
        "entity.created" => {
            // React to entity creation
        },
        _ => {
            // Ignore unknown events
        }
    }
    ResponseBuilder::success().build()
}
```

## Troubleshooting

### Module Not Loading

1. Check that the library exports the required FFI functions
2. Verify the module.toml is in the correct location
3. Ensure the module type matches the binary format

### Symbol Not Found

The loader tries both with and without underscore prefixes:
- Linux typically uses `create_module`
- macOS typically uses `_create_module`

### Memory Issues

- Always free strings returned by the module using `free_string`
- Don't hold pointers across FFI boundaries
- Use the SDK's string conversion utilities

## Example Module

See `modules/example-module/` for a complete working example that demonstrates:
- Command handling
- State management
- Query and mutation building
- Error handling
- Testing

## Further Resources

- [Module SDK API Documentation](../api/module-sdk.md)
- [FFI Safety Guidelines](./FFI_SAFETY.md)
- [Performance Optimization](./MODULE_PERFORMANCE.md)