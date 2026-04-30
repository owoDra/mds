# mds — Markdown Source (VS Code Extension)

Language support for [mds](https://github.com/owo-x-project/owox-mds) implementation files (`.{lang}.md` — e.g. `.ts.md`, `.py.md`, `.go.md`).

## Features

- **Syntax highlighting** for mds Markdown files with embedded code blocks
- **Language Server** integration via `mds-lsp` for real-time diagnostics
- **Snippets** for common mds section structures
- **Code block detection** with proper embedded language support
- **Any language** — automatically detects all `.{ext}.md` files in `src-md/`

## Requirements

- VS Code 1.85+
- `mds-lsp` binary installed:
  ```bash
  curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh
  # or
  cargo install --git https://github.com/owo-x-project/owox-mds mds-lsp
  ```

## Configuration

| Setting | Default | Description |
| --- | --- | --- |
| `mds.lsp.enabled` | `true` | Enable/disable the mds language server |
| `mds.lsp.path` | `""` | Path to the mds-lsp binary (uses PATH if empty) |
| `mds.lsp.logLevel` | `"info"` | Log level for the language server |
| `mds.lsp.additionalLanguages` | `[]` | Additional language extensions (e.g., `.go.md`) |

## License

MIT — see [LICENSE](../../LICENSE).
