"""Byte identity corpus: the generated core against the daemon's rules.

Writes model/corpus.json, a fixed set of requests with the exact prompt and
params bytes the freeinference daemon (github.com/humuhumu33/freeinference,
src/modules/openai.rs at 5c4ded7) keys its memos on, then runs the core
crate's corpus test, which feeds every request to the generated functions
and compares byte for byte. The expected values here are the daemon's rule
restated once more in Python; the test proves the generated Rust agrees
with it on every case, and the theorems in the model prove the rule's shape.

Run from the project root: python3 tools/corpus.py
"""
import json, pathlib, subprocess, sys

root = pathlib.Path(__file__).resolve().parent.parent

def render_prompt(messages):
    # openai.rs render_prompt: "role: content" lines joined by newline; array content joins its text parts by newline.
    lines = []
    for m in messages:
        role = m.get("role", "user")
        content = m.get("content", "")
        if isinstance(content, list):
            content = "\n".join(part.get("text", "") for part in content if "text" in part)
        lines.append(f"{role}: {content}")
    return "\n".join(lines)

def params_canonical(max_tokens, seed, temperature):
    # openai.rs: json!({"max_tokens", "temperature", "seed"}).to_string() with serde_json's sorted keys.
    # temperature is an f32 there; the model takes the spelled decimal string, so the corpus
    # uses spellings that are unambiguous under f32 Display.
    spell = lambda v: "null" if v is None else str(v)
    return '{"max_tokens":' + spell(max_tokens) + ',"seed":' + spell(seed) + ',"temperature":' + (temperature if temperature is not None else "null") + "}"

cases = []
OR_MODEL = "qwen/qwen3.8-flash"
def openrouter_bytes(model, messages, max_tokens, seed, temperature, stream):
    # encodeOpenRouterRequest: only these fields, in this order, absent when absent, never a key.
    J = lambda v: json.dumps(v, separators=(",", ":"), ensure_ascii=False)
    msgs = ",".join('{"role":' + J(m[0]) + ',"content":' + J(m[1]) + "}" for m in messages)
    return ('{"model":' + J(model) + ',"messages":[' + msgs + "]"
            + ("" if max_tokens is None else ',"max_tokens":' + str(max_tokens))
            + ("" if seed is None else ',"seed":' + str(seed))
            + ("" if temperature is None else ',"temperature":' + temperature)
            + ',"stream":' + ("true" if stream else "false") + ',"usage":{"include":true},"reasoning":{"enabled":false},"provider":{"require_parameters":true}}')
def case(name, messages, max_tokens=None, seed=None, temperature=None):
    cases.append({
        "name": name,
        "messages": [{"role": m[0], "content": m[1]} for m in messages],
        "max_tokens": max_tokens, "seed": seed, "temperature": temperature,
        "prompt": render_prompt([{"role": m[0], "content": m[1]} for m in messages]),
        "params": params_canonical(max_tokens, seed, temperature),
        "openrouter_model": OR_MODEL,
        "openrouter_stream": openrouter_bytes(OR_MODEL, messages, max_tokens, seed, temperature, True),
        "openrouter_plain": openrouter_bytes(OR_MODEL, messages, max_tokens, seed, temperature, False),
    })

case("single user", [("user", "What is the capital of France?")])
case("system and user", [("system", "Be brief."), ("user", "hello")], 64, 1, "0.7")
case("three turns", [("user", "a"), ("assistant", "b"), ("user", "c")], 512, None, "0.0")
case("empty content", [("user", "")])
case("unicode", [("user", "κ addressed, naïve café, 日本語, emoji 🙂")], 32, 0, "1.0")
case("newlines inside content", [("user", "line one\nline two\n"), ("assistant", "ok\n")], 16)
case("colon in content", [("user", "role: content: more")])
case("max seed", [("user", "x")], 4096, 18446744073709551615, "2.0")
case("temperature only", [("user", "x")], None, None, "0.7")
case("max tokens only", [("user", "x")], 1)
case("seed only", [("user", "x")], None, 7)
case("long transcript", [("user", f"turn {i}") if i % 2 == 0 else ("assistant", f"reply {i}") for i in range(16)], 256, 3, "0.5")
case("odd role", [("tool", "result"), ("user", "and?")])
case("whitespace content", [("user", "   "), ("assistant", "\t")])
case("quotes and braces", [("user", 'say "hi" {now}')], 8, 8, "0.7")
case("model like names in content", [("user", "webgpu:BitNet smollm2")], 100, 100, "0.1")

