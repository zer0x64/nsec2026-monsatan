use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    db::AppState,
    error::{AppError, Result},
};

/// JWTs issued by this server expire after this many hours.
const JWT_EXPIRY_HOURS: i64 = 24;

// ── Claims ────────────────────────────────────────────────────────────────────

/// JWT payload stored inside every token issued by this server.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Username of the authenticated user (`sub`ject field).
    pub sub: String,
    /// Unix timestamp at which the token expires.
    pub exp: usize,
}

// ── Token helpers ─────────────────────────────────────────────────────────────

/// Create a signed HS256 JWT for `username` that expires in [`JWT_EXPIRY_HOURS`].
pub fn create_token(username: &str, secret: &[u8]) -> Result<String> {
    let expiry = Utc::now() + Duration::hours(JWT_EXPIRY_HOURS);
    let claims = Claims {
        sub: username.to_string(),
        exp: expiry.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|_| AppError::Unauthorized)
}

/// Decode and verify a JWT, returning the embedded [`Claims`] on success.
pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}

// ── Axum extractor ────────────────────────────────────────────────────────────

/// Axum extractor that reads the `Authorization: Bearer <token>` header,
/// verifies the JWT against the application secret, and returns the [`Claims`].
///
/// Handlers that require authentication simply declare `claims: Claims` as a
/// parameter; unauthenticated handlers omit it entirely.
impl FromRequestParts<Arc<AppState>> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &Arc<AppState>,
    ) -> std::result::Result<Self, Self::Rejection> {
        let claims: Option<&Claims> = parts.extensions.get();

        if let Some(claims) = claims {
            return Ok(claims.clone());
        }

        Err(AppError::Unauthorized)
    }
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> std::result::Result<Response, StatusCode> {
    // Extract the raw Authorization header value.
    let header = match headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    {
        Some(header) => header,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Expect the "Bearer <token>" scheme.
    let token = match header.strip_prefix("Bearer ") {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let claims = match verify_token(token, &state.jwt_secret) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}
