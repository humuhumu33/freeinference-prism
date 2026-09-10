#!/bin/bash
# Plant a defect in the wire encoder, run the lane, show the corpus refusing while Lean stays
# green, restore, run the lane again, show the corpus green. Run from the repo root on Linux.
#
#   ./scripts/plant.sh
#
# The defect: the `created` and `model` fields swap places in the completion head. The theorem
# `encodeCompletion_shape` is stated through the same helper, so Lean proves the swapped rule as
# written; only the byte vectors in model/wire.json can catch it.
set -uo pipefail  # a failing gate is the point; never stop before the restore
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
A=tools/author.py
GOOD='s(\x27,"object":"chat.completion","created":\x27), call("createdOf", c), s(\x27,"model":\x27), q(call("modelOf", c))'
BAD='s(\x27,"object":"chat.completion","model":\x27), q(call("modelOf", c)), s(\x27,"created":\x27), call("createdOf", c)'
grep -qF "$(printf "$GOOD")" "$A" || { echo "plant: the completion head is not in its expected shape" >&2; exit 1; }
python3 - "$A" "$(printf "$GOOD")" "$(printf "$BAD")" <<'EOF'
import sys, pathlib
p = pathlib.Path(sys.argv[1]); s = p.read_text(encoding="utf-8")
assert s.count(sys.argv[2]) == 1
p.write_text(s.replace(sys.argv[2], sys.argv[3]), encoding="utf-8", newline="\n")
EOF
echo "== planted: created and model swapped in encodeCompletion"
python3 tools/author.py
LANE_WRITE=1 ./scripts/lane.sh 2>&1 | grep -E "attestation|byte identical|lane:|verify" | tail -5 || true
echo "== corpus on the planted model (expected: refuse)"
if python3 tools/corpus.py 2>&1 | tail -12; then echo "corpus PASSED on a planted defect: the gate is blind" >&2; RESTORE_ONLY=1; else echo "== corpus refused, as it must"; fi
echo "== restoring"
python3 - "$A" "$(printf "$BAD")" "$(printf "$GOOD")" <<'EOF'
import sys, pathlib
p = pathlib.Path(sys.argv[1]); s = p.read_text(encoding="utf-8")
assert s.count(sys.argv[2]) == 1
p.write_text(s.replace(sys.argv[2], sys.argv[3]), encoding="utf-8", newline="\n")
EOF
python3 tools/author.py
LANE_WRITE=1 ./scripts/lane.sh 2>&1 | grep -E "attestation|byte identical|lane:|verify" | tail -5 || true
python3 tools/corpus.py 2>&1 | tail -4
[ -z "${RESTORE_ONLY:-}" ] || exit 1
echo "== restored, corpus green"
