<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import { auth } from '$lib/auth.svelte';

  // SvelteKit 5: layout children are passed as a snippet prop
  let { children } = $props();

  // Auth redirect guard: runs whenever the token or the current path changes.
  // - Unauthenticated users are sent to /auth (unless already there).
  // - Authenticated users visiting /auth are sent to /swipe.
  $effect(() => {
    if (!auth.isAuthenticated && page.url.pathname !== '/auth') {
      goto('/auth');
    } else if (auth.isAuthenticated && page.url.pathname === '/auth') {
      goto('/swipe');
    }
  });

  function handleLogout() {
    auth.logout();
    goto('/auth');
  }
</script>

<div class="app">
  {#if auth.isAuthenticated}
    <header>
      <span class="logo">♚ Checkmate</span>

      <nav>
        <a href="/swipe" class:active={page.url.pathname === '/swipe'}>
          ♟ Discover
        </a>
        <a
          href="/matches"
          class:active={page.url.pathname.startsWith('/matches')}
        >
          ♛ Matches
        </a>
      </nav>

      <button class="resign-btn" onclick={handleLogout}>Resign</button>
    </header>
  {/if}

  <main>
    {@render children()}
  </main>
</div>

<style>
  /* ── Reset & base ─────────────────────────────────────────── */
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: 'Georgia', serif;
    /* Subtle checkerboard using two near-identical dark squares */
    background-color: #2b2b2b;
    background-image:
      linear-gradient(45deg, #323232 25%, transparent 25%),
      linear-gradient(-45deg, #323232 25%, transparent 25%),
      linear-gradient(45deg, transparent 75%, #323232 75%),
      linear-gradient(-45deg, transparent 75%, #323232 75%);
    background-size: 40px 40px;
    background-position: 0 0, 0 20px, 20px -20px, -20px 0;
    min-height: 100vh;
    color: #1a1a1a;
  }

  :global(button) {
    cursor: pointer;
    font-family: inherit;
  }

  :global(input), :global(textarea) {
    font-family: inherit;
  }

  :global(a) {
    text-decoration: none;
  }

  /* ── App shell ────────────────────────────────────────────── */
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  /* ── Header / Navigation ──────────────────────────────────── */
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 1.5rem;
    height: 56px;
    background: #1a1a1a;
    border-bottom: 3px solid #b58863; /* chess dark-square amber */
    position: sticky;
    top: 0;
    z-index: 100;
  }

  .logo {
    font-size: 1.4rem;
    font-weight: bold;
    color: #f0d9b5; /* chess light-square cream */
    letter-spacing: 0.05em;
    user-select: none;
  }

  nav {
    display: flex;
    gap: 0.5rem;
  }

  nav a {
    display: inline-block;
    padding: 0.35rem 1rem;
    border-radius: 4px;
    color: #c0b090;
    font-size: 0.95rem;
    transition: background 0.15s, color 0.15s;
  }

  nav a:hover {
    background: #2e2e2e;
    color: #f0d9b5;
  }

  nav a.active {
    background: #b58863;
    color: #1a1a1a;
    font-weight: bold;
  }

  .resign-btn {
    background: transparent;
    border: 1px solid #555;
    color: #888;
    padding: 0.3rem 0.8rem;
    border-radius: 4px;
    font-size: 0.85rem;
    transition: border-color 0.15s, color 0.15s;
  }

  .resign-btn:hover {
    border-color: #b05050;
    color: #c06060;
  }

  /* ── Main content ─────────────────────────────────────────── */
  main {
    flex: 1;
    display: flex;
    justify-content: center;
    padding: 2rem 1rem;
  }
</style>
