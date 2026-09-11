// The in-browser product. Adapters only: the rules are in core.wasm, generated from the Lean
// verified model; the engine is Hologram Q's snapshot under q/; addresses are BLAKE3 from the
// holospaces wasm; the store is IndexedDB on this device. No server is contacted except
// Hugging Face, once, for the model's blocks.

const $ = (id) => document.getElementById(id);
const short = (k) => (k ? String(k).replace(/^blake3:/, "").replace("did:holo:sha256:", "").slice(0, 12) + "…" : "");
const enc = new TextEncoder();

// ---- the verified core, through its JSON ABI
let core = null;
async function coreReady() {
  if (core) return core;
  const { instance } = await WebAssembly.instantiateStreaming(fetch("core.wasm"), {});
  const { memory, holo_alloc, holo_free, holo_run } = instance.exports;
  core = {
    run(request) {
      const bytes = enc.encode(JSON.stringify(request));
      const ptr = holo_alloc(bytes.length);
      new Uint8Array(memory.buffer, ptr, bytes.length).set(bytes);
      const packed = holo_run(ptr, bytes.length);
      const outPtr = Number(packed >> 32n), outLen = Number(packed & 0xffffffffn);
      const text = new TextDecoder().decode(new Uint8Array(memory.buffer, outPtr, outLen));
      holo_free(outPtr, outLen); holo_free(ptr, bytes.length);
      const out = JSON.parse(text);
      if (out.error) throw new Error(out.error);
      return out;
    },
  };
  return core;
}
let VIEW = null;

// ---- BLAKE3 addresses from the holospaces wasm (the same κ every daemon computes)
let kappaFn = null;
async function kappaReady() {
  if (kappaFn) return kappaFn;
  const mod = await import("./q/pkg/holospaces_web.js");
  if (typeof mod.default === "function") { try { await mod.default(); } catch (e) {} }
  kappaFn = (bytes) => String(mod.kappa(bytes));
  return kappaFn;
}

// ---- the device store: one object store keyed by κ, indexed by kind and by memo prompt κ
function db() {
  return new Promise((resolve, reject) => {
    const req = indexedDB.open("freeinference", 1);
    req.onupgradeneeded = () => {
      const s = req.result.createObjectStore("objects", { keyPath: "id" });
      s.createIndex("kind", "kind"); s.createIndex("promptKappa", "promptKappa");
    };
    req.onsuccess = () => resolve(req.result); req.onerror = () => reject(req.error);
  });
}
async function put(kind, value, extra = {}) {
  const kappa = await kappaReady();
  const bytes = enc.encode(typeof value === "string" ? value : JSON.stringify(value));
  const id = kappa(bytes);
  const d = await db();
  await new Promise((res, rej) => { const t = d.transaction("objects", "readwrite"); t.objectStore("objects").put({ id, kind, value, created: Date.now(), ...extra }); t.oncomplete = res; t.onerror = () => rej(t.error); });
  return id;
}
async function get(id) {
  const d = await db();
  return new Promise((res, rej) => { const r = d.transaction("objects").objectStore("objects").get(id); r.onsuccess = () => res(r.result); r.onerror = () => rej(r.error); });
}
async function byPrompt(promptKappa) {
  const d = await db();
  return new Promise((res, rej) => { const r = d.transaction("objects").objectStore("objects").index("promptKappa").getAll(promptKappa); r.onsuccess = () => res(r.result || []); r.onerror = () => rej(r.error); });
}
async function all() {
  const d = await db();
  return new Promise((res, rej) => { const r = d.transaction("objects").objectStore("objects").getAll(); r.onsuccess = () => res(r.result || []); r.onerror = () => rej(r.error); });
}

// ---- memo: the core owns the preimages and the decision, the wasm owns the address, the store owns the rest
async function keyOf(body) {
  const c = await coreReady(); const kappa = await kappaReady();
  const pre = c.run({ op: "preimages", request: body });
  return { promptKappa: kappa(enc.encode(pre.prompt)), paramsKappa: kappa(enc.encode(pre.params)) };
}
async function lookup(body) {
  const key = await keyOf(body);
  const memos = (await byPrompt(key.promptKappa)).filter((o) => o.kind === "memo" && o.value.paramsKappa === key.paramsKappa && o.value.model.includes(body.model)).sort((a, b) => b.created - a.created);
  for (const m of memos) {
    const receipt = await get(m.value.receipt); const answer = await get(m.value.outputKappa);
    if (!receipt || !answer) continue;
    if (receipt.kind === "or-receipt") {
      const kappa = await kappaReady();
      if (kappa(enc.encode(JSON.stringify(receipt.value))) !== receipt.id || kappa(enc.encode(answer.value)) !== answer.id) continue;
      return { hit: true, text: answer.value, receipt: m.value.receipt, rec: receipt.value, paid: true, fingerprint: `${receipt.value.model};${receipt.value.provider || ""};${receipt.value.fingerprint || ""}` };
    }
    const { verifyIntegrity } = await import("./q/core/kappa.js");
    if (!(await verifyIntegrity(receipt.value)).ok) continue;
    return { hit: true, text: answer.value, receipt: m.value.receipt, rec: receipt.value, fingerprint: `${receipt.value.body["prov:used"]["holo:model"]};${receipt.value.body["prov:used"]["holo:engine"]}` };
  }
  return { hit: false, key };
}
async function seal(body, rec, key) {
  const receipt = await put("q-receipt", { id: rec.id, body: rec.body, text: rec.text, promptText: rec.promptText, ctxIds: rec.ctxIds, turnIds: rec.turnIds, outIds: rec.outIds, params: rec.params });
  const outputKappa = await put("answer", rec.text);
  const used = rec.body["prov:used"] || {};
  const memo = { iri: "https://freeinference.ai/memo/v1", model: [used["holo:model"], body.model], engineKappa: used["holo:engine"] || "", promptKappa: key.promptKappa, paramsKappa: key.paramsKappa, outputKappa, receipt };
  await put("memo", memo, { promptKappa: key.promptKappa });
  return receipt;
}

