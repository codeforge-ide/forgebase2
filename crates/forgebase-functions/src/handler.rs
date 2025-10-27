//! HTTP handlers for function operations

use crate::executor::FunctionExecutor;
use crate::models::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct FunctionState {
    pub executor: Arc<FunctionExecutor>,
}

/// Invoke function handler
pub async fn invoke_function_handler(
    State(state): State<FunctionState>,
    Path(function_id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let request = InvocationRequest {
        function_id,
        payload,
        headers: std::collections::HashMap::new(),
        query_params: std::collections::HashMap::new(),
    };

    match state.executor.execute(request).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
