//! Application error type that maps to appropriate HTTP responses per the pub spec.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// All error variants that route handlers can return.
/// Each variant carries a human-readable message string.
#[derive(Debug)]
pub enum AppError {
    /// 404 – Package or resource not found.
    NotFound(String),
    /// 401 – Missing or invalid Bearer token.
    Unauthorized(String),
    /// 409 – A version with the same name already exists.
    Conflict(String),
    /// 400 – The client sent a malformed or invalid request.
    BadRequest(String),
    /// 500 – An unexpected server-side error occurred.
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": { "code": "not_found", "message": msg } })),
            )
                .into_response(),

            // Per spec: 401 must include a WWW-Authenticate header with a message.
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                [(
                    "WWW-Authenticate",
                    format!("Bearer realm=\"pub\", message=\"{}\"", msg),
                )],
                Json(json!({ "error": { "code": "unauthorized", "message": msg } })),
            )
                .into_response(),

            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                Json(json!({ "error": { "code": "conflict", "message": msg } })),
            )
                .into_response(),

            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": { "code": "bad_request", "message": msg } })),
            )
                .into_response(),

            AppError::Internal(msg) => {
                // Log the internal details but do not expose them to the client.
                tracing::error!("Internal server error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        json!({ "error": { "code": "internal_error", "message": "An internal server error occurred." } }),
                    ),
                )
                    .into_response()
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Internal(format!("Database error: {e}"))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Internal(format!("HTTP client error: {e}"))
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Internal(format!("IO error: {e}"))
    }
}
