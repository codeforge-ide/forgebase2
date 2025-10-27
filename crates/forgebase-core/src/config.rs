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

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgresql://forgebase:forgebase@localhost:5432/forgebase".to_string(),
            max_connections: 100,
            min_connections: 10,
            acquire_timeout: 30,
            idle_timeout: 600,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "dev-secret-key-change-in-production".to_string(),
            jwt_expiration: 3600,
            refresh_token_expiration: 2592000,
            password_min_length: 8,
            enable_email_verification: true,
            enable_magic_links: true,
            oauth_providers: HashMap::new(),
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::Local,
            bucket: "forgebase-bucket".to_string(),
            region: None,
            endpoint: None,
            access_key: None,
            secret_key: None,
            max_file_size: 500 * 1024 * 1024, // 500MB
        }
    }
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_host: "localhost".to_string(),
            smtp_port: 1025,
            smtp_username: String::new(),
            smtp_password: String::new(),
            from_email: "noreply@forgebase.dev".to_string(),
            from_name: "ForgeBase".to_string(),
        }
    }
}

impl Default for SitesConfig {
    fn default() -> Self {
        Self {
            storage_path: "./data/sites".to_string(),
            max_deployment_size: 500 * 1024 * 1024, // 500MB
            custom_domains_enabled: true,
            ssl_enabled: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            storage: StorageConfig::default(),
            email: EmailConfig::default(),
            sites: SitesConfig::default(),
        }
    }
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        
        let mut builder = config::Config::builder()
            .add_source(config::Environment::default().separator("__"));
        
        // Try to build the config, and fall back to defaults if any field is missing
        match builder.build() {
            Ok(cfg) => cfg.try_deserialize::<Self>()
                .or_else(|_| {
                    // If deserialization fails due to missing fields, use defaults
                    // but override with whatever is actually provided
                    let mut defaults = Config::default();
                    
                    // Override with actual env vars if they exist
                    if let Ok(host) = std::env::var("SERVER__HOST") {
                        defaults.server.host = host;
                    }
                    if let Ok(port_str) = std::env::var("SERVER__PORT") {
                        if let Ok(port) = port_str.parse() {
                            defaults.server.port = port;
                        }
                    }
                    if let Ok(env_str) = std::env::var("SERVER__ENVIRONMENT") {
                        defaults.server.environment = match env_str.as_str() {
                            "production" => Environment::Production,
                            "staging" => Environment::Staging,
                            _ => Environment::Development,
                        };
                    }
                    if let Ok(db_url) = std::env::var("DATABASE__URL") {
                        defaults.database.url = db_url;
                    }
                    if let Ok(jwt_secret) = std::env::var("AUTH__JWT_SECRET") {
                        defaults.auth.jwt_secret = jwt_secret;
                    }
                    
                    Ok(defaults)
                }),
            Err(_) => Ok(Config::default())
        }
    }

    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name(path))
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        
        Ok(config.try_deserialize()?)
    }
}
