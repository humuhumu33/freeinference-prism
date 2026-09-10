#!/bin/bash
# The full lane, replayed from the committed tree on Linux x86_64 (WSL or CI).
#
#   1. LexLean, the compiler PrismPM vendors, at the PrismPM commit this repo pins:
#      lock, check, build, verify the authored model under Lean 4.32.1
#      (leanchecker replay, exact per declaration axiom policy).
#   2. lean4-prod, also as PrismPM vendors it: export every definition root to
#      kernel LCNF twice (byte identical), then generate Rust twice (byte
#      identical) into generated/freeinference_core.rs.
#   3. Compile the generated Rust for the host and for wasm32-unknown-unknown.
#
# Needs: git, cargo (rustup), python3, and the elan toolchain
# leanprover/lean4:v4.32.1 (installed on demand; the only network use besides
# the first PrismPM clone).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="${FREEINFERENCE_LANE_WORK:-$HOME/.cache/freeinference-prism-lane}"
PRISMPM_REV="$(tr -d '\n' < "$ROOT/PRISMPM_REV")"
mkdir -p "$WORK"

# 0. PrismPM at the pinned commit, its vendored LexLean built once.
if [ ! -d "$WORK/PrismPM/.git" ]; then
  git clone -q https://github.com/UOR-Foundation/PrismPM.git "$WORK/PrismPM"
fi
git -C "$WORK/PrismPM" checkout -q "$PRISMPM_REV"
if [ ! -x "$WORK/PrismPM/target/release/lexlean" ]; then
  (cd "$WORK/PrismPM" && cargo build --release -q --locked --manifest-path vendor/lexlean/Cargo.toml --target-dir target)
fi
LX="$WORK/PrismPM/target/release/lexlean"

# 1. Lean toolchain.
export PATH="$HOME/.elan/bin:$PATH"
if ! elan toolchain list 2>/dev/null | grep -q 'leanprover/lean4:v4.32.1'; then
  command -v elan >/dev/null || {
    curl -sSfL https://elan.lean-lang.org/elan-init.sh | sh -s -- -y --default-toolchain none
    export PATH="$HOME/.elan/bin:$PATH"
  }
  elan toolchain install leanprover/lean4:v4.32.1
fi

