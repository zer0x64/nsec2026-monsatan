<!--
  register/+page.svelte — New account registration.

  Displays the @monsatan.ctf domain suffix inline next to the username field
  so the user can see their full email address before submitting.
-->
<script lang="ts">
    import { goto } from "$app/navigation";
    import { apiRegister } from "$lib/api";
    import { saveAuth } from "$lib/auth.svelte";
    import { EMAIL_DOMAIN } from "$lib/const";
    import AuthCard from "$lib/AuthCard.svelte";

    // Fine print shown below the form
    const FINE_PRINT =
        `By registering you agree to let Monsatan™ process your data for ` +
        `"ecosystem optimization purposes". It's fine, we're the good guys.`;

    let username = $state("");
    let password = $state("");
    let error = $state<string | null>(null);
    let loading = $state(false);

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        error = null;
        loading = true;
        try {
            const res = await apiRegister({ username, password });
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
    <title>Register — Monsatan Mail™</title>
</svelte:head>

<AuthCard activeTab="register" finePrint={FINE_PRINT}>
    <form onsubmit={handleSubmit}>
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
                autocomplete="new-password"
                required
            />
        </label>

        {#if error}
            <p class="error-box">{error}</p>
        {/if}

        <button type="submit" class="btn-primary" disabled={loading}>
            {loading ? "Registering…" : "Create Free™ Account"}
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
