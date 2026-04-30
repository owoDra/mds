# mds — Markdown Design System (VS Code Extension)

Language support for [mds](https://github.com/owo-x-project/mds) implementation files (`.ts.md`, `.py.md`, `.rs.md`).

## Features

- **Syntax highlighting** for mds Markdown files with embedded TypeScript, Python, and Rust code blocks
- **Language Server** integration via `mds-lsp` for real-time diagnostics
- **Snippets** for common mds section structures
- **Code block detection** with proper embedded language support

## Requirements

- VS Code 1.85+
- [mds CLI](https://github.com/owo-x-project/mds) installed (`cargo install mds-cli` or via npm `mds-cli`)
- For LSP support: `mds-lsp` binary on PATH (installed via `cargo install mds-lsp`)

## Configuration

| Setting | Default | Description |
| --- | --- | --- |
| `mds.lsp.enabled` | `true` | Enable/disable the mds language server |
| `mds.lsp.path` | `""` | Path to the mds-lsp binary (uses PATH if empty) |
| `mds.lsp.logLevel` | `"info"` | Log level for the language server |
| `mds.lsp.additionalLanguages` | `[]` | Additional language extensions (e.g., `.go.md`) |

## License

MIT — see [LICENSE](../../LICENSE).
