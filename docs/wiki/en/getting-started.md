# Getting Started

> *This page was translated from [Japanese](../ja/getting-started.md) by AI.*

This page explains the prerequisites for trying mds and the basic execution steps.

## Installation

Install the latest platform-specific binary from GitHub Releases with the one-liner script (recommended):

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh
```

This downloads the matching release archive for your OS / architecture and installs both `mds` and `mds-lsp` to `~/.local/bin` by default.

To pin a version, pass the release version without or with the leading `v`:

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh -s -- --version 0.2.1-alpha
```

### VSCode Extension

Search for **"mds"** in the Marketplace, or install with the following command.

```bash
code --install-extension owo-x-project.mds
```

The Marketplace extension is published as a platform-specific package and includes the matching `mds-lsp` binary. You only need to install `mds-lsp` separately when using another editor or when overriding `mds.lsp.path`.

## Prerequisites

mds is a tool under development. It is currently released as an alpha version.

## Required Runtime Environment

No runtime dependencies are required for the pre-built `mds` CLI binary.

| Purpose | Requirements |
| --- | --- |
| Running mds commands | None (pre-built binary from GitHub Releases) |
| TypeScript checking, fixing, testing | Node.js 24 or later, plus your chosen ESLint, Prettier, Biome, Vitest, Jest, etc. |
| Python checking, fixing, testing | Python 3.13 or later, plus your chosen Ruff, Black, Pytest, unittest, etc. |
| Rust checking, fixing, testing | Rust 1.86 or later, Cargo, plus your chosen rustfmt, Clippy, cargo-nextest, etc. |

`mds lint` and `mds build` handle Markdown structure and generation. `mds typecheck`, `mds lint`, and `mds test` use the selected type checker, linter, and test runner for each target language. Tools that are not selected are not implicitly required.

## Minimal Setup

Prepare the following files for an mds target package.

| File | Role |
| --- | --- |
| `mds.config.toml` | Configures mds activation, input source, output destination, and language adapters. |
| `package.md` | Describes the package name, dependencies, and per-package rules. |
| `src-md/**/*.ts.md` | Implementation Markdown for TypeScript. |
| `src-md/**/*.py.md` | Implementation Markdown for Python. |
| `src-md/**/*.rs.md` | Implementation Markdown for Rust. |
| Recognized package manager metadata such as `package.json`, `pyproject.toml`, `Cargo.toml`, `pubspec.yaml`, `*.csproj`, `CMakeLists.txt` | Package manager metadata required by `mds init` and package detection. |

You do not need to use all languages simultaneously. Enable only the languages you target.

## Basic Workflow

First, lint the structure of the target package and configured code blocks.

```bash
mds lint --package ./path/to/package
```

If the package configures a type checker, run it next.

```bash
mds typecheck --package ./path/to/package
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
