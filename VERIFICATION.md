# VERIFICATION

What is proven, what is measured, what was planted to prove the gates can fail, and what was found on the way. Dated 2026-09-10, one developer machine (WSL Ubuntu on Windows 11, x86_64), PrismPM at `f57a697`, Lean `leanprover/lean4:v4.32.1`.

## Proven, by Lean

`src/Freeinference.lex.tex`: 28 declarations, 4 structures, 1 inductive, 12 definitions, 11 theorems; 26,949 bytes of source, written by 188 lines of `tools/author.py`. `lexlean verify` builds the module, checks every theorem, replays the kernel with leanchecker, and audits the observed axiom set of every declaration against the exact set pinned in `tools/axioms.json`: 13 declarations observe `propext` or `Classical.choice, Quot.sound, propext` through Lean's string library, 15 observe none.

| Theorem | What it pins | Proof |
| --- | --- | --- |
| `renderMessage_shape` | one line is role, colon space, content | reflexivity |
| `renderPrompt_singleUser`, `renderPrompt_twoTurns` | lines joined by newline, folded without an intermediate list | reflexivity |
| `paramsCanonical_empty`, `paramsCanonical_full` | key order `max_tokens`, `seed`, `temperature`; nulls spelled `null`; numbers by `format_decimal` | reflexivity |
| `decide_serve`, `decide_execute`, `decide_refuse` | a hit serves; a worker executes; otherwise refuse | reflexivity |
| `memoMatches_ownKey`, `memoMatches_otherPrompt`, `memoMatches_otherModel` | a memo matches its own key and neither another prompt nor another model | decide |

String literal equality is not kernel reducible in Lean 4.32, where `String` is byte based: `rfl` and `decide` both stall on `paramsCanonical none none "" = "{...}"` (diagnostics LLV7002, recorded on the first attempt). So the theorems pin each rule definitionally, what the function unfolds to on a concrete input, and the bytes are pinned by the corpus below. That split is honest and it is also where the planted defect lands.

## Measured

| Step | Result |
| --- | --- |
| `lexlean check` | 0.2 s |
| `lexlean verify` (Lean, leanchecker, axiom audit) | 8 s |
| `scripts/lane.sh`, warm (export twice, generate twice, host and wasm32 builds, byte compare) | 26 s |
| `scripts/lane.sh`, first run (PrismPM clone, LexLean release build, lean4-prod build) | about 6 min |
| Generated | `generated/kernel.ir` 68 lines, `generated/freeinference_core.rs` 132 lines, wrapper `core/src/lib.rs` 24 lines |
| Corpus | 16 requests, prompt and params bytes identical between the generated core and the daemon's rule |
| Export determinism | `kernel.ir`, `roots.json`, `coverage.json` byte identical across two exports; `freeinference_core.rs` byte identical across two generations |

The wasm32 build is 326 bytes because the crate exports no C ABI yet; the guest ABI is the adapter's job, not the model's.

## Measured, the page, 2026-09-10

Served locally from `site/` on this machine's browser pane, then the same files deployed by the Pages workflow.

| Case | Result |
| --- | --- |
| Words | `site/index.html` projected from `view()`; the headline is theorem `view_headline`; zero hyphens in the View's strings |
| Core in the browser | `site/core.wasm` 118 KB, the generated core behind the JSON ABI; `view`, `preimages` and `decide` answered from it |
| Shell | 36 files hash listed in `manifest.json`, every digest equal to hashlib's; closure written into the worker; the worker precached the closure and, on a changed stylesheet, installed the new closure and dropped the old cache |
| First visit | BitNet 2B 4T streamed from Hugging Face and verified block by block, resident on the GPU in 56.7 s |
| Later visit | Resident from the device store in 4.6 s to 6.9 s |
| First answer | Cold prefill, first token 1361 ms, a short answer in 1660 ms, sealed under a Q receipt, stored with its answer bytes and memo |
| Repeat after reload | Served from the receipt in 4 ms, while the model was still loading; memo key from the wasm core, address from the holospaces wasm, integrity from the receipt's own `did:holo` |
| Re-derive | The stored receipt replayed on the GPU: identical |
| Phone viewport | 375 px wide: no horizontal overflow after the composer row fix; the page paints with the shell alone |
| Appearance | Immersive by default on a first visit (Alpine Dawn, applied before the first paint from `holo.theme.v1`); Light and Dark switch live from the top right tile and persist; the three vendored photos switch from the popover with the credit line updating |
| Chips | One row of three: the model by name, sealed or served from the seal, and the check; addresses only in hover titles |
| Worker cache | A stale `core.wasm` was served once because the precache went through the HTTP cache; the worker now fetches every shell file with `cache: "reload"` at install |

