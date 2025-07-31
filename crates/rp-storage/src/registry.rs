//! Storage backend registry for dynamic backend selection

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

use crate::{StorageError, StorageResult};

/// Trait object-safe version of StorageBackend
#[async_trait]
pub trait DynStorageBackend: Send + Sync {
    /// Initialize storage (create tables, directories, etc.)
    async fn initialize(&self) -> StorageResult<()>;
    
    /// Check if storage is properly configured and accessible
    async fn health_check(&self) -> StorageResult<()>;
    
    /// Get backend type/name
    fn backend_type(&self) -> &str;
}

/// Factory trait for creating storage backends
#[async_trait]
pub trait BackendFactory: Send + Sync {
    /// Create a new backend instance from URL
    async fn create(&self, url: &str) -> StorageResult<Box<dyn DynStorageBackend>>;
    
    /// Get the scheme this factory handles (e.g., "postgres", "sqlite")
    fn scheme(&self) -> &str;
    
    /// Validate a URL without creating the backend
    fn validate_url(&self, url: &str) -> StorageResult<()>;
}

/// Registry for storage backend factories
pub struct StorageRegistry {
    factories: HashMap<String, Arc<dyn BackendFactory>>,
}

impl StorageRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }
    
    /// Create a registry with default backends registered
    pub fn with_defaults() -> Self {
        let registry = Self::new();
        
        // Register built-in backends based on features
        // TODO: Add feature flags and implementations when ready
        // #[cfg(feature = "postgres")]
        // registry.register(PostgresBackendFactory::new());
        
        // #[cfg(feature = "sqlite")]
        // registry.register(SqliteBackendFactory::new());
        
        // #[cfg(feature = "storage-git")]
        // registry.register(GitBackendFactory::new());
        
        // #[cfg(feature = "storage-fs")]
        // registry.register(FilesystemBackendFactory::new());
        
        registry
    }
    
    /// Register a backend factory
    pub fn register<F: BackendFactory + 'static>(&mut self, factory: F) {
        let scheme = factory.scheme().to_string();
        self.factories.insert(scheme, Arc::new(factory));
    }
    
    /// Create a backend from a URL
    pub async fn create_backend(&self, url: &str) -> StorageResult<Box<dyn DynStorageBackend>> {
        let scheme = extract_scheme(url)?;
        
        let factory = self.factories.get(scheme)
            .ok_or_else(|| StorageError::ConfigError(
                format!("Unknown storage backend: {}", scheme)
            ))?;
        
        factory.create(url).await
    }
    
    /// List available backend schemes
    pub fn available_backends(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_str()).collect()
    }
    
    /// Check if a backend is available
    pub fn has_backend(&self, scheme: &str) -> bool {
        self.factories.contains_key(scheme)
    }
}

impl Default for StorageRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Extract scheme from storage URL
fn extract_scheme(url: &str) -> StorageResult<&str> {
    url.split("://")
        .next()
        .ok_or_else(|| StorageError::ConfigError(
            format!("Invalid storage URL format: {}", url)
        ))
}

/// Parse storage URL into components
pub fn parse_storage_url(url: &str) -> StorageResult<StorageUrl> {
    let scheme = extract_scheme(url)?;
    let remainder = &url[scheme.len() + 3..]; // Skip "://"
    
    // Handle different URL formats
    let (connection_string, params) = if let Some(pos) = remainder.find('?') {
        let (conn, params_str) = remainder.split_at(pos);
        let params = parse_query_params(&params_str[1..])?;
        (conn.to_string(), params)
    } else {
        (remainder.to_string(), HashMap::new())
    };
    
    Ok(StorageUrl {
        scheme: scheme.to_string(),
        connection_string,
        parameters: params,
    })
}

/// Parsed storage URL
#[derive(Debug, Clone)]
pub struct StorageUrl {
    pub scheme: String,
    pub connection_string: String,
    pub parameters: HashMap<String, String>,
}

/// Parse query parameters from URL
fn parse_query_params(params: &str) -> StorageResult<HashMap<String, String>> {
    let mut map = HashMap::new();
    
    for pair in params.split('&') {
        if pair.is_empty() {
            continue;
        }
        
        let (key, value) = if let Some(pos) = pair.find('=') {
            let (k, v) = pair.split_at(pos);
            (k, &v[1..])
        } else {
            (pair, "")
        };
        
        map.insert(
            key.to_string(),
            urlencoding::decode(value)
                .map_err(|e| StorageError::ConfigError(e.to_string()))?
                .into_owned()
        );
    }
    
    Ok(map)
}

/// Helper to build storage URLs
pub struct StorageUrlBuilder {
    scheme: String,
    connection: String,
    parameters: HashMap<String, String>,
}

impl StorageUrlBuilder {
    /// Create a new URL builder
    pub fn new(scheme: impl Into<String>) -> Self {
        Self {
            scheme: scheme.into(),
            connection: String::new(),
            parameters: HashMap::new(),
        }
    }
    
    /// Set connection string
    pub fn connection(mut self, conn: impl Into<String>) -> Self {
        self.connection = conn.into();
        self
    }
    
    /// Add a parameter
    pub fn param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }
    
    /// Build the URL
    pub fn build(self) -> String {
        let mut url = format!("{}://{}", self.scheme, self.connection);
        
        if !self.parameters.is_empty() {
            url.push('?');
            let params: Vec<String> = self.parameters
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(&v)))
                .collect();
            url.push_str(&params.join("&"));
        }
        
        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_storage_url() {
        let url = "postgres://user:pass@localhost/db?vector=true&graph=true";
        let parsed = parse_storage_url(url).unwrap();
        
        assert_eq!(parsed.scheme, "postgres");
        assert_eq!(parsed.connection_string, "user:pass@localhost/db");
        assert_eq!(parsed.parameters.get("vector"), Some(&"true".to_string()));
        assert_eq!(parsed.parameters.get("graph"), Some(&"true".to_string()));
    }
    
    #[test]
    fn test_url_builder() {
        let url = StorageUrlBuilder::new("sqlite")
            .connection("/path/to/db.sqlite")
            .param("vector", "true")
            .param("mode", "ro")
            .build();
        
        assert_eq!(url, "sqlite:///path/to/db.sqlite?vector=true&mode=ro");
    }
}