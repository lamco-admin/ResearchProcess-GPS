use crate::auth::{AuthContext, API_KEY_STORE};
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());
    
    if let Some(auth_value) = auth_header {
        // Check if it's a Bearer token
        if let Some(token) = auth_value.strip_prefix("Bearer ") {
            // Validate the API key
            if let Some(api_key) = API_KEY_STORE.validate_key(token) {
                // Add user context to request extensions
                request.extensions_mut().insert(AuthContext {
                    user_id: api_key.user_id,
                    api_key_id: api_key.id,
                    permissions: api_key.permissions.clone(),
                });
                
                return Ok(next.run(request).await);
            }
        }
    }
    
    // No valid auth found
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::now_v7().to_string();
    request.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        "x-request-id",
        request_id.parse().unwrap(),
    );
    
    response
}