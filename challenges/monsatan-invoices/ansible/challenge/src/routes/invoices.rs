//! Route handler for `GET /api/invoice/:id`.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use sqlx::FromRow;

use crate::{
    catalog,
    db::AppState,
    models::{Invoice, InvoiceItem, OrderItem},
};

/// Raw row returned from the `orders` table.
#[derive(FromRow)]
struct OrderRow {
    id: i64,
    customer_name: String,
    customer_email: String,
    shipping_address: String,
    /// JSON-encoded `Vec<OrderItem>`.
    items: String,
    total_cents: i64,
    created_at: String,
}

/// `GET /api/invoice/:id` — retrieve an invoice by its sequential order ID.
///
/// Returns the full invoice including customer details, line items with
/// resolved product names and prices, and the grand total.
pub async fn get_invoice(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> (StatusCode, Json<Value>) {
    let result = sqlx::query_as::<_, OrderRow>(
        r#"
        SELECT id, customer_name, customer_email, shipping_address,
               items, total_cents, created_at
        FROM   orders
        WHERE  id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(row)) => {
            // Deserialize the stored JSON blob back into order items.
            let stored_items: Vec<OrderItem> = match serde_json::from_str(&row.items) {
                Ok(items) => items,
                Err(e) => {
                    tracing::error!(invoice_id = id, "Failed to deserialize items: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": "Failed to parse invoice line items." })),
                    );
                }
            };

            // Resolve each stored item against the live catalog to get the
            // current product name and price. Items for discontinued products
            // are silently dropped.
            let invoice_items: Vec<InvoiceItem> = stored_items
                .iter()
                .filter_map(|item| {
                    catalog::get_product(&item.product_id).map(|product| InvoiceItem {
                        product_id: item.product_id.clone(),
                        product_name: product.name.to_string(),
                        quantity: item.quantity,
                        unit_price: product.price_cents as f64 / 100.0,
                        line_total: product.price_cents as f64 / 100.0 * item.quantity as f64,
                    })
                })
                .collect();

            let invoice = Invoice {
                id: row.id,
                customer_name: row.customer_name,
                customer_email: row.customer_email,
                shipping_address: row.shipping_address,
                items: invoice_items,
                total: row.total_cents as f64 / 100.0,
                created_at: row.created_at,
            };

            (StatusCode::OK, Json(json!(invoice)))
        }

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("Invoice #{} not found.", id) })),
        ),

        Err(e) => {
            tracing::error!(invoice_id = id, "DB error fetching invoice: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Failed to retrieve invoice." })),
            )
        }
    }
}
