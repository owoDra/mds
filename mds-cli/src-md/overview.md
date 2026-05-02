# mds-cli

## Purpose

Native CLI package for mds commands, including argument parsing, wizard flow, and command execution handoff to mds-core.

## Architecture

This package is authored under `mds-cli/src-md/` and synchronized into package `src/` / `tests/` and `.build/rust/mds-cli/` before Cargo commands. Package metadata is read from `../Cargo.toml`; mds does not use a package root `index.md`.

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-cli | ../../.build/rust/mds-cli | Generated Cargo package. |

## Rules

- Keep package-level source design in this overview.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `../src`, `../tests`, or `.build/rust/mds-cli`.
