# tree-sitter-symbols

[![crates.io](https://img.shields.io/crates/v/tss?label=tss)](https://crates.io/crates/tss)
[![crates.io](https://img.shields.io/crates/v/tss-rust?label=tss-rust)](https://crates.io/crates/tss-rust)
[![documentation](https://img.shields.io/docsrs/tss?label=docs%3A%20tss)](https://docs.rs/tss)
[![documentation](https://img.shields.io/docsrs/tss-rust?label=docs%3A%20tss-rust)](https://docs.rs/tss-rust)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/tss.svg)](./LICENSE)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/lmmx/tree-sitter-symbols/master.svg)](https://results.pre-commit.ci/latest/github/lmmx/tree-sitter-symbols/master)

Tree-sitter symbol node enums and metadata, generated at build time.

## Overview

The `tss-macros` crate in this repo generates enums and structs mapping tree-sitter language grammars. This means that downstream usage can replace string literal comparisons like `node.kind() == "function_item"` with type-safe comparison.

## Usage

```toml
[dependencies]
tss-macros = "0.1"

tree-sitter = "0.24"
# your language here
tree-sitter-rust = "0.24"
```

```rust
use tree_sitter_symbols::generate_nodes;
use std::str::FromStr;

#[generate_nodes(tree_sitter_rust)]
enum RustNodes {}

let node_type = RustNodes::from_str("function_item")?;
assert_eq!(node_type, RustNodes::FunctionItem);
assert_eq!(node_type.to_string(), "function_item");
```
## How it works

The `generate_nodes` macro reads `src/node-types.json`, which is found in every `tree-sitter-*` crate. This file describes all the possible nodes that the parser can create, and their relationship to each other.

During expansion, these nodes are translated into:
- *Many* enums and structs representing all node types. These new types maintain the visibility and attributes of the parent node.
- `FromStr` implementation for parsing node type strings into type-safe representations
- `Display` implementation for converting back to strings

See [the schema](https://github.com/lmmx/isotarp/blob/master/crates/tss-rust/codegen/schema.rs)
for what specifically it extracts from the `NODE_TYPES`. This schema was generated using
[genson-cli](https://docs.rs/genson-cli) as JSON schema then Rust (serde) structs generated
through the [app.quicktype.io](https://app.quicktype.io/?l=rust) web app.

## Licensing

MIT licensed - see [LICENSE](https://github.com/lmmx/tree-sitter-symbols/blob/master/LICENSE) for details.
