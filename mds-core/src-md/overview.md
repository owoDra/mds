# mds-core

## Purpose

Rust core library for parsing, validating, generating, and initializing mds projects.

## Architecture

This package is authored under `mds-core/src-md/` and synchronized into package `src/` / `tests/` and `.build/rust/mds-core/` before Cargo commands. Package metadata is read from `../Cargo.toml`; mds does not use a package root `index.md`.

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-core | ../../.build/rust/mds-core | Generated Cargo package. |

## Rules

- Keep package-level source design in this overview.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `../src`, `../tests`, or `.build/rust/mds-core`.
