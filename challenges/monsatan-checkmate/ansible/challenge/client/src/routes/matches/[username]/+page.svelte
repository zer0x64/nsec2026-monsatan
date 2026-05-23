<script lang="ts">
    import { page } from "$app/state";
    import { auth } from "$lib/auth.svelte";
    import { getContact, getMessages, sendMessage } from "$lib/api";
    import type { ContactResponse, MessageResponse } from "$lib/types";

    // Username comes from the [username] dynamic route segment.
    // Use a fallback empty string so the type is `string` (not `string | undefined`).
    const username = $derived(page.params.username ?? "");

    // ── State ──────────────────────────────────────────────────────────────────

    let messages = $state<MessageResponse[]>([]);
    let contact = $state<ContactResponse | null>(null);

    /** Tracks whether the initial data is still loading. */
    let loadingMessages = $state(true);
    let loadingContact = $state(true);

    let messageError = $state<string | null>(null);
    let contactError = $state<string | null>(null);

    /** The text in the message input field. */
    let draft = $state("");

    /** True while a send is in-flight (prevents double-sends). */
    let sending = $state(false);

    /**
     * Reference to the message list container, used to auto-scroll
     * to the bottom whenever new messages arrive.
     */
    let messageListEl = $state<HTMLElement | null>(null);

    // ── Data fetching ──────────────────────────────────────────────────────────

    /** Fetch the full message history for this conversation. */
    async function loadMessages() {
        // Capture into a local const so TypeScript can narrow the type.
        const token = auth.token;
        if (!token) return;
        loadingMessages = true;
        messageError = null;
        try {
            messages = await getMessages(token, username);
        } catch (e) {
            messageError =
                e instanceof Error ? e.message : "Failed to load messages.";
        } finally {
            loadingMessages = false;
        }
    }

    /** Fetch private contact info (only available because we are matched). */
    async function loadContact() {
        // Capture into a local const so TypeScript can narrow the type.
        const token = auth.token;
        if (!token) return;
        loadingContact = true;
        contactError = null;
        try {
            contact = await getContact(token, username);
        } catch (e) {
            contactError =
                e instanceof Error ? e.message : "Contact info unavailable.";
        } finally {
            loadingContact = false;
        }
    }

    // Load everything when the component mounts (or when the username changes).
    $effect(() => {
        // Track `username` so the effect re-runs on navigation between match pages.
        const _ = username;
        loadMessages();
        loadContact();
    });

    // Auto-scroll to the bottom of the chat whenever the message list changes.
    $effect(() => {
        if (messageListEl && messages.length > 0) {
            messageListEl.scrollTop = messageListEl.scrollHeight;
        }
    });

    // ── Actions ────────────────────────────────────────────────────────────────

    async function handleSend() {
        const content = draft.trim();
        // Capture into a local const so TypeScript can narrow the type.
        const token = auth.token;
        if (!content || !token || sending) return;

        sending = true;
        draft = "";
        try {
            const newMessage = await sendMessage(token, username, content);
            messages = [...messages, newMessage];
        } catch (e) {
            // Restore the draft so the user doesn't lose their text.
            draft = content;
            messageError =
                e instanceof Error ? e.message : "Failed to send message.";
        } finally {
            sending = false;
        }
    }

    // Allow submitting with Enter (Shift+Enter inserts a newline).
    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            handleSend();
        }
    }

    // ── Helpers ────────────────────────────────────────────────────────────────

    /** Format an ISO timestamp to a short human-readable string. */
    function formatTime(iso: string): string {
        try {
            return new Date(iso).toLocaleString(undefined, {
                month: "short",
                day: "numeric",
                hour: "2-digit",
                minute: "2-digit",
            });
        } catch {
            return iso;
        }
    }
</script>

