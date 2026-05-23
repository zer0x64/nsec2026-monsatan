<!--
  +page.svelte — Root route.
  Redirects to /inbox if authenticated, otherwise to /login.
-->
<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { auth } from "$lib/auth.svelte";

    onMount(() => {
        goto(auth.token ? "/inbox" : "/login", { replaceState: true });
    });
</script>
