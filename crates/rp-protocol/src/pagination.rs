//! Pagination utilities and types

use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, IntoParams};

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct PaginationParams {
    /// Maximum number of items to return
    #[serde(default = "default_page_size")]
    pub limit: u32,
    
    /// Number of items to skip
    #[serde(default)]
    pub offset: u32,
    
    /// Cursor for cursor-based pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

fn default_page_size() -> u32 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: default_page_size(),
            offset: 0,
            cursor: None,
        }
    }
}

impl PaginationParams {
    pub const MAX_LIMIT: u32 = 100;
    
    /// Validate and normalize pagination parameters
    pub fn validate(&mut self) {
        if self.limit > Self::MAX_LIMIT {
            self.limit = Self::MAX_LIMIT;
        }
        if self.limit == 0 {
            self.limit = default_page_size();
        }
    }
    
    /// Calculate if there are more results
    pub fn has_more(&self, total: u64) -> bool {
        (self.offset as u64 + self.limit as u64) < total
    }
    
    /// Get the next offset
    pub fn next_offset(&self) -> u32 {
        self.offset + self.limit
    }
}

/// Pagination metadata for responses
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl PaginationMeta {
    pub fn new(params: &PaginationParams, total: u64) -> Self {
        Self {
            total,
            limit: params.limit,
            offset: params.offset,
            has_more: params.has_more(total),
            cursor: params.cursor.clone(),
            next_cursor: None,
        }
    }
    
    pub fn with_cursor(mut self, next_cursor: Option<String>) -> Self {
        self.next_cursor = next_cursor;
        self
    }
}

/// Cursor for cursor-based pagination
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Cursor {
    pub value: String,
    pub timestamp: i64,
}

impl Cursor {
    pub fn new(value: String) -> Self {
        Self {
            value,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    pub fn encode(&self) -> String {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        // Cursor is a simple struct, serialization should never fail
        let json = serde_json::to_string(self)
            .expect("Cursor serialization should never fail");
        URL_SAFE_NO_PAD.encode(json.as_bytes())
    }
    
    pub fn decode(encoded: &str) -> Option<Self> {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let bytes = URL_SAFE_NO_PAD.decode(encoded).ok()?;
        let json = String::from_utf8(bytes).ok()?;
        serde_json::from_str(&json).ok()
    }
}