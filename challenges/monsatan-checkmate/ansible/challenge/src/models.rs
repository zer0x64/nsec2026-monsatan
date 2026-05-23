use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ── Requests ─────────────────────────────────────────────────────────────────

/// Optional contact details supplied at registration time.
#[derive(Debug, Default, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContactRequest {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub location: Option<String>,
}

/// Body for `POST /api/register`.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub bio: Option<String>,
    /// Optional profile picture, supplied as a base64-encoded string.
    pub profile_picture: Option<String>,
    pub contact: Option<ContactRequest>,
}

/// Body for `POST /api/login`.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Body for `POST /api/user/:username/swipe`.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SwipeRequest {
    pub liked: bool,
}

/// Body for `POST /api/user/:username/dm`.
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SendMessageRequest {
    pub content: String,
}

// ── Responses ─────────────────────────────────────────────────────────────────

/// Returned after a successful register or login.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub token: String,
}

/// Public profile information for a user.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub username: String,
    pub bio: String,
    pub rating: i64,
    /// Base64-encoded profile picture, if one was set.
    pub profile_picture: Option<String>,
}

/// Private contact information, only visible to matched users.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContactResponse {
    pub phone: String,
    pub email: String,
    pub location: String,
}

/// List of usernames that mutually liked the authenticated user.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MatchesResponse {
    pub matches: Vec<String>,
}

/// A single direct message.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub id: i64,
    pub sender: String,
    pub content: String,
    pub sent_at: String,
}

/// Result of a swipe action.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SwipeResponse {
    /// Whether this swipe created a mutual match.
    pub matched: bool,
}
