# src/lib.rs

## Purpose

Migrated implementation source for `src/lib.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/cli/src/lib.rs`.

## Exports

| Name | Visibility | Summary |
| --- | --- | --- |
| cli-root | public | CLI package root module surface. |

## Source


##### cli-root

Exports the CLI argument parser and interactive wizard modules.

````rs
pub mod args;
pub mod wizard;
````
