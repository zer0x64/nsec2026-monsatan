import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),

  kit: {
    adapter: adapter({
      pages: "../dist",
      assets: "../dist",
      // Serve this fallback for any route not found as a static file,
      // enabling client-side routing for dynamic paths like /invoice/[id].
      fallback: "index.html",
    }),
    paths: {
      // Use absolute URLs for all assets so they resolve correctly
      // regardless of the current route depth (e.g. /invoice/1).
      relative: false,
    },
  },
};

export default config;
