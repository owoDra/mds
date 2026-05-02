# mds-core

## Purpose

Rust core library for parsing, validating, generating, and initializing mds projects.

## Architecture

This package is authored under `mds-core/.mds/source/` and synchronized into package `src/` / `tests/` and `.build/rust/mds-core/` before Cargo commands. Package metadata is read from `../Cargo.toml`; mds does not use a package root `index.md`.

<!-- mds:begin package-summary -->
| Name | Version |
| --- | --- |
| mds-core | 0.1.0-alpha.1 |
<!-- mds:end package-summary -->

<!-- mds:begin dependencies -->
| Name | Version | Summary |
| --- | --- | --- |
| serde | 1 |  |
| serde_json | 1 |  |
| toml | 0.8 |  |
<!-- mds:end dependencies -->

<!-- mds:begin dev-dependencies -->
| Name | Version | Summary |
| --- | --- | --- |
<!-- mds:end dev-dependencies -->

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-core | ../../.build/rust/mds-core | Generated Cargo package. |

## Rules

- Keep package-level source design in this overview.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `../src`, `../tests`, or `.build/rust/mds-core`.
