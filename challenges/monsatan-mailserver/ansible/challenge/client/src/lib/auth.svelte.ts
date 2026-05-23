/**
 * auth.svelte.ts — Shared authentication state.
 *
 * Using a .svelte.ts file allows $state runes at module level,
 * making the auth object reactive across all routes.
 */

// ── Reactive auth state ───────────────────────────────────────────────────────

export const auth = $state({
  token: null as string | null,
  /** Full email address, e.g. alice@monsatan.ctf */
  email: null as string | null,
});

// ── Helpers ───────────────────────────────────────────────────────────────────

/** Read the session from localStorage and populate the store. Call once on app mount. */
export function initAuth(): void {
  auth.token = localStorage.getItem("token");
  auth.email = localStorage.getItem("email");
}

/** Persist a new session to the store and localStorage. */
export function saveAuth(token: string, email: string): void {
  auth.token = token;
  auth.email = email;
  localStorage.setItem("token", token);
  localStorage.setItem("email", email);
}

/** Clear the session from the store and localStorage. */
export function clearAuth(): void {
  auth.token = null;
  auth.email = null;
  localStorage.removeItem("token");
  localStorage.removeItem("email");
}
