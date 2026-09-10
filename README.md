# freeinference-prism

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
- the look is the Hologram brand kit's light warm layer and its Archivo, Geist and Geist Mono web fonts, on a φ scale of type and spacing.

Next on this page: the OpenAI compatible endpoint served by the same service worker on the page's origin, so any agent harness that can reach a page on that origin, or embed it, talks to the tab.

## What is generated

`scripts/lane.sh` replays the whole lane from the committed tree, on Linux:

1. LexLean, the compiler PrismPM vendors, at the PrismPM commit in `PRISMPM_REV`: `lock` must be current, then `check`, `build`, `verify` (Lean, leanchecker replay, per declaration axiom audit).
2. lean4-prod, as PrismPM vendors it: every definition in `model/roots.txt` exported to kernel LCNF twice, byte identical, then generated to Rust twice, byte identical, into `generated/freeinference_core.rs`.
3. The `core/` crate, which is that generated file behind a wrapper that only names the refusal type, built for the host and for `wasm32-unknown-unknown`; the wasm is copied to `site/core.wasm` and the page is projected from `view()`.

`tools/corpus.py` then feeds a fixed request corpus to the generated core and compares the prompt and params bytes against the daemon's rule. `just vv` is the only definition of green; CI runs it.

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
