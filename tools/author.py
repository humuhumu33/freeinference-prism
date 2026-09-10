"""Authoring helper for src/Freeinference.lex.tex.

The authority is the .lex.tex file this script writes; LexLean and PrismPM read
only that. This script exists so the semantic module JSON
(lexlean/semantic-module/1, the dialect PrismPM's examples/Calculator uses) can
be written as short Python instead of by hand.
Run: python tools/author.py  (rewrites src/Freeinference.lex.tex).

Shapes the model keeps because of lean4-prod's Rust generator (findings, see
VERIFICATION.md): a list returning definition cannot be an intermediate; a
string parameter of a string returning definition is owned, and a projected
field arrives borrowed, so such definitions take the record and project
inside; an owned copy of a projected string is made by splitting on a
delimiter and joining with the same delimiter, which the generator renders as
an allocation and which is the identity.
"""
import json, pathlib

NL = "\n"

# Exact axiom sets Lean observes per declaration, pinned by tools/pin_axioms.py.
# LexLean treats the `axioms` field as an exact policy; anything else is refused.
_PINS = pathlib.Path(__file__).resolve().parent / "axioms.json"
AXIOMS = json.loads(_PINS.read_text(encoding="utf-8")) if _PINS.exists() else {}

# ---- types
STRING = {"kind": "string"}
BYTES = {"kind": "bytes"}
BOOL = {"kind": "bool"}
U64 = {"kind": "uint64"}
def named(n): return {"arguments": [], "kind": "named", "member": {"name": n}}
def lst(t): return {"element": t, "kind": "list"}
def opt(t): return {"kind": "option", "value": t}

# ---- expressions
def var(n): return {"kind": "var", "name": n}
def s(v): return {"kind": "string", "value": v}
def u64(v): return {"kind": "integer", "representation": "uint64", "value": str(v)}
def u32(v): return {"kind": "integer", "representation": "uint32", "value": str(v)}
def b(v): return {"kind": "bool", "value": v}
def call(f, *args): return {"arguments": list(args), "function": {"name": f}, "kind": "call"}
def prim(op, result, *args): return {"arguments": list(args), "kind": "primitive", "operation": op, "result": result}
def ctor(name, *args, targs=()):
    return {"arguments": list(args), "constructor": {"name": name}, "kind": "constructor", "type_arguments": list(targs)}
def project(field, value): return {"field": field, "kind": "project", "value": value}
def if_(cond, then, else_): return {"condition": cond, "kind": "if", "then_value": then, "else_value": else_}
def match(scrutinee, *branches): return {"branches": list(branches), "kind": "match", "scrutinee": scrutinee}
def branch(ctor_name, binders, body): return {"binders": binders, "body": body, "constructor": {"name": ctor_name}}
def record(type_name, **fields): return {"fields": [{"field": k, "value": v} for k, v in fields.items()], "kind": "record", "type": {"name": type_name}}
def cons(head, tail): return {"head": head, "kind": "cons", "tail": tail}
def nil(t): return {"element": t, "kind": "nil"}
def strings(*items):
    out = nil(STRING)
    for item in reversed(items): out = cons(item, out)
    return out
def join(list_expr, sep=""): return prim("join", STRING, list_expr, s(sep))
def equal(a, c): return prim("equal", BOOL, a, c)
def utf8(x): return prim("utf8_encode", BYTES, x)
def eq(l, r): return {"kind": "eq", "left": l, "right": r}
# An owned copy of a string expression: split on newline, join with newline. Identity on every
# string; rendered by the generator as split(...).map(String::from).collect() then join.
def owned(expr):
    return match(prim("split_exact", opt(lst(STRING)), expr, s(NL), u32(2147483647)),
                 branch("Option.none", [], s("")),
                 branch("Option.some", ["fields"], join(var("fields"), NL)))

