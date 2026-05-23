import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter({
      pages: "../dist",
      assets: "../dist",
      // Serve this file for any route the static server doesn't recognise,
      // letting SvelteKit's client-side router take over.
      fallback: "index.html",
    }),
  },
};

export default config;
