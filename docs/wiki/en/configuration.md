# Configuration

> *This page was translated from [Japanese](../ja/configuration.md) by AI.*

This page explains the role and main settings of `mds.config.toml`.

## Basic Policy

The mds configuration file name is `mds.config.toml`.

Rather than providing multiple configuration formats, it is fixed to TOML. This simplifies configuration interpretation and makes it easy to confirm where mds is enabled within a repository.

## Minimal Example

```toml
[package]
enabled = true

[roots]
markdown = "src-md"
source = "src"
types = "src"
test = "tests"

[adapters.ts]
enabled = true

[adapters.py]
enabled = false

[adapters.rs]
enabled = false

[quality.ts]
linter = "eslint"
fixer = "prettier --write"
test_runner = "vitest run"
required = ["node", "eslint", "prettier", "vitest"]
optional = []
```

## `[package]`

`[package]` configures per-package mds activation.

| Key | Meaning |
| --- | --- |
| `enabled` | Specifies whether this package is an mds target. |
| `allow_raw_source` | Specifies whether to allow raw source files not targeted for generation. |

Normally, specify `enabled = true` for packages you want to target with mds.

## `[roots]`

`[roots]` specifies the locations of Markdown and generation targets.

| Key | Default | Meaning |
| --- | --- | --- |
| `markdown` | `src-md` | Location for implementation Markdown. |
| `source` | `src` | Output destination for files generated from `Source`. |
| `types` | `src` | Output destination for files generated from `Types`. |
| `test` | `tests` | Output destination for files generated from `Test`. |

Generation targets must be within the target package. Output destinations that escape outside the package are rejected.

## `[adapters]`

`[adapters]` configures target language activation.

| Section | Target Language |
| --- | --- |
| `[adapters.ts]` | TypeScript |
| `[adapters.py]` | Python |
| `[adapters.rs]` | Rust |

Languages not in use can be set to `enabled = false`.

## Quality Check Settings

Commands used for per-language checking, fixing, and testing are handled in quality check settings. Quality tools selected via `mds init` are specified in `[quality.ts]`, `[quality.py]`, and `[quality.rs]`.

Representative candidates that can be selected are as follows.

| Language | Checking | Fixing | Testing |
| --- | --- | --- | --- |
| TypeScript | ESLint, Biome | Prettier, Biome | Vitest, Jest |
| Python | Ruff | Ruff, Black | Pytest, unittest |
| Rust | Cargo Clippy | rustfmt | Cargo test, cargo-nextest |

Features not in use can be set to `false`. For example, to disable TypeScript quality checks:

```toml
[quality.ts]
linter = false
fixer = false
test_runner = false
required = []
optional = []
```

If required tools are missing from the runtime environment, you can check with `mds doctor`. Unselected tools are not treated as missing.

## Configuration Notes

- The configuration file name is fixed to `mds.config.toml`.
- You cannot freely change mds semantics through section or key names.
- Unsupported settings may be treated as warnings.
- Settings where the generation target escapes outside the package are rejected.
- Maintaining conventions is prioritized over increasing flexibility through configuration.
