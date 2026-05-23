<script lang="ts">
    import { auth } from "$lib/auth.svelte";
    import { getMatches } from "$lib/api";

    // ── State ──────────────────────────────────────────────────────────────────

    /** Usernames of everyone who mutually liked the current user. */
    let matches = $state<string[]>([]);

    let loading = $state(true);
    let error = $state<string | null>(null);

    // ── Data loading ───────────────────────────────────────────────────────────

    async function loadMatches() {
        if (!auth.token) return;
        loading = true;
        error = null;
        try {
            const data = await getMatches(auth.token);
            matches = data.matches;
        } catch (e) {
            error = e instanceof Error ? e.message : "Failed to load matches.";
        } finally {
            loading = false;
        }
    }

    // Load on mount.
    $effect(() => {
        loadMatches();
    });

    // ── Helpers ────────────────────────────────────────────────────────────────

    /**
     * Deterministically pick a chess piece avatar for a username so each match
     * always shows the same icon (no randomness on re-render).
     */
    const PIECES = ["♙", "♘", "♗", "♖", "♕", "♔"];
    function avatarPiece(username: string): string {
        let hash = 0;
        for (const ch of username) hash = (hash * 31 + ch.charCodeAt(0)) >>> 0;
        return PIECES[hash % PIECES.length];
    }
</script>

<div class="page">
    <h1 class="page-title">♛ Your Matches</h1>

    {#if error}
        <div class="error-box" role="alert">
            <span>⚠ {error}</span>
            <button class="retry-btn" onclick={loadMatches}>Retry</button>
        </div>
    {:else if loading}
        <p class="status-text">Scanning the board…</p>
    {:else if matches.length === 0}
        <!-- Empty state -->
        <div class="empty">
            <span class="empty-piece">♟</span>
            <h2>No matches yet</h2>
            <p>Keep discovering — your perfect opponent is still out there.</p>
            <a href="/swipe" class="discover-link">♟ Start discovering</a>
        </div>
    {:else}
        <p class="count">
            {matches.length} mutual {matches.length === 1 ? "match" : "matches"}
        </p>

        <ul class="match-list" aria-label="Matches">
            {#each matches as username (username)}
                <li class="match-card">
                    <span class="avatar" aria-hidden="true"
                        >{avatarPiece(username)}</span
                    >
                    <div class="info">
                        <!-- Profile link: shows the username, goes to /profile -->
                        <a
                            href="/profile/{encodeURIComponent(username)}"
                            class="username">{username}</a
                        >
                        <span class="hint">
                            <!-- Chat link: separate from the profile link -->
                            <a
                                href="/matches/{encodeURIComponent(username)}"
                                class="chat-link">Open conversation →</a
                            >
                        </span>
                    </div>
                </li>
            {/each}
        </ul>
    {/if}
</div>

<style>
    /* ── Page layout ─────────────────────────────────────────────────────────── */
    .page {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        max-width: 560px;
        gap: 1.25rem;
    }

    .page-title {
        color: #f0d9b5;
        font-size: 1.6rem;
        letter-spacing: 0.05em;
        align-self: flex-start;
    }

    /* ── Status / count text ─────────────────────────────────────────────────── */
    .status-text {
        color: #b58863;
        font-style: italic;
    }

    .count {
        color: #c0a880;
        font-size: 0.85rem;
        align-self: flex-start;
    }

    /* ── Match list ──────────────────────────────────────────────────────────── */
    .match-list {
        list-style: none;
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.6rem;
    }

    /* Each match row — no longer an <a> itself to avoid nested anchors. */
    .match-card {
        display: flex;
        align-items: center;
        gap: 1rem;
        background: #fffdf5;
        border-radius: 10px;
        padding: 1rem 1.25rem;
        box-shadow: 0 4px 14px rgba(0, 0, 0, 0.35);
        border-left: 4px solid #b58863;
        list-style: none;
    }

    /* ── Avatar piece ────────────────────────────────────────────────────────── */
    .avatar {
        font-size: 2rem;
        line-height: 1;
        flex-shrink: 0;
        width: 2.5rem;
        text-align: center;
    }

    /* ── Info block ──────────────────────────────────────────────────────────── */
    .info {
        display: flex;
        flex-direction: column;
        gap: 0.15rem;
    }

    .username {
        font-size: 1.05rem;
        font-weight: bold;
        color: #1a1a1a;
        text-decoration: none;
    }

    .username:hover {
        text-decoration: underline;
        color: #5a3e28;
    }

    .hint {
        font-size: 0.78rem;
        color: #8a7060;
        font-style: italic;
    }

    /* Inline chat link inside the hint line */
    .chat-link {
        color: #8a7060;
        text-decoration: none;
        transition: color 0.15s;
    }

    .chat-link:hover {
        color: #5a3e28;
        text-decoration: underline;
    }

    /* ── Error box ───────────────────────────────────────────────────────────── */
    .error-box {
        display: flex;
        align-items: center;
        gap: 1rem;
        background: #3a1a1a;
        color: #f0a0a0;
        border-radius: 8px;
        padding: 0.8rem 1.2rem;
        width: 100%;
        font-size: 0.9rem;
    }

    .retry-btn {
        margin-left: auto;
        background: #4e2020;
        color: #f0c0c0;
        border: none;
        border-radius: 4px;
        padding: 0.3rem 0.7rem;
        font-size: 0.85rem;
        cursor: pointer;
    }

    .retry-btn:hover {
        background: #6a2a2a;
    }

    /* ── Empty state ─────────────────────────────────────────────────────────── */
    .empty {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.75rem;
        background: #fffdf5;
        border-radius: 12px;
        padding: 2.5rem 2rem;
        width: 100%;
        text-align: center;
        box-shadow: 0 6px 24px rgba(0, 0, 0, 0.4);
    }

    .empty-piece {
        font-size: 3.5rem;
        opacity: 0.3;
    }

    .empty h2 {
        font-size: 1.3rem;
        color: #333;
    }

    .empty p {
        color: #666;
        line-height: 1.6;
        font-size: 0.95rem;
    }

    .discover-link {
        margin-top: 0.5rem;
        display: inline-block;
        padding: 0.65rem 1.4rem;
        background: #b58863;
        color: #1a1a1a;
        border-radius: 6px;
        font-weight: bold;
        text-decoration: none;
        font-size: 0.95rem;
        transition: background 0.15s;
    }

    .discover-link:hover {
        background: #c99a75;
    }
</style>
