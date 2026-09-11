# freeinference-prism

> **Merged into the product.** Since 2026-09-11 this model and its adapters live in [dyad-prism](https://github.com/humuhumu33/dyad-prism), the browser builder, as its Inference and Object modules; the landing page with Immersive, Dark and Light, the local versus paid switch, the Q engine, the seals and the OpenAI compatible endpoint are served from https://humuhumu33.github.io/dyad-prism/. This repository stays as the record of the inference module's own lane and gates; https://humuhumu33.github.io/freeinference-prism/ now sends every visitor to the product.


The trust core of [freeinference](https://github.com/humuhumu33/freeinference), "Free verified AI inference.", as one Lean verified model that is projected into Rust and WebAssembly. Nothing in the core is written by hand twice: the daemon and the browser get the same generated code, so a memo key or a canonical byte string is identical on both sides by construction.

## What is in the model

`src/Freeinference.lex.tex` is the only authority. It is a LexLean semantic module, the dialect PrismPM's own Calculator example is written in, and it declares:

- the shapes: `Message`, `Request`, `Memo`, `Preimages`, `Decision`;
- the rules: `renderPrompt` (the transcript a request keys on), `paramsCanonical` (the exact params bytes, keys sorted, nulls spelled), `preimages` (the two byte strings the BLAKE3 adapter addresses), `memoMatches` (when a stored answer serves a request), `decide` (serve, execute, or refuse with where compute runs);
- the facts, proven under Lean 4.32.1: what each rule unfolds to on concrete inputs, and that a memo matches its own key and nothing else.

`tools/author.py` writes the `.lex.tex` from short Python so the JSON dialect does not have to be typed by hand; CI checks that what is committed is what the script writes. `tools/axioms.json` pins the exact axiom set Lean observes for every declaration, because LexLean's `axioms` field is an exact policy.

## Modular by construction

LexLean models are modules that import each other, verified and content addressed one by one; PrismPM's own stdlib is a tree of `Foundation/Holo/V1/*.lex.tex` and `Foundation/View/...` modules referenced across files, and a project declares `source_roots`, `entrypoints` and an import depth. So freeinference's law, one capability equals one module, carries over unchanged: one `.lex.tex` module per capability, one entrypoint that composes them, one generated crate per module if wanted. What is not modular in PrismPM v0.3 is the application unit: one closed application root per `.holo`, one View family.

## The page, and the product in it

`https://humuhumu33.github.io/freeinference-prism/` is the product: one headline, one line, one box. Inference runs in the browser on your own GPU, with no server. It is a projection of the model, not a hand written page:

- every visible word is a field of the `View` record in the model, proven by Lean (`view_headline` pins the headline), and `core/src/bin/project_site.rs` renders `site/index.html`, the web manifest, the shell hash list and the service worker from the generated `view()`; CI refuses a page whose words drifted from the model;
- the rules the page applies are the generated core itself, `site/core.wasm`, reached through a small JSON ABI (`core/src/abi.rs`, a host transport adapter): memo preimages and the serve or refuse decision come from the verified code, addresses are BLAKE3 from the holospaces wasm, the store is IndexedDB on the device, receipts are Hologram Q's, and "check again" replays a sealed answer on the device;
- the engine is Hologram Q's WebGPU ternary engine, the same hash listed snapshot under `site/q/`, streaming BitNet 2B 4T from Hugging Face once, verified block by block, then resident from the device store;
- the shell is one versioned closure: the projector writes the closure digest into the service worker, so a new release is a new worker and a new cache, and the page keeps working offline;
- the look is the Hologram brand kit's tokens and its Archivo, Geist and Geist Mono web fonts, on a φ scale of type and spacing, with the three appearances Hologram OS has: Immersive by default, a curated Unsplash photo behind frosted glass, its attribution kept in `site/wallpapers/curated.receipt.jsonld` and on each thumbnail; Light; Dark. Nothing else is on the page: loading shows inside the box and clears when the model is ready. The toggle top right writes the same canonical state Hologram OS keeps (`holo.theme.v1`, `data-holo-palette`, `data-holo-immersive`, `--holo-wallpaper`), applied before the first paint; the three photos are vendored under `site/wallpapers/` with their attribution record.

## Connect any agent: the endpoint, from the homepage

The page is also an OpenAI compatible endpoint. Every response byte, the completion, each streamed chunk, the `[DONE]` frame, the error envelope and the model list, is produced by encoders in the model (`encodeCompletion`, `encodeRole`, `encodeDelta`, `encodeFinal`, `done`, `encodeError`, `encodeModels`, with JSON escaping as `split_exact` and `join` steps), generated into `core.wasm` and called through the same ABI the page uses. The adapters compute nothing: they decode the request, look the memo up, run the engine, seal, and move bytes.

A visitor enables it from the box. Once the local model is resident, or an OpenRouter key is kept and the device is online, a **Connect** pill appears on the box's bottom row; `endpointReady` in the model says when (`endpointReady_resident`, `endpointReady_paid`, `endpointReady_paidOffline`, `endpointReady_nothing`, one theorem per row), and the page asks the core through `endpoint-ready`. The pill opens one sheet: the state with a dot that turns green when the relay attaches, the one line to run for macOS and Linux or for Windows with the file's SHA-256 and the command to verify it, the base URL, one snippet per harness, and a **Send a test request** button that sends one request through the relay as a harness would and shows the answer with its receipt. The relay is probed only after the visitor has pressed Connect once, never at first paint, because a public page reaching `127.0.0.1` may make the browser ask.

A browser tab cannot listen on a port, so the endpoint has two transports, and the README says what each cannot reach:

1. **The page's own origin.** While the page is open, its service worker answers `POST /v1/chat/completions` and `GET /v1/models` under `https://humuhumu33.github.io/freeinference-prism/` for any page on that origin. Zero servers. It does not reach a native process, and a page on another origin is not routed through this worker.
2. **The relay.** `freeinference-relay.py`, one file on the Python standard library, served by the page itself (`site/freeinference-relay.py`, hashed into `manifest.json` with the rest of the shell), listens on `http://127.0.0.1:11435/v1` and forwards each request to the open tab, which streams the answer back. It is a local process, not a server anyone else can reach; it stores nothing and computes nothing. It is what native harnesses need.

A third shape, the harness driving the tab over Chrome DevTools, was not built: Hermes's and OpenClaw's model providers are HTTP clients, not browser sessions.

```bash
curl -fsSLO https://humuhumu33.github.io/freeinference-prism/freeinference-relay.py && python3 freeinference-relay.py
shasum -a 256 freeinference-relay.py      # the sheet shows the expected digest; on Windows, Get-FileHash
```

| Client | The one line |
| --- | --- |
| any OpenAI client | `OPENAI_BASE_URL=http://127.0.0.1:11435/v1`, any key (for example `local`), model `webgpu:BitNet` or an `openrouter/…` id |
| Hermes 0.15 | `CUSTOM_BASE_URL=http://127.0.0.1:11435/v1 CUSTOM_API_KEY=local hermes chat -q "…" -m webgpu:BitNet --provider custom --ignore-rules -t none` |
| OpenClaw | in `~/.openclaw/openclaw.json`: `models.providers.local = { baseUrl: "http://127.0.0.1:11435/v1", apiKey: "local", api: "openai-completions", models: [{ id: "webgpu:BitNet", name: "BitNet 2B, in the browser" }] }`, then `openclaw agent exec --model local/webgpu:BitNet "…"` |

While the local model is still loading, a visitor who keeps an OpenRouter key on the device is answered through OpenRouter and told so by the chip (`warmup`, a rule of the model); and the site may include a key of its own: the Pages workflow writes `site/warmup.json` from the repository secret `OPENROUTER_WARMUP_KEY` at deploy time, never from the tree, and the page counts it as a key with the visitor's own key winning. A key in a public page is public, so the operator caps it at OpenRouter and rotates it by updating the secret; `relay/openrouter-worker.js` is the guarded form, a small server with a daily budget per visitor, for the day the cap is not enough. Every answer carries `x-hologram-receipt`; a repeated request is served from its seal on the device with `x-hologram-reuse: 1`, and the last streamed chunk carries `hologram.receipt`. `/v1/openapi.json` is written by the projector with examples taken from the encoders, so it cannot drift from the model. `model/wire.json` holds the acceptance vectors; `tools/corpus.py` checks them through the crate and, with `tools/guest.mjs`, through the wasm guest exactly as the page calls it.

**What the relay defends against, and what it does not.** The threat is a web page on another origin, open in the same browser, using the relay: to make the visitor's GPU answer for it, to pose as the tab and inject answers, or to drain the jobs a real harness sent. So the relay binds loopback only and refuses any other host; every `/v1` request needs an `Authorization` header of any value, and every `/tab` request needs a custom header and, after `/tab/hello`, the token this run minted, so a browser must preflight, and the preflight is granted only to the page's origin (or a copy of the page served from the machine's own loopback); one tab serves at a time and a second is told which; bodies are capped at 1 MB, each job has a timeout, and the log names method, path, status and time, never a message. What it does not defend against is another process on the same machine, which can already read the screen and the disk: the relay trusts the machine as Ollama does. No key ever passes the relay: paid requests leave from the tab to `openrouter.ai` directly. `tools/relay_test.py` plays a fake tab, a hostile page and a client without a bearer against the relay on every CI run.

## The switch: on your device, or paid

The box has one switch with two words from the View: on your device, and paid. Local is the default and needs nothing. Paid runs the same request through OpenRouter with a key the visitor pastes once, kept in the device store under the page's origin and sent nowhere but the `Authorization` header to `openrouter.ai` (the browser may call it directly: the preflight from the page origin is allowed, so the paid path is zero servers too). The route is a rule in the model (`Provider`, `Route`, `route`), one theorem per row: a memo hit serves on both sides, paid without a key refuses with the View's word, paid offline refuses, local without a GPU refuses. The bytes OpenRouter receives are the model's `encodeOpenRouterRequest` (model, messages, the optional fields, stream, usage, reasoning off, and providers that honor every field), pinned by the corpus, never carrying a key. A paid answer is sealed like a local one, with OpenRouter's id, provider, usage and cost in its receipt and the honest note that it cannot be re derived on device; the identical request next is served from the seal, free, on either side. The endpoint follows the switch, or a request names `openrouter/<id>` explicitly; `/v1/models` lists the resident model and the three paid names. CI refuses any `sk-or-` string in the tree.

## The κ object: every weight its own address

`model/` also carries the first κ object built with this model's addressing rule (`Range`, `Obj`, `Shard`, `Manifest`; `expertPage`, `tablePage`, `objEntry`, `rootPreimage`, `admitPage`): edge0's 8B mixture of experts checkpoint, 1,550 tensor κs and 26,496 expert page κs derived straight off the wire with nothing stored by `HOLOGRAM/tools/kappa_object.py`. Every tensor is a κ over its bytes; every expert of every layer is a page κ over its rows; every fixed page of an n gram table would be a page κ; each shard's object list is its own κ object, one line per object as `objEntry` spells it; the manifest names them and the root is BLAKE3 over `rootPreimage`, the canonical JSON the model produces. The corpus feeds the real manifest and all 28,046 object lines through the crate and the wasm guest and refuses any drift; a page binds only if `admitPage` says the root lists it and the bytes derive it. The page arithmetic is checked: an overflow is a refusal, never a wrapped address.

## The pool: the streaming expert pool's rules, verified

The next object the page will stream is a mixture of experts whose pages page in and out of a pool. Its rules are in the model as decision tables, one theorem per row: what happens to a routed page (`pageAction`: bind if resident, fetch on a miss under true routing, drop on a miss under staged replace), how the pool admits (`poolAdmit`: touch, insert, evict then insert), where a missing page comes from (`fetchSource`: the device store, a faster peer, a mirror, nowhere) and what the prefetcher pulls next (`prefetchOrder`: a predicted page first, a popular one to fill). The bookkeeping of slots is transport; the decisions are the model's. A recorded routing trace of OLMoE 1B 7B, the engine's real mixture of experts, is a corpus vector: replayed through `poolAdmit` the hit counts per pool size must equal the Python restatement through the crate and the guest. The archive's first use order (`packRank`, `packed`), the first token's readiness (`firstTokenReady`), the ladder that promotes from the small resident model to the large one only when it is resident and measured fast (`promote`, never demoting) and how a visit starts (`loaderStart`: resume, warm, cold) are tables of the same kind. So are context as κ (`kvBlockPreimage`, `hitLength`, `checkpointDue`, `replayBound`: a block of tokens' KV has an address, a turn computes only its new blocks, a session resumes from the longest common prefix), the device plan from a real probe (`planFor`) and the quant tier the pack and the loader agree on (`qwen38Tier`).

## What is generated

`scripts/lane.sh` replays the whole lane from the committed tree, on Linux:

1. LexLean, the compiler PrismPM vendors, at the PrismPM commit in `PRISMPM_REV`: `lock` must be current, then `check`, `build`, `verify` (Lean, leanchecker replay, per declaration axiom audit).
2. lean4-prod, as PrismPM vendors it: every definition in `model/roots.txt` exported to kernel LCNF twice, byte identical, then generated to Rust twice, byte identical, into `generated/freeinference_core.rs`.
3. The `core/` crate, which is that generated file behind a wrapper that only names the refusal type, built for the host and for `wasm32-unknown-unknown`; the wasm is copied to `site/core.wasm` and the page is projected from `view()`.

`tools/corpus.py` then feeds a fixed request corpus to the generated core and compares the prompt and params bytes against the daemon's rule, feeds the wire vectors and the OpenRouter request vectors to the encoders, and feeds the real edge0-8b κ object's manifest and object lines to the addressing rule, all through the crate and through `site/core.wasm`. `just vv` is the only definition of green; CI runs it. `scripts/plant.sh`, `scripts/plant-openrouter.sh`, `scripts/plant-address.sh` and `scripts/plant-readiness.sh` plant defects (a field swap in the completion encoder, a field swap in the OpenRouter request and a route that pays without a key, an off by one in the expert page, a key that counts as ready offline) and show the corpus or Lean refusing each.

## What PrismPM itself does and does not do here, with evidence

This repo pins PrismPM at `f57a697` and uses its vendored LexLean and lean4-prod exactly as PrismPM's own lane does. It does not run `prismpm build`, and it says why:

- PrismPM's application projection is one shape. `crates/prismpm/src/holo/application.rs` requires an application record with `operationType`, `errorType`, `functionName`, and a `view` whose fields are `leftLabel`, `rightLabel`, `operationLabel`, `divisionError`, `overflowError` and an operations list; `application_build.rs` binds the generated browser adapter to `core_function(Operation, i64, i64)`. That is the Calculator. A chat request is not two operands and an operation, and padding the model to look like one would be a lie the source audit is there to catch.
- PrismPM's other path is the ISO facet system model. `crates/prismpm/src/holo/projector.rs` refuses any non application snapshot whose lexicon closure is not exactly `prism.arch`, `prism.qual`, `prism.sec` ("facet closure is not exact"). That path describes architectures; it generates no code.
- The generic parts PrismPM has, an import free Core-Wasm guest over `dispatch(bytes) -> bytes` exported as `holo_run`, acceptance vectors, and a v4 `.holo` archive, are reachable only through the application projection today.

So this repo is conformant where conformance is possible: the same compiler, the same extraction, the same exact axiom discipline, the same byte identical double generation, the same honesty about what is proven and what is measured. The step that turns it into a PrismPM application is upstream: a second View family for text requests and responses, or an application root without a View. `UPSTREAM.md` states that request precisely.

## Build

Linux x86_64, WSL on this machine. Needs git, cargo with the `wasm32-unknown-unknown` target, python3, and the elan toolchain `leanprover/lean4:v4.32.1` (installed on demand).

```bash
just vv
```

The first run clones PrismPM at the pinned commit and builds its LexLean, which takes a few minutes; every later run is seconds for the checks and about a minute for the full lane.
