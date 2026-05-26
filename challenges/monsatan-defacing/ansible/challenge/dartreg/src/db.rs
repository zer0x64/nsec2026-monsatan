//! Database initialization, schema setup, seed data, and all query functions.
//!
//! All SQL lives here. Route handlers interact with the database exclusively
//! through the typed functions below, never by writing raw queries themselves.
//!
//! On every startup, `init_db` runs `seed.sql` (embedded at compile time via
//! `include_str!`) to populate the database with initial data. All statements
//! in that file use `CREATE TABLE IF NOT EXISTS` and `INSERT OR IGNORE`, so
//! re-running against an already-populated database is safe.

use sqlx::{sqlite::SqliteConnectOptions, Row, SqlitePool};
use std::str::FromStr;

// ---------------------------------------------------------------------------
// Row types returned to callers
// ---------------------------------------------------------------------------

/// A version record as stored in the database.
/// Callers are responsible for constructing the full `archive_url` from
/// `archive_filename` and the server's base URL.
pub struct VersionRecord {
    pub version: String,
    pub archive_filename: String,
    pub archive_sha256: String,
    /// `pubspec.yaml` contents serialized as a JSON string.
    pub pubspec_json: String,
}

/// A security advisory record as stored in the database.
/// `osv_json` is the full OSV-format advisory serialized as a JSON string.
/// Insert rows manually via SQLite to add advisories for local packages.
pub struct AdvisoryRecord {
    /// OSV advisory identifier (e.g. `GHSA-xxxx-xxxx-xxxx`).
    pub id: String,
    /// The OSV advisory as a raw JSON string.
    pub osv_json: String,
}

// ---------------------------------------------------------------------------
// Initialization
// ---------------------------------------------------------------------------

/// SQL seed file, embedded at compile time.
/// Path is relative to this source file (src/db.rs → ../seed.sql).
const SEED_SQL: &str = include_str!("../seed.sql");

/// Initialize the SQLite connection pool, create all tables, and seed initial
/// data. The database file is created automatically when `db_path` is absent.
pub async fn init_db(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let options =
        SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    // Packages are identified solely by name.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS packages (
            name TEXT PRIMARY KEY
        )",
    )
    .execute(&pool)
    .await?;

    // Each row represents one published version of a package.
    // archive_filename is a UUID-based name on disk, not the original filename.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS versions (
            id               INTEGER PRIMARY KEY AUTOINCREMENT,
            package_name     TEXT    NOT NULL REFERENCES packages(name),
            version          TEXT    NOT NULL,
            archive_filename TEXT    NOT NULL,
            archive_sha256   TEXT    NOT NULL,
            pubspec_json     TEXT    NOT NULL,
            created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(package_name, version)
        )",
    )
    .execute(&pool)
    .await?;

    // Tracks uploads that have been received but not yet finalized.
    // Rows are deleted once finalize_upload succeeds.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS pending_uploads (
            upload_id        TEXT PRIMARY KEY,
            archive_filename TEXT NOT NULL,
            created_at       DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await?;

    // Stores security advisories in OSV format for locally published packages.
    // Rows are intended to be inserted manually via the SQLite CLI or a DB
    // browser. Each row represents one advisory affecting one package.
    //
    // Example insertion:
    //   INSERT INTO advisories (id, package_name, osv_json) VALUES (
    //     'GHSA-xxxx-xxxx-xxxx',
    //     'mypkg',
    //     '{"id":"GHSA-xxxx-xxxx-xxxx","affected":[{"package":{"name":"mypkg"},"versions":["1.0.0"]}]}'
    //   );
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS advisories (
            id           TEXT PRIMARY KEY,
            package_name TEXT     NOT NULL REFERENCES packages(name),
            osv_json     TEXT     NOT NULL,
            updated_at   DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await?;

    seed_db(&pool).await?;

    Ok(pool)
}

/// Execute the embedded `seed.sql` against the pool.
///
/// The seed file is idempotent (`CREATE TABLE IF NOT EXISTS` + `INSERT OR
/// IGNORE`), so this is safe to call on every startup.
async fn seed_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(SEED_SQL).execute(pool).await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Package version queries
// ---------------------------------------------------------------------------

