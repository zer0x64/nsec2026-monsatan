//! Entry point for the dartreg pub package registry server.
//!
//! Parses CLI arguments, initialises the database and shared state, then
//! builds the Axum router and starts listening.
//!
//! ## Authentication model (opt-out)
//!
//! All routes are protected by Bearer token auth **by default**.
//! Only routes explicitly placed in the `public` router below are anonymous.
//! To make a new route public, add it to `public` instead of `protected`.

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{middleware, response::Html, routing::get, Json, Router};
use clap::Parser;
use tower_http::{services::ServeDir, trace::TraceLayer};
use utoipa_axum::{router::OpenApiRouter, routes};

mod auth;
mod config;
mod db;
mod error;
mod models;
mod routes;
mod state;

use config::{DEFAULT_ARTIFACTS_DIR, DEFAULT_DB_PATH};
use routes::{
    advisories::{__path_list_advisories, list_advisories},
    download::{__path_download_package, download_package},
    packages::{
        __path_list_package_versions, __path_list_packages, list_package_versions, list_packages,
    },
    publish::{
        __path_finalize_upload, __path_get_upload_url, __path_upload_package, finalize_upload,
        get_upload_url, upload_package,
    },
};
use state::AppState;

#[derive(Parser)]
#[clap(about = "A hosted pub package repository (pub repository spec v2)")]
struct Cli {
    /// Address and port to listen on.
    #[clap(short, long, default_value = "127.0.0.1:3000")]
    bind_address: SocketAddr,

    /// Path to the SQLite database file (created if it does not exist).
    #[clap(long, default_value = DEFAULT_DB_PATH)]
    db_path: String,

    /// Directory where uploaded package archives are stored.
    #[clap(long, default_value = DEFAULT_ARTIFACTS_DIR)]
    artifacts_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    // Default to info-level logging with debug traces for tower_http so that
    // every request/response is printed without requiring RUST_LOG to be set.
    // Setting RUST_LOG at runtime will override this default.
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,tower_http=debug"));

    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let cli = Cli::parse();

    // Ensure the artifacts directory exists before accepting any uploads.
    tokio::fs::create_dir_all(&cli.artifacts_dir)
        .await
        .expect("Failed to create artifacts directory");

    // Extract the test packages tarballs
    let test_package_path = cli
        .artifacts_dir
        .join("2e3c5613-ea0d-41dd-aaef-defa3db4420c.tar.gz");
    if !test_package_path.exists() {
        tokio::fs::write(
            &test_package_path,
            include_bytes!("../seed_packages/2e3c5613-ea0d-41dd-aaef-defa3db4420c.tar.gz"),
        )
        .await
        .expect("Failed to write 2e3c5613-ea0d-41dd-aaef-defa3db4420c.tar.gz");
    }

    let test_package_path = cli
        .artifacts_dir
        .join("9c9cce66-2734-4254-b5d9-e386977c44e9.tar.gz");
    if !test_package_path.exists() {
        tokio::fs::write(
            &test_package_path,
            include_bytes!("../seed_packages/9c9cce66-2734-4254-b5d9-e386977c44e9.tar.gz"),
        )
        .await
        .expect("Failed to write 9c9cce66-2734-4254-b5d9-e386977c44e9.tar.gz");
    }

    let db = db::init_db(&cli.db_path)
        .await
        .expect("Failed to initialise database");

    let http_client = reqwest::Client::new();

    // Keep a separate binding so we can pass artifacts_dir to both AppState
    // and ServeDir without a use-after-move error.
    let artifacts_dir = cli.artifacts_dir;

    let state = Arc::new(AppState {
        db,
        http_client,
        artifacts_dir: artifacts_dir.clone(),
    });

    let (api_router, mut api) = OpenApiRouter::new()
        .routes(routes!(get_upload_url))
        .routes(routes!(upload_package))
        .routes(routes!(finalize_upload))
        .routes(routes!(list_packages))
        .routes(routes!(list_package_versions))
        .routes(routes!(list_advisories))
        .routes(routes!(download_package))
        .split_for_parts();

    api.info = utoipa::openapi::Info::builder()
        .title("dartreg")
        .description(Some("Monsatan's private Dart repository"))
        .build();

    // Protected routes: authentication is required for all of these.
    // The auth middleware is applied as a layer to this sub-router so that
    // adding a new route here automatically protects it.
    let protected = api_router
        .nest_service("/artifacts", ServeDir::new(&artifacts_dir))
        .with_state(state)
        .layer(middleware::from_fn(auth::require_auth));

    // Public routes: explicitly anonymous — no authentication required.
    // Only add routes here if they should be accessible without a token.
    let public = Router::new()
        .route(
            "/",
            get(|| async {
                Html(
                    "
Welcome to dartreg! Monsatan's private Dart repository!
This service is meant to be used from the Dart or Flutter CLI.
Alternatively, you can check out the <a href=\"/openapi.json\">OpenAPI documentation</a>
            ",
                )
            }),
        )
        .route("/openapi.json", get(move || async move { Json(api) }));

    let app = Router::new()
        .merge(protected)
        .merge(public)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(cli.bind_address)
        .await
        .expect("Failed to bind to address");

    tracing::info!("dartreg listening on http://{}", cli.bind_address);

    axum::serve(listener, app).await.expect("Server error");
}
