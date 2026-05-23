use base64::prelude::*;
use rand::prelude::*;
use serde::Serialize;
use sqlx::{sqlite::SqlitePool, FromRow};
use utoipa::ToSchema;

use crate::error::{AppError, Result};

// ── Application state ─────────────────────────────────────────────────────────

/// Shared application state passed to every Axum handler via `State<Arc<AppState>>`.
pub struct AppState {
    /// SQLite connection pool.
    pub db: SqlitePool,
    /// Secret used to sign and verify JWT tokens.
    pub jwt_secret: Vec<u8>,
}

// ── Database row types ────────────────────────────────────────────────────────

/// Raw database row from the `users` table.
#[derive(Debug, FromRow, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRow {
    pub username: String,
    /// Base64-encoded PBKDF2-SHA512 hash of the password.
    pub password_pbkdf2_hash: String,
    /// Base64-encoded random salt used for hashing.
    pub password_salt: String,
    pub bio: String,
    pub rating: i64,
    /// Optional profile picture stored as a base64-encoded string.
    pub profile_picture: Option<String>,
}

/// Raw database row from the `contact_info` table.
#[derive(Debug, FromRow)]
pub struct ContactRow {
    #[allow(dead_code)]
    pub username: String,
    pub phone: String,
    pub email: String,
    pub location: String,
}

/// Raw database row from the `messages` table.
#[derive(Debug, FromRow)]
pub struct MessageRow {
    pub id: i64,
    pub sender: String,
    #[allow(dead_code)]
    pub recipient: String,
    pub content: String,
    pub sent_at: String,
}

// ── Schema initialisation ─────────────────────────────────────────────────────

