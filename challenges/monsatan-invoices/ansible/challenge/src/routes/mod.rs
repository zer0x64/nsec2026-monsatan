//! Route registration for the Monsatan™ Invoices API.
//!
//! Wires all handlers together into a single [`axum::Router`] with shared
//! [`AppState`].

pub mod invoices;
pub mod orders;

use axum::{
    routing::{get, post},
    Router,
};

use crate::db::AppState;

/// Build the API router with all routes and shared state attached.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Product catalog — used by the frontend to populate the order form
        .route("/api/catalog", get(orders::get_catalog))
        // Order submission — returns a link to the generated invoice
        .route("/api/orders", post(orders::post_order))
        // Invoice retrieval — intentionally unprotected (IDOR challenge)
        .route("/api/invoice/{id}", get(invoices::get_invoice))
        .with_state(state)
}
