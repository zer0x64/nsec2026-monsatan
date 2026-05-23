<script lang="ts">
    import { goto } from "$app/navigation";
    import { login, register } from "$lib/api";
    import { auth } from "$lib/auth.svelte";

    // Which tab is active: 'login' or 'register'
    let activeTab = $state<"login" | "register">("login");

    // Shared fields
    let username = $state("");
    let password = $state("");

    // Register-only fields
    let bio = $state("");
    let email = $state("");
    let phone = $state("");
    let location = $state("");
    /** Base64-encoded profile picture (data URL prefix is stripped before sending). */
    let profilePicture = $state<string | undefined>(undefined);

    let errorMessage = $state("");
    let loading = $state(false);

    /** Reset error and form state when switching tabs */
    function switchTab(tab: "login" | "register") {
        activeTab = tab;
        errorMessage = "";
    }

    /**
     * Read the selected image file and store it as a raw base64 string,
     * stripping the "data:<mime>;base64," prefix added by FileReader.
     */
    function handleFileInput(e: Event) {
        const file = (e.target as HTMLInputElement).files?.[0];
        if (!file) {
            profilePicture = undefined;
            return;
        }
        const reader = new FileReader();
        reader.onload = () => {
            const dataUrl = reader.result as string;
            // Keep only the base64 payload after the comma.
            profilePicture = dataUrl.split(",")[1];
        };
        reader.readAsDataURL(file);
    }

    async function handleLogin() {
        errorMessage = "";
        loading = true;
        try {
            const { token } = await login(username, password);
            auth.setToken(token);
            goto("/swipe");
        } catch (err) {
            errorMessage = err instanceof Error ? err.message : "Login failed.";
        } finally {
            loading = false;
        }
    }

    async function handleRegister() {
        errorMessage = "";
        loading = true;
        try {
            // Only pass contact block if at least one field is filled
            const hasContact = email || phone || location;
            const contact = hasContact
                ? {
                      email: email || undefined,
                      phone: phone || undefined,
                      location: location || undefined,
                  }
                : undefined;

            const { token } = await register(
                username,
                password,
                bio || undefined,
                contact,
                profilePicture,
            );
            auth.setToken(token);
            goto("/swipe");
        } catch (err) {
            errorMessage =
                err instanceof Error ? err.message : "Registration failed.";
        } finally {
            loading = false;
        }
    }
</script>

