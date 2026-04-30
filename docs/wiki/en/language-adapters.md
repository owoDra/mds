# Language Adapters

> *This page was translated from [Japanese](../ja/language-adapters.md) by AI.*

This page explains the role of language adapters in mds.

## Role

Language adapters are components that handle language-specific differences.

The core processing of mds is responsible for Markdown reading, structural validation, and generation planning. Output rules and connections to inspection tools specific to TypeScript, Python, and Rust are handled by language adapters.

## Responsibilities

Language adapters are primarily responsible for the following:

- Generating dependency declarations from `Uses`.
- Determining output filenames for `Source`, `Types`, and `Test`.
- Managing language-specific additional generated artifacts.
- Connecting to static analysis, auto-fix, and test execution commands.
- Mapping diagnostic results back to positions in the Markdown.

## TypeScript

For TypeScript, `*.ts.md` files are targeted.

Default generation examples:

| Type | Output Example |
| --- | --- |
| `Source` | `src/foo/bar.ts` |
| `Types` | `src/foo/bar.types.ts` |
| `Test` | `tests/foo/bar.test.ts` |

Dependency declarations are generated as TypeScript imports. Relative imports for internal dependencies are generated without file extensions.

## Python

For Python, `*.py.md` files are targeted.

Default generation examples:

| Type | Output Example |
| --- | --- |
| `Source` | `src/pkg/foo.py` |
| `Types` | `src/pkg/foo.pyi` |
| `Test` | `tests/pkg/test_foo.py` |

Internal dependencies are generated as absolute package imports relative to the generated source root.

## Rust

For Rust, `*.rs.md` files are targeted.

Default generation examples:

| Type | Output Example |
| --- | --- |
| `Source` | `src/foo/bar.rs` |
| `Types` | `src/foo/bar_types.rs` |
| `Test` | `tests/foo_bar_test.rs` |

For Rust, mds also handles mds-managed blocks for exposing generated modules.

## Relationship with Quality Inspection

Language adapters connect to the target language's inspection tools and test execution.

| Language | Inspection | Fix | Test |
| --- | --- | --- | --- |
| TypeScript | ESLint, Biome | Prettier, Biome | Vitest, Jest |
| Python | Ruff | Ruff, Black | Pytest, unittest |
| Rust | Cargo Clippy | rustfmt | Cargo test, cargo-nextest |

These tools are selected based on the features being used. Selections made during `mds init` are explicitly recorded in the `[quality.*]` section of `mds.config.toml`, and unselected tools are not implicitly executed.
