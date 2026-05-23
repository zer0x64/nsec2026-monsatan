//! Data models for the Monsatan™ Invoices API.
//!
//! Covers both the incoming order request payload and the outgoing
//! invoice/order response structures.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// A single line item inside an order request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    /// Must match a product `id` from the catalog.
    pub product_id: String,
    pub quantity: u32,
}

/// Body of `POST /api/orders`.
#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    pub customer_name: String,
    pub customer_email: String,
    pub shipping_address: String,
    pub items: Vec<OrderItem>,
}

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

/// Response returned after a successful order submission.
#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order_id: i64,
    /// Relative URL to retrieve the invoice, e.g. `/api/invoice/42`.
    pub invoice_url: String,
    pub message: String,
}

/// A resolved line item stored in an invoice (product details included).
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub product_id: String,
    pub product_name: String,
    pub quantity: u32,
    /// Unit price in USD (e.g. 149.99).
    pub unit_price: f64,
    /// `unit_price * quantity`.
    pub line_total: f64,
}

/// Full invoice returned by `GET /api/invoice/:id`.
#[derive(Debug, Serialize)]
pub struct Invoice {
    pub id: i64,
    pub customer_name: String,
    pub customer_email: String,
    pub shipping_address: String,
    pub items: Vec<InvoiceItem>,
    /// Grand total in USD.
    pub total: f64,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
}
