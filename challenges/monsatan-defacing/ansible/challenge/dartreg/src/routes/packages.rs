//! Handler for `GET /api/packages/:package`.
//!
//! Returns all locally known versions of a package. If none are found in the
//! database, the request is transparently forwarded to pub.dev.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{header, HeaderMap},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;

use crate::{
    config::{PUB_DEV_BASE_URL, PUB_V2_CONTENT_TYPE},
    db,
    error::AppError,
    models::{AllPackagesListResponse, PackageListResponse, PackageVersion},
    state::AppState,
};

/// Timestamp used as `advisoriesUpdated` when no advisories exist for a package.
const EPOCH_TIMESTAMP: &str = "1970-01-01T00:00:00Z";

/// `GET /api/packages`
///
/// Lists all local package names.
#[utoipa::path(
    get,
    path = "/api/packages",
    responses(
        (status = 200, body = AllPackagesListResponse, description = "Packages found"),
        (status = 401, description = "Auth token not provided or invalid"),
        (status = 404, description = "No packages found!"),
    ),
)]
pub async fn list_packages(State(state): State<Arc<AppState>>) -> Result<Response, AppError> {
    let packages = db::get_packages_names(&state.db).await?;

    if packages.is_empty() {
        Err(AppError::NotFound("No packages found!".to_string()))
    } else {
        Ok(Json(AllPackagesListResponse { packages }).into_response())
    }
}

/// `GET /api/packages/:package`
///
/// Lists all versions of a package. If the package has no locally published
/// versions, the response from pub.dev is returned as-is.
#[utoipa::path(
    get,
    path = "/api/packages/{package}",
    responses(
        (status = 200, body = PackageListResponse, description = "Package found"),
        (status = 401, description = "Auth token not provided or invalid"),
        (status = 404, description = "Package not found")),
    params(
        ("package" = String, Path, description = "Package name")
    )
)]
pub async fn list_package_versions(
    State(state): State<Arc<AppState>>,
    Path(package): Path<String>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let records = db::get_package_versions(&state.db, &package).await?;

    if records.is_empty() {
        // No local versions — proxy the request upstream to pub.dev.
        return proxy_to_pubdev(&state, &package).await;
    }

    let versions: Vec<PackageVersion> = records
        .into_iter()
        .filter_map(|record| {
            let pubspec: Value = serde_json::from_str(&record.pubspec_json)
                .unwrap_or(Value::Object(Default::default()));

            let host_header = match headers.get("host") {
                Some(h) => h.to_str().ok(),
                None => None,
            };

            if let Some(host_header) = host_header {
                Some(PackageVersion {
                    version: record.version,
                    // Point the client at our own artifact-serving endpoint.
                    archive_url: format!("https://{}/artifacts/{}", host_header, record.archive_filename),
                    archive_sha256: Some(record.archive_sha256),
                    pubspec,
                })
            } else {
                None
            }
        })
        .collect();

    // The latest version is the most recently inserted one (last in ASC order).
    let latest = versions
        .last()
        .cloned()
        .expect("records is non-empty, guaranteed above");

    // Include the most recent advisory timestamp for this package so the client
    // knows the advisories endpoint is supported and can cache-bust correctly.
    let advisories_updated = db::get_advisories_updated_for_package(&state.db, &package)
        .await?
        .or_else(|| Some(EPOCH_TIMESTAMP.to_string()));

    let body = PackageListResponse {
        name: package,
        latest,
        versions,
        advisories_updated,
    };

    Ok(([(header::CONTENT_TYPE, PUB_V2_CONTENT_TYPE)], Json(body)).into_response())
}

/// Forward a package listing request to pub.dev and relay the JSON response.
///
/// The archive URLs in the forwarded response already point to pub.dev, so the
/// client will download archives directly from there — no local proxy needed.
async fn proxy_to_pubdev(state: &AppState, package: &str) -> Result<Response, AppError> {
    let url = format!("{}/api/packages/{}", PUB_DEV_BASE_URL, package);

    let resp = state
        .http_client
        .get(&url)
        .header("Accept", PUB_V2_CONTENT_TYPE)
        .send()
        .await?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(AppError::NotFound(format!(
            "Package '{}' not found locally or on pub.dev.",
            package
        )));
    }

    if !resp.status().is_success() {
        return Err(AppError::Internal(format!(
            "pub.dev returned unexpected status {} for package '{}'",
            resp.status(),
            package
        )));
    }

    let body: Value = resp.json().await?;

    Ok(([(header::CONTENT_TYPE, PUB_V2_CONTENT_TYPE)], Json(body)).into_response())
}
