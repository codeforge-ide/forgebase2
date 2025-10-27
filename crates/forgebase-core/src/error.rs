use thiserror::Error;

/// Core error types for ForgeBase
#[derive(Error, Debug)]
pub enum ForgeBaseError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Rate limit exceeded")]
    RateLimit,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Network error: {0}")]
    Network(String),
}

impl ForgeBaseError {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::Config(_) | Self::Internal(_) => 500,
            Self::Database(_) => 500,
            Self::Auth(_) => 401,
            Self::Authorization(_) => 403,
            Self::Validation(_) => 400,
            Self::NotFound(_) => 404,
            Self::Conflict(_) => 409,
            Self::ExternalService(_) => 502,
            Self::RateLimit => 429,
            Self::InvalidInput(_) => 400,
            Self::Storage(_) => 500,
            Self::Network(_) => 503,
        }
    }

    pub fn error_code(&self) -> &str {
        match self {
            Self::Config(_) => "CONFIG_ERROR",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Auth(_) => "AUTH_ERROR",
            Self::Authorization(_) => "AUTHORIZATION_ERROR",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Conflict(_) => "CONFLICT",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            Self::RateLimit => "RATE_LIMIT_EXCEEDED",
            Self::InvalidInput(_) => "INVALID_INPUT",
            Self::Storage(_) => "STORAGE_ERROR",
            Self::Network(_) => "NETWORK_ERROR",
        }
    }
}

pub type Result<T> = std::result::Result<T, ForgeBaseError>;

/// HTTP error response
#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn from_error(error: &ForgeBaseError) -> Self {
        Self {
            error: error.error_code().to_string(),
            code: error.error_code().to_string(),
            message: error.to_string(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}
