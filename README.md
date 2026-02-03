# Stylus Interface Packs + Bindgen (SIPB)

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Overload-safe Stylus bindings for calling Solidity contracts from Rust.**

SIPB solves the interoperability friction when calling existing Solidity contracts from Arbitrum Stylus. It provides pre-generated interface packs for common ERC standards and a code generator that produces selector-exact, collision-free bindings from any ABI JSON.

## The Problem

When calling Solidity contracts from Stylus, developers face several challenges:

1. **Overloaded Functions Collide**: Solidity allows multiple functions with the same name but different parameters (e.g., ERC721's two `safeTransferFrom` variants). Naive bindings create name collisions.

2. **Inherited Interfaces**: Real-world contracts inherit from multiple interfaces. Flattening these without conflicts is error-prone.

3. **Manual Selector Management**: Developers must manually compute and track function selectors, which is tedious and error-prone.

## The Solution

SIPB uses a **deterministic naming scheme** that appends the 4-byte function selector to each function name:

```rust
// Instead of this (collision!):
fn safe_transfer_from(...) // Which one?

// SIPB generates this (unambiguous):
fn safe_transfer_from__0x42842e0e(...) // safeTransferFrom(address,address,uint256)
fn safe_transfer_from__0xb88d4fde(...) // safeTransferFrom(address,address,uint256,bytes)
```

Each function name is globally unique and maps exactly to one selector.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
stylus-interfaces = { git = "https://github.com/psyberpath/stylus-sipb.git" }
```

Or install the CLI:

```bash
cargo install --git https://github.com/psyberpath/stylus-sipb.git stylus-bindgen
```

---

## Quick Start

### Using Pre-built Interface Packs

```rust
use stylus_interfaces::erc20::Contract as ERC20;
use stylus_sdk::alloy_primitives::{Address, U256};

// Create a contract instance
let token = ERC20::new(token_address);

// Call functions using the selector-suffixed names
let balance = token.balance_of__0x70a08231(owner_address)?;
```

### Generating Custom Bindings

```bash
# Generate bindings from any ABI JSON
stylus-bindgen --input my_contract.abi.json --output src/my_contract.rs
```

---

## Crates

| Crate | Description |
|-------|-------------|
| `stylus-interfaces` | Pre-generated bindings for ERC20, ERC721, ERC1155, IERC165 |
| `stylus-bindgen` | CLI tool to generate bindings from ABI JSON |
| `sipb-preflight` | CI tool for detecting stale bindings (Milestone 3) |

---

## Interface Packs

### ERC20

```rust
use stylus_interfaces::erc20::Contract;

let token = Contract::new(address);
token.transfer__0xa9059cbb(to, amount)?;
token.approve__0x095ea7b3(spender, amount)?;
token.balance_of__0x70a08231(owner)?;
```

### ERC721 (with Overload Safety)

```rust
use stylus_interfaces::erc721::Contract;

let nft = Contract::new(address);

// Two distinct functions for the overloaded safeTransferFrom:
nft.safe_transfer_from__0x42842e0e(from, to, token_id)?;           // without data
nft.safe_transfer_from__0xb88d4fde(from, to, token_id, data)?;     // with data
```

### ERC1155

```rust
use stylus_interfaces::erc1155::Contract;

let multi = Contract::new(address);
multi.balance_of__0x00fdd58e(account, id)?;
multi.safe_transfer_from__0xf242432a(from, to, id, value, data)?;
```

### IERC165

```rust
use stylus_interfaces::ierc165::Contract;

let contract = Contract::new(address);
contract.supports_interface__0x01ffc9a7(interface_id)?;
```

---

## Naming Convention

All generated functions follow this pattern:

```
{snake_case_name}__0x{4_byte_selector}
```

Examples:

| Solidity Signature | Generated Rust Function |
|-------------------|------------------------|
| `transfer(address,uint256)` | `transfer__0xa9059cbb` |
| `balanceOf(address)` | `balance_of__0x70a08231` |
| `safeTransferFrom(address,address,uint256)` | `safe_transfer_from__0x42842e0e` |
| `safeTransferFrom(address,address,uint256,bytes)` | `safe_transfer_from__0xb88d4fde` |

The selector is computed as `keccak256(signature)[0:4]`.

---

## Development

### Build

```bash
cargo build --workspace
```

### Test

```bash
# Run full workspace tests (recommended)
cargo test --workspace

# If the first run seems stuck, the golden tests build the bindgen binary once (you'll see
# "[golden_tests] Building stylus-bindgen binary (one-time)..."). To avoid that wait, run:
cargo test-ready && cargo test --workspace

# Run golden output tests only
cargo test -p stylus-bindgen --test golden_tests

# Build interface packs
cargo build -p stylus-interfaces
```

### Mutation testing (cargo-mutants)

The project uses [cargo-mutants](https://mutants.rs/) to check that the test suite catches mutated code. CI runs it on pushes to `main` and on PRs that touch `crates/` or config.

```bash
# Install once (if not in CI)
cargo install cargo-mutants

# Run mutation tests (uses .cargo/mutants.toml)
cargo mutants
```

Config: `.cargo/mutants.toml`. Results are written to `mutants.out` (CI uploads this as an artifact).

### Generate Interface from ABI

```bash
cargo run -p stylus-bindgen -- --input abis/erc721.json --output output.rs
```

---

## Project Structure

```
stylus-sipb/
├── .cargo/
│   ├── config.toml                # test-ready alias
│   └── mutants.toml               # cargo-mutants config
├── abis/                          # ABI JSON fixtures
│   ├── erc20.json
│   ├── erc721.json                # Contains safeTransferFrom overloads
│   ├── erc1155.json
│   └── ierc165.json
├── crates/
│   ├── stylus-interfaces/         # Pre-generated interface packs
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── erc20.rs
│   │       ├── erc721.rs
│   │       ├── erc1155.rs
│   │       └── ierc165.rs
│   ├── stylus-bindgen/            # Code generator CLI
│   │   ├── src/main.rs
│   │   └── tests/
│   │       ├── golden_tests.rs    # Regression tests
│   │       └── expected/          # Golden output files
│   └── sipb-preflight/            # CI preflight tool (M3)
└── Cargo.toml                     # Workspace config
```

---

## Roadmap

### Milestone 1 ✅ (Complete)
- [x] Interface packs: ERC20, ERC721, ERC1155, IERC165
- [x] Overload-safe bindgen with `name__0x<selector>` scheme
- [x] Golden output regression tests

### Milestone 2 (Planned)
- [ ] Tuple support via `sol!` macro
- [ ] Multi-ABI merge with deduplication
- [ ] Orbit chain profiles (`sipb.toml`)

### Milestone 3 (Planned)
- [ ] Preflight command for CI
- [ ] SARIF output for GitHub Code Scanning
- [ ] GitHub Action packaging

---

## License

MIT License - see [LICENSE](LICENSE) for details.

---

## Contributing

Contributions welcome! Please open an issue first to discuss proposed changes.