<div class="auth-page">
    <!-- Logo / title -->
    <div class="brand">
        <span class="brand-icon">♚</span>
        <h1 class="brand-name">Checkmate</h1>
        <p class="brand-tagline">Find your perfect partner. Make your move.</p>
    </div>

    <div class="card">
        <!-- Tab switcher -->
        <div class="tabs" role="tablist">
            <button
                role="tab"
                class:active={activeTab === "login"}
                onclick={() => switchTab("login")}
            >
                Sign In
            </button>
            <button
                role="tab"
                class:active={activeTab === "register"}
                onclick={() => switchTab("register")}
            >
                Register
            </button>
        </div>

        <!-- Login form -->
        {#if activeTab === "login"}
            <form
                onsubmit={(e) => {
                    e.preventDefault();
                    handleLogin();
                }}
            >
                <label for="login-username">Username</label>
                <input
                    id="login-username"
                    type="text"
                    bind:value={username}
                    placeholder="grandmaster42"
                    autocomplete="username"
                    required
                />

                <label for="login-password">Password</label>
                <input
                    id="login-password"
                    type="password"
                    bind:value={password}
                    placeholder="••••••••"
                    autocomplete="current-password"
                    required
                />

                {#if errorMessage}
                    <p class="error">{errorMessage}</p>
                {/if}

                <button type="submit" class="submit-btn" disabled={loading}>
                    {loading ? "Signing in…" : "♟ Sign In"}
                </button>
            </form>

            <!-- Register form -->
        {:else}
            <form
                onsubmit={(e) => {
                    e.preventDefault();
                    handleRegister();
                }}
            >
                <label for="reg-username"
                    >Username <span class="required">*</span></label
                >
                <input
                    id="reg-username"
                    type="text"
                    bind:value={username}
                    placeholder="grandmaster42"
                    autocomplete="username"
                    required
                />

                <label for="reg-password"
                    >Password <span class="required">*</span></label
                >
                <input
                    id="reg-password"
                    type="password"
                    bind:value={password}
                    placeholder="••••••••"
                    autocomplete="new-password"
                    required
                />

                <label for="reg-avatar"
                    >Profile Picture <span class="optional">(optional)</span
                    ></label
                >
                <input
                    id="reg-avatar"
                    type="file"
                    accept="image/*"
                    onchange={handleFileInput}
                />
                {#if profilePicture}
                    <img
                        class="avatar-preview"
                        src="data:image/*;base64,{profilePicture}"
                        alt="Profile picture preview"
                    />
                {/if}

                <label for="reg-bio"
                    >Bio <span class="optional">(optional)</span></label
                >
                <textarea
                    id="reg-bio"
                    bind:value={bio}
                    placeholder="e.g. Sicilian Defence enthusiast, coffee addict, seeking my endgame partner…"
                    rows="3"
                ></textarea>

                <fieldset>
                    <legend
                        >Contact Info <span class="optional"
                            >(optional — only shown to matches)</span
                        ></legend
                    >

                    <label for="reg-email">Email</label>
                    <input
                        id="reg-email"
                        type="email"
                        bind:value={email}
                        placeholder="you@example.com"
                        autocomplete="email"
                    />

                    <label for="reg-phone">Phone</label>
                    <input
                        id="reg-phone"
                        type="tel"
                        bind:value={phone}
                        placeholder="+1 555 000 0000"
                        autocomplete="tel"
                    />

                    <label for="reg-location">Location</label>
                    <input
                        id="reg-location"
                        type="text"
                        bind:value={location}
                        placeholder="Montréal, QC"
                    />
                </fieldset>

                {#if errorMessage}
                    <p class="error">{errorMessage}</p>
                {/if}

                <button type="submit" class="submit-btn" disabled={loading}>
                    {loading ? "Creating account…" : "♙ Create Account"}
                </button>
            </form>
        {/if}
    </div>
</div>

<style>
    /* ── Page layout ──────────────────────────────────────────── */
    .auth-page {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.5rem;
        width: 100%;
        max-width: 420px;
    }

    /* ── Brand header ─────────────────────────────────────────── */
    .brand {
        text-align: center;
        color: #f0d9b5;
    }

    .brand-icon {
        font-size: 3.5rem;
        display: block;
        filter: drop-shadow(0 2px 6px rgba(0, 0, 0, 0.6));
    }

    .brand-name {
        font-size: 2rem;
        letter-spacing: 0.08em;
        margin: 0.25rem 0 0.1rem;
    }

    .brand-tagline {
        font-size: 0.9rem;
        color: #b58863;
        font-style: italic;
    }

    /* ── Card ─────────────────────────────────────────────────── */
    .card {
        background: #f0d9b5;
        border-radius: 8px;
        box-shadow: 0 6px 24px rgba(0, 0, 0, 0.5);
        width: 100%;
        overflow: hidden;
    }

    /* ── Tabs ─────────────────────────────────────────────────── */
    .tabs {
        display: flex;
        border-bottom: 2px solid #b58863;
    }

    .tabs button {
        flex: 1;
        padding: 0.75rem;
        border: none;
        background: #d4b896;
        color: #5a3e28;
        font-size: 0.95rem;
        font-weight: bold;
        font-family: inherit;
        transition:
            background 0.15s,
            color 0.15s;
    }

    .tabs button:hover {
        background: #c8a87e;
    }

    .tabs button.active {
        background: #f0d9b5;
        color: #1a1a1a;
        border-bottom: 2px solid #f0d9b5;
        margin-bottom: -2px;
    }

    /* ── Forms ────────────────────────────────────────────────── */
    form {
        display: flex;
        flex-direction: column;
        gap: 0.6rem;
        padding: 1.5rem;
    }

    label {
        font-size: 0.85rem;
        font-weight: bold;
        color: #3a2a18;
        margin-bottom: -0.2rem;
    }

    /* ── Avatar preview ───────────────────────────────────────── */
    .avatar-preview {
        width: 96px;
        height: 96px;
        border-radius: 50%;
        object-fit: cover;
        border: 2px solid #b58863;
        align-self: center;
    }

    input[type="file"] {
        padding: 0.3rem 0;
        border: none;
        background: transparent;
        font-size: 0.85rem;
        color: #3a2a18;
    }

    input,
    textarea {
        padding: 0.55rem 0.75rem;
        border: 1px solid #c4a882;
        border-radius: 4px;
        background: #fffef8;
        color: #1a1a1a;
        font-size: 0.95rem;
        transition:
            border-color 0.15s,
            box-shadow 0.15s;
        width: 100%;
    }

    input:focus,
    textarea:focus {
        outline: none;
        border-color: #8b6340;
        box-shadow: 0 0 0 2px rgba(139, 99, 64, 0.25);
    }

    textarea {
        resize: vertical;
    }

    /* ── Contact fieldset ─────────────────────────────────────── */
    fieldset {
        border: 1px solid #c4a882;
        border-radius: 6px;
        padding: 0.75rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    legend {
        font-size: 0.8rem;
        font-weight: bold;
        color: #5a3e28;
        padding: 0 0.25rem;
    }

    /* ── Misc text ────────────────────────────────────────────── */
    .required {
        color: #b04040;
    }

    .optional {
        color: #7a6050;
        font-weight: normal;
        font-size: 0.78rem;
    }

    .error {
        background: #f5d0d0;
        border: 1px solid #c08080;
        border-radius: 4px;
        color: #7a2020;
        font-size: 0.875rem;
        padding: 0.5rem 0.75rem;
    }

    /* ── Submit button ────────────────────────────────────────── */
    .submit-btn {
        margin-top: 0.4rem;
        padding: 0.7rem;
        border: none;
        border-radius: 4px;
        background: #5a8a40;
        color: #fff;
        font-size: 1rem;
        font-weight: bold;
        letter-spacing: 0.03em;
        transition:
            background 0.15s,
            transform 0.1s;
    }

    .submit-btn:hover:not(:disabled) {
        background: #4a7a30;
        transform: translateY(-1px);
    }

    .submit-btn:disabled {
        background: #8aaa78;
        cursor: not-allowed;
    }
</style>
