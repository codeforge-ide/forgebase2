//! Database migration management

use forgebase_core::{ForgeBaseError, Result};
use sqlx::PgPool;
use std::collections::HashMap;

/// Migration metadata
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: i64,
    pub name: String,
    pub up_sql: String,
    pub down_sql: String,
}

/// Migration manager
pub struct MigrationManager {
    pool: PgPool,
}

impl MigrationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Initialize migration tracking table
    pub async fn initialize(&self) -> Result<()> {
        let create_table = r#"
            CREATE TABLE IF NOT EXISTS forgebase_migrations (
                id SERIAL PRIMARY KEY,
                version BIGINT NOT NULL UNIQUE,
                name VARCHAR(255) NOT NULL,
                applied_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
        "#;

        sqlx::query(create_table)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }

    /// Run pending migrations
    pub async fn migrate(&self, migrations: &[Migration]) -> Result<Vec<i64>> {
        self.initialize().await?;

        let applied = self.get_applied_versions().await?;
        let mut newly_applied = Vec::new();

        for migration in migrations {
            if !applied.contains(&migration.version) {
                self.apply_migration(migration).await?;
                newly_applied.push(migration.version);
            }
        }

        Ok(newly_applied)
    }

    /// Apply a single migration
    async fn apply_migration(&self, migration: &Migration) -> Result<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        // Execute migration SQL
        sqlx::query(&migration.up_sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        // Record migration
        sqlx::query(
            "INSERT INTO forgebase_migrations (version, name) VALUES ($1, $2)",
        )
        .bind(migration.version)
        .bind(&migration.name)
        .execute(&mut *tx)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }

    /// Rollback the last migration
    pub async fn rollback(&self, migration: &Migration) -> Result<()> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        // Execute rollback SQL
        sqlx::query(&migration.down_sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        // Remove migration record
        sqlx::query("DELETE FROM forgebase_migrations WHERE version = $1")
            .bind(migration.version)
            .execute(&mut *tx)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }

    /// Get list of applied migration versions
    async fn get_applied_versions(&self) -> Result<Vec<i64>> {
        let rows = sqlx::query_as::<_, (i64,)>(
            "SELECT version FROM forgebase_migrations ORDER BY version",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|(v,)| v).collect())
    }

    /// Get migration status
    pub async fn status(&self) -> Result<HashMap<i64, MigrationStatus>> {
        self.initialize().await?;

        let rows = sqlx::query_as::<_, (i64, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT version, name, applied_at FROM forgebase_migrations ORDER BY version",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        let mut status = HashMap::new();
        for (version, name, applied_at) in rows {
            status.insert(
                version,
                MigrationStatus {
                    version,
                    name,
                    applied_at: Some(applied_at),
                },
            );
        }

        Ok(status)
    }
}

/// Migration status
#[derive(Debug, Clone)]
pub struct MigrationStatus {
    pub version: i64,
    pub name: String,
    pub applied_at: Option<chrono::DateTime<chrono::Utc>>,
}
