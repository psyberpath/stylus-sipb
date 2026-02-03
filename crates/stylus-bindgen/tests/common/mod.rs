//! Shared helpers for stylus-bindgen integration tests.

#![allow(dead_code)] // Each test binary uses a different subset of helpers.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

/// All supported ERC/interface standards.
pub const STANDARDS: &[&str] = &["erc20", "erc721", "erc1155", "ierc165"];

pub fn workspace_root() -> String {
    format!("{}/../..", env!("CARGO_MANIFEST_DIR"))
}

pub fn abi_path(name: &str) -> String {
    format!("{}/abis/{}.json", workspace_root(), name)
}

/// Path to the stylus-bindgen binary. Uses CARGO_BIN_EXE when set (e.g. in cargo test), else
/// the workspace target dir. Builds the binary once if missing so we avoid spawning `cargo run`
/// many times (slow + lock contention).
pub fn bindgen_binary() -> PathBuf {
    if let Ok(path) = std::env::var("CARGO_BIN_EXE_stylus_bindgen") {
        return PathBuf::from(path);
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("target"));
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".into());
    let exe = format!("stylus-bindgen{}", std::env::consts::EXE_SUFFIX);
    let bin = target.join(&profile).join(&exe);
    if !bin.exists() {
        static BUILD: std::sync::Once = std::sync::Once::new();
        BUILD.call_once(|| {
            eprintln!("[golden_tests] Building stylus-bindgen binary (one-time)...");
            let mut cmd = std::process::Command::new("cargo");
            cmd.args(["build", "-p", "stylus-bindgen"])
                .current_dir(&root)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit());
            let status = cmd.status().expect("Failed to run cargo build");
            assert!(status.success(), "cargo build -p stylus-bindgen failed");
        });
    }
    bin
}

/// Unique temp path per call so we never read stale output (catches main->Ok() mutant).
pub fn unique_output_path() -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let c = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("stylus_bindgen_{}_{}.rs", std::process::id(), c))
}

pub fn run_bindgen(input: &str) -> String {
    let output = unique_output_path();
    let bin = bindgen_binary();
    assert!(
        bin.exists(),
        "stylus-bindgen binary not found at {} (run `cargo build -p stylus-bindgen` first)",
        bin.display()
    );
    let status = Command::new(&bin)
        .args(["--input", input, "--output", output.to_str().unwrap()])
        .current_dir(workspace_root())
        .status()
        .expect("Failed to execute stylus-bindgen");

    assert!(status.success(), "stylus-bindgen failed");
    fs::read_to_string(&output).expect("Failed to read output file")
}

pub fn read_expected(name: &str) -> String {
    let path = format!("{}/tests/expected/{}.rs", env!("CARGO_MANIFEST_DIR"), name);
    fs::read_to_string(&path).expect(&format!("Failed to read expected file: {}", path))
}

/// Extract all `pub fn <name>(` function names from source code.
pub fn extract_fn_names(src: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in src.lines() {
        if let Some(pos) = line.find("pub fn ") {
            let after = &line[pos + 7..];
            if let Some(paren) = after.find('(') {
                names.push(after[..paren].to_string());
            }
        }
    }
    names
}

/// Extract selector-suffixed function names (excluding `new`).
pub fn extract_selector_fns(src: &str) -> Vec<String> {
    extract_fn_names(src)
        .into_iter()
        .filter(|n| n != "new")
        .collect()
}

/// Validate that a function name follows `snake_case__0x[0-9a-f]{8}`.
pub fn is_valid_selector_name(name: &str) -> bool {
    let parts: Vec<&str> = name.splitn(2, "__0x").collect();
    if parts.len() != 2 {
        return false;
    }
    let selector = parts[1];
    selector.len() == 8 && selector.chars().all(|c| c.is_ascii_hexdigit())
}
