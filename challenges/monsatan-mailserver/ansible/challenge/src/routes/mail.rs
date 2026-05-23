use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{auth::AuthUser, constants::MAIL_DOMAIN, plugins, AppState};

/// Summary shown in the inbox listing (no body).
#[derive(Serialize)]
pub struct MailSummary {
    pub id: i64,
    pub from: String,
    pub subject: String,
    pub sent_at: String,
    pub is_read: bool,
}

/// Full mail detail including body.
#[derive(Serialize)]
pub struct MailDetail {
    pub id: i64,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub sent_at: String,
    pub is_read: bool,
}

#[derive(Serialize)]
pub struct SendMailResponse {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct SendMailRequest {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub plugins: Option<Vec<String>>,
}

/// GET /api/mails — list all mails in the authenticated user's inbox.
pub async fn list_mails(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<Json<Vec<MailSummary>>, StatusCode> {
    // (id, sender_username, subject, sent_at, is_read)
    let rows = sqlx::query_as::<_, (i64, String, String, String, i64)>(
        "SELECT m.id, u.username, m.subject, m.sent_at, m.is_read
         FROM mails m
         JOIN users u ON m.from_id = u.username
         WHERE m.to_id = ?
         ORDER BY m.sent_at DESC",
    )
    .bind(claims.sub)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mails = rows
        .into_iter()
        .map(|(id, from, subject, sent_at, is_read)| MailSummary {
            id,
            from: format!("{from}@{MAIL_DOMAIN}"),
            subject,
            sent_at,
            is_read: is_read != 0,
        })
        .collect();

    Ok(Json(mails))
}

/// GET /api/mails/{id} — fetch a single mail and mark it as read.
pub async fn get_mail(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(mail_id): Path<i64>,
) -> Result<Json<MailDetail>, StatusCode> {
    // Only allow reading mails addressed to the authenticated user.
    // (id, from_username, to_username, subject, body, sent_at, is_read)
    let row = sqlx::query_as::<_, (i64, String, String, String, String, String)>(
        "SELECT m.id, sender.username, recipient.username,
                m.subject, m.body, m.sent_at
         FROM mails m
         JOIN users sender    ON m.from_id = sender.username
         JOIN users recipient ON m.to_id   = recipient.username
         WHERE m.id = ? AND m.to_id = ?",
    )
    .bind(mail_id)
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let (id, from, to, subject, body, sent_at) = row;

    // Mark the mail as read now that it has been fetched.
    sqlx::query("UPDATE mails SET is_read = 1 WHERE id = ?")
        .bind(mail_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MailDetail {
        id,
        from: format!("{from}@{MAIL_DOMAIN}"),
        to: format!("{to}@{MAIL_DOMAIN}"),
        subject,
        body,
        sent_at,
        is_read: true,
    }))
}

/// POST /api/mails — send a mail to another user on this server.
#[axum::debug_handler]
pub async fn send_mail(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<SendMailRequest>,
) -> Result<(StatusCode, Json<SendMailResponse>), (StatusCode, String)> {
    // Accept both "user" and "user@monsatan.ctf" as the recipient address.
    let to_username = req
        .to
        .strip_suffix(&format!("@{MAIL_DOMAIN}"))
        .unwrap_or(&req.to);

    // Ensure the recipient exists,
    let to_id = sqlx::query_scalar::<_, String>("SELECT username FROM users WHERE username = ?")
        .bind(to_username)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database error".to_string(),
            )
        })?
        .ok_or((StatusCode::NOT_FOUND, "recipient not found".to_string()))?;

    // Process plugins, returning 400 Bad Request on any plugin error
    let body = if let Some(plugins) = req.plugins {
        match plugins::process_plugins(&req.body, plugins, &state.signing_key.verifying_key()).await
        {
            Ok(body) => body,
            Err(e) => return Err((StatusCode::BAD_REQUEST, e.to_string())),
        }
    } else {
        req.body.clone()
    };

    let sent_at = Utc::now().to_rfc3339();

    let id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO mails (from_id, to_id, subject, body, sent_at)
         VALUES (?, ?, ?, ?, ?)
         RETURNING id",
    )
    .bind(claims.sub)
    .bind(to_id)
    .bind(&req.subject)
    .bind(body)
    .bind(&sent_at)
    .fetch_one(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "database error".to_string(),
        )
    })?;

    Ok((StatusCode::CREATED, Json(SendMailResponse { id })))
}
