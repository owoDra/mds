# mds

> *This document was translated from [Japanese](README.ja.md) by AI.*

mds is a development toolchain that treats Markdown as the source of truth for both design and implementation.

Write real code — TypeScript, Python, Rust — inside Markdown code blocks, then extract them as executable source files with `mds build`. Because the code in Markdown is the actual running code, design intent and implementation always stay in sync.

## Features

- Generate `.ts`, `.py`, `.rs` files from `Types`, `Source`, `Test` code blocks in Markdown
- `mds check` validates Markdown structure and consistency
- `mds lint` / `mds test` runs linters and tests against code in Markdown
- `mds init` initializes a project with an interactive wizard

## Quick Start

```bash
# Install (pick one)
cargo install mds-cli          # Rust
npm install -g @owox-mds/cli   # Node.js
pip install mds-cli            # Python

# Basic usage
mds init --package ./path/to/package
mds check --package ./path/to/package
mds build --package ./path/to/package
```

VS Code extension: `code --install-extension owo-x-project.mds`

See [examples/](examples/) for minimal working configurations.

## Requirements

- Rust 1.86+ (required)
- Node.js 24+ (for TypeScript)
- Python 3.13+ (for Python)

## Documentation

| Audience | Entry point |
| --- | --- |
| **Users** | [Wiki (EN)](docs/wiki/en/index.md) — Getting started, commands, configuration, generation, troubleshooting |
| **Contributors** | [CONTRIBUTING.md](CONTRIBUTING.md) — Setup, dev workflow, testing |

[日本語版 README](README.ja.md) | [日本語 Wiki](docs/wiki/ja/index.md)

### Key links

- [Getting Started](docs/wiki/en/getting-started.md) — Prerequisites and minimal setup
- [Commands](docs/wiki/en/commands.md) — Full command reference
- [Development Guide](docs/wiki/en/development.md) — Build, test, debug
- [AI Agent Integration](docs/wiki/en/ai-agent-integration.md) — Claude Code, Codex, Opencode, GitHub Copilot
- [Editor Integration (LSP)](docs/wiki/en/editor-integration.md) — VS Code extension, Neovim, real-time diagnostics

## Contributing

Bug reports, documentation improvements, and implementation improvements are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

MIT License. See [LICENSE](LICENSE) for details.
