//! Verdachem Industries — Invoice Portal backend.
//!
//! Parses CLI arguments, initialises the SQLite database, builds the axum
//! router, and starts the HTTP server.

use std::net::SocketAddr;

use clap::Parser;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod catalog;
mod db;
mod models;
mod routes;
mod seed;

/// CLI arguments for the Verdachem Industries invoice server.
#[derive(Parser)]
#[command(
    name = "verdachem-invoices",
    about = "Verdachem Industries — Invoice Portal"
)]
struct Cli {
    /// Address and port to listen on.
    #[clap(short, long, default_value = "127.0.0.1:3000")]
    bind_address: SocketAddr,

    /// Path to the SQLite database file.
    /// Will be created automatically if it does not exist.
    #[clap(short, long, default_value = "verdachem.db")]
    database: String,

    /// Path to the website directory.
    #[clap(short, long, default_value = "dist")]
    website_dir: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    let pool = db::init_db(&cli.database)
        .await
        .expect("Failed to initialise the database");

    let state = db::AppState { db: pool };

    let app = routes::build_router(state)
        // Serve the compiled SvelteKit frontend from ./dist for all
        // non-API routes. The frontend is built by the build.rs script.
        .layer(TraceLayer::new_for_http())
        .fallback_service(
            ServeDir::new(&cli.website_dir)
                .fallback(ServeFile::new(cli.website_dir.to_string() + "/index.html")),
        );

    let listener = tokio::net::TcpListener::bind(cli.bind_address)
        .await
        .expect("Failed to bind to address");

    tracing::info!(
        "Verdachem Industries invoice server listening on {}",
        cli.bind_address
    );
    tracing::info!("Database: {}", cli.database);

    axum::serve(listener, app).await.expect("Server error");
}