<div class="page">
    <!-- ── Back link ──────────────────────────────────────────────────────────── -->
    <a href="/matches" class="back-link">← All matches</a>

    <div class="layout">
        <!-- ── Chat column ──────────────────────────────────────────────────────── -->
        <section class="chat-column" aria-label="Conversation with {username}">
            <div class="chat-header">
                <span class="chat-piece">♙</span>
                <h1 class="chat-title">
                    <a
                        href="/profile/{encodeURIComponent(username)}"
                        class="profile-link"
                    >
                        {username}
                    </a>
                </h1>
            </div>

            <!-- Message list -->
            <div
                class="message-list"
                bind:this={messageListEl}
                role="log"
                aria-live="polite"
                aria-label="Messages"
            >
                {#if loadingMessages}
                    <p class="state-text">Loading conversation…</p>
                {:else if messageError}
                    <div class="error-box" role="alert">
                        <span>⚠ {messageError}</span>
                        <button class="retry-btn" onclick={loadMessages}
                            >Retry</button
                        >
                    </div>
                {:else if messages.length === 0}
                    <p class="state-text">
                        No messages yet. Make the first move!
                    </p>
                {:else}
                    {#each messages as msg (msg.id)}
                        <div
                            class="message"
                            class:mine={msg.sender === auth.username}
                            class:theirs={msg.sender !== auth.username}
                        >
                            <div class="bubble">
                                <p class="content">{msg.content}</p>
                                <time class="timestamp" datetime={msg.sentAt}>
                                    {formatTime(msg.sentAt)}
                                </time>
                            </div>
                        </div>
                    {/each}
                {/if}
            </div>

            <!-- Compose area -->
            <form
                class="compose"
                onsubmit={(e) => {
                    e.preventDefault();
                    handleSend();
                }}
                aria-label="Send message"
            >
                <textarea
                    bind:value={draft}
                    onkeydown={handleKeydown}
                    placeholder="Your move… (Enter to send)"
                    rows="2"
                    disabled={sending}
                    aria-label="Message text"
                ></textarea>
                <button
                    type="submit"
                    class="send-btn"
                    disabled={!draft.trim() || sending}
                    aria-label="Send"
                >
                    {sending ? "…" : "♟"}
                </button>
            </form>
        </section>

        <!-- ── Contact info panel ──────────────────────────────────────────────── -->
        <aside class="contact-panel" aria-label="Contact information">
            <h2 class="contact-title">♞ Contact Info</h2>

            {#if loadingContact}
                <p class="state-text">Loading…</p>
            {:else if contactError}
                <p class="contact-error">{contactError}</p>
            {:else if contact}
                <dl class="contact-list">
                    {#if contact.email}
                        <div class="contact-row">
                            <dt>✉ Email</dt>
                            <dd>
                                <a href="mailto:{contact.email}"
                                    >{contact.email}</a
                                >
                            </dd>
                        </div>
                    {/if}

                    {#if contact.phone}
                        <div class="contact-row">
                            <dt>☎ Phone</dt>
                            <dd>
                                <a href="tel:{contact.phone}">{contact.phone}</a
                                >
                            </dd>
                        </div>
                    {/if}

                    {#if contact.location}
                        <div class="contact-row">
                            <dt>📍 Location</dt>
                            <dd>{contact.location}</dd>
                        </div>
                    {/if}

                    {#if !contact.email && !contact.phone && !contact.location}
                        <p class="state-text">
                            This player hasn't shared contact details.
                        </p>
                    {/if}
                </dl>
            {/if}
        </aside>
    </div>
</div>

<style>
    /* ── Page ─────────────────────────────────────────────────────────────────── */
    .page {
        display: flex;
        flex-direction: column;
        width: 100%;
        max-width: 900px;
        gap: 1rem;
    }

    .back-link {
        color: #b58863;
        font-size: 0.9rem;
        transition: color 0.15s;
        align-self: flex-start;
    }

    .back-link:hover {
        color: #f0d9b5;
    }

    /* ── Two-column layout ───────────────────────────────────────────────────── */
    .layout {
        display: grid;
        grid-template-columns: 1fr 260px;
        gap: 1.25rem;
        align-items: start;
    }

    @media (max-width: 640px) {
        /* Stack vertically on small screens */
        .layout {
            grid-template-columns: 1fr;
        }
    }

    /* ── Chat column ─────────────────────────────────────────────────────────── */
    .chat-column {
        background: #fffdf5;
        border-radius: 10px;
        box-shadow: 0 6px 24px rgba(0, 0, 0, 0.45);
        display: flex;
        flex-direction: column;
        overflow: hidden;
        /* Keep the panel a fixed height so the list scrolls */
        height: calc(100vh - 160px);
        min-height: 400px;
    }

    .chat-header {
        display: flex;
        align-items: center;
        gap: 0.6rem;
        padding: 1rem 1.25rem;
        background: #1a1a1a;
        border-bottom: 3px solid #b58863;
        flex-shrink: 0;
    }

    .chat-piece {
        font-size: 1.4rem;
        color: #f0d9b5;
    }

    .chat-title {
        font-size: 1.2rem;
        color: #f0d9b5;
        font-weight: bold;
        letter-spacing: 0.04em;
    }

    /* Username link in the chat header */
    .profile-link {
        color: inherit;
        text-decoration: none;
        border-bottom: 1px dotted #b58863;
        transition:
            color 0.15s,
            border-color 0.15s;
    }

    .profile-link:hover {
        color: #f0d9b5;
        border-bottom-color: #f0d9b5;
    }

    /* ── Message list ────────────────────────────────────────────────────────── */
    .message-list {
        flex: 1;
        overflow-y: auto;
        padding: 1rem;
        display: flex;
        flex-direction: column;
        gap: 0.6rem;
        scroll-behavior: smooth;
    }

    .state-text {
        color: #888;
        font-style: italic;
        text-align: center;
        margin: auto;
        font-size: 0.9rem;
    }

    /* ── Individual messages ─────────────────────────────────────────────────── */
    .message {
        display: flex;
    }

    /* Messages sent by the current user align to the right */
    .message.mine {
        justify-content: flex-end;
    }

    /* Messages from the other user align to the left */
    .message.theirs {
        justify-content: flex-start;
    }

    .bubble {
        max-width: 70%;
        padding: 0.55rem 0.9rem;
        border-radius: 14px;
        display: flex;
        flex-direction: column;
        gap: 0.2rem;
    }

    .mine .bubble {
        background: #1a3a1a;
        border-bottom-right-radius: 4px;
    }

    .theirs .bubble {
        background: #2a2a2a;
        border-bottom-left-radius: 4px;
    }

    .content {
        color: #f0f0f0;
        font-size: 0.95rem;
        line-height: 1.5;
        word-break: break-word;
        white-space: pre-wrap;
    }

    .timestamp {
        color: #888;
        font-size: 0.7rem;
        align-self: flex-end;
    }

    /* ── Compose area ────────────────────────────────────────────────────────── */
    .compose {
        display: flex;
        gap: 0.5rem;
        padding: 0.75rem;
        border-top: 2px solid #e8d4b4;
        background: #fdf5e6;
        flex-shrink: 0;
        align-items: flex-end;
    }

    .compose textarea {
        flex: 1;
        padding: 0.55rem 0.75rem;
        border: 1px solid #c4a882;
        border-radius: 8px;
        resize: none;
        font-size: 0.95rem;
        background: #fffef8;
        color: #1a1a1a;
        line-height: 1.4;
        transition:
            border-color 0.15s,
            box-shadow 0.15s;
    }

    .compose textarea:focus {
        outline: none;
        border-color: #8b6340;
        box-shadow: 0 0 0 2px rgba(139, 99, 64, 0.25);
    }

    .compose textarea:disabled {
        background: #f0e8d8;
        color: #888;
    }

    .send-btn {
        width: 44px;
        height: 44px;
        border-radius: 8px;
        border: none;
        background: #5a8a40;
        color: #fff;
        font-size: 1.3rem;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        transition:
            background 0.15s,
            transform 0.1s;
    }

    .send-btn:hover:not(:disabled) {
        background: #4a7a30;
        transform: translateY(-1px);
    }

    .send-btn:disabled {
        background: #8aaa78;
        cursor: not-allowed;
    }

    /* ── Error box ───────────────────────────────────────────────────────────── */
    .error-box {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        background: #3a1a1a;
        color: #f0a0a0;
        border-radius: 8px;
        padding: 0.7rem 1rem;
        font-size: 0.875rem;
    }

    .retry-btn {
        margin-left: auto;
        background: #4e2020;
        color: #f0c0c0;
        border: none;
        border-radius: 4px;
        padding: 0.25rem 0.6rem;
        font-size: 0.8rem;
        cursor: pointer;
    }

    .retry-btn:hover {
        background: #6a2a2a;
    }

    /* ── Contact panel ───────────────────────────────────────────────────────── */
    .contact-panel {
        background: #fffdf5;
        border-radius: 10px;
        padding: 1.25rem;
        box-shadow: 0 6px 24px rgba(0, 0, 0, 0.45);
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .contact-title {
        font-size: 1rem;
        color: #1a1a1a;
        padding-bottom: 0.6rem;
        border-bottom: 2px solid #b58863;
        letter-spacing: 0.03em;
    }

    .contact-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .contact-row {
        display: flex;
        flex-direction: column;
        gap: 0.15rem;
    }

    .contact-row dt {
        font-size: 0.75rem;
        font-weight: bold;
        color: #7a5a38;
        text-transform: uppercase;
        letter-spacing: 0.06em;
    }

    .contact-row dd {
        font-size: 0.9rem;
        color: #333;
        word-break: break-word;
    }

    .contact-row dd a {
        color: #5a8a40;
        transition: color 0.15s;
    }

    .contact-row dd a:hover {
        color: #3a6a20;
    }

    .contact-error {
        color: #9a4040;
        font-size: 0.875rem;
        font-style: italic;
    }
</style>
