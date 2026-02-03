//! Stylus Interface Packs - Pre-generated bindings for common ERC standards
//!
//! This crate provides overload-safe Stylus bindings for:
//! - ERC20 (fungible tokens)
//! - ERC721 (non-fungible tokens) with overload-safe wrappers
//! - ERC1155 (multi-tokens)
//! - IERC165 (interface detection)

#![allow(non_snake_case)] // selector-suffixed names e.g. approve__0x095ea7b3 are intentional
#![allow(unused_variables)] // params not yet encoded (Milestone 2); kept for API clarity

pub mod erc1155;
pub mod erc20;
pub mod erc721;
pub mod ierc165;

#[cfg(test)]
mod tests {
    fn sources() -> &'static [(&'static str, &'static str)] {
        &[
            ("erc20", include_str!("erc20.rs")),
            ("erc721", include_str!("erc721.rs")),
            ("erc1155", include_str!("erc1155.rs")),
            ("ierc165", include_str!("ierc165.rs")),
        ]
    }

    mod instantiation {
        use stylus_sdk::alloy_primitives::Address;

        #[test]
        fn erc20() {
            let _ = crate::erc20::Contract::new(Address::ZERO);
        }

        #[test]
        fn erc721() {
            let _ = crate::erc721::Contract::new(Address::ZERO);
        }

        #[test]
        fn erc1155() {
            let _ = crate::erc1155::Contract::new(Address::ZERO);
        }

        #[test]
        fn ierc165() {
            let _ = crate::ierc165::Contract::new(Address::ZERO);
        }
    }

    mod erc20 {
        #[test]
        fn has_all_expected_functions() {
            let src = include_str!("erc20.rs");
            let expected = [
                ("approve__0x095ea7b3", "approve(address,uint256)"),
                ("balance_of__0x70a08231", "balanceOf(address)"),
                ("transfer__0xa9059cbb", "transfer(address,uint256)"),
            ];
            for (fn_name, sig) in expected {
                assert!(
                    src.contains(fn_name),
                    "ERC20 must contain function {} for {}",
                    fn_name,
                    sig
                );
            }
        }

        #[test]
        fn has_exactly_3_functions() {
            let src = include_str!("erc20.rs");
            let fn_count = src.matches("pub fn ").count();
            // 3 ERC20 functions + 1 new() constructor = 4
            assert_eq!(
                fn_count, 4,
                "ERC20 must have exactly 3 selector-suffixed functions plus new()"
            );
        }

        #[test]
        fn selectors_are_correct() {
            let src = include_str!("erc20.rs");
            assert!(
                src.contains("fn approve__0x095ea7b3") && src.contains("hex::decode(\"095ea7b3\")"),
                "approve selector mismatch"
            );
            assert!(
                src.contains("fn balance_of__0x70a08231")
                    && src.contains("hex::decode(\"70a08231\")"),
                "balanceOf selector mismatch"
            );
            assert!(
                src.contains("fn transfer__0xa9059cbb")
                    && src.contains("hex::decode(\"a9059cbb\")"),
                "transfer selector mismatch"
            );
        }
    }

    mod erc721 {
        #[test]
        fn has_all_expected_functions() {
            let src = include_str!("erc721.rs");
            let expected = [
                ("approve__0x095ea7b3", "approve(address,uint256)"),
                ("balance_of__0x70a08231", "balanceOf(address)"),
                ("get_approved__0x081812fc", "getApproved(uint256)"),
                (
                    "is_approved_for_all__0xe985e9c5",
                    "isApprovedForAll(address,address)",
                ),
                ("owner_of__0x6352211e", "ownerOf(uint256)"),
                (
                    "safe_transfer_from__0x42842e0e",
                    "safeTransferFrom(address,address,uint256)",
                ),
                (
                    "safe_transfer_from__0xb88d4fde",
                    "safeTransferFrom(address,address,uint256,bytes)",
                ),
                (
                    "set_approval_for_all__0xa22cb465",
                    "setApprovalForAll(address,bool)",
                ),
                (
                    "transfer_from__0x23b872dd",
                    "transferFrom(address,address,uint256)",
                ),
            ];
            for (fn_name, sig) in expected {
                assert!(
                    src.contains(fn_name),
                    "ERC721 must contain function {} for {}",
                    fn_name,
                    sig
                );
            }
        }

        #[test]
        fn has_exactly_9_functions() {
            let src = include_str!("erc721.rs");
            let fn_count = src.matches("pub fn ").count();
            // 9 ERC721 functions + 1 new() = 10
            assert_eq!(
                fn_count, 10,
                "ERC721 must have exactly 9 selector-suffixed functions plus new()"
            );
        }

        #[test]
        fn overload_safe_wrappers() {
            let src = include_str!("erc721.rs");
            assert!(
                src.contains("safe_transfer_from__0x42842e0e"),
                "ERC721 must expose safeTransferFrom(address,address,uint256) as safe_transfer_from__0x42842e0e"
            );
            assert!(
                src.contains("safe_transfer_from__0xb88d4fde"),
                "ERC721 must expose safeTransferFrom(address,address,uint256,bytes) as safe_transfer_from__0xb88d4fde"
            );
        }

        #[test]
        fn overloads_have_distinct_selectors() {
            assert_ne!("0x42842e0e", "0xb88d4fde", "sanity: selectors must differ");
            let src = include_str!("erc721.rs");
            let count_3arg = src.matches("fn safe_transfer_from__0x42842e0e(").count();
            let count_4arg = src.matches("fn safe_transfer_from__0xb88d4fde(").count();
            assert_eq!(
                count_3arg, 1,
                "safeTransferFrom(3-arg) must be defined exactly once"
            );
            assert_eq!(
                count_4arg, 1,
                "safeTransferFrom(4-arg) must be defined exactly once"
            );
        }

        #[test]
        fn overload_3arg_has_correct_params() {
            let src = include_str!("erc721.rs");
            assert!(
                src.contains("safe_transfer_from__0x42842e0e"),
                "3-arg overload must exist"
            );
            assert!(
                src.contains("from: Address"),
                "3-arg overload must have from: Address"
            );
            assert!(
                src.contains("to: Address"),
                "3-arg overload must have to: Address"
            );
            assert!(
                src.contains("tokenId: U256"),
                "3-arg overload must have tokenId: U256"
            );
            let pos_3arg = src.find("safe_transfer_from__0x42842e0e").unwrap();
            let slice_before_4arg = &src[..src
                .find("safe_transfer_from__0xb88d4fde")
                .unwrap_or(src.len())];
            assert!(
                slice_before_4arg[pos_3arg..].contains("tokenId: U256")
                    && !slice_before_4arg[pos_3arg..].contains("data: Vec<u8>"),
                "3-arg overload must take (from: Address, to: Address, tokenId: U256) without data"
            );
        }

        #[test]
        fn overload_4arg_has_correct_params() {
            let src = include_str!("erc721.rs");
            assert!(
                src.contains("safe_transfer_from__0xb88d4fde"),
                "4-arg overload must exist"
            );
            assert!(
                src.find("safe_transfer_from__0xb88d4fde")
                    .map(|i| src[i..].contains("data: Vec<u8>"))
                    .unwrap_or(false),
                "4-arg overload must take (from: Address, to: Address, tokenId: U256, data: Vec<u8>)"
            );
        }
    }

    mod erc1155 {
        #[test]
        fn has_all_expected_functions() {
            let src = include_str!("erc1155.rs");
            let expected = [
                ("balance_of__0x00fdd58e", "balanceOf(address,uint256)"),
                (
                    "balance_of_batch__0x4e1273f4",
                    "balanceOfBatch(address[],uint256[])",
                ),
                (
                    "is_approved_for_all__0xe985e9c5",
                    "isApprovedForAll(address,address)",
                ),
                (
                    "safe_batch_transfer_from__0x2eb2c2d6",
                    "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)",
                ),
                (
                    "safe_transfer_from__0xf242432a",
                    "safeTransferFrom(address,address,uint256,uint256,bytes)",
                ),
                (
                    "set_approval_for_all__0xa22cb465",
                    "setApprovalForAll(address,bool)",
                ),
            ];
            for (fn_name, sig) in expected {
                assert!(
                    src.contains(fn_name),
                    "ERC1155 must contain function {} for {}",
                    fn_name,
                    sig
                );
            }
        }

        #[test]
        fn has_exactly_6_functions() {
            let src = include_str!("erc1155.rs");
            let fn_count = src.matches("pub fn ").count();
            // 6 ERC1155 functions + 1 new() = 7
            assert_eq!(
                fn_count, 7,
                "ERC1155 must have exactly 6 selector-suffixed functions plus new()"
            );
        }
    }

    mod ierc165 {
        #[test]
        fn has_all_expected_functions() {
            let src = include_str!("ierc165.rs");
            assert!(
                src.contains("supports_interface__0x01ffc9a7"),
                "IERC165 must contain supports_interface__0x01ffc9a7"
            );
        }

        #[test]
        fn has_exactly_1_function() {
            let src = include_str!("ierc165.rs");
            let fn_count = src.matches("pub fn ").count();
            // 1 IERC165 function + 1 new() = 2
            assert_eq!(
                fn_count, 2,
                "IERC165 must have exactly 1 selector-suffixed function plus new()"
            );
        }
    }

    mod cross_interface {
        use super::sources;

        #[test]
        fn all_use_selector_suffixed_naming() {
            for (name, src) in sources() {
                for line in src.lines() {
                    if let Some(fn_start) = line.find("pub fn ") {
                        let after = &line[fn_start + 7..];
                        let fn_name = after.split('(').next().unwrap();
                        if fn_name == "new" {
                            continue;
                        }
                        assert!(
                            fn_name.contains("__0x"),
                            "In {}: function '{}' must use __0x<selector> naming",
                            name,
                            fn_name
                        );
                        let selector_part = fn_name.split("__0x").last().unwrap();
                        assert_eq!(
                            selector_part.len(),
                            8,
                            "In {}: function '{}' selector must be 8 hex chars, got '{}'",
                            name,
                            fn_name,
                            selector_part
                        );
                        assert!(
                            selector_part.chars().all(|c| c.is_ascii_hexdigit()),
                            "In {}: function '{}' selector '{}' must be hex",
                            name,
                            fn_name,
                            selector_part
                        );
                    }
                }
            }
        }

        #[test]
        fn all_have_contract_struct_with_address() {
            for (name, src) in sources() {
                assert!(
                    src.contains("pub struct Contract"),
                    "{} must define pub struct Contract",
                    name
                );
                assert!(
                    src.contains("pub address: Address"),
                    "{} Contract must have pub address: Address field",
                    name
                );
                assert!(
                    src.contains("pub fn new(address: Address) -> Self"),
                    "{} Contract must have pub fn new(address: Address) constructor",
                    name
                );
            }
        }

        #[test]
        fn no_duplicate_selectors() {
            for (name, src) in sources() {
                let mut selectors: Vec<&str> = Vec::new();
                for line in src.lines() {
                    if let Some(fn_start) = line.find("pub fn ") {
                        let after = &line[fn_start + 7..];
                        let fn_name = after.split('(').next().unwrap();
                        if fn_name == "new" {
                            continue;
                        }
                        let selector = fn_name.split("__0x").last().unwrap();
                        assert!(
                            !selectors.contains(&selector),
                            "In {}: duplicate selector 0x{} found",
                            name,
                            selector
                        );
                        selectors.push(selector);
                    }
                }
            }
        }

        #[test]
        fn selector_in_hex_decode_matches_function_name() {
            for (name, src) in sources() {
                let lines: Vec<&str> = src.lines().collect();
                for (i, line) in lines.iter().enumerate() {
                    if let Some(fn_start) = line.find("pub fn ") {
                        let after = &line[fn_start + 7..];
                        let fn_name = after.split('(').next().unwrap();
                        if fn_name == "new" {
                            continue;
                        }
                        let selector = fn_name.split("__0x").last().unwrap();
                        let body_window = &lines[i..std::cmp::min(i + 12, lines.len())];
                        let body = body_window.join("\n");
                        assert!(
                            body.contains(&format!("hex::decode(\"{}\")", selector)),
                            "In {}: function {} must call hex::decode(\"{}\") in its body",
                            name,
                            fn_name,
                            selector
                        );
                    }
                }
            }
        }
    }
}
