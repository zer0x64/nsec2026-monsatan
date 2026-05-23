use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Response body returned by the `/api/flag` REST endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlagData {
    /// Authenticated employee's username (from the JWT subject claim).
    pub username: String,
    /// The flag string.
    pub flag: String,
}

/// Fetches the flag from the REST endpoint using an `Authorization: Bearer` header.
///
/// Only compiled for the WASM target — the server never calls this function.
/// Reads the JWT from `localStorage`, sends it as a Bearer token, and returns
/// the parsed `FlagData` on success.
#[cfg(feature = "hydrate")]
async fn fetch_flag() -> Result<FlagData, String> {
    use gloo_net::http::Request;

    // Read the JWT that was stored in localStorage after login.
    let token = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|ls| ls.get_item(crate::TOKEN_STORAGE_KEY).ok().flatten())
        .ok_or_else(|| "No session token found".to_string())?;

    let response = Request::get("/api/flag")
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    match response.status() {
        401 => return Err("Your session token is invalid or has expired. Please log in again.".to_string()),
        403 => return Err("Access denied. The flag can only be read by admin@monsatan.ctf.".to_string()),
        _ => {}
    }

    response.json::<FlagData>().await.map_err(|e| e.to_string())
}

/// Clears the stored JWT from `localStorage` and navigates to `/login`.
///
/// Only compiled for the WASM target.
#[cfg(feature = "hydrate")]
fn logout() {
    if let Some(window) = web_sys::window() {
        // Remove the JWT from localStorage.
        if let Ok(Some(ls)) = window.local_storage() {
            let _ = ls.remove_item(crate::TOKEN_STORAGE_KEY);
        }
        // Redirect to the login page.
        let _ = window.location().replace("/login");
    }
}

/// Possible states for the flag page while fetching data.
// Variants are only constructed inside #[cfg(feature = "hydrate")] blocks,
// so the SSR build would otherwise warn about dead code.
#[cfg_attr(not(feature = "hydrate"), allow(dead_code))]
#[derive(Clone)]
enum FlagState {
    /// Still reading localStorage / waiting for the fetch to complete.
    Loading,
    /// Flag data retrieved successfully.
    Ready(FlagData),
    /// Auth failed or fetch error — displays an error message in place.
    Error(String),
}

/// Flag page — displays the supplier procurement data after successful authentication.
///
/// Because the JWT lives in `localStorage` (not a cookie), authentication
/// cannot be checked during SSR.  The page therefore renders a loading
/// placeholder on the server and performs the real auth + data fetch
/// entirely on the client after WASM has loaded.
#[component]
pub fn FlagPage() -> impl IntoView {
    use crate::components::layout::PageShell;

    let (state, set_state) = signal(FlagState::Loading);

    // On the SSR build set_state is never called (the Effect body is gated),
    // so we suppress the unused-variable warning explicitly.
    #[cfg(not(feature = "hydrate"))]
    let _ = set_state;

    // Effect runs once on the client after the component mounts.
    // The entire body is gated so web_sys is never referenced in the SSR build.
    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        leptos::task::spawn_local(async move {
            match fetch_flag().await {
                Ok(data) => set_state.set(FlagState::Ready(data)),
                Err(e) => set_state.set(FlagState::Error(e)),
            }
        });
    });

    view! {
        <PageShell>
            {move || match state.get() {
                FlagState::Loading => view! {
                    <p class="loading">"Authenticating…"</p>
                }.into_any(),

                FlagState::Ready(data) => view! {
                    <div class="flag-card">
                        <h1 class="flag-welcome">
                            "Welcome back, "
                            {data.username.clone()}
                            "."
                        </h1>

                        <p class="flag-section-label">
                            "🔒 Classified Supplier Procurement Data"
                        </p>

                        <div class="flag-display">
                            {data.flag.clone()}
                        </div>

                        <p class="flag-note">
                            "This record is protected under Monsatan Proprietary License v7.3. "
                            "Unauthorised redistribution will trigger immediate gene-lock "
                            "enforcement on all associated biomass contracts."
                        </p>

                        <button
                            class="logout-button"
                            on:click=move |_| {
                                #[cfg(feature = "hydrate")]
                                logout();
                            }
                        >
                            "Log out"
                        </button>
                    </div>
                }.into_any(),

                // Show the error message in place instead of redirecting.
                FlagState::Error(msg) => view! {
                    <div class="flag-card">
                        <p class="error">"Authentication failed: " {msg}</p>

                        <button
                            class="logout-button"
                            on:click=move |_| {
                                #[cfg(feature = "hydrate")]
                                logout();
                            }
                        >
                            "Back to login"
                        </button>
                    </div>
                }.into_any(),
            }}
        </PageShell>
    }
}
