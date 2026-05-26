//! Handlers for the package publishing flow.
//!
//! Publishing a package requires three steps per the pub repository spec v2:
//!
//! 1. `GET  /api/packages/versions/new`
//!    → Returns the multipart upload URL and any extra form fields.
//!
//! 2. `POST /api/packages/versions/new/upload`
//!    → Accepts the `.tar.gz` archive as a multipart `file` field.
//!      Saves the archive to disk and records a pending upload row.
//!      Responds `204 No Content` with a `Location` pointing to step 3.
//!
//! 3. `GET  /api/packages/versions/new/finalize/:upload_id`
//!    → Reads the pending archive, extracts `pubspec.yaml`, computes the
//!      SHA-256 checksum, inserts the version into the database, and removes
//!      the pending-upload row.

use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Multipart, Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use flate2::read::GzDecoder;
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use tar::Archive;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{config::PUB_V2_CONTENT_TYPE, db, error::AppError, state::AppState};

#[derive(Serialize, ToSchema)]
struct UploadUrlResponse {
    // URL to post the archive to
    url: String,
    // Extra form fields required by standard
    fields: HashMap<String, String>,
}

#[derive(Serialize, ToSchema)]
struct FinalizeResponse {
    // Success response
    success: FinalizeSuccess,
}

#[derive(Serialize, ToSchema)]
struct FinalizeSuccess {
    // Success message
    message: String,
}

// ---------------------------------------------------------------------------
// Step 1 – return the upload URL
// ---------------------------------------------------------------------------

/// `GET /api/packages/versions/new`
///
/// Returns the URL the client should POST the archive to, plus any extra
/// form fields required. We use no extra fields, so `fields` is empty.
#[utoipa::path(
    get,
    path = "/api/packages/versions/new",
    responses(
        (status = 200, body = UploadUrlResponse, description = "Package upload record created"),
        (status = 401, description = "Auth token not provided or invalid"),
    )
)]
pub async fn get_upload_url(headers: HeaderMap) -> Result<Response, AppError> {
    let host_header = match headers.get("host") {
        Some(h) => h
            .to_str()
            .map_err(|_| AppError::BadRequest("Invalid host header!".to_string()))?,
        None => return Err(AppError::BadRequest("Missing 'host' header".to_string()))?,
    };

    let upload_url = format!("https://{}/api/packages/versions/new/upload", host_header);

    let body = UploadUrlResponse {
        url: upload_url,
        fields: HashMap::new(),
    };

    Ok(([(header::CONTENT_TYPE, PUB_V2_CONTENT_TYPE)], Json(body)).into_response())
}

// ---------------------------------------------------------------------------
// Step 2 – receive the multipart archive
// ---------------------------------------------------------------------------

/// `POST /api/packages/versions/new/upload`
///
/// Expects a `multipart/form-data` body with a field named `file` containing
/// the gzipped TAR archive. The archive is written to disk under a random UUID
/// filename. A `pending_uploads` row is created so the finalize step can
/// locate the file.
#[utoipa::path(
    post,
    path = "/api/packages/versions/new/upload",
    responses(
        (status = 204, description = "Data uploaded successfully"),
        (status = 400, description = "Invalid request body"),
        (status = 401, description = "Auth token not provided or invalid"),
    ),
    request_body(content_type = "multipart/form-data", description = "GZipped TAR archive of the package")
)]
pub async fn upload_package(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let host_header = match headers.get("host") {
        Some(h) => h
            .to_str()
            .map_err(|_| AppError::BadRequest("Invalid host header!".to_string()))?,
        None => return Err(AppError::BadRequest("Missing 'host' header".to_string()))?,
    };

    // Pull the `file` field from the multipart body.
    let mut archive_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read multipart field: {e}")))?
    {
        if field.name() == Some("file") {
            let bytes = field
                .bytes()
                .await
                .map_err(|e| AppError::BadRequest(format!("Failed to read file field: {e}")))?;
            archive_bytes = Some(bytes.to_vec());
            break;
        }
    }

    let bytes = archive_bytes
        .ok_or_else(|| AppError::BadRequest("Missing 'file' field in multipart upload.".into()))?;

    // Generate a random UUID filename so the original name is never trusted.
    let upload_id = Uuid::new_v4().to_string();
    let archive_filename = format!("{}.tar.gz", Uuid::new_v4());
    let archive_path = state.artifacts_dir.join(&archive_filename);

    tokio::fs::write(&archive_path, &bytes).await?;

    // Record the pending upload so the finalize step can look it up.
    db::create_pending_upload(&state.db, &upload_id, &archive_filename).await?;

    let finalize_url = format!(
        "https://{}/api/packages/versions/new/finalize/{}",
        host_header, upload_id
    );

    // Per spec: respond 204 No Content with Location pointing to the finalize URL.
    Ok((StatusCode::NO_CONTENT, [(header::LOCATION, finalize_url)]).into_response())
}

// ---------------------------------------------------------------------------
// Step 3 – finalize and persist the version
// ---------------------------------------------------------------------------

