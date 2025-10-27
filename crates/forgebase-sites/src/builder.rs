use crate::models::*;
use forgebase_core::{ForgeBaseError, Result};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tracing::info;
use uuid::Uuid;

/// Build manager for deploying sites
pub struct BuildManager {
    builds_dir: PathBuf,
}

impl BuildManager {
    pub fn new(builds_dir: PathBuf) -> Self {
        Self { builds_dir }
    }

    /// Build a site deployment
    pub async fn build_deployment(
        &self,
        deployment_id: Uuid,
        source_path: &Path,
        config: &BuildConfig,
    ) -> Result<BuildResult> {
        info!("Starting build for deployment {}", deployment_id);

        let build_dir = self.builds_dir.join(deployment_id.to_string());
        tokio::fs::create_dir_all(&build_dir).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to create build directory: {}", e))
        })?;

        let start_time = std::time::Instant::now();
        let mut logs = Vec::new();

        // Install dependencies
        if let Some(install_cmd) = &config.install_command {
            info!("Running install command: {}", install_cmd);
            let install_result = self
                .run_command(install_cmd, source_path, &config.environment_variables)
                .await?;
            logs.push(format!("=== Install ===\n{}", install_result.output));

            if !install_result.success {
                return Ok(BuildResult {
                    success: false,
                    logs: logs.join("\n"),
                    duration_ms: start_time.elapsed().as_millis() as i64,
                    output_path: None,
                    size_bytes: None,
                    error: Some("Install failed".to_string()),
                });
            }
        }

        // Run build command
        let build_cmd = config
            .build_command
            .clone()
            .unwrap_or_else(|| config.default_build_command());
        
        info!("Running build command: {}", build_cmd);
        let build_result = self
            .run_command(&build_cmd, source_path, &config.environment_variables)
            .await?;
        logs.push(format!("=== Build ===\n{}", build_result.output));

        if !build_result.success {
            return Ok(BuildResult {
                success: false,
                logs: logs.join("\n"),
                duration_ms: start_time.elapsed().as_millis() as i64,
                output_path: None,
                size_bytes: None,
                error: Some("Build failed".to_string()),
            });
        }

        // Copy output to build directory
        let output_dir = config
            .output_directory
            .clone()
            .unwrap_or_else(|| config.default_output_directory());
        
        let output_path = source_path.join(&output_dir);
        if !output_path.exists() {
            return Ok(BuildResult {
                success: false,
                logs: logs.join("\n"),
                duration_ms: start_time.elapsed().as_millis() as i64,
                output_path: None,
                size_bytes: None,
                error: Some(format!("Output directory not found: {}", output_dir)),
            });
        }

        // Copy files
        self.copy_dir_recursive(&output_path, &build_dir).await?;

        // Calculate size
        let size = self.calculate_dir_size(&build_dir).await?;

        Ok(BuildResult {
            success: true,
            logs: logs.join("\n"),
            duration_ms: start_time.elapsed().as_millis() as i64,
            output_path: Some(build_dir),
            size_bytes: Some(size as i64),
            error: None,
        })
    }

    /// Run a shell command
    async fn run_command(
        &self,
        command: &str,
        working_dir: &Path,
        env_vars: &serde_json::Value,
    ) -> Result<CommandResult> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .current_dir(working_dir)
            .envs(
                env_vars
                    .as_object()
                    .unwrap_or(&serde_json::Map::new())
                    .iter()
                    .filter_map(|(k, v)| v.as_str().map(|v| (k.clone(), v.to_string()))),
            )
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to spawn command: {}", e)))?;

        let output = child.wait_with_output().await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to wait for command: {}", e))
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = format!("{}\n{}", stdout, stderr);

        Ok(CommandResult {
            success: output.status.success(),
            output: combined,
        })
    }

    /// Copy directory recursively
    async fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        tokio::fs::create_dir_all(dst).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to create directory: {}", e))
        })?;

        let mut entries = tokio::fs::read_dir(src).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to read directory: {}", e))
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to read entry: {}", e))
        })? {
            let file_type = entry.file_type().await.map_err(|e| {
                ForgeBaseError::Internal(format!("Failed to get file type: {}", e))
            })?;
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                Box::pin(self.copy_dir_recursive(&entry.path(), &dst_path)).await?;
            } else {
                tokio::fs::copy(&entry.path(), &dst_path).await.map_err(|e| {
                    ForgeBaseError::Internal(format!("Failed to copy file: {}", e))
                })?;
            }
        }

        Ok(())
    }

    /// Calculate directory size
    async fn calculate_dir_size(&self, dir: &Path) -> Result<u64> {
        let mut size = 0u64;
        let mut entries = tokio::fs::read_dir(dir).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to read directory: {}", e))
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to read entry: {}", e))
        })? {
            let metadata = entry.metadata().await.map_err(|e| {
                ForgeBaseError::Internal(format!("Failed to get metadata: {}", e))
            })?;

            if metadata.is_dir() {
                size += Box::pin(self.calculate_dir_size(&entry.path())).await?;
            } else {
                size += metadata.len();
            }
        }

        Ok(size)
    }
}

/// Build result
pub struct BuildResult {
    pub success: bool,
    pub logs: String,
    pub duration_ms: i64,
    pub output_path: Option<PathBuf>,
    pub size_bytes: Option<i64>,
    pub error: Option<String>,
}

/// Command execution result
struct CommandResult {
    success: bool,
    output: String,
}
