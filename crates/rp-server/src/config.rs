use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub enabled: bool,
    pub api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
            },
            database: DatabaseConfig {
                url: Self::build_database_url(),
                max_connections: 10,
            },
            auth: AuthConfig {
                enabled: false,
                api_key: None,
            },
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Override with environment variables
        if let Ok(host) = std::env::var("SERVER_HOST") {
            config.server.host = host;
        }
        if let Ok(port) = std::env::var("SERVER_PORT") {
            if let Ok(port) = port.parse() {
                config.server.port = port;
            }
        }

        config.database.url = Self::build_database_url();

        if let Ok(max_conn) = std::env::var("DB_MAX_CONNECTIONS") {
            if let Ok(max_conn) = max_conn.parse() {
                config.database.max_connections = max_conn;
            }
        }

        if let Ok(api_key) = std::env::var("API_KEY") {
            config.auth.enabled = true;
            config.auth.api_key = Some(api_key);
        }

        config
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid server address")
    }

    fn build_database_url() -> String {
        // Database configuration with development defaults
        let host = std::env::var("DB_HOST").unwrap_or_else(|_| "192.168.10.90".to_string());
        let port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let name = std::env::var("DB_NAME").unwrap_or_else(|_| "researchprocess_gps".to_string());
        let user = std::env::var("DB_USER").unwrap_or_else(|_| "researchprocess_gps".to_string());
        let password =
            std::env::var("DB_PASSWORD").unwrap_or_else(|_| "researchprocess_gps".to_string()); // Dev password

        format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, name)
    }
}