/// `GET /api/packages/versions/new/finalize/:upload_id`
///
/// Reads the pending archive, extracts `pubspec.yaml`, computes its SHA-256
/// checksum, persists the package version to the database, and removes the
/// pending-upload row on success.
#[utoipa::path(
    get,
    path = "/api/packages/versions/new/finalize/{upload_id}",
    responses(
        (status = 200, description = "Package published successfully"),
        (status = 401, description = "Auth token not provided or invalid"),
        (status = 404, description = "ID not found")),
    params(
        ("upload_id" = String, Path, description="Upload ID")
    )
)]
pub async fn finalize_upload(
    State(state): State<Arc<AppState>>,
    Path(upload_id): Path<String>,
) -> Result<Response, AppError> {
    // Look up the pending upload record.
    let archive_filename = db::get_pending_upload(&state.db, &upload_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Upload not found or already finalized.".into()))?;

    let archive_path = state.artifacts_dir.join(&archive_filename);

    // Read the saved archive from disk.
    let bytes = tokio::fs::read(&archive_path).await?;

    // Compute SHA-256 over the raw archive bytes.
    let sha256 = compute_sha256(&bytes);

    // Extract pubspec.yaml in a blocking thread (synchronous tar/gz APIs).
    let pubspec = tokio::task::spawn_blocking(move || extract_pubspec(&bytes))
        .await
        .map_err(|e| AppError::Internal(format!("spawn_blocking join error: {e}")))??;

    let (pkg_name, pkg_version) = get_name_and_version(&pubspec)?;

    let pubspec_json = serde_json::to_string(&pubspec)
        .map_err(|e| AppError::Internal(format!("Failed to serialize pubspec: {e}")))?;

    // Ensure the package row exists (idempotent).
    db::upsert_package(&state.db, &pkg_name).await?;

    // Insert the new version, checking for duplicates.
    let insert_result = db::insert_version(
        &state.db,
        &pkg_name,
        &pkg_version,
        &archive_filename,
        &sha256,
        &pubspec_json,
    )
    .await;

    // Translate a UNIQUE constraint violation into a user-facing Conflict error.
    if let Err(sqlx::Error::Database(ref db_err)) = insert_result {
        if db_err.is_unique_violation() {
            return Err(AppError::Conflict(format!(
                "Version {} of package '{}' is already published.",
                pkg_version, pkg_name
            )));
        }
    }
    insert_result?;

    // Remove the pending-upload record now that everything succeeded.
    db::delete_pending_upload(&state.db, &upload_id).await?;

    let body = FinalizeResponse {
        success: FinalizeSuccess {
            message: format!("Successfully published {}@{}.", pkg_name, pkg_version),
        },
    };

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, PUB_V2_CONTENT_TYPE)],
        Json(body),
    )
        .into_response())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Compute the hex-encoded SHA-256 digest of `data`.
fn compute_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Extract `pubspec.yaml` from a gzipped TAR archive and return it as a
/// `serde_json::Value` (converted from YAML).
///
/// Dart archives have the layout `<package>-<version>/pubspec.yaml`, so we
/// simply look for any entry whose filename component is `pubspec.yaml`.
fn extract_pubspec(archive_bytes: &[u8]) -> Result<Value, AppError> {
    let gz = GzDecoder::new(archive_bytes);
    let mut archive = Archive::new(gz);

    for entry in archive
        .entries()
        .map_err(|e| AppError::BadRequest(format!("Failed to read archive entries: {e}")))?
    {
        let mut entry =
            entry.map_err(|e| AppError::BadRequest(format!("Invalid archive entry: {e}")))?;

        let path = entry
            .path()
            .map_err(|e| AppError::BadRequest(format!("Invalid entry path: {e}")))?
            .into_owned();

        if path.file_name() == Some(std::ffi::OsStr::new("pubspec.yaml")) {
            use std::io::Read;
            let mut content = String::new();
            entry
                .read_to_string(&mut content)
                .map_err(|e| AppError::BadRequest(format!("Failed to read pubspec.yaml: {e}")))?;

            // Parse YAML then re-encode as JSON so we store a canonical format.
            let yaml: serde_yaml::Value = serde_yaml::from_str(&content)
                .map_err(|e| AppError::BadRequest(format!("Invalid pubspec.yaml: {e}")))?;

            let json = serde_json::to_value(yaml)
                .map_err(|e| AppError::Internal(format!("YAML→JSON conversion failed: {e}")))?;

            return Ok(json);
        }
    }

    Err(AppError::BadRequest(
        "pubspec.yaml not found in the uploaded archive.".into(),
    ))
}

/// Extract the `name` and `version` fields from a parsed pubspec JSON value.
fn get_name_and_version(pubspec: &Value) -> Result<(String, String), AppError> {
    let name = pubspec["name"]
        .as_str()
        .ok_or_else(|| AppError::BadRequest("pubspec.yaml is missing the 'name' field.".into()))?
        .to_string();

    let version = pubspec["version"]
        .as_str()
        .ok_or_else(|| AppError::BadRequest("pubspec.yaml is missing the 'version' field.".into()))?
        .to_string();

    Ok((name, version))
}
