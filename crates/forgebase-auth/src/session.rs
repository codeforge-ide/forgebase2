use crate::{models::Session, User};
use chrono::{DateTime, Duration, Utc};
use forgebase_core::{ForgeBaseError, Result};
use uuid::Uuid;

/// Session manager for refresh tokens
pub struct SessionManager {
    refresh_token_expiration_days: i64,
}

impl SessionManager {
    pub fn new(refresh_token_expiration_days: i64) -> Self {
        Self {
            refresh_token_expiration_days,
        }
    }

    /// Generate a refresh token
    pub fn generate_refresh_token(&self) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        
        (0..64)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Calculate expiration time for a refresh token
    pub fn calculate_expiration(&self) -> DateTime<Utc> {
        Utc::now() + Duration::days(self.refresh_token_expiration_days)
    }

    /// Create a new session
    pub fn create_session(
        &self,
        user_id: Uuid,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Session {
        Session {
            id: Uuid::new_v4(),
            user_id,
            refresh_token: self.generate_refresh_token(),
            user_agent,
            ip_address,
            expires_at: self.calculate_expiration(),
            created_at: Utc::now(),
        }
    }

    /// Validate if a session is still valid
    pub fn is_session_valid(&self, session: &Session) -> bool {
        Utc::now() < session.expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_refresh_token() {
        let manager = SessionManager::new(30);
        let token1 = manager.generate_refresh_token();
        let token2 = manager.generate_refresh_token();

        assert_eq!(token1.len(), 64);
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_session_validity() {
        let manager = SessionManager::new(30);
        let user_id = Uuid::new_v4();
        let session = manager.create_session(user_id, None, None);

        assert!(manager.is_session_valid(&session));
    }
}
