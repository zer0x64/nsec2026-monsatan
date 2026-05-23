use leptos::prelude::*;

/// Server function: validates employee credentials and returns the unsigned JWT
/// parts — `base64url(header).base64url(payload)` — on success.
///
/// The expiry timestamp is set here so the server controls token lifetime.
/// Signing is left to the client via [`append_signature`].
#[server]
pub async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    use crate::constants::{VALID_PASSWORD, VALID_USERNAME};
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use std::time::{SystemTime, UNIX_EPOCH};

    if username != VALID_USERNAME || password != VALID_PASSWORD {
        // Deliberately vague — we don't confirm whether the username exists.
        return Err(ServerFnError::new("Invalid credentials. Access denied."));
    }

    // Standard HS256 JWT header.
    let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","typ":"JWT"}"#);

    // Payload: subject + expiry (1 hour from now).
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the Unix epoch")
        .as_secs()
        + 3600;
    let payload_json = format!(r#"{{"sub":"{username}","exp":{exp}}}"#);
    let payload = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());

    // Return only the signing input — the client will append the signature.
    Ok(format!("{header}.{payload}"))
}

/// Completes a JWT by computing an HMAC-SHA256 signature over `signing_input`
/// (`base64url(header).base64url(payload)`) using [`crate::JWT_SECRET`] and
/// appending it as the third segment.
///
/// Only compiled for the WASM target.
#[cfg(feature = "hydrate")]
fn append_signature(signing_input: &str) -> String {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(crate::JWT_SECRET.as_slice())
        .expect("HMAC accepts any key length");
    mac.update(signing_input.as_bytes());
    let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());

    format!("{signing_input}.{signature}")
}

/// Login page — the front door of the Monsatan Supplier Portal.
///
/// On a successful login the server returns the unsigned token parts. The
/// client signs them, stores the resulting JWT in `localStorage`, and
/// navigates to `/flag`.
#[component]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<Login>::new();

    // Show the error message returned by the last failed login attempt.
    let error_msg = move || {
        login_action
            .value()
            .get()
            .and_then(|result| result.err())
            .map(|e| e.to_string())
    };

    // After a successful login the server returns `base64url(header).base64url(payload)`.
    // The client appends the HMAC-SHA256 signature and stores the full JWT locally.
    Effect::new(move |_| {
        if let Some(Ok(signing_input)) = login_action.value().get() {
            #[cfg(feature = "hydrate")]
            {
                use crate::TOKEN_STORAGE_KEY;

                let token = append_signature(&signing_input);

                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item(TOKEN_STORAGE_KEY, &token);
                    }
                    let _ = window.location().replace("/flag");
                }
            }
            // Suppress the "unused variable" warning on the SSR build.
            let _ = signing_input;
        }
    });

    view! {
        <div class="login-wrapper">
            <div class="login-card">

                <div class="brand">
                    <span class="brand-icon">"🌱"</span>
                    <h1 class="brand-name">"Monsatan"</h1>
                    <p class="brand-tagline">"Growing Tomorrow, Today™"</p>
                </div>

                <h2 class="login-title">"Supplier Portal — Employee Access"</h2>

                // ActionForm wires the <form> submit directly to the login server action.
                <ActionForm action=login_action>
                    <div class="form-group">
                        <label for="username">"Employee ID"</label>
                        <input
                            type="text"
                            id="username"
                            name="username"
                            placeholder="Enter your employee email"
                            autocomplete="username"
                        />
                    </div>

                    <div class="form-group">
                        <label for="password">"Passphrase"</label>
                        <input
                            type="password"
                            id="password"
                            name="password"
                            placeholder="Enter your passphrase"
                            autocomplete="current-password"
                        />
                    </div>

                    // Only rendered when there is an error to display.
                    {move || error_msg().map(|msg| view! {
                        <div class="error-box">{msg}</div>
                    })}

                    <button type="submit" class="btn-primary">
                        "Access Portal"
                    </button>
                </ActionForm>

                <p class="disclaimer">
                    "⚠ Unauthorised access is monitored and will result in immediate "
                    "termination of your biomass contract and/or gene-lock enforcement "
                    "under the Monsatan Proprietary Genome Act of 2020."
                </p>

            </div>
        </div>
    }
}
