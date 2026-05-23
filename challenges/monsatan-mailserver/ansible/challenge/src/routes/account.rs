use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{auth::create_token, constants::MAIL_DOMAIN, AppState};

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub email: String,
}

/// Register a new user and return a JWT.
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<AuthRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, &'static str)> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing failed"))?
        .to_string();

    sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(&body.username)
        .bind(&hash)
        .execute(&state.db)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(e) if e.is_unique_violation() => {
                (StatusCode::CONFLICT, "Username already taken")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
        })?;

    let token = create_token(&body.username, &state.jwt_secret)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed"))?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            email: format!("{}@{}", body.username, MAIL_DOMAIN),
        }),
    ))
}

/// Verify credentials and return a JWT.
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, &'static str)> {
    let row: (String,) = sqlx::query_as("SELECT password_hash FROM users WHERE username = ?")
        .bind(&body.username)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials"))?;

    let hash = row.0;

    let parsed = PasswordHash::new(&hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid stored hash"))?;

    Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials"))?;

    let token = create_token(&body.username, &state.jwt_secret)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed"))?;

    Ok(Json(AuthResponse {
        token,
        email: format!("{}@{}", body.username, MAIL_DOMAIN),
    }))
}
