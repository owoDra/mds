# Markdown Source

> *This page was translated from [Japanese](../ja/markdown-source.md) by AI.*

This page explains the current Markdown document model used by mds.

## Canonical Roots

- `.mds/source` contains source docs and source overviews.
- `.mds/test` contains verification docs and test overviews.

These roots are fixed. They are part of the authoring model, not optional naming preferences.

## Document Kinds

| Path | Role | Expected sections |
| --- | --- | --- |
| `.mds/source/overview.md` | Source hierarchy overview | `Purpose`, `Architecture`, `Rules` |
| `.mds/test/overview.md` | Verification overview | `Purpose`, `Architecture`, `Rules` |
| `.mds/source/**/*.lang.md` | One feature or root module in a source tree | `Purpose`, `Contract`, `API`, optional `Source`, `Cases` |
| `.mds/test/**/*.md` | Executable verification for one feature or module | `Purpose`, `Covers`, `Cases`, `Test` |

Language root module docs such as `.mds/source/index.ts.md`, `.mds/source/lib.rs.md`, and `.mds/source/mod.rs.md` usually focus on `Purpose` and `API`, and add `Source` only when the root module owns runtime behavior.

## Source Docs

Source docs are the place for stable behavior, public surface notes, and generated source code.

- `Purpose` explains why the feature exists.
- `Contract` records stable inputs, outputs, constraints, and failure conditions.
- `API` describes public exports and entrypoints in prose.
- `Source` contains executable code fences that become source outputs.
- `Cases` records representative behavior.

A source doc can begin as prose only and gain a `Source` fence later.

## Test Docs

Test docs describe executable verification.

- `Purpose` explains the behavior being verified.
- `Covers` names the source module ids under test.
- `Cases` records the expectations.
- `Test` contains executable test fences.

Keep source behavior in source docs and executable verification in test docs. With the default check policy, mixing them is an error.
## Writing Public Interface

Functions, types, modules, and other public items are written in the `Exports` table of the root module or implementation Markdown.

Do not put `Imports`, `Exports`, or `Exposes` sections in `overview.md`; overview documents describe hierarchy purpose, architecture, metadata, and rules only.

## Important Constraints

- One implementation Markdown handles only one feature.
- Generated code is not the source of truth.
- Instead of directly editing generated code, edit the source Markdown.
- Implementation Markdown that contains only design descriptions is not treated as a complete implementation.
- The goal is not to interpret arbitrary free-form Markdown.
