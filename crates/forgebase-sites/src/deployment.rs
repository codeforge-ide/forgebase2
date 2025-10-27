use crate::models::*;
use crate::builder::{BuildManager, BuildResult};
use forgebase_core::{ForgeBaseError, Result};
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use tracing::info;
use uuid::Uuid;

/// Deployment manager
pub struct DeploymentManager {
    storage_path: PathBuf,
    build_manager: BuildManager,
}

impl DeploymentManager {
    pub fn new(storage_path: PathBuf) -> Self {
        let builds_dir = storage_path.join("builds");
        let build_manager = BuildManager::new(builds_dir);

        Self {
            storage_path,
            build_manager,
        }
    }

    /// Deploy a site
    pub async fn deploy(
        &self,
        deployment: &Deployment,
        site: &Site,
        source_archive: &[u8],
    ) -> Result<BuildResult> {
        info!("Starting deployment {} for site {}", deployment.id, site.id);

        // Extract source archive
        let source_dir = self.extract_source(deployment.id, source_archive).await?;

        // Build configuration
        let build_config = BuildConfig {
            framework: site.framework.clone(),
            build_command: site.build_command.clone(),
            output_directory: site.output_directory.clone(),
            install_command: site.install_command.clone(),
            node_version: "18".to_string(),
            environment_variables: site.environment_variables.clone(),
        };

        // Build the deployment
        let build_result = self
            .build_manager
            .build_deployment(deployment.id, &source_dir, &build_config)
            .await?;

        // If successful, deploy to storage
        if build_result.success {
            if let Some(output_path) = &build_result.output_path {
                self.deploy_to_storage(deployment.id, output_path).await?;
            }
        }

        Ok(build_result)
    }

    /// Extract source archive
    async fn extract_source(&self, deployment_id: Uuid, archive: &[u8]) -> Result<PathBuf> {
        let source_dir = self.storage_path.join("sources").join(deployment_id.to_string());
        tokio::fs::create_dir_all(&source_dir).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to create source directory: {}", e))
        })?;

        // Write archive to temp file
        let archive_path = source_dir.join("source.tar.gz");
        let mut file = tokio::fs::File::create(&archive_path).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to create archive file: {}", e))
        })?;
        file.write_all(archive).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to write archive: {}", e))
        })?;

        // Extract archive
        let tar_gz = std::fs::File::open(&archive_path).map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to open archive: {}", e))
        })?;
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);
        
        archive.unpack(&source_dir).map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to extract archive: {}", e))
        })?;

        Ok(source_dir)
    }

    /// Deploy built files to storage
    async fn deploy_to_storage(&self, deployment_id: Uuid, output_path: &Path) -> Result<()> {
        let deploy_dir = self
            .storage_path
            .join("deployments")
            .join(deployment_id.to_string());

        tokio::fs::create_dir_all(&deploy_dir).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to create deployment directory: {}", e))
        })?;

        // Copy built files
        self.copy_dir_recursive(output_path, &deploy_dir).await?;

        info!("Deployed to {}", deploy_dir.display());
        Ok(())
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

    /// Get deployment path
    pub fn get_deployment_path(&self, deployment_id: Uuid) -> PathBuf {
        self.storage_path
            .join("deployments")
            .join(deployment_id.to_string())
    }

    /// Serve static file from deployment
    pub async fn serve_file(
        &self,
        deployment_id: Uuid,
        file_path: &str,
    ) -> Result<(Vec<u8>, String)> {
        let deployment_path = self.get_deployment_path(deployment_id);
        let full_path = deployment_path.join(file_path.trim_start_matches('/'));

        // Security check: ensure path is within deployment directory
        let canonical_path = full_path.canonicalize().map_err(|_| {
            ForgeBaseError::NotFound("File not found".to_string())
        })?;
        
        if !canonical_path.starts_with(&deployment_path) {
            return Err(ForgeBaseError::Authorization(
                "Access denied".to_string(),
            ));
        }

        // If path is a directory, try to serve index.html
        let file_to_serve = if canonical_path.is_dir() {
            canonical_path.join("index.html")
        } else {
            canonical_path
        };

        if !file_to_serve.exists() {
            return Err(ForgeBaseError::NotFound("File not found".to_string()));
        }

        // Read file
        let content = tokio::fs::read(&file_to_serve).await.map_err(|e| {
            ForgeBaseError::Internal(format!("Failed to read file: {}", e))
        })?;

        // Determine content type
        let content_type = mime_guess::from_path(&file_to_serve)
            .first_or_octet_stream()
            .to_string();

        Ok((content, content_type))
    }
}
