use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Router};
use clap::Parser;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};

mod auth;
mod db;
mod error;
mod models;
mod openapi;
mod routes;

use db::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// ── CLI ───────────────────────────────────────────────────────────────────────

/// Command-line arguments for the Checkmate server.
#[derive(Parser)]
struct Cli {
    /// Address and port to listen on.
    #[clap(short, long, default_value = "127.0.0.1:3000")]
    bind_address: SocketAddr,

    /// SQLite database URL (e.g. `sqlite://checkmate.db`).
    #[clap(
        short,
        long,
        env = "DATABASE_URL",
        default_value = "sqlite://checkmate.db"
    )]
    database_url: String,

    /// Expose OpenAPI documentation at `/openapi.json`.
    #[clap(short, long, default_value = "false")]
    openapi: bool,
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    // Initialise structured logging to stderr.
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

    // Open (or create) the SQLite database.
    let pool = SqlitePool::connect_with(
        SqliteConnectOptions::from_str(&cli.database_url)
            .expect("invalid DATABASE_URL")
            .create_if_missing(true),
    )
    .await
    .expect("failed to open database");

    // Run schema migrations (CREATE TABLE IF NOT EXISTS …).
    db::init_db(&pool)
        .await
        .expect("failed to initialise database schema");

    // Populate the database with initial users on first run.
    db::seed_db(&pool).await.expect("failed to seed database");

    let jwt_secret = db::get_or_create_jwt_secret(&pool).await;

    let state = Arc::new(AppState {
        db: pool,
        jwt_secret: jwt_secret,
    });

    // Compose the full application:
    //   - API routes (auth, users, matches, messages)
    //   - OpenAPI JSON + Swagger UI
    //   - Static file fallback for the SvelteKit frontend
    let mut app: Router = routes::build_router(state);

    if cli.openapi {
        app = app.merge(openapi::openapi_router());
    }
    app = app
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(tower::buffer::BufferLayer::new(1024))
                .layer(tower::limit::RateLimitLayer::new(5, Duration::from_secs(1))),
        )
        .fallback_service(ServeDir::new("./dist").fallback(ServeFile::new("./dist/index.html")));

    let listener = tokio::net::TcpListener::bind(cli.bind_address)
        .await
        .expect("failed to bind listener");

    tracing::info!("listening on {}", cli.bind_address);

    axum::serve(listener, app).await.expect("server error");
}
