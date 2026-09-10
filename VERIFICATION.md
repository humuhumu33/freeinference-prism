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

## Findings about the tools, for upstream

All from this model; none required editing generated code.

1. lean4-prod's exporter requires roots strictly ASCII sorted (`roots are not strictly sorted`), and a Lean name with a trailing carriage return is `invalid Lean name`; `model/roots.txt` is LF only and sorted.
2. lean4-prod's exporter refuses a closure it cannot lower (`opaque lowering ... [_f_6-closure]`) when a `match` sits inside a list literal; hoisting the match into its own definition resolves it.
3. lean4-prod's generator writes list results into caller owned buffers, so a list returning definition cannot be an intermediate value (`UnsupportedList`); the transcript is folded without one.
4. lean4-prod's generator types a string parameter as owned unless the definition returns a boolean (`borrowed_parameter` in `prod-codegen/src/lib.rs`), and passes a projected field borrowed without cloning, so such a definition cannot be called with a projection; definitions that need a field take the record and project inside, and an owned copy is made with `split_exact` and `join` on the same delimiter, which is the identity.
5. lean4-prod's generator prints integer literals untyped; a `uint32` literal above `i32::MAX` fails to compile in `usize::try_from(...)`. The split maximum is `2147483647`.
6. LexLean's `axioms` field is an exact policy and `verify` reports one violation per run; `tools/pin_axioms.py` loops until the pinned sets match what Lean observes.
7. LexLean's `\title` must be a lexicon phrase (`LLL1004`); the Calculator's `Boolean` is used.

## Not built

- No guest ABI: the wasm has no exports, so nothing calls it yet. The daemon and the Playground still run their handwritten copies of these rules; replacing them with the generated crate and a wasm adapter is the next step, and the corpus is the gate for it.
- Not a PrismPM application: see README, "What PrismPM itself does and does not do here". `prismpm check` refuses this model with `facet closure is not exact`, which is the correct answer from a tool that today projects one View family.
- Receipt canonical bytes, JCS and `did:holo` are not in the model yet; they need byte level length prefixes and key sorting, both expressible, not yet written.
