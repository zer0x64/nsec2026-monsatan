use std::net::SocketAddr;

use clap::Parser;
use ed25519_dalek::SigningKey;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod constants;
mod db;
mod plugins;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub jwt_secret: Vec<u8>,
    pub signing_key: SigningKey,
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long, env = "BIND_ADDRESS", default_value = "127.0.0.1:3000")]
    bind_address: SocketAddr,

    #[clap(
        short,
        long,
        env = "DB_STRING",
        default_value = "mysql://root:password@localhost/mailserver"
    )]
    database_url: String,
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

    let (db, jwt_secret, signing_key) = db::init_db(&cli.database_url)
        .await
        .expect("Failed to initialise database");

    let state = AppState {
        db,
        jwt_secret,
        signing_key,
    };

    let app = routes::create_router(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .fallback_service(ServeDir::new("./dist").fallback(ServeFile::new("./dist/index.html")));

    let listener = tokio::net::TcpListener::bind(cli.bind_address)
        .await
        .expect("Failed to bind address");

    tracing::info!("Listening on {}", cli.bind_address);
    axum::serve(listener, app).await.unwrap();
}
