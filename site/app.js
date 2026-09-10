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
      onProgress: (d, t) => setState(t ? `${VIEW.loadingLabel} · ${Math.round((d / t) * 100)}%` : VIEW.loadingLabel),
    });
    if (!loaded || !loaded.gpu) throw new Error("model load failed");
    gpuEngine = await E.createEngine(gpuModel, loaded);
    setState(`${VIEW.residentLabel} · ${((performance.now() - t0) / 1000).toFixed(1)} s`, true);
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

// ---- the page
const history = [];
function setState(text, ok = false) { const s = $("state"); s.textContent = text; s.className = "state mono" + (ok ? " ok" : ""); }
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

async function turn(messages, body, el) {
  const t0 = performance.now();
  const hit = await lookup(body);
  if (hit.hit) {
    el.textContent = hit.text;
    chips(el, [{ text: VIEW.modelLabel, title: hit.fingerprint }, { text: VIEW.servedLabel, title: hit.receipt, ok: true }, rederiveChip(hit.rec)]);
    $("hint").textContent = `${Math.round(performance.now() - t0)} ms · ${VIEW.servedLabel}`;
    return hit.text;
  }
  if (!navigator.gpu) throw new Error(VIEW.noGpuLabel);
  const engine = await gpuReady();
  const { ids, warm } = gpuIds(engine, messages);
  const started = performance.now();
  const live = (s) => `${warm ? "warm" : "cold"} · first token ${Math.round(s.ttft || 0)} ms · ${(s.tokps || 0).toFixed(1)} tok/s`;
  const res = await engine.generate(ids, { maxNew: body.max_tokens, onToken: ({ text, stats }) => { el.textContent = text; if (stats) $("hint").textContent = live(stats); } });
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
  const body = { model: "webgpu:BitNet", messages, max_tokens: 512, temperature: "0.7" };
  const el = add("assistant", "");
  try { const content = await turn(messages, body, el); history.push({ role: "assistant", content }); }
  catch (err) { el.textContent = `Error: ${err.message}`; }
  finally { $("send").disabled = false; $("input").focus(); }
};

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
  try { localStorage.setItem(KEY, JSON.stringify(s)); } catch (e) {}
  const mode = s.immersive ? "immersive" : s.palette === "light" ? "light" : "dark";
  for (const b of document.querySelectorAll(".mode")) b.setAttribute("aria-pressed", String(b.dataset.mode === mode));
  for (const b of document.querySelectorAll(".wall")) b.setAttribute("aria-pressed", String(s.immersive && b.dataset.wall === s.wallpaper));
  $("walls").classList.toggle("on", !!s.immersive);
  const w = s.immersive && WALLS.find((x) => x.file === s.wallpaper);
  $("credit").innerHTML = w && VIEW ? `${VIEW.photoLabel} <strong>${w.name}</strong> ${VIEW.byLabel} <a href="${w.byUrl}" rel="noopener">${w.by}</a> <a href="https://unsplash.com/?utm_source=Hologram_AI&utm_medium=referral" rel="noopener">${VIEW.unsplashLabel}</a>` : "";
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

// ---- start: the shell is precached for offline, the words come from the verified core
(async () => {
  if ("serviceWorker" in navigator) navigator.serviceWorker.register("sw.js").catch(() => {});
  const c = await coreReady(); VIEW = c.run({ op: "view" });
  applyTheme(readTheme());
  const stored = (await all()).filter((o) => o.kind === "memo").length;
  if (!navigator.gpu) setState(VIEW.noGpuLabel);
  else if (!navigator.onLine) setState(VIEW.offlineLabel, true);
  else setState(stored ? `${VIEW.residentLabel}, ${stored} answers on this device` : VIEW.loadingLabel);
  window.addEventListener("offline", () => setState(VIEW.offlineLabel, true));
  if (navigator.gpu) gpuReady().catch((err) => setState(`Error: ${err.message}`));
})();
