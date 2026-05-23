use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    auth::Claims,
    db::{self, AppState},
    error::Result,
    models::MatchesResponse,
};

// ── GET /api/matches ──────────────────────────────────────────────────────────

/// Return all usernames that have mutually liked the authenticated user.
#[utoipa::path(
    get,
    path = "/api/matches",
    responses(
        (status = 200, description = "List of matched usernames", body = MatchesResponse),
        (status = 401, description = "Unauthorized"),
    ),
    security(("BearerAuth" = []))
)]
pub async fn get_matches(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<MatchesResponse>> {
    let matches = db::get_matches(&state.db, &claims.sub).await?;

    Ok(Json(MatchesResponse { matches }))
}
