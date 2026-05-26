//! Data models for pub repository API v2 responses.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

/// A single version entry within a package listing response.
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct PackageVersion {
    /// The semver version string, e.g. `"1.2.3"`.
    pub version: String,

    /// Absolute URL from which the `.tar.gz` archive can be downloaded.
    pub archive_url: String,

    /// Hex-encoded SHA-256 checksum of the archive (optional per spec).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_sha256: Option<String>,

    /// Full contents of `pubspec.yaml` serialized as a JSON object.
    pub pubspec: Value,
}

#[derive(Serialize, ToSchema)]
pub struct AllPackagesListResponse {
    pub packages: Vec<String>,
}

/// Response body for `GET /api/packages/:package`.
#[derive(Serialize, ToSchema)]
pub struct PackageListResponse {
    /// The package name.
    pub name: String,

    /// The most recently published version (used by the client as the default).
    pub latest: PackageVersion,

    /// All published versions, in ascending publication order.
    pub versions: Vec<PackageVersion>,

    /// Timestamp of the last time the advisories endpoint changed for this
    /// package. Its presence signals to the client that the advisories API
    /// is supported by this server (optional per spec).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "advisoriesUpdated")]
    pub advisories_updated: Option<String>,
}

/// Response body for `GET /api/packages/:package/advisories`.
///
/// `advisories` holds a list of security advisories in OSV format
/// (see <https://ossf.github.io/osv-schema/>).
/// `Value` is used for each entry so that the full OSV object is preserved
/// without needing to model every field.
#[derive(Serialize, ToSchema)]
pub struct AdvisoriesResponse {
    /// List of OSV-format security advisories affecting this package.
    /// Empty when no advisories are known.
    pub advisories: Vec<Value>,

    /// ISO 8601 / RFC 3339 timestamp of the last change to this package's
    /// advisory list. Used by clients for caching.
    #[serde(rename = "advisoriesUpdated")]
    pub advisories_updated: String,
}
