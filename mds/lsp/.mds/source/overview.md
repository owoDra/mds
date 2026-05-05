# mds-lsp

## Purpose

Language Server Protocol package for mds editor integration.

## Architecture

This package is authored under `mds/lsp/.mds/source/` and synchronized into package `src/` / `tests/` by `mds build`. The same command also refreshes the repo-local self-hosted mirror under `.build/rust/mds/lsp/`. Package metadata is read from `../Cargo.toml`; mds does not use a package root `index.md`.

### Package Summary

| Name | Version |
| --- | --- |
| mds-lsp | 0.2.1-alpha |

### Dependencies

| Name | Version | Summary |
| --- | --- | --- |
| mds-core | 0.2.1-alpha |  |
| serde | 1 |  |
| serde_json | 1 |  |
| tokio | 1 |  |
| toml | 0.8 |  |
| tower-lsp | 0.20 |  |
| tracing | 0.1 |  |
| tracing-subscriber | 0.3 |  |

### Dev Dependencies

| Name | Version | Summary |
| --- | --- | --- |
| tempfile | 3 |  |

## Rules

- Keep package-level source design in this overview.
- Keep package-level Imports / Exports in `lib.rs.md`.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `../src`, `../tests`, or `.build/rust/mds-lsp`.