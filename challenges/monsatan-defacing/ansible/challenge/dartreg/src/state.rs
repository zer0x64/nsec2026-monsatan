//! Shared application state injected into every route handler via Axum's `State` extractor.

use sqlx::SqlitePool;
use std::path::PathBuf;

/// State shared across all request handlers.
pub struct AppState {
    /// Connection pool for the SQLite database.
    pub db: SqlitePool,

    /// HTTP client used to proxy requests to pub.dev.
    pub http_client: reqwest::Client,

    /// Filesystem directory where package archives are stored.
    pub artifacts_dir: PathBuf,
}
