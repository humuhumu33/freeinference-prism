// The site moved to https://humuhumu33.github.io/dyad-prism/. This worker replaces the shell's worker
// on every returning visitor: it drops every cache the old shell kept, sends every open tab to the
// new address, and removes itself, so no cached page of the old site is served again.
const TARGET = "https://humuhumu33.github.io/dyad-prism/";
self.addEventListener("install", (event) => { event.waitUntil(self.skipWaiting()); });
self.addEventListener("activate", (event) => {
  event.waitUntil((async () => {
    for (const name of await caches.keys()) await caches.delete(name);
    await self.clients.claim();
    const clients = await self.clients.matchAll({ type: "window", includeUncontrolled: true });
    for (const client of clients) { try { await client.navigate(TARGET); } catch (e) {} }
    await self.registration.unregister();
  })());
});
self.addEventListener("fetch", (event) => {
  if (event.request.mode === "navigate") event.respondWith(Response.redirect(TARGET, 302));
});
