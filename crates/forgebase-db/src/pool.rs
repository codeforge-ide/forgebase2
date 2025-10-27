//! Database connection pooling with advanced features

use forgebase_core::{ForgeBaseError, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Database pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            database_url: String::from("postgres://localhost/forgebase"),
            max_connections: 100,
            min_connections: 10,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Some(Duration::from_secs(600)),
            max_lifetime: Some(Duration::from_secs(1800)),
        }
    }
}

/// Database pool manager
pub struct DatabasePool {
    pool: PgPool,
    config: PoolConfig,
}

impl DatabasePool {
    /// Create a new database pool
    pub async fn new(config: PoolConfig) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .max_lifetime(config.max_lifetime)
            .connect(&config.database_url)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(Self { pool, config })
    }

    /// Get the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get pool size information
    pub fn size(&self) -> u32 {
        self.pool.size()
    }

    /// Get number of idle connections
    pub fn num_idle(&self) -> usize {
        self.pool.num_idle()
    }

    /// Check if the pool is closed
    pub fn is_closed(&self) -> bool {
        self.pool.is_closed()
    }

    /// Close the pool
    pub async fn close(&self) {
        self.pool.close().await;
    }

    /// Health check
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;
        Ok(())
    }
}

impl Clone for DatabasePool {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            config: self.config.clone(),
        }
    }
}
