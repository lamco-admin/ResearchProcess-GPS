//! ResearchProcess-GPS Protocol Definitions
//! 
//! This crate defines the protocol for client-server communication,
//! supporting REST, WebSocket, and future GraphQL operations.

pub mod requests;
pub mod responses;
pub mod websocket;
pub mod errors;
pub mod pagination;
pub mod filters;
pub mod auth;

pub use requests::*;
pub use responses::*;
pub use websocket::*;
pub use errors::*;
pub use pagination::*;
pub use filters::*;
pub use auth::*;


/// Protocol version for this implementation
pub const PROTOCOL_VERSION: &str = "1.0.0";

/// API version path segment
pub const API_VERSION: &str = "v1";

/// Base path for REST API
pub fn api_base_path() -> String {
    format!("/api/{}", API_VERSION)
}

/// Base path for WebSocket
pub fn ws_base_path() -> String {
    format!("/ws/{}", API_VERSION)
}
