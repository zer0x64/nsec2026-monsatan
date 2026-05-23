//! Route handlers for `GET /api/catalog` and `POST /api/orders`.

use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use serde_json::{json, Value};

use crate::{
    catalog,
    db::AppState,
    models::{OrderRequest, OrderResponse},
};

/// `GET /api/catalog` — returns the full product catalog.
///
/// The frontend uses this to populate the order form.
pub async fn get_catalog() -> Json<Value> {
    let products: Vec<Value> = catalog::CATALOG
        .iter()
        .map(|p| {
            json!({
                "id":          p.id,
                "name":        p.name,
                "description": p.description,
                // Price exposed in dollars so the frontend doesn't need to divide.
                "price_cents":       p.price_cents,
                "unit":        p.unit,
            })
        })
        .collect();

    Json(json!(products))
}

/// `POST /api/orders` — submit a new order.
///
/// Validates all product IDs and quantities, persists the order, and returns
/// a link to the generated invoice.
pub async fn post_order(
    State(state): State<AppState>,
    Json(payload): Json<OrderRequest>,
) -> (StatusCode, Json<Value>) {
    // --- Basic validation ---------------------------------------------------

    if payload.customer_name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "customer_name is required." })),
        );
    }

    if payload.customer_email.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "customer_email is required." })),
        );
    }

    if payload.items.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "An order must contain at least one item." })),
        );
    }

    // --- Resolve products and compute total ---------------------------------

    let mut total_cents: i64 = 0;

    for item in &payload.items {
        if item.quantity == 0 {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": format!(
                        "Quantity for '{}' must be greater than zero.",
                        item.product_id
                    )
                })),
            );
        }

        match catalog::get_product(&item.product_id) {
            Some(product) => {
                total_cents += product.price_cents as i64 * item.quantity as i64;
            }
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": format!(
                            "Unknown product '{}'. Please refer to our catalog.",
                            item.product_id
                        )
                    })),
                );
            }
        }
    }

    // --- Persist to database ------------------------------------------------

    // Items are stored as a JSON blob to avoid a separate line-items table.
    let items_json = match serde_json::to_string(&payload.items) {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("Failed to serialize order items: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to process order items." })),
            );
        }
    };

    let created_at = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        INSERT INTO orders (customer_name, customer_email, shipping_address, items, total_cents, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&payload.customer_name)
    .bind(&payload.customer_email)
    .bind(&payload.shipping_address)
    .bind(&items_json)
    .bind(total_cents)
    .bind(&created_at)
    .execute(&state.db)
    .await;

    match result {
        Ok(row) => {
            let order_id = row.last_insert_rowid();
            let invoice_url = format!("/api/invoice/{}", order_id);

            tracing::info!(
                order_id,
                customer = %payload.customer_email,
                total_cents,
                "New order placed"
            );

            (
                StatusCode::CREATED,
                Json(json!(OrderResponse {
                    order_id,
                    invoice_url,
                    message: "Thank you for your order! \
                              Your invoice is ready. \
                              Verdachem Industries appreciates your continued business."
                        .to_string(),
                })),
            )
        }
        Err(e) => {
            tracing::error!("DB error inserting order: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to save your order. Please try again." })),
            )
        }
    }
}
