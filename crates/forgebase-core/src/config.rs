use serde::Deserialize;
use std::collections::HashMap;

/// Global configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
    pub storage: StorageConfig,
    pub email: EmailConfig,
    pub sites: SitesConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: Environment,
    pub cors_origins: Vec<String>,
    pub max_body_size: usize,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub refresh_token_expiration: i64,
    pub password_min_length: usize,
    pub enable_email_verification: bool,
    pub enable_magic_links: bool,
    pub oauth_providers: HashMap<String, OAuthProviderConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub backend: StorageBackend,
    pub bucket: String,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub max_file_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackend {
    Local,
    S3,
    Gcs,
    Azure,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub from_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SitesConfig {
    pub storage_path: String,
    pub max_deployment_size: usize,
    pub custom_domains_enabled: bool,
    pub ssl_enabled: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            environment: Environment::Development,
            cors_origins: vec!["*".to_string()],
            max_body_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        
        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        
        Ok(config.try_deserialize()?)
    }

    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        
        Ok(config.try_deserialize()?)
    }
}
