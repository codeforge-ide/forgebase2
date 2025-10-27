use chrono::{Duration, Utc};
use forgebase_core::{ForgeBaseError, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub email: String,      // User email
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub role: Option<String>, // User role
    pub permissions: Vec<String>, // User permissions
}

impl Claims {
    pub fn new(user_id: Uuid, email: String, expiration_seconds: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::seconds(expiration_seconds);

        Self {
            sub: user_id.to_string(),
            email,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            role: None,
            permissions: Vec::new(),
        }
    }

    pub fn with_role(mut self, role: String) -> Self {
        self.role = Some(role);
        self
    }

    pub fn with_permissions(mut self, permissions: Vec<String>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub)
            .map_err(|_| ForgeBaseError::Auth("Invalid user ID in token".to_string()))
    }
}

/// JWT token manager
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtManager {
    pub fn new(secret: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 60; // 60 seconds leeway for clock skew

        Self {
            encoding_key,
            decoding_key,
            validation,
        }
    }

    /// Generate an access token
    pub fn generate_access_token(&self, claims: Claims) -> Result<String> {
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to generate token: {}", e)))
    }

    /// Verify and decode a token
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    ForgeBaseError::Auth("Token has expired".to_string())
                }
                _ => ForgeBaseError::Auth(format!("Invalid token: {}", e)),
            })
    }

    /// Extract token from Authorization header
    pub fn extract_token_from_header(auth_header: &str) -> Result<&str> {
        if !auth_header.starts_with("Bearer ") {
            return Err(ForgeBaseError::Auth(
                "Invalid authorization header format".to_string(),
            ));
        }

        Ok(&auth_header[7..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_verification() {
        let manager = JwtManager::new("test-secret-key-123");
        let user_id = Uuid::new_v4();
        let claims = Claims::new(user_id, "test@example.com".to_string(), 3600);

        let token = manager.generate_access_token(claims.clone()).unwrap();
        let decoded = manager.verify_token(&token).unwrap();

        assert_eq!(decoded.sub, user_id.to_string());
        assert_eq!(decoded.email, "test@example.com");
    }

    #[test]
    fn test_extract_token_from_header() {
        let token = "abc123";
        let header = format!("Bearer {}", token);
        let extracted = JwtManager::extract_token_from_header(&header).unwrap();
        assert_eq!(extracted, token);

        let invalid_header = "Invalid abc123";
        assert!(JwtManager::extract_token_from_header(invalid_header).is_err());
    }
}
