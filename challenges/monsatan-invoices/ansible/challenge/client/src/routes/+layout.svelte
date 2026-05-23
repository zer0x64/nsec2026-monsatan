<script lang="ts">
    import type { Snippet } from "svelte";

    const { children }: { children: Snippet } = $props();
</script>

<header>
    <div class="header-inner">
        <div class="brand">
            <span class="brand-logo">⬡</span>
            <div>
                <span class="brand-name">Verdachem Industries</span>
                <span class="brand-tagline"
                    >Supplying Tomorrow's Agriculture Today™</span
                >
            </div>
        </div>
        <nav>
            <a href="/">Order</a>
        </nav>
    </div>
</header>

<main>
    {@render children()}
</main>

<footer>
    <div class="footer-inner">
        <p class="footer-brand">
            © 2026 Verdachem Industries LLC. All rights reserved.
        </p>
        <p class="footer-disclaimer">
            Products may cause environmental, ecological, or human health
            effects. Consult applicable regulations before use. Verdachem
            Industries assumes no liability for downstream consequences.
            Regulatory compliance is entirely your responsibility.
        </p>
    </div>
</footer>

<style>
    /* ── Global reset & CSS variables ──────────────────────────────────────── */
    :global(*),
    :global(*::before),
    :global(*::after) {
        box-sizing: border-box;
        margin: 0;
        padding: 0;
    }

    :global(:root) {
        /* Brand colours */
        --colour-green-dark: #1e4620;
        --colour-green-mid: #2e6b32;
        --colour-green-light: #4a9e50;
        --colour-acid: #8fbc2a;
        --colour-amber: #d4900a;
        --colour-danger: #c0392b;

        /* Neutrals */
        --colour-bg: #f4f5f0;
        --colour-surface: #ffffff;
        --colour-border: #d0d4c8;
        --colour-text: #1a1a18;
        --colour-text-muted: #5a5e52;

        /* Spacing scale */
        --space-xs: 0.25rem;
        --space-sm: 0.5rem;
        --space-md: 1rem;
        --space-lg: 1.5rem;
        --space-xl: 2.5rem;

        /* Typography */
        --font-body: "Segoe UI", system-ui, sans-serif;
        --font-mono: "Courier New", monospace;
        --radius: 4px;
    }

    :global(body) {
        font-family: var(--font-body);
        background: var(--colour-bg);
        color: var(--colour-text);
        line-height: 1.6;
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }

    :global(a) {
        color: var(--colour-green-mid);
    }

    :global(a:hover) {
        color: var(--colour-green-light);
    }

    /* ── Header ─────────────────────────────────────────────────────────────── */
    header {
        background: var(--colour-green-dark);
        color: #fff;
        padding: var(--space-md) var(--space-xl);
        border-bottom: 4px solid var(--colour-acid);
    }

    .header-inner {
        max-width: 1100px;
        margin: 0 auto;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: var(--space-md);
    }

    .brand {
        display: flex;
        align-items: center;
        gap: var(--space-md);
    }

    .brand-logo {
        font-size: 2.4rem;
        color: var(--colour-acid);
        line-height: 1;
        /* Hexagon icon evokes both chemistry and honeycomb (the bees we'll miss) */
    }

    .brand > div {
        display: flex;
        flex-direction: column;
    }

    .brand-name {
        font-size: 1.4rem;
        font-weight: 700;
        letter-spacing: 0.04em;
        text-transform: uppercase;
    }

    .brand-tagline {
        font-size: 0.75rem;
        color: var(--colour-acid);
        letter-spacing: 0.06em;
    }

    nav a {
        color: #fff;
        text-decoration: none;
        font-size: 0.9rem;
        font-weight: 600;
        letter-spacing: 0.05em;
        text-transform: uppercase;
        border: 1px solid rgba(255, 255, 255, 0.3);
        padding: var(--space-xs) var(--space-md);
        border-radius: var(--radius);
        transition: background 0.15s;
    }

    nav a:hover {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
    }

    /* ── Main content ────────────────────────────────────────────────────────── */
    main {
        flex: 1;
        max-width: 1100px;
        width: 100%;
        margin: 0 auto;
        padding: var(--space-xl) var(--space-lg);
    }

    /* ── Footer ──────────────────────────────────────────────────────────────── */
    footer {
        background: var(--colour-green-dark);
        color: rgba(255, 255, 255, 0.7);
        padding: var(--space-lg) var(--space-xl);
        margin-top: auto;
    }

    .footer-inner {
        max-width: 1100px;
        margin: 0 auto;
        display: flex;
        flex-direction: column;
        gap: var(--space-sm);
    }

    .footer-brand {
        font-weight: 600;
        color: #fff;
        font-size: 0.85rem;
    }

    .footer-disclaimer {
        font-size: 0.75rem;
        line-height: 1.5;
    }

    /* ── Responsive ──────────────────────────────────────────────────────────── */
    @media (max-width: 640px) {
        header {
            padding: var(--space-md);
        }

        .brand-name {
            font-size: 1.1rem;
        }

        .brand-tagline {
            display: none;
        }

        main {
            padding: var(--space-lg) var(--space-md);
        }
    }
</style>
