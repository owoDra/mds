# mds-lsp

## Purpose

Language Server Protocol implementation for mds Markdown files.

## Architecture

This package is authored under `src-md/mds-lsp/` and synchronized into `.build/rust/mds-lsp/` before Cargo commands.

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-lsp | .build/rust/mds-lsp | Generated Cargo package. |

## Rules

- Put directory-level design in the nearest `index.md`.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `.build/rust/mds-lsp`.
