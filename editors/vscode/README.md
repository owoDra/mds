# mds — Markdown Source (VS Code Extension)

Language support for [mds](https://github.com/owo-x-project/owox-mds) implementation files (`.{lang}.md` — e.g. `.ts.md`, `.py.md`, `.go.md`).

## Features

- **Syntax highlighting** for mds Markdown files with embedded code blocks
- **Language Server** integration via `mds-lsp` for real-time diagnostics
- **Markdown preview** commands for mds implementation Markdown files
- **Snippets** for common mds section structures
- **Code block detection** with embedded language support through stable virtual documents
- **Authoring assistance** for `Imports`, `Exports`, H5 shared definitions, and quick fixes
- **Any language** — automatically detects all `.{ext}.md` files in `src-md/` and `.mds/source/`, plus test docs under `.mds/test/`

## Requirements

- Marketplace builds include the matching `mds-lsp` binary for the current platform
- Optional: install `mds-lsp` separately only when using a custom `mds.lsp.path` or local development build

## Configuration

| Setting | Default | Description |
| --- | --- | --- |
| `mds.lsp.enabled` | `true` | Enable/disable the mds language server |
| `mds.lsp.path` | `""` | Path to the mds-lsp binary. If empty, uses the bundled server first, then PATH |
| `mds.lsp.logLevel` | `"info"` | Log level for the language server |
| `mds.lsp.additionalLanguages` | `[]` | Additional language extensions (e.g., `.go.md`) |

## License

MIT — see [LICENSE](../../LICENSE).
