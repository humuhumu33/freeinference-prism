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

// The endpoint on this origin: /v1/chat/completions and /v1/models are answered by the open page,
// which runs the model and encodes every byte through the verified core. The worker only carries
// the request to a window client and streams its frames back. Nothing is computed here.
const ENDPOINT = new Set(["v1/chat/completions", "v1/models"]);
async function serveFromPage(request, path) {
  const clients = await self.clients.matchAll({ type: "window", includeUncontrolled: true });
  const page = clients.find((c) => c.url.startsWith(self.registration.scope));
  if (!page) return new Response(JSON.stringify({ error: { message: "open the page and leave it open", type: "server_error" } }), { status: 503, headers: { "content-type": "application/json" } });
  const body = request.method === "POST" ? await request.text() : "";
  const channel = new MessageChannel();
  const headers = new Headers();
  let resolveHead; const head = new Promise((r) => (resolveHead = r));
  const stream = new ReadableStream({
    start(controller) {
      channel.port1.onmessage = ({ data }) => {
        if (data.head) { resolveHead(data.head); return; }
        if (data.frame != null) controller.enqueue(new TextEncoder().encode(data.frame));
        if (data.done) controller.close();
      };
    },
  });
  page.postMessage({ endpoint: path, method: request.method, body }, [channel.port2]);
  const h = await head;
  for (const [k, v] of Object.entries(h.headers || {})) headers.set(k, v);
  return new Response(stream, { status: h.status || 200, headers });
}

self.addEventListener("fetch", (event) => {
  const url = new URL(event.request.url);
  if (url.origin !== self.location.origin) return;
  const path = url.pathname.slice(new URL(self.registration.scope).pathname.length);
  if (ENDPOINT.has(path)) { event.respondWith(serveFromPage(event.request, path)); return; }
  if (event.request.method !== "GET") return;
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