# 2. LexLean over the committed source. `lock` must report current: the
#    committed lock is the authority, never rewritten here.
cd "$ROOT"
lake env lean --version >/dev/null 2>&1 || true
"$LX" lock | grep -q 'is current' || { echo "lexlean.lock is not current; run lexlean lock and commit it" >&2; exit 1; }
"$LX" check
# The build directory is content addressed; take the one this build reports, never the newest by
# time, or a restored model would export a stale sibling's module.
BUILD_ID=$("$LX" build | tee /dev/stderr | grep -o "build/[0-9a-f]*" | head -1 | cut -d/ -f2)
[ -n "$BUILD_ID" ] || { echo "lexlean build reported no build id" >&2; exit 1; }
"$LX" verify
ATTESTATION=$(ls -t "$ROOT"/.lexlean/verified/*/attestation.json | head -1)
echo "attestation: $ATTESTATION"

# 3. Export workspace: the generated Lean module beside vendored lean4-prod.
WS="$WORK/export-ws"
rm -rf "$WS"; mkdir -p "$WS/lean4-prod"
tar xf "$WORK/PrismPM/vendor/lean4-prod/lean.tar" -C "$WS/lean4-prod"
mkdir -p "$WS/PrismFreeinference"
cp "$ROOT/.lexlean/build/$BUILD_ID/modules/PrismFreeinference/Freeinference.lean" "$WS/PrismFreeinference/"
cat > "$WS/lakefile.toml" <<'EOF'
name = "freeinference_verify"
version = "0.1.0"

[[lean_lib]]
name = "PrismGenerated"
roots = ["PrismFreeinference.Freeinference"]
EOF
printf 'leanprover/lean4:v4.32.1\n' > "$WS/lean-toolchain"
cd "$WS"
lake build PrismGenerated
lake env leanchecker PrismFreeinference.Freeinference
cd "$WS/lean4-prod"
lake build prod-export
export LEAN_PATH="$WS/.lake/build/lib/lean"
ROOTS=""
for r in $(tr '\n' ' ' < "$ROOT/model/roots.txt"); do
  ROOTS="$ROOTS --root PrismFreeinference.Freeinference.$r"
done
# shellcheck disable=SC2086
lake exe prod-export --module PrismFreeinference.Freeinference $ROOTS --ir-module PrismFreeinference --out "$WS/export-a"
# shellcheck disable=SC2086
lake exe prod-export --module PrismFreeinference.Freeinference $ROOTS --ir-module PrismFreeinference --out "$WS/export-b"
cmp "$WS/export-a/kernel.ir" "$WS/export-b/kernel.ir"
cmp "$WS/export-a/roots.json" "$WS/export-b/roots.json"
cmp "$WS/export-a/coverage.json" "$WS/export-b/coverage.json"

# 4. Rust generation over the vendored prod crates, twice.
DRIVER="$WORK/codegen"
rm -rf "$DRIVER" "$WORK/prod-rust"; mkdir -p "$DRIVER/src"
cp -r "$WORK/PrismPM/vendor/lean4-prod/rust" "$WORK/prod-rust"
cat > "$DRIVER/Cargo.toml" <<'EOF'
[package]
name = "codegen-driver"
version = "0.1.0"
edition = "2021"

[dependencies]
prod-ir = { path = "../prod-rust/prod-ir" }
prod-codegen = { path = "../prod-rust/prod-codegen" }

[workspace]
EOF
cat > "$DRIVER/src/main.rs" <<'EOF'
fn main() {
    let path = std::env::args().nth(1).expect("kernel.ir path");
    let out = std::env::args().nth(2).expect("output path");
    let text = std::fs::read_to_string(&path).expect("read kernel.ir");
    let (rest, module) = prod_ir::parser::parse_module(&text).expect("parse kernel.ir");
    assert!(rest.trim().is_empty(), "trailing kernel.ir bytes");
    let a = prod_codegen::generate_module(&module).expect("generate a");
    let b = prod_codegen::generate_module(&module).expect("generate b");
    assert_eq!(a, b, "generation is not deterministic");
    std::fs::write(&out, &a).expect("write generated.rs");
}
EOF
(cd "$DRIVER" && cargo build --release -q)
"$DRIVER/target/release/codegen-driver" "$WS/export-a/kernel.ir" "$WS/generated.rs"

# 5. Compare or refresh the committed artifacts.
mkdir -p "$ROOT/generated"
compare() { cmp <(tr -d '\r' < "$1") <(tr -d '\r' < "$2") && echo "match: $2"; }
if [ "${LANE_WRITE:-0}" = "1" ]; then
  cp "$WS/export-a/kernel.ir" "$ROOT/generated/kernel.ir"
  cp "$WS/export-a/roots.json" "$ROOT/generated/roots.json"
  cp "$WS/export-a/coverage.json" "$ROOT/generated/coverage.json"
  cp "$WS/generated.rs" "$ROOT/generated/freeinference_core.rs"
  echo "wrote generated/ (review and commit)"
else
  compare "$WS/export-a/kernel.ir" "$ROOT/generated/kernel.ir"
  compare "$WS/export-a/roots.json" "$ROOT/generated/roots.json"
  compare "$WS/export-a/coverage.json" "$ROOT/generated/coverage.json"
  compare "$WS/generated.rs" "$ROOT/generated/freeinference_core.rs"
fi

# 6. The generated core for the host and for wasm32; the wasm is what the page loads.
cd "$ROOT/core"
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
cargo build --release -q
cargo build --release -q --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/freeinference_core.wasm "$ROOT/site/core.wasm"
ls -la "$ROOT/site/core.wasm"

# 7. The page is a projection of the verified View: index.html, the web manifest and the shell
#    hash list are written from view() by the projector. Compare mode refuses drift in the words.
cargo run --release -q --bin project-site
cd "$ROOT"
if [ "${LANE_WRITE:-0}" != "1" ]; then
  git diff --exit-code -- site/index.html site/app.webmanifest || { echo "site/index.html drifted from the projected View; run LANE_WRITE=1 ./scripts/lane.sh and commit" >&2; exit 1; }
fi
echo "lane: green"
