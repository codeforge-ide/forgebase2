use crate::{
    jwt::{Claims, JwtManager},
    models::*,
    password::{hash_password, verify_password, validate_password_strength},
    repository::{SessionRepository, UserRepository, VerificationTokenRepository},
    session::SessionManager,
};
use chrono::Utc;
use forgebase_core::{ForgeBaseError, Result};
use sqlx::PgPool;
use uuid::Uuid;

/// Authentication service
pub struct AuthService {
    user_repo: UserRepository,
    session_repo: SessionRepository,
    token_repo: VerificationTokenRepository,
    jwt_manager: JwtManager,
    session_manager: SessionManager,
    jwt_expiration: i64,
}

impl AuthService {
    pub fn new(
        pool: PgPool,
        jwt_secret: String,
        jwt_expiration: i64,
        refresh_token_expiration_days: i64,
    ) -> Self {
        Self {
            user_repo: UserRepository::new(pool.clone()),
            session_repo: SessionRepository::new(pool.clone()),
            token_repo: VerificationTokenRepository::new(pool),
            jwt_manager: JwtManager::new(&jwt_secret),
            session_manager: SessionManager::new(refresh_token_expiration_days),
            jwt_expiration,
        }
    }

    /// Sign up a new user
    pub async fn sign_up(
        &self,
        request: SignUpRequest,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<AuthResponse> {
        // Validate password strength
        validate_password_strength(&request.password)?;

        // Check if user already exists
        if let Some(_) = self.user_repo.find_by_email(&request.email).await? {
            return Err(ForgeBaseError::Conflict(
                "User with this email already exists".to_string(),
            ));
        }

        // Hash password
        let password_hash = hash_password(&request.password)?;

        // Create user
        let user = User {
            id: Uuid::new_v4(),
            email: request.email.clone(),
            email_verified: false,
            phone: None,
            phone_verified: false,
            password_hash: Some(password_hash),
            full_name: request.full_name,
            avatar_url: None,
            metadata: request.metadata.unwrap_or(serde_json::json!({})),
            is_anonymous: false,
            is_active: true,
            last_sign_in_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_user = self.user_repo.create(&user).await?;

        // Create session
        let session = self
            .session_manager
            .create_session(created_user.id, user_agent, ip_address);
        let created_session = self.session_repo.create(&session).await?;

        // Generate tokens
        let claims = Claims::new(created_user.id, created_user.email.clone(), self.jwt_expiration);
        let access_token = self.jwt_manager.generate_access_token(claims)?;

        Ok(AuthResponse {
            user: created_user.into(),
            access_token,
            refresh_token: created_session.refresh_token,
            expires_in: self.jwt_expiration,
        })
    }

    /// Sign in with email and password
    pub async fn sign_in(
        &self,
        request: SignInRequest,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<AuthResponse> {
        // Find user
        let user = self
            .user_repo
            .find_by_email(&request.email)
            .await?
            .ok_or_else(|| ForgeBaseError::Auth("Invalid credentials".to_string()))?;

        // Verify user is active
        if !user.is_active {
            return Err(ForgeBaseError::Auth("Account is disabled".to_string()));
        }

        // Verify password
        let password_hash = user
            .password_hash
            .as_ref()
            .ok_or_else(|| ForgeBaseError::Auth("Password not set".to_string()))?;

        if !verify_password(&request.password, password_hash)? {
            return Err(ForgeBaseError::Auth("Invalid credentials".to_string()));
        }

        // Update last sign in
        self.user_repo.update_last_sign_in(user.id).await?;

        // Create session
        let session = self
            .session_manager
            .create_session(user.id, user_agent, ip_address);
        let created_session = self.session_repo.create(&session).await?;

        // Generate tokens
        let claims = Claims::new(user.id, user.email.clone(), self.jwt_expiration);
        let access_token = self.jwt_manager.generate_access_token(claims)?;

        Ok(AuthResponse {
            user: user.into(),
            access_token,
            refresh_token: created_session.refresh_token,
            expires_in: self.jwt_expiration,
        })
    }

    /// Refresh access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthResponse> {
        // Find session
        let session = self
            .session_repo
            .find_by_refresh_token(refresh_token)
            .await?
            .ok_or_else(|| ForgeBaseError::Auth("Invalid refresh token".to_string()))?;

        // Validate session
        if !self.session_manager.is_session_valid(&session) {
            return Err(ForgeBaseError::Auth("Session expired".to_string()));
        }

        // Find user
        let user = self
            .user_repo
            .find_by_id(session.user_id)
            .await?
            .ok_or_else(|| ForgeBaseError::Auth("User not found".to_string()))?;

        // Verify user is active
        if !user.is_active {
            return Err(ForgeBaseError::Auth("Account is disabled".to_string()));
        }

        // Generate new access token
        let claims = Claims::new(user.id, user.email.clone(), self.jwt_expiration);
        let access_token = self.jwt_manager.generate_access_token(claims)?;

        Ok(AuthResponse {
            user: user.into(),
            access_token,
            refresh_token: session.refresh_token,
            expires_in: self.jwt_expiration,
        })
    }

    /// Sign out (invalidate session)
    pub async fn sign_out(&self, refresh_token: &str) -> Result<()> {
        if let Some(session) = self
            .session_repo
            .find_by_refresh_token(refresh_token)
            .await?
        {
            self.session_repo.delete(session.id).await?;
        }
        Ok(())
    }

    /// Verify access token and get claims
    pub fn verify_access_token(&self, token: &str) -> Result<Claims> {
        self.jwt_manager.verify_token(token)
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: Uuid) -> Result<UserProfile> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| ForgeBaseError::NotFound("User not found".to_string()))?;

        Ok(user.into())
    }

