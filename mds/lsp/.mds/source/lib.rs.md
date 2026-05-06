# src/lib.rs

## Purpose

Migrated implementation source for `src/lib.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/lsp/src/lib.rs`.

## Exports

| Name | Visibility | Summary |
| --- | --- | --- |
| lsp-root | public | LSP package root module surface. |

## Source


##### lsp-root

Exports capabilities, conversion helpers, labels, server, and workspace state modules.

````rs
pub mod capabilities;
pub mod convert;
pub mod labels;
pub mod server;
pub mod state;
````
