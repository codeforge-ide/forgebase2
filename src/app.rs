use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use forgebase_core::Config;
use forgebase_db::DatabasePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: DatabasePool,
    pub config: Arc<Config>,
}

/// Create the application router
pub async fn create_app(db_pool: DatabasePool, config: Config) -> Router {
    let state = AppState {
        db: db_pool,
        config: Arc::new(config),
    };

    // Build the main router with health and root endpoints
    let mut app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        .route("/api/v1/health", get(health_check_json))
        .with_state(state.clone());

    // TODO: Add feature routes as we expand
    // .nest("/api/v1/auth", auth_routes)
    // .nest("/api/v1/sites", sites_routes)
    // .nest("/api/v1/storage", storage_routes)
    // .nest("/graphql", graphql_route)

    // Add middleware
    app = app.layer(CorsLayer::very_permissive());
    app = app.layer(TraceLayer::new_for_http());

    app
}

async fn root_handler(axum::extract::State(state): axum::extract::State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        "name": "ForgeBase",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Open-source Firebase/Supabase/Vercel alternative",
        "status": "running",
        "environment": format!("{:?}", state.config.server.environment),
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
