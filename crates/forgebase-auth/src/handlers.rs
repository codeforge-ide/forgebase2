use crate::{
    middleware::{extract_claims, AuthState},
    models::*,
    AuthService,
};
use axum::{
    extract::{Request, State, Extension},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use forgebase_core::{ApiResponse, ErrorResponse, ForgeBaseError};
use std::sync::Arc;
use validator::Validate;

/// Create authentication routes
pub fn create_auth_routes() -> Router<AuthState> {
    Router::new()
        .route("/auth/signup", post(sign_up_handler))
        .route("/auth/signin", post(sign_in_handler))
        .route("/auth/signout", post(sign_out_handler))
        .route("/auth/refresh", post(refresh_token_handler))
        .route("/auth/user", get(get_user_handler))
        .route("/auth/user", post(update_profile_handler))
        .route("/auth/password/change", post(change_password_handler))
        .route("/auth/password/reset", post(request_password_reset_handler))
        .route("/auth/password/update", post(reset_password_handler))
        .route("/auth/verify", post(verify_email_handler))
}

/// Sign up handler
async fn sign_up_handler(
    State(state): State<AuthState>,
    Json(payload): Json<SignUpRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, ApiError> {
    // Extract user agent and IP from request
    // Note: In real implementation, would need proper request extraction
    let user_agent = None;
    let ip_address = None;

    let response = state
        .service
        .sign_up(payload, user_agent, ip_address)
        .await?;

    Ok(Json(ApiResponse::success(response)))
}

/// Sign in handler
async fn sign_in_handler(
    State(state): State<AuthState>,
    Json(payload): Json<SignInRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, ApiError> {
    payload.validate()?;

    let user_agent = None;
    let ip_address = None;

    let response = state
        .service
        .sign_in(payload, user_agent, ip_address)
        .await?;

    Ok(Json(ApiResponse::success(response)))
}

/// Refresh token handler
async fn refresh_token_handler(
    State(state): State<AuthState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, ApiError> {
    let response = state.service.refresh_token(&payload.refresh_token).await?;
    Ok(Json(ApiResponse::success(response)))
}

/// Sign out handler
async fn sign_out_handler(
    State(state): State<AuthState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state.service.sign_out(&payload.refresh_token).await?;
    Ok(Json(ApiResponse::success(())))
}

/// Get user handler
async fn get_user_handler(
    State(state): State<AuthState>,
    Extension(claims): Extension<crate::jwt::Claims>,
) -> Result<Json<ApiResponse<UserProfile>>, ApiError> {
    let user_id = claims.user_id()?;
    let user = state.service.get_user(user_id).await?;

    Ok(Json(ApiResponse::success(user)))
}

/// Update profile handler
async fn update_profile_handler(
    State(state): State<AuthState>,
    Extension(claims): Extension<crate::jwt::Claims>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<ApiResponse<UserProfile>>, ApiError> {
    let user_id = claims.user_id()?;
    let user = state.service.update_profile(user_id, payload).await?;

    Ok(Json(ApiResponse::success(user)))
}

/// Change password handler
async fn change_password_handler(
    State(state): State<AuthState>,
    Extension(claims): Extension<crate::jwt::Claims>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    payload.validate()?;

    let user_id = claims.user_id()?;
    state.service.change_password(user_id, payload).await?;

    Ok(Json(ApiResponse::success(())))
}

/// Request password reset handler
async fn request_password_reset_handler(
    State(state): State<AuthState>,
    Json(payload): Json<PasswordResetRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    payload.validate()?;

    // Always return success even if email doesn't exist (security)
    let _ = state.service.request_password_reset(&payload.email).await;

    Ok(Json(ApiResponse::success(())))
}

/// Reset password handler
async fn reset_password_handler(
    State(state): State<AuthState>,
    Json(payload): Json<PasswordUpdateRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    payload.validate()?;
    state.service.reset_password(payload).await?;
    Ok(Json(ApiResponse::success(())))
}

/// Verify email handler
async fn verify_email_handler(
    State(state): State<AuthState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let token = payload["token"]
        .as_str()
        .ok_or_else(|| ForgeBaseError::InvalidInput("Missing token".to_string()))?;

    state.service.verify_email(token).await?;
    Ok(Json(ApiResponse::success(())))
}

/// API error wrapper
pub struct ApiError(ForgeBaseError);

impl From<ForgeBaseError> for ApiError {
    fn from(error: ForgeBaseError) -> Self {
        ApiError(error)
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(errors: validator::ValidationErrors) -> Self {
        ApiError(ForgeBaseError::Validation(errors.to_string()))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.0.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = serde_json::to_string(&ErrorResponse::from_error(&self.0))
            .unwrap_or_else(|_| "{}".to_string());

        (status, body).into_response()
    }
}
