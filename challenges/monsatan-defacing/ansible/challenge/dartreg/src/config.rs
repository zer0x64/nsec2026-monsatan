//! Server-wide constants. Edit these to configure the server defaults.

/// The hardcoded API key for authenticating all requests.
/// Clients must run `dart pub token add <hosted-url>` and enter this value.
pub const API_KEY: &str = "DARTREG_74ea1036f817ffaf8a45001750c05fb0";

/// Default path to the SQLite database file.
pub const DEFAULT_DB_PATH: &str = "dartreg.db";

/// Default directory where uploaded package archives are stored on disk.
pub const DEFAULT_ARTIFACTS_DIR: &str = "artifacts";

/// The pub.dev base URL used when proxying unfound packages upstream.
pub const PUB_DEV_BASE_URL: &str = "https://pub.dev";

/// Content-Type header value required by the pub repository spec v2.
pub const PUB_V2_CONTENT_TYPE: &str = "application/vnd.pub.v2+json";
