//! Fixture corpus validation tests.
//! Ensures ABI fixture files and golden output files exist and are well-formed.

mod common;

use std::fs;
use std::path::Path;

use common::{abi_path, STANDARDS};

#[test]
fn fixture_corpus_exists_and_valid() {
    for name in STANDARDS {
        let path = abi_path(name);
        let p = Path::new(&path);
        assert!(p.exists(), "Fixture corpus must include abis/{}.json", name);
        let content = fs::read_to_string(&path).expect("abi file must be readable");
        let json: serde_json::Value =
            serde_json::from_str(&content).expect("abi file must be valid JSON");
        let arr = json.as_array().expect("ABI must be a JSON array");
        let has_functions = arr
            .iter()
            .any(|e| e.get("type").and_then(|t| t.as_str()) == Some("function"));
        assert!(
            has_functions,
            "Fixture abis/{}.json must contain at least one function entry",
            name
        );
    }
}

#[test]
fn golden_files_exist_for_all_standards() {
    for name in STANDARDS {
        let path = format!("{}/tests/expected/{}.rs", env!("CARGO_MANIFEST_DIR"), name);
        assert!(
            Path::new(&path).exists(),
            "Golden output file tests/expected/{}.rs must exist",
            name
        );
    }
}
