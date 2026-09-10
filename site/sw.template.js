// The app shell as one versioned closure. manifest.json, written by the page projector, lists
// every shell file with its SHA-256; the cache is named by the closure digest, so a new release
// is a new cache and a stale worker never serves a mixed shell. Model blocks are not here: the
// engine keeps them in the device store. Same origin requests are served cache first; anything
// else goes to the network untouched.
const CLOSURE = "__CLOSURE__";   // written by the projector; a new shell is a new worker

async function readManifest() {
  const r = await fetch("manifest.json", { cache: "no-store" });
  return r.json();
}

self.addEventListener("install", (event) => {
  event.waitUntil((async () => {
    const m = await readManifest();
    if (m.closure !== CLOSURE) throw new Error("manifest closure does not match this worker");
    const cache = await caches.open("shell-" + CLOSURE);
    // Fetch every shell file fresh at install: the HTTP cache must not hand an older byte into a
    // closure whose digest says otherwise.
    await cache.addAll(m.files.map((f) => f.path).concat(["manifest.json"]).map((p) => new Request(p, { cache: "reload" })));
    await self.skipWaiting();
  })());
});

self.addEventListener("activate", (event) => {
  event.waitUntil((async () => {
    const keep = "shell-" + CLOSURE;
    for (const name of await caches.keys()) if (name !== keep) await caches.delete(name);
    await self.clients.claim();
  })());
});

self.addEventListener("fetch", (event) => {
  const url = new URL(event.request.url);
  if (url.origin !== self.location.origin || event.request.method !== "GET") return;
  event.respondWith((async () => {
    const hit = await caches.match(event.request, { ignoreSearch: true });
    if (hit) return hit;
    try {
      return await fetch(event.request);
    } catch (error) {
      if (event.request.mode === "navigate") return (await caches.match("index.html")) || (await caches.match("./"));
      throw error;
    }
  })());
});
