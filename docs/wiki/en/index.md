# mds English Wiki

> *This page was translated from [Japanese](../ja/index.md) by AI.*

This wiki documents the current authoring-v2 surface of mds.

mds uses canonical source and verification roots, tableless Markdown documents, package output planning, and editor tooling that remaps generated-file results back to Markdown. Legacy metadata-table workflows are not the live model for this wiki.

## Start Here

1. [Getting Started](getting-started.md) - Install mds and prepare a package
2. [Configuration](configuration.md) - Canonical roots, output patterns, and checks
3. [Markdown Source](markdown-source.md) - Current source and test document shapes
4. [Commands](commands.md) - CLI reference and common flows
5. [Generation Mechanism](generation.md) - How outputs are planned and written

## Core Topics

- [Core Concepts](concepts.md) - Source of truth, logical modules, generated-file bridge
- [Quality Checks](quality.md) - Structural checks, tool execution, check policies
- [Troubleshooting](troubleshooting.md) - Common authoring-v2 failures and fixes
- [Editor Integration (LSP)](editor-integration.md) - VS Code extension and other editors
- [AI Agent Integration](ai-agent-integration.md) - Agent kit generation and template upkeep

## Additional Guides

- [Monorepo Usage](monorepo.md) - Package-by-package adoption in larger repositories
- [Package Info Sync](package-sync.md) - How `mds package sync` updates managed package metadata
- [Distribution Policy](distribution.md) - CLI, installer, and editor distribution
- [Contributing](contributing.md) - Reporting and proposal workflow
- [Development Guide](development.md) - Build, test, and debug this repository
- [LSP Development Guide](lsp-development.md) - Extension and server implementation notes
- [Roadmap](roadmap.md) - Current focus and follow-up areas

## Current Live Model

- Source docs live under `.mds/source`; verification docs live under `.mds/test`.
- Source docs use `Purpose`, `Contract`, `API`, `Source`, and `Cases`.
- Test docs use `Purpose`, `Covers`, `Cases`, and `Test`.
- Output paths come from `[roots]`, `[output]`, and optional `[[output.override]]`.
- `mds-lsp` can bridge hover, definition, and diagnostics from generated files back to Markdown.

## Page List

| Page | Description |
| --- | --- |
| [Getting Started](getting-started.md) | Installation, minimal package layout, first commands |
| [Configuration](configuration.md) | `mds.config.toml`, canonical roots, output patterns, checks |
| [Markdown Source](markdown-source.md) | Source/test docs, overview docs, root module docs |
| [Commands](commands.md) | `init`, `new`, `build`, `lint`, `typecheck`, `test`, `doctor`, `package sync` |
| [Generation Mechanism](generation.md) | Logical modules, default outputs, overrides, manifests |
| [Core Concepts](concepts.md) | Source of truth, output planning, package boundaries |
| [Quality Checks](quality.md) | Structural diagnostics, selected tools, check policy |
| [Troubleshooting](troubleshooting.md) | Common failures and confirmation steps |
| [Editor Integration (LSP)](editor-integration.md) | Bundled VS Code extension and other editor setup |
| [AI Agent Integration](ai-agent-integration.md) | Agent kit generation and template maintenance |
| [Monorepo Usage](monorepo.md) | Per-package enablement and safe output boundaries |
| [Package Info Sync](package-sync.md) | Managed package metadata synchronization |
| [Distribution Policy](distribution.md) | Release binaries, installer, and editor packages |
| [Contributing](contributing.md) | Contribution policy |
| [Development Guide](development.md) | Repository build, test, and debug workflow |
| [LSP Development Guide](lsp-development.md) | Internal notes for the editor stack |
| [Roadmap](roadmap.md) | Current focus areas |