# ---- declarations
def inductive(name, *ctors): return {"constructors": [{"fields": [], "name": c} for c in ctors], "kind": "inductive", "name": name, "parameters": [], "type_parameters": []}
def structure(name, **fields): return {"fields": [{"name": k, "type": v} for k, v in fields.items()], "kind": "structure", "name": name, "parameters": [], "type_parameters": []}
def definition(name, params, result, body, recursive=None):
    node = {"axioms": AXIOMS.get(name, []), "body": body, "kind": "definition", "name": name, "parameters": [{"name": k, "type": v} for k, v in params], "result": result}
    if recursive: node["recursive_argument"] = recursive
    return node
def theorem(name, statement, proof="reflexivity"): return {"axioms": AXIOMS.get(name, []), "kind": "theorem", "name": name, "parameters": [], "proof": {"kind": proof}, "statement": statement}

def message(role, content): return record("Message", role=s(role), content=s(content))
def request(messages, max_tokens=None, seed=None, temperature=""):
    return record("Request", model=s("webgpu:BitNet"), messages=messages,
                  maxTokens=ctor("Option.none", targs=[U64]) if max_tokens is None else ctor("Option.some", u64(max_tokens), targs=[U64]),
                  seed=ctor("Option.none", targs=[U64]) if seed is None else ctor("Option.some", u64(seed), targs=[U64]),
                  temperature=s(temperature))
def memo(*models): return record("Memo", model=strings(*[s(m) for m in models]), engineKappa=s("e"), promptKappa=s("p"), paramsKappa=s("q"), outputKappa=s("o"), receipt=s("r"))

# The exact params bytes for a request: keys sorted, nulls spelled null, temperature as the wire spells it
# (the empty string is absent). Written once here so the definition and its theorems share the term.
def params_body(req):
    return join(strings(
        s('{"max_tokens":'), call("optionalDecimal", project("maxTokens", req)),
        s(',"seed":'), call("optionalDecimal", project("seed", req)),
        s(',"temperature":'), call("temperatureText", req),
        s("}")))
def message_body(msg):
    return join(strings(call("roleOf", msg), s(": "), call("contentOf", msg)))

