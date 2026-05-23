use std::sync::Arc;

use axum::{extract::State, Json};
use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::Hmac;
use rand::Rng;
use subtle::ConstantTimeEq;

use crate::{
    auth,
    db::{self, AppState},
    error::{AppError, Result},
    models::{LoginRequest, RegisterRequest, TokenResponse},
};

/// Number of PBKDF2 iterations used when hashing passwords.
const PBKDF2_ROUNDS: u32 = 1623;

/// Length of the derived key in bytes.
const HASH_LEN: usize = 32;

/// Length of the random salt in bytes.
const SALT_LEN: usize = 16;

// ── Register ──────────────────────────────────────────────────────────────────

/// Register a new user account.
///
/// Validates input, hashes the password with PBKDF2-SHA512, stores the user,
/// and returns a signed JWT.
#[utoipa::path(
    post,
    path = "/api/register",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "Registered successfully", body = TokenResponse),
        (status = 400, description = "Invalid input (empty username or password)"),
        (status = 409, description = "Username already taken"),
    )
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<TokenResponse>> {
    // Validate that required fields are non-empty.
    if req.username.is_empty() {
        return Err(AppError::BadRequest("username must not be empty".into()));
    }
    if req.password.is_empty() {
        return Err(AppError::BadRequest("password must not be empty".into()));
    }

    // Reject if the username is already taken.
    if db::get_user_by_username(&state.db, &req.username)
        .await
        .is_ok()
    {
        return Err(AppError::Conflict(format!(
            "username '{}' is already taken",
            req.username
        )));
    }

    // Generate a random 16-byte salt.
    let salt: [u8; SALT_LEN] = rand::thread_rng().gen();

    // Derive a 32-byte key with PBKDF2-HMAC-SHA512.
    let mut hash_buf = [0u8; HASH_LEN];
    pbkdf2::pbkdf2::<Hmac<sha2::Sha512>>(
        req.password.as_bytes(),
        &salt,
        PBKDF2_ROUNDS,
        &mut hash_buf,
    )
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    // Encode everything as base64 for storage.
    let password_hash_b64 = STANDARD.encode(hash_buf);
    let password_salt_b64 = STANDARD.encode(salt);

    let bio = req.bio.unwrap_or_default();
    // Keep the profile picture as an Option<&str> for the DB helper.
    let profile_picture = req.profile_picture;

    // Persist the user row.
    db::create_user(
        &state.db,
        &req.username,
        &password_hash_b64,
        &password_salt_b64,
        &bio,
        profile_picture,
    )
    .await?;

    // Persist contact info (empty strings for any omitted fields).
    let contact = req.contact.unwrap_or_default();
    db::create_contact_info(
        &state.db,
        &req.username,
        &contact.phone.unwrap_or_default(),
        &contact.email.unwrap_or_default(),
        &contact.location.unwrap_or_default(),
    )
    .await?;

    let token = auth::create_token(&req.username, &state.jwt_secret)?;
    Ok(Json(TokenResponse { token }))
}

// ── Login ─────────────────────────────────────────────────────────────────────

/// Authenticate with an existing account.
///
/// Re-derives the PBKDF2 hash from the supplied password and compares it to
/// the stored hash.  Returns a signed JWT on success.
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Authenticated successfully", body = TokenResponse),
        (status = 401, description = "Invalid username or password"),
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<TokenResponse>> {
    // Treat a missing user the same as a bad password to avoid user enumeration.
    let user = db::get_user_by_username(&state.db, &req.username)
        .await
        .map_err(|_| AppError::Unauthorized)?;

    // Decode the stored salt and re-derive the hash from the supplied password.
    let salt = STANDARD
        .decode(&user.password_salt)
        .map_err(|_| AppError::Unauthorized)?;

    let mut hash_buf = [0u8; HASH_LEN];
    pbkdf2::pbkdf2_hmac::<sha2::Sha512>(
        req.password.as_bytes(),
        &salt,
        PBKDF2_ROUNDS,
        &mut hash_buf,
    );

    let db_hash = STANDARD
        .decode(user.password_pbkdf2_hash)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    if hash_buf.ct_eq(&db_hash).into() {
        let token = auth::create_token(&req.username, &state.jwt_secret)?;
        Ok(Json(TokenResponse { token }))
    } else {
        Err(AppError::Unauthorized)
    }
}