// ---- the engine: Hologram Q on WebGPU, resident across turns, warm KV as q-brain-fast does it
let gpuEngine = null, gpuModel = null, gpuMods = null, gpuLoading = null, gpuSession = null;
const sigOf = (list) => (list || []).map((m) => (m.role || "") + (m.content || "")).join("");
const tailFor = (M, u) => M.llama3 ? `<|eot_id|><|start_header_id|>user<|end_header_id|>\n\n${u || ""}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n` : null;
function gpuReady() {
  if (gpuEngine) return Promise.resolve(gpuEngine);
  if (gpuLoading) return gpuLoading;
  gpuLoading = (async () => {
    const [L, E, F] = await Promise.all([import("./q/core/loader.js"), import("./q/core/engine.js"), import("./q/core/q-brain-fast.mjs")]);
    gpuMods = { L, E, F }; gpuModel = L.MODELS.find((m) => m.fam === "BitNet");
    setState(VIEW.loadingLabel); await L.ready();
    const t0 = performance.now();
    const loaded = await L.loadModel(gpuModel, {
      // The engine's own status strings are for engineers; the page says one plain thing and a number.
      onStatus: () => setState(VIEW.loadingLabel),
      onProgress: (d, t) => { gpuPct = t ? Math.round((d / t) * 100) : null; setState(t ? `${VIEW.loadingLabel} · ${gpuPct}%` : VIEW.loadingLabel); if (gpuPct % 5 === 0) refreshWho(); },
    });
    if (!loaded || !loaded.gpu) throw new Error("model load failed");
    gpuEngine = await E.createEngine(gpuModel, loaded);
    setState(""); refreshWho();
    refreshConnect();
    return gpuEngine;
  })().catch((err) => { gpuLoading = null; throw err; });
  return gpuLoading;
}
function gpuIds(engine, messages) {
  const last = messages[messages.length - 1];
  if (gpuSession && messages.length >= 2 && last && last.role === "user") {
    const tail = tailFor(gpuModel, last.content);
    if (tail != null && sigOf(messages.slice(0, -1)) === gpuSession.sig) return { ids: gpuSession.ids.concat(engine.tokenize(tail)), warm: true };
  }
  let ids = engine.tokenize(gpuMods.F.frameHistory(gpuModel, messages));
  if (gpuModel.bos && engine.bosId != null) ids = [engine.bosId, ...ids];
  return { ids, warm: false };
}

