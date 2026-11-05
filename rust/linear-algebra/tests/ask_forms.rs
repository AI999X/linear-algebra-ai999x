//! The question reader on Linear Algebra's documented questions (ask.rs): a documented question itself, word for word
//! and number for number, has its documented reading even when its words, numbers free, are also other capabilities'
//! documented questions ("Find the determinant of [[1, 2], [3, 4]]." is determinant_2x2's; with other matrices it is
//! determinant's and determinant_3x3's). It was AMBIGUOUS: found by the release gate on the Rust-engine wheel
//! (tools/contract_acceptance.py --wheel: templates 200/208; the clean install's documented questions, 2026-11-03).

use linear_algebra_ai999x::pack;
use serde_json::Value;

fn ask(q: &str) -> Value {
    serde_json::from_str(&ax_engine::ask::ask_json(pack(), q)).unwrap()
}

#[test]
fn a_documented_question_word_for_word_has_its_documented_reading() {
    let a = ask("Find the determinant of [[1, 2], [3, 4]].");
    assert_eq!(a["outcome"], "SOLVED", "{a}");
    assert_eq!(a["capability"], "determinant_2x2", "{a}");
    assert_eq!(a["answer"]["value"], -2.0, "{a}");
}

#[test]
fn every_documented_question_has_its_documented_outcome() {
    let doc: Value = serde_json::from_str(linear_algebra_ai999x::PACK_JSON).unwrap();
    let mut n = 0;
    for f in doc["forms"].as_array().unwrap() {
        let want = f["outcome"].as_str().unwrap_or("");
        if want.is_empty() || (want == "SOLVED" && f.get("key").is_none()) {
            continue;
        }
        n += 1;
        let a = ask(f["q"].as_str().unwrap());
        assert_eq!(a["outcome"], want, "{}: {a}", f["q"]);
        if let Some(k) = f.get("key") {
            assert_eq!(&a["capability"], k, "{}: {a}", f["q"]);
        }
    }
    assert!(n >= 50, "only {n} documented questions");
}
