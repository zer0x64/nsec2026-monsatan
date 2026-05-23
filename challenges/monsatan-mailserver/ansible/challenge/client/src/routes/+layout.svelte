<!--
  +layout.svelte — Root layout.

  Responsibilities:
    • Initialise the auth store once from localStorage on mount.
    • Render the top navigation bar on authenticated pages.
    • Provide global CSS reset and design-token styles for all routes.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { auth, initAuth, clearAuth } from "$lib/auth.svelte";

  let { children } = $props();

  // Routes that don't require authentication (and shouldn't show the nav bar).
  const AUTH_ROUTES = ["/login", "/register"];

  const isAuthRoute = $derived(AUTH_ROUTES.includes($page.url.pathname));

  onMount(() => {
    initAuth();

    if (auth.token && isAuthRoute) {
      goto("/inbox");
    } else if (!auth.token && !isAuthRoute) {
      goto("/login");
    }
  });

  function logout() {
    clearAuth();
    goto("/login");
  }
</script>

<!-- Top nav bar — only rendered on authenticated pages -->
{#if !isAuthRoute && auth.token}
  <header class="topbar">
    <span class="brand">🌱 Monsatan Mail™</span>
    <span class="user-chip">{auth.email}</span>
    <nav>
      <a class="nav-btn" href="/compose">✉ Compose</a>
      <a class="nav-btn" href="/inbox">📥 Inbox</a>
      <button class="nav-btn" onclick={logout}>Logout</button>
    </nav>
  </header>
{/if}

<main class="content" class:full={isAuthRoute}>
  {@render children()}
</main>

<style>
  /* ── Reset ───────────────────────────────────────────────────────────────── */
  :global(*, *::before, *::after) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: "Segoe UI", system-ui, sans-serif;
    background: #0d130d;
    color: #d4e8d0;
    min-height: 100vh;
  }

  /* ── Shared form elements ────────────────────────────────────────────────── */
  :global(label) {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    font-size: 0.85rem;
    color: #8fad8f;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  :global(input[type="text"]),
  :global(input[type="password"]),
  :global(textarea) {
    background: #0d130d;
    border: 1px solid #2a3f2a;
    border-radius: 4px;
    color: #d4e8d0;
    font-family: inherit;
    font-size: 1rem;
    outline: none;
    padding: 0.55rem 0.75rem;
    transition: border-color 0.15s;
    width: 100%;
  }

  :global(input[type="text"]:focus),
  :global(input[type="password"]:focus),
  :global(textarea:focus) {
    border-color: #4a9e4a;
  }

  :global(textarea) {
    line-height: 1.5;
    resize: vertical;
  }

  /* ── Shared button styles ────────────────────────────────────────────────── */
  :global(.btn-primary) {
    background: #2e6e2e;
    border: 1px solid #4a9e4a;
    border-radius: 4px;
    color: #d4e8d0;
    cursor: pointer;
    font-size: 1rem;
    margin-top: 0.25rem;
    padding: 0.65rem 1.2rem;
    transition: background 0.15s;
    width: 100%;
  }

  :global(.btn-primary:hover:not(:disabled)) {
    background: #3a8a3a;
  }

  :global(.btn-primary:disabled) {
    cursor: default;
    opacity: 0.5;
  }

  :global(.btn-ghost) {
    background: none;
    border: 1px solid #2a3f2a;
    border-radius: 4px;
    color: #8fad8f;
    cursor: pointer;
    font-size: 0.85rem;
    padding: 0.4rem 0.8rem;
    transition:
      border-color 0.15s,
      color 0.15s;
  }

  :global(.btn-ghost:hover:not(:disabled)) {
    border-color: #4a9e4a;
    color: #d4e8d0;
  }

  :global(.btn-ghost:disabled) {
    cursor: default;
    opacity: 0.5;
  }

  /* ── Shared utility ──────────────────────────────────────────────────────── */
  :global(.error-box) {
    background: #2a1414;
    border: 1px solid #6e2e2e;
    border-radius: 4px;
    color: #cf8f8f;
    font-size: 0.85rem;
    padding: 0.5rem 0.75rem;
  }

  /* ── Top nav bar ─────────────────────────────────────────────────────────── */
  .topbar {
    align-items: center;
    background: #141f14;
    border-bottom: 1px solid #2a3f2a;
    display: flex;
    gap: 1rem;
    padding: 0.75rem 1.5rem;
  }

  .brand {
    color: #6ecf6e;
    flex: 1;
    font-size: 1rem;
    font-weight: 600;
  }

  .user-chip {
    background: #1e2f1e;
    border: 1px solid #2a3f2a;
    border-radius: 12px;
    color: #8fad8f;
    font-size: 0.8rem;
    padding: 0.2rem 0.7rem;
  }

  nav {
    display: flex;
    gap: 0.5rem;
  }

  /* Nav links and buttons share the same look */
  .nav-btn {
    background: none;
    border: 1px solid #2a3f2a;
    border-radius: 4px;
    color: #8fad8f;
    cursor: pointer;
    font-size: 0.85rem;
    padding: 0.4rem 0.8rem;
    text-decoration: none;
    transition:
      border-color 0.15s,
      color 0.15s;
  }

  .nav-btn:hover {
    border-color: #4a9e4a;
    color: #d4e8d0;
  }

  /* ── Page content wrapper ────────────────────────────────────────────────── */
  .content {
    margin: 0 auto;
    max-width: 900px;
    padding: 2rem;
    width: 100%;
  }

  /* Auth pages get a full-viewport wrapper so they can centre vertically */
  .content.full {
    display: flex;
    justify-content: center;
    max-width: 100%;
    min-height: 100vh;
    padding: 0;
  }
</style>
