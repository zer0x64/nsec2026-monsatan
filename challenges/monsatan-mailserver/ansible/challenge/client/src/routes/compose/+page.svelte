<!--
  compose/+page.svelte — New message editor.

  Accepts optional ?to= and ?subject= query params so the mail detail
  page can pre-fill a reply without needing a shared store.

  Plugins are .tar.gz files read by the browser as base64 and sent
  in the API's `plugins` array field.
-->
<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import { auth } from "$lib/auth.svelte";
    import { apiSendMail } from "$lib/api";
    import { EMAIL_DOMAIN } from "$lib/const";

    // ── Form state ─────────────────────────────────────────────────────────────
    let to = $state("");
    let subject = $state("");
    let body = $state("");

    /** Loaded plugins: file name for display, base64 data for the API. */
    let plugins = $state<{ name: string; data: string }[]>([]);

    let error = $state<string | null>(null);
    let sending = $state(false);

    /** Hidden file input element — triggered programmatically by the button. */
    let fileInput = $state<HTMLInputElement | null>(null);

    // ── Lifecycle ──────────────────────────────────────────────────────────────

    onMount(() => {
        if (!auth.token) {
            goto("/login");
            return;
        }

        // Pre-fill from query params (used by the Reply button in mail detail)
        to = $page.url.searchParams.get("to") ?? "";
        subject = $page.url.searchParams.get("subject") ?? "";
    });

    // ── Send ───────────────────────────────────────────────────────────────────

    async function handleSend(e: SubmitEvent) {
        e.preventDefault();
        error = null;
        sending = true;
        try {
            await apiSendMail(auth.token!, {
                to,
                subject,
                body,
                plugins: plugins.map((p) => p.data),
            });
            goto("/inbox");
        } catch (err: any) {
            error = err.message;
        } finally {
            sending = false;
        }
    }

    // ── Plugin loader ──────────────────────────────────────────────────────────

    function handleFileChange(e: Event) {
        const input = e.target as HTMLInputElement;
        if (!input.files) return;

        for (const file of input.files) {
            const reader = new FileReader();
            reader.onload = () => {
                // FileReader returns a data URL (data:<mime>;base64,<data>); keep only the base64 part
                const base64 = (reader.result as string).split(",")[1];
                plugins = [...plugins, { name: file.name, data: base64 }];
            };
            reader.readAsDataURL(file);
        }

        // Reset so the same file can be re-added if needed
        input.value = "";
    }

    function removePlugin(index: number) {
        plugins = plugins.filter((_, i) => i !== index);
    }
</script>

<svelte:head>
    <title>Compose — Monsatan Mail™</title>
</svelte:head>

<div class="pane">
    <div class="pane-header">
        <h2>New Message</h2>
        <a class="btn-ghost" href="/inbox">✕ Discard</a>
    </div>

    <form onsubmit={handleSend}>
        <label>
            To
            <input
                type="text"
                bind:value={to}
                placeholder="recipient or recipient@{EMAIL_DOMAIN}"
                required
            />
        </label>

        <label>
            Subject
            <input
                type="text"
                bind:value={subject}
                placeholder="Re: Global Crop Yields"
                required
            />
        </label>

        <label>
            Message
            <textarea
                bind:value={body}
                rows="14"
                placeholder="Write your message here…"
                required
            ></textarea>
        </label>

        <!-- ── Plugin section ──────────────────────────────────────────────── -->
        <div class="plugin-section">
            <div class="plugin-header">
                <span class="plugin-title">⚙ Plugins</span>
                <div class="plugin-actions">
                    <a class="btn-ghost" href="/plugins" target="_blank"
                        >📖 Plugin Docs</a
                    >
                    <button
                        type="button"
                        class="btn-ghost"
                        onclick={() => fileInput?.click()}
                    >
                        + Load .tar.gz plugin
                    </button>
                </div>
            </div>

            <!-- Hidden input; triggered by the button above -->
            <input
                type="file"
                accept=".tar.gz,.tgz,application/gzip"
                multiple
                style="display:none"
                bind:this={fileInput}
                onchange={handleFileChange}
            />

            {#if plugins.length > 0}
                <ul class="plugin-list">
                    {#each plugins as plugin, i}
                        <li class="plugin-item">
                            <span class="plugin-name">📦 {plugin.name}</span>
                            <button
                                type="button"
                                class="btn-remove"
                                onclick={() => removePlugin(i)}
                                title="Remove plugin">✕</button
                            >
                        </li>
                    {/each}
                </ul>
            {:else}
                <p class="plugin-hint">
                    No plugins loaded. Emails sent plugin-free are 12% more
                    carbon-neutral.*
                </p>
            {/if}
        </div>

        {#if error}
            <p class="error-box">{error}</p>
        {/if}

        <button type="submit" class="btn-primary" disabled={sending}>
            {sending ? "Sending…" : "Send Message"}
        </button>
    </form>
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

    form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    /* ── Plugin section ──────────────────────────────────────────────────────── */
    .plugin-section {
        background: #0d130d;
        border: 1px solid #2a3f2a;
        border-radius: 4px;
        display: flex;
        flex-direction: column;
        gap: 0.6rem;
        padding: 0.75rem 1rem;
    }

    .plugin-header {
        align-items: center;
        display: flex;
        justify-content: space-between;
    }

    .plugin-actions {
        display: flex;
        gap: 0.5rem;
    }

    .plugin-title {
        color: #6ecf6e;
        font-size: 0.85rem;
        font-weight: 600;
    }

    .plugin-list {
        display: flex;
        flex-direction: column;
        gap: 0.4rem;
        list-style: none;
    }

    .plugin-item {
        align-items: center;
        background: #141f14;
        border: 1px solid #2a3f2a;
        border-radius: 4px;
        display: flex;
        justify-content: space-between;
        padding: 0.35rem 0.6rem;
    }

    .plugin-name {
        color: #8fad8f;
        font-family: monospace;
        font-size: 0.85rem;
    }

    .btn-remove {
        background: none;
        border: none;
        color: #8f4a4a;
        cursor: pointer;
        font-size: 0.8rem;
        padding: 0.1rem 0.3rem;
    }

    .btn-remove:hover {
        color: #cf6e6e;
    }

    .plugin-hint {
        color: #4a6a4a;
        font-size: 0.8rem;
        font-style: italic;
    }
</style>
