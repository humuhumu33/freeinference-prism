#!/bin/bash
# A planted defect in the κ object's addressing rule: `expertPage` starts one byte late (an off by
# one that Lean proves as written, because the theorem is stated through the same helper); the
# corpus refuses it through the crate and the guest. Restored afterwards. Run from the repo root on Linux.
#
#   ./scripts/plant-address.sh
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
GOOD='    return record("Range", start=add(start, mul(expert, call("stride", length, experts))),'
BAD='    return record("Range", start=add(add(start, nat(1)), mul(expert, call("stride", length, experts))),'
echo "== planted: expertPage starts one byte late"
swap "$GOOD" "$BAD"; python3 tools/author.py >/dev/null
LANE_WRITE=1 ./scripts/lane.sh 2>&1 | grep -E "verified 1 module|lane:" | tail -2
echo "-- corpus (expected: refuse)"
python3 tools/corpus.py 2>&1 | grep -E "expert_and_table|panicked|guest: expert|FAILED" | head -4 || true
if python3 tools/corpus.py >/dev/null 2>&1; then echo "corpus PASSED on the planted defect: the gate is blind" >&2; BLIND=1; else echo "-- corpus refused the planted defect"; fi
swap "$BAD" "$GOOD"; python3 tools/author.py >/dev/null
echo "== restored"
LANE_WRITE=1 ./scripts/lane.sh 2>&1 | grep -E "verified 1 module|lane:" | tail -2
python3 tools/corpus.py 2>&1 | grep -E "guest: kappa|FAILED"
[ -z "${BLIND:-}" ] || exit 1
echo "== defect caught, model restored, corpus green"
