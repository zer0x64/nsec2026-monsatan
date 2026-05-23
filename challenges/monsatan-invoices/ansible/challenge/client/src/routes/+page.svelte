<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    // ── Types (matching the backend API) ────────────────────────────────────
    interface Product {
        id: string;
        name: string;
        description: string;
        price_cents: number;
        unit: string;
    }

    interface OrderItem {
        product_id: string;
        quantity: number;
    }

    // ── State ────────────────────────────────────────────────────────────────
    let catalog: Product[] = $state([]);
    let catalogLoading = $state(true);
    let catalogError = $state("");

    /** Map of product_id -> selected quantity. */
    let quantities: Record<string, number> = $state({});

    let customerName = $state("");
    let customerEmail = $state("");
    let shippingAddress = $state("");
    let submitting = $state(false);
    let formError = $state("");

    // ── Derived values ───────────────────────────────────────────────────────

    /** Line items where the user selected a non-zero quantity. */
    const orderItems: OrderItem[] = $derived(
        Object.entries(quantities)
            .filter(([, qty]) => qty > 0)
            .map(([product_id, quantity]) => ({ product_id, quantity })),
    );

    /** Grand total in cents. */
    const totalCents: number = $derived(
        catalog.reduce(
            (sum, p) => sum + p.price_cents * (quantities[p.id] ?? 0),
            0,
        ),
    );

    // ── Helpers ──────────────────────────────────────────────────────────────

    /** Format an integer cent amount as a USD string, e.g. 14999 -> "$149.99". */
    function formatUsd(cents: number): string {
        console.log(cents);
        return `$${(cents / 100).toFixed(2)}`;
    }

    /** Update a product's quantity, clamping to >= 0. */
    function clampQty(id: string, value: number) {
        quantities[id] = Math.max(0, Math.floor(value) || 0);
    }

    // ── Lifecycle ────────────────────────────────────────────────────────────

    onMount(async () => {
        try {
            const res = await fetch("/api/catalog");
            if (!res.ok) throw new Error(`Server returned ${res.status}`);
            catalog = await res.json();
            // Initialise all quantities to zero
            quantities = Object.fromEntries(catalog.map((p) => [p.id, 0]));
        } catch (e: any) {
            catalogError =
                "Failed to load product catalog. Please try again later.";
        } finally {
            catalogLoading = false;
        }
    });

    // ── Form submission ──────────────────────────────────────────────────────

    async function submitOrder(event: SubmitEvent) {
        event.preventDefault();
        formError = "";

        if (orderItems.length === 0) {
            formError = "Please select at least one product before submitting.";
            return;
        }

        submitting = true;
        try {
            const res = await fetch("/api/orders", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    customer_name: customerName,
                    customer_email: customerEmail,
                    shipping_address: shippingAddress,
                    items: orderItems,
                }),
            });

            if (!res.ok) {
                const body = await res.json().catch(() => ({}));
                throw new Error(
                    body.error ?? `Order failed (HTTP ${res.status})`,
                );
            }

            const data = await res.json();
            await goto(`/invoice/${data.order_id}`);
        } catch (e: any) {
            formError = e.message ?? "An unexpected error occurred.";
        } finally {
            submitting = false;
        }
    }
</script>

<!-- ── Hero ──────────────────────────────────────────────────────────────── -->
<section class="hero">
    <h1>Verdachem Industries B2B Portal</h1>
    <p class="hero-sub">
        Welcome, valued purchasing agent. Verdachem Industries supplies the
        world's foremost agricultural corporations with the active ingredients
        that keep modern farming productive — whatever the long-term cost.
    </p>
    <p class="hero-notice">
        ⚠ This portal is for <strong>authorised corporate buyers only</strong>.
        All orders are binding. Minimum liability waivers apply. Results may
        vary by hemisphere.
    </p>
</section>

