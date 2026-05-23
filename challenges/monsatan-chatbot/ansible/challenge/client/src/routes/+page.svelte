<script lang="ts">
    // Ollama message format: { role: "user" | "assistant" | "system", content: string }
    type Role = "user" | "assistant" | "system";

    interface Message {
        role: Role;
        content: string;
    }

    // Each tab holds its own independent conversation state.
    interface Chat {
        id: number;
        name: string;
        messages: Message[];
        isLoading: boolean;
        error: string;
    }

    const API_ENDPOINT = "/api/chat";
    const REBOOT_ENDPOINT = "/api/reboot";

    // Reboot state machine: idle → confirming → loading → success | error
    type RebootState = "idle" | "confirming" | "loading" | "success" | "error";
    let rebootState: RebootState = "idle";
    let rebootError = "";

    // Optional API key sent as a Bearer token with every request.
    let apiKey = "";

    // Build the Authorization header, or return an empty object if no key is set.
    function authHeaders(): Record<string, string> {
        return apiKey.trim()
            ? { Authorization: `Bearer ${apiKey.trim()}` }
            : {};
    }

    async function reboot() {
        if (rebootState === "idle") {
            // First click: ask for confirmation
            rebootState = "confirming";
            return;
        }
        if (rebootState !== "confirming") return;

        rebootState = "loading";
        rebootError = "";
        try {
            const response = await fetch(REBOOT_ENDPOINT, {
                method: "POST",
                headers: { ...authHeaders() },
            });
            if (!response.ok) {
                throw new Error(
                    `Server returned ${response.status}: ${response.statusText}`,
                );
            }
            rebootState = "success";
        } catch (err) {
            rebootError = err instanceof Error ? err.message : String(err);
            rebootState = "error";
        }
    }

    function cancelReboot() {
        rebootState = "idle";
        rebootError = "";
    }

    // Auto-incrementing ID counter for new chats.
    let nextId = 1;

    let chats: Chat[] = [newChat()];
    let activeChatId: number = chats[0].id;

    // Reactive shorthand for the currently visible chat.
    $: activeChat = chats.find((c) => c.id === activeChatId)!;

    let userInput = "";

    // Scroll the message list to the bottom after each DOM update.
    let messageList: HTMLElement;
    function scrollToBottom() {
        setTimeout(() => {
            if (messageList) messageList.scrollTop = messageList.scrollHeight;
        }, 0);
    }

    function newChat(): Chat {
        return {
            id: nextId++,
            name: `Chat ${nextId - 1}`,
            messages: [],
            isLoading: false,
            error: "",
        };
    }

    function addChat() {
        const chat = newChat();
        chats = [...chats, chat];
        activeChatId = chat.id;
        userInput = "";
    }

    function closeChat(id: number) {
        if (chats.length === 1) return; // always keep at least one tab
        const idx = chats.findIndex((c) => c.id === id);
        chats = chats.filter((c) => c.id !== id);
        // If the closed tab was active, move focus to the nearest remaining tab.
        if (activeChatId === id) {
            activeChatId = chats[Math.min(idx, chats.length - 1)].id;
            userInput = "";
        }
    }

    function switchChat(id: number) {
        activeChatId = id;
        userInput = "";
        scrollToBottom();
    }

    // Update a single chat immutably and trigger Svelte reactivity.
    function updateChat(id: number, patch: Partial<Chat>) {
        chats = chats.map((c) => (c.id === id ? { ...c, ...patch } : c));
    }

    async function sendMessage() {
        const trimmed = userInput.trim();
        if (!trimmed || activeChat.isLoading) return;

        // Snapshot the chat ID so the async callbacks update the right tab
        // even if the user switches away before the response arrives.
        const chatId = activeChatId;
        const updatedMessages: Message[] = [
            ...activeChat.messages,
            { role: "user", content: trimmed },
        ];

        userInput = "";
        updateChat(chatId, {
            messages: updatedMessages,
            isLoading: true,
            error: "",
        });
        scrollToBottom();

        try {
            const response = await fetch(API_ENDPOINT, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    ...authHeaders(),
                },
                // Send the full conversation history so the model has context.
                body: JSON.stringify({ messages: updatedMessages }),
            });

            if (!response.ok) {
                throw new Error(
                    `Server returned ${response.status}: ${response.statusText}`,
                );
            }

            const data = await response.json();

            // Ollama wraps the assistant reply under data.message.content.
            const assistantContent: string =
                data?.message?.content ?? JSON.stringify(data);

            const chat = chats.find((c) => c.id === chatId)!;
            updateChat(chatId, {
                messages: [
                    ...chat.messages,
                    { role: "assistant", content: assistantContent },
                ],
                isLoading: false,
            });
        } catch (err) {
            updateChat(chatId, {
                isLoading: false,
                error: err instanceof Error ? err.message : String(err),
            });
        } finally {
            scrollToBottom();
        }
    }

    // Submit on Enter; Shift+Enter inserts a newline.
    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter" && !event.shiftKey) {
            event.preventDefault();
            sendMessage();
        }
    }
