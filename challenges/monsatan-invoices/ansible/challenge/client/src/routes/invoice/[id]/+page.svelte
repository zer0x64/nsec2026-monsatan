<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/stores";

    // ── Types (matching the backend API) ────────────────────────────────────
    interface InvoiceItem {
        product_id: string;
        product_name: string;
        quantity: number;
        unit_price: number;
        line_total: number;
    }

    interface Invoice {
        id: number;
        customer_name: string;
        customer_email: string;
        shipping_address: string;
        items: InvoiceItem[];
        total: number;
        created_at: string;
    }

    // ── State ────────────────────────────────────────────────────────────────
    let invoice: Invoice | null = $state(null);
    let loading = $state(true);
    let error = $state("");

    // ── Helpers ──────────────────────────────────────────────────────────────

    /** Format a float dollar amount as a USD string, e.g. 149.99 -> "$149.99". */
    function formatUsd(amount: number): string {
        return `$${amount.toFixed(2)}`;
    }

    /** Format an ISO 8601 timestamp into a human-readable date. */
    function formatDate(iso: string): string {
        return new Date(iso).toLocaleDateString("en-US", {
            year: "numeric",
            month: "long",
            day: "numeric",
        });
    }

    // ── Lifecycle ────────────────────────────────────────────────────────────

    onMount(async () => {
        const id = $page.params.id;
        try {
            const res = await fetch(`/api/invoice/${id}`);
            if (res.status === 404) {
                error = `Invoice #${id} was not found. It may not exist yet, or the ID is incorrect.`;
                return;
            }
            if (!res.ok) throw new Error(`Server returned ${res.status}`);
            invoice = await res.json();
        } catch (e: any) {
            error =
                e.message ?? "Failed to load invoice. Please try again later.";
        } finally {
            loading = false;
        }
    });
</script>

