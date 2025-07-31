//! WebSocket protocol definitions for real-time communication

use crate::{SubscriptionRequest, EntityResponse, ErrorDetail};
use chrono::{DateTime, Utc};
use rp_core::layer3::EntityType;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Client-to-server WebSocket message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMessage {
    /// Client-generated request ID for correlation
    pub id: String,
    /// Message type
    #[serde(flatten)]
    pub payload: ClientMessagePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessagePayload {
    /// Authenticate the connection
    Auth { token: String },
    
    /// Subscribe to real-time updates
    Subscribe(SubscriptionRequest),
    
    /// Unsubscribe from updates
    Unsubscribe { subscription_id: String },
    
    /// Execute an API request over WebSocket
    Request { 
        method: String,
        path: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<JsonValue>,
    },
    
    /// Respond to server ping
    Pong { timestamp: DateTime<Utc> },
}

/// Server-to-client WebSocket message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMessage {
    /// Request ID if responding to a client request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    
    /// Message payload
    #[serde(flatten)]
    pub payload: ServerMessagePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessagePayload {
    /// Authentication result
    AuthResult { 
        success: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
    
    /// Real-time event notification
    Event(EventNotification),
    
    /// Response to a request
    Response { 
        status: u16,
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<JsonValue>,
    },
    
    /// Error message
    Error(ErrorMessage),
    
    /// Subscription confirmation
    SubscriptionCreated { 
        subscription_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        existing_data: Option<Vec<EntityResponse>>,
    },
    
    /// Subscription removed
    SubscriptionRemoved { 
        subscription_id: String,
    },
    
    /// Keep-alive ping
    Ping { timestamp: DateTime<Utc> },
    
    /// Server is shutting down
    Shutdown { 
        reason: String,
        reconnect_after: Option<DateTime<Utc>>,
    },
}

/// Real-time event notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNotification {
    pub event_id: Uuid,
    pub event_type: EventType,
    pub entity_type: EntityType,
    pub entity_id: Uuid,
    pub version: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Created,
    Updated,
    Deleted,
    StateChanged,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub occurred_at: DateTime<Utc>,
    pub actor_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// WebSocket error message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    AuthenticationError,
    AuthorizationError,
    ValidationError,
    NotFound,
    Conflict,
    RateLimit,
    InternalError,
    ProtocolError,
    SubscriptionError,
}

/// WebSocket connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    /// Maximum subscriptions per connection
    pub max_subscriptions: u32,
    /// Ping interval in seconds
    pub ping_interval: u32,
    /// Timeout for pong response in seconds
    pub pong_timeout: u32,
    /// Maximum message size in bytes
    pub max_message_size: usize,
    /// Enable compression
    pub compression: bool,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            max_subscriptions: 100,
            ping_interval: 30,
            pong_timeout: 10,
            max_message_size: 1024 * 1024, // 1MB
            compression: true,
        }
    }
}

/// WebSocket frame type for binary protocol (future)
#[derive(Debug, Clone, Copy)]
pub enum FrameType {
    Text,
    Binary,
    Close,
    Ping,
    Pong,
}

/// Connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Connecting,
    Authenticating,
    Connected,
    Disconnecting,
    Disconnected,
}

/// Subscription state
#[derive(Debug, Clone)]
pub struct SubscriptionState {
    pub id: String,
    pub request: SubscriptionRequest,
    pub created_at: DateTime<Utc>,
    pub event_count: u64,
    pub last_event_at: Option<DateTime<Utc>>,
}