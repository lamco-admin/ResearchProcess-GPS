//! Message builders for the FFI-based module protocol
//! 
//! This module provides utilities for building and parsing messages
//! in the unified JSON protocol used by both native and WASM modules.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

/// Standard message types supported by the module protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    /// Initialize the module
    Init,
    /// Execute a command
    Command,
    /// Query Layer 1 data
    Query,
    /// Mutate Layer 1 data
    Mutation,
    /// Handle an event
    Event,
    /// Shutdown the module
    Shutdown,
}

impl MessageType {
    /// Get the string representation for FFI
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Init => "init",
            MessageType::Command => "command",
            MessageType::Query => "query",
            MessageType::Mutation => "mutation",
            MessageType::Event => "event",
            MessageType::Shutdown => "shutdown",
        }
    }
    
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "init" => Some(MessageType::Init),
            "command" => Some(MessageType::Command),
            "query" => Some(MessageType::Query),
            "mutation" => Some(MessageType::Mutation),
            "event" => Some(MessageType::Event),
            "shutdown" => Some(MessageType::Shutdown),
            _ => None,
        }
    }
}

/// Standard response status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseStatus {
    /// Operation succeeded
    Success,
    /// Resource was created
    Created,
    /// Operation failed
    Error,
}

/// Builder for response messages
#[derive(Debug, Clone)]
pub struct ResponseBuilder {
    status: ResponseStatus,
    data: Value,
}

impl ResponseBuilder {
    /// Create a success response
    pub fn success() -> Self {
        Self {
            status: ResponseStatus::Success,
            data: json!({}),
        }
    }
    
    /// Create a created response
    pub fn created() -> Self {
        Self {
            status: ResponseStatus::Created,
            data: json!({}),
        }
    }
    
    /// Create an error response
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            status: ResponseStatus::Error,
            data: json!({
                "error": message.into()
            }),
        }
    }
    
    /// Add a field to the response
    pub fn with(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Some(obj) = self.data.as_object_mut() {
            obj.insert(key.into(), json!(value));
        }
        self
    }
    
    /// Build the response as a JSON Value
    pub fn build(self) -> Value {
        let mut response = json!({
            "status": self.status,
        });
        
        // Merge data fields into response
        if let (Some(response_obj), Some(data_obj)) = 
            (response.as_object_mut(), self.data.as_object()) {
            for (k, v) in data_obj {
                response_obj.insert(k.clone(), v.clone());
            }
        }
        
        response
    }
    
    /// Build the response as a JSON string
    pub fn build_string(self) -> String {
        self.build().to_string()
    }
}

/// Builder for query messages
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    entity_type: Option<String>,
    filters: Value,
    includes: Vec<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            entity_type: None,
            filters: json!({}),
            includes: Vec::new(),
            limit: None,
            offset: None,
        }
    }
    
    /// Set the entity type to query
    pub fn entity(mut self, entity_type: impl Into<String>) -> Self {
        self.entity_type = Some(entity_type.into());
        self
    }
    
    /// Add a filter condition
    pub fn filter(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Some(obj) = self.filters.as_object_mut() {
            obj.insert(key.into(), json!(value));
        }
        self
    }
    
    /// Include related entities
    pub fn include(mut self, relation: impl Into<String>) -> Self {
        self.includes.push(relation.into());
        self
    }
    
    /// Set the limit
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Set the offset
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
    
    /// Build the query
    pub fn build(self) -> Value {
        let mut query = json!({
            "filters": self.filters,
        });
        
        if let Some(entity_type) = self.entity_type {
            query["entity"] = json!(entity_type);
        }
        
        if !self.includes.is_empty() {
            query["include"] = json!(self.includes);
        }
        
        if let Some(limit) = self.limit {
            query["limit"] = json!(limit);
        }
        
        if let Some(offset) = self.offset {
            query["offset"] = json!(offset);
        }
        
        query
    }
}

/// Builder for mutation messages
#[derive(Debug, Clone)]
pub struct MutationBuilder {
    entity_type: String,
    operation: MutationOperation,
    data: Value,
}

#[derive(Debug, Clone, Copy)]
pub enum MutationOperation {
    Create,
    Update,
    Delete,
}

impl MutationBuilder {
    /// Create a new entity
    pub fn create(entity_type: impl Into<String>) -> Self {
        Self {
            entity_type: entity_type.into(),
            operation: MutationOperation::Create,
            data: json!({}),
        }
    }
    
    /// Update an existing entity
    pub fn update(entity_type: impl Into<String>, id: Uuid) -> Self {
        Self {
            entity_type: entity_type.into(),
            operation: MutationOperation::Update,
            data: json!({ "id": id }),
        }
    }
    
    /// Delete an entity
    pub fn delete(entity_type: impl Into<String>, id: Uuid) -> Self {
        Self {
            entity_type: entity_type.into(),
            operation: MutationOperation::Delete,
            data: json!({ "id": id }),
        }
    }
    
    /// Set a field value
    pub fn set(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Some(obj) = self.data.as_object_mut() {
            obj.insert(key.into(), json!(value));
        }
        self
    }
    
    /// Build the mutation
    pub fn build(self) -> Value {
        json!({
            "entity": self.entity_type,
            "operation": match self.operation {
                MutationOperation::Create => "create",
                MutationOperation::Update => "update",
                MutationOperation::Delete => "delete",
            },
            "data": self.data,
        })
    }
}

/// Parse a message payload
pub fn parse_message(message_type: &str, payload: &str) -> Result<(MessageType, Value), String> {
    let msg_type = MessageType::from_str(message_type)
        .ok_or_else(|| format!("Unknown message type: {}", message_type))?;
    
    let payload_value: Value = serde_json::from_str(payload)
        .map_err(|e| format!("Invalid JSON payload: {}", e))?;
    
    Ok((msg_type, payload_value))
}

/// Helper to extract command info from a command message
pub fn parse_command(payload: &Value) -> Result<(&str, &Value), String> {
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or("Missing command name")?;
    
    let args = payload.get("args")
        .ok_or("Missing command args")?;
    
    Ok((name, args))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_response_builder() {
        let response = ResponseBuilder::success()
            .with("message", "Operation completed")
            .with("count", 42)
            .build();
        
        assert_eq!(response["status"], "success");
        assert_eq!(response["message"], "Operation completed");
        assert_eq!(response["count"], 42);
    }
    
    #[test]
    fn test_error_response() {
        let response = ResponseBuilder::error("Something went wrong").build();
        
        assert_eq!(response["status"], "error");
        assert_eq!(response["error"], "Something went wrong");
    }
    
    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new()
            .entity("research_log")
            .filter("status", "active")
            .include("entries")
            .limit(10)
            .build();
        
        assert_eq!(query["entity"], "research_log");
        assert_eq!(query["filters"]["status"], "active");
        assert_eq!(query["include"], json!(["entries"]));
        assert_eq!(query["limit"], 10);
    }
    
    #[test]
    fn test_mutation_builder() {
        let id = Uuid::new_v4();
        let mutation = MutationBuilder::update("research_log", id)
            .set("title", "Updated Title")
            .set("status", "completed")
            .build();
        
        assert_eq!(mutation["entity"], "research_log");
        assert_eq!(mutation["operation"], "update");
        assert_eq!(mutation["data"]["id"], id.to_string());
        assert_eq!(mutation["data"]["title"], "Updated Title");
    }
}