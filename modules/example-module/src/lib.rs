//! Example module demonstrating the new message-based FFI architecture
//! 
//! This module shows how to:
//! - Use the SDK's message builders
//! - Implement the FfiModule trait
//! - Handle different message types
//! - Query and mutate Layer 1 data

use rp_module_sdk::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// Example module that manages a simple counter and demonstrates Layer 1 operations
pub struct ExampleModule {
    /// Module initialized flag
    initialized: bool,
    /// Instance ID from initialization
    instance_id: Option<Uuid>,
    /// Simple counter for demonstration
    counter: i32,
    /// Cache of entities we've created
    created_entities: HashMap<String, Uuid>,
}

impl ExampleModule {
    /// Create a new module instance
    pub fn new() -> Self {
        Self {
            initialized: false,
            instance_id: None,
            counter: 0,
            created_entities: HashMap::new(),
        }
    }
}

impl FfiModule for ExampleModule {
    fn init(&mut self, payload: Value) -> Value {
        // Parse initialization data
        let instance_id = match payload.get("instance_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok()) {
            Some(id) => id,
            None => return ResponseBuilder::error("Missing or invalid instance_id").build(),
        };
        
        self.instance_id = Some(instance_id);
        self.initialized = true;
        
        ResponseBuilder::success()
            .with("message", "Example module initialized")
            .with("instance_id", instance_id)
            .build()
    }
    
    fn handle_command(&mut self, name: &str, args: Value) -> Value {
        if !self.initialized {
            return ResponseBuilder::error("Module not initialized").build();
        }
        
        match name {
            "increment" => {
                self.counter += 1;
                ResponseBuilder::success()
                    .with("counter", self.counter)
                    .with("message", "Counter incremented")
                    .build()
            },
            
            "decrement" => {
                self.counter -= 1;
                ResponseBuilder::success()
                    .with("counter", self.counter)
                    .with("message", "Counter decremented")
                    .build()
            },
            
            "get_counter" => {
                ResponseBuilder::success()
                    .with("counter", self.counter)
                    .build()
            },
            
            "create_note" => {
                let title = match args.get("title").and_then(|v| v.as_str()) {
                    Some(t) => t,
                    None => return ResponseBuilder::error("Missing title parameter").build(),
                };
                
                let content = args.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                // In a real module, this would send a mutation to the host
                // For demo purposes, we just simulate it
                let note_id = Uuid::new_v4();
                self.created_entities.insert(title.to_string(), note_id);
                
                ResponseBuilder::created()
                    .with("note_id", note_id)
                    .with("title", title)
                    .with("message", "Note created (simulated)")
                    .build()
            },
            
            _ => ResponseBuilder::error(format!("Unknown command: {}", name)).build(),
        }
    }
    
    fn handle_query(&self, payload: Value) -> Value {
        if !self.initialized {
            return ResponseBuilder::error("Module not initialized").build();
        }
        
        // Example query handling
        if let Some(entity_type) = payload.get("entity").and_then(|v| v.as_str()) {
            match entity_type {
                "note" => {
                    // Return our simulated notes
                    let notes: Vec<Value> = self.created_entities.iter()
                        .map(|(title, id)| json!({
                            "id": id,
                            "title": title,
                            "type": "note"
                        }))
                        .collect();
                    
                    ResponseBuilder::success()
                        .with("entities", notes)
                        .with("count", self.created_entities.len())
                        .build()
                },
                _ => ResponseBuilder::error(format!("Unknown entity type: {}", entity_type)).build(),
            }
        } else {
            ResponseBuilder::error("Missing entity type in query").build()
        }
    }
    
    fn handle_mutation(&mut self, payload: Value) -> Value {
        if !self.initialized {
            return ResponseBuilder::error("Module not initialized").build();
        }
        
        // Example mutation handling
        let operation = payload.get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match operation {
            "create" => {
                // Forward to host in real implementation
                ResponseBuilder::success()
                    .with("message", "Mutation would be forwarded to host")
                    .with("operation", operation)
                    .build()
            },
            _ => ResponseBuilder::error(format!("Unsupported operation: {}", operation)).build(),
        }
    }
    
    fn handle_event(&mut self, event_type: &str, payload: Value) -> Value {
        if !self.initialized {
            return ResponseBuilder::error("Module not initialized").build();
        }
        
        // Log the event
        match event_type {
            "research_log.created" => {
                if let Some(log_id) = payload.get("entity_id").and_then(|v| v.as_str()) {
                    ResponseBuilder::success()
                        .with("message", format!("Acknowledged research log creation: {}", log_id))
                        .build()
                } else {
                    ResponseBuilder::error("Missing entity_id in event").build()
                }
            },
            _ => {
                ResponseBuilder::success()
                    .with("message", format!("Received event: {}", event_type))
                    .build()
            }
        }
    }
    
    fn shutdown(&mut self) -> Value {
        self.initialized = false;
        
        ResponseBuilder::success()
            .with("message", "Example module shut down")
            .with("final_counter", self.counter)
            .with("entities_created", self.created_entities.len())
            .build()
    }
}

// Generate the FFI exports using the SDK macro
rp_module_sdk::ffi_module!(ExampleModule, MODULE_HOLDER);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_initialization() {
        let mut module = ExampleModule::new();
        assert!(!module.initialized);
        
        let init_payload = json!({
            "instance_id": Uuid::new_v4().to_string(),
            "workspace_id": Uuid::new_v4().to_string(),
        });
        
        let response = module.init(init_payload);
        assert_eq!(response["status"], "success");
        assert!(module.initialized);
    }
    
    #[test]
    fn test_counter_commands() {
        let mut module = ExampleModule::new();
        
        // Initialize first
        let init_payload = json!({
            "instance_id": Uuid::new_v4().to_string(),
        });
        module.init(init_payload);
        
        // Test increment
        let response = module.handle_command("increment", json!({}));
        assert_eq!(response["status"], "success");
        assert_eq!(response["counter"], 1);
        
        // Test get_counter
        let response = module.handle_command("get_counter", json!({}));
        assert_eq!(response["counter"], 1);
        
        // Test decrement
        let response = module.handle_command("decrement", json!({}));
        assert_eq!(response["counter"], 0);
    }
}