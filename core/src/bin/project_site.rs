//! The page projector: renders `site/index.html` from the generated `view()`.
//!
//! An adapter, not the model: every visible word comes from the Lean
//! verified `View` record; this file owns only markup and the brand kit's
//! tokens. It also writes `site/manifest.json`, the hash list the service
//! worker precaches from, so the shell is one versioned closure.
//!
//! Run from the repo root: cargo run --release --manifest-path core/Cargo.toml --bin project-site

use freeinference_core::{encodeCompletion, encodeError, encodeModels, view, Completion};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn esc(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn page() -> String {
    let v = view();
    let mut h = String::new();
    let _ = write!(
        h,
        r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
<meta name="color-scheme" content="light">
<meta name="theme-color" content="#d8e1fc">
<title>{title}</title>
<meta name="description" content="{lede}">
<link rel="icon" href="mark.svg" type="image/svg+xml">
<link rel="manifest" href="app.webmanifest">
<link rel="stylesheet" href="app.css">
<script>
// Pre paint appearance, the same canonical state Hologram OS keeps (holo.theme.v1: palette, immersive,
// wallpaper) and the same hooks (data-holo-palette, data-holo-immersive, --holo-wallpaper, color-scheme),
// so the first frame already wears the chosen look. First run, and once for anyone who chose before
// look 2 (the sky gradient): light, no photo.
(function () {{
  var root = document.documentElement, s = null;
  try {{ s = JSON.parse(localStorage.getItem("holo.theme.v1") || "null"); }} catch (e) {{}}
  if (!s || s.look !== 2) {{ s = {{ look: 2, palette: "light", immersive: false, wallpaper: "wallpapers/{wall0}" }}; try {{ localStorage.setItem("holo.theme.v1", JSON.stringify(s)); }} catch (e) {{}} }}
  root.setAttribute("data-holo-palette", s.palette === "light" ? "light" : "dark");
  root.setAttribute("data-holo-immersive", s.immersive ? "on" : "off");
  root.style.setProperty("color-scheme", s.palette === "light" ? "light" : "dark");
  if (s.wallpaper) root.style.setProperty("--holo-wallpaper", "url(" + JSON.stringify(s.wallpaper) + ")");
}})();
</script>
</head>
<body>
<a class="mark" href="./" aria-label="Hologram"><img class="on-dark" src="lockup-white.svg" alt="Hologram" width="157" height="30"><img class="on-light" src="lockup-black.svg" alt="Hologram" width="157" height="30"></a>
<button class="appearance" id="appearance" type="button" aria-haspopup="dialog" aria-expanded="false" aria-label="{appearance}"><svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg></button>
<div class="popover" id="popover" role="dialog" aria-label="{appearance}" hidden>
  <button class="mode" type="button" data-mode="immersive"><span>{immersive}</span><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="3" y="4" width="18" height="16" rx="2"/><circle cx="8.5" cy="9.5" r="1.5"/><path d="M21 16l-5-5-8 8M3 18l4-4 3 3"/></svg></button>
  <div class="walls" id="walls">{walls}</div>
  <button class="mode" type="button" data-mode="dark"><span>{dark}</span><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M20 14.5A8 8 0 0 1 9.5 4a8 8 0 1 0 10.5 10.5z"/></svg></button>
  <button class="mode" type="button" data-mode="light"><span>{light}</span><svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" aria-hidden="true"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg></button>
</div>
<script type="application/json" id="wallpapers">{walls_json}</script>
<main>
  <h1>{title}</h1>
  <p class="lede">{lede}</p>
  <div class="messages" id="messages"></div>
  <form class="composer" id="composer">
    <textarea id="input" rows="1" placeholder="{placeholder}" autocomplete="off" autofocus></textarea>
    <div class="keyrow" id="keyrow" hidden>
      <input class="key" id="key" type="password" placeholder="{key_placeholder}" aria-label="{key_label}" autocomplete="off" spellcheck="false">
      <span class="hint mono" id="keyhint">{paid_once}</span>
    </div>
    <div class="row">
      <div class="who" role="radiogroup" aria-label="{local} / {paid}">
        <button class="mode2" type="button" data-provider="local" aria-pressed="true">{local}</button>
        <button class="mode2" type="button" data-provider="paid" aria-pressed="false">{paid}</button>
        <select class="pick" id="paidModel" aria-label="{paid}" hidden>{paid_models}</select>
      </div>
      <span class="hint mono" id="hint"></span>
      <button class="btn" id="send" type="submit" aria-label="{send}">{send}</button>
    </div>
  </form>
</main>
<script type="module" src="app.js"></script>
</body>
</html>
"##,
        title = esc(&v.headline),
        lede = esc(&v.lede),
        placeholder = esc(&v.promptPlaceholder),
        send = esc(&v.sendLabel),
        appearance = esc(&v.appearanceLabel),
        dark = esc(&v.darkLabel),
        light = esc(&v.lightLabel),
        immersive = esc(&v.immersiveLabel),
        wall0 = esc(&v.wallpapers[0].file),
        local = esc(&v.localLabel),
        paid = esc(&v.paidLabel),
        key_label = esc(&v.keyLabel),
        key_placeholder = esc(&v.keyPlaceholder),
        paid_once = esc(&v.paidOnceLabel),
        paid_models = v.paidModels.iter().map(|m| format!(r#"<option value="{}">{}</option>"#, esc(&m.id), esc(&m.label))).collect::<Vec<_>>().join(""),
        walls = v.wallpapers.iter().map(|w| format!(r#"<button class="wall" type="button" data-wall="wallpapers/{}" title="{}" aria-label="{}" style="background-image:url(wallpapers/{})"></button>"#, esc(&w.file), esc(&w.label), esc(&w.label), esc(&w.file))).collect::<Vec<_>>().join(""),
        walls_json = serde_json::json!(v.wallpapers.iter().map(|w| serde_json::json!({ "file": format!("wallpapers/{}", w.file), "name": w.label, "by": w.author, "byUrl": w.authorUrl })).collect::<Vec<_>>()).to_string().replace("</", "<\\/"),
    );
    h
}

fn webmanifest() -> String {
    let v = view();
    serde_json::json!({
        "name": "freeinference",
        "short_name": "freeinference",
        "description": v.lede,
        "start_url": "./",
        "display": "standalone",
        "background_color": "#d8e1fc",
        "theme_color": "#d8e1fc",
        "icons": [{ "src": "mark.svg", "sizes": "any", "type": "image/svg+xml" }]
    })
    .to_string()
        + "\n"
}

/// The endpoint's OpenAPI document. The examples are not typed here: they are the bytes the
/// generated encoders produce, so the document cannot drift from the model.
fn openapi() -> String {
    let sample = Completion {
        id: "chatcmpl-3f1c9a2b7d40".to_owned(),
        created: "1757500000".to_owned(),
        model: "webgpu:BitNet".to_owned(),
        text: "Hello.".to_owned(),
        fingerprint: "blake3:e41292a4…;blake3:9c0d…".to_owned(),
        receipt: "did:holo:sha256:…".to_owned(),
    };
    let parse = |bytes: String| serde_json::from_str::<serde_json::Value>(&bytes).expect("encoder emits JSON");
    let doc = serde_json::json!({
        "openapi": "3.1.0",
        "info": {
            "title": "freeinference",
            "version": "1",
            "summary": "OpenAI compatible chat completions served from the browser tab that has this page open.",
            "description": "Every answer carries a receipt (x-hologram-receipt, and hologram.receipt on the last streamed chunk). A repeated request is served from its seal on the device with x-hologram-reuse: 1. On this origin the service worker answers; on a machine, relay/freeinference-relay.py forwards http://127.0.0.1:11435/v1 to the tab. No server computes or stores anything."
        },
        "servers": [
            { "url": "https://humuhumu33.github.io/freeinference-prism/v1", "description": "the page's own origin, answered by the service worker while the page is open" },
            { "url": "http://127.0.0.1:11435/v1", "description": "the local relay, for native clients such as Hermes and OpenClaw" }
        ],
        "paths": {
            "/models": { "get": { "operationId": "listModels", "responses": { "200": { "description": "the resident model", "content": { "application/json": { "example": parse(encodeModels(&["webgpu:BitNet".to_owned()])) } } } } } },
            "/chat/completions": { "post": {
                "operationId": "createChatCompletion",
                "requestBody": { "required": true, "content": { "application/json": { "schema": { "$ref": "#/components/schemas/ChatCompletionRequest" },
                    "example": { "model": "webgpu:BitNet", "messages": [{ "role": "user", "content": "Hello" }], "max_tokens": 512, "temperature": 0.7, "stream": true } } } },
                "responses": {
                    "200": { "description": "the answer; with stream: true, chunks as text/event-stream ending in data: [DONE]",
                        "headers": {
                            "x-hologram-receipt": { "description": "the answer's receipt id", "schema": { "type": "string" } },
                            "x-hologram-reuse": { "description": "1 when served from the seal without running the model", "schema": { "type": "string" } },
                            "x-hologram-stream": { "description": "native, memo, or plain", "schema": { "type": "string" } }
                        },
                        "content": { "application/json": { "example": parse(encodeCompletion(&sample)) }, "text/event-stream": { "schema": { "type": "string" } } } },
                    "400": { "description": "malformed request", "content": { "application/json": { "example": parse(encodeError("messages must not be empty".to_owned(), "invalid_request_error".to_owned())) } } },
                    "503": { "description": "no page is open, or this browser cannot run the model" }
                } } }
        },
        "components": { "schemas": {
            "ChatCompletionRequest": { "type": "object", "required": ["messages"], "properties": {
                "model": { "type": "string", "description": "ignored; the resident model answers" },
                "messages": { "type": "array", "items": { "type": "object", "required": ["role", "content"], "properties": { "role": { "type": "string" }, "content": { "type": "string" } } } },
                "max_tokens": { "type": "integer" }, "max_completion_tokens": { "type": "integer" },
                "temperature": { "type": "number" }, "seed": { "type": "integer" }, "stream": { "type": "boolean" }
            } }
        } }
    });
    serde_json::to_string_pretty(&doc).unwrap() + "\n"
}

/// Every file of the app shell with its SHA-256, lexically ordered. Model
/// weights are not here: they live in the device store the engine keeps.
fn manifest(site: &Path) -> String {
    fn walk(dir: &Path, root: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).expect("site dir").flatten().collect();
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, root, out);
            } else {
                out.push(path.strip_prefix(root).unwrap().to_path_buf());
            }
        }
    }
    let mut files = Vec::new();
    walk(site, site, &mut files);
    let mut rows = Vec::new();
    for rel in files {
        let name = rel.to_string_lossy().replace('\\', "/");
        if name == "manifest.json" || name == "sw.js" || name == "sw.template.js" {
            continue;
        }
        let bytes = std::fs::read(site.join(&rel)).expect("read site file");
        rows.push(serde_json::json!({ "path": name, "sha256": sha256(&bytes), "bytes": bytes.len() }));
    }
    let closure = sha256(serde_json::to_string(&rows).unwrap().as_bytes());
    serde_json::json!({ "spec": "freeinference/shell/1", "closure": closure, "files": rows }).to_string() + "\n"
}

fn sha256(bytes: &[u8]) -> String {
    // A tiny SHA-256 so the projector carries no extra dependency; correctness is
    // checked against Python's hashlib by tools/corpus.py on every run.
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut data = bytes.to_vec();
    let bit_len = (bytes.len() as u64) * 8;
    data.push(0x80);
    while data.len() % 64 != 56 {
        data.push(0);
    }
    data.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in data.chunks(64) {
        let mut w = [0u32; 64];
        for (i, word) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g; g = f; f = e; e = d.wrapping_add(t1); d = c; c = b; b = a; a = t1.wrapping_add(t2);
        }
        for (slot, value) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(value);
        }
    }
    h.iter().map(|word| format!("{word:08x}")).collect()
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let site = root.join("site");
    std::fs::write(site.join("index.html"), page()).expect("write index.html");
    std::fs::write(site.join("app.webmanifest"), webmanifest()).expect("write webmanifest");
    std::fs::create_dir_all(site.join("v1")).expect("site/v1");
    std::fs::write(site.join("v1").join("openapi.json"), openapi()).expect("write openapi.json");
    let manifest = manifest(&site);
    std::fs::write(site.join("manifest.json"), &manifest).expect("write manifest.json");
    // The closure digest is written into the worker itself, so a new shell is a new worker: the
    // browser installs it, precaches the new closure, and drops the old cache on activation.
    let closure = serde_json::from_str::<serde_json::Value>(&manifest).unwrap()["closure"].as_str().unwrap().to_owned();
    let worker = std::fs::read_to_string(site.join("sw.template.js")).expect("read sw.template.js");
    std::fs::write(site.join("sw.js"), worker.replace("__CLOSURE__", &closure)).expect("write sw.js");
    println!("projected site/index.html, site/app.webmanifest, site/v1/openapi.json, site/manifest.json, site/sw.js from the model; closure {}", &closure[..12]);
}