/// Create all tables if they do not yet exist.
///
/// Call this once at startup before serving any requests.
pub async fn init_db(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS config (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            username         TEXT PRIMARY KEY,
            password_pbkdf2_hash TEXT NOT NULL,
            password_salt    TEXT NOT NULL,
            bio              TEXT NOT NULL DEFAULT '',
            rating           INTEGER NOT NULL DEFAULT 1200,
            profile_picture  TEXT
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS contact_info (
            username TEXT PRIMARY KEY REFERENCES users(username) ON DELETE CASCADE,
            phone    TEXT NOT NULL DEFAULT '',
            email    TEXT NOT NULL DEFAULT '',
            location TEXT NOT NULL DEFAULT ''
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS swipes (
            swiper TEXT NOT NULL REFERENCES users(username) ON DELETE CASCADE,
            swiped TEXT NOT NULL REFERENCES users(username) ON DELETE CASCADE,
            liked  INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (swiper, swiped)
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            sender    TEXT NOT NULL REFERENCES users(username) ON DELETE CASCADE,
            recipient TEXT NOT NULL REFERENCES users(username) ON DELETE CASCADE,
            content   TEXT NOT NULL,
            sent_at   TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ── Seeding ───────────────────────────────────────────────────────────────────

/// Seed the database with initial users if the `users` table is empty.
///
/// The seed data lives in `seed.sql` (embedded at compile time via
/// [`include_str!`]) and uses `INSERT OR IGNORE` so it is safe to call on
/// an already-populated database.
pub async fn seed_db(pool: &SqlitePool) -> Result<()> {
    // Only seed when there are no users yet.
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Ok(());
    }

    // The SQL file is baked into the binary at compile time.
    let sql = include_str!("seed.sql");

    sqlx::raw_sql(sql).execute(pool).await?;

    tracing::info!("database seeded with initial users");

    seed_profile_pic(
        pool,
        "tactical_tanya",
        include_bytes!("profile_pics/tactical_tanya.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "blundering_bob",
        include_bytes!("profile_pics/blundering_bob.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "queen_gambette",
        include_bytes!("profile_pics/queen_gambette.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "rook_rupert",
        include_bytes!("profile_pics/rook_rupert.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "knight_nadia",
        include_bytes!("profile_pics/knight_nadia.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "pawn_storm_pat",
        include_bytes!("profile_pics/pawn_storm_pat.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "fianchetto_fio",
        include_bytes!("profile_pics/fianchetto_fio.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "en_passant_ed",
        include_bytes!("profile_pics/en_passant_ed.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "bishop_pair_bo",
        include_bytes!("profile_pics/bishop_pair_bo.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "fork_felix",
        include_bytes!("profile_pics/fork_felix.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "zwischenzug_zoe",
        include_bytes!("profile_pics/zwischenzug_zoe.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "skewer_simon",
        include_bytes!("profile_pics/skewer_simon.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "pin_priya",
        include_bytes!("profile_pics/pin_priya.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "zugzwang_zelda",
        include_bytes!("profile_pics/zugzwang_zelda.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "overload_omar",
        include_bytes!("profile_pics/overload_omar.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "tempo_tomas",
        include_bytes!("profile_pics/tempo_tomas.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "deflect_diana",
        include_bytes!("profile_pics/deflect_diana.png"),
    )
    .await?;
    seed_profile_pic(
        pool,
        "checkmate_chad",
        include_bytes!("profile_pics/checkmate_chad.png"),
    )
    .await?;
    seed_profile_pic(pool, "rao", include_bytes!("profile_pics/rao.png")).await?;
    seed_profile_pic(pool, "MrRook", include_bytes!("profile_pics/profile.png")).await?;

    Ok(())
}

async fn seed_profile_pic(pool: &SqlitePool, username: &str, profile_pic: &[u8]) -> Result<()> {
    let profile_pic_b64 = BASE64_STANDARD.encode(profile_pic);
    sqlx::query("UPDATE users SET profile_picture = ? WHERE username = ?")
        .bind(profile_pic_b64)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}

// ── User helpers ──────────────────────────────────────────────────────────────

/// Insert a new user row.
///
/// `password_hash_b64` and `password_salt_b64` are base64-encoded strings
/// produced by the registration handler.  `profile_picture` is an optional
/// base64-encoded image string supplied by the caller.
pub async fn create_user(
    pool: &SqlitePool,
    username: &str,
    password_hash_b64: &str,
    password_salt_b64: &str,
    bio: &str,
    profile_picture: Option<String>,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO users (username, password_pbkdf2_hash, password_salt, bio, profile_picture)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(username)
    .bind(password_hash_b64)
    .bind(password_salt_b64)
    .bind(bio)
    .bind(profile_picture)
    .execute(pool)
    .await?;

    Ok(())
}

/// Insert contact info for an existing user.
///
/// Pass empty strings for any optional fields the caller did not provide.
pub async fn create_contact_info(
    pool: &SqlitePool,
    username: &str,
    phone: &str,
    email: &str,
    location: &str,
) -> Result<()> {
    sqlx::query("INSERT INTO contact_info (username, phone, email, location) VALUES (?, ?, ?, ?)")
        .bind(username)
        .bind(phone)
        .bind(email)
        .bind(location)
        .execute(pool)
        .await?;

    Ok(())
}

/// Fetch a user by username.
///
/// Returns `AppError::NotFound` (via the `From<sqlx::Error>` impl) when the
/// user does not exist.
pub async fn get_user_by_username(pool: &SqlitePool, username: &str) -> Result<UserRow> {
    sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
}

/// Fetch contact info for a user.
///
/// Returns `AppError::NotFound` when no row exists.
pub async fn get_contact_by_username(pool: &SqlitePool, username: &str) -> Result<ContactRow> {
    sqlx::query_as::<_, ContactRow>(
        "SELECT username, phone, email, location FROM contact_info WHERE username = ?",
    )
    .bind(username)
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

// ── Swipe helpers ─────────────────────────────────────────────────────────────

/// Record (or overwrite) a swipe from `swiper` toward `swiped`.
pub async fn record_swipe(
    pool: &SqlitePool,
    swiper: &str,
    swiped: &str,
    liked: bool,
) -> Result<()> {
    sqlx::query("INSERT OR REPLACE INTO swipes (swiper, swiped, liked) VALUES (?, ?, ?)")
        .bind(swiper)
        .bind(swiped)
        .bind(liked as i64)
        .execute(pool)
        .await?;

    Ok(())
}

/// Return `true` when both `user_a` and `user_b` have liked each other.
pub async fn are_matched(pool: &SqlitePool, user_a: &str, user_b: &str) -> Result<bool> {
    // Count rows where each user liked the other (expects exactly 2 for a match).
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM swipes
         WHERE ((swiper = ? AND swiped = ?) OR (swiper = ? AND swiped = ?))
           AND liked = 1",
    )
    .bind(user_a)
    .bind(user_b)
    .bind(user_b)
    .bind(user_a)
    .fetch_one(pool)
    .await?;

    Ok(count == 2)
}

/// Return all usernames that mutually liked `username`.
pub async fn get_matches(pool: &SqlitePool, username: &str) -> Result<Vec<String>> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT a.swiped FROM swipes a
         JOIN swipes b ON a.swiped = b.swiper AND a.swiper = b.swiped
         WHERE a.swiper = ? AND a.liked = 1 AND b.liked = 1",
    )
    .bind(username)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|(name,)| name).collect())
}

/// Return a random user that `username` has not yet swiped on (excluding self).
///
/// Returns `None` when every other user has already been swiped.
pub async fn get_next_user(pool: &SqlitePool, username: &str) -> Result<Option<UserRow>> {
    sqlx::query_as::<_, UserRow>(
        "SELECT * FROM users
         WHERE username != ?
           AND username NOT IN (SELECT swiped FROM swipes WHERE swiper = ?)
         ORDER BY RANDOM()
         LIMIT 1",
    )
    .bind(username)
    .bind(username)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)
}

// ── Message helpers ───────────────────────────────────────────────────────────

/// Insert a message and return the new row's id.
pub async fn insert_message(
    pool: &SqlitePool,
    sender: &str,
    recipient: &str,
    content: &str,
) -> Result<i64> {
    let id = sqlx::query_scalar(
        "INSERT INTO messages (sender, recipient, content) VALUES (?, ?, ?) RETURNING id",
    )
    .bind(sender)
    .bind(recipient)
    .bind(content)
    .fetch_one(pool)
    .await?;

    Ok(id)
}

/// Fetch all messages exchanged between two users, ordered chronologically.
pub async fn get_messages(
    pool: &SqlitePool,
    user_a: &str,
    user_b: &str,
) -> Result<Vec<MessageRow>> {
    sqlx::query_as::<_, MessageRow>(
        "SELECT id, sender, recipient, content, sent_at FROM messages
         WHERE (sender = ? AND recipient = ?) OR (sender = ? AND recipient = ?)
         ORDER BY id ASC",
    )
    .bind(user_a)
    .bind(user_b)
    .bind(user_b)
    .bind(user_a)
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

pub async fn get_or_create_jwt_secret(pool: &SqlitePool) -> Vec<u8> {
    let res: Option<String> =
        sqlx::query_scalar("SELECT value FROM config WHERE key = 'jwt_secret'")
            .fetch_optional(pool)
            .await
            .expect("couldn't fetch jwt secret!");

    match res {
        Some(secret) => BASE64_STANDARD
            .decode(secret)
            .expect("couldn't decode jwt secret!"),
        None => {
            let mut rng = rand::thread_rng();
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);
            let secret = BASE64_STANDARD.encode(bytes);
            sqlx::query("INSERT INTO config (key, value) VALUES ('jwt_secret', ?)")
                .bind(secret)
                .execute(pool)
                .await
                .expect("couldn't store generated jwt secret!");
            bytes.to_vec()
        }
    }
}
