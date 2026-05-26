//! Handler for `GET /api/packages/:package/advisories`.
//!
//! Returns security advisories for a package in OSV format
//! (see <https://ossf.github.io/osv-schema/>).
//!
//! Strategy:
//! - If the package has locally published versions, advisories are served
//!   exclusively from the local `advisories` SQLite table. An empty list is
//!   returned when no rows exist — pub.dev is never contacted for local pkgs.
//! - If the package is not known locally, the request is proxied to pub.dev
//!   and the response is relayed verbatim. If pub.dev is unreachable or
//!   returns a non-success status, an empty list is returned gracefully.
//!
//! Advisories for local packages must be inserted manually into the database:
//!
//! ```sql
//! INSERT INTO advisories (id, package_name, osv_json) VALUES (
//!   'GHSA-xxxx-xxxx-xxxx',
//!   'mypkg',
//!   '{"id":"GHSA-xxxx-xxxx-xxxx","affected":[{"package":{"name":"mypkg"},"versions":["1.0.0"]}]}'
//! );
//! ```

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::header,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;

use crate::{
    config::{PUB_DEV_BASE_URL, PUB_V2_CONTENT_TYPE},
    db,
    error::AppError,
    models::AdvisoriesResponse,
    state::AppState,
};

/// Timestamp used as `advisoriesUpdated` when there are no known advisories.
/// The Unix epoch is a safe sentinel: any real advisory will have a later
/// timestamp, so clients will always consider their cache stale after an update.
const EPOCH_TIMESTAMP: &str = "1970-01-01T00:00:00Z";

/// `GET /api/packages/:package/advisories`
///
/// Lists security advisories (OSV format) for the given package.
/// See the module-level documentation for the lookup strategy.
#[utoipa::path(
    get,
    path = "/api/packages/{package}/advisories",
    responses(
        (status = 200, body = AdvisoriesResponse, description = "Advisory list (may be empty)"),
        (status = 401, description = "Auth token not provided or invalid"),
    ),
    params(
        ("package" = String, Path, description = "Package name")
    )
)]
pub async fn list_advisories(
    State(state): State<Arc<AppState>>,
    Path(package): Path<String>,
) -> Result<Response, AppError> {
    // Check whether this package has any locally published versions.
    let local_versions = db::get_package_versions(&state.db, &package).await?;

    if !local_versions.is_empty() {
        // Package is local — serve advisories from the local DB only.
        local_advisories_response(&state, &package).await
    } else {
        // Package is not local — proxy to pub.dev.
        match proxy_advisories_from_pubdev(&state, &package).await {
            Ok(response) => Ok(response),
            Err(err) => {
                tracing::warn!(
                    package = %package,
                    error = ?err,
                    "Could not fetch advisories from pub.dev; returning empty list"
                );
                Ok(empty_advisories_response())
            }
        }
    }
}

/// Build a response from the local `advisories` table for a known package.
///
/// Each stored `osv_json` string is parsed back into a `Value` so it is
/// embedded as a proper JSON object in the response array, not a string.
async fn local_advisories_response(state: &AppState, package: &str) -> Result<Response, AppError> {
    let records = db::get_advisories_for_package(&state.db, package).await?;

    // Determine the most recent update timestamp across all advisories.
    // Fall back to the epoch when the package has no advisories yet.
    let advisories_updated = db::get_advisories_updated_for_package(&state.db, package)
        .await?
        .unwrap_or_else(|| EPOCH_TIMESTAMP.to_string());

    // Parse each stored JSON string into a Value; skip rows that are malformed
    // and log a warning so the operator can fix the bad data.
    let advisories: Vec<Value> = records
        .into_iter()
        .filter_map(
            |record| match serde_json::from_str::<Value>(&record.osv_json) {
                Ok(v) => Some(v),
                Err(err) => {
                    tracing::warn!(
                        advisory_id = %record.id,
                        package = %package,
                        error = ?err,
                        "Skipping advisory: stored osv_json is not valid JSON"
                    );
                    None
                }
            },
        )
        .collect();

    let body = AdvisoriesResponse {
        advisories,
        advisories_updated,
    };

    Ok(([(header::CONTENT_TYPE, PUB_V2_CONTENT_TYPE)], Json(body)).into_response())
}

/// Fetch the advisory list from pub.dev and relay it verbatim.
///
/// Returns an `Err` if the HTTP request fails, the connection times out, or
/// pub.dev returns a non-success status (including 404). The caller is
/// responsible for deciding how to handle the error gracefully.
async fn proxy_advisories_from_pubdev(
    state: &AppState,
    package: &str,
) -> Result<Response, AppError> {
    let url = format!("{}/api/packages/{}/advisories", PUB_DEV_BASE_URL, package);

    let resp = state
        .http_client
        .get(&url)
        .header("Accept", PUB_V2_CONTENT_TYPE)
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(AppError::Internal(format!(
            "pub.dev returned status {} for advisories of '{}'",
            resp.status(),
            package
        )));
    }

    // Relay the response body verbatim so that all OSV fields are preserved
    // without needing to model the full schema locally.
    let body: Value = resp.json().await?;

    Ok(([(header::CONTENT_TYPE, PUB_V2_CONTENT_TYPE)], Json(body)).into_response())
}

/// Build a well-formed response with an empty advisory list.
fn empty_advisories_response() -> Response {
    let body = AdvisoriesResponse {
        advisories: Vec::new(),
        advisories_updated: EPOCH_TIMESTAMP.to_string(),
    };

    ([(header::CONTENT_TYPE, PUB_V2_CONTENT_TYPE)], Json(body)).into_response()
}
