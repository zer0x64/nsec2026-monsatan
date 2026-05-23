<script lang="ts">
    import { goto } from "$app/navigation";
    import { auth } from "$lib/auth.svelte";
    import { getNext, swipe } from "$lib/api";
    import type { UserResponse } from "$lib/types";

    // ── State ──────────────────────────────────────────────────────────────────

    /** The profile currently on the card, or null when loading / exhausted. */
    let profile = $state<UserResponse | null>(null);

    /** True while an API request is in-flight (prevents double-clicks). */
    let loading = $state(true);

    /** Error message to display, or null when everything is fine. */
    let error = $state<string | null>(null);

    /** Set to true when there are no more profiles to show. */
    let exhausted = $state(false);

    /** Username of the person we just matched with (shows the modal). */
    let matchedWith = $state<string | null>(null);

    // ── Bootstrap ──────────────────────────────────────────────────────────────

    /** Fetch the next profile on mount and after every swipe. */
    async function loadNext() {
        if (!auth.token) return;
        loading = true;
        error = null;
        try {
            profile = await getNext(auth.token);
            if (profile === null) exhausted = true;
        } catch (e) {
            error = e instanceof Error ? e.message : "Something went wrong.";
        } finally {
            loading = false;
        }
    }

    // Load the first card immediately.
    $effect(() => {
        loadNext();
    });

    // ── Actions ────────────────────────────────────────────────────────────────

    /**
     * Record a like or pass, then show the match modal or move to the next card.
     * @param liked - true for "Check!" (like), false for "Pass".
     */
    async function handleSwipe(liked: boolean) {
        if (!auth.token || !profile || loading) return;
        const target = profile.username;
        loading = true;
        try {
            const result = await swipe(auth.token, target, liked);
            if (result.matched) {
                // Show the "Checkmate!" modal before advancing.
                matchedWith = target;
            } else {
                await loadNext();
            }
        } catch (e) {
            error = e instanceof Error ? e.message : "Swipe failed.";
            loading = false;
        }
    }

    /** Dismiss the match modal and load the next card. */
    async function dismissMatch() {
        matchedWith = null;
        await loadNext();
    }

    // ── Helpers ────────────────────────────────────────────────────────────────

    /**
     * Return a chess piece glyph that corresponds to the Elo-style rating.
     * Lower ratings → pawn; higher ratings → king/queen.
     */
    function ratingPiece(rating: number): string {
        if (rating >= 2400) return "♛"; // queen
        if (rating >= 2000) return "♜"; // rook
        if (rating >= 1600) return "♝"; // bishop
        if (rating >= 1200) return "♞"; // knight
        return "♟"; // pawn
    }
</script>