# The wire vectors: what the encoders in the model must emit, byte for byte, restated once in
# Python with json.dumps in the encoders' key order. Python escapes only what the model escapes
# on these inputs: backslash, quote, newline, return, tab.
J = lambda v: json.dumps(v, separators=(",", ":"), ensure_ascii=False)
def completion_bytes(c):
    return ('{"id":' + J(c["id"]) + ',"object":"chat.completion","created":' + c["created"] + ',"model":' + J(c["model"])
            + ',"system_fingerprint":' + J(c["fingerprint"]) + ',"choices":[{"index":0,"message":{"role":"assistant","content":' + J(c["text"])
            + ',"refusal":null},"logprobs":null,"finish_reason":"stop"}],"usage":null}')
def chunk_head(c):
    return '{"id":' + J(c["id"]) + ',"object":"chat.completion.chunk","created":' + c["created"] + ',"model":' + J(c["model"]) + ',"system_fingerprint":' + J(c["fingerprint"])
def role_bytes(c): return chunk_head(c) + ',"choices":[{"index":0,"delta":{"role":"assistant","content":""},"finish_reason":null}]}'
def delta_bytes(c, d): return chunk_head(c) + ',"choices":[{"index":0,"delta":{"content":' + J(d) + '},"finish_reason":null}]}'
def final_bytes(c): return chunk_head(c) + ',"choices":[{"index":0,"delta":{},"finish_reason":"stop"}],"hologram":{"receipt":' + J(c["receipt"]) + '}}'
def error_bytes(m, k): return '{"error":{"message":' + J(m) + ',"type":' + J(k) + ',"param":null,"code":null}}'
def models_bytes(ids): return '{"object":"list","data":[' + ",".join('{"id":' + J(i) + ',"object":"model","created":0,"owned_by":"browser"}' for i in ids) + "]}"

wire = []
def vector(name, completion, delta="", error=("", ""), ids=()):
    wire.append({"name": name, "completion": completion, "delta": delta, "error": {"message": error[0], "type": error[1]}, "ids": list(ids),
                 "completion_bytes": completion_bytes(completion), "role_bytes": role_bytes(completion), "delta_bytes": delta_bytes(completion, delta),
                 "final_bytes": final_bytes(completion), "error_bytes": error_bytes(*error), "models_bytes": models_bytes(ids), "done_bytes": "[DONE]"})
