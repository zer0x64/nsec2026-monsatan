<!--
  inbox/+page.svelte — Inbox page.

  Loads the mail list on mount and displays it in a table.
  Each row links to /mail/[id] for the full detail view.
-->
<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { auth } from "$lib/auth.svelte";
    import { apiListMails } from "$lib/api";
    import type { MailSummary } from "$lib/api";

    let mails = $state<MailSummary[]>([]);
    let loading = $state(false);
    let error = $state<string | null>(null);

    onMount(() => {
        if (!auth.token) {
            goto("/login");
            return;
        }
        loadInbox();
    });

    async function loadInbox() {
        loading = true;
        error = null;
        try {
            mails = await apiListMails(auth.token!);
        } catch (err: any) {
            error = err.message;
        } finally {
            loading = false;
        }
    }

    function formatDate(iso: string): string {
        return new Date(iso).toLocaleString();
    }
</script>

<svelte:head>
    <title>Inbox — Monsatan Mail™</title>
</svelte:head>

<div class="pane">
    <div class="pane-header">
        <h2>Inbox</h2>
        <button class="btn-ghost" onclick={loadInbox} disabled={loading}>
            {loading ? "Loading…" : "↻ Refresh"}
        </button>
    </div>

    {#if error}
        <p class="error-box">{error}</p>
    {/if}

    {#if !loading && mails.length === 0}
        <p class="empty-state">
            No messages yet. Seems even the algae-powered data centre has nothing for you.
        </p>
    {:else}
        <table class="mail-list">
            <thead>
                <tr>
                    <th></th>
                    <th>From</th>
                    <th>Subject</th>
                    <th>Date</th>
                </tr>
            </thead>
            <tbody>
                {#each mails as mail (mail.id)}
                    <tr
                        class="mail-row"
                        class:unread={!mail.is_read}
                        onclick={() => goto(`/mail/${mail.id}`)}
                        role="button"
                        tabindex="0"
                        onkeydown={(e) => e.key === "Enter" && goto(`/mail/${mail.id}`)}
                    >
                        <td class="dot-cell">
                            {#if !mail.is_read}
                                <span class="unread-dot" title="Unread"></span>
                            {/if}
                        </td>
                        <td class="from-cell">{mail.from}</td>
                        <td class="subject-cell">{mail.subject}</td>
                        <td class="date-cell">{formatDate(mail.sent_at)}</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    {/if}
</div>

<style>
    .pane {
        display: flex;
        flex-direction: column;
        gap: 1.25rem;
    }

    .pane-header {
        align-items: center;
        display: flex;
        justify-content: space-between;
    }

    .pane-header h2 {
        color: #6ecf6e;
        font-size: 1.2rem;
        font-weight: 600;
    }

    /* ── Mail table ──────────────────────────────────────────────────────── */
    .mail-list {
        border-collapse: collapse;
        width: 100%;
    }

    .mail-list th {
        border-bottom: 1px solid #2a3f2a;
        color: #4a6a4a;
        font-size: 0.75rem;
        font-weight: 600;
        letter-spacing: 0.06em;
        padding: 0.4rem 0.6rem;
        text-align: left;
        text-transform: uppercase;
    }

    .mail-row {
        border-bottom: 1px solid #1a2a1a;
        color: #8fad8f;
        cursor: pointer;
        transition: background 0.1s;
    }

    .mail-row:hover {
        background: #1a2a1a;
    }

    .mail-row td {
        font-size: 0.9rem;
        padding: 0.65rem 0.6rem;
    }

    /* Unread mails are brighter and bolder */
    .mail-row.unread {
        color: #d4e8d0;
    }

    .mail-row.unread td {
        font-weight: 600;
    }

    .dot-cell {
        width: 1.2rem;
    }

    .unread-dot {
        background: #4a9e4a;
        border-radius: 50%;
        display: inline-block;
        height: 8px;
        width: 8px;
    }

    .from-cell {
        max-width: 200px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }

    .date-cell {
        color: #4a6a4a;
        font-size: 0.8rem;
        white-space: nowrap;
    }

    .empty-state {
        color: #4a6a4a;
        font-style: italic;
        padding: 3rem 1rem;
        text-align: center;
    }
</style>
