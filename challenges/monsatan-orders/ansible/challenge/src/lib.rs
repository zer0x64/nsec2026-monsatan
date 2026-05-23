/// localStorage key under which the JWT is stored client-side.
/// Used by the login page to save the token and by the flag page to retrieve it.
pub const TOKEN_STORAGE_KEY: &str = "monsatan_token";

/// HS256 signing secret shared between the client (WASM) and the server.
pub const JWT_SECRET: [u8; 32] = [
    84, 164, 184, 194, 67, 45, 141, 120, 92, 217, 125, 229, 254, 165, 158, 21, 122, 233, 104, 151,
    98, 160, 240, 111, 187, 74, 216, 123, 188, 58, 45, 146,
];

pub mod app;
pub mod components;
pub mod pages;

// Server-only modules — used by server functions inside pages/
#[cfg(feature = "ssr")]
pub mod auth;
#[cfg(feature = "ssr")]
pub mod constants;

use leptos::prelude::*;
use leptos_meta::MetaTags;

/// HTML document shell: wraps the Leptos app with the full `<html>` structure and
/// injects the hydration + hot-reload scripts that cargo-leptos produces.
///
/// This function is called server-side to render the initial HTML, and the
/// `<HydrationScripts>` component tells the browser which WASM bundle to load.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                // Injects a JS snippet for cargo-leptos hot-reload in dev mode (no-op in prod)
                <AutoReload options=options.clone()/>
                // Injects the WASM bootstrap script and serialized server state
                <HydrationScripts options/>
                // Renders any <Title>, <Meta>, <Link> etc. set via leptos_meta components
                <MetaTags/>
            </head>
            <body>
                <app::App/>
            </body>
        </html>
    }
}

/// WASM entry point: called by the browser after the WASM bundle loads.
/// Hydrates the SSR-rendered HTML, attaching reactive event listeners without
/// re-rendering the DOM from scratch.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
