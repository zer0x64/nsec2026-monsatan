use base64::prelude::*;
use ed25519_dalek::SigningKey;
use rand::RngCore;
use sqlx::{mysql::MySqlConnectOptions, MySqlPool};
use std::str::FromStr;

/// Initialises the SQLite pool, creates all tables, and returns the JWT secret.
pub async fn init_db(database_url: &str) -> Result<(MySqlPool, Vec<u8>, SigningKey), sqlx::Error> {
    let options = MySqlConnectOptions::from_str(database_url)?;
    let pool = MySqlPool::connect_with(options).await?;

    let secret = get_or_create_secret(&pool).await?;
    let signing_key = get_or_create_signing_key(&pool).await?;

    Ok((pool, secret, signing_key))
}

/// Returns the stored JWT secret, or generates and persists a new one.
async fn get_or_create_secret(pool: &MySqlPool) -> Result<Vec<u8>, sqlx::Error> {
    if let Some(secret) =
        sqlx::query_scalar::<_, String>("SELECT value FROM config WHERE name = 'jwt_secret'")
            .fetch_optional(pool)
            .await?
    {
        return Ok(BASE64_STANDARD
            .decode(secret)
            .expect("Coudln't decode JWT secret!"));
    }

    let mut secret = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret);

    let secret_encoded = BASE64_STANDARD.encode(&secret);

    sqlx::query("INSERT INTO config (name, value) VALUES ('jwt_secret', ?)")
        .bind(&secret_encoded)
        .execute(pool)
        .await?;

    Ok(secret)
}

async fn get_or_create_signing_key(pool: &MySqlPool) -> Result<SigningKey, sqlx::Error> {
    if let Some(key) =
        sqlx::query_scalar::<_, String>("SELECT value FROM config WHERE name = 'signing_key'")
            .fetch_optional(pool)
            .await?
    {
        return Ok(SigningKey::from_bytes(
            &BASE64_STANDARD
                .decode(&key)
                .expect("Couldn't base64 decode signing key!")
                .try_into()
                .expect("Signing key is the wrong size!"),
        ));
    }

    // Signing key is hardcoded so we can sign our plugins. Still put it in the DB for consistency.
    let key = SigningKey::from_bytes(&crate::constants::SIGNING_KEY);

    let key_encoded = BASE64_STANDARD.encode(key.as_bytes());

    sqlx::query("INSERT INTO config (name, value) VALUES ('signing_key', ?)")
        .bind(&key_encoded)
        .execute(pool)
        .await?;

    Ok(key)
}
