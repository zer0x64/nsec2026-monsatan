//! Database initialisation and shared application state.

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

use crate::seed::seed_orders_if_empty;

/// Shared application state injected into every route handler via axum's
/// [`axum::extract::State`] extractor.
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

/// Initialise the SQLite database at `db_path`, creating the file and schema
/// if they do not already exist.
///
/// If the `orders` table is freshly created (empty), mock seed data is
/// inserted to provide a realistic starting dataset.
pub async fn init_db(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let options =
        SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    // Items are stored as a JSON blob to avoid a separate line-items table.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id               INTEGER PRIMARY KEY AUTOINCREMENT,
            customer_name    TEXT    NOT NULL,
            customer_email   TEXT    NOT NULL,
            shipping_address TEXT    NOT NULL,
            items            TEXT    NOT NULL,  -- JSON-encoded Vec<OrderItem>
            total_cents      INTEGER NOT NULL,
            created_at       TEXT    NOT NULL   -- ISO 8601
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Populate mock orders the first time the database is created.
    seed_orders_if_empty(&pool).await?;

    Ok(pool)
}