<!-- ── Match modal ──────────────────────────────────────────────────────────── -->
{#if matchedWith}
    <div
        class="modal-backdrop"
        role="dialog"
        aria-modal="true"
        aria-label="Match notification"
    >
        <div class="modal">
            <div class="modal-piece">♛</div>
            <h2 class="modal-title">Checkmate!</h2>
            <p class="modal-sub">
                You and <strong>{matchedWith}</strong> liked each other.
            </p>
            <div class="modal-actions">
                <button
                    class="btn-primary"
                    onclick={() => goto(`/matches/${matchedWith}`)}
                >
                    Send a message
                </button>
                <button class="btn-secondary" onclick={dismissMatch}>
                    Keep discovering
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- ── Main content ─────────────────────────────────────────────────────────── -->
<div class="page">
    <h1 class="page-title">Discover</h1>

    {#if error}
        <div class="error-box" role="alert">
            <span>⚠ {error}</span>
            <button class="retry-btn" onclick={loadNext}>Retry</button>
        </div>
    {:else if loading}
        <div
            class="card placeholder"
            aria-busy="true"
            aria-label="Loading profile"
        >
            <div class="piece-placeholder">♟</div>
            <p class="loading-text">Finding your next opponent…</p>
        </div>
    {:else if exhausted}
        <div class="card exhausted">
            <div class="piece-placeholder">♔</div>
            <h2>No more players</h2>
            <p>
                You've seen everyone on the board.<br />Check back later for new
                challengers.
            </p>
            <a href="/matches" class="btn-primary matches-link"
                >View your matches →</a
            >
        </div>
    {:else if profile}
        <div class="card" aria-label="Profile card for {profile.username}">
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
            <h2 class="username">
                <a
                    href="/profile/{encodeURIComponent(profile.username)}"
                    class="username-link"
                >
                    {profile.username}
                </a>
            </h2>
            <p class="bio">{profile.bio || "No bio yet."}</p>

            <!-- Action buttons -->
            <div class="actions">
                <button
                    class="btn-pass"
                    onclick={() => handleSwipe(false)}
                    disabled={loading}
                    title="Pass"
                >
                    ✕ Pass
                </button>
                <button
                    class="btn-like"
                    onclick={() => handleSwipe(true)}
                    disabled={loading}
                    title="Like"
                >
                    ♟ Check!
                </button>
            </div>
        </div>
    {/if}
</div>

<style>
    /* ── Page layout ────────────────────────────────────────────────────────── */
    .page {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        max-width: 480px;
        gap: 1.5rem;
    }

    .page-title {
        color: #f0d9b5;
        font-size: 1.6rem;
        letter-spacing: 0.05em;
    }

    /* ── Profile card ───────────────────────────────────────────────────────── */
    .card {
        background: #fffdf5;
        border-radius: 12px;
        padding: 2rem;
        width: 100%;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.2rem;
        min-height: 340px;
        justify-content: center;
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

    /* ── Rating badge ───────────────────────────────────────────────────────── */
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

    /* ── Avatar ─────────────────────────────────────────────────────────────── */
    .avatar {
        width: 100px;
        height: 100px;
        border-radius: 50%;
        object-fit: cover;
        border: 3px solid #b58863;
        margin-top: 0.5rem;
    }

    .avatar-placeholder {
        width: 100px;
        height: 100px;
        border-radius: 50%;
        background: #e8d5b0;
        border: 3px solid #b58863;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 3rem;
        margin-top: 0.5rem;
    }

    /* ── Profile text ───────────────────────────────────────────────────────── */
    .username {
        font-size: 1.8rem;
        color: #1a1a1a;
        text-align: center;
    }

    .username-link {
        color: inherit;
        text-decoration: none;
        border-bottom: 2px solid transparent;
        transition:
            border-color 0.15s,
            color 0.15s;
    }

    .username-link:hover {
        color: #5a3e28;
        border-bottom-color: #b58863;
    }

    .bio {
        color: #444;
        text-align: center;
        line-height: 1.6;
        font-size: 0.95rem;
        max-width: 340px;
    }

    /* ── Action buttons ─────────────────────────────────────────────────────── */
    .actions {
        display: flex;
        gap: 1.5rem;
        margin-top: 1rem;
    }

    .btn-pass,
    .btn-like {
        flex: 1;
        padding: 0.75rem 1.5rem;
        border-radius: 8px;
        font-size: 1rem;
        font-weight: bold;
        border: none;
        transition:
            transform 0.1s,
            opacity 0.15s;
    }

    .btn-pass:disabled,
    .btn-like:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .btn-pass:not(:disabled):active,
    .btn-like:not(:disabled):active {
        transform: scale(0.95);
    }

    .btn-pass {
        background: #3a1a1a;
        color: #f0a0a0;
    }

    .btn-pass:not(:disabled):hover {
        background: #4e2020;
    }

    .btn-like {
        background: #1a3a1a;
        color: #90ee90;
    }

    .btn-like:not(:disabled):hover {
        background: #254f25;
    }

    /* ── Placeholder / exhausted states ────────────────────────────────────── */
    .placeholder,
    .exhausted {
        color: #555;
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

    .exhausted h2 {
        color: #333;
        font-size: 1.3rem;
    }

    .exhausted p {
        color: #555;
        text-align: center;
        line-height: 1.6;
    }

    .matches-link {
        display: inline-block;
        margin-top: 0.5rem;
        padding: 0.6rem 1.4rem;
        background: #b58863;
        color: #1a1a1a;
        border-radius: 6px;
        font-weight: bold;
        text-decoration: none;
        transition: background 0.15s;
    }

    .matches-link:hover {
        background: #c99a75;
    }

    /* ── Error box ──────────────────────────────────────────────────────────── */
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
    }

    .retry-btn:hover {
        background: #6a2a2a;
    }

    /* ── Match modal ────────────────────────────────────────────────────────── */
    .modal-backdrop {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.75);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 200;
        padding: 1rem;
    }

    .modal {
        background: #fffdf5;
        border-radius: 16px;
        padding: 2.5rem;
        max-width: 380px;
        width: 100%;
        text-align: center;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        box-shadow: 0 16px 48px rgba(0, 0, 0, 0.6);
        border-top: 5px solid #b58863;
    }

    .modal-piece {
        font-size: 3.5rem;
        line-height: 1;
    }

    .modal-title {
        font-size: 2rem;
        color: #1a1a1a;
        letter-spacing: 0.05em;
    }

    .modal-sub {
        color: #555;
        font-size: 1rem;
        line-height: 1.6;
    }

    .modal-actions {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        width: 100%;
        margin-top: 0.5rem;
    }

    .btn-primary {
        background: #b58863;
        color: #1a1a1a;
        border: none;
        border-radius: 8px;
        padding: 0.75rem 1.5rem;
        font-size: 1rem;
        font-weight: bold;
        width: 100%;
        transition: background 0.15s;
    }

    .btn-primary:hover {
        background: #c99a75;
    }

    .btn-secondary {
        background: transparent;
        color: #888;
        border: 1px solid #ccc;
        border-radius: 8px;
        padding: 0.65rem 1.5rem;
        font-size: 0.9rem;
        width: 100%;
        transition:
            border-color 0.15s,
            color 0.15s;
    }

    .btn-secondary:hover {
        border-color: #999;
        color: #555;
    }
</style>