<!-- ── Catalog ───────────────────────────────────────────────────────────── -->
<section class="section">
    <h2>Product Catalog</h2>
    <p class="section-intro">
        Select quantities below, then complete your contact details to generate
        a binding invoice. Prices are per unit (USD). Volume discounts available
        upon written request.
    </p>

    {#if catalogLoading}
        <p class="status-msg">Loading catalog…</p>
    {:else if catalogError}
        <p class="status-msg error">{catalogError}</p>
    {:else}
        <div class="product-grid">
            {#each catalog as product (product.id)}
                <div class="product-card">
                    <div class="product-header">
                        <span class="product-name">{product.name}</span>
                        <span class="product-price"
                            >{formatUsd(
                                product.price_cents,
                            )}/{product.unit}</span
                        >
                    </div>
                    <p class="product-description">{product.description}</p>
                    <div class="product-footer">
                        <span class="qty-label">Qty ({product.unit})</span>
                        <div class="qty-control">
                            <button
                                type="button"
                                class="qty-btn"
                                onclick={() =>
                                    clampQty(
                                        product.id,
                                        (quantities[product.id] ?? 0) - 1,
                                    )}>−</button
                            >
                            <input
                                type="number"
                                class="qty-input"
                                min="0"
                                value={quantities[product.id] ?? 0}
                                oninput={(e) =>
                                    clampQty(
                                        product.id,
                                        parseInt(
                                            (e.target as HTMLInputElement)
                                                .value,
                                        ),
                                    )}
                            />
                            <button
                                type="button"
                                class="qty-btn"
                                onclick={() =>
                                    clampQty(
                                        product.id,
                                        (quantities[product.id] ?? 0) + 1,
                                    )}>+</button
                            >
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</section>

<!-- ── Order form ─────────────────────────────────────────────────────────── -->
<section class="section">
    <h2>Place Your Order</h2>
    <div class="order-layout">
        <!-- Contact & submit form -->
        <form class="order-form" onsubmit={submitOrder}>
            <h3>Billing &amp; Shipping Details</h3>

            <div class="field">
                <label for="name"
                    >Full Name <span class="required">*</span></label
                >
                <input
                    id="name"
                    type="text"
                    required
                    autocomplete="name"
                    bind:value={customerName}
                />
            </div>

            <div class="field">
                <label for="email"
                    >Corporate Email <span class="required">*</span></label
                >
                <input
                    id="email"
                    type="email"
                    required
                    autocomplete="email"
                    bind:value={customerEmail}
                />
            </div>

            <div class="field">
                <label for="address"
                    >Shipping Address <span class="required">*</span></label
                >
                <textarea
                    id="address"
                    required
                    rows="3"
                    bind:value={shippingAddress}
                ></textarea>
            </div>

            {#if formError}
                <p class="form-error">{formError}</p>
            {/if}

            <button
                type="submit"
                class="submit-btn"
                disabled={submitting || orderItems.length === 0}
            >
                {#if submitting}
                    Processing…
                {:else}
                    Confirm Order &amp; Generate Invoice
                {/if}
            </button>

            <p class="form-note">
                By placing this order you acknowledge that Verdachem Industries
                bears no liability for any downstream agricultural, ecological,
                or legal outcomes.
            </p>
        </form>

        <!-- Live order summary -->
        <aside class="order-summary">
            <h3>Order Summary</h3>
            {#if orderItems.length === 0}
                <p class="summary-empty">
                    No products selected. Add quantities from the catalog above.
                </p>
            {:else}
                <ul class="summary-list">
                    {#each orderItems as item (item.product_id)}
                        {@const product = catalog.find(
                            (p) => p.id === item.product_id,
                        )}
                        {#if product}
                            <li class="summary-item">
                                <span class="summary-name">{product.name}</span>
                                <span class="summary-qty">×{item.quantity}</span
                                >
                                <span class="summary-line">
                                    {formatUsd(
                                        product.price_cents * item.quantity,
                                    )}
                                </span>
                            </li>
                        {/if}
                    {/each}
                </ul>
                <div class="summary-total">
                    <span>Total</span>
                    <span>{formatUsd(totalCents)}</span>
                </div>
                <p class="summary-note">
                    Shipping and handling billed separately. Taxes not included.
                </p>
            {/if}
        </aside>
    </div>
</section>

<style>
    /* ── Hero ────────────────────────────────────────────────────────────────── */
    .hero {
        background: var(--colour-green-dark);
        color: #fff;
        border-radius: var(--radius);
        padding: var(--space-xl);
        margin-bottom: var(--space-xl);
        border-left: 6px solid var(--colour-acid);
    }

    .hero h1 {
        font-size: 2rem;
        font-weight: 800;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        margin-bottom: var(--space-md);
        color: var(--colour-acid);
    }

    .hero-sub {
        font-size: 1rem;
        line-height: 1.7;
        color: rgba(255, 255, 255, 0.88);
        margin-bottom: var(--space-md);
    }

    .hero-notice {
        font-size: 0.85rem;
        background: rgba(255, 255, 255, 0.07);
        border: 1px solid rgba(255, 255, 255, 0.15);
        border-radius: var(--radius);
        padding: var(--space-sm) var(--space-md);
        color: var(--colour-amber);
    }

    /* ── Sections ────────────────────────────────────────────────────────────── */
    .section {
        margin-bottom: var(--space-xl);
    }

    .section h2 {
        font-size: 1.4rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        color: var(--colour-green-dark);
        border-bottom: 2px solid var(--colour-acid);
        padding-bottom: var(--space-xs);
        margin-bottom: var(--space-md);
    }

    .section-intro {
        color: var(--colour-text-muted);
        margin-bottom: var(--space-lg);
        font-size: 0.9rem;
    }

    /* ── Status messages ─────────────────────────────────────────────────────── */
    .status-msg {
        padding: var(--space-lg);
        text-align: center;
        color: var(--colour-text-muted);
        font-style: italic;
    }

    .status-msg.error {
        color: var(--colour-danger);
        font-style: normal;
        font-weight: 600;
    }

    /* ── Product grid ────────────────────────────────────────────────────────── */
    .product-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: var(--space-lg);
    }

    .product-card {
        background: var(--colour-surface);
        border: 1px solid var(--colour-border);
        border-radius: var(--radius);
        padding: var(--space-lg);
        display: flex;
        flex-direction: column;
        gap: var(--space-sm);
        transition: box-shadow 0.15s;
    }

    .product-card:hover {
        box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
    }

    .product-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        gap: var(--space-sm);
    }

    .product-name {
        font-size: 1rem;
        font-weight: 700;
        color: var(--colour-green-dark);
        line-height: 1.3;
    }

    .product-price {
        font-size: 0.9rem;
        font-weight: 700;
        color: var(--colour-amber);
        white-space: nowrap;
        font-family: var(--font-mono);
    }

    .product-description {
        font-size: 0.85rem;
        color: var(--colour-text-muted);
        line-height: 1.5;
        flex: 1;
    }

    .product-footer {
        margin-top: var(--space-sm);
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: var(--space-md);
    }

    .qty-label {
        font-size: 0.8rem;
        font-weight: 600;
        color: var(--colour-text-muted);
        text-transform: uppercase;
        letter-spacing: 0.04em;
    }

    .qty-control {
        display: flex;
        align-items: center;
        border: 1px solid var(--colour-border);
        border-radius: var(--radius);
        overflow: hidden;
    }

    .qty-btn {
        background: var(--colour-bg);
        border: none;
        width: 2rem;
        height: 2rem;
        font-size: 1.1rem;
        cursor: pointer;
        color: var(--colour-green-dark);
        font-weight: 700;
        transition: background 0.1s;
    }

    .qty-btn:hover {
        background: var(--colour-border);
    }

    .qty-input {
        width: 3.5rem;
        height: 2rem;
        border: none;
        border-left: 1px solid var(--colour-border);
        border-right: 1px solid var(--colour-border);
        text-align: center;
        font-size: 0.9rem;
        font-family: var(--font-mono);
        background: var(--colour-surface);
        -moz-appearance: textfield;
    }

    /* Hide browser number spinners */
    .qty-input::-webkit-inner-spin-button,
    .qty-input::-webkit-outer-spin-button {
        -webkit-appearance: none;
    }

    /* ── Order layout ────────────────────────────────────────────────────────── */
    .order-layout {
        display: grid;
        grid-template-columns: 1fr 340px;
        gap: var(--space-xl);
        align-items: start;
    }

    /* ── Order form ──────────────────────────────────────────────────────────── */
    .order-form {
        background: var(--colour-surface);
        border: 1px solid var(--colour-border);
        border-radius: var(--radius);
        padding: var(--space-lg);
        display: flex;
        flex-direction: column;
        gap: var(--space-md);
    }

    .order-form h3 {
        font-size: 1rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        color: var(--colour-green-dark);
        margin-bottom: var(--space-xs);
    }

    .field {
        display: flex;
        flex-direction: column;
        gap: var(--space-xs);
    }

    .field label {
        font-size: 0.85rem;
        font-weight: 600;
        color: var(--colour-text);
    }

    .required {
        color: var(--colour-danger);
    }

    .field input,
    .field textarea {
        font-family: var(--font-body);
        font-size: 0.95rem;
        padding: var(--space-sm) var(--space-md);
        border: 1px solid var(--colour-border);
        border-radius: var(--radius);
        background: var(--colour-bg);
        color: var(--colour-text);
        transition: border-color 0.15s;
    }

    .field input:focus,
    .field textarea:focus {
        outline: none;
        border-color: var(--colour-green-light);
    }

    .field textarea {
        resize: vertical;
    }

    .form-error {
        color: var(--colour-danger);
        font-size: 0.9rem;
        font-weight: 600;
        padding: var(--space-sm) var(--space-md);
        background: #fdf0ee;
        border: 1px solid #f5c6c0;
        border-radius: var(--radius);
    }

    .submit-btn {
        background: var(--colour-green-dark);
        color: #fff;
        border: none;
        border-radius: var(--radius);
        padding: var(--space-md) var(--space-lg);
        font-size: 1rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        cursor: pointer;
        transition: background 0.15s;
    }

    .submit-btn:hover:not(:disabled) {
        background: var(--colour-green-mid);
    }

    .submit-btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .form-note {
        font-size: 0.75rem;
        color: var(--colour-text-muted);
        line-height: 1.5;
        font-style: italic;
    }

    /* ── Order summary ───────────────────────────────────────────────────────── */
    .order-summary {
        background: var(--colour-surface);
        border: 1px solid var(--colour-border);
        border-radius: var(--radius);
        padding: var(--space-lg);
        position: sticky;
        top: var(--space-lg);
    }

    .order-summary h3 {
        font-size: 1rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.04em;
        color: var(--colour-green-dark);
        margin-bottom: var(--space-md);
        padding-bottom: var(--space-xs);
        border-bottom: 1px solid var(--colour-border);
    }

    .summary-empty {
        font-size: 0.85rem;
        color: var(--colour-text-muted);
        font-style: italic;
    }

    .summary-list {
        list-style: none;
        display: flex;
        flex-direction: column;
        gap: var(--space-sm);
        margin-bottom: var(--space-md);
    }

    .summary-item {
        display: grid;
        grid-template-columns: 1fr auto auto;
        gap: var(--space-sm);
        align-items: baseline;
        font-size: 0.875rem;
    }

    .summary-name {
        color: var(--colour-text);
        font-weight: 500;
    }

    .summary-qty {
        color: var(--colour-text-muted);
        font-family: var(--font-mono);
        font-size: 0.8rem;
    }

    .summary-line {
        font-family: var(--font-mono);
        font-weight: 600;
        color: var(--colour-amber);
        text-align: right;
    }

    .summary-total {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-top: var(--space-sm);
        border-top: 2px solid var(--colour-green-dark);
        font-weight: 700;
        font-size: 1.05rem;
        font-family: var(--font-mono);
        color: var(--colour-green-dark);
        margin-bottom: var(--space-sm);
    }

    .summary-note {
        font-size: 0.75rem;
        color: var(--colour-text-muted);
        font-style: italic;
    }

    /* ── Responsive ──────────────────────────────────────────────────────────── */
    @media (max-width: 768px) {
        .order-layout {
            grid-template-columns: 1fr;
        }

        /* Show the summary above the form on mobile */
        .order-summary {
            position: static;
            order: -1;
        }

        .hero h1 {
            font-size: 1.5rem;
        }
    }
</style>
