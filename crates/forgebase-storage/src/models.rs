//! Storage models and types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Storage object metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageObject {
    pub id: Uuid,
    pub bucket: String,
    pub key: String,
    pub size: i64,
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Upload metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadMetadata {
    pub content_type: Option<String>,
    pub custom_metadata: HashMap<String, String>,
}

/// File metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub size: i64,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub etag: Option<String>,
}

/// Upload request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRequest {
    pub bucket: String,
    pub key: String,
    pub content_type: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Upload response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub object: StorageObject,
    pub url: String,
}

/// Download request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub bucket: String,
    pub key: String,
}

/// Presigned URL request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedUrlRequest {
    pub bucket: String,
    pub key: String,
    pub expires_in: i64,
    pub operation: PresignedUrlOperation,
}

/// Presigned URL operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PresignedUrlOperation {
    Upload,
    Download,
}

/// Presigned URL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedUrlResponse {
    pub url: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// List files request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesRequest {
    pub bucket: String,
    pub prefix: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// List files response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFilesResponse {
    pub files: Vec<StorageObject>,
    pub total: i32,
    pub has_more: bool,
}

/// Delete request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequest {
    pub bucket: String,
    pub key: String,
}

/// Copy request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyRequest {
    pub source_bucket: String,
    pub source_key: String,
    pub dest_bucket: String,
    pub dest_key: String,
}
