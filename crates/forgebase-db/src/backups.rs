//! Database backup and restore functionality

use forgebase_core::{ForgeBaseError, Result};
use sqlx::PgPool;
use std::path::Path;
use tokio::process::Command;

/// Backup configuration
#[derive(Debug, Clone)]
pub struct BackupConfig {
    pub database_url: String,
    pub backup_dir: String,
}

/// Backup manager
pub struct BackupManager {
    pool: PgPool,
    config: BackupConfig,
}

impl BackupManager {
    pub fn new(pool: PgPool, config: BackupConfig) -> Self {
        Self { pool, config }
    }

    /// Create a database backup
    pub async fn create_backup(&self, backup_name: &str) -> Result<String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_file = format!("{}/{}_{}.sql", self.config.backup_dir, backup_name, timestamp);

        // Ensure backup directory exists
        tokio::fs::create_dir_all(&self.config.backup_dir)
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to create backup directory: {}", e)))?;

        // Use pg_dump to create backup
        let output = Command::new("pg_dump")
            .arg(&self.config.database_url)
            .arg("--file")
            .arg(&backup_file)
            .arg("--format=plain")
            .arg("--verbose")
            .output()
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to execute pg_dump: {}", e)))?;

        if !output.status.success() {
            return Err(ForgeBaseError::Internal(format!(
                "Backup failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(backup_file)
    }

    /// Restore from a backup
    pub async fn restore_backup(&self, backup_file: &str) -> Result<()> {
        if !Path::new(backup_file).exists() {
            return Err(ForgeBaseError::NotFound(format!(
                "Backup file not found: {}",
                backup_file
            )));
        }

        // Use psql to restore backup
        let output = Command::new("psql")
            .arg(&self.config.database_url)
            .arg("--file")
            .arg(backup_file)
            .output()
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to execute psql: {}", e)))?;

        if !output.status.success() {
            return Err(ForgeBaseError::Internal(format!(
                "Restore failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        Ok(())
    }

    /// List available backups
    pub async fn list_backups(&self) -> Result<Vec<BackupInfo>> {
        let mut backups = Vec::new();
        let backup_dir = Path::new(&self.config.backup_dir);

        if !backup_dir.exists() {
            return Ok(backups);
        }

        let mut entries = tokio::fs::read_dir(backup_dir)
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to read backup directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                let metadata = entry
                    .metadata()
                    .await
                    .map_err(|e| ForgeBaseError::Internal(format!("Failed to read metadata: {}", e)))?;

                backups.push(BackupInfo {
                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                    path: path.to_string_lossy().to_string(),
                    size: metadata.len(),
                    created_at: metadata
                        .modified()
                        .ok()
                        .and_then(|t| chrono::DateTime::from_timestamp(
                            t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                            0
                        ))
                        .unwrap_or_else(chrono::Utc::now),
                });
            }
        }

        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(backups)
    }

    /// Delete a backup
    pub async fn delete_backup(&self, backup_file: &str) -> Result<()> {
        let path = Path::new(backup_file);
        if !path.exists() {
            return Err(ForgeBaseError::NotFound(format!(
                "Backup file not found: {}",
                backup_file
            )));
        }

        tokio::fs::remove_file(path)
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to delete backup: {}", e)))?;

        Ok(())
    }

    /// Create a point-in-time recovery checkpoint
    pub async fn create_checkpoint(&self) -> Result<()> {
        sqlx::query("CHECKPOINT")
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(e.to_string()))?;

        Ok(())
    }
}

/// Backup information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
