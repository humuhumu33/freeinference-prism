//! Byte identity of the generated core against the daemon's rule, on the
//! fixed corpus `tools/corpus.py` writes to `model/corpus.json`.

use freeinference_core::{decide, memoMatches, preimages, Decision, Memo, Message, Request};
use serde_json::Value;

fn corpus() -> Vec<Value> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../model/corpus.json");
    let text = std::fs::read_to_string(path).expect("model/corpus.json; run python3 tools/corpus.py");
    serde_json::from_str(&text).expect("corpus is JSON")
}

fn request(case: &Value) -> Request {
    Request {
        model: "webgpu:BitNet".to_owned(),
        messages: case["messages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| Message {
                role: m["role"].as_str().unwrap().to_owned(),
                content: m["content"].as_str().unwrap().to_owned(),
            })
            .collect(),
        maxTokens: case["max_tokens"].as_u64(),
        seed: case["seed"].as_u64(),
        // The model spells an absent temperature as the empty string.
        temperature: case["temperature"].as_str().unwrap_or("").to_owned(),
    }
}

#[test]
fn prompt_and_params_bytes_match_the_daemon_rule_on_every_case() {
    let cases = corpus();
    assert!(cases.len() >= 16, "corpus has {} cases", cases.len());
    let mut checked = 0;
    for case in &cases {
        let name = case["name"].as_str().unwrap();
        let got = preimages(&request(case));
        assert_eq!(
            String::from_utf8(got.prompt.clone()).unwrap(),
            case["prompt"].as_str().unwrap(),
            "prompt bytes differ on case {name}"
        );
        assert_eq!(
            String::from_utf8(got.params.clone()).unwrap(),
            case["params"].as_str().unwrap(),
            "params bytes differ on case {name}"
        );
        checked += 1;
    }
    println!("corpus: {checked} cases, prompt and params bytes identical");
}

#[test]
fn memo_matches_its_own_key_and_nothing_else() {
    let memo = Memo {
        model: vec!["did:holo:sha256:model".to_owned(), "webgpu:BitNet".to_owned()],
        engineKappa: "e".to_owned(),
        promptKappa: "blake3:p".to_owned(),
        paramsKappa: "blake3:q".to_owned(),
        outputKappa: "blake3:o".to_owned(),
        receipt: "blake3:r".to_owned(),
    };
    let by_name = vec!["webgpu:BitNet".to_owned()];
    let by_kappa = vec!["did:holo:sha256:model".to_owned()];
    let other = vec!["smollm2".to_owned()];
    assert!(memoMatches(&memo, &by_name, "blake3:p", "blake3:q"));
    assert!(memoMatches(&memo, &by_kappa, "blake3:p", "blake3:q"));
    assert!(!memoMatches(&memo, &other, "blake3:p", "blake3:q"));
    assert!(!memoMatches(&memo, &by_name, "blake3:x", "blake3:q"));
    assert!(!memoMatches(&memo, &by_name, "blake3:p", "blake3:x"));
    assert!(!memoMatches(&memo, &[], "blake3:p", "blake3:q"));
}

#[test]
fn decision_serves_first_executes_with_a_worker_and_refuses_otherwise() {
    assert_eq!(decide(true, false), Decision::Serve);
    assert_eq!(decide(true, true), Decision::Serve);
    assert_eq!(decide(false, true), Decision::Execute);
    assert_eq!(decide(false, false), Decision::Refuse);
}
