// Run everything client-side — no SSR, no build-time prerendering.
// This is required because invoice IDs are not known at build time.
export const ssr = false;
export const prerender = false;
