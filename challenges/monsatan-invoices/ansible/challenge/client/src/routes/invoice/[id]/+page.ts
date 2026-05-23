// This page must not be prerendered because invoice IDs are not known at build
// time. Disabling SSR keeps all data fetching on the client side.
export const ssr = false;
export const prerender = false;
