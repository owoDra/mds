# Getting Started

> *This page was translated from [Japanese](../ja/getting-started.md) by AI.*

This page explains the prerequisites for trying mds and the basic execution steps.

## Installation

Choose according to your language ecosystem. Installing any one of these will make the `mds` command available.

### Rust (cargo)

```bash
cargo install mds-cli
```

### Node.js (npm)

```bash
npm install -g @owox-mds/cli
```

### Python (pip / uv)

```bash
pip install mds-cli
# or
uvx mds-cli
```

### VSCode Extension

Search for **"mds"** in the Marketplace, or install with the following command.

```bash
code --install-extension owo-x-project.mds
```

## Prerequisites

mds is a tool under development. It is currently released as an alpha version.

## Required Runtime Environment

Rust is required for basic checking and generation.

| Purpose | Requirements |
| --- | --- |
| Running mds commands | Rust 1.86 or later |
| TypeScript checking, fixing, testing | Node.js 24 or later, plus your chosen ESLint, Prettier, Biome, Vitest, Jest, etc. |
| Python checking, fixing, testing | Python 3.13 or later, plus your chosen Ruff, Black, Pytest, unittest, etc. |
| Rust checking, fixing, testing | Rust 1.86 or later, Cargo, plus your chosen rustfmt, Clippy, cargo-nextest, etc. |

`mds check` and `mds build` handle Markdown structure and generation. `mds lint` and `mds test` use the checking tools and test runners selected for each target language. Tools that are not selected are not implicitly required.

## Minimal Setup

Prepare the following files for an mds target package.

| File | Role |
| --- | --- |
| `mds.config.toml` | Configures mds activation, input source, output destination, and language adapters. |
| `package.md` | Describes the package name, dependencies, and per-package rules. |
| `src-md/**/*.ts.md` | Implementation Markdown for TypeScript. |
| `src-md/**/*.py.md` | Implementation Markdown for Python. |
| `src-md/**/*.rs.md` | Implementation Markdown for Rust. |
| `package.json`, `pyproject.toml`, `Cargo.toml` | Package information for the target language. |

You do not need to use all languages simultaneously. Enable only the languages you target.

## Basic Workflow

First, check the structure of the target package.

```bash
mds check --package ./path/to/package
```

Next, verify the generation plan and differences.

```bash
mds build --package ./path/to/package --dry-run
```

If there are no problems, write the derived code.

```bash
mds build --package ./path/to/package
```

## What Gets Generated

Files for the target language are generated from code blocks written in `Types`, `Source`, and `Test` sections of implementation Markdown.

For example, `src-md/foo/bar.ts.md` corresponds by default to the following files.

| Kind | Example Output |
| --- | --- |
| `Source` | `src/foo/bar.ts` |
| `Types` | `src/foo/bar.types.ts` |
| `Test` | `tests/foo/bar.test.ts` |

For details on output destinations, see [Generation Mechanism](generation.md).

## Next Pages to Read

- [Core Concepts](concepts.md)
- [Markdown Source](markdown-source.md)
- [Commands](commands.md)
- [Configuration](configuration.md)