</script>

<div class="app">
    <!-- ── Sidebar: branding & context ── -->
    <aside class="sidebar">
        <div class="sidebar-brand">
            <span class="sidebar-icon">🌿</span>
            <h1>TerraBot</h1>
            <p class="sidebar-tagline">Your agrochemical assistant</p>
        </div>

        <div class="sidebar-divider"></div>

        <div class="sidebar-info">
            <p>
                Powered by AI to assist with crop protection, yield
                optimization, and sustainable agriculture practices.
            </p>
            <p>
                Ask about pesticide usage, soil health, harvest planning, or
                regulatory compliance.
            </p>
        </div>

        <!-- Advanced options collapsed at the bottom of the sidebar -->
        <details class="advanced">
            <summary>Advanced</summary>

            <div class="advanced-content">
                <!-- API key is forwarded as an Authorization: Bearer header -->
                <label class="api-key-label" for="api-key">API Key</label>
                <input
                    id="api-key"
                    class="api-key-input"
                    type="password"
                    placeholder="Leave blank for no authentication"
                    bind:value={apiKey}
                />
                {#if rebootState === "success"}
                    <p class="reboot-success">✔ Service is rebooting…</p>
                {:else}
                    {#if rebootState === "confirming"}
                        <p class="reboot-confirm-msg">
                            This will restart the service and drop all active
                            sessions. Are you sure?
                        </p>
                    {/if}
                    {#if rebootState === "error"}
                        <p class="reboot-error">Error: {rebootError}</p>
                    {/if}
                    <div class="reboot-actions">
                        <button
                            class="reboot-btn"
                            class:danger={rebootState === "confirming"}
                            disabled={rebootState === "loading"}
                            on:click={reboot}
                        >
                            {#if rebootState === "loading"}
                                Rebooting…
                            {:else if rebootState === "confirming"}
                                Confirm reboot
                            {:else}
                                Reboot service
                            {/if}
                        </button>
                        {#if rebootState === "confirming"}
                            <button class="cancel-btn" on:click={cancelReboot}>
                                Cancel
                            </button>
                        {/if}
                    </div>
                {/if}
            </div>
        </details>

        <div class="sidebar-footer">
            <span>🌱 Grow responsibly</span>
        </div>
    </aside>

    <!-- ── Main chat panel ── -->
    <div class="chat-panel">
        <!-- Tab bar -->
        <div class="tab-bar">
            {#each chats as chat (chat.id)}
                <button
                    class="tab"
                    class:active={chat.id === activeChatId}
                    on:click={() => switchChat(chat.id)}
                >
                    <span class="tab-name">{chat.name}</span>
                    <!-- Only show close button when more than one tab is open -->
                    {#if chats.length > 1}
                        <span
                            class="tab-close"
                            role="button"
                            tabindex="0"
                            aria-label="Close {chat.name}"
                            on:click|stopPropagation={() => closeChat(chat.id)}
                            on:keydown={(e) =>
                                e.key === "Enter" && closeChat(chat.id)}>×</span
                        >
                    {/if}
                </button>
            {/each}
            <button class="tab-add" on:click={addChat} aria-label="New chat"
                >+</button
            >
        </div>

        <!-- Message history for the active chat -->
        <div class="message-list" bind:this={messageList}>
            {#each activeChat.messages as message}
                <div class="message {message.role}">
                    <span class="role-label">{message.role}</span>
                    <p class="content">{message.content}</p>
                </div>
            {/each}

            {#if activeChat.isLoading}
                <div class="message assistant loading">
                    <span class="role-label">assistant</span>
                    <p class="content">…</p>
                </div>
            {/if}

            {#if activeChat.error}
                <div class="error">Error: {activeChat.error}</div>
            {/if}
        </div>

        <!-- Input area -->
        <form class="input-area" on:submit|preventDefault={sendMessage}>
            <textarea
                bind:value={userInput}
                on:keydown={handleKeydown}
                placeholder="Type a message… (Enter to send, Shift+Enter for newline)"
                rows="4"
                disabled={activeChat.isLoading}
            ></textarea>
            <button
                type="submit"
                disabled={activeChat.isLoading || !userInput.trim()}
            >
                Send
            </button>
        </form>
    </div>
</div>

<style>
    /*
     * ── Solarpunk dark theme ──────────────────────────────────────────────────
     * Palette inspired by deep forest canopies, bioluminescent moss, and warm
     * solar amber — fitting for a company at the intersection of nature and
     * agrochemical technology.
     *
     * --bg-deep      : page background, near-black with a green tint
     * --bg-surface   : card / bubble surfaces
     * --bg-elevated  : input area, slightly lighter than surface
     * --border       : subtle divider lines
     * --text-primary : main readable text
     * --text-muted   : labels, placeholders, secondary text
     * --amber        : solar accent — user bubbles & CTA button
     * --amber-hover  : darker amber for hover states
     * --green-bubble : assistant message bubble
     * --error-*      : error state colours
     */
    :global(body) {
        margin: 0;
        background: #0b1610;
        color: #cce5b4;
        --bg-deep: #0b1610;
        --bg-surface: #132218;
        --bg-elevated: #1a2e20;
        --border: #2b4a30;
        --text-primary: #cce5b4;
        --text-muted: #6b9e60;
        --amber: #b8872a;
        --amber-hover: #9a6e1e;
        --amber-text: #fdeec8;
        --green-bubble: #1b3424;
        --green-bubble-text: #b8dba0;
        --error-bg: #1f0e0e;
        --error-border: #5c2020;
        --error-text: #e07070;
    }

    /* ── Root layout: sidebar + chat panel side by side ── */
    .app {
        display: flex;
        height: 100vh;
        font-family: "Segoe UI", system-ui, sans-serif;
        background: var(--bg-deep);
        color: var(--text-primary);
    }

    /* ── Sidebar ── */
    .sidebar {
        width: 280px;
        flex-shrink: 0;
        display: flex;
        flex-direction: column;
        background: var(--bg-surface);
        border-right: 1px solid var(--border);
        padding: 2rem 1.5rem;
        gap: 1.25rem;
    }

    .sidebar-brand {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        gap: 0.3rem;
    }

    .sidebar-icon {
        font-size: 3rem;
        line-height: 1;
        filter: drop-shadow(0 0 8px #4caf5088);
        margin-bottom: 0.5rem;
    }

    .sidebar-brand h1 {
        margin: 0;
        font-size: 1.6rem;
        font-weight: 700;
        color: var(--text-primary);
        letter-spacing: 0.04em;
    }

    .sidebar-tagline {
        margin: 0;
        font-size: 0.72rem;
        color: var(--text-muted);
        letter-spacing: 0.08em;
        text-transform: uppercase;
    }

    .sidebar-divider {
        border: none;
        border-top: 1px solid var(--border);
    }

    .sidebar-info {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        font-size: 0.9rem;
        color: var(--text-muted);
        line-height: 1.6;
    }

    .sidebar-info p {
        margin: 0;
    }

    .sidebar-footer {
        margin-top: auto;
        font-size: 0.8rem;
        color: var(--text-muted);
        letter-spacing: 0.05em;
    }

    /* ── Chat panel ── */
    .chat-panel {
        flex: 1;
        display: flex;
        flex-direction: column;
        min-width: 0; /* prevent flex child from overflowing */
    }

    /* ── Tab bar ── */
    .tab-bar {
        display: flex;
        align-items: stretch;
        background: var(--bg-surface);
        border-bottom: 1px solid var(--border);
        overflow-x: auto;
        /* Hide scrollbar on the tab strip itself */
        scrollbar-width: none;
        flex-shrink: 0;
    }

    .tab-bar::-webkit-scrollbar {
        display: none;
    }

    .tab {
        display: flex;
        align-items: center;
        gap: 0.4rem;
        padding: 0.6rem 1rem;
        background: transparent;
        border: none;
        border-bottom: 2px solid transparent;
        color: var(--text-muted);
        font-family: inherit;
        font-size: 0.88rem;
        cursor: pointer;
        white-space: nowrap;
        transition:
            color 0.15s,
            border-color 0.15s,
            background 0.15s;
    }

    .tab:hover {
        color: var(--text-primary);
        background: var(--bg-elevated);
    }

    /* Active tab gets an amber underline and full text brightness */
    .tab.active {
        color: var(--text-primary);
        border-bottom-color: var(--amber);
        background: var(--bg-elevated);
    }

    .tab-close {
        font-size: 1rem;
        line-height: 1;
        opacity: 0.45;
        border-radius: 3px;
        padding: 0 2px;
        transition:
            opacity 0.15s,
            color 0.15s;
    }

    .tab-close:hover {
        opacity: 1;
        color: var(--error-text);
    }

    /* "+" button at the end of the tab strip */
    .tab-add {
        padding: 0.6rem 0.9rem;
        background: transparent;
        border: none;
        color: var(--text-muted);
        font-size: 1.2rem;
        cursor: pointer;
        transition: color 0.15s;
        align-self: center;
    }

    .tab-add:hover {
        color: var(--text-primary);
    }

    /* ── Message list ── */
    .message-list {
        flex: 1;
        overflow-y: auto;
        padding: 2rem 3rem;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        scrollbar-width: thin;
        scrollbar-color: var(--border) transparent;
    }

    .message {
        max-width: 60%;
        padding: 0.75rem 1.1rem;
        border-radius: 10px;
        word-break: break-word;
        border: 1px solid transparent;
    }

    /* User messages — warm amber, aligned right */
    .message.user {
        align-self: flex-end;
        background: var(--amber);
        color: var(--amber-text);
        border-color: #d49a3a44;
    }

    /* Assistant messages — deep mossy green, aligned left */
    .message.assistant {
        align-self: flex-start;
        background: var(--green-bubble);
        color: var(--green-bubble-text);
        border-color: var(--border);
    }

    .role-label {
        font-size: 0.65rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.08em;
        opacity: 0.55;
        display: block;
        margin-bottom: 0.3rem;
    }

    .content {
        margin: 0;
        white-space: pre-wrap; /* preserve line breaks from the model */
        line-height: 1.6;
        font-size: 0.97rem;
    }

    /* Loading indicator — gentle pulse */
    .loading .content {
        animation: pulse 1.2s ease-in-out infinite;
    }

    @keyframes pulse {
        0%,
        100% {
            opacity: 1;
        }
        50% {
            opacity: 0.25;
        }
    }

    /* Error banner */
    .error {
        color: var(--error-text);
        background: var(--error-bg);
        border: 1px solid var(--error-border);
        border-radius: 8px;
        padding: 0.6rem 1rem;
        font-size: 0.9rem;
        align-self: stretch;
    }

    /* ── Input area ── */
    .input-area {
        display: flex;
        gap: 0.75rem;
        padding: 1rem 3rem;
        border-top: 1px solid var(--border);
        background: var(--bg-elevated);
    }

    textarea {
        flex: 1;
        resize: vertical;
        padding: 0.65rem 0.9rem;
        font-size: 0.97rem;
        font-family: inherit;
        background: var(--bg-surface);
        color: var(--text-primary);
        border: 1px solid var(--border);
        border-radius: 8px;
        outline: none;
        transition: border-color 0.2s;
    }

    textarea::placeholder {
        color: var(--text-muted);
    }

    textarea:focus {
        border-color: var(--amber);
    }

    textarea:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    button[type="submit"] {
        padding: 0 1.75rem;
        font-size: 0.97rem;
        font-family: inherit;
        font-weight: 600;
        background: var(--amber);
        color: var(--amber-text);
        border: none;
        border-radius: 8px;
        cursor: pointer;
        align-self: flex-end;
        height: 2.6rem;
        letter-spacing: 0.03em;
        transition: background 0.2s;
    }

    button[type="submit"]:disabled {
        background: var(--bg-surface);
        color: var(--text-muted);
        border: 1px solid var(--border);
        cursor: not-allowed;
    }

    button[type="submit"]:not(:disabled):hover {
        background: var(--amber-hover);
    }

    /* ── Advanced section (sidebar) ── */
    .api-key-label {
        display: block;
        font-size: 0.78rem;
        color: var(--text-muted, #9ca3af);
        margin-bottom: 0.25rem;
    }

    .api-key-input {
        width: 100%;
        padding: 0.4rem 0.6rem;
        border-radius: 6px;
        border: 1px solid var(--border, #374151);
        background: var(--input-bg, #1f2937);
        color: var(--text, #e5e7eb);
        font-size: 0.85rem;
        margin-bottom: 0.75rem;
        box-sizing: border-box;
    }

    .api-key-input:focus {
        outline: none;
        border-color: var(--accent, #22c55e);
    }

    .advanced {
        margin-top: auto;
        border-top: 1px solid var(--border);
        padding-top: 1rem;
    }

    .advanced summary {
        font-size: 0.78rem;
        font-weight: 700;
        text-transform: uppercase;
        letter-spacing: 0.08em;
        color: var(--text-muted);
        cursor: pointer;
        user-select: none;
        list-style: none; /* hide default arrow on some browsers */
    }

    /* Custom disclosure triangle */
    .advanced summary::before {
        content: "▶ ";
        font-size: 0.6rem;
        vertical-align: middle;
    }

    .advanced[open] summary::before {
        content: "▼ ";
    }

    .advanced-content {
        display: flex;
        flex-direction: column;
        gap: 0.6rem;
        padding-top: 0.75rem;
    }

    .reboot-confirm-msg {
        margin: 0;
        font-size: 0.82rem;
        color: var(--text-muted);
        line-height: 1.5;
    }

    .reboot-error {
        margin: 0;
        font-size: 0.82rem;
        color: var(--error-text);
    }

    .reboot-success {
        margin: 0;
        font-size: 0.85rem;
        color: #7ecb6e;
    }

    .reboot-actions {
        display: flex;
        gap: 0.5rem;
    }

    /* Base style for reboot / cancel buttons */
    .reboot-btn,
    .cancel-btn {
        padding: 0.4rem 0.85rem;
        font-size: 0.83rem;
        font-family: inherit;
        font-weight: 600;
        border-radius: 6px;
        cursor: pointer;
        border: 1px solid var(--border);
        transition:
            background 0.15s,
            color 0.15s,
            border-color 0.15s;
    }

    /* Default (idle / loading) reboot button — muted */
    .reboot-btn {
        background: var(--bg-elevated);
        color: var(--text-muted);
    }

    .reboot-btn:hover:not(:disabled) {
        color: var(--text-primary);
        border-color: var(--text-muted);
    }

    /* Confirming state — highlight in error red to signal danger */
    .reboot-btn.danger {
        background: var(--error-bg);
        color: var(--error-text);
        border-color: var(--error-border);
    }

    .reboot-btn.danger:hover {
        background: var(--error-border);
        color: #fff;
    }

    .reboot-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    /* Cancel button — plain / ghost */
    .cancel-btn {
        background: transparent;
        color: var(--text-muted);
    }

    .cancel-btn:hover {
        color: var(--text-primary);
        border-color: var(--text-muted);
    }
</style>
