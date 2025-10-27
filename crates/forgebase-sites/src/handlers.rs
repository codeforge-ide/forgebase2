// HTTP handlers for sites
use axum::{routing::{get, post}, Router};
use forgebase_core::ApiResponse;

pub fn create_sites_routes() -> Router {
    Router::new()
        .route("/sites", post(create_site))
        .route("/sites/:id", get(get_site))
        .route("/sites/:id/deploy", post(deploy_site))
        .route("/sites/:id/domains", post(add_domain))
}

async fn create_site() -> axum::Json<ApiResponse<String>> {
    // TODO: Implement
    axum::Json(ApiResponse::success("Not implemented".to_string()))
}

async fn get_site() -> axum::Json<ApiResponse<String>> {
    // TODO: Implement
    axum::Json(ApiResponse::success("Not implemented".to_string()))
}

async fn deploy_site() -> axum::Json<ApiResponse<String>> {
    // TODO: Implement
    axum::Json(ApiResponse::success("Not implemented".to_string()))
}

async fn add_domain() -> axum::Json<ApiResponse<String>> {
    // TODO: Implement
    axum::Json(ApiResponse::success("Not implemented".to_string()))
}
