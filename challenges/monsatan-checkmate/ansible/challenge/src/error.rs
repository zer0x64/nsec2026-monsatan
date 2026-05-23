use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Convenience alias so every function in this crate can return `Result<T>`.
pub type Result<T> = std::result::Result<T, AppError>;

/// All errors that can occur within the application.
///
/// Each variant maps to an HTTP status code and a human-readable message
/// when converted into an Axum response.
#[derive(Debug)]
pub enum AppError {
    /// A SQLite / sqlx operation failed.
    Database(sqlx::Error),
    /// An internal server error occurred.
    InternalServerError(String),
    /// The request is missing or has an invalid JWT.
    Unauthorized,
    /// The caller is authenticated but not permitted to access the resource.
    Forbidden,
    /// The requested resource does not exist.
    NotFound,
    /// A unique-constraint conflict (e.g. duplicate username).
    Conflict(String),
    /// The caller supplied invalid input.
    BadRequest(String),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        // Treat a missing row as a 404 rather than a 500.
        match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            other => AppError::Database(other),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                tracing::error!("database error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => {
                tracing::error!("internal server error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