Not measured yet: a second device with a different GPU, and the airplane test with the model already resident (the shell part is by construction; the model part is the engine's device store).

## Measured, the endpoint, 2026-09-10

The model gained `Completion` and the seven wire encoders (57 declarations, 35 with pinned axiom sets, verify attestation `be3fdf46…`; lane closure `0eefca52…` before the page changes). Measured on this machine with the page open in the local browser pane and `relay/freeinference-relay.py` on `127.0.0.1:11435`.

| Claim | Scenario | Result |
| --- | --- | --- |
| EP-01 | `{}` and non JSON bodies to the endpoint | `400` with the model's error envelope, from `encodeError`; no panic. The request decoder is the adapter's `JSON.parse` plus a field map, not the model: see "Not built" |
| EP-02 | a request whose memo hits | served from the seal, `x-hologram-reuse: 1`, the original receipt in `x-hologram-receipt`; no engine call; 5 ms on the page origin, 58 ms end to end through the relay with curl, 24 ms with the openai client |
| EP-03 | a fresh request with `stream: true` | role chunk, one chunk per token, final chunk with `hologram.receipt`, `data: [DONE]`; sealed on the device; the identical request next was served from the seal |
| EP-04 | `GET /v1/models`, `GET /v1/openapi.json` | `{"object":"list","data":[{"id":"webgpu:BitNet",…}]}` from `encodeModels`; the OpenAPI 3.1 document written by the projector with the encoders' bytes as examples |
| EP-05 | Hermes 0.15.1, `CUSTOM_BASE_URL` and `--provider custom`, no config file | first turn 17.6 s wall clock of which Hermes's own startup is 8.6 s (`hermes prompt-size` with no API call); the repeated turn 10.7 s and 11.7 s wall clock, both served from the seal (no new receipt in the store), identical text |
| EP-06 | OpenClaw | see below |
| EP-07 | network off | not measured here: the pane cannot cut the network; by construction the endpoint path touches only the precached shell, `core.wasm`, IndexedDB and the GPU |
| EP-08 | transport shape stated; vectors equal through crate and guest | README states both transports and what each cannot reach; `model/wire.json`, 5 vectors, 35 encodings byte identical through the crate (`cargo test`) and through `core.wasm` (`tools/guest.mjs`) |

The official `openai` Python client, streaming: first token 16 ms and 18 ms total on a repeated prompt, the receipt in the last chunk; plain: 24 ms, `x-hologram-reuse: 1`, `id` from the receipt, identical text.

What Hermes needed: version 0.15.1 no longer reads `OPENAI_BASE_URL`; the bare custom provider takes `CUSTOM_BASE_URL` or `model.base_url` in `config.yaml`, and any key. Its default prompt (15 KB system prompt, 8.5 KB skills index, 38 KB of tool schemas, 3,573 tokens as sent) made BitNet 2B answer with a single `"`; with `--ignore-rules -t none` the prompt is 604 tokens and the answer is a sentence. That is the model's size, not the endpoint's shape; it is recorded because it is what a user will meet first.

The relay's stream headers come from the page (`/tab/{id}/head` before the first frame), so a memo hit through the relay says `x-hologram-stream: memo` and carries its receipt like the page origin does.

Lines: model, 26 declarations added by 71 lines of `tools/author.py`; generated, `freeinference_core.rs` 132 → 330 lines; handwritten adapters, `site/app.js` +114 lines (endpoint, worker message handler, relay loop), `site/sw.template.js` +28, `relay/freeinference-relay.py` 196, `core/src/abi.rs` +26, `core/src/bin/project_site.rs` +56 (the OpenAPI document). Deleted: nothing yet; the daemon's `openai.rs` and worker bridge are the fallback and are superseded, see "Not built".

## Measured, the switch, 2026-09-10

| Case | Result |
| --- | --- |
| CORS from the page origin to `openrouter.ai` | an authenticated `GET /api/v1/models` from `http://localhost:8090` returned 200 (436 models), so the paid path runs from the browser with no server |
| Paid first answer in the page, `qwen/qwen3.8-flash` | 994 ms total, first token 264 ms, $0.00006, provider Makora, sealed with id, provider, usage and cost |
| Repeated paid request through the endpoint | 4 ms plain, 3 ms streamed, `x-hologram-reuse: 1`, `x-hologram-provider: openrouter`, the receipt in the header and the last chunk |
| Ten prompts local, through the endpoint | first token 378 ms average, 795 ms total, repeats 10 ms average, all served from the seal |
| Ten prompts paid, same | first token 834 ms average, 1,612 ms total, repeats 4 ms; four of ten came back empty because one provider ignored `reasoning.enabled=false` and spent the budget on reasoning; the encoder now asks for providers that honor every field (`provider.require_parameters`) and the adapter refuses to seal an empty answer |
| No key | the page shows the View's word; the endpoint answers 401 with the model's error envelope |
| Hermes 0.15.1 through the relay with the switch on paid | 26.0 s first turn, 10.7 s repeated, both dominated by Hermes's own 8.6 s startup |
| The key | never in a file: CI greps the tree for `sk-or-v1-` and refuses; the key lives in IndexedDB under the page origin |

Words: the switch, the key field, the paid model names, the cost chip and every refusal are View strings; the projector writes them.

## Measured, the κ object, 2026-09-10

`HOLOGRAM/tools/kappa_object.py` streams each shard once and hashes as it arrives, nothing stored, so a model of any size is addressed with no disk. Measured on `Edge0/Edge0-8B-A1B-preview` (4.6 GB, three shards) and on `Qwen/Qwen3.8-Flash-Next` (360 GB, 131 shards, running at the time of writing):

| Step | Result |
| --- | --- |
| edge0-8b, off the wire | 1,550 tensor κ and 26,496 expert page κ in 162 s at 28 MB/s (the wire, not the hash: BLAKE3 ran at 3.17 GB/s on a stored shard) |
| Root | `blake3:2f62cfb5…` over a 1,052 byte manifest naming three object list κs; the object lists hold 28,046 lines |
| The rule in the model | `rootPreimage` of the real manifest equals the Python restatement byte for byte through the crate and through `core.wasm`; every one of the 28,046 object lines equals the generated `objEntry`; `admitPage` admits a listed κ that derives, refuses a listed κ that does not and an unlisted one |
| Page arithmetic | `expertPage` tiles 4 experts of 80 bytes at stride 20; `tablePage` tiles a 20 byte tensor into 6, 6, 6, 2; both refuse an overflowing offset (`Result`, `AddOverflow`, `MulOverflow`) instead of wrapping |
| Qwen3.8-Flash-Next | 19 of 131 shards at 15 to 40 MB/s per stream, four streams, 156,256 κ per n gram table shard (320 KiB pages), when this was written; the root will be recorded when the run completes |
| This GPU through WebGPU | a 1 GiB storage buffer read in 6 ms, 195 GB/s, against the Q engine's 25 to 30 GB/s effective on BitNet: the spine kernel headroom |

## Planted defect

The params key order swapped in the model (`temperature` before `seed` in `tools/author.py`, regenerated, lane run with `LANE_WRITE=1`):

- `lexlean verify`: passed, attestation `638bd0ab…`. Lean proves the rule as written; a wrong rule is consistent with itself.
- The lane: green, the swapped generated core compiles.
- `tools/corpus.py`: refused on the first case:

```
params bytes differ on case single user
  left:  "{\"max_tokens\":null,\"temperature\":null,\"seed\":null}"
  right: "{\"max_tokens\":null,\"seed\":null,\"temperature\":null}"
```

Restored; verify attestation `b28b2144…`, lane green, corpus 16 of 16, compare mode matches all four artifacts.

The restore exposed a second defect, in the lane script itself: it selected the LexLean build directory by newest modification time, so the restored model, whose content addressed build directory already existed from before the defect, exported the defective sibling's module and the corpus kept failing after the restore. The lane now takes the build id `lexlean build` reports. Recorded here because a lane that can pick the wrong input is a lane whose green means nothing.

### The wire encoder, 2026-09-10

`scripts/plant.sh`: `created` and `model` swapped in the completion head of `tools/author.py`, regenerated, lane run with `LANE_WRITE=1`.

- `lexlean verify`: passed, attestation `2a177b39…`. The theorem `encodeCompletion_shape` is stated through the same helper as the definition, so Lean proves the swapped rule as written.
- The lane: green; the swapped encoder compiles and both generations agree.
- `tools/corpus.py`: refused, `wire_bytes_match_the_vectors_on_every_case` failed on the first vector (`completion bytes differ on plain`), while the 16 request cases still passed.

Restored; verify attestation `be3fdf46…`, lane green, corpus 16 of 16 and 35 of 35 through the crate and the guest. The run also showed the lane printing the attestation path by newest modification time, the same defect the build directory had; the lane now takes the id `lexlean verify` reports.

### The switch and the κ object, 2026-09-10

`scripts/plant-openrouter.sh` and `scripts/plant-address.sh`:

- OpenRouter request, `model` and `messages` swapped in `encodeOpenRouterRequest`: `lexlean verify` passed (attestation `4167c49b…`, the theorem is stated through the same helper), the lane stayed green, and the corpus refused: `openrouter_request_bytes_match_on_every_case` failed on the first case through the crate and the guest.
- The route, paid without a key runs anyway (`Route.NoKey` replaced by `Route.Paid`): Lean refused, `rfl` failed on the route theorems (`route_noKey`, `route_noKeyOffline`); nothing was generated.
- The κ object, `expertPage` one byte late: `lexlean verify` passed, the lane stayed green, and the corpus refused: `expert_and_table_pages_tile_their_tensors` failed, and with it the real manifest's page vectors through the guest.

All three restored; verify attestation `d559b851…`, lane green, corpus 9 of 9 through the crate and every vector through the guest. The first run of `plant-openrouter.sh` aborted between its two defects and left the swapped encoder in the working tree, which the address run then caught as a second failing test: the scripts no longer stop on a failing gate before the restore.

## Findings about the tools, for upstream

All from this model; none required editing generated code.

1. lean4-prod's exporter requires roots strictly ASCII sorted (`roots are not strictly sorted`), and a Lean name with a trailing carriage return is `invalid Lean name`; `model/roots.txt` is LF only and sorted.
2. lean4-prod's exporter refuses a closure it cannot lower (`opaque lowering ... [_f_6-closure]`) when a `match` sits inside a list literal; hoisting the match into its own definition resolves it.
3. lean4-prod's generator writes list results into caller owned buffers, so a list returning definition cannot be an intermediate value (`UnsupportedList`); the transcript is folded without one.
4. lean4-prod's generator types a string parameter as owned unless the definition returns a boolean (`borrowed_parameter` in `prod-codegen/src/lib.rs`), and passes a projected field borrowed without cloning, so such a definition cannot be called with a projection; definitions that need a field take the record and project inside, and an owned copy is made with `split_exact` and `join` on the same delimiter, which is the identity.
5. lean4-prod's generator prints integer literals untyped; a `uint32` literal above `i32::MAX` fails to compile in `usize::try_from(...)`. The split maximum is `2147483647`.
6. LexLean's `axioms` field is an exact policy and `verify` reports one violation per run; `tools/pin_axioms.py` loops until the pinned sets match what Lean observes.
7. LexLean's `\title` must be a lexicon phrase (`LLL1004`); the Calculator's `Boolean` is used.
8. lean4-prod's generated recursion is a descent per element: a structurally recursive string builder over a list clones its accumulator and recurses once per element, so a real object list (28,046 lines for edge0-8b, 156,256 per n gram shard for Qwen3.8) overflowed the host stack and the wasm stack, both in accumulator form and in nested form. The model therefore names each shard's object list by its κ and spells one object as one line (`objEntry`); the join is the adapter's, the hash is the adapter's, and the corpus checks every line. A list map into a list is also refused (`UnsupportedList`, finding 3), so there is no list level encoder to reach for.
9. LexLean's unchecked `multiply` and `quotient` require `nat` or `int` operands, while `format_decimal` requires a fixed integer; the page arithmetic runs on `nat` (lowered to checked `u64`, `Result` on overflow) and the manifest's counts on `uint64`. `add` is a term kind, not a primitive; `ble` is the boolean order.
10. Names that are Lean keywords break the generated module (`end` as a field, `admit` as a definition) and `name` clashes with the author helper's own parameter; the fields are `stop`, `admitPage` and `label`.

## Not built

- The request decoder is not in the model: `site/app.js` parses the request with `JSON.parse` and maps the fields the studied harnesses send (`messages` with string or part list content, `max_tokens` or `max_completion_tokens`, `temperature`, `seed`, `stream`). A JSON decoder over LexLean's string primitives is expressible as structurally recursive definitions; it is the next model step, and EP-01 is only half met until then.
- Tool calls are not answered: Hermes and OpenClaw send `tools`; the endpoint ignores them and returns text, which both accept as a plain answer.
- The daemon (`github.com/humuhumu33/freeinference`) and its worker bridge still exist as the fallback; the relay supersedes the bridge and should replace it.
- The κ object's rules live in the one `Freeinference` module. One LexLean module per capability (`Address`, `Pool`, `Stage`, `Pack`, `Ladder`, `Loader`) needs cross module names, which LexLean resolves through lexicon package glossaries the lane does not yet produce; the split is the next lane step and is filed in `UPSTREAM.md`.
- `Pool`, `Stage`, `Pack`, `Ladder` and `Loader` are not in the model yet: the κ object's addressing rule is, and it is the one the streaming loader will admit pages by.
- The Qwen3.8-Flash-Next root, the expert sliced `.holo`, the prerouter port and the kernels the engine lacks are the work `HOLOGRAM-QWEN38-KAPPA-NATIVE-PROMPT.md` names, not done here.
- Not a PrismPM application: see README, "What PrismPM itself does and does not do here". `prismpm check` refuses this model with `facet closure is not exact`, which is the correct answer from a tool that today projects one View family.
- Receipt canonical bytes, JCS and `did:holo` are not in the model yet; they need byte level length prefixes and key sorting, both expressible, not yet written.
