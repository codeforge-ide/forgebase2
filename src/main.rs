use axum::{
    Router,
    http::StatusCode,
    routing::get,
    Json,
};
use serde_json::json;
use std::time::Duration;
use tracing_subscriber;
use forgebase_core::Config;
use forgebase_db::{DatabasePool, PoolConfig, init_database};

// State struct to hold shared resources
#[derive(Clone)]
pub struct AppState {
    pub db: DatabasePool,
}

#[tokio::main]
async fn main() {
    // Initialize environment and logging
    dotenvy::dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into()),
        )
        .init();

    tracing::info!("ForgeBase v{} starting...", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = match Config::from_env() {
        Ok(cfg) => {
            tracing::info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            tracing::warn!("Failed to load config from environment, using defaults: {}", e);
            Config::default()
        }
    };

    // Initialize database pool
    tracing::info!("Connecting to database...");
    let pool_config = PoolConfig {
        database_url: config.database.url.clone(),
        max_connections: config.database.max_connections,
        min_connections: config.database.min_connections,
        acquire_timeout: Duration::from_secs(config.database.acquire_timeout),
        idle_timeout: Some(Duration::from_secs(config.database.idle_timeout)),
        max_lifetime: Some(Duration::from_secs(1800)),
    };

    let db_pool = match DatabasePool::new(pool_config).await {
        Ok(pool) => {
            tracing::info!("Database pool created successfully");
            
            // Run migrations
            if let Err(e) = init_database(pool.pool()).await {
                tracing::error!("Failed to initialize database: {}", e);
                std::process::exit(1);
            }
            
            pool
        }
        Err(e) => {
            tracing::error!("Failed to create database pool: {}", e);
            // For development, allow running without database
            if config.server.environment == forgebase_core::Environment::Production {
                std::process::exit(1);
            }
            tracing::warn!("Continuing without database (development mode)");
            // Create a dummy pool - this will fail at runtime if DB operations are attempted
            // In a real implementation, you might want to skip database operations or use mocks
            panic!("Database connection is required for now");
        }
    };

    // Initialize application state
    let app_state = AppState {
        db: db_pool.clone(),
    };

    // Build the router
    let mut router = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        .route("/api/v1/health", get(health_check_json))
        .with_state(app_state);

    // TODO: Add routes as we build out features:
    // - Authentication routes from forgebase-auth
    // - Sites/Deployment routes from forgebase-sites
    // - Storage routes from forgebase-storage
    // - Database routes from forgebase-db
    // - GraphQL API from forgebase-api

    // Add CORS middleware
    use tower_http::cors::CorsLayer;
    router = router.layer(CorsLayer::very_permissive());

    // Add trace layer
    use tower_http::trace::TraceLayer;
    router = router.layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid socket address");

    tracing::info!("Starting ForgeBase server on {}", addr);
    tracing::info!("Environment: {:?}", config.server.environment);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, router)
        .await
        .expect("Server failed");
}

async fn root_handler() -> Json<serde_json::Value> {
    Json(json!({
        "name": "ForgeBase",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Open-source Firebase/Supabase/Vercel alternative",
        "status": "running"
    }))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn health_check_json() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
}
