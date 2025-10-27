//! API middleware

use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};

/// Request ID middleware
pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();
    request.extensions_mut().insert(request_id.clone());
    
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    response
}

/// Logging middleware
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    tracing::info!("{} {}", method, uri);
    
    let response = next.run(request).await;
    
    tracing::info!("{} {} -> {}", method, uri, response.status());
    
    response
}
