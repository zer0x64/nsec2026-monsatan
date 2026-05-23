#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router};
    use leptos::config::get_configuration;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use monsatan_orders::{app::App, shell};
    use tower_http::trace::TraceLayer;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    // Read site address, pkg dir, etc. from [package.metadata.leptos] in Cargo.toml.
    // These can be overridden at runtime via LEPTOS_* environment variables.
    let conf = get_configuration(Some("Cargo.toml")).unwrap_or(leptos::config::get_config_from_env().unwrap());
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // Collect all routes declared in the Leptos App component so Axum knows
    // which paths to hand off to the SSR renderer.
    let routes = generate_route_list(App);

    let app = Router::new()
        // Redirect the root to the login page.
        .route("/", get(|| async { axum::response::Redirect::permanent("/login") }))
        // REST endpoint: returns the flag JSON when a valid Bearer token is presented.
        .route("/api/flag", get(get_flag_api))
        // Hand all Leptos routes (SSR + server functions) to leptos_axum.
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        // Serve static assets (CSS, WASM, JS) from target/site — built by cargo-leptos.
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    tracing::info!("Monsatan portal listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

/// GET /api/flag — validates the `Authorization: Bearer <token>` header using
/// [`monsatan_orders::JWT_SECRET`] and returns the flag as JSON.
///
/// Returns 401 if the header is absent, malformed, or the token is
/// invalid / expired.
#[cfg(feature = "ssr")]
async fn get_flag_api(headers: axum::http::HeaderMap) -> axum::response::Response {
    use axum::{http::StatusCode, Json};
    use axum::response::IntoResponse;
    use monsatan_orders::{auth, constants::FLAG, pages::flag::FlagData, JWT_SECRET};

    // Extract the Bearer token from the Authorization header.
    let token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");

    match auth::validate_token(token, JWT_SECRET.as_slice()) {
        Ok(claims) => {
            if claims.sub != "admin@monsatan.ctf" {
                return StatusCode::FORBIDDEN.into_response();
            }

            Json(FlagData {
                username: claims.sub,
                flag: FLAG.to_string(),
            })
            .into_response()
        }
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}

// Fallback entry point for non-SSR builds (e.g. Trunk / CSR mode).
// cargo-leptos always uses the ssr feature for the binary, so this is
// only here to satisfy the compiler when building without features.
#[cfg(not(feature = "ssr"))]
pub fn main() {}
