use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;

use crate::{
    auth::Claims,
    db::{self, AppState},
    error::{AppError, Result},
    models::{MessageResponse, SendMessageRequest},
};

// ── GET /api/user/:username/dm ────────────────────────────────────────────────

/// Retrieve the full conversation between the authenticated user and another user.
///
/// Both users must be mutually matched; returns `403 Forbidden` otherwise.
#[utoipa::path(
    get,
    path = "/api/user/{username}/dm",
    params(
        ("username" = String, Path, description = "Username of the other participant"),
    ),
    responses(
        (status = 200, description = "Message history", body = Vec<MessageResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Not matched with this user"),
        (status = 404, description = "User not found"),
    ),
    security(("BearerAuth" = []))
)]
pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(username): Path<String>,
) -> Result<Json<Vec<MessageResponse>>> {
    // Ensure the target user exists.
    db::get_user_by_username(&state.db, &username).await?;

    // Only matched users may read each other's messages.
    if !db::are_matched(&state.db, &claims.sub, &username).await? {
        return Err(AppError::Forbidden);
    }

    let rows = db::get_messages(&state.db, &claims.sub, &username).await?;

    let messages = rows
        .into_iter()
        .map(|row| MessageResponse {
            id: row.id,
            sender: row.sender,
            content: row.content,
            sent_at: row.sent_at,
        })
        .collect();

    Ok(Json(messages))
}

// ── POST /api/user/:username/dm ───────────────────────────────────────────────

/// Send a direct message to a matched user.
///
/// Both users must be mutually matched; returns `403 Forbidden` otherwise.
/// Returns the newly created message on success.
#[utoipa::path(
    post,
    path = "/api/user/{username}/dm",
    params(
        ("username" = String, Path, description = "Username of the recipient"),
    ),
    request_body = SendMessageRequest,
    responses(
        (status = 200, description = "Message sent", body = MessageResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Not matched with this user"),
        (status = 404, description = "User not found"),
    ),
    security(("BearerAuth" = []))
)]
pub async fn send_message(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(username): Path<String>,
    Json(req): Json<SendMessageRequest>,
) -> Result<Json<MessageResponse>> {
    // Ensure the target user exists.
    db::get_user_by_username(&state.db, &username).await?;

    // Only matched users may message each other.
    if !db::are_matched(&state.db, &claims.sub, &username).await? {
        return Err(AppError::Forbidden);
    }

    let id = db::insert_message(&state.db, &claims.sub, &username, &req.content).await?;

    // Use the current UTC time formatted the same way SQLite's datetime('now') does.
    let sent_at = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(Json(MessageResponse {
        id,
        sender: claims.sub,
        content: req.content,
        sent_at,
    }))
}