C = lambda **k: {"id": "chatcmpl-1", "created": "1789000000", "model": "webgpu:BitNet", "text": "", "fingerprint": "", "receipt": "", **k}
vector("plain", C(text="One planet is Earth.", fingerprint="did:holo:sha256:aa;did:holo:sha256:bb", receipt="blake3:cc"), "Earth", ("messages must not be empty", "invalid_request_error"), ["webgpu:BitNet"])
vector("empty", C(), "", ("", ""), [])
vector("escapes", C(id='q"uote', text='back\\slash "quoted"\nnew line\r\ttab', fingerprint="f;g", receipt="r"), 'say "hi"\n', ('a "b" \\ c\n', "server_error"), ["a", "b\"c", "d"])
vector("unicode", C(text="κ addressed, naïve café, 日本語, emoji 🙂", receipt="blake3:00"), "🙂", ("日本語", "invalid_request_error"), ["webgpu:BitNet", "webgpu:Qwen"])
vector("long", C(text="x" * 4000, created="2147483647"), "y" * 1000, ("z" * 300, "server_error"), ["m%d" % i for i in range(12)])
wire_path = root / "model" / "wire.json"
wire_path.write_text(json.dumps(wire, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"wrote {wire_path}: {len(wire)} wire vectors")

# The recorded OLMoE routing trace (HOLOGRAM/tools/olmoe-lab, 2026-09-11): 282 tokens, 128 routed pages per
# token, -1 marks a token. Replayed through an LRU pool whose admission is the model's poolAdmit table
# (present -> touch, space -> insert, else evict the oldest then insert); the hit counts per pool size are
# pinned here in Python and must be reproduced by the generated table through the crate and the guest.
from collections import OrderedDict
trace_path = root / "model" / "traces" / "olmoe-trace.json"
if trace_path.exists():
    trace = json.loads(trace_path.read_text(encoding="utf-8"))
    def replay(capacity):
        lru = OrderedDict(); hits = misses = 0
        for k in trace:
            if k == -1: continue
            present = k in lru; space = len(lru) < capacity
            if present: hits += 1; lru.move_to_end(k)            # Touch
            else:
                misses += 1
                if not space: lru.popitem(last=False)             # EvictThenInsert
                lru[k] = True                                     # Insert
        return {"capacity": capacity, "hits": hits, "misses": misses}
    expected = [replay(c) for c in (256, 512, 1024)]
    (root / "model" / "traces" / "olmoe-expected.json").write_text(json.dumps({"pages": 1024, "perToken": 128, "tokens": trace.count(-1), "pools": expected}, indent=1) + "\n", encoding="utf-8")
    print("olmoe trace: " + ", ".join(f"{e['capacity']} pages -> {e['hits']} hits / {e['misses']} misses" for e in expected))

# Context as κ: a 12 turn agent style session where every turn appends one or two blocks and turn 9
# rewrites its last block. Block preimages by the model's rule (canonical JSON, sorted keys), block κ
# by BLAKE3 of the preimage as the adapter derives it, hit lengths as the longest common prefix.
import hashlib
def kv_preimage(root, prefix, group, index):
    return '{"group":' + str(group) + ',"index":' + str(index) + ',"prefix":' + json.dumps(prefix) + ',"root":' + json.dumps(root) + "}"
def blake3_hex(text):
    try:
        import blake3; return "blake3:" + blake3.blake3(text.encode()).hexdigest()
    except ImportError:
        return "sha256:" + hashlib.sha256(text.encode()).hexdigest()   # the digest is the adapter's; the vector pins the preimage
ROOT = "blake3:7aca5963"; turns = []; path = []; prefix = "blake3:0"; every = 8
for turn in range(12):
    new = 2 if turn % 3 == 0 else 1
    if turn == 9: path = path[:-1]
    prompt = list(path)
    for _ in range(new):
        pre = kv_preimage(ROOT, prefix, 0, len(prompt)); k = blake3_hex(pre); prompt.append(k); prefix = k
    hit = 0
    for a, b_ in zip(path, prompt):
        if a == b_: hit += 1
        else: break
    turns.append({"turn": turn, "path": list(path), "prompt": prompt, "hit": hit, "preimage_last": kv_preimage(ROOT, prompt[-2] if len(prompt) > 1 else "blake3:0", 0, len(prompt) - 1), "checkpoint_due": (len(prompt) - 1) % every == 0, "replay": (len(prompt) - 1) % every})
    path = prompt
kv_dir = root / "model" / "kv"; kv_dir.mkdir(exist_ok=True)
(kv_dir / "session.json").write_text(json.dumps({"root": ROOT, "every": every, "turns": turns}, indent=1) + "\n", encoding="utf-8")
print(f"kv session: {len(turns)} turns, hits " + ",".join(str(t['hit']) for t in turns))

out = root / "model" / "corpus.json"
out.write_text(json.dumps(cases, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"wrote {out}: {len(cases)} cases")

if "--write-only" in sys.argv:
    sys.exit(0)

# The shell hash list the service worker precaches from must match hashlib over the files.
import hashlib
manifest_path = root / "site" / "manifest.json"
if manifest_path.exists():
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    bad = [f["path"] for f in manifest["files"] if hashlib.sha256((root / "site" / f["path"]).read_bytes()).hexdigest() != f["sha256"]]
    if bad:
        sys.exit(f"site/manifest.json digests differ from hashlib on: {bad}")
    print(f"site/manifest.json: {len(manifest['files'])} shell files, every SHA-256 matches hashlib; closure {manifest['closure'][:12]}")
run = subprocess.run(["cargo", "test", "--release", "-q", "--", "--nocapture"], cwd=root / "core")
if run.returncode:
    sys.exit(run.returncode)
# The same vectors through the wasm guest's ABI, as the page calls it.
import shutil
if shutil.which("node") and (root / "site" / "core.wasm").exists():
    guest = subprocess.run(["node", str(root / "tools" / "guest.mjs")], cwd=root)
    sys.exit(guest.returncode)
print("node or site/core.wasm missing: guest check skipped")
