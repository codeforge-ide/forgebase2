mod app;

use std::time::Duration;
use tracing_subscriber;
use forgebase_core::Config;
use forgebase_db::{DatabasePool, PoolConfig};

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
            tracing::info!("Database pool created successfully (size: {})", pool.size());
            
            // Run migrations
            if let Err(e) = forgebase_db::init_database(pool.pool()).await {
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
            panic!("Database connection is required for now");
        }
    };

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port)
        .parse::<std::net::SocketAddr>()
        .expect("Invalid socket address");

    tracing::info!("Starting ForgeBase server on {}", addr);
    tracing::info!("Environment: {:?}", config.server.environment);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    let app = app::create_app(db_pool, config).await;

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
