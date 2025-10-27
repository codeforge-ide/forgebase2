//! HTTP server implementation

use axum::{
    routing::get,
    Router,
};
use forgebase_core::{ForgeBaseError, Result};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_cors: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            enable_cors: true,
        }
    }
}

/// HTTP server
pub struct HttpServer {
    config: ServerConfig,
    router: Router,
}

impl HttpServer {
    /// Create a new HTTP server
    pub fn new(config: ServerConfig) -> Self {
        let router = Router::new()
            .route("/health", get(health_handler))
            .route("/", get(root_handler));

        Self { config, router }
    }

    /// Add routes to the server
    pub fn with_routes(mut self, routes: Router) -> Self {
        self.router = self.router.merge(routes);
        self
    }

    /// Add middleware layers
    pub fn with_middleware(mut self) -> Self {
        let mut router = self.router;

        // Add CORS if enabled
        if self.config.enable_cors {
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);
            router = router.layer(cors);
        }

        // Add tracing
        router = router.layer(TraceLayer::new_for_http());

        self.router = router;
        self
    }

    /// Start the server
    pub async fn start(self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.port)
            .parse::<SocketAddr>()
            .map_err(|e| ForgeBaseError::Internal(format!("Invalid address: {}", e)))?;

        tracing::info!("Starting server on {}", addr);

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Failed to bind: {}", e)))?;

        axum::serve(listener, self.router)
            .await
            .map_err(|e| ForgeBaseError::Internal(format!("Server error: {}", e)))?;

        Ok(())
    }
}

/// Health check handler
async fn health_handler() -> &'static str {
    "OK"
}

/// Root handler
async fn root_handler() -> &'static str {
    "ForgeBase API v1.0"
}
