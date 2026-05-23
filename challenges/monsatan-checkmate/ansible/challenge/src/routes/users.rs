use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    auth::Claims,
    db::{self, AppState, UserRow},
    error::{AppError, Result},
    models::{ContactResponse, SwipeRequest, SwipeResponse, UserResponse},
};

// ── GET /api/user/:username ───────────────────────────────────────────────────

/// Get the public profile of any user by username.
#[utoipa::path(
    get,
    path = "/api/user/{username}",
    params(
        ("username" = String, Path, description = "Username of the profile to fetch"),
    ),
    responses(
        (status = 200, description = "User profile", body = UserRow),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    ),
    security(("BearerAuth" = []))
)]
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<Json<UserRow>> {
    let user = db::get_user_by_username(&state.db, &username).await?;

    Ok(Json(user))
}

// ── GET /api/user/:username/contact ──────────────────────────────────────────

/// Get the private contact information of a matched user.
///
/// Returns `403 Forbidden` when the authenticated user is not yet matched
/// with the requested user.
#[utoipa::path(
    get,
    path = "/api/user/{username}/contact",
    params(
        ("username" = String, Path, description = "Username whose contact info to fetch"),
    ),
    responses(
        (status = 200, description = "Contact information", body = ContactResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Not matched with this user"),
        (status = 404, description = "User not found"),
    ),
    security(("BearerAuth" = []))
)]
pub async fn get_contact(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(username): Path<String>,
) -> Result<Json<ContactResponse>> {
    // Ensure the target user actually exists.
    db::get_user_by_username(&state.db, &username).await?;

    // Only mutually matched users may see each other's contact info.
    if !db::are_matched(&state.db, &claims.sub, &username).await? {
        return Err(AppError::Forbidden);
    }

    let contact = db::get_contact_by_username(&state.db, &username).await?;

    Ok(Json(ContactResponse {
        phone: contact.phone,
        email: contact.email,
        location: contact.location,
    }))
}

// ── GET /api/next ─────────────────────────────────────────────────────────────

/// Return a random user that the authenticated user has not yet swiped on.
///
/// Returns `404 Not Found` when every other user has already been swiped.
#[utoipa::path(
    get,
    path = "/api/next",
    responses(
        (status = 200, description = "Next unswiped user profile", body = UserResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "No more users to swipe on"),
    ),
    security(("BearerAuth" = []))
)]
pub async fn get_next(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<UserResponse>> {
    let user = db::get_next_user(&state.db, &claims.sub)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Json(UserResponse {
        username: user.username,
        bio: user.bio,
        rating: user.rating,
        profile_picture: user.profile_picture,
    }))
}

// ── POST /api/user/:username/swipe ────────────────────────────────────────────

/// Record a like or pass on another user's profile.
///
/// Returns `{ "matched": true }` when this swipe created a mutual match.
#[utoipa::path(
    post,
    path = "/api/user/{username}/swipe",
    params(
        ("username" = String, Path, description = "Username of the user being swiped on"),
    ),
    request_body = SwipeRequest,
    responses(
        (status = 200, description = "Swipe recorded", body = SwipeResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
    ),
    security(("BearerAuth" = []))
)]
pub async fn swipe(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(username): Path<String>,
    Json(req): Json<SwipeRequest>,
) -> Result<Json<SwipeResponse>> {
    // Ensure the target user exists before recording the swipe.
    db::get_user_by_username(&state.db, &username).await?;

    db::record_swipe(&state.db, &claims.sub, &username, req.liked).await?;

    // A match only occurs when both users liked each other.
    let matched = req.liked && db::are_matched(&state.db, &claims.sub, &username).await?;

    Ok(Json(SwipeResponse { matched }))
}
