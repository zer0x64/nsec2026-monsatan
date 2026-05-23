mod api;
mod db;

use std::net::SocketAddr;

use api::{AppState, OllamaConfig};
use axum::Router;
use clap::Parser;
use tower_http::services::ServeDir;

/// Default Ollama chat endpoint.
const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434/api/chat";

/// Default Ollama model.
const DEFAULT_OLLAMA_MODEL: &str = "qwen2.5:0.5b";

/// Default SQLite database path.
const DEFAULT_DB_PATH: &str = "./monsatan.db";

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "127.0.0.1:3000")]
    bind_address: SocketAddr,

    /// URL of the Ollama chat API endpoint.
    #[clap(long, default_value = DEFAULT_OLLAMA_URL)]
    ollama_url: String,

    /// Ollama model name to use for chat completions.
    #[clap(long, default_value = DEFAULT_OLLAMA_MODEL)]
    ollama_model: String,

    /// Path to the SQLite database file. Created and seeded on first boot.
    #[clap(long, default_value = DEFAULT_DB_PATH)]
    db_path: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing for structured logging.
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Open (or create) the power-plant database and seed it if empty.
    let db = db::init(&cli.db_path).unwrap_or_else(|e| {
        tracing::error!("Failed to initialise database at {}: {e}", cli.db_path);
        std::process::exit(1);
    });

    tracing::info!("Database ready at {}", cli.db_path);

    let state = AppState {
        ollama: OllamaConfig {
            chat_url: cli.ollama_url,
            model: cli.ollama_model,
        },
        db,
    };

    let app = Router::new()
        // Mount all /api/* routes, with shared state injected.
        .nest("/api", api::router(state))
        // Serve the compiled frontend from ./dist for all other paths.
        .fallback_service(ServeDir::new("./dist"));

    let listener = tokio::net::TcpListener::bind(cli.bind_address)
        .await
        .unwrap();

    tracing::info!("Listening on {}", cli.bind_address);
    axum::serve(listener, app).await.unwrap();
}
