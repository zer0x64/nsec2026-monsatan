<!--
  mail/[id]/+page.svelte — Mail detail page.

  Fetches a single mail by its URL id parameter and displays the full content.
  The Reply link navigates to /compose with pre-filled ?to and ?subject params.
-->
<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { auth } from "$lib/auth.svelte";
    import { apiGetMail } from "$lib/api";
    import type { MailDetail } from "$lib/api";

    let mail = $state<MailDetail | null>(null);
    let error = $state<string | null>(null);
    let loading = $state(true);

    onMount(async () => {
        if (!auth.token) {
            goto("/login");
            return;
        }

        const id = Number($page.params.id);
        if (isNaN(id)) {
            error = "Invalid mail ID.";
            loading = false;
            return;
        }

        try {
            mail = await apiGetMail(auth.token, id);
        } catch (err: any) {
            error = err.message;
        } finally {
            loading = false;
        }
    });

    /** Build the reply URL, encoding both params to handle special characters. */
    function replyUrl(m: MailDetail): string {
        const params = new URLSearchParams({
            to: m.from,
            subject: `Re: ${m.subject}`,
        });
        return `/compose?${params.toString()}`;
    }

    function formatDate(iso: string): string {
        return new Date(iso).toLocaleString();
    }
</script>

<svelte:head>
    <title>{mail ? mail.subject : "Mail"} — Monsatan Mail™</title>
</svelte:head>

<div class="pane">
    <div class="pane-header">
        <a class="btn-ghost" href="/inbox">← Back</a>
        {#if mail}
            <a class="btn-ghost" href={replyUrl(mail)}>↩ Reply</a>
        {/if}
    </div>

    {#if loading}
        <p class="status">Loading…</p>
    {:else if error}
        <p class="error-box">{error}</p>
    {:else if mail}
        <div class="mail-detail">
            <h2 class="mail-subject">{mail.subject}</h2>

            <dl class="mail-meta">
                <dt>From</dt><dd>{mail.from}</dd>
                <dt>To</dt><dd>{mail.to}</dd>
                <dt>Date</dt><dd>{formatDate(mail.sent_at)}</dd>
            </dl>

            <hr class="divider" />

            <pre class="mail-body">{mail.body}</pre>
        </div>
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

    .status {
        color: #4a6a4a;
        font-style: italic;
        padding: 2rem 0;
        text-align: center;
    }

    /* ── Mail card ───────────────────────────────────────────────────────── */
    .mail-detail {
        background: #141f14;
        border: 1px solid #2a3f2a;
        border-radius: 6px;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        padding: 1.5rem;
    }

    .mail-subject {
        color: #d4e8d0;
        font-size: 1.2rem;
        font-weight: 600;
    }

    /* Definition list for From / To / Date metadata */
    .mail-meta {
        display: grid;
        grid-template-columns: max-content 1fr;
        column-gap: 1rem;
        row-gap: 0.2rem;
        font-size: 0.85rem;
    }

    .mail-meta dt {
        color: #8fad8f;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }

    .mail-meta dd {
        color: #6b8f6b;
    }

    .divider {
        border: none;
        border-top: 1px solid #2a3f2a;
        margin: 0.25rem 0;
    }

    .mail-body {
        color: #c4d8c0;
        font-family: monospace;
        font-size: 0.9rem;
        line-height: 1.6;
        white-space: pre-wrap;
        word-break: break-word;
    }
</style>
