# Monorepo Usage

> *This page was translated from [Japanese](../ja/monorepo.md) by AI.*

This page explains the approach to using mds in repositories containing multiple packages.

## Basic Policy

mds determines targets on a per-package basis.

Even if a single repository contains a mix of TypeScript, Python, and Rust packages, you can decide whether to enable mds for each package individually.

## Target Package Detection

mds determines target packages primarily from the following information.

| Information | Role |
| --- | --- |
| `mds.config.toml` | Determines whether mds is enabled. |
| `package.md` | Confirms per-package description and sync targets. |
| `package.json` | Reads Node.js package information. |
| `pyproject.toml` | Reads Python package information. |
| `Cargo.toml` | Reads Rust package information. |

Non-target packages are never automatically included in generation or modification.

## Specifying `--package`

To target only a specific package, specify `--package`.

```bash
mds lint --package packages/example
```

If `--package` is omitted, mds searches for enabled packages under the current directory.

## Multi-Language Handling

Even if a single repository contains multiple languages, language-specific differences are handled by language adapters.

| Language | Primary Detection Targets |
| --- | --- |
| TypeScript | `*.ts.md`, `package.json` |
| Python | `*.py.md`, `pyproject.toml` |
| Rust | `*.rs.md`, `Cargo.toml` |

Even within the same repository, enabled languages can differ per package.

## Generation Target Safety

mds raises an error if a generation target escapes outside the target package.

Additionally, it does not overwrite existing files that lack an mds management header. This protects other packages and hand-written files within the same repository.

## Recommended Practices for Monorepos

- Place an `mds.config.toml` in each package.
- Prepare a `package.md` for each package.
- Separate implementation Markdown into the default `src-md` directory.
- Avoid mixing responsibilities between generated code and hand-written code.
- Verify the generation plan with `mds build --dry-run` on the first run.
- Run `mds lint` first for continuous inspection.
