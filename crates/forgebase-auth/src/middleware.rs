use crate::{jwt::Claims, jwt::JwtManager, AuthService};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use forgebase_core::{ErrorResponse, ForgeBaseError};
use std::sync::Arc;

/// Authentication state shared across handlers
#[derive(Clone)]
pub struct AuthState {
    pub service: Arc<AuthService>,
    pub jwt_manager: Arc<JwtManager>,
}

/// Middleware to require authentication
pub async fn require_auth(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::MissingToken)?;

    let token = JwtManager::extract_token_from_header(auth_header)
        .map_err(|_| AuthError::InvalidToken)?;

    let claims = state
        .jwt_manager
        .verify_token(token)
        .map_err(|_| AuthError::InvalidToken)?;

    // Add claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Optional authentication middleware
pub async fn optional_auth(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(auth_header) = request.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Ok(token) = JwtManager::extract_token_from_header(auth_str) {
                if let Ok(claims) = state.jwt_manager.verify_token(token) {
                    request.extensions_mut().insert(claims);
                }
            }
        }
    }

    next.run(request).await
}

/// Extract claims from request
pub fn extract_claims(request: &Request) -> Option<Claims> {
    request.extensions().get::<Claims>().cloned()
}

/// Authentication errors
#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    Internal(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            AuthError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                ForgeBaseError::Auth("Missing authentication token".to_string()),
            ),
            AuthError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                ForgeBaseError::Auth("Invalid or expired token".to_string()),
            ),
            AuthError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ForgeBaseError::Internal(msg),
            ),
        };

        let body = serde_json::to_string(&ErrorResponse::from_error(&error))
            .unwrap_or_else(|_| "{}".to_string());

        (status, body).into_response()
    }
}

/// Rate limiting middleware (simplified version)
pub async fn rate_limit(request: Request, next: Next) -> Response {
    // TODO: Implement proper rate limiting with Redis or in-memory store
    // For now, just pass through
    next.run(request).await
}
