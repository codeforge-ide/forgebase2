//! File upload handling

use forgebase_core::{ForgeBaseError, Result};
use bytes::Bytes;

/// Upload validator
pub struct UploadValidator {
    max_file_size: Option<i64>,
    allowed_mime_types: Option<Vec<String>>,
}

impl UploadValidator {
    pub fn new(max_file_size: Option<i64>, allowed_mime_types: Option<Vec<String>>) -> Self {
        Self {
            max_file_size,
            allowed_mime_types,
        }
    }

    /// Validate upload
    pub fn validate(&self, data: &Bytes, content_type: Option<&str>) -> Result<()> {
        // Check file size
        if let Some(max_size) = self.max_file_size {
            if data.len() as i64 > max_size {
                return Err(ForgeBaseError::Validation(format!(
                    "File size {} exceeds maximum allowed size {}",
                    data.len(),
                    max_size
                )));
            }
        }

        // Check MIME type
        if let Some(ref allowed_types) = self.allowed_mime_types {
            if let Some(content_type) = content_type {
                if !allowed_types.iter().any(|t| {
                    if t.ends_with("/*") {
                        let prefix = &t[..t.len() - 2];
                        content_type.starts_with(prefix)
                    } else {
                        t == content_type
                    }
                }) {
                    return Err(ForgeBaseError::Validation(format!(
                        "Content type '{}' is not allowed",
                        content_type
                    )));
                }
            }
        }

        Ok(())
    }
}

/// Generate a unique file key
pub fn generate_file_key(original_name: &str) -> String {
    let uuid = uuid::Uuid::new_v4();
    let extension = std::path::Path::new(original_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");

    format!("{}.{}", uuid, extension)
}

/// Parse content type from file extension
pub fn parse_content_type(filename: &str) -> String {
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match extension {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "pdf" => "application/pdf",
        "json" => "application/json",
        "xml" => "application/xml",
        "txt" => "text/plain",
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        _ => "application/octet-stream",
    }
    .to_string()
}

/// Upload progress tracker
pub struct UploadProgress {
    pub total_bytes: usize,
    pub uploaded_bytes: usize,
    pub percentage: f64,
}

impl UploadProgress {
    pub fn new(total_bytes: usize) -> Self {
        Self {
            total_bytes,
            uploaded_bytes: 0,
            percentage: 0.0,
        }
    }

    pub fn update(&mut self, bytes: usize) {
        self.uploaded_bytes += bytes;
        self.percentage = (self.uploaded_bytes as f64 / self.total_bytes as f64) * 100.0;
    }

    pub fn is_complete(&self) -> bool {
        self.uploaded_bytes >= self.total_bytes
    }
}
