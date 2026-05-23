use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::auth::auth_middleware;
use crate::AppState;

mod account;
mod mail;

/// Builds the full application router.
/// Protected routes are wrapped with the JWT auth middleware via `route_layer`.
/// Public routes (register/login) are merged separately without the layer.
pub fn create_router(state: AppState) -> Router {
    let protected = Router::new()
        .route("/api/mails", get(mail::list_mails))
        .route("/api/mails", post(mail::send_mail))
        .route("/api/mails/{id}", get(mail::get_mail))
        // Only applies to routes declared above in this router
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let public = Router::new()
        .route("/api/register", post(account::register))
        .route("/api/login", post(account::login));

    Router::new()
        .merge(protected)
        .merge(public)
        .with_state(state)
}
