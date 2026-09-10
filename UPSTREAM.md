# What this repo asks of PrismPM

Measured against `UOR-Foundation/PrismPM` at `f57a697`. Everything below is stated with the file that decides it, so it can be checked in a minute.

## 1. A second application shape

`crates/prismpm/src/holo/application.rs` (`project_application`) accepts exactly one application record shape: `operationType`, `errorType`, `functionName`, and a `view` with `leftLabel`, `rightLabel`, `operationLabel`, `divisionError`, `overflowError`, an operations list and `initialOperation`. `application_build.rs` binds the browser adapter to `core_function(Operation, i64, i64)`. The generic machinery underneath, `entryRoot` as `dispatch(bytes) -> bytes` exported as `holo_run`, `acceptanceVectors` as request and response byte pairs, `requestMaximum`, `responseMaximum`, `guestAllocationMaximum`, the `.holo` v4 archive and its provenance extension, does not depend on that View at all.

Request: let an application root omit the View, or add one text View family: a request box, a response area, and a state line, bound to `dispatch`. With either, this model becomes a PrismPM application unchanged: `dispatch` is the OpenAI request bytes in, the answer or refusal bytes out, and the acceptance vectors are the corpus in `model/corpus.json`.

## 2. A non application model that is not an ISO facet model

`crates/prismpm/src/holo/projector.rs` (`facets`) refuses every non application snapshot whose lexicon closure is not exactly `prism.arch`, `prism.qual`, `prism.sec`. A verified library, which is what this repo is, has no path through `prismpm check`.

Request: accept a snapshot with no `prism.*` packages as a library model, and give it the LCNF export and the Cargo crate generation the application path already has.

## 3. lean4-prod generator gaps met by this model

None blocked the lane; each cost a reshaping of the model that a user should not have to know about. In `vendor/lean4-prod/rust/prod-codegen/src/lib.rs`:

- A string parameter is owned unless the definition returns a boolean (`borrowed_parameter`), and a projected field is passed borrowed without a clone, so `f(record.field)` does not compile for a string returning `f`. Cloning at the call site, or borrowing string parameters uniformly and cloning inside, would remove the "take the record and project inside" rule this model follows.
- A list returning definition cannot be an intermediate (`UnsupportedList`), which forbids `join(lines(xs), sep)`; a list valued local built by a callee into a fresh `Vec` would remove the fold this model uses instead.
- Integer literals are printed untyped, so a `uint32` literal above `i32::MAX` fails at `usize::try_from(2147483648)`; printing the representation's suffix would remove the `2147483647` ceiling this model uses.
- In `Prod/Export.lean`, a `match` nested inside a list literal lowers to a closure the exporter refuses (`opaque lowering`); hoisting is the workaround.

## 4. Two small things in LexLean

- `\title` must be a lexicon phrase (`LLL1004`), so every model in the wild is titled `Boolean`. A free title, or a documented convention, would help.
- `verify` reports one axiom policy violation per run; reporting all of them at once would turn `tools/pin_axioms.py` from a loop into one edit.

## 5. What the endpoint step adds to request 1 and 2

The wire encoders (`encodeCompletion`, the chunk encoders, `encodeError`, `encodeModels`) are exactly the response half of a PrismPM application root `dispatch(bytes) -> bytes`: byte strings out of a closed record, no View. Their acceptance vectors are in `model/wire.json` and are checked through the crate and through the Core-Wasm guest the same way the Calculator's vectors are. The day PrismPM accepts an application root without a View, or with a text View family, this model registers as one with no change to its declarations.

## 6. Iterative recursion and list maps in lean4-prod

Finding 8 in `VERIFICATION.md`: a structurally recursive definition over a list is generated as a call per element that clones its accumulator, so a list of tens of thousands of records cannot be folded in generated code (host and wasm stacks overflow). Request: tail recursion lowered to a loop, and a list map into a list (`UnsupportedList` today) at least for string results, so a model can spell a whole manifest and not only one line of it. Until then the object list is named by its κ and the join is the adapter's.

## 7. Cross module names for one module per capability

The Calculator and this repository are one module each. A model of several capabilities wants one `.lex.tex` per capability with names resolved across them; LexLean resolves names through lexicon package glossaries. Request: a documented path from a project's own modules to glossaries the other modules can `\useglossary`, or an import field in the semantic module, so `Address`, `Pool`, `Stage`, `Pack`, `Ladder` and `Loader` can be separate verified modules composed by one entrypoint.
