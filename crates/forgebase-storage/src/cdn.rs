//! CDN integration and caching

use forgebase_core::{ForgeBaseError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// CDN cache entry
#[derive(Debug, Clone)]
struct CacheEntry {
    data: bytes::Bytes,
    content_type: Option<String>,
    expires_at: chrono::DateTime<chrono::Utc>,
}

/// CDN configuration
#[derive(Debug, Clone)]
pub struct CdnConfig {
    pub enable_caching: bool,
    pub cache_ttl_seconds: i64,
    pub max_cache_size_mb: usize,
    pub cdn_domain: Option<String>,
}

impl Default for CdnConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_ttl_seconds: 3600,
            max_cache_size_mb: 100,
            cdn_domain: None,
        }
    }
}

/// CDN manager
pub struct CdnManager {
    config: CdnConfig,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl CdnManager {
    pub fn new(config: CdnConfig) -> Self {
        Self {
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get cached file
    pub async fn get_cached(&self, key: &str) -> Option<(bytes::Bytes, Option<String>)> {
        if !self.config.enable_caching {
            return None;
        }

        let cache = self.cache.read().await;
        if let Some(entry) = cache.get(key) {
            if entry.expires_at > chrono::Utc::now() {
                return Some((entry.data.clone(), entry.content_type.clone()));
            }
        }

        None
    }

    /// Cache a file
    pub async fn cache_file(
        &self,
        key: &str,
        data: bytes::Bytes,
        content_type: Option<String>,
    ) -> Result<()> {
        if !self.config.enable_caching {
            return Ok(());
        }

        let expires_at =
            chrono::Utc::now() + chrono::Duration::seconds(self.config.cache_ttl_seconds);

        let entry = CacheEntry {
            data,
            content_type,
            expires_at,
        };

        let mut cache = self.cache.write().await;
        cache.insert(key.to_string(), entry);

        // Simple cache size management
        if cache.len() > 1000 {
            self.evict_expired(&mut cache).await;
        }

        Ok(())
    }

    /// Invalidate cache for a key
    pub async fn invalidate(&self, key: &str) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.remove(key);
        Ok(())
    }

    /// Clear all cache
    pub async fn clear_cache(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }

    /// Evict expired entries
    async fn evict_expired(&self, cache: &mut HashMap<String, CacheEntry>) {
        let now = chrono::Utc::now();
        cache.retain(|_, entry| entry.expires_at > now);
    }

    /// Get CDN URL for a file
    pub fn get_cdn_url(&self, bucket: &str, key: &str) -> String {
        if let Some(ref domain) = self.config.cdn_domain {
            format!("https://{}/{}/{}", domain, bucket, key)
        } else {
            format!("/storage/{}/{}", bucket, key)
        }
    }

    /// Generate cache control headers
    pub fn get_cache_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert(
            "Cache-Control".to_string(),
            format!("public, max-age={}", self.config.cache_ttl_seconds),
        );
        headers
    }
}

impl Clone for CdnManager {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            cache: Arc::clone(&self.cache),
        }
    }
}
