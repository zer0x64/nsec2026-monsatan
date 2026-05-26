//! Handler for the deprecated package download endpoint.
//!
//! `GET /packages/:package/versions/:version.tar.gz`
//!
//! This endpoint is deprecated as of Dart 2.8 but must still be supported for
//! compatibility with older `pub` clients. Servers are allowed to respond with
//! a redirect, which is what we do here:
//!
//! - If the version exists locally → redirect to our `/artifacts/<uuid>.tar.gz`
//! - If not found locally          → redirect to the equivalent pub.dev URL

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{config::PUB_DEV_BASE_URL, db, error::AppError, state::AppState};

/// `GET /packages/:package/versions/:version_tar_gz`
///
/// The `:version_tar_gz` segment is the semver string with a `.tar.gz` suffix
/// appended by the client (e.g. `1.2.3.tar.gz`). We strip that suffix before
/// querying the database.
#[utoipa::path(
    get,
    path = "/packages/{package}/versions/{version_tar_gz}",
    params(("package" = String, Path), ("version_tar_gz" = String, Path)),
    responses((status = 302, description = "Package archive not found, redirecting"),
        (status = 401, description = "Auth token not provided or invalid"),
        (status = 404, description = "Package not found")))
]
pub async fn download_package(
    State(state): State<Arc<AppState>>,
    Path((package, version_tar_gz)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    // Strip the mandatory `.tar.gz` suffix to obtain the bare version string.
    let version = version_tar_gz
        .strip_suffix(".tar.gz")
        .ok_or_else(|| {
            AppError::BadRequest(format!(
                "Expected a version ending in '.tar.gz', got '{}'.",
                version_tar_gz
            ))
        })?
        .to_string();

    let redirect_url = match db::get_version_filename(&state.db, &package, &version).await? {
        // Found locally — redirect to our own artifact-serving endpoint.
        Some(archive_filename) => {
            let host_header = match headers.get("host") {
                Some(h) => h
                    .to_str()
                    .map_err(|_| AppError::BadRequest("Invalid host header!".to_string()))?,
                None => return Err(AppError::BadRequest("Missing 'host' header".to_string()))?,
            };
            format!("{}/artifacts/{}", host_header, archive_filename)
        }

        // Not found locally — redirect to the equivalent pub.dev URL.
        // The client will follow the redirect and download from pub.dev directly.
        None => format!(
            "{}/packages/{}/versions/{}.tar.gz",
            PUB_DEV_BASE_URL, package, version
        ),
    };

    Ok((StatusCode::FOUND, [(header::LOCATION, redirect_url)]).into_response())
}
