use crate::models::*;
use chrono::{DateTime, Utc};
use forgebase_core::{ForgeBaseError, Result};
use sqlx::PgPool;
use uuid::Uuid;

/// User repository for database operations
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new user
    pub async fn create(&self, user: &User) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                id, email, email_verified, phone, phone_verified, password_hash,
                full_name, avatar_url, metadata, is_anonymous, is_active,
                last_sign_in_at, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(user.email_verified)
        .bind(&user.phone)
        .bind(user.phone_verified)
        .bind(&user.password_hash)
        .bind(&user.full_name)
        .bind(&user.avatar_url)
        .bind(&user.metadata)
        .bind(user.is_anonymous)
        .bind(user.is_active)
        .bind(user.last_sign_in_at)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(format!("Failed to create user: {}", e)))?;

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(format!("Failed to find user: {}", e)))?;

        Ok(user)
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(format!("Failed to find user: {}", e)))?;

        Ok(user)
    }

    /// Update user
    pub async fn update(&self, user: &User) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET
                email = $2,
                email_verified = $3,
                phone = $4,
                phone_verified = $5,
                password_hash = $6,
                full_name = $7,
                avatar_url = $8,
                metadata = $9,
                is_active = $10,
                last_sign_in_at = $11,
                updated_at = $12
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(user.email_verified)
        .bind(&user.phone)
        .bind(user.phone_verified)
        .bind(&user.password_hash)
        .bind(&user.full_name)
        .bind(&user.avatar_url)
        .bind(&user.metadata)
        .bind(user.is_active)
        .bind(user.last_sign_in_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(format!("Failed to update user: {}", e)))?;

        Ok(user)
    }

    /// Delete user
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(format!("Failed to delete user: {}", e)))?;

        Ok(())
    }

    /// Update last sign in time
    pub async fn update_last_sign_in(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE users SET last_sign_in_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(format!("Failed to update last sign in: {}", e)))?;

        Ok(())
    }
}

/// Session repository
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new session
    pub async fn create(&self, session: &Session) -> Result<Session> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (
                id, user_id, refresh_token, user_agent, ip_address, expires_at, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(session.id)
        .bind(session.user_id)
        .bind(&session.refresh_token)
        .bind(&session.user_agent)
        .bind(&session.ip_address)
        .bind(session.expires_at)
        .bind(session.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(format!("Failed to create session: {}", e)))?;

        Ok(session)
    }

    /// Find session by refresh token
    pub async fn find_by_refresh_token(&self, refresh_token: &str) -> Result<Option<Session>> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE refresh_token = $1 AND expires_at > NOW()",
        )
        .bind(refresh_token)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(format!("Failed to find session: {}", e)))?;

        Ok(session)
    }

    /// Delete session
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(format!("Failed to delete session: {}", e)))?;

        Ok(())
    }

    /// Delete all sessions for a user
    pub async fn delete_all_for_user(&self, user_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM sessions WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                ForgeBaseError::Database(format!("Failed to delete user sessions: {}", e))
            })?;

        Ok(())
    }

    /// Delete expired sessions
    pub async fn delete_expired(&self) -> Result<u64> {
        let result = sqlx::query("DELETE FROM sessions WHERE expires_at < NOW()")
            .execute(&self.pool)
            .await
            .map_err(|e| {
                ForgeBaseError::Database(format!("Failed to delete expired sessions: {}", e))
            })?;

        Ok(result.rows_affected())
    }
}

/// Verification token repository
pub struct VerificationTokenRepository {
    pool: PgPool,
}

impl VerificationTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a verification token
    pub async fn create(&self, token: &VerificationToken) -> Result<VerificationToken> {
        let token = sqlx::query_as::<_, VerificationToken>(
            r#"
            INSERT INTO verification_tokens (
                id, user_id, token, token_type, expires_at, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(token.id)
        .bind(token.user_id)
        .bind(&token.token)
        .bind(&token.token_type)
        .bind(token.expires_at)
        .bind(token.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            ForgeBaseError::Database(format!("Failed to create verification token: {}", e))
        })?;

        Ok(token)
    }

    /// Find token by value and type
    pub async fn find_valid_token(
        &self,
        token: &str,
        token_type: &str,
    ) -> Result<Option<VerificationToken>> {
        let token = sqlx::query_as::<_, VerificationToken>(
            "SELECT * FROM verification_tokens WHERE token = $1 AND token_type = $2 AND expires_at > NOW()",
        )
        .bind(token)
        .bind(token_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ForgeBaseError::Database(format!("Failed to find token: {}", e)))?;

        Ok(token)
    }

    /// Delete token
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM verification_tokens WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(format!("Failed to delete token: {}", e)))?;

        Ok(())
    }

    /// Delete all tokens for a user
    pub async fn delete_all_for_user(&self, user_id: Uuid, token_type: &str) -> Result<()> {
        sqlx::query("DELETE FROM verification_tokens WHERE user_id = $1 AND token_type = $2")
            .bind(user_id)
            .bind(token_type)
            .execute(&self.pool)
            .await
            .map_err(|e| ForgeBaseError::Database(format!("Failed to delete tokens: {}", e)))?;

        Ok(())
    }
}
