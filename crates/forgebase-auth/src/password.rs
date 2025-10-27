use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use forgebase_core::ForgeBaseError;

/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> Result<String, ForgeBaseError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| ForgeBaseError::Internal(format!("Failed to hash password: {}", e)))
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, ForgeBaseError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| ForgeBaseError::Internal(format!("Failed to parse hash: {}", e)))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Validate password strength
pub fn validate_password_strength(password: &str) -> Result<(), ForgeBaseError> {
    if password.len() < 8 {
        return Err(ForgeBaseError::Validation(
            "Password must be at least 8 characters long".to_string(),
        ));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(ForgeBaseError::Validation(
            "Password must contain uppercase, lowercase, and digit characters".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "TestPassword123";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_validate_password_strength() {
        assert!(validate_password_strength("TestPass123").is_ok());
        assert!(validate_password_strength("short").is_err());
        assert!(validate_password_strength("nouppercase123").is_err());
        assert!(validate_password_strength("NOLOWERCASE123").is_err());
        assert!(validate_password_strength("NoDigitsHere").is_err());
    }
}
