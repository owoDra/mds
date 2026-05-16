# Core Concepts

> *This page was translated from [Japanese](../ja/concepts.md) by AI.*

This page explains the concepts behind the current mds model.

## Source of Truth

In mds, Markdown is the primary artifact for package design, implementation, and verification. Generated source files are derived outputs, not the place to make the authoritative change.

## Source Docs

Source docs live under `.mds/source` and usually describe one feature or one root module.

They combine:

- intent in `Purpose`
- stable behavior in `Contract`
- public surface notes in `API`
- executable code in `Source`
- representative behavior in `Cases`

## Test Docs

Test docs live under `.mds/test` and hold executable verification.

They combine:

- verification intent in `Purpose`
- source module references in `Covers`
- expectations in `Cases`
- executable test code in `Test`

## Logical Module Id

Each source or test doc maps to a logical module id derived from its path inside the canonical root. mds uses that id for output planning and wiki-style links.

## Package Output Config

`mds.config.toml` separates authoring roots from output locations.

- `[roots]` fixes `.mds/source` and `.mds/test` while letting you choose output base directories.
- `[output]` and `[[output.override]]` decide the exact file paths.

This keeps authoring stable while allowing package-specific output conventions.

## Generated-File Bridge

mds records source maps during generation planning. `mds-lsp` can use those source maps to send editor operations through generated files and then remap the result back to the Markdown range that owns the code fence.

## Package Boundary

Package-manager metadata such as `package.json`, `pyproject.toml`, and `Cargo.toml` stays authoritative for package-manager behavior. `package.md` is the mds-facing package document, and `.mds/source` plus `.mds/test` hold feature-level authoring.