pub async fn get_packages_names(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT name
         FROM packages
         ORDER BY name ASC",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|row| row.get("name")).collect())
}

/// Return all locally published versions of `package_name`, oldest first.
/// An empty `Vec` means the package is not known locally.
pub async fn get_package_versions(
    pool: &SqlitePool,
    package_name: &str,
) -> Result<Vec<VersionRecord>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT version, archive_filename, archive_sha256, pubspec_json
         FROM versions
         WHERE package_name = ?
         ORDER BY created_at ASC",
    )
    .bind(package_name)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| VersionRecord {
            version: row.get("version"),
            archive_filename: row.get("archive_filename"),
            archive_sha256: row.get("archive_sha256"),
            pubspec_json: row.get("pubspec_json"),
        })
        .collect())
}

/// Return the archive filename for a specific package version, or `None` if
/// the version is not stored locally.
pub async fn get_version_filename(
    pool: &SqlitePool,
    package_name: &str,
    version: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT archive_filename FROM versions
         WHERE package_name = ? AND version = ?",
    )
    .bind(package_name)
    .bind(version)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.get("archive_filename")))
}

/// Ensure a package row exists. Safe to call even if the package was already
/// inserted previously (uses `INSERT OR IGNORE`).
pub async fn upsert_package(pool: &SqlitePool, name: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT OR IGNORE INTO packages (name) VALUES (?)")
        .bind(name)
        .execute(pool)
        .await?;
    Ok(())
}

/// Insert a new version row.
///
/// Returns the raw `sqlx::Error` on failure so that callers can inspect it for
/// a UNIQUE constraint violation (meaning the version was already published).
pub async fn insert_version(
    pool: &SqlitePool,
    package_name: &str,
    version: &str,
    archive_filename: &str,
    archive_sha256: &str,
    pubspec_json: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO versions
             (package_name, version, archive_filename, archive_sha256, pubspec_json)
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(package_name)
    .bind(version)
    .bind(archive_filename)
    .bind(archive_sha256)
    .bind(pubspec_json)
    .execute(pool)
    .await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Advisory queries
// ---------------------------------------------------------------------------

/// Return all advisories for `package_name`, ordered by `updated_at` ascending.
/// An empty `Vec` means no advisories are stored for that package.
pub async fn get_advisories_for_package(
    pool: &SqlitePool,
    package_name: &str,
) -> Result<Vec<AdvisoryRecord>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, osv_json, updated_at
         FROM advisories
         WHERE package_name = ?
         ORDER BY updated_at ASC",
    )
    .bind(package_name)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| AdvisoryRecord {
            id: row.get("id"),
            osv_json: row.get("osv_json"),
        })
        .collect())
}

/// Return the most recent `updated_at` timestamp across all advisories for
/// `package_name`, or `None` if no advisories exist for that package.
pub async fn get_advisories_updated_for_package(
    pool: &SqlitePool,
    package_name: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT MAX(updated_at) AS latest
         FROM advisories
         WHERE package_name = ?",
    )
    .bind(package_name)
    .fetch_optional(pool)
    .await?;

    Ok(row.and_then(|r| r.get("latest")))
}

// ---------------------------------------------------------------------------
// Pending upload queries
// ---------------------------------------------------------------------------

/// Record a new pending upload so the finalize step can locate the archive.
pub async fn create_pending_upload(
    pool: &SqlitePool,
    upload_id: &str,
    archive_filename: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO pending_uploads (upload_id, archive_filename) VALUES (?, ?)")
        .bind(upload_id)
        .bind(archive_filename)
        .execute(pool)
        .await?;
    Ok(())
}

/// Return the archive filename for a pending upload, or `None` if the
/// `upload_id` does not exist (or was already finalized).
pub async fn get_pending_upload(
    pool: &SqlitePool,
    upload_id: &str,
) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query("SELECT archive_filename FROM pending_uploads WHERE upload_id = ?")
        .bind(upload_id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| r.get("archive_filename")))
}

/// Delete a pending upload record after a successful finalize.
pub async fn delete_pending_upload(pool: &SqlitePool, upload_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM pending_uploads WHERE upload_id = ?")
        .bind(upload_id)
        .execute(pool)
        .await?;
    Ok(())
}
