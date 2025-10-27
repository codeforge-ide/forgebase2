use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// User model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub metadata: serde_json::Value,
    pub is_anonymous: bool,
    pub is_active: bool,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Public user profile (safe to expose)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            email_verified: user.email_verified,
            phone: user.phone,
            phone_verified: user.phone_verified,
            full_name: user.full_name,
            avatar_url: user.avatar_url,
            metadata: user.metadata,
            created_at: user.created_at,
        }
    }
}

/// Sign up request
#[derive(Debug, Deserialize, Validate)]
pub struct SignUpRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub full_name: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Sign in request
#[derive(Debug, Deserialize, Validate)]
pub struct SignInRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

/// Magic link request
#[derive(Debug, Deserialize, Validate)]
pub struct MagicLinkRequest {
    #[validate(email)]
    pub email: String,
    pub redirect_to: Option<String>,
}

/// OAuth sign in request
#[derive(Debug, Deserialize)]
pub struct OAuthSignInRequest {
    pub provider: String,
    pub code: String,
    pub redirect_uri: String,
}

/// Password reset request
#[derive(Debug, Deserialize, Validate)]
pub struct PasswordResetRequest {
    #[validate(email)]
    pub email: String,
}

/// Password update request
#[derive(Debug, Deserialize, Validate)]
pub struct PasswordUpdateRequest {
    pub token: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

/// Change password request (for authenticated users)
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

/// Update profile request
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserProfile,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// Refresh token request
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Session model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// OAuth account model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OAuthAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Email verification token
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct VerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub token_type: String, // email_verification, password_reset, magic_link
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Role model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// User role assignment
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// API Key model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    #[serde(skip_serializing)]
    pub key_hash: String,
    pub prefix: String,
    pub scopes: Vec<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// API Key creation request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub scopes: Vec<String>,
    pub expires_in_days: Option<i64>,
}

/// API Key response (includes the actual key once)
#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub prefix: String,
    pub key: String, // Only returned on creation
    pub scopes: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
