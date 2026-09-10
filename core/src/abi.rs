//! The browser boundary: a host transport adapter, not the model.
//!
//! `holo_alloc`, `holo_free` and `holo_run` are the guest shape PrismPM's
//! Core-Wasm guests export. Requests and responses are UTF-8 JSON; the
//! adapter only decodes the request into the generated `Request`, calls the
//! generated core, and encodes what came back. No rule lives here.
//!
//! Operations:
//!   {"op":"preimages","request":{model,messages:[{role,content}],max_tokens?,seed?,temperature?}}
//!     -> {"prompt":"<bytes as text>","params":"<bytes as text>"}
//!   {"op":"decide","hit":bool,"attached":bool} -> {"decision":"Serve"|"Execute"|"Refuse"}
//!   {"op":"view"} -> the View record as JSON
//! Errors: {"error":"..."}.

use crate::{
    admitPage, decide, done, encodeCompletion, encodeDelta, encodeError, encodeFinal, encodeModels,
    encodeOpenRouterRequest, encodeRole, endpointReady, expertPage, objEntry, preimages, rootPreimage, route, tablePage, view, Completion,
    Decision, Manifest, Message, Obj, Provider, Request, Route, Shard,
};
use serde_json::{json, Value};

fn request(value: &Value) -> Result<Request, String> {
    let messages = value["messages"]
        .as_array()
        .ok_or("messages must be an array")?
        .iter()
        .map(|m| Message {
            role: m["role"].as_str().unwrap_or("user").to_owned(),
            content: match &m["content"] {
                Value::String(text) => text.clone(),
                Value::Array(parts) => parts
                    .iter()
                    .filter_map(|part| part["text"].as_str())
                    .collect::<Vec<_>>()
                    .join("\n"),
                _ => String::new(),
            },
        })
        .collect();
    let temperature = match &value["temperature"] {
        Value::Null => String::new(),
        Value::String(text) => text.clone(),
        // The wire spelling is the adapter's job: what the daemon's f32 Display prints.
        other => other
            .as_f64()
            .map(|f| format!("{}", f as f32))
            .ok_or("temperature must be a number or a decimal string")?,
    };
    Ok(Request {
        model: value["model"].as_str().unwrap_or("").to_owned(),
        messages,
        maxTokens: value["max_tokens"].as_u64(),
        seed: value["seed"].as_u64(),
        temperature,
    })
}

fn completion(value: &Value) -> Completion {
    let text = |key: &str| value[key].as_str().unwrap_or("").to_owned();
    Completion {
        id: text("id"),
        created: match &value["created"] {
            Value::Number(n) => n.to_string(),
            other => other.as_str().unwrap_or("0").to_owned(),
        },
        model: text("model"),
        text: text("text"),
        fingerprint: text("fingerprint"),
        receipt: text("receipt"),
    }
}

fn u(value: &Value) -> u64 {
    value.as_u64().unwrap_or(0)
}

fn text(value: &Value) -> String {
    value.as_str().unwrap_or("").to_owned()
}

fn manifest(value: &Value) -> Manifest {
    Manifest {
        spec: text(&value["spec"]),
        repo: text(&value["repo"]),
        revision: text(&value["revision"]),
        experts: u(&value["experts"]),
        tableRows: u(&value["table_rows"]),
        shards: value["shards"].as_array().map(|shards| shards.iter().map(|sh| Shard {
            label: text(&sh["name"]),
            bytes: u(&sh["bytes"]),
            sha256: text(&sh["sha256"]),
            kappa: text(&sh["kappa"]),
            objects: text(&sh["objects"]),
        }).collect()).unwrap_or_default(),
    }
}

