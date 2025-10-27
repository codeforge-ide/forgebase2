use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Site/Project model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Site {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub repository_url: Option<String>,
    pub default_branch: String,
    pub framework: Option<SiteFramework>,
    pub build_command: Option<String>,
    pub output_directory: Option<String>,
    pub install_command: Option<String>,
    pub environment_variables: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Site framework detection
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum SiteFramework {
    #[serde(rename = "nextjs")]
    NextJs,
    #[serde(rename = "react")]
    React,
    #[serde(rename = "vue")]
    Vue,
    #[serde(rename = "svelte")]
    Svelte,
    #[serde(rename = "sveltekit")]
    SvelteKit,
    #[serde(rename = "nuxt")]
    Nuxt,
    #[serde(rename = "astro")]
    Astro,
    #[serde(rename = "remix")]
    Remix,
    #[serde(rename = "gatsby")]
    Gatsby,
    #[serde(rename = "hugo")]
    Hugo,
    #[serde(rename = "static")]
    Static,
}

/// Deployment model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Deployment {
    pub id: Uuid,
    pub site_id: Uuid,
    pub commit_sha: Option<String>,
    pub commit_message: Option<String>,
    pub branch: String,
    pub status: DeploymentStatus,
    pub build_logs: Option<String>,
    pub error_message: Option<String>,
    pub deployment_url: String,
    pub preview_url: Option<String>,
    pub build_duration_ms: Option<i64>,
    pub deployment_size_bytes: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deployed_at: Option<DateTime<Utc>>,
}

/// Deployment status
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text")]
pub enum DeploymentStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "building")]
    Building,
    #[serde(rename = "deploying")]
    Deploying,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// Custom domain model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Domain {
    pub id: Uuid,
    pub site_id: Uuid,
    pub domain: String,
    pub is_verified: bool,
    pub is_primary: bool,
    pub ssl_enabled: bool,
    pub ssl_cert: Option<String>,
    pub ssl_key: Option<String>,
    pub verification_token: String,
    pub created_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
}

/// Create site request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateSiteRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 50), regex = "^[a-z0-9-]+$")]
    pub slug: String,
    pub description: Option<String>,
    pub repository_url: Option<String>,
    #[serde(default = "default_branch")]
    pub default_branch: String,
    pub framework: Option<SiteFramework>,
    pub build_command: Option<String>,
    pub output_directory: Option<String>,
    pub install_command: Option<String>,
}

fn default_branch() -> String {
    "main".to_string()
}

/// Update site request
#[derive(Debug, Deserialize)]
pub struct UpdateSiteRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub repository_url: Option<String>,
    pub default_branch: Option<String>,
    pub framework: Option<SiteFramework>,
    pub build_command: Option<String>,
    pub output_directory: Option<String>,
    pub install_command: Option<String>,
    pub environment_variables: Option<serde_json::Value>,
}

/// Deploy site request
#[derive(Debug, Deserialize)]
pub struct DeploySiteRequest {
    pub branch: Option<String>,
    pub commit_sha: Option<String>,
}

/// Add domain request
#[derive(Debug, Deserialize, Validate)]
pub struct AddDomainRequest {
    #[validate(length(min = 3, max = 255))]
    pub domain: String,
    pub is_primary: bool,
}

/// Deployment response
#[derive(Debug, Serialize)]
pub struct DeploymentResponse {
    pub id: Uuid,
    pub site_id: Uuid,
    pub status: DeploymentStatus,
    pub deployment_url: String,
    pub preview_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Deployment> for DeploymentResponse {
    fn from(d: Deployment) -> Self {
        Self {
            id: d.id,
            site_id: d.site_id,
            status: d.status,
            deployment_url: d.deployment_url,
            preview_url: d.preview_url,
            created_at: d.created_at,
        }
    }
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub framework: Option<SiteFramework>,
    pub build_command: Option<String>,
    pub output_directory: Option<String>,
    pub install_command: Option<String>,
    pub node_version: String,
    pub environment_variables: serde_json::Value,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            framework: None,
            build_command: None,
            output_directory: None,
            install_command: Some("npm install".to_string()),
            node_version: "18".to_string(),
            environment_variables: serde_json::json!({}),
        }
    }
}

impl BuildConfig {
    /// Detect framework from package.json or other files
    pub fn detect_framework(package_json: &serde_json::Value) -> Option<SiteFramework> {
        let dependencies = package_json.get("dependencies")?;
        
        if dependencies.get("next").is_some() {
            Some(SiteFramework::NextJs)
        } else if dependencies.get("nuxt").is_some() {
            Some(SiteFramework::Nuxt)
        } else if dependencies.get("@sveltejs/kit").is_some() {
            Some(SiteFramework::SvelteKit)
        } else if dependencies.get("svelte").is_some() {
            Some(SiteFramework::Svelte)
        } else if dependencies.get("astro").is_some() {
            Some(SiteFramework::Astro)
        } else if dependencies.get("@remix-run/react").is_some() {
            Some(SiteFramework::Remix)
        } else if dependencies.get("gatsby").is_some() {
            Some(SiteFramework::Gatsby)
        } else if dependencies.get("vue").is_some() {
            Some(SiteFramework::Vue)
        } else if dependencies.get("react").is_some() {
            Some(SiteFramework::React)
        } else {
            None
        }
    }

    /// Get default build command for framework
    pub fn default_build_command(&self) -> String {
        match &self.framework {
            Some(SiteFramework::NextJs) => "npm run build".to_string(),
            Some(SiteFramework::React) => "npm run build".to_string(),
            Some(SiteFramework::Vue) => "npm run build".to_string(),
            Some(SiteFramework::Svelte) => "npm run build".to_string(),
            Some(SiteFramework::SvelteKit) => "npm run build".to_string(),
            Some(SiteFramework::Nuxt) => "npm run build".to_string(),
            Some(SiteFramework::Astro) => "npm run build".to_string(),
            Some(SiteFramework::Remix) => "npm run build".to_string(),
            Some(SiteFramework::Gatsby) => "gatsby build".to_string(),
            Some(SiteFramework::Hugo) => "hugo".to_string(),
            _ => "npm run build".to_string(),
        }
    }

    /// Get default output directory for framework
    pub fn default_output_directory(&self) -> String {
        match &self.framework {
            Some(SiteFramework::NextJs) => ".next".to_string(),
            Some(SiteFramework::React) => "build".to_string(),
            Some(SiteFramework::Vue) => "dist".to_string(),
            Some(SiteFramework::Svelte) => "public".to_string(),
            Some(SiteFramework::SvelteKit) => "build".to_string(),
            Some(SiteFramework::Nuxt) => ".output".to_string(),
            Some(SiteFramework::Astro) => "dist".to_string(),
            Some(SiteFramework::Remix) => "build".to_string(),
            Some(SiteFramework::Gatsby) => "public".to_string(),
            Some(SiteFramework::Hugo) => "public".to_string(),
            _ => "dist".to_string(),
        }
    }
}
