use crate::ApiError;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Authentication context extracted from requests
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub api_key_id: Uuid,
    pub permissions: Vec<String>,
}

/// API key data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub key: String,
    pub user_id: Uuid,
    pub name: String,
    pub permissions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub active: bool,
}

/// In-memory API key store (for development)
pub struct ApiKeyStore {
    keys: HashMap<String, ApiKey>,
}

impl ApiKeyStore {
    pub fn new() -> Self {
        let mut store = Self {
            keys: HashMap::new(),
        };
        
        // Add default development API keys
        store.add_development_keys();
        store
    }
    
    fn add_development_keys(&mut self) {
        // Development key 1: Full access
        let dev_key = ApiKey {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            key: "dev_key_full_access".to_string(),
            user_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
            name: "Development Full Access".to_string(),
            permissions: vec!["*".to_string()],
            created_at: chrono::Utc::now(),
            expires_at: None,
            last_used_at: None,
            active: true,
        };
        self.keys.insert(dev_key.key.clone(), dev_key);
        
        // Development key 2: Read only
        let readonly_key = ApiKey {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            key: "dev_key_readonly".to_string(),
            user_id: Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(),
            name: "Development Read Only".to_string(),
            permissions: vec!["read:*".to_string()],
            created_at: chrono::Utc::now(),
            expires_at: None,
            last_used_at: None,
            active: true,
        };
        self.keys.insert(readonly_key.key.clone(), readonly_key);
        
        // Test key (the one used in tests)
        let test_key = ApiKey {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap(),
            key: "test-token".to_string(),
            user_id: Uuid::nil(),
            name: "Test Token".to_string(),
            permissions: vec!["*".to_string()],
            created_at: chrono::Utc::now(),
            expires_at: None,
            last_used_at: None,
            active: true,
        };
        self.keys.insert(test_key.key.clone(), test_key);
    }
    
    pub fn validate_key(&self, key: &str) -> Option<&ApiKey> {
        self.keys.get(key).filter(|k| {
            k.active && 
            k.expires_at.map_or(true, |exp| exp > chrono::Utc::now())
        })
    }
}

impl Default for ApiKeyStore {
    fn default() -> Self {
        Self::new()
    }
}

// Make ApiKeyStore globally accessible (for now)
lazy_static::lazy_static! {
    pub static ref API_KEY_STORE: ApiKeyStore = ApiKeyStore::new();
}

/// Extractor for authentication context
#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to extract authorization header
        let headers = parts.headers.clone();
        let auth_value = headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(ApiError::Unauthorized)?;
        
        // Check if it's a Bearer token
        let token = auth_value
            .strip_prefix("Bearer ")
            .ok_or(ApiError::Unauthorized)?;
        
        // Validate API key
        let api_key = API_KEY_STORE
            .validate_key(token)
            .ok_or(ApiError::Unauthorized)?;
        
        Ok(AuthContext {
            user_id: api_key.user_id,
            api_key_id: api_key.id,
            permissions: api_key.permissions.clone(),
        })
    }
}

/// Check if a user has a specific permission
pub fn has_permission(permissions: &[String], required: &str) -> bool {
    permissions.iter().any(|p| {
        p == "*" || p == required || {
            // Check wildcard permissions like "read:*"
            if let Some((prefix, _)) = required.split_once(':') {
                p == &format!("{}:*", prefix)
            } else {
                false
            }
        }
    })
}

/// Middleware function for authentication
pub async fn require_auth(
    auth: Result<AuthContext, ApiError>,
) -> Result<AuthContext, ApiError> {
    auth
}

/// Middleware function for specific permission
pub async fn require_permission(
    auth: AuthContext,
    permission: &str,
) -> Result<AuthContext, ApiError> {
    if has_permission(&auth.permissions, permission) {
        Ok(auth)
    } else {
        Err(ApiError::Forbidden)
    }
}