    /// Update user profile
    pub async fn update_profile(
        &self,
        user_id: Uuid,
        request: UpdateProfileRequest,
    ) -> Result<UserProfile> {
        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| ForgeBaseError::NotFound("User not found".to_string()))?;

        if let Some(full_name) = request.full_name {
            user.full_name = Some(full_name);
        }

        if let Some(avatar_url) = request.avatar_url {
            user.avatar_url = Some(avatar_url);
        }

        if let Some(metadata) = request.metadata {
            user.metadata = metadata;
        }

        user.updated_at = Utc::now();
        let updated_user = self.user_repo.update(&user).await?;

        Ok(updated_user.into())
    }

    /// Change password
    pub async fn change_password(
        &self,
        user_id: Uuid,
        request: ChangePasswordRequest,
    ) -> Result<()> {
        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| ForgeBaseError::NotFound("User not found".to_string()))?;

        // Verify current password
        let current_hash = user
            .password_hash
            .as_ref()
            .ok_or_else(|| ForgeBaseError::Auth("Password not set".to_string()))?;

        if !verify_password(&request.current_password, current_hash)? {
            return Err(ForgeBaseError::Auth("Invalid current password".to_string()));
        }

        // Validate new password
        validate_password_strength(&request.new_password)?;

        // Hash new password
        let new_hash = hash_password(&request.new_password)?;
        user.password_hash = Some(new_hash);
        user.updated_at = Utc::now();

        self.user_repo.update(&user).await?;

        // Invalidate all sessions
        self.session_repo.delete_all_for_user(user_id).await?;

        Ok(())
    }

    /// Create email verification token
    pub async fn create_email_verification_token(&self, user_id: Uuid) -> Result<String> {
        use chrono::Duration;
        use rand::Rng;

        let token: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let verification_token = VerificationToken {
            id: Uuid::new_v4(),
            user_id,
            token: token.clone(),
            token_type: "email_verification".to_string(),
            expires_at: Utc::now() + Duration::hours(24),
            created_at: Utc::now(),
        };

        self.token_repo.create(&verification_token).await?;

        Ok(token)
    }

    /// Verify email with token
    pub async fn verify_email(&self, token: &str) -> Result<()> {
        let verification_token = self
            .token_repo
            .find_valid_token(token, "email_verification")
            .await?
            .ok_or_else(|| ForgeBaseError::Auth("Invalid or expired token".to_string()))?;

        let mut user = self
            .user_repo
            .find_by_id(verification_token.user_id)
            .await?
            .ok_or_else(|| ForgeBaseError::NotFound("User not found".to_string()))?;

        user.email_verified = true;
        user.updated_at = Utc::now();

        self.user_repo.update(&user).await?;
        self.token_repo.delete(verification_token.id).await?;

        Ok(())
    }

    /// Request password reset
    pub async fn request_password_reset(&self, email: &str) -> Result<String> {
        use chrono::Duration;
        use rand::Rng;

        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| ForgeBaseError::NotFound("User not found".to_string()))?;

        let token: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let verification_token = VerificationToken {
            id: Uuid::new_v4(),
            user_id: user.id,
            token: token.clone(),
            token_type: "password_reset".to_string(),
            expires_at: Utc::now() + Duration::hours(1),
            created_at: Utc::now(),
        };

        self.token_repo.create(&verification_token).await?;

        Ok(token)
    }

    /// Reset password with token
    pub async fn reset_password(&self, request: PasswordUpdateRequest) -> Result<()> {
        let verification_token = self
            .token_repo
            .find_valid_token(&request.token, "password_reset")
            .await?
            .ok_or_else(|| ForgeBaseError::Auth("Invalid or expired token".to_string()))?;

        let mut user = self
            .user_repo
            .find_by_id(verification_token.user_id)
            .await?
            .ok_or_else(|| ForgeBaseError::NotFound("User not found".to_string()))?;

        // Validate new password
        validate_password_strength(&request.new_password)?;

        // Hash new password
        let new_hash = hash_password(&request.new_password)?;
        user.password_hash = Some(new_hash);
        user.updated_at = Utc::now();

        self.user_repo.update(&user).await?;
        self.token_repo.delete(verification_token.id).await?;

        // Invalidate all sessions
        self.session_repo.delete_all_for_user(user.id).await?;

        Ok(())
    }
}
