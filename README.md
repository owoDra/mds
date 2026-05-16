<p align="center">
	<img src=".github/assets/readme-header.png" alt="markdown source" width="720">
</p>

# mds

> *This document was translated from [Japanese](README.ja.md) by AI.*

mds is a development toolchain that treats Markdown as the source of truth for package design, source code, and executable verification.

Use tableless authoring under `.mds/source` and `.mds/test`, describe the public surface in prose, write runnable code in `Source` or `Test` fences, and generate package outputs with `mds build`. `mds-lsp` can remap diagnostics, hover, and definition results from generated files back to the Markdown that owns them.

## Features

- Canonical authoring roots: `.mds/source` for source docs and `.mds/test` for verification docs
- Tableless authoring-v2: source docs use `Purpose`, `Contract`, `API`, `Source`, `Cases`; test docs use `Purpose`, `Covers`, `Cases`, `Test`
- Package output planning with `[roots]`, `[output]`, and `[[output.override]]`
- `mds init`, `mds new`, and `mds init --ai` scaffold current projects, docs, and agent kits
- `mds-lsp` provides diagnostics, snippets, and generated-file bridge navigation

## Quick Start

```bash
# Install the latest GitHub Releases binary
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh

# Initialize or inspect a package
mds init --package ./path/to/package
mds lint --package ./path/to/package
mds build --package ./path/to/package --dry-run
mds build --package ./path/to/package
```

VS Code extension: `code --install-extension owo-x-project.mds`

The installer downloads the platform-specific GitHub Releases archive and installs both `mds` and `mds-lsp`. The Marketplace VS Code extension already includes the matching `mds-lsp` binary, so VS Code users do not need a separate LSP install.

## Minimal Package Layout

```text
my-package/
├── mds.config.toml
├── package.md
├── package.json
├── .mds/
│   ├── source/
│   │   ├── overview.md
│   │   └── greet.ts.md
│   └── test/
│       ├── overview.md
│       └── greet.ts.md
├── src/
└── tests/
```

See [examples/](examples/) for working minimal configurations.

## Requirements

No runtime dependencies are required for the prebuilt `mds` CLI binary. Language-specific checks still use the toolchain configured for that language, such as Node.js, Python, or Rust.

## Documentation

| Audience | Entry point |
| --- | --- |
| **Users** | [Wiki (EN)](docs/wiki/en/index.md) - Getting started, configuration, generation, editor support, troubleshooting |
| **Contributors** | [CONTRIBUTING.md](CONTRIBUTING.md) - Setup, dev workflow, testing |

[日本語版 README](README.ja.md) | [日本語 Wiki](docs/wiki/ja/index.md)

### Key links

- [Getting Started](docs/wiki/en/getting-started.md) - Install mds and set up a package
- [Configuration](docs/wiki/en/configuration.md) - Canonical roots, output patterns, and checks
- [Markdown Source](docs/wiki/en/markdown-source.md) - Current source/test document model
- [Generation Mechanism](docs/wiki/en/generation.md) - Output planning, overrides, and manifests
- [Commands](docs/wiki/en/commands.md) - CLI reference
- [AI Agent Integration](docs/wiki/en/ai-agent-integration.md) - Agent kit generation and template maintenance
- [Editor Integration (LSP)](docs/wiki/en/editor-integration.md) - VS Code extension, other editors, generated-file bridge
- [Development Guide](docs/wiki/en/development.md) - Build, test, and debug the repository

## Contributing

Bug reports, documentation improvements, and implementation improvements are welcome. First-party repository development edits the checked-in source and test trees under `mds/` and `editors/vscode/` directly, then validates with Cargo or npm as appropriate. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

MIT License. See [LICENSE](LICENSE) for details.
