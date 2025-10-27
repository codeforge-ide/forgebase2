pub mod handlers;
pub mod jwt;
pub mod middleware;
pub mod models;
pub mod oauth;
pub mod password;
pub mod repository;
pub mod service;
pub mod session;
pub mod email;
pub mod mfa;

pub use handlers::*;
pub use jwt::*;
pub use middleware::*;
pub use models::*;
pub use service::*;
pub use session::*;

use axum::Router;

/// Create authentication routes
pub fn create_routes() -> Router<AuthState> {
    handlers::create_auth_routes()
}
