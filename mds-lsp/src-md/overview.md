# mds-lsp

## Purpose

Language Server Protocol package for mds editor integration.

## Architecture

This package is authored under `mds-lsp/src-md/` and synchronized into package `src/` / `tests/` and `.build/rust/mds-lsp/` before Cargo commands. Package metadata is read from `../Cargo.toml`; mds does not use a package root `index.md`.

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-lsp | ../../.build/rust/mds-lsp | Generated Cargo package. |

## Rules

- Keep package-level source design in this overview.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `../src`, `../tests`, or `.build/rust/mds-lsp`.
