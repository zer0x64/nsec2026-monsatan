use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};

use crate::pages::{flag::FlagPage, login::LoginPage};

/// Root application component.
///
/// Sets up global metadata (title, stylesheet) and the client-side router.
/// All pages are flat routes — no nesting needed for this portal.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Monsatan Supplier Portal"/>
        // The compiled CSS bundle is served by cargo-leptos from /pkg/
        <Stylesheet id="leptos" href="/pkg/monsatan-orders.css"/>

        <Router>
            <FlatRoutes fallback=|| {
                view! {
                    <p class="error">
                        "404 — Resource not found. Your procurement clearance may have been revoked."
                    </p>
                }
            }>
                <Route path=StaticSegment("login") view=LoginPage/>
                <Route path=StaticSegment("flag")  view=FlagPage/>
            </FlatRoutes>
        </Router>
    }
}
