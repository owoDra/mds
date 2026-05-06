# src/capabilities/mod.rs

## Purpose

Migrated implementation source for `src/capabilities/mod.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/capabilities/mod.rs`.

## Exports

| Name | Visibility | Summary |
| --- | --- | --- |
| capabilities-root | public | LSP capabilities module surface. |

## Source


##### capabilities-root

Exports all mds LSP capability provider modules from one root.

````rs
pub mod code_action;
pub mod completion;
pub mod diagnostics;
pub mod hover;
pub mod navigation;
pub mod symbols;
````
