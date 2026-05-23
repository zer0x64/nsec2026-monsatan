use leptos::prelude::*;

/// A full-page shell that wraps authenticated portal pages.
///
/// Renders the Monsatan corporate header at the top and a legal disclaimer
/// footer at the bottom, with the page content in between.
#[component]
pub fn PageShell(children: Children) -> impl IntoView {
    view! {
        <div class="portal-shell">
            <header class="portal-header">
                <span class="brand-icon">"🌱"</span>
                <span class="brand-name">"Monsatan"</span>
                <span class="brand-tagline">"— Supplier Order Management"</span>
            </header>

            <main class="portal-main">
                {children()}
            </main>

            <footer class="portal-footer">
                <p>
                    "© 2026 Monsatan Corporation. All genomes reserved. "
                    "Unauthorized reproduction — biological or digital — is strictly prosecuted "
                    "under the Unified Seed & Data Sovereignty Act of 2024."
                </p>
            </footer>
        </div>
    }
}
