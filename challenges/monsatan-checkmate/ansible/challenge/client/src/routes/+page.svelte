<script lang="ts">
    import { goto } from "$app/navigation";
    import { auth } from "$lib/auth.svelte";

    // Immediately redirect based on current auth state.
    // Since ssr = false, this runs only in the browser where auth is initialised.
    if (auth.isAuthenticated) {
        goto("/swipe", { replaceState: true });
    } else {
        goto("/auth", { replaceState: true });
    }
</script>
