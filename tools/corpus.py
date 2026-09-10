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
def case(name, messages, max_tokens=None, seed=None, temperature=None):
    cases.append({
        "name": name,
        "messages": [{"role": m[0], "content": m[1]} for m in messages],
        "max_tokens": max_tokens, "seed": seed, "temperature": temperature,
        "prompt": render_prompt([{"role": m[0], "content": m[1]} for m in messages]),
        "params": params_canonical(max_tokens, seed, temperature),
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

out = root / "model" / "corpus.json"
out.write_text(json.dumps(cases, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"wrote {out}: {len(cases)} cases")

if "--write-only" in sys.argv:
    sys.exit(0)
run = subprocess.run(["cargo", "test", "--release", "-q", "--", "--nocapture"], cwd=root / "core")
sys.exit(run.returncode)
