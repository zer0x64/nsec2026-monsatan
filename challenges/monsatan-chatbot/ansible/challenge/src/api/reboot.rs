use axum::{http::StatusCode, response::IntoResponse, Json};
use std::process::Command;

/// POST /api/reboot
///
/// Triggers a system reboot via `systemctl reboot`.
pub async fn reboot() -> impl IntoResponse {
    let result = Command::new("systemctl").arg("reboot").status();

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "Rebooting..." })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to reboot: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to initiate reboot" })),
            )
                .into_response()
        }
    }
}
