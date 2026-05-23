<script lang="ts">
    import { page } from "$app/state";
    import { auth } from "$lib/auth.svelte";
    import { getUser } from "$lib/api";
    import type { UserRow } from "$lib/types";

    // Username comes from the [username] dynamic route segment.
    const username = $derived(page.params.username ?? "");

    // ── State ──────────────────────────────────────────────────────────────────

    let profile = $state<UserRow | null>(null);
    let loading = $state(true);
    let error = $state<string | null>(null);

    // ── Data fetching ──────────────────────────────────────────────────────────

    async function loadProfile() {
        const token = auth.token;
        if (!token) return;
        loading = true;
        error = null;
        try {
            profile = await getUser(token, username);
        } catch (e) {
            error = e instanceof Error ? e.message : "Failed to load profile.";
        } finally {
            loading = false;
        }
    }

    // Re-fetch whenever the route username changes.
    $effect(() => {
        const _ = username;
        loadProfile();
    });

    // ── Helpers ────────────────────────────────────────────────────────────────

    /**
     * Return a chess piece glyph that corresponds to the Elo-style rating.
     * Lower ratings → pawn; higher ratings → queen.
     */
    function ratingPiece(rating: number): string {
        if (rating >= 2400) return "♛";
        if (rating >= 2000) return "♜";
        if (rating >= 1600) return "♝";
        if (rating >= 1200) return "♞";
        return "♟";
    }
</script>

<div class="page">
    <!-- ── Back link ────────────────────────────────────────────────────────── -->
    <button class="back-link" onclick={() => history.back()}>← Back</button>

    {#if error}
        <div class="error-box" role="alert">
            <span>⚠ {error}</span>
            <button class="retry-btn" onclick={loadProfile}>Retry</button>
        </div>
    {:else if loading}
        <div
            class="card placeholder"
            aria-busy="true"
            aria-label="Loading profile"
        >
            <div class="piece-placeholder">♟</div>
            <p class="loading-text">Loading profile…</p>
        </div>
    {:else if profile}
        <div class="card">
            <!-- Rating badge -->
            <div class="rating-badge" title="Elo rating">
                {ratingPiece(profile.rating)}
                <span>{profile.rating} Elo</span>
            </div>

            <!-- Avatar -->
            {#if profile.profilePicture}
                <img
                    class="avatar"
                    src="data:image/*;base64,{profile.profilePicture}"
                    alt="Profile picture of {profile.username}"
                />
            {:else}
                <div class="avatar-placeholder" aria-hidden="true">
                    {ratingPiece(profile.rating)}
                </div>
            {/if}

            <!-- Profile info -->
            <h1 class="username">{profile.username}</h1>
            <p class="bio">{profile.bio || "No bio yet."}</p>

            <!-- Link to chat if this person is a match -->
            <a
                href="/matches/{encodeURIComponent(profile.username)}"
                class="message-link"
                title="Open conversation"
            >
                ♟ Send a message
            </a>
        </div>
    {/if}
</div>

<style>
    /* ── Page layout ─────────────────────────────────────────────────────────── */
    .page {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        max-width: 480px;
        gap: 1.25rem;
    }

    .back-link {
        background: none;
        border: none;
        padding: 0;
        color: #b58863;
        font-size: 0.9rem;
        font-family: inherit;
        cursor: pointer;
        transition: color 0.15s;
        align-self: flex-start;
        text-decoration: none;
    }

    .back-link:hover {
        color: #f0d9b5;
    }

    /* ── Profile card ────────────────────────────────────────────────────────── */
    .card {
        background: #fffdf5;
        border-radius: 12px;
        padding: 2.5rem 2rem;
        width: 100%;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.2rem;
        position: relative;
    }

    /* Decorative corner squares mimicking a chessboard */
    .card::before,
    .card::after {
        content: "";
        position: absolute;
        width: 20px;
        height: 20px;
        background: #b58863;
        border-radius: 3px;
        opacity: 0.4;
    }

    .card::before {
        top: 10px;
        left: 10px;
    }

    .card::after {
        bottom: 10px;
        right: 10px;
    }

    /* ── Avatar ──────────────────────────────────────────────────────────────── */
    .avatar {
        width: 120px;
        height: 120px;
        border-radius: 50%;
        object-fit: cover;
        border: 3px solid #b58863;
        margin-top: 0.5rem;
    }

    .avatar-placeholder {
        width: 120px;
        height: 120px;
        border-radius: 50%;
        background: #e8d5b0;
        border: 3px solid #b58863;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 3.5rem;
        margin-top: 0.5rem;
    }

    /* ── Rating badge ────────────────────────────────────────────────────────── */
    .rating-badge {
        position: absolute;
        top: 12px;
        right: 12px;
        display: flex;
        align-items: center;
        gap: 4px;
        background: #1a1a1a;
        color: #f0d9b5;
        border-radius: 20px;
        padding: 4px 10px;
        font-size: 0.8rem;
    }

    /* ── Profile text ────────────────────────────────────────────────────────── */
    .username {
        font-size: 2rem;
        color: #1a1a1a;
        text-align: center;
        /* Push down so the rating badge doesn't overlap */
        margin-top: 1rem;
    }

    .bio {
        color: #444;
        text-align: center;
        line-height: 1.6;
        font-size: 0.95rem;
        max-width: 340px;
    }

    /* ── Message link ────────────────────────────────────────────────────────── */
    .message-link {
        margin-top: 0.5rem;
        display: inline-block;
        padding: 0.65rem 1.6rem;
        background: #b58863;
        color: #1a1a1a;
        border-radius: 6px;
        font-weight: bold;
        font-size: 0.95rem;
        text-decoration: none;
        transition:
            background 0.15s,
            transform 0.1s;
    }

    .message-link:hover {
        background: #c99a75;
        transform: translateY(-1px);
    }

    .message-link:active {
        transform: translateY(0);
    }

    /* ── Placeholder state ───────────────────────────────────────────────────── */
    .placeholder {
        min-height: 280px;
        justify-content: center;
        gap: 0.8rem;
    }

    .piece-placeholder {
        font-size: 4rem;
        opacity: 0.3;
    }

    .loading-text {
        color: #666;
        font-style: italic;
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
</style>
