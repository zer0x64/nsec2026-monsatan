/**
 * Reactive authentication store.
 *
 * Uses Svelte 5 runes so any component that reads `auth.token`,
 * `auth.username`, or `auth.isAuthenticated` will re-render automatically
 * when the value changes.
 *
 * Safe to instantiate at module level because SSR is disabled for the whole
 * app (`ssr = false` in +layout.ts), so this code always runs in the browser
 * where `localStorage` is available.
 */
class AuthStore {
  /** The JWT returned by /api/login or /api/register, or null when logged out. */
  token = $state<string | null>(localStorage.getItem("token"));

  /** Convenience getter – true when a token is present. */
  get isAuthenticated(): boolean {
    return this.token !== null;
  }

  /**
   * The username of the currently logged-in user, decoded from the JWT
   * `sub` claim.  Returns null when no token is stored or the token is
   * malformed.
   */
  get username(): string | null {
    if (!this.token) return null;
    try {
      // JWTs are three base64url segments separated by dots.
      // The middle segment is the JSON payload.
      const payloadB64 = this.token.split(".")[1];
      // base64url → base64 → JSON string
      const json = atob(payloadB64.replace(/-/g, "+").replace(/_/g, "/"));
      const payload = JSON.parse(json) as Record<string, unknown>;
      // The backend signs with the username stored in the `sub` claim.
      return typeof payload.sub === "string" ? payload.sub : null;
    } catch {
      return null;
    }
  }

  /** Call after a successful login or registration to persist the token. */
  setToken(token: string) {
    this.token = token;
    localStorage.setItem("token", token);
  }

  /** Clear the token and remove it from storage. */
  logout() {
    this.token = null;
    localStorage.removeItem("token");
  }
}

/** Singleton instance shared across all components. */
export const auth = new AuthStore();
