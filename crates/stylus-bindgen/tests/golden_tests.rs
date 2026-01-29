use std::fs;
use std::process::Command;

fn run_bindgen(input: &str, output: &str) -> String {
    let status = Command::new("cargo")
        .args(["run", "-p", "stylus-bindgen", "--", "--input", input, "--output", output])
        .current_dir(env!("CARGO_MANIFEST_DIR").to_owned() + "/../..")
        .status()
        .expect("Failed to execute stylus-bindgen");
    
    assert!(status.success(), "stylus-bindgen failed");
    fs::read_to_string(output).expect("Failed to read output file")
}

fn read_expected(name: &str) -> String {
    let path = format!("{}/tests/expected/{}.rs", env!("CARGO_MANIFEST_DIR"), name);
    fs::read_to_string(&path).expect(&format!("Failed to read expected file: {}", path))
}

#[test]
fn test_erc20_golden() {
    let temp_output = "/tmp/erc20_test.rs";
    let input = format!("{}/../../abis/erc20.json", env!("CARGO_MANIFEST_DIR"));
    
    let generated = run_bindgen(&input, temp_output);
    let expected = read_expected("erc20");
    
    assert_eq!(
        generated.trim(),
        expected.trim(),
        "ERC20 generated bindings do not match expected golden output"
    );
}

#[test]
fn test_erc721_golden() {
    let temp_output = "/tmp/erc721_test.rs";
    let input = format!("{}/../../abis/erc721.json", env!("CARGO_MANIFEST_DIR"));
    
    let generated = run_bindgen(&input, temp_output);
    let expected = read_expected("erc721");
    
    assert_eq!(
        generated.trim(),
        expected.trim(),
        "ERC721 generated bindings do not match expected golden output"
    );
}

#[test]
fn test_erc721_overload_safety() {
    let expected = read_expected("erc721");
    
    assert!(
        expected.contains("safe_transfer_from__0x42842e0e"),
        "Missing safeTransferFrom(address,address,uint256) overload"
    );
    assert!(
        expected.contains("safe_transfer_from__0xb88d4fde"),
        "Missing safeTransferFrom(address,address,uint256,bytes) overload"
    );
    
    let count_42842e0e = expected.matches("safe_transfer_from__0x42842e0e").count();
    let count_b88d4fde = expected.matches("safe_transfer_from__0xb88d4fde").count();
    
    assert!(count_42842e0e >= 1, "Expected at least 1 occurrence of 0x42842e0e function");
    assert!(count_b88d4fde >= 1, "Expected at least 1 occurrence of 0xb88d4fde function");
}

#[test]
fn test_erc1155_golden() {
    let temp_output = "/tmp/erc1155_test.rs";
    let input = format!("{}/../../abis/erc1155.json", env!("CARGO_MANIFEST_DIR"));
    
    let generated = run_bindgen(&input, temp_output);
    let expected = read_expected("erc1155");
    
    assert_eq!(
        generated.trim(),
        expected.trim(),
        "ERC1155 generated bindings do not match expected golden output"
    );
}

#[test]
fn test_ierc165_golden() {
    let temp_output = "/tmp/ierc165_test.rs";
    let input = format!("{}/../../abis/ierc165.json", env!("CARGO_MANIFEST_DIR"));
    
    let generated = run_bindgen(&input, temp_output);
    let expected = read_expected("ierc165");
    
    assert_eq!(
        generated.trim(),
        expected.trim(),
        "IERC165 generated bindings do not match expected golden output"
    );
}

#[test]
fn test_deterministic_output() {
    let input = format!("{}/../../abis/erc20.json", env!("CARGO_MANIFEST_DIR"));
    
    let output1 = run_bindgen(&input, "/tmp/determinism_test_1.rs");
    let output2 = run_bindgen(&input, "/tmp/determinism_test_2.rs");
    
    assert_eq!(
        output1, output2,
        "Bindgen output is not deterministic"
    );
}
