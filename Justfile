set shell := ["bash", "-c"]

default:
    @just --list

# Rewrite src/Freeinference.lex.tex from tools/author.py (the .lex.tex stays the authority).
author:
    python3 tools/author.py

# Static validity of the model through LexLean's linked IR.
check:
    "$HOME/.cache/freeinference-prism-lane/PrismPM/target/release/lexlean" check

# Lean 4.32.1 proof check with leanchecker replay and exact axiom policy.
verify:
    "$HOME/.cache/freeinference-prism-lane/PrismPM/target/release/lexlean" verify

# The whole lane: LexLean, lean4-prod extraction twice, Rust generation twice, host and wasm32 builds, byte compare.
lane:
    ./scripts/lane.sh

# Refresh generated/ from a reviewed lane run.
lane-write:
    LANE_WRITE=1 ./scripts/lane.sh

# Byte identity of the generated core against the handwritten daemon, on the request corpus.
corpus:
    python3 tools/corpus.py

# The only definition of green.
vv: lane corpus
    @echo "vv: green"
