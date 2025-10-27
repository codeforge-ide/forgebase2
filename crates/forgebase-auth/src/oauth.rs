use forgebase_core::{ForgeBaseError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OAuth provider
#[derive(Debug, Clone)]
pub enum OAuthProvider {
    Google,
    GitHub,
    GitLab,
    Discord,
    Microsoft,
    Apple,
}

impl OAuthProvider {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "google" => Ok(Self::Google),
            "github" => Ok(Self::GitHub),
            "gitlab" => Ok(Self::GitLab),
            "discord" => Ok(Self::Discord),
            "microsoft" => Ok(Self::Microsoft),
            "apple" => Ok(Self::Apple),
            _ => Err(ForgeBaseError::InvalidInput(format!(
                "Unknown OAuth provider: {}",
                s
            ))),
        }
    }

    pub fn authorization_url(&self) -> &str {
        match self {
            Self::Google => "https://accounts.google.com/o/oauth2/v2/auth",
            Self::GitHub => "https://github.com/login/oauth/authorize",
            Self::GitLab => "https://gitlab.com/oauth/authorize",
            Self::Discord => "https://discord.com/api/oauth2/authorize",
            Self::Microsoft => "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
            Self::Apple => "https://appleid.apple.com/auth/authorize",
        }
    }

    pub fn token_url(&self) -> &str {
        match self {
            Self::Google => "https://oauth2.googleapis.com/token",
            Self::GitHub => "https://github.com/login/oauth/access_token",
            Self::GitLab => "https://gitlab.com/oauth/token",
            Self::Discord => "https://discord.com/api/oauth2/token",
            Self::Microsoft => "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            Self::Apple => "https://appleid.apple.com/auth/token",
        }
    }

    pub fn user_info_url(&self) -> &str {
        match self {
            Self::Google => "https://www.googleapis.com/oauth2/v2/userinfo",
            Self::GitHub => "https://api.github.com/user",
            Self::GitLab => "https://gitlab.com/api/v4/user",
            Self::Discord => "https://discord.com/api/users/@me",
            Self::Microsoft => "https://graph.microsoft.com/v1.0/me",
            Self::Apple => "https://appleid.apple.com/auth/userinfo",
        }
    }
}

/// OAuth configuration
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

/// OAuth user info
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub provider_user_id: String,
    pub email: String,
    pub email_verified: bool,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub metadata: serde_json::Value,
}

/// OAuth manager
pub struct OAuthManager {
    configs: HashMap<String, OAuthConfig>,
    http_client: reqwest::Client,
}

impl OAuthManager {
    pub fn new(configs: HashMap<String, OAuthConfig>) -> Self {
        Self {
            configs,
            http_client: reqwest::Client::new(),
        }
    }

    /// Get authorization URL
    pub fn get_authorization_url(
        &self,
        provider_name: &str,
        state: &str,
    ) -> Result<String> {
        let provider = OAuthProvider::from_str(provider_name)?;
        let config = self
            .configs
            .get(provider_name)
            .ok_or_else(|| {
                ForgeBaseError::Config(format!("OAuth provider not configured: {}", provider_name))
            })?;

        let mut url = url::Url::parse(provider.authorization_url())
            .map_err(|e| ForgeBaseError::Internal(format!("Invalid OAuth URL: {}", e)))?;

        url.query_pairs_mut()
            .append_pair("client_id", &config.client_id)
            .append_pair("redirect_uri", &config.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("state", state)
            .append_pair("scope", &config.scopes.join(" "));

        Ok(url.to_string())
    }

    /// Exchange code for token
    pub async fn exchange_code(
        &self,
        provider_name: &str,
        code: &str,
    ) -> Result<String> {
        let provider = OAuthProvider::from_str(provider_name)?;
        let config = self
            .configs
            .get(provider_name)
            .ok_or_else(|| {
                ForgeBaseError::Config(format!("OAuth provider not configured: {}", provider_name))
            })?;

        let mut params = HashMap::new();
        params.insert("client_id", config.client_id.as_str());
        params.insert("client_secret", config.client_secret.as_str());
        params.insert("code", code);
        params.insert("grant_type", "authorization_code");
        params.insert("redirect_uri", config.redirect_uri.as_str());

        let response = self
            .http_client
            .post(provider.token_url())
            .form(&params)
            .send()
            .await
            .map_err(|e| ForgeBaseError::ExternalService(format!("OAuth token request failed: {}", e)))?;

        let token_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ForgeBaseError::ExternalService(format!("Failed to parse token response: {}", e)))?;

        token_data["access_token"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| ForgeBaseError::ExternalService("No access token in response".to_string()))
    }

    /// Get user info from provider
    pub async fn get_user_info(
        &self,
        provider_name: &str,
        access_token: &str,
    ) -> Result<OAuthUserInfo> {
        let provider = OAuthProvider::from_str(provider_name)?;

        let response = self
            .http_client
            .get(provider.user_info_url())
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| ForgeBaseError::ExternalService(format!("User info request failed: {}", e)))?;

        let user_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| ForgeBaseError::ExternalService(format!("Failed to parse user info: {}", e)))?;

        // Parse user info based on provider
        self.parse_user_info(&provider, user_data)
    }

    fn parse_user_info(&self, provider: &OAuthProvider, data: serde_json::Value) -> Result<OAuthUserInfo> {
        match provider {
            OAuthProvider::Google => Ok(OAuthUserInfo {
                provider_user_id: data["id"]
                    .as_str()
                    .ok_or_else(|| ForgeBaseError::ExternalService("Missing user ID".to_string()))?
                    .to_string(),
                email: data["email"]
                    .as_str()
                    .ok_or_else(|| ForgeBaseError::ExternalService("Missing email".to_string()))?
                    .to_string(),
                email_verified: data["verified_email"].as_bool().unwrap_or(false),
                full_name: data["name"].as_str().map(String::from),
                avatar_url: data["picture"].as_str().map(String::from),
                metadata: data,
            }),
            OAuthProvider::GitHub => Ok(OAuthUserInfo {
                provider_user_id: data["id"]
                    .as_i64()
                    .ok_or_else(|| ForgeBaseError::ExternalService("Missing user ID".to_string()))?
                    .to_string(),
                email: data["email"]
                    .as_str()
                    .ok_or_else(|| ForgeBaseError::ExternalService("Missing email".to_string()))?
                    .to_string(),
                email_verified: true, // GitHub emails are verified
                full_name: data["name"].as_str().map(String::from),
                avatar_url: data["avatar_url"].as_str().map(String::from),
                metadata: data,
            }),
            // Add other providers as needed
            _ => Err(ForgeBaseError::Internal(format!(
                "User info parsing not implemented for provider: {:?}",
                provider
            ))),
        }
    }
}