fn run(input: &[u8]) -> Value {
    let value: Value = match serde_json::from_slice(input) {
        Ok(value) => value,
        Err(error) => return json!({ "error": format!("request is not JSON: {error}") }),
    };
    match value["op"].as_str().unwrap_or("") {
        "preimages" => match request(&value["request"]) {
            Ok(request) => {
                let out = preimages(&request);
                json!({
                    "prompt": String::from_utf8_lossy(&out.prompt),
                    "params": String::from_utf8_lossy(&out.params),
                })
            }
            Err(error) => json!({ "error": error }),
        },
        "decide" => {
            let decision = decide(
                value["hit"].as_bool().unwrap_or(false),
                value["attached"].as_bool().unwrap_or(false),
            );
            json!({ "decision": match decision {
                Decision::Serve => "Serve",
                Decision::Execute => "Execute",
                Decision::Refuse => "Refuse",
            } })
        }
        "view" => view_json(),
        // The wire: every response byte comes from the generated encoders.
        "encode-completion" => json!({ "bytes": encodeCompletion(&completion(&value["completion"])) }),
        "encode-role" => json!({ "bytes": encodeRole(&completion(&value["completion"])) }),
        "encode-delta" => json!({ "bytes": encodeDelta(&completion(&value["completion"]), value["delta"].as_str().unwrap_or("").to_owned()) }),
        "encode-final" => json!({ "bytes": encodeFinal(&completion(&value["completion"])) }),
        "encode-error" => json!({ "bytes": encodeError(value["message"].as_str().unwrap_or("").to_owned(), value["type"].as_str().unwrap_or("server_error").to_owned()) }),
        "encode-models" => json!({ "bytes": encodeModels(&value["ids"].as_array().map(|ids| ids.iter().filter_map(|i| i.as_str().map(str::to_owned)).collect::<Vec<_>>()).unwrap_or_default()) }),
        "done" => json!({ "bytes": done() }),
        // The κ object: page ranges, the root preimage and the admission rule come from the model.
        // The page arithmetic is checked: an overflow is a refusal, never a wrapped address.
        "expert-page" => match expertPage(u(&value["start"]), u(&value["length"]), u(&value["experts"]), u(&value["expert"])) {
            Ok(r) => json!({ "start": r.start, "end": r.stop }),
            Err(e) => json!({ "error": format!("{e:?}") }),
        },
        "table-page" => match tablePage(u(&value["start"]), u(&value["end"]), u(&value["rowBytes"]), u(&value["rows"]), u(&value["index"])) {
            Ok(r) => json!({ "start": r.start, "end": r.stop }),
            Err(e) => json!({ "error": format!("{e:?}") }),
        },
        "root-preimage" => json!({ "bytes": rootPreimage(&manifest(&value["manifest"])) }),
        "object-line" => json!({ "bytes": objEntry(&Obj { kind: text(&value["kind"]), label: text(&value["name"]), kappa: text(&value["kappa"]), bytes: u(&value["bytes"]) }) }),
        "admit" => json!({ "admit": admitPage(&value["listed"].as_array().map(|l| l.iter().filter_map(|k| k.as_str().map(str::to_owned)).collect::<Vec<_>>()).unwrap_or_default(), value["kappa"].as_str().unwrap_or(""), value["derived"].as_str().unwrap_or("")) }),
        // Who answers: the route table in the model, every row a theorem.
        "route" => {
            let provider = if value["provider"].as_str() == Some("paid") { Provider::Paid } else { Provider::Local };
            let r = route(
                value["hit"].as_bool().unwrap_or(false),
                provider,
                value["gpuReady"].as_bool().unwrap_or(false),
                value["keyPresent"].as_bool().unwrap_or(false),
                value["online"].as_bool().unwrap_or(false),
            );
            json!({ "route": match r { Route::Serve => "Serve", Route::Local => "Local", Route::Paid => "Paid", Route::NoKey => "NoKey", Route::NoGpu => "NoGpu", Route::PaidOffline => "PaidOffline" } })
        }
        // The endpoint control: shown only when the model says a request could be answered.
        "endpoint-ready" => json!({ "ready": endpointReady(
            value["resident"].as_bool().unwrap_or(false),
            value["keyPresent"].as_bool().unwrap_or(false),
            value["online"].as_bool().unwrap_or(false),
        ) }),
        // The bytes OpenRouter receives: only what the model spells, never a key.
        "encode-openrouter-request" => match request(&value["request"]) {
            Ok(request) => json!({ "bytes": encodeOpenRouterRequest(value["model"].as_str().unwrap_or("").to_owned(), &request, value["stream"].as_bool().unwrap_or(false)) }),
            Err(message) => json!({ "error": message }),
        },
        other => json!({ "error": format!("unknown op {other:?}") }),
    }
}

