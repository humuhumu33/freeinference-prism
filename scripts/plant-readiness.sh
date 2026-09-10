#!/bin/bash
# A planted defect in the endpoint readiness rule: a kept key counts as ready even offline. The
# theorem endpointReady_paidOffline is stated on literal rows, so Lean itself refuses the model
# before any code is generated; the crate test and the guest would refuse the same row. Restored
# afterwards. Run from the repo root on Linux.
#
#   ./scripts/plant-readiness.sh
set -uo pipefail  # a failing gate is the point; never stop before the restore
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
WORK="${FREEINFERENCE_LANE_WORK:-$HOME/.cache/freeinference-prism-lane}"
export PATH="$WORK/PrismPM/target/release:$HOME/.elan/bin:$PATH"
A=tools/author.py
swap() { python3 - "$A" "$1" "$2" <<'EOF'
import sys, pathlib
p = pathlib.Path(sys.argv[1]); s = p.read_text(encoding="utf-8")
assert s.count(sys.argv[2]) == 1, sys.argv[2]
p.write_text(s.replace(sys.argv[2], sys.argv[3]), encoding="utf-8", newline="\n")
EOF
}
GOOD='    return if_(resident, b(True), if_(key, online, b(False)))'
BAD='    return if_(resident, b(True), if_(key, b(True), b(False)))'
echo "== planted: a kept key is ready even offline"
swap "$GOOD" "$BAD"; python3 tools/author.py >/dev/null
echo "-- lexlean verify (expected: refuse endpointReady_paidOffline)"
if lexlean verify 2>&1 | grep -E "endpointReady_paidOffline|error" | head -3; then :; fi
if lexlean verify >/dev/null 2>&1; then echo "verify PASSED on the planted defect: the gate is blind" >&2; BLIND=1; else echo "-- Lean refused the planted defect"; fi
swap "$BAD" "$GOOD"; python3 tools/author.py >/dev/null
echo "== restored"
lexlean verify 2>&1 | grep -E "verified|attestation" | tail -1
[ -z "${BLIND:-}" ] || exit 1
echo "== defect caught, model restored"
