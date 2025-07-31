//! PostgreSQL configuration

use serde::{Deserialize, Serialize};
use url::Url;
use std::time::Duration;

/// PostgreSQL backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    /// Database host
    pub host: String,
    
    /// Database port
    pub port: u16,
    
    /// Database name
    pub database: String,
    
    /// Username
    pub username: String,
    
    /// Password
    pub password: String,
    
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    
    /// Minimum number of connections to maintain
    pub min_connections: u32,
    
    /// Connection timeout
    pub connect_timeout: Duration,
    
    /// Idle timeout before closing connections
    pub idle_timeout: Option<Duration>,
    
    /// Maximum lifetime of a connection
    pub max_lifetime: Option<Duration>,
    
    /// Connection timeout
    pub connection_timeout: Duration,
    
    /// Enable statement-level caching
    pub statement_cache_capacity: usize,
    
    /// Enable SSL/TLS
    pub ssl_mode: SslMode,
    
    /// Application name for pg_stat_activity
    pub application_name: String,
    
    /// Enable pgvector extension
    pub enable_vector: bool,
    
    /// Enable Apache AGE extension
    pub enable_graph: bool,
    
    /// Enable LISTEN/NOTIFY for real-time updates
    pub enable_notifications: bool,
}

/// SSL/TLS modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            username: "postgres".to_string(),
            password: String::new(),
            max_connections: 32,
            min_connections: 5,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Some(Duration::from_secs(600)),
            max_lifetime: Some(Duration::from_secs(1800)),
            connection_timeout: Duration::from_secs(30),
            statement_cache_capacity: 100,
            ssl_mode: SslMode::Prefer,
            application_name: "researchprocess-gps".to_string(),
            enable_vector: false,
            enable_graph: false,
            enable_notifications: true,
        }
    }
}

impl PostgresConfig {
    /// Create config from database URL
    pub fn from_url(url_str: &str) -> Result<Self, rp_storage::StorageError> {
        let url = Url::parse(url_str)
            .map_err(|e| rp_storage::StorageError::ConfigError(e.to_string()))?;
        
        if url.scheme() != "postgres" && url.scheme() != "postgresql" {
            return Err(rp_storage::StorageError::ConfigError(
                format!("Invalid scheme: expected 'postgres' or 'postgresql', got '{}'", url.scheme())
            ));
        }
        
        // Host is required for database connection
        let host = url.host_str()
            .ok_or_else(|| rp_storage::StorageError::ConfigError(
                "Database URL must include a host".to_string()
            ))?
            .to_string();
        
        // Port can be optional - PostgreSQL default is 5432
        let port = url.port().unwrap_or(5432); // PostgreSQL default port
        
        // Database name is required
        let database = url.path().trim_start_matches('/');
        if database.is_empty() {
            return Err(rp_storage::StorageError::ConfigError(
                "Database URL must include a database name after the host".to_string()
            ));
        }
        
        // Username is required
        let username = url.username();
        if username.is_empty() {
            return Err(rp_storage::StorageError::ConfigError(
                "Database URL must include a username".to_string()
            ));
        }
        
        // Password can be empty but should be explicit
        let password = url.password().map(|s| s.to_string());
        
        let mut config = Self {
            host,
            port,
            database: database.to_string(),
            username: username.to_string(),
            password: password.unwrap_or_default(), // Empty password is valid
            ..Default::default()
        };
        
        // Parse query parameters
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "max_connections" => {
                    config.max_connections = value.parse()
                        .map_err(|_| rp_storage::StorageError::ConfigError(
                            format!("Invalid max_connections: {}", value)
                        ))?;
                }
                "min_connections" => {
                    config.min_connections = value.parse()
                        .map_err(|_| rp_storage::StorageError::ConfigError(
                            format!("Invalid min_connections: {}", value)
                        ))?;
                }
                "connect_timeout" => {
                    let secs: u64 = value.parse()
                        .map_err(|_| rp_storage::StorageError::ConfigError(
                            format!("Invalid connect_timeout: {}", value)
                        ))?;
                    config.connect_timeout = Duration::from_secs(secs);
                }
                "sslmode" => {
                    config.ssl_mode = match value.as_ref() {
                        "disable" => SslMode::Disable,
                        "allow" => SslMode::Allow,
                        "prefer" => SslMode::Prefer,
                        "require" => SslMode::Require,
                        "verify-ca" => SslMode::VerifyCa,
                        "verify-full" => SslMode::VerifyFull,
                        _ => return Err(rp_storage::StorageError::ConfigError(
                            format!("Invalid sslmode: {}", value)
                        )),
                    };
                }
                "application_name" => {
                    config.application_name = value.to_string();
                }
                "vector" | "enable_vector" => {
                    config.enable_vector = value == "true" || value == "1";
                }
                "graph" | "enable_graph" => {
                    config.enable_graph = value == "true" || value == "1";
                }
                "notifications" | "enable_notifications" => {
                    config.enable_notifications = value == "true" || value == "1";
                }
                _ => {} // Ignore unknown parameters
            }
        }
        
        Ok(config)
    }
    
    /// Build database URL from components
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            urlencoding::encode(&self.username),
            urlencoding::encode(&self.password),
            &self.host,
            self.port,
            &self.database
        )
    }
    
    /// Build SQLx connection options
    pub fn to_sqlx_options(&self) -> sqlx::postgres::PgConnectOptions {
        let mut options = self.database_url().parse::<sqlx::postgres::PgConnectOptions>()
            .expect("Invalid database URL");
        
        options = options
            .application_name(&self.application_name)
            .statement_cache_capacity(self.statement_cache_capacity);
        
        // Set SSL mode
        options = match self.ssl_mode {
            SslMode::Disable => options.ssl_mode(sqlx::postgres::PgSslMode::Disable),
            SslMode::Allow => options.ssl_mode(sqlx::postgres::PgSslMode::Allow),
            SslMode::Prefer => options.ssl_mode(sqlx::postgres::PgSslMode::Prefer),
            SslMode::Require => options.ssl_mode(sqlx::postgres::PgSslMode::Require),
            SslMode::VerifyCa => options.ssl_mode(sqlx::postgres::PgSslMode::VerifyCa),
            SslMode::VerifyFull => options.ssl_mode(sqlx::postgres::PgSslMode::VerifyFull),
        };
        
        options
    }
}