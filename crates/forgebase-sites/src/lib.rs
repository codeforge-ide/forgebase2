pub mod builder;
pub mod deployment;
pub mod domain;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod service;
pub mod ssr;

pub use handlers::*;
pub use models::*;
pub use service::*;

use axum::Router;

/// Create sites routes
pub fn create_routes() -> Router {
    handlers::create_sites_routes()
}
