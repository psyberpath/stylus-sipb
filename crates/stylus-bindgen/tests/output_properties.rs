//! Property tests on generated golden output.
//! These read golden files directly (no subprocess) and validate structural properties.

mod common;

use std::collections::HashSet;

use common::{extract_selector_fns, is_valid_selector_name, read_expected, STANDARDS};

// ── Naming convention ─────────────────────────────────────────────

#[test]
#[allow(non_snake_case)]
fn naming_convention_name__0x_selector() {
    for name in STANDARDS {
        let src = read_expected(name);
        for fn_name in extract_selector_fns(&src) {
            assert!(
                is_valid_selector_name(&fn_name),
                "Bindgen must use name__0x<selector> naming; got '{}' in {}",
                fn_name,
                name
            );
        }
    }
}

// ── Function count verification ───────────────────────────────────

#[test]
fn function_counts() {
    let expected_counts = [("erc20", 3), ("erc721", 9), ("erc1155", 6), ("ierc165", 1)];
    for (name, expected) in expected_counts {
        let src = read_expected(name);
        let count = extract_selector_fns(&src).len();
        assert_eq!(
            count, expected,
            "{} must have exactly {} selector-suffixed functions, got {}",
            name, expected, count
        );
    }
}

// ── Type mapping ──────────────────────────────────────────────────

#[test]
fn type_mapping() {
    let erc20 = read_expected("erc20");
    assert!(
        erc20.contains("owner: Address"),
        "Solidity 'address' must map to Rust 'Address'"
    );
    assert!(
        erc20.contains("value: U256"),
        "Solidity 'uint256' must map to Rust 'U256'"
    );

    let erc721 = read_expected("erc721");
    assert!(
        erc721.contains("approved: bool"),
        "Solidity 'bool' must map to Rust 'bool'"
    );
    assert!(
        erc721.contains("data: Vec<u8>"),
        "Solidity 'bytes' must map to Rust 'Vec<u8>'"
    );
}

// ── Snake case conversion ─────────────────────────────────────────

#[test]
fn snake_case_conversion() {
    let src = read_expected("erc721");
    assert!(
        src.contains("balance_of__"),
        "balanceOf must become balance_of"
    );
    assert!(src.contains("owner_of__"), "ownerOf must become owner_of");
    assert!(
        src.contains("get_approved__"),
        "getApproved must become get_approved"
    );
    assert!(
        src.contains("is_approved_for_all__"),
        "isApprovedForAll must become is_approved_for_all"
    );
    assert!(
        src.contains("set_approval_for_all__"),
        "setApprovalForAll must become set_approval_for_all"
    );
    assert!(
        src.contains("transfer_from__"),
        "transferFrom must become transfer_from"
    );
    assert!(
        src.contains("safe_transfer_from__"),
        "safeTransferFrom must become safe_transfer_from"
    );
}

// ── Contract struct and constructor ───────────────────────────────

#[test]
fn contract_struct_and_constructor() {
    for name in STANDARDS {
        let src = read_expected(name);
        assert!(
            src.contains("pub struct Contract"),
            "{}: must have pub struct Contract",
            name
        );
        assert!(
            src.contains("pub address: Address"),
            "{}: must have pub address: Address",
            name
        );
        assert!(
            src.contains("pub fn new(address: Address) -> Self"),
            "{}: must have new() constructor",
            name
        );
    }
}

// ── Return type ───────────────────────────────────────────────────

#[test]
fn all_selector_functions_return_result() {
    for name in STANDARDS {
        let src = read_expected(name);
        for line in src.lines() {
            if line.contains("pub fn ") && !line.contains("fn new(") {
                assert!(
                    line.contains("Result<Vec<u8>, Vec<u8>>"),
                    "{}: selector function must return Result<Vec<u8>, Vec<u8>>: {}",
                    name,
                    line.trim()
                );
            }
        }
    }
}

// ── No duplicate selectors ────────────────────────────────────────

#[test]
fn no_duplicate_selectors() {
    for name in STANDARDS {
        let src = read_expected(name);
        let fns = extract_selector_fns(&src);
        let selectors: Vec<&str> = fns
            .iter()
            .map(|f| f.split("__0x").last().unwrap())
            .collect();
        let unique: HashSet<&&str> = selectors.iter().collect();
        assert_eq!(
            selectors.len(),
            unique.len(),
            "{}: duplicate selectors found in {:?}",
            name,
            selectors
        );
    }
}

