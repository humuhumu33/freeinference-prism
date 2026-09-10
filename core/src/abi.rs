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

use crate::{decide, preimages, view, Decision, Message, Request};
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
        other => json!({ "error": format!("unknown op {other:?}") }),
    }
}

/// The View record as JSON, field by field, so the page and the projector read the same words.
pub fn view_json() -> Value {
    let v = view();
    json!({
        "headline": v.headline, "lede": v.lede, "promptPlaceholder": v.promptPlaceholder, "sendLabel": v.sendLabel,
        "loadingLabel": v.loadingLabel, "residentLabel": v.residentLabel, "servedLabel": v.servedLabel,
        "sealedLabel": v.sealedLabel, "rederiveLabel": v.rederiveLabel, "identicalLabel": v.identicalLabel,
        "noGpuLabel": v.noGpuLabel, "offlineLabel": v.offlineLabel, "repoLabel": v.repoLabel, "repoUrl": v.repoUrl,
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
