#!/bin/bash
# Two planted defects for the OpenRouter step, each restored afterwards. Run from the repo root on Linux.
#
#   ./scripts/plant-openrouter.sh
#
# 1. The request encoder: `model` and `messages` swap places. The theorem is stated through the same
#    helper, so Lean stays green; only the corpus bytes catch it.
# 2. The route table: paid without a key runs anyway. Lean catches it: `route_noKey` no longer holds.
set -uo pipefail  # a failing gate is the point; never stop before the restore
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="$HOME/.cache/freeinference-prism-lane/PrismPM/target/release:$HOME/.elan/bin:$PATH"
A=tools/author.py
swap() { python3 - "$A" "$1" "$2" <<'EOF'
import sys, pathlib
p = pathlib.Path(sys.argv[1]); s = p.read_text(encoding="utf-8")
assert s.count(sys.argv[2]) == 1, sys.argv[2]
p.write_text(s.replace(sys.argv[2], sys.argv[3]), encoding="utf-8", newline="\n")
EOF
}
GOOD1="s('{\"model\":'), q(var(model) if isinstance(model, str) else model), s(',\"messages\":['), call(\"orMessages\", project(\"messages\", req)), s(\"]\"),"
BAD1="s('{\"messages\":['), call(\"orMessages\", project(\"messages\", req)), s('],\"model\":'), q(var(model) if isinstance(model, str) else model),"
GOOD2="branch(\"Provider.Paid\", [], if_(key, if_(online, ctor(\"Route.Paid\"), ctor(\"Route.PaidOffline\")), ctor(\"Route.NoKey\")))))"
BAD2="branch(\"Provider.Paid\", [], if_(key, if_(online, ctor(\"Route.Paid\"), ctor(\"Route.PaidOffline\")), ctor(\"Route.Paid\")))))"

echo "== defect 1: model and messages swapped in encodeOpenRouterRequest"
swap "$GOOD1" "$BAD1"; python3 tools/author.py >/dev/null
LANE_WRITE=1 ./scripts/lane.sh 2>&1 | grep -E "verified 1 module|lane:" | tail -2
echo "-- corpus (expected: refuse)"
if python3 tools/corpus.py 2>&1 | grep -E "differ|FAILED|test result" | head -4; then :; fi
python3 tools/corpus.py >/dev/null 2>&1 && { echo "corpus PASSED on defect 1: the gate is blind" >&2; BLIND=1; } || echo "-- corpus refused defect 1"
swap "$BAD1" "$GOOD1"; python3 tools/author.py >/dev/null

echo "== defect 2: paid without a key runs anyway"
swap "$GOOD2" "$BAD2"; python3 tools/author.py >/dev/null
echo "-- lexlean verify (expected: refuse at route_noKey)"
if "$HOME/.cache/freeinference-prism-lane/PrismPM/target/release/lexlean" verify 2>&1 | grep -E "route_noKey|error|verified" | head -3; then :; fi
"$HOME/.cache/freeinference-prism-lane/PrismPM/target/release/lexlean" verify >/dev/null 2>&1 && { echo "verify PASSED on defect 2: the theorem is blind" >&2; BLIND=1; } || echo "-- Lean refused defect 2"
swap "$BAD2" "$GOOD2"; python3 tools/author.py >/dev/null

echo "== restored"
LANE_WRITE=1 ./scripts/lane.sh 2>&1 | grep -E "verified 1 module|lane:" | tail -2
python3 tools/corpus.py 2>&1 | grep -E "guest: 5|FAILED"
[ -z "${BLIND:-}" ] || exit 1
echo "== both defects caught, model restored, corpus green"
