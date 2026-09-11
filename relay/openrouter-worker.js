// openrouter-worker.js — the operator funded warm up rung, if the operator chooses to run one.
//
// A page cannot carry an OpenRouter key: anything in a static page is public within a minute, and
// a shared key's free tier dies on the first afternoon of traffic. The only way to answer a visitor
// who holds no key is a process the operator runs that holds the key and forwards a bounded number
// of turns. This is that process, as small as it can be: a Cloudflare Worker (free tier, 100k
// requests a day), the key in a Worker secret, a per visitor daily budget, one cheap model, short
// answers, CORS to the page's origin only. It is a server. The page's chip says so.
//
//   wrangler secret put OPENROUTER_API_KEY
//   wrangler deploy
//   then set WARMUP_URL in the page to this worker's /v1/chat/completions
//
// Nothing here decides anything about the product: the page's route and warmup rules do, and the
// page treats this endpoint exactly as it treats openrouter.ai with the visitor's own key.
const MODEL = "qwen/qwen3.8-flash";      // measured 2026-09-10: about 1 s to first token, $0.00006 per short answer
const MAX_TOKENS = 256;
const TURNS_PER_DAY = 20;                 // per visitor (a salted hash of the address), the operator's spend cap
const ORIGINS = new Set(["https://humuhumu33.github.io", "http://localhost:8090"]);

export default {
  async fetch(request, env) {
    const origin = request.headers.get("Origin") || "";
    const cors = { "Access-Control-Allow-Origin": ORIGINS.has(origin) ? origin : "null", "Access-Control-Allow-Headers": "content-type", "Access-Control-Allow-Methods": "POST, OPTIONS" };
    if (request.method === "OPTIONS") return new Response(null, { status: 204, headers: cors });
    if (request.method !== "POST" || !ORIGINS.has(origin)) return new Response("", { status: 403, headers: cors });
    // the daily budget per visitor, in KV (bind a namespace named BUDGET)
    const day = new Date().toISOString().slice(0, 10);
    const who = await sha256((request.headers.get("CF-Connecting-IP") || "") + env.SALT + day);
    const used = Number((await env.BUDGET.get(who)) || 0);
    if (used >= TURNS_PER_DAY) return new Response(JSON.stringify({ error: { message: "today's free turns are used; your own model is loading", type: "rate_limit" } }), { status: 429, headers: { ...cors, "content-type": "application/json" } });
    await env.BUDGET.put(who, String(used + 1), { expirationTtl: 86400 });
    let body; try { body = await request.json(); } catch (e) { return new Response("", { status: 400, headers: cors }); }
    const upstream = await fetch("https://openrouter.ai/api/v1/chat/completions", {
      method: "POST",
      headers: { Authorization: "Bearer " + env.OPENROUTER_API_KEY, "content-type": "application/json", "HTTP-Referer": origin, "X-Title": "freeinference warm up" },
      body: JSON.stringify({ model: MODEL, messages: body.messages, max_tokens: Math.min(MAX_TOKENS, body.max_tokens || MAX_TOKENS), temperature: body.temperature ?? 0.7, stream: !!body.stream, usage: { include: true }, reasoning: { enabled: false }, provider: { require_parameters: true } }),
    });
    const headers = new Headers(cors); headers.set("content-type", upstream.headers.get("content-type") || "application/json"); headers.set("x-hologram-warmup", "1");
    return new Response(upstream.body, { status: upstream.status, headers });
  },
};

async function sha256(text) {
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
  return [...new Uint8Array(digest)].map((b) => b.toString(16).padStart(2, "0")).join("");
}