{#if loading}
    <p class="status-msg">Loading invoice…</p>
{:else if error}
    <div class="error-card">
        <h2>Invoice Not Found</h2>
        <p>{error}</p>
        <a href="/" class="back-link">← Place a new order</a>
    </div>
{:else if invoice}
    <div class="invoice-wrapper">
        <!-- Invoice header: supplier info + invoice metadata -->
        <div class="invoice-header">
            <div class="invoice-from">
                <div class="invoice-logo">⬡</div>
                <div>
                    <p class="invoice-company">Verdachem Industries LLC</p>
                    <p class="invoice-company-sub">
                        Agrochemical Supply Division
                    </p>
                </div>
            </div>
            <div class="invoice-meta">
                <h1 class="invoice-title">INVOICE</h1>
                <table class="meta-table">
                    <tbody>
                        <tr>
                            <th>Invoice #</th>
                            <td>{invoice.id}</td>
                        </tr>
                        <tr>
                            <th>Date</th>
                            <td>{formatDate(invoice.created_at)}</td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>

        <!-- Bill to: customer contact & shipping info -->
        <div class="invoice-bill-to">
            <h2 class="section-label">Bill To</h2>
            <p class="bill-name">{invoice.customer_name}</p>
            <p class="bill-detail">{invoice.customer_email}</p>
            <p class="bill-detail bill-address">{invoice.shipping_address}</p>
        </div>

        <!-- Line items table -->
        <table class="items-table">
            <thead>
                <tr>
                    <th class="col-product">Product</th>
                    <th class="col-qty">Qty</th>
                    <th class="col-price">Unit Price</th>
                    <th class="col-total">Total</th>
                </tr>
            </thead>
            <tbody>
                {#each invoice.items as item (item.product_id)}
                    <tr>
                        <td class="col-product">{item.product_name}</td>
                        <td class="col-qty">{item.quantity}</td>
                        <td class="col-price">{formatUsd(item.unit_price)}</td>
                        <td class="col-total">{formatUsd(item.line_total)}</td>
                    </tr>
                {/each}
            </tbody>
            <tfoot>
                <tr class="grand-total-row">
                    <td colspan="3" class="grand-total-label"
                        >Grand Total (USD)</td
                    >
                    <td class="grand-total-value">{formatUsd(invoice.total)}</td
                    >
                </tr>
            </tfoot>
        </table>

        <!-- Terms and closing remarks -->
        <div class="invoice-notes">
            <p>
                Thank you for choosing Verdachem Industries as your agrochemical
                supplier. Your order will be processed and shipped within <strong
                    >5–10 business days</strong
                >.
            </p>
            <p>
                Payment terms: Net 30. Late payments subject to a 2% monthly
                service charge. All sales are final. Verdachem Industries
                assumes no liability for product misuse, regulatory
                non-compliance, or unintended ecological side-effects.
            </p>
        </div>

        <!-- Action buttons (hidden when printing) -->
        <div class="invoice-actions no-print">
            <a href="/" class="btn-secondary">← Place Another Order</a>
            <button class="btn-primary" onclick={() => window.print()}>
                Print / Save as PDF
            </button>
        </div>
    </div>
{/if}

<style>
    /* ── Status & error ──────────────────────────────────────────────────────── */
    .status-msg {
        text-align: center;
        padding: var(--space-xl);
        color: var(--colour-text-muted);
        font-style: italic;
    }

    .error-card {
        background: var(--colour-surface);
        border: 1px solid #f5c6c0;
        border-radius: var(--radius);
        padding: var(--space-xl);
        max-width: 500px;
        margin: var(--space-xl) auto;
        text-align: center;
    }

    .error-card h2 {
        color: var(--colour-danger);
        margin-bottom: var(--space-md);
    }

    .error-card p {
        color: var(--colour-text-muted);
        margin-bottom: var(--space-lg);
    }

    .back-link {
        font-weight: 600;
        color: var(--colour-green-mid);
    }

    /* ── Invoice wrapper ─────────────────────────────────────────────────────── */
    .invoice-wrapper {
        background: var(--colour-surface);
        border: 1px solid var(--colour-border);
        border-radius: var(--radius);
        padding: var(--space-xl);
        max-width: 820px;
        margin: 0 auto;
        display: flex;
        flex-direction: column;
        gap: var(--space-xl);
    }

    /* ── Invoice header ──────────────────────────────────────────────────────── */
    .invoice-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: var(--space-lg);
        padding-bottom: var(--space-lg);
        border-bottom: 3px solid var(--colour-green-dark);
    }

    .invoice-from {
        display: flex;
        align-items: center;
        gap: var(--space-md);
    }

    .invoice-logo {
        font-size: 3rem;
        color: var(--colour-acid);
        line-height: 1;
    }

    .invoice-company {
        font-size: 1.2rem;
        font-weight: 800;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: var(--colour-green-dark);
    }

    .invoice-company-sub {
        font-size: 0.8rem;
        color: var(--colour-text-muted);
        letter-spacing: 0.04em;
    }

    .invoice-meta {
        text-align: right;
    }

    .invoice-title {
        font-size: 2.2rem;
        font-weight: 900;
        letter-spacing: 0.1em;
        color: var(--colour-green-dark);
        margin-bottom: var(--space-sm);
    }

    .meta-table {
        border-collapse: collapse;
        margin-left: auto;
    }

    .meta-table th {
        font-size: 0.8rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        color: var(--colour-text-muted);
        padding-right: var(--space-md);
        text-align: left;
    }

    .meta-table td {
        font-size: 0.9rem;
        font-family: var(--font-mono);
        color: var(--colour-text);
        font-weight: 600;
    }

    /* ── Bill to ─────────────────────────────────────────────────────────────── */
    .invoice-bill-to {
        background: var(--colour-bg);
        border-left: 4px solid var(--colour-acid);
        padding: var(--space-md) var(--space-lg);
        border-radius: 0 var(--radius) var(--radius) 0;
    }

    .section-label {
        font-size: 0.75rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.08em;
        color: var(--colour-text-muted);
        margin-bottom: var(--space-sm);
    }

    .bill-name {
        font-size: 1.05rem;
        font-weight: 700;
        color: var(--colour-text);
    }

    .bill-detail {
        font-size: 0.9rem;
        color: var(--colour-text-muted);
    }

    .bill-address {
        white-space: pre-line;
        margin-top: var(--space-xs);
    }

    /* ── Line items table ────────────────────────────────────────────────────── */
    .items-table {
        width: 100%;
        border-collapse: collapse;
        font-size: 0.9rem;
    }

    .items-table thead tr {
        background: var(--colour-green-dark);
        color: #fff;
    }

    .items-table thead th {
        padding: var(--space-sm) var(--space-md);
        text-align: left;
        font-size: 0.8rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .items-table tbody tr {
        border-bottom: 1px solid var(--colour-border);
    }

    .items-table tbody tr:last-child {
        border-bottom: none;
    }

    .items-table tbody td {
        padding: var(--space-sm) var(--space-md);
        color: var(--colour-text);
    }

    /* Column alignment */
    .col-product {
        width: 50%;
    }
    .col-qty {
        text-align: center;
    }
    .col-price,
    .col-total {
        text-align: right;
        font-family: var(--font-mono);
    }

    /* Grand total footer row */
    .grand-total-row {
        background: var(--colour-bg);
        border-top: 2px solid var(--colour-green-dark);
    }

    .grand-total-label {
        padding: var(--space-md);
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        font-size: 0.9rem;
        color: var(--colour-text-muted);
    }

    .grand-total-value {
        padding: var(--space-md);
        text-align: right;
        font-family: var(--font-mono);
        font-size: 1.2rem;
        font-weight: 900;
        color: var(--colour-green-dark);
    }

    /* ── Notes ───────────────────────────────────────────────────────────────── */
    .invoice-notes {
        font-size: 0.8rem;
        color: var(--colour-text-muted);
        line-height: 1.6;
        display: flex;
        flex-direction: column;
        gap: var(--space-sm);
        padding-top: var(--space-md);
        border-top: 1px solid var(--colour-border);
    }

    /* ── Actions ─────────────────────────────────────────────────────────────── */
    .invoice-actions {
        display: flex;
        gap: var(--space-md);
        justify-content: flex-end;
        align-items: center;
        padding-top: var(--space-md);
        border-top: 1px solid var(--colour-border);
    }

    .btn-secondary {
        font-size: 0.9rem;
        font-weight: 600;
        color: var(--colour-green-mid);
        text-decoration: none;
        padding: var(--space-sm) var(--space-md);
        border: 1px solid var(--colour-border);
        border-radius: var(--radius);
        transition: background 0.15s;
    }

    .btn-secondary:hover {
        background: var(--colour-bg);
        color: var(--colour-green-mid);
    }

    .btn-primary {
        background: var(--colour-green-dark);
        color: #fff;
        border: none;
        border-radius: var(--radius);
        padding: var(--space-sm) var(--space-lg);
        font-size: 0.9rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        cursor: pointer;
        transition: background 0.15s;
    }

    .btn-primary:hover {
        background: var(--colour-green-mid);
    }

    /* ── Print styles ────────────────────────────────────────────────────────── */
    @media print {
        .no-print {
            display: none;
        }

        .invoice-wrapper {
            border: none;
            padding: 0;
        }
    }

    /* ── Responsive ──────────────────────────────────────────────────────────── */
    @media (max-width: 640px) {
        .invoice-header {
            flex-direction: column;
        }

        .invoice-meta {
            text-align: left;
        }

        .meta-table {
            margin-left: 0;
        }

        .invoice-title {
            font-size: 1.6rem;
        }

        .invoice-actions {
            flex-direction: column;
            align-items: stretch;
        }

        .btn-secondary,
        .btn-primary {
            text-align: center;
        }
    }
</style>
