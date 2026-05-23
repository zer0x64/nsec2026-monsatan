use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::db::AppState;

pub mod auth;
pub mod matches;
pub mod messages;
pub mod users;

/// Build the full application router.
///
/// Authentication is enforced inside individual handlers via the [`crate::auth::Claims`]
/// extractor — handlers that require auth simply declare it as a parameter, while
/// public handlers omit it.  This means no middleware layer is needed here.
pub fn build_router(state: Arc<AppState>) -> Router {
    // Routes that require no authentication.
    let public = Router::new()
        .route("/api/register", post(auth::register))
        .route("/api/login", post(auth::login));

    // Routes that require a valid JWT (enforced via the Claims extractor).
    let authenticated = Router::new()
        .route("/api/user/{username}", get(users::get_user))
        .route("/api/user/{username}/contact", get(users::get_contact))
        .route(
            "/api/user/{username}/dm",
            get(messages::get_messages).post(messages::send_message),
        )
        .route("/api/user/{username}/swipe", post(users::swipe))
        .route("/api/matches", get(matches::get_matches))
        .route("/api/next", get(users::get_next))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::auth_middleware,
        ));

    public.merge(authenticated).with_state(state)
}