/// The View record as JSON, field by field, so the page and the projector read the same words.
pub fn view_json() -> Value {
    let v = view();
    json!({
        "headline": v.headline, "lede": v.lede, "promptPlaceholder": v.promptPlaceholder, "sendLabel": v.sendLabel,
        "loadingLabel": v.loadingLabel, "servedLabel": v.servedLabel,
        "sealedLabel": v.sealedLabel, "rederiveLabel": v.rederiveLabel, "identicalLabel": v.identicalLabel,
        "noGpuLabel": v.noGpuLabel, "offlineLabel": v.offlineLabel,
        "modelLabel": v.modelLabel, "appearanceLabel": v.appearanceLabel, "darkLabel": v.darkLabel, "lightLabel": v.lightLabel, "immersiveLabel": v.immersiveLabel,
        "wallpapers": v.wallpapers.iter().map(|w| json!({ "file": w.file, "name": w.label, "by": w.author, "byUrl": w.authorUrl })).collect::<Vec<_>>(),
        "localLabel": v.localLabel, "paidLabel": v.paidLabel, "keyLabel": v.keyLabel, "keyPlaceholder": v.keyPlaceholder, "keySavedLabel": v.keySavedLabel,
        "paidOnceLabel": v.paidOnceLabel, "costLabel": v.costLabel, "freeLabel": v.freeLabel, "noKeyLabel": v.noKeyLabel, "noCreditLabel": v.noCreditLabel,
        "providerBusyLabel": v.providerBusyLabel, "paidOfflineLabel": v.paidOfflineLabel,
        "paidModels": v.paidModels.iter().map(|m| json!({ "id": m.id, "label": m.label })).collect::<Vec<_>>(),
        "connectLabel": v.connectLabel, "connectedLabel": v.connectedLabel, "listeningLabel": v.listeningLabel, "notConnectedLabel": v.notConnectedLabel,
        "runLabel": v.runLabel, "verifyLabel": v.verifyLabel, "baseUrlLabel": v.baseUrlLabel, "anyKeyLabel": v.anyKeyLabel, "modelIdLabel": v.modelIdLabel,
        "testLabel": v.testLabel, "stayOpenLabel": v.stayOpenLabel, "askLabel": v.askLabel, "secondTabLabel": v.secondTabLabel,
        "copyLabel": v.copyLabel, "copiedLabel": v.copiedLabel, "macLabel": v.macLabel, "windowsLabel": v.windowsLabel,
    })
}

/// Allocate `len` bytes the host writes a request into.
#[no_mangle]
pub extern "C" fn holo_alloc(len: i32) -> i32 {
    let mut buffer = Vec::<u8>::with_capacity(len.max(0) as usize);
    let pointer = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    pointer as i32
}

/// Release a buffer `holo_alloc` or `holo_run` handed out.
#[no_mangle]
pub extern "C" fn holo_free(pointer: i32, len: i32) {
    if pointer != 0 && len > 0 {
        // SAFETY: only pointers this module allocated with the same length are passed back.
        unsafe { drop(Vec::from_raw_parts(pointer as *mut u8, len as usize, len as usize)) };
    }
}

/// Run one request. Returns `(pointer << 32) | len` of a UTF-8 JSON response the host must free.
#[no_mangle]
pub extern "C" fn holo_run(pointer: i32, len: i32) -> i64 {
    // SAFETY: the host wrote `len` bytes at a pointer from `holo_alloc`.
    let input = unsafe { std::slice::from_raw_parts(pointer as *const u8, len.max(0) as usize) };
    let output = run(input).to_string().into_bytes();
    let out_len = output.len() as i64;
    let mut output = output.into_boxed_slice();
    let out_pointer = output.as_mut_ptr() as i64;
    std::mem::forget(output);
    (out_pointer << 32) | out_len
}
