<!--
  AuthCard.svelte — Shared shell for the login and register pages.

  Props:
    activeTab  – which tab to highlight ('login' | 'register')
    finePrint  – small disclaimer text rendered below the form
    children   – the form snippet rendered inside the card
-->
<script lang="ts">
    import type { Snippet } from "svelte";

    interface Props {
        activeTab: "login" | "register";
        finePrint: string;
        children: Snippet;
    }

    let { activeTab, finePrint, children }: Props = $props();
</script>

<div class="auth-wrapper">
    <div class="auth-card">
        <div class="brand">
            <span class="brand-icon">🌱</span>
            <h1>Monsatan Mail™</h1>
        </div>
        <p class="tagline">
            Carbon-neutral. Algorithmically optimized. Legally distinct from
            surveillance.
        </p>

        <div class="tab-bar">
            <a class="tab" class:active={activeTab === "login"} href="/login">
                Login
            </a>
            <a
                class="tab"
                class:active={activeTab === "register"}
                href="/register"
            >
                Register
            </a>
        </div>

        {@render children()}

        <p class="fine-print">{finePrint}</p>
    </div>
</div>

<style>
    .auth-wrapper {
        align-items: center;
        display: flex;
        justify-content: center;
        min-height: 100vh;
        padding: 2rem;
    }

    .auth-card {
        background: #141f14;
        border: 1px solid #2a3f2a;
        border-radius: 8px;
        padding: 2.5rem;
        width: 100%;
        max-width: 420px;
    }

    /* ── Brand ───────────────────────────────────────────────────────────── */
    .brand {
        align-items: center;
        display: flex;
        gap: 0.5rem;
        margin-bottom: 0.4rem;
    }

    .brand-icon {
        font-size: 1.8rem;
    }

    .brand h1 {
        color: #6ecf6e;
        font-size: 1.5rem;
        letter-spacing: 0.02em;
    }

    .tagline {
        color: #6b8f6b;
        font-size: 0.8rem;
        font-style: italic;
        margin-bottom: 1.6rem;
    }

    /* ── Tab bar ─────────────────────────────────────────────────────────── */
    .tab-bar {
        border-bottom: 1px solid #2a3f2a;
        display: flex;
        margin-bottom: 1.5rem;
    }

    .tab {
        border-bottom: 2px solid transparent;
        color: #6b8f6b;
        cursor: pointer;
        flex: 1;
        font-size: 0.95rem;
        margin-bottom: -1px;
        padding: 0.6rem;
        text-align: center;
        text-decoration: none;
        transition:
            color 0.15s,
            border-color 0.15s;
    }

    .tab:hover {
        color: #d4e8d0;
    }

    .tab.active {
        border-bottom-color: #6ecf6e;
        color: #6ecf6e;
        cursor: default;
    }

    /* ── Fine print ──────────────────────────────────────────────────────── */
    .fine-print {
        color: #4a6a4a;
        font-size: 0.72rem;
        line-height: 1.4;
        margin-top: 1.5rem;
        text-align: center;
    }
</style>
