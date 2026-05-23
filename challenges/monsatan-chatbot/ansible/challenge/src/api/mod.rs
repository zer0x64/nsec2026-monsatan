mod chat;
mod reboot;

pub use chat::OllamaConfig;

use std::sync::{Arc, Mutex};

use axum::{routing::post, Router};
use rusqlite::Connection;

/// Combined application state shared across all API handlers.
#[derive(Clone)]
pub struct AppState {
    /// Ollama backend configuration (URL + model name).
    pub ollama: OllamaConfig,
    /// Handle to the SQLite power-plant database.
    pub db: Arc<Mutex<Connection>>,
}

/// Builds the `/api` sub-router with all API endpoints.
///
/// The provided `state` is attached as shared state and made available
/// to all handlers via Axum's `State` extractor.
pub fn router(state: AppState) -> Router {
    Router::new()
        // POST /api/chat — proxy to Ollama, with optional tool access
        .route("/chat", post(chat::chat))
        // POST /api/reboot — reboot the server
        .route("/reboot", post(reboot::reboot))
        .with_state(state)
}
