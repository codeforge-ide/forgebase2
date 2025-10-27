//! Storage service implementation

use crate::models::*;
use forgebase_core::{ForgeBaseError, Result};
use futures::stream::StreamExt;
use object_store::{aws::AmazonS3Builder, local::LocalFileSystem, ObjectStore};
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

/// Storage backend type
#[derive(Debug, Clone)]
pub enum StorageBackend {
    Local(PathBuf),
    S3 {
        bucket: String,
        region: String,
        access_key: String,
        secret_key: String,
        endpoint: Option<String>,
    },
}

/// Storage service
pub struct StorageService {
    backend: Arc<dyn ObjectStore>,
    backend_type: StorageBackend,
}

impl StorageService {
    /// Create a new storage service
    pub async fn new(backend: StorageBackend) -> Result<Self> {
        let store: Arc<dyn ObjectStore> = match &backend {
            StorageBackend::Local(path) => {
                tokio::fs::create_dir_all(path)
                    .await
                    .map_err(|e| ForgeBaseError::Internal(format!("Failed to create storage directory: {}", e)))?;
                Arc::new(LocalFileSystem::new_with_prefix(path)
                    .map_err(|e| ForgeBaseError::Internal(format!("Failed to initialize local storage: {}", e)))?)
            }
            StorageBackend::S3 {
                bucket,
                region,
                access_key,
                secret_key,
                endpoint,
            } => {
                let mut builder = AmazonS3Builder::new()
                    .with_bucket_name(bucket)
                    .with_region(region)
                    .with_access_key_id(access_key)
                    .with_secret_access_key(secret_key);

                if let Some(endpoint) = endpoint {
                    builder = builder.with_endpoint(endpoint);
                }

                Arc::new(
                    builder
                        .build()
                        .map_err(|e| ForgeBaseError::Internal(format!("Failed to initialize S3 storage: {}", e)))?,
                )
            }
        };

        Ok(Self {
            backend: store,
            backend_type: backend,
        })
    }

    /// Upload a file
    pub async fn upload_file(
        &self,
        bucket: &str,
        key: &str,
        data: bytes::Bytes,
        metadata: UploadMetadata,
    ) -> Result<StorageObject> {
        let path = object_store::path::Path::from(format!("{}/{}", bucket, key));

        self.backend
            .put(&path, data.clone())
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to upload file: {}", e)))?;

        Ok(StorageObject {
            id: Uuid::new_v4(),
            bucket: bucket.to_string(),
            key: key.to_string(),
            size: data.len() as i64,
            content_type: metadata.content_type,
            etag: None,
            metadata: metadata.custom_metadata,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    /// Download a file
    pub async fn download_file(&self, bucket: &str, key: &str) -> Result<bytes::Bytes> {
        let path = object_store::path::Path::from(format!("{}/{}", bucket, key));

        let result = self
            .backend
            .get(&path)
            .await
            .map_err(|e| ForgeBaseError::NotFound(format!("File not found: {}", e)))?;

        let bytes = result
            .bytes()
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to read file: {}", e)))?;

        Ok(bytes)
    }

    /// Delete a file
    pub async fn delete_file(&self, bucket: &str, key: &str) -> Result<()> {
        let path = object_store::path::Path::from(format!("{}/{}", bucket, key));

        self.backend
            .delete(&path)
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to delete file: {}", e)))?;

        Ok(())
    }

    /// List files in a bucket
    pub async fn list_files(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<String>> {
        let prefix_path = if let Some(prefix) = prefix {
            format!("{}/{}", bucket, prefix)
        } else {
            bucket.to_string()
        };

        let path = object_store::path::Path::from(prefix_path);

        let mut list_stream = self.backend.list(Some(&path));
        
        let mut files = Vec::new();
        while let Some(result) = StreamExt::next(&mut list_stream).await {
            match result {
                Ok(meta) => files.push(meta.location.to_string()),
                Err(e) => return Err(ForgeBaseError::Internal(format!("Failed to list files: {}", e))),
            }
        }

        Ok(files)
    }

    /// Get file metadata
    pub async fn get_file_metadata(&self, bucket: &str, key: &str) -> Result<FileMetadata> {
        let path = object_store::path::Path::from(format!("{}/{}", bucket, key));

        let meta = self
            .backend
            .head(&path)
            .await
            .map_err(|e| ForgeBaseError::NotFound(format!("File not found: {}", e)))?;

        Ok(FileMetadata {
            size: meta.size as i64,
            last_modified: chrono::DateTime::from_timestamp(meta.last_modified.timestamp(), 0)
                .unwrap_or_else(chrono::Utc::now),
            etag: meta.e_tag,
        })
    }

    /// Copy a file
    pub async fn copy_file(
        &self,
        source_bucket: &str,
        source_key: &str,
        dest_bucket: &str,
        dest_key: &str,
    ) -> Result<()> {
        let source_path = object_store::path::Path::from(format!("{}/{}", source_bucket, source_key));
        let dest_path = object_store::path::Path::from(format!("{}/{}", dest_bucket, dest_key));

        let data = self
            .backend
            .get(&source_path)
            .await
            .map_err(|e| ForgeBaseError::NotFound(format!("Source file not found: {}", e)))?;

        let bytes = data
            .bytes()
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to read source file: {}", e)))?;

        self.backend
            .put(&dest_path, bytes)
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to copy file: {}", e)))?;

        Ok(())
    }

    /// Generate a presigned URL for file upload
    pub async fn generate_upload_url(
        &self,
        bucket: &str,
        key: &str,
        _expires_in: i64,
    ) -> Result<String> {
        // This is a simplified implementation
        // In production, you'd generate a real presigned URL with proper signing
        let url = match &self.backend_type {
            StorageBackend::Local(_) => {
                format!("/storage/{}/{}", bucket, key)
            }
            StorageBackend::S3 { .. } => {
                // Would use AWS SDK to generate presigned URL
                format!("https://s3.amazonaws.com/{}/{}", bucket, key)
            }
        };

        Ok(url)
    }

    /// Generate a presigned URL for file download
    pub async fn generate_download_url(
        &self,
        bucket: &str,
        key: &str,
        expires_in: i64,
    ) -> Result<String> {
        self.generate_upload_url(bucket, key, expires_in).await
    }
}