decls = [
    # The transcript one OpenAI request carries, as the daemon renders it.
    structure("Message", role=STRING, content=STRING),
    structure("Request", model=STRING, messages=lst(named("Message")), maxTokens=opt(U64), seed=opt(U64), temperature=STRING),
    # What a later request needs to find a sealed answer without executing.
    structure("Memo", model=lst(STRING), engineKappa=STRING, promptKappa=STRING, paramsKappa=STRING, outputKappa=STRING, receipt=STRING),
    # The bytes the hash adapter addresses; the model owns preimages, not digests.
    structure("Preimages", prompt=BYTES, params=BYTES),
    inductive("Decision", "Serve", "Execute", "Refuse"),
    # Every visible word of the page. The page is projected from this record by core/src/bin/project-site.rs;
    # no copy is written in HTML. Zero hyphens in any string, as the product's site rule requires.
    # A curated backdrop: an Unsplash photo vendored with the page, credited as the Unsplash License asks.
    structure("Wallpaper", file=STRING, label=STRING, author=STRING, authorUrl=STRING),
    structure("View", headline=STRING, lede=STRING, promptPlaceholder=STRING, sendLabel=STRING,
              loadingLabel=STRING, residentLabel=STRING, servedLabel=STRING, sealedLabel=STRING, rederiveLabel=STRING,
              identicalLabel=STRING, noGpuLabel=STRING, offlineLabel=STRING, repoLabel=STRING, repoUrl=STRING,
              modelLabel=STRING, appearanceLabel=STRING, darkLabel=STRING, lightLabel=STRING, immersiveLabel=STRING,
              photoLabel=STRING, byLabel=STRING, unsplashLabel=STRING, wallpapers=lst(named("Wallpaper"))),

    # Owned copies of projected strings live in their own record taking definitions: the generator
    # borrows a record parameter and returns an owned string, and a match nested inside a list literal
    # would lower to a closure the exporter refuses.
    definition("roleOf", [("message", named("Message"))], STRING, owned(project("role", var("message")))),
    definition("contentOf", [("message", named("Message"))], STRING, owned(project("content", var("message")))),
    definition("temperatureText", [("request", named("Request"))], STRING,
        if_(equal(project("temperature", var("request")), s("")), s("null"), owned(project("temperature", var("request"))))),
    definition("renderMessage", [("message", named("Message"))], STRING, message_body(var("message"))),
    # No intermediate list: the transcript is folded directly, one line per message, joined by newline.
    definition("renderPrompt", [("messages", lst(named("Message")))], STRING,
        match(var("messages"),
            branch("List.nil", [], s("")),
            branch("List.cons", ["message", "rest"],
                match(var("rest"),
                    branch("List.nil", [], call("renderMessage", var("message"))),
                    branch("List.cons", ["next", "more"], join(strings(call("renderMessage", var("message")), call("renderPrompt", var("rest"))), NL))))),
        recursive="messages"),
    definition("optionalDecimal", [("value", opt(U64))], STRING,
        match(var("value"),
            branch("Option.none", [], s("null")),
            branch("Option.some", ["number"], prim("format_decimal", STRING, var("number"))))),
    definition("paramsCanonical", [("request", named("Request"))], STRING, params_body(var("request"))),
    definition("preimages", [("request", named("Request"))], named("Preimages"),
        record("Preimages",
               prompt=utf8(call("renderPrompt", project("messages", var("request")))),
               params=utf8(call("paramsCanonical", var("request"))))),

    definition("anyEqual", [("candidates", lst(STRING)), ("model", STRING)], BOOL,
        match(var("candidates"),
            branch("List.nil", [], b(False)),
            branch("List.cons", ["candidate", "rest"], if_(equal(var("candidate"), var("model")), b(True), call("anyEqual", var("rest"), var("model"))))),
        recursive="candidates"),
    definition("intersects", [("models", lst(STRING)), ("candidates", lst(STRING))], BOOL,
        match(var("models"),
            branch("List.nil", [], b(False)),
            branch("List.cons", ["model", "rest"], if_(call("anyEqual", var("candidates"), var("model")), b(True), call("intersects", var("rest"), var("candidates"))))),
        recursive="models"),
    definition("memoMatches", [("memo", named("Memo")), ("candidates", lst(STRING)), ("promptKappa", STRING), ("paramsKappa", STRING)], BOOL,
        if_(equal(project("promptKappa", var("memo")), var("promptKappa")),
            if_(equal(project("paramsKappa", var("memo")), var("paramsKappa")),
                call("intersects", project("model", var("memo")), var("candidates")),
                b(False)),
            b(False))),
    definition("view", [], named("View"), record("View",
        headline=s("Free verified AI inference."),
        lede=s("Ask anything. Every answer is sealed on your device and can be checked again."),
        promptPlaceholder=s("Ask anything"),
        sendLabel=s("Ask"),
        loadingLabel=s("getting the model, once"),
        residentLabel=s("ready"),
        servedLabel=s("Instant, from the seal"),
        sealedLabel=s("Sealed"),
        rederiveLabel=s("Check again"),
        identicalLabel=s("Checked, identical"),
        noGpuLabel=s("This browser cannot run the model. Try Chrome or Edge on a computer."),
        offlineLabel=s("offline, working from your device"),
        repoLabel=s("How it is built"),
        repoUrl=s("https://github.com/humuhumu33/freeinference-prism"),
        modelLabel=s("BitNet 2B, on your device"),
        appearanceLabel=s("Appearance"),
        darkLabel=s("Dark"),
        lightLabel=s("Light"),
        immersiveLabel=s("Immersive"),
        photoLabel=s("Photo"),
        byLabel=s("by"),
        unsplashLabel=s("on Unsplash"),
        wallpapers=cons(record("Wallpaper", file=s("alps.jpg"), label=s("Alpine Dawn"), author=s("Unsplash"), authorUrl=s("https://unsplash.com/?utm_source=Hologram_AI&utm_medium=referral")),
                   cons(record("Wallpaper", file=s("galaxy.jpg"), label=s("Galaxy"), author=s("Tiago Ferreira"), authorUrl=s("https://unsplash.com/@tiago_f_ferreira?utm_source=Hologram_AI&utm_medium=referral")),
                   cons(record("Wallpaper", file=s("aurora.jpg"), label=s("Aurora"), author=s("Lightscape"), authorUrl=s("https://unsplash.com/@lightscape?utm_source=Hologram_AI&utm_medium=referral")),
                   nil(named("Wallpaper"))))))),
    definition("decide", [("hit", BOOL), ("workerAttached", BOOL)], named("Decision"),
        if_(var("hit"), ctor("Decision.Serve"), if_(var("workerAttached"), ctor("Decision.Execute"), ctor("Decision.Refuse")))),

    # Closed facts checked by Lean. String literals are not kernel reducible in Lean 4.32 (String is byte
    # based; rfl and decide both stall on them), so each theorem pins the RULE definitionally: what the
    # function unfolds to for a concrete input. The literal bytes are then pinned by the execution corpus.
    theorem("renderMessage_shape",
        eq(call("renderMessage", message("user", "hello")), message_body(message("user", "hello")))),
    theorem("renderPrompt_singleUser",
        eq(call("renderPrompt", cons(message("user", "hello"), nil(named("Message")))), call("renderMessage", message("user", "hello")))),
    theorem("renderPrompt_twoTurns",
        eq(call("renderPrompt", cons(message("user", "a"), cons(message("assistant", "b"), nil(named("Message"))))),
           join(strings(call("renderMessage", message("user", "a")), call("renderPrompt", cons(message("assistant", "b"), nil(named("Message"))))), NL))),
    theorem("paramsCanonical_empty",
        eq(call("paramsCanonical", request(nil(named("Message")))), params_body(request(nil(named("Message")))))),
    theorem("paramsCanonical_full",
        eq(call("paramsCanonical", request(nil(named("Message")), 32, 1, "0.0")), params_body(request(nil(named("Message")), 32, 1, "0.0")))),
    theorem("view_headline", eq(project("headline", call("view")), s("Free verified AI inference."))),
    theorem("decide_serve", eq(call("decide", b(True), b(False)), ctor("Decision.Serve"))),
    theorem("decide_execute", eq(call("decide", b(False), b(True)), ctor("Decision.Execute"))),
    theorem("decide_refuse", eq(call("decide", b(False), b(False)), ctor("Decision.Refuse"))),
    theorem("memoMatches_ownKey",
        eq(call("memoMatches", memo("webgpu:BitNet"), strings(s("webgpu:BitNet")), s("p"), s("q")), b(True)), proof="decide"),
    theorem("memoMatches_otherPrompt",
        eq(call("memoMatches", memo("webgpu:BitNet"), strings(s("webgpu:BitNet")), s("x"), s("q")), b(False)), proof="decide"),
    theorem("memoMatches_otherModel",
        eq(call("memoMatches", memo("webgpu:BitNet"), strings(s("smollm2")), s("p"), s("q")), b(False)), proof="decide"),
]

module = {"declarations": decls, "spec": "lexlean/semantic-module/1"}
text = ("\\begin{lexlean}{Freeinference}\n"
        "\\useglossary{lexlean.std.bool@1.1.0}\n"
        "\\useglossary{lexlean.std.nat@1.1.0}\n"
        "\\title{Boolean}\n"
        "\\begin{semanticmodule}\n"
        "\\semanticdata{" + json.dumps(module, separators=(",", ":"), sort_keys=True, ensure_ascii=False) + "}\n"
        "\\end{semanticmodule}\n"
        "\\end{lexlean}\n")
out = pathlib.Path(__file__).resolve().parent.parent / "src" / "Freeinference.lex.tex"
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(text, encoding="utf-8", newline="\n")
print(f"wrote {out} ({len(text)} bytes, {len(decls)} declarations)")
