//! Golden output regression tests for stylus-bindgen.
//! These spawn the bindgen subprocess and compare output against expected golden files.

mod common;

use std::collections::HashSet;
use std::fs;

use common::{abi_path, extract_selector_fns, read_expected, run_bindgen, STANDARDS};

#[test]
fn erc20_golden() {
    let generated = run_bindgen(&abi_path("erc20"));
    let expected = read_expected("erc20");
    assert_eq!(
        generated.trim(),
        expected.trim(),
        "ERC20 generated bindings do not match expected golden output"
    );
}

#[test]
fn erc721_golden() {
    let generated = run_bindgen(&abi_path("erc721"));
    let expected = read_expected("erc721");
    assert_eq!(
        generated.trim(),
        expected.trim(),
        "ERC721 generated bindings do not match expected golden output"
    );
}

#[test]
fn erc1155_golden() {
    let generated = run_bindgen(&abi_path("erc1155"));
    let expected = read_expected("erc1155");
    assert_eq!(
        generated.trim(),
        expected.trim(),
        "ERC1155 generated bindings do not match expected golden output"
    );
}

#[test]
fn ierc165_golden() {
    let generated = run_bindgen(&abi_path("ierc165"));
    let expected = read_expected("ierc165");
    assert_eq!(
        generated.trim(),
        expected.trim(),
        "IERC165 generated bindings do not match expected golden output"
    );
}

#[test]
fn deterministic_output() {
    let input = abi_path("erc20");
    let output1 = run_bindgen(&input);
    let output2 = run_bindgen(&input);
    assert_eq!(output1, output2, "Bindgen output is not deterministic");
}

#[test]
fn golden_matches_interfaces_crate() {
    let interface_dir = format!(
        "{}/../../crates/stylus-interfaces/src",
        env!("CARGO_MANIFEST_DIR")
    );
    for name in STANDARDS {
        let expected = read_expected(name);
        let interface_src = fs::read_to_string(format!("{}/{}.rs", interface_dir, name))
            .expect(&format!("interface source {}.rs must exist", name));

        let gen_fns: HashSet<String> = extract_selector_fns(&expected).into_iter().collect();
        let iface_fns: HashSet<String> = extract_selector_fns(&interface_src).into_iter().collect();

        assert_eq!(
            gen_fns, iface_fns,
            "{}: golden output function set must match interfaces crate.\nGolden: {:?}\nInterface: {:?}",
            name, gen_fns, iface_fns
        );
    }
}
