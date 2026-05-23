use axum::{response::Html, routing::get, Json, Router};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::models::{
    ContactRequest, ContactResponse, LoginRequest, MatchesResponse, MessageResponse,
    RegisterRequest, SendMessageRequest, SwipeRequest, SwipeResponse, TokenResponse, UserResponse,
};

// ── Security scheme ───────────────────────────────────────────────────────────

/// Adds the `BearerAuth` HTTP security scheme to the generated OpenAPI spec.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "BearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

// ── OpenAPI document ──────────────────────────────────────────────────────────

/// Root OpenAPI document for the Checkmate API.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Checkmate API",
        version = "1.0.0",
        description = "Chess-themed dating app backend"
    ),
    paths(
        crate::routes::auth::register,
        crate::routes::auth::login,
        crate::routes::users::get_user,
        crate::routes::users::get_contact,
        crate::routes::users::get_next,
        crate::routes::users::swipe,
        crate::routes::matches::get_matches,
        crate::routes::messages::get_messages,
        crate::routes::messages::send_message,
    ),
    components(schemas(
        RegisterRequest,
        ContactRequest,
        LoginRequest,
        SwipeRequest,
        SendMessageRequest,
        TokenResponse,
        UserResponse,
        ContactResponse,
        MatchesResponse,
        MessageResponse,
        SwipeResponse,
    )),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

// ── Swagger UI HTML ───────────────────────────────────────────────────────────

/// A minimal HTML page that loads Swagger UI from a CDN and points it at our
/// `/api/openapi.json` endpoint.
const SWAGGER_HTML: &str = r##"<!DOCTYPE html>
<html>
  <head>
    <title>Checkmate API Docs</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css" />
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
    <script>
      window.onload = function () {
        SwaggerUIBundle({
          url: "/api/openapi.json",
          dom_id: "#swagger-ui",
          presets: [SwaggerUIBundle.presets.apis, SwaggerUIBundle.SwaggerUIStandalonePreset],
          layout: "BaseLayout",
        });
      };
    </script>
  </body>
</html>"##;

// ── Router ────────────────────────────────────────────────────────────────────

/// Build a router that serves the OpenAPI JSON spec and the Swagger UI.
///
/// Also writes `openapi.json` to the current working directory at startup so
/// the spec can be consumed by other tools without running the server.
pub fn openapi_router() -> Router {
    let spec = ApiDoc::openapi();

    // Persist the spec for offline tooling (e.g. code generators, linters).
    match spec.to_pretty_json() {
        Ok(json) => {
            if let Err(e) = std::fs::write("openapi.json", &json) {
                tracing::warn!("could not write openapi.json: {e}");
            }
        }
        Err(e) => tracing::warn!("could not serialise OpenAPI spec: {e}"),
    }

    // Clone the spec so it can be moved into the closure.
    let spec_clone = spec.clone();

    Router::new()
        // Serve the raw OpenAPI JSON spec.
        .route(
            "/api/openapi.json",
            get(move || {
                let s = spec_clone.clone();
                async move { Json(s) }
            }),
        )
        // Serve a CDN-backed Swagger UI that reads from the JSON endpoint.
        .route("/api/docs", get(|| async { Html(SWAGGER_HTML) }))
}
