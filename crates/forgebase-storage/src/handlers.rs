//! HTTP handlers for storage operations

use crate::models::*;
use crate::service::StorageService;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct StorageState {
    pub service: Arc<StorageService>,
}

/// Upload file handler
pub async fn upload_file_handler(
    State(state): State<StorageState>,
    Json(request): Json<UploadRequest>,
    body: bytes::Bytes,
) -> impl IntoResponse {
    let metadata = UploadMetadata {
        content_type: request.content_type,
        custom_metadata: request.metadata.unwrap_or_default(),
    };

    match state
        .service
        .upload_file(&request.bucket, &request.key, body, metadata)
        .await
    {
        Ok(object) => {
            let url = format!("/storage/{}/{}", object.bucket, object.key);
            let response = UploadResponse { object, url };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Download file handler
pub async fn download_file_handler(
    State(state): State<StorageState>,
    Path((bucket, key)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.service.download_file(&bucket, &key).await {
        Ok(data) => (StatusCode::OK, data).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

/// Delete file handler
pub async fn delete_file_handler(
    State(state): State<StorageState>,
    Path((bucket, key)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.service.delete_file(&bucket, &key).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// List files handler
#[derive(Deserialize)]
pub struct ListQuery {
    prefix: Option<String>,
}

pub async fn list_files_handler(
    State(state): State<StorageState>,
    Path(bucket): Path<String>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    match state
        .service
        .list_files(&bucket, query.prefix.as_deref())
        .await
    {
        Ok(files) => (StatusCode::OK, Json(files)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Generate presigned URL handler
pub async fn presigned_url_handler(
    State(state): State<StorageState>,
    Json(request): Json<PresignedUrlRequest>,
) -> impl IntoResponse {
    let result = match request.operation {
        PresignedUrlOperation::Upload => {
            state
                .service
                .generate_upload_url(&request.bucket, &request.key, request.expires_in)
                .await
        }
        PresignedUrlOperation::Download => {
            state
                .service
                .generate_download_url(&request.bucket, &request.key, request.expires_in)
                .await
        }
    };

    match result {
        Ok(url) => {
            let expires_at =
                chrono::Utc::now() + chrono::Duration::seconds(request.expires_in);
            let response = PresignedUrlResponse { url, expires_at };
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
