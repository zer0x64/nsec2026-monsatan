<!--
  login/+page.svelte — Login page.
  Authenticates with username + password and redirects to /inbox on success.
-->
<script lang="ts">
    import { goto } from "$app/navigation";
    import { apiLogin } from "$lib/api";
    import { auth, saveAuth } from "$lib/auth.svelte";
    import { onMount } from "svelte";
    import { EMAIL_DOMAIN } from "$lib/const";
    import AuthCard from "$lib/AuthCard.svelte";

    // Fine print shown below the form
    const FINE_PRINT =
        `By logging in you confirm that your biometric crop-yield data may be ` +
        `processed for "ecosystem synergy". Standard Monsatan™ terms apply.`;

    let username = $state("");
    let password = $state("");
    let error = $state<string | null>(null);
    let loading = $state(false);

    // Already logged in — skip straight to inbox
    onMount(() => {
        if (auth.token) goto("/inbox");
    });

    async function handleLogin(e: SubmitEvent) {
        e.preventDefault();
        error = null;
        loading = true;
        try {
            const res = await apiLogin({ username, password });
            saveAuth(res.token, res.email);
            goto("/inbox");
        } catch (err: any) {
            error = err.message;
        } finally {
            loading = false;
        }
    }
</script>

<svelte:head>
    <title>Login — Monsatan Mail™</title>
</svelte:head>

<AuthCard activeTab="login" finePrint={FINE_PRINT}>
    <form onsubmit={handleLogin}>
        <label>
            Username
            <!-- The domain suffix is shown inline so the user sees their full address -->
            <div class="email-field">
                <input
                    type="text"
                    bind:value={username}
                    autocomplete="username"
                    required
                />
                <span class="domain-suffix">@{EMAIL_DOMAIN}</span>
            </div>
        </label>
        <label>
            Password
            <input
                type="password"
                bind:value={password}
                autocomplete="current-password"
                required
            />
        </label>

        {#if error}
            <p class="error-box">{error}</p>
        {/if}

        <button type="submit" class="btn-primary" disabled={loading}>
            {loading ? "Logging in…" : "Login"}
        </button>
    </form>
</AuthCard>

<style>
    /* ── Form ────────────────────────────────────────────────────────────── */
    form {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    /* Username input with inline @domain suffix */
    .email-field {
        align-items: center;
        background: #0d130d;
        border: 1px solid #2a3f2a;
        border-radius: 4px;
        display: flex;
        overflow: hidden;
        transition: border-color 0.15s;
    }

    .email-field:focus-within {
        border-color: #4a9e4a;
    }

    /* Override the global input border/background since the wrapper handles it */
    .email-field :global(input) {
        background: transparent;
        border: none;
        flex: 1;
        min-width: 0;
    }

    .email-field :global(input:focus) {
        border-color: transparent;
    }

    .domain-suffix {
        color: #4a9e4a;
        font-size: 0.9rem;
        padding: 0 0.75rem;
        user-select: none;
        white-space: nowrap;
    }
</style>
