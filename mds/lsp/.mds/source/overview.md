# mds-lsp

## Purpose

Language Server Protocol package for mds editor integration.

## Architecture

This package is authored under `mds/lsp/.mds/source/` and synchronized into package `src/` / `tests/` and `.build/rust/mds/lsp/` before Cargo commands. Package metadata is read from `../Cargo.toml`; mds does not use a package root `index.md`.

### Package Summary

| Name | Version |
| --- | --- |
| mds-lsp | 0.1.0-alpha.1 |

### Dependencies

| Name | Version | Summary |
| --- | --- | --- |
| mds-core | 0.1.0-alpha.1 |  |
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

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-lsp | ../../.build/rust/mds-lsp | Generated Cargo package. |

## Rules

- Keep package-level source design in this overview.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `../src`, `../tests`, or `.build/rust/mds-lsp`.