// ---- who answers: local or paid. The route is the model's (`route` op); the key lives in the device
// store under this origin and leaves it only in the Authorization header to openrouter.ai.
const WHO = "holo.inference.v1";
function readWho() { try { return JSON.parse(localStorage.getItem(WHO) || "null") || { provider: "local", model: "" }; } catch (e) { return { provider: "local", model: "" }; } }
function writeWho(w) { try { localStorage.setItem(WHO, JSON.stringify(w)); } catch (e) {} }
// The site may include a key of its own (site/warmup.json, written at deploy time from a secret, never in
// the repository): it counts as a key for the route and warm up rules, the visitor's own key wins over it.
let siteKey = "";
async function siteKeyReady() { try { const r = await fetch("warmup.json", { cache: "no-store" }); if (r.ok) { const j = await r.json(); siteKey = String(j.key || ""); } } catch (e) {} return siteKey; }
async function deviceKeyGet() { const o = await get("openrouter-key"); return o && o.value ? String(o.value) : ""; }
async function keyGet() { return (await deviceKeyGet()) || siteKey; }
async function keySet(value) {
  const d = await db();
  await new Promise((res, rej) => { const t = d.transaction("objects", "readwrite"); t.objectStore("objects").put({ id: "openrouter-key", kind: "secret", value, created: Date.now() }); t.oncomplete = res; t.onerror = () => rej(t.error); });
}
const OPENROUTER = "https://openrouter.ai/api/v1/chat/completions";
const paidId = (model) => model.replace(/^openrouter\//, "");
// Runs one request on OpenRouter. The request bytes are the model's encoder; the response is decoded
// here and sealed like a local answer. `onDelta(text)` streams; returns { text, rec }.
async function paidGenerate(body, onDelta) {
  const c = await coreReady(); const key = await keyGet();
  const bytes = c.run({ op: "encode-openrouter-request", model: paidId(body.model), request: body, stream: true }).bytes;
  const r = await fetch(OPENROUTER, { method: "POST", headers: { Authorization: "Bearer " + key, "content-type": "application/json", "HTTP-Referer": location.origin, "X-Title": "freeinference" }, body: bytes });
  if (!r.ok) {
    let msg = ""; try { msg = (await r.json()).error.message; } catch (e) {}
    const word = r.status === 401 ? VIEW.noKeyLabel : r.status === 402 ? VIEW.noCreditLabel : VIEW.providerBusyLabel;
    throw Object.assign(new Error(word), { status: r.status, detail: msg });
  }
  const reader = r.body.getReader(); const dec = new TextDecoder(); let buf = "", text = "", last = null, first = null;
  const t0 = performance.now();
  for (;;) {
    const { value, done } = await reader.read(); if (done) break; buf += dec.decode(value, { stream: true });
    let i; while ((i = buf.indexOf("\n")) >= 0) {
      const line = buf.slice(0, i).trim(); buf = buf.slice(i + 1);
      if (!line.startsWith("data: ") || line === "data: [DONE]") continue;
      let j; try { j = JSON.parse(line.slice(6)); } catch (e) { continue; }
      if (j.error) throw new Error(VIEW.providerBusyLabel);
      last = j; const d = j.choices && j.choices[0] && j.choices[0].delta && j.choices[0].delta.content;
      if (d) { if (first === null) first = performance.now() - t0; text += d; if (onDelta) onDelta(text); }
    }
  }
  if (!text.trim()) throw new Error(VIEW.providerBusyLabel);
  const usage = (last && last.usage) || {};
  const rec = { kind: "openrouter", id: last && last.id, model: (last && last.model) || paidId(body.model), provider: last && last.provider, fingerprint: (last && last.system_fingerprint) || "", usage: { prompt: usage.prompt_tokens || 0, completion: usage.completion_tokens || 0 }, cost: String(usage.cost ?? ""), text: text.trim(), rederivable: false, ttftMs: Math.round(first || 0) };
  return { text: rec.text, rec };
}
// Seals a paid answer with the same memo shape as a local one; the receipt says it cannot be re derived.
async function sealPaid(body, rec, key) {
  const receipt = await put("or-receipt", rec);
  const outputKappa = await put("answer", rec.text);
  const memo = { iri: "https://freeinference.ai/memo/v1", model: [rec.model, body.model], engineKappa: "openrouter:" + (rec.provider || ""), promptKappa: key.promptKappa, paramsKappa: key.paramsKappa, outputKappa, receipt };
  await put("memo", memo, { promptKappa: key.promptKappa });
  return receipt;
}
const costText = (cost) => { const n = Number(cost); return !n ? VIEW.freeLabel : `${VIEW.costLabel} · $${n < 0.001 ? n.toFixed(5) : n.toFixed(3)}`; };

// One engine, one generation at a time: the page and the endpoint share it.
let gpuBusy = Promise.resolve();
function withEngine(fn) { const run = gpuBusy.then(fn, fn); gpuBusy = run.catch(() => {}); return run; }

// ---- the page
const history = [];
// Progress and states show in the hint inside the box; ready shows nothing.
function setState(text) { $("hint").textContent = text || ""; }
const grow = (t) => { t.style.height = "auto"; t.style.height = Math.min(t.scrollHeight, 220) + "px"; };
$("input").addEventListener("input", (e) => grow(e.target));
$("input").addEventListener("keydown", (e) => { if (e.key === "Enter" && !e.shiftKey) { e.preventDefault(); $("composer").requestSubmit(); } });
function add(role, text) {
  const el = document.createElement("div"); el.className = `msg ${role}`; el.textContent = text;
  $("messages").appendChild(el); el.scrollIntoView({ block: "nearest" }); return el;
}
function chips(el, items) {
  const meta = document.createElement("div"); meta.className = "meta";
  for (const it of items) {
    const node = document.createElement(it.button ? "button" : "span");
    node.className = "badge" + (it.ok ? " ok" : ""); node.textContent = it.text; if (it.title) node.title = it.title;
    if (it.button) { node.type = "button"; node.onclick = () => it.button(node); }
    meta.appendChild(node);
  }
  el.appendChild(meta);
}
function rederiveChip(rec) {
  return { text: VIEW.rederiveLabel, button: async (node) => {
    node.textContent = "…";
    try { const v = await (await gpuReady()).reDerive(rec); node.textContent = v.ok ? VIEW.identicalLabel : `not identical: ${v.reason || "output κ differs"}`; node.className = v.ok ? "badge ok" : "badge"; }
    catch (err) { node.textContent = `re derive failed: ${err.message}`; }
  } };
}

const paidChip = (rec) => ({ text: costText(rec.cost), title: `${rec.model} · ${rec.provider || ""} · ${rec.id || ""}` });
async function routeOf(hit, body) {
  const c = await coreReady(); const who = readWho();
  const r = c.run({ op: "route", hit: !!hit.hit, provider: who.provider === "paid" ? "paid" : "local", gpuReady: !!navigator.gpu, keyPresent: !!(await keyGet()), online: navigator.onLine });
  return r.route;
}
const refusal = { NoKey: () => VIEW.noKeyLabel, NoGpu: () => VIEW.noGpuLabel, PaidOffline: () => VIEW.paidOfflineLabel };
async function turn(messages, body, el) {
  const t0 = performance.now();
  const hit = await lookup(body);
  const route = await routeOf(hit, body);
  if (route === "Serve") {
    el.textContent = hit.text;
    chips(el, hit.paid
      ? [{ text: VIEW.paidModels.find((m) => m.id === hit.rec.model)?.label || hit.rec.model, title: hit.fingerprint }, { text: VIEW.servedLabel, title: hit.receipt, ok: true }, { text: VIEW.freeLabel, title: VIEW.paidOnceLabel }]
      : [{ text: VIEW.modelLabel, title: hit.fingerprint }, { text: VIEW.servedLabel, title: hit.receipt, ok: true }, rederiveChip(hit.rec)]);
    $("hint").textContent = `${Math.round(performance.now() - t0)} ms · ${VIEW.servedLabel}`;
    return hit.text;
  }
  if (refusal[route]) throw new Error(refusal[route]());
  // Warm up: the local model is chosen but not resident yet; a held key answers through OpenRouter meanwhile.
  if (route === "Local" && !gpuEngine && (await coreReady()).run({ op: "warmup", localReady: !!gpuEngine, keyPresent: !!(await keyGet()), online: navigator.onLine }).warmup) {
    const warmBody = { ...body, model: "openrouter/" + VIEW.paidModels[0].id };
    const warmHit = await lookup(warmBody);
    if (warmHit.hit) { el.textContent = warmHit.text; chips(el, [{ text: VIEW.warmupLabel, title: warmHit.fingerprint }, { text: VIEW.servedLabel, title: warmHit.receipt, ok: true }]); return warmHit.text; }
    const { text, rec } = await paidGenerate(warmBody, (t) => { el.textContent = t; });
    el.textContent = text;
    const receipt = await sealPaid(warmBody, rec, warmHit.key);
    chips(el, [{ text: VIEW.warmupLabel, title: `${rec.model} · ${rec.provider || ""}` }, { text: VIEW.sealedLabel, title: receipt, ok: true }, paidChip(rec)]);
    $("hint").textContent = `${Math.round(performance.now() - t0)} ms · first token ${rec.ttftMs} ms`;
    return text;
  }
  if (route === "Paid") {
    const { text, rec } = await paidGenerate(body, (t) => { el.textContent = t; });
    el.textContent = text;
    const receipt = await sealPaid(body, rec, hit.key);
    chips(el, [{ text: VIEW.paidModels.find((m) => m.id === rec.model)?.label || rec.model, title: `${rec.provider || ""} · ${rec.id || ""}` }, { text: VIEW.sealedLabel, title: receipt, ok: true }, paidChip(rec)]);
    $("hint").textContent = `${Math.round(performance.now() - t0)} ms · first token ${rec.ttftMs} ms`;
    return text;
  }
  if (!navigator.gpu) throw new Error(VIEW.noGpuLabel);
  const engine = await gpuReady();
  const { ids, warm } = gpuIds(engine, messages);
  const started = performance.now();
  const live = (s) => `${warm ? "warm" : "cold"} · first token ${Math.round(s.ttft || 0)} ms · ${(s.tokps || 0).toFixed(1)} tok/s`;
  const res = await withEngine(() => engine.generate(ids, { maxNew: body.max_tokens, onToken: ({ text, stats }) => { el.textContent = text; if (stats) $("hint").textContent = live(stats); } }));
  const text = (res.text || "").trim(); el.textContent = text;
  if (res.ids && text && !res.error) gpuSession = { ids: res.ids.slice(), sig: sigOf(messages.concat([{ role: "assistant", content: text }])) };
  const promptText = (messages.filter((m) => m.role === "user").slice(-1)[0] || {}).content || "";
  const rec = await engine.buildReceipt({ promptText, ctxIds: [], turnIds: ids, outIds: res.outIds });
  const receipt = await seal(body, rec, hit.key);
  const used = rec.body["prov:used"] || {};
  chips(el, [{ text: VIEW.modelLabel, title: used["holo:model"] }, { text: VIEW.sealedLabel, title: receipt, ok: true }, rederiveChip(rec)]);
  $("hint").textContent = `${Math.round(performance.now() - started)} ms · ${live(res.stats || {})}`;
  return text;
}

$("composer").onsubmit = async (e) => {
  e.preventDefault();
  const text = $("input").value.trim(); if (!text) return;
  $("input").value = ""; grow($("input")); $("send").disabled = true;
  add("user", text); history.push({ role: "user", content: text });
  const messages = history.slice();
  const who = readWho();
  const body = { model: who.provider === "paid" ? "openrouter/" + (who.model || VIEW.paidModels[0].id) : "webgpu:BitNet", messages, max_tokens: 512, temperature: "0.7" };
  const el = add("assistant", "");
  try { const content = await turn(messages, body, el); history.push({ role: "assistant", content }); }
  catch (err) { el.textContent = `Error: ${err.message}`; }
  finally { $("send").disabled = false; $("input").focus(); }
};

// ---- the endpoint: one function answers an OpenAI request, memo first, then the engine. Every byte
// it returns is encoded by the verified core (encode-role, encode-delta, encode-final,
// encode-completion, encode-error, encode-models, done). Two transports carry it and compute
// nothing: the service worker on this origin, and the one file relay for native harnesses.
const MODEL_ID = "webgpu:BitNet";
const wire = (op, extra) => core.run({ op, ...extra }).bytes;
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
function completionOf(id, text, fingerprint, receipt, model = MODEL_ID) {
  return { id, created: String(Math.floor(Date.now() / 1000)), model, text, fingerprint, receipt };
}
// The request as the model's Request: text only, max_tokens as OpenClaw and Hermes spell it,
// temperature spelled as a decimal string.
function requestOf(raw) {
  const messages = raw.messages.map((m) => ({ role: m.role || "user", content: Array.isArray(m.content) ? m.content.map((p) => (p && p.text) || "").join("\n") : String(m.content ?? "") }));
  const maxTokens = raw.max_completion_tokens ?? raw.max_tokens;
  const who = readWho();
  const model = typeof raw.model === "string" && raw.model.startsWith("openrouter/") ? raw.model : who.provider === "paid" ? "openrouter/" + (who.model || VIEW.paidModels[0].id) : MODEL_ID;
  return { model, messages, max_tokens: Number.isInteger(maxTokens) && maxTokens > 0 ? Math.min(maxTokens, 2048) : 512, temperature: raw.temperature == null ? "" : String(raw.temperature), seed: raw.seed };
}
// Answers one request. With a sink ({head, frame}) the answer streams as chunk frames; without,
// it returns the whole completion. Either way {status, headers[, bytes]} comes back.
async function serve(raw, sink) {
  await coreReady();
  if (!raw || !Array.isArray(raw.messages) || !raw.messages.length) {
    return { status: 400, bytes: wire("encode-error", { message: "messages must not be empty", type: "invalid_request_error" }), headers: {} };
  }
  const body = requestOf(raw);
  const hit = await lookup(body);
  const paidWanted = body.model.startsWith("openrouter/");
  const rt = (await coreReady()).run({ op: "route", hit: !!hit.hit, provider: paidWanted ? "paid" : "local", gpuReady: !!navigator.gpu, keyPresent: !!(await keyGet()), online: navigator.onLine }).route;
  if (refusal[rt]) return { status: rt === "NoKey" ? 401 : 503, bytes: wire("encode-error", { message: refusal[rt](), type: rt === "NoKey" ? "authentication_error" : "server_error" }), headers: {} };
  if (rt === "Serve") {
    const c = completionOf("chatcmpl-" + hit.receipt.replace("blake3:", "").slice(0, 12), hit.text, hit.fingerprint, hit.receipt, body.model);
    const headers = { "x-hologram-receipt": hit.receipt, "x-hologram-reuse": "1", "x-hologram-stream": sink ? "memo" : "plain", "x-hologram-provider": hit.paid ? "openrouter" : "local" };
    if (!sink) return { status: 200, bytes: wire("encode-completion", { completion: c }), headers };
    sink.head(headers);
    sink.frame(wire("encode-role", { completion: c })); sink.frame(wire("encode-delta", { completion: c, delta: hit.text }));
    sink.frame(wire("encode-final", { completion: c })); sink.frame(wire("done"));
    return { status: 200, headers };
  }
  if (rt === "Paid") {
    const c = completionOf("chatcmpl-" + Math.random().toString(16).slice(2, 14), "", "", "", body.model);
    let sent = "";
    if (sink) { sink.head({ "x-hologram-stream": "native", "x-hologram-provider": "openrouter" }); sink.frame(wire("encode-role", { completion: c })); }
    const { text, rec } = await paidGenerate(body, (t) => { if (sink && t.startsWith(sent) && t.length > sent.length) { sink.frame(wire("encode-delta", { completion: c, delta: t.slice(sent.length) })); sent = t; } });
    const receipt = await sealPaid(body, rec, hit.key);
    const done = { ...c, text, fingerprint: `${rec.model};${rec.provider || ""};${rec.fingerprint || ""}`, receipt, model: body.model };
    const headers = { "x-hologram-receipt": receipt, "x-hologram-stream": sink ? "native" : "plain", "x-hologram-provider": "openrouter", "x-hologram-cost": rec.cost };
    if (!sink) return { status: 200, bytes: wire("encode-completion", { completion: done }), headers };
    if (text.length > sent.length && text.startsWith(sent)) sink.frame(wire("encode-delta", { completion: done, delta: text.slice(sent.length) }));
    sink.frame(wire("encode-final", { completion: done })); sink.frame(wire("done"));
    return { status: 200, headers };
  }
  const engine = await gpuReady();
  const messages = body.messages;
  const c = completionOf("chatcmpl-" + Math.random().toString(16).slice(2, 14), "", "", "");
  if (sink) { sink.head({ "x-hologram-stream": "native" }); sink.frame(wire("encode-role", { completion: c })); }
  let sent = "";
  const { ids, warm } = gpuIds(engine, messages);
  const res = await withEngine(() => engine.generate(ids, { maxNew: body.max_tokens, onToken: ({ text }) => {
    if (sink && text.startsWith(sent) && text.length > sent.length) { sink.frame(wire("encode-delta", { completion: c, delta: text.slice(sent.length) })); sent = text; }
  } }));
  const text = (res.text || "").trim();
  if (res.ids && text && !res.error) gpuSession = { ids: res.ids.slice(), sig: sigOf(messages.concat([{ role: "assistant", content: text }])) };
  const promptText = (messages.filter((m) => m.role === "user").slice(-1)[0] || {}).content || "";
  const rec = await engine.buildReceipt({ promptText, ctxIds: [], turnIds: ids, outIds: res.outIds });
  const receipt = await seal(body, rec, hit.key);
  const used = rec.body["prov:used"] || {};
  const done = { ...c, text, fingerprint: `${used["holo:model"]};${used["holo:engine"]}`, receipt };
  const headers = { "x-hologram-receipt": receipt, "x-hologram-stream": sink ? "native" : "plain", "x-hologram-warm": warm ? "1" : "0" };
  if (!sink) return { status: 200, bytes: wire("encode-completion", { completion: done }), headers };
  if (text.length > sent.length && text.startsWith(sent)) sink.frame(wire("encode-delta", { completion: done, delta: text.slice(sent.length) }));
  sink.frame(wire("encode-final", { completion: done })); sink.frame(wire("done"));
  return { status: 200, headers };
}
const modelsBytes = () => wire("encode-models", { ids: [MODEL_ID].concat(VIEW.paidModels.map((m) => "openrouter/" + m.id)) });

// Transport 1: the service worker hands each /v1 request on this origin to this page over a
// MessageChannel: {head:{status,headers}} once, then {frame} per wire frame, then {done}.
navigator.serviceWorker.addEventListener("message", async (event) => {
  const { endpoint, body } = event.data || {}; const port = event.ports && event.ports[0];
  if (!port || !endpoint) return;
  const json = (status, bytes, headers = {}) => { port.postMessage({ head: { status, headers: { "content-type": "application/json", ...headers } } }); port.postMessage({ frame: bytes, done: true }); };
  try {
    await coreReady();
    if (endpoint === "v1/models") return json(200, modelsBytes());
    let raw; try { raw = JSON.parse(body || ""); } catch (e) { return json(400, wire("encode-error", { message: "request is not JSON", type: "invalid_request_error" })); }
    if (!raw.stream) { const out = await serve(raw, null); return json(out.status, out.bytes, out.headers); }
    let headed = false;
    const sink = {
      head: (headers) => { if (!headed) { headed = true; port.postMessage({ head: { status: 200, headers: { "content-type": "text/event-stream", "cache-control": "no-cache", ...headers } } }); } },
      frame: (bytes) => port.postMessage({ frame: "data: " + bytes + "\n\n" }),
    };
    const out = await serve(raw, sink);
    if (!headed) return json(out.status, out.bytes, out.headers);
    port.postMessage({ done: true });
  } catch (err) { try { json(500, wire("encode-error", { message: err.message, type: "server_error" })); } catch (e) {} }
});

// Transport 2: the relay. When freeinference-relay.py runs on this machine, this tab serves its
// jobs: long poll /tab/next, post frames and results back. The tab presents the token it read
// through /tab/hello and a custom header on every call, so a page on another origin cannot pose as
// the tab. The relay is probed only after the visitor pressed Connect once (a public page reaching
// 127.0.0.1 may make the browser ask), then every 2 s while the sheet is open, every 10 s closed.
const RELAY_PORT = Number(new URLSearchParams(location.search).get("relay")) || 11435;
const RELAY = `http://127.0.0.1:${RELAY_PORT}`;
// One id per browser tab, kept across reloads (sessionStorage is per tab), so a reload reattaches at
// once instead of waiting out the previous poll as a second tab.
const TAB_ID = (() => { try { const k = "holo.tab.v1"; let id = sessionStorage.getItem(k); if (!id) { id = (crypto.randomUUID ? crypto.randomUUID() : String(Math.random())).replace(/-/g, "").slice(0, 12); sessionStorage.setItem(k, id); } return id; } catch (e) { return String(Math.random()).slice(2, 14); } })();
const CONNECT = "holo.connect.v1";
const readConnect = () => { try { return JSON.parse(localStorage.getItem(CONNECT) || "null") || {}; } catch (e) { return {}; } };
const writeConnect = (c) => { try { localStorage.setItem(CONNECT, JSON.stringify(c)); } catch (e) {} };
let relayToken = "", relayRunning = false, relayTimer = null, relayState = "off", relaySeen = false, relayOther = "";
const tabHeaders = () => ({ "X-Freeinference-Tab": relayToken || "hello", "X-Freeinference-Tab-Id": TAB_ID });
function setRelayState(state, other) {
  if (state === "connected") relaySeen = true;
  if (state === relayState && (other || "") === relayOther) return;
  relayState = state; relayOther = other || ""; paintConnect();
}
async function relayLoop() {
  if (relayRunning) return; relayRunning = true;
  try {
    for (;;) {
      let job = null;
      try {
        const r = await fetch(RELAY + "/tab/next", { cache: "no-store", headers: tabHeaders() });
        if (r.status === 200) job = await r.json();
        else if (r.status === 409) { setRelayState("second", ((await r.json()).serving || "")); await sleep(5000); continue; }
        else if (r.status === 403) { relayToken = ""; scheduleWatch(2000); return; }
        else if (r.status !== 204) { await sleep(2000); continue; }
        setRelayState("connected");
      } catch (e) { setRelayState("listening"); scheduleWatch(); return; }
      if (!job) continue;
      const post = (action, payload) => fetch(`${RELAY}/tab/${job.id}/${action}`, { method: "POST", headers: { "content-type": "application/json", ...tabHeaders() }, body: JSON.stringify(payload) }).catch(() => {});
      try {
        await coreReady();
        if (job.op === "models") { await post("result", { bytes: modelsBytes(), headers: {} }); continue; }
        const raw = job.request || {};
        if (!raw.stream) { const out = await serve(raw, null); await post(out.status === 200 ? "result" : "fail", out.status === 200 ? { bytes: out.bytes, headers: out.headers } : { message: JSON.parse(out.bytes).error.message }); continue; }
        let chain = Promise.resolve();
        const sink = { head: (headers) => { chain = chain.then(() => post("head", { headers })); }, frame: (bytes) => { chain = chain.then(() => post("frame", { bytes })); } };
        const out = await serve(raw, sink); await chain;
        if (out.bytes) await post("fail", { message: JSON.parse(out.bytes).error.message });
      } catch (err) { await post("fail", { message: err.message }); }
    }
  } finally { relayRunning = false; }
}
function scheduleWatch(ms) { clearTimeout(relayTimer); relayTimer = setTimeout(relayWatch, ms ?? (sheetOpen() ? 2000 : 10000)); }
async function relayWatch() {
  if (!readConnect().enabled) return;
  try {
    const r = await fetch(RELAY + "/tab/hello", { cache: "no-store", headers: tabHeaders() });
    // Hello answered with a token: the relay is here and the loop's first poll registers this tab at
    // once, so the state is connected now; a 409 or a failed poll downgrades it.
    if (r.ok) { const h = await r.json(); relayToken = h.token || ""; setRelayState("connected"); relayLoop(); return; }
  } catch (e) {}
  setRelayState("listening"); scheduleWatch();
}

// ---- Connect: the pill in the box and its sheet. The pill shows when the model says the endpoint could
// answer (endpoint-ready: a resident model, or a kept key online). The words are the View's; the
// commands and snippets are product names and bytes, kept here.
const sheetOpen = () => !$("sheet").hidden;
let os = /Windows/i.test(navigator.userAgent) ? "win" : "mac", snip = "python", relayHash = "";
const RELAY_FILE = new URL("freeinference-relay.py", location.href).href;
const baseUrl = () => `${RELAY}/v1`;
const modelId = () => { const w = readWho(); return w.provider === "paid" ? "openrouter/" + (w.model || VIEW.paidModels[0].id) : MODEL_ID; };
const portEnv = { mac: RELAY_PORT === 11435 ? "" : `FREEINFERENCE_RELAY_PORT=${RELAY_PORT} `, win: RELAY_PORT === 11435 ? "" : `$env:FREEINFERENCE_RELAY_PORT=${RELAY_PORT}; ` };
const COMMANDS = {
  mac: { run: () => `curl -fsSLO ${RELAY_FILE} && ${portEnv.mac}python3 freeinference-relay.py`, verify: () => "shasum -a 256 freeinference-relay.py" },
  win: { run: () => `iwr ${RELAY_FILE} -OutFile freeinference-relay.py; ${portEnv.win}py freeinference-relay.py`, verify: () => "Get-FileHash freeinference-relay.py" },
};
const SNIPPETS = [
  ["python", "Python", () => `from openai import OpenAI\nclient = OpenAI(base_url="${baseUrl()}", api_key="local")\nr = client.chat.completions.create(model="${modelId()}", messages=[{"role": "user", "content": "Hello"}])\nprint(r.choices[0].message.content)`],
  ["node", "Node", () => `import OpenAI from "openai";\nconst client = new OpenAI({ baseURL: "${baseUrl()}", apiKey: "local" });\nconst r = await client.chat.completions.create({ model: "${modelId()}", messages: [{ role: "user", content: "Hello" }] });\nconsole.log(r.choices[0].message.content);`],
  ["curl", "curl", () => `curl ${baseUrl()}/chat/completions -H "Authorization: Bearer local" -H "Content-Type: application/json" -d '{"model":"${modelId()}","messages":[{"role":"user","content":"Hello"}]}'`],
  ["hermes", "Hermes", () => `CUSTOM_BASE_URL=${baseUrl()} CUSTOM_API_KEY=local hermes chat -q "Hello" -m ${modelId()} --provider custom --ignore-rules -t none`],
  ["openclaw", "OpenClaw", () => `// ~/.openclaw/openclaw.json\n"models": { "providers": { "local": { "baseUrl": "${baseUrl()}", "apiKey": "local", "api": "openai-completions",\n  "models": [{ "id": "${modelId()}", "name": "freeinference, in the browser" }] } } }\n// then: openclaw agent exec --model local/${modelId()} "Hello"`],
];
function paintConnect() {
  if (!VIEW) return;
  const dot = relayState === "connected" ? "dot on" : relayState === "listening" || relayState === "second" ? "dot wait" : "dot";
  $("pillDot").className = dot; $("sheetDot").className = dot;
  $("state").textContent = relayState === "connected" ? `${VIEW.connectedLabel} · 127.0.0.1:${RELAY_PORT}`
    : relayState === "second" ? `${VIEW.secondTabLabel}${relayOther ? " · " + relayOther : ""}`
    : relayState === "listening" ? VIEW.listeningLabel : VIEW.notConnectedLabel;
  $("ask").hidden = !(readConnect().enabled && !relaySeen && relayState !== "connected");
  $("cmd").textContent = COMMANDS[os].run(); $("verifycmd").textContent = COMMANDS[os].verify();
  $("hash").textContent = relayHash ? `sha256 ${relayHash}` : ""; $("hash").title = relayHash;
  $("baseUrl").textContent = baseUrl(); $("modelId").textContent = `${VIEW.modelIdLabel} · ${modelId()}`;
  $("snippet").textContent = SNIPPETS.find((s) => s[0] === snip)[2]();
  for (const b of document.querySelectorAll(".ostab")) b.setAttribute("aria-pressed", String(b.dataset.os === os));
  for (const b of document.querySelectorAll(".snip")) b.setAttribute("aria-pressed", String(b.dataset.snip === snip));
  $("test").disabled = relayState !== "connected";
}
async function refreshConnect() {
  if (!VIEW) return;
  const c = await coreReady();
  const ready = c.run({ op: "endpoint-ready", resident: !!gpuEngine, keyPresent: !!(await keyGet()), online: navigator.onLine }).ready;
  $("connect").hidden = !ready;
  if (!ready && sheetOpen()) closeSheet();
  paintConnect();
}
function openSheet() {
  $("sheet").hidden = false; $("scrim").hidden = false; $("connect").setAttribute("aria-expanded", "true");
  const c = readConnect(); if (!c.enabled) writeConnect({ ...c, enabled: true, since: Date.now() });
  if (relayState === "off") setRelayState("listening"); else paintConnect();
  relayWatch();
}
function closeSheet() { $("sheet").hidden = true; $("scrim").hidden = true; $("connect").setAttribute("aria-expanded", "false"); }
function initConnect() {
  for (const [id, label] of SNIPPETS) { const b = document.createElement("button"); b.type = "button"; b.className = "snip"; b.dataset.snip = id; b.textContent = label; b.onclick = () => { snip = id; paintConnect(); }; $("snips").appendChild(b); }
  for (const b of document.querySelectorAll(".ostab")) b.onclick = () => { os = b.dataset.os; paintConnect(); };
  for (const b of document.querySelectorAll(".copy")) b.onclick = async () => {
    try { await navigator.clipboard.writeText($(b.dataset.copy).textContent); b.textContent = VIEW.copiedLabel; setTimeout(() => { b.textContent = VIEW.copyLabel; }, 1200); } catch (e) {}
  };
  $("connect").onclick = (e) => { e.stopPropagation(); if (sheetOpen()) closeSheet(); else openSheet(); };
  $("scrim").onclick = closeSheet;
  document.addEventListener("keydown", (e) => { if (e.key === "Escape" && sheetOpen()) closeSheet(); });
  // The first proof: one request through the relay, as a harness would send it, answered by this tab.
  $("test").onclick = async () => {
    $("test").disabled = true; $("testOut").textContent = "…"; $("testOut").title = "";
    try {
      const t0 = performance.now();
      const r = await fetch(`${baseUrl()}/chat/completions`, { method: "POST", headers: { authorization: "Bearer local", "content-type": "application/json" }, body: JSON.stringify({ model: modelId(), messages: [{ role: "user", content: "Say hello in five words." }], max_tokens: 24 }) });
      const j = await r.json();
      if (!r.ok) throw new Error((j.error && j.error.message) || String(r.status));
      const reuse = r.headers.get("x-hologram-reuse") === "1"; const receipt = r.headers.get("x-hologram-receipt") || "";
      $("testOut").textContent = `“${j.choices[0].message.content}” · ${Math.round(performance.now() - t0)} ms · ${reuse ? VIEW.servedLabel : VIEW.sealedLabel} · ${short(receipt)}`;
      $("testOut").title = receipt;
    } catch (err) { $("testOut").textContent = `Error: ${err.message}`; }
    finally { $("test").disabled = relayState !== "connected"; }
  };
  fetch("manifest.json", { cache: "no-store" }).then((r) => r.json()).then((m) => { const f = (m.files || []).find((x) => x.path === "freeinference-relay.py"); relayHash = f ? f.sha256 : ""; paintConnect(); }).catch(() => {});
  window.addEventListener("online", refreshConnect); window.addEventListener("offline", refreshConnect);
  if (readConnect().enabled) relayWatch();
  refreshConnect();
}

// ---- appearance: the same canonical state and hooks Hologram OS keeps (holo.theme.v1; data-holo-palette,
// data-holo-immersive, --holo-wallpaper, color-scheme). Dark, Light, or Immersive on a curated Unsplash
// photo, credited as the Unsplash License asks. The pre paint script in index.html applied the saved
// choice before the first frame; this only changes it.
const KEY = "holo.theme.v1";
const WALLS = JSON.parse($("wallpapers").textContent);
const root = document.documentElement;
function readTheme() { try { return JSON.parse(localStorage.getItem(KEY) || "null") || {}; } catch (e) { return {}; } }
function applyTheme(s) {
  root.setAttribute("data-holo-palette", s.palette === "light" ? "light" : "dark");
  root.setAttribute("data-holo-immersive", s.immersive ? "on" : "off");
  root.style.setProperty("color-scheme", s.palette === "light" ? "light" : "dark");
  if (s.wallpaper) root.style.setProperty("--holo-wallpaper", `url(${JSON.stringify(s.wallpaper)})`);
  try { localStorage.setItem(KEY, JSON.stringify({ look: 2, ...s })); } catch (e) {}
  const mode = s.immersive ? "immersive" : s.palette === "light" ? "light" : "dark";
  for (const b of document.querySelectorAll(".mode")) b.setAttribute("aria-pressed", String(b.dataset.mode === mode));
  for (const b of document.querySelectorAll(".wall")) b.setAttribute("aria-pressed", String(s.immersive && b.dataset.wall === s.wallpaper));
  $("walls").classList.toggle("on", !!s.immersive);
}
function setMode(mode) {
  const s = readTheme();
  if (mode === "immersive") applyTheme({ palette: "dark", immersive: true, wallpaper: s.wallpaper || WALLS[0].file });
  else applyTheme({ palette: mode, immersive: false, wallpaper: s.wallpaper || WALLS[0].file });
}
$("appearance").onclick = (e) => { e.stopPropagation(); const open = $("popover").hidden; $("popover").hidden = !open; $("appearance").setAttribute("aria-expanded", String(open)); };
document.addEventListener("click", (e) => { if (!$("popover").contains(e.target)) { $("popover").hidden = true; $("appearance").setAttribute("aria-expanded", "false"); } });
for (const b of document.querySelectorAll(".mode")) b.onclick = () => setMode(b.dataset.mode);
for (const b of document.querySelectorAll(".wall")) b.onclick = () => applyTheme({ palette: "dark", immersive: true, wallpaper: b.dataset.wall });

// ---- the switch: two words, a model choice on the paid side, and a key field until a key is kept.
// One pill, one menu. The pill names who answers now; the menu is the whole list, your device or a
// paid model, so choosing a model is choosing the provider. No second control.
function openWhoMenu(open) { $("whoMenu").hidden = !open; $("whoPill").setAttribute("aria-expanded", String(open)); }
// The indicator: the concise name of the model that answers the next question, and a dot that pulses
// while the local model loads. Paid: the chosen paid model. Local and resident: the local model. Local
// and still loading with a key at hand: the warm up model, because that is who answers now.
let gpuPct = null;
async function refreshWho() {
  if (!VIEW) return;
  const w = readWho(); const paid = w.provider === "paid";
  const paidLabel = (VIEW.paidModels.find((m) => m.id === (w.model || VIEW.paidModels[0].id)) || VIEW.paidModels[0]).label;
  const keyed = !!(await keyGet());
  let name, state, title;
  if (paid) { name = paidLabel; state = keyed ? "ready" : "off"; title = keyed ? paidLabel : VIEW.noKeyLabel; }
  else if (gpuEngine) { name = VIEW.localModelName; state = "ready"; title = VIEW.modelLabel; }
  else if (keyed && navigator.onLine) { name = VIEW.paidModels[0].label; state = "loading"; title = `${VIEW.warmupLabel}${gpuPct != null ? ` · ${gpuPct}%` : ""}`; }
  else { name = VIEW.localModelName; state = "loading"; title = `${VIEW.loadingWord}${gpuPct != null ? ` · ${gpuPct}%` : ""}`; }
  $("whoCurrent").textContent = name; $("whoDot").dataset.state = state; $("whoPill").title = title;
}
async function applyWho(w) {
  const provider = w.provider === "paid" ? "paid" : "local";
  const model = provider === "paid" ? (w.model || VIEW.paidModels[0].id) : "";
  writeWho({ provider, model });
  await refreshWho();
  for (const o of document.querySelectorAll("#whoMenu .opt")) o.setAttribute("aria-selected", String(o.dataset.provider === provider && (provider !== "paid" || o.dataset.model === model)));
  $("keyrow").hidden = !(provider === "paid") || !!(await keyGet());
  $("keyhint").textContent = (await deviceKeyGet()) ? VIEW.keySavedLabel : siteKey ? VIEW.siteKeyLabel : VIEW.paidOnceLabel;
  refreshConnect();
}
$("whoPill").onclick = (e) => { e.stopPropagation(); openWhoMenu($("whoMenu").hidden); };
for (const o of document.querySelectorAll("#whoMenu .opt")) o.onclick = () => { applyWho({ provider: o.dataset.provider, model: o.dataset.model || "" }); openWhoMenu(false); $("whoPill").focus(); };
document.addEventListener("click", (e) => { if (!e.target.closest(".who")) openWhoMenu(false); });
document.addEventListener("keydown", (e) => { if (e.key === "Escape" && !$("whoMenu").hidden) { openWhoMenu(false); $("whoPill").focus(); } });
$("key").addEventListener("change", async () => { const v = $("key").value.trim(); if (!v) return; await keySet(v); $("key").value = ""; applyWho(readWho()); });
$("key").addEventListener("keydown", (e) => { if (e.key === "Enter") { e.preventDefault(); $("key").dispatchEvent(new Event("change")); } });

// ---- start: the shell is precached for offline, the words come from the verified core
(async () => {
  if ("serviceWorker" in navigator) navigator.serviceWorker.register("sw.js").catch(() => {});
  const c = await coreReady(); VIEW = c.run({ op: "view" });
  await siteKeyReady();
  applyTheme(readTheme()); applyWho(readWho());
  if (!navigator.gpu) setState(VIEW.noGpuLabel);
  else if (!navigator.onLine) setState(VIEW.offlineLabel);
  window.addEventListener("offline", () => setState(VIEW.offlineLabel));
  if (navigator.gpu) gpuReady().catch((err) => setState(`Error: ${err.message}`));
  initConnect();
})();
