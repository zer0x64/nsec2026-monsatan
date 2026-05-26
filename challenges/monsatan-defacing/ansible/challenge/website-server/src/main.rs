use std::{path::PathBuf, sync::Arc, time::Duration};

use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderValue, Request, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use clap::Parser;
use tokio::fs;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{EnvFilter, fmt};

const FLAG: &'static [u8] = b"FLAG-{4f2cf974cb28e5e8389fadbde2a5a978}";

const EGG: &'static [u8] = b"We will not back down until Monsatan is crumbling in its filth";

/// A simple static file server that reads files on every request,
/// allowing the served content to be modified on the fly.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The directory whose contents will be served
    root: PathBuf,

    /// The address to listen on (e.g. "127.0.0.1:8080")
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    address: String,
}

/// Shared application state passed to every handler.
struct AppState {
    root: PathBuf,
}

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber. Defaults to INFO level so requests are
    // always printed, but can be overridden via the RUST_LOG environment variable.
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    // Resolve the root directory to an absolute path so that path traversal
    // checks below are reliable regardless of the working directory.
    let root = args
        .root
        .canonicalize()
        .expect("Failed to resolve the root directory");

    tracing::info!("Serving '{}' on http://{}", root.display(), args.address);

    let state = Arc::new(AppState { root });

    let app = Router::new()
        // Match both "/" (index) and any file path.
        .route("/", get(serve_index))
        .route("/{*path}", get(serve_file))
        .with_state(state)
        // Log every incoming request and its response status code.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_response(|response: &Response, latency: Duration, _span: &Span| {
                    tracing::info!(status = %response.status(), latency = ?latency, "response");
                }),
        );

    let listener = tokio::net::TcpListener::bind(&args.address)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server encountered a fatal error");
}

/// Serve the index file ("index.html") at the root path.
async fn serve_index(State(state): State<Arc<AppState>>) -> Response {
    serve_path(&state.root, "index.html").await
}

/// Serve an arbitrary file under the root directory.
async fn serve_file(State(state): State<Arc<AppState>>, Path(path): Path<String>) -> Response {
    serve_path(&state.root, &path).await
}

/// Read the file at `root/relative_path` and return it as an HTTP response.
///
/// Returns 404 if the file does not exist and 403 if the resolved path
/// escapes the root directory (path traversal attempt).
async fn serve_path(root: &PathBuf, relative_path: &str) -> Response {
    // Build the candidate path and canonicalize it to resolve ".." segments.
    let candidate = root.join(relative_path);

    let canonical = match candidate.canonicalize() {
        Ok(p) => p,
        // canonicalize fails when the file simply doesn't exist.
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    // Reject any path that escapes the root directory.
    if !canonical.starts_with(root) {
        return StatusCode::FORBIDDEN.into_response();
    }

    // Read the file contents fresh on every request so that on-disk changes
    // are immediately visible without restarting the server.
    let mut contents = match fs::read(&canonical).await {
        Ok(data) => data,
        Err(_) => return StatusCode::NOT_FOUND.into_response(),
    };

    if let Some(i) = find_subsequence(&contents, EGG) {
        contents.splice(i..i + EGG.len(), FLAG.to_vec());
    }

    // Guess the MIME type from the file extension, defaulting to
    // "application/octet-stream" for unknown types.
    let mime = mime_guess::from_path(&canonical)
        .first_or_octet_stream()
        .to_string();

    let content_type = HeaderValue::from_str(&mime)
        .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, content_type)],
        contents,
    )
        .into_response()
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