// ── Selector in function body matches function name ───────────────

#[test]
fn selector_in_body_matches_name() {
    for name in STANDARDS {
        let src = read_expected(name);
        let lines: Vec<&str> = src.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if let Some(pos) = line.find("pub fn ") {
                let after = &line[pos + 7..];
                let fn_name = after.split('(').next().unwrap();
                if fn_name == "new" {
                    continue;
                }
                let selector = fn_name.split("__0x").last().unwrap();
                let body = lines[i..std::cmp::min(i + 6, lines.len())].join("\n");
                assert!(
                    body.contains(&format!("hex::decode(\"{}\")", selector)),
                    "{}: function {} body must call hex::decode(\"{}\")",
                    name,
                    fn_name,
                    selector
                );
            }
        }
    }
}

// ── Original Solidity signature preserved as comment ──────────────

#[test]
fn original_signature_comments() {
    let cases: &[(&str, &[&str])] = &[
        (
            "erc20",
            &[
                "approve(address,uint256)",
                "balanceOf(address)",
                "transfer(address,uint256)",
            ],
        ),
        (
            "erc721",
            &[
                "safeTransferFrom(address,address,uint256)",
                "safeTransferFrom(address,address,uint256,bytes)",
                "balanceOf(address)",
                "ownerOf(uint256)",
                "transferFrom(address,address,uint256)",
                "approve(address,uint256)",
                "getApproved(uint256)",
                "setApprovalForAll(address,bool)",
                "isApprovedForAll(address,address)",
            ],
        ),
        (
            "erc1155",
            &[
                "balanceOf(address,uint256)",
                "safeTransferFrom(address,address,uint256,uint256,bytes)",
            ],
        ),
        ("ierc165", &["supportsInterface(bytes4)"]),
    ];
    for (name, sigs) in cases {
        let src = read_expected(name);
        for sig in *sigs {
            assert!(
                src.contains(&format!("// Original: {}", sig)),
                "{}: must preserve original Solidity signature '{}' as comment",
                name,
                sig
            );
        }
    }
}

// ── ERC721 overload safety ────────────────────────────────────────

#[test]
fn erc721_overload_both_selectors_present() {
    let src = read_expected("erc721");
    assert!(
        src.contains("safe_transfer_from__0x42842e0e"),
        "Missing safeTransferFrom(address,address,uint256) overload"
    );
    assert!(
        src.contains("safe_transfer_from__0xb88d4fde"),
        "Missing safeTransferFrom(address,address,uint256,bytes) overload"
    );
}

#[test]
fn erc721_overloads_defined_exactly_once() {
    let src = read_expected("erc721");
    assert_eq!(
        src.matches("fn safe_transfer_from__0x42842e0e(").count(),
        1,
        "3-arg safeTransferFrom must be defined exactly once"
    );
    assert_eq!(
        src.matches("fn safe_transfer_from__0xb88d4fde(").count(),
        1,
        "4-arg safeTransferFrom must be defined exactly once"
    );
}

#[test]
fn erc721_overload_3arg_signature() {
    let src = read_expected("erc721");
    assert!(
        src.contains(
            "safe_transfer_from__0x42842e0e(&self, from: Address, to: Address, tokenId: U256)"
        ),
        "3-arg overload must have signature (from: Address, to: Address, tokenId: U256)"
    );
}

#[test]
fn erc721_overload_4arg_signature() {
    let src = read_expected("erc721");
    assert!(src.contains("safe_transfer_from__0xb88d4fde(&self, from: Address, to: Address, tokenId: U256, data: Vec<u8>)"),
        "4-arg overload must have signature (from: Address, to: Address, tokenId: U256, data: Vec<u8>)");
}

// ── ERC1155 no overload collision ─────────────────────────────────

#[test]
fn erc1155_no_overload_collision() {
    let src = read_expected("erc1155");
    assert!(
        src.contains("safe_transfer_from__0xf242432a"),
        "ERC1155 must have safe_transfer_from__0xf242432a"
    );
    assert!(
        src.contains("safe_batch_transfer_from__0x2eb2c2d6"),
        "ERC1155 must have safe_batch_transfer_from__0x2eb2c2d6"
    );
    assert_eq!(
        src.matches("fn safe_transfer_from__").count(),
        1,
        "ERC1155 must have exactly one safe_transfer_from function"
    );
}
