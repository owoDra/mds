# Descriptor Guide

This page explains the built-in and workspace descriptor directories used by mds.

## Directory Layout

| Scope | Path | Purpose |
| --- | --- | --- |
| Built-in language descriptors | `mds/core/src/descriptors/languages/base/` | Base language rules such as file mapping and syntax |
| Built-in framework overlays | `mds/core/src/descriptors/languages/overlays/` | Framework-specific overlays such as Flutter or Rails |
| Built-in quality tools | `mds/core/src/descriptors/tools/` | Lint, typecheck, and test command manifests |
| Built-in package managers | `mds/core/src/descriptors/package-managers/` | Metadata files, lockfiles, and command recommendations |
| Workspace overrides | `.mds/descriptors/` | Per-repository overrides for languages, tools, and package managers |

## Language Descriptors

Language descriptors define:

- Markdown file suffix matching
- Output file naming rules for `Source`, `Types`, and `Test`
- Syntax hints for imports, top-level declarations, comments, and doc comments
- Default quality commands and tool profiles

Framework overlays are separate descriptors that share the same schema but live under `languages/overlays/`.

## Quality Tool Descriptors

Quality tool descriptors are grouped under:

- `tools/lint/`
- `tools/typecheck/`
- `tools/test/`

Each descriptor maps one or more command prefixes to:

- input mode
- output mode
- diagnostic capture regex

Add a new TOML file when you need a new runner or parser.

## Imports Table Shape

The current canonical Imports columns are:

- `Kind`
- `From`
- `Target`
- `Symbols`
- `Via`
- `Summary`
- `Code`

`Code` is kept as a renderer fallback while descriptors absorb more language-specific import synthesis.

## Package Manager Descriptors

Package manager descriptors define:

- metadata files required for detection
- lockfiles used to rank matches
- command recommendations for install, build, typecheck, lint, and test
- metadata reader kind used by `mds init` and package sync

Examples include npm, pnpm, yarn, bun, cargo, uv, poetry, bundler, pub, Flutter pub, dotnet, CMake, Meson, Conan, vcpkg, and Zig.

## Workspace Overrides

Place overrides in one of the following directories:

- `.mds/descriptors/languages/`
- `.mds/descriptors/tools/`
- `.mds/descriptors/package-managers/`

Workspace descriptors override built-ins by id or alias.