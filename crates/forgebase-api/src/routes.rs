//! API routes

use axum::{
    routing::{delete, get, post, put},
    Router,
};

/// Create all API routes
pub fn create_routes() -> Router {
    Router::new()
        .nest("/api/v1", api_v1_routes())
}

/// API v1 routes
fn api_v1_routes() -> Router {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/db", database_routes())
        .nest("/storage", storage_routes())
        .nest("/functions", functions_routes())
}

/// Authentication routes
fn auth_routes() -> Router {
    Router::new()
        .route("/signup", post(|| async { "Sign up" }))
        .route("/signin", post(|| async { "Sign in" }))
        .route("/signout", post(|| async { "Sign out" }))
        .route("/refresh", post(|| async { "Refresh token" }))
        .route("/user", get(|| async { "Get user" }))
        .route("/user", put(|| async { "Update user" }))
}

/// Database routes
fn database_routes() -> Router {
    Router::new()
        .route("/query", post(|| async { "Execute query" }))
        .route("/tables", get(|| async { "List tables" }))
        .route("/tables/:table", get(|| async { "Get table schema" }))
        .route("/tables", post(|| async { "Create table" }))
        .route("/tables/:table", delete(|| async { "Drop table" }))
}

/// Storage routes
fn storage_routes() -> Router {
    Router::new()
        .route("/upload", post(|| async { "Upload file" }))
        .route("/:bucket/:key", get(|| async { "Download file" }))
        .route("/:bucket/:key", delete(|| async { "Delete file" }))
        .route("/:bucket", get(|| async { "List files" }))
        .route("/presigned", post(|| async { "Generate presigned URL" }))
}

/// Functions routes
fn functions_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "List functions" }))
        .route("/", post(|| async { "Deploy function" }))
        .route("/:id", get(|| async { "Get function" }))
        .route("/:id", put(|| async { "Update function" }))
        .route("/:id", delete(|| async { "Delete function" }))
        .route("/:id/invoke", post(|| async { "Invoke function" }))
}
