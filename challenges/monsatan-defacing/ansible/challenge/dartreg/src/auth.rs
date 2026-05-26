//! Authentication middleware for enforcing Bearer token auth on all routes.
//!
//! Auth is **opt-out**: every route is protected by default.
//! Only routes explicitly placed in the public router (see `main.rs`) bypass this check.

use axum::{
    body::Body, http::Request, middleware::Next, response::IntoResponse, response::Response,
};

use subtle::ConstantTimeEq;

use crate::config::API_KEY;
use crate::error::AppError;

/// Tower middleware that enforces `Authorization: Bearer <token>` on all requests.
///
/// Applied as a layer to the protected router in `main.rs`. Routes that should
/// be accessible without authentication must be explicitly placed in the public
/// router instead.
pub async fn require_auth(request: Request<Body>, next: Next) -> Response {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let expected = format!("Bearer {}", API_KEY);

    match auth_header {
        Some(header) if header.as_bytes().ct_eq(&expected.as_bytes()).into() => {
            next.run(request).await
        }
        _ => AppError::Unauthorized(
            "A valid API key is required. \
            Run `dart pub token add <hosted-url>` to configure one."
                .to_string(),
        )
        .into_response(),
    }
}
