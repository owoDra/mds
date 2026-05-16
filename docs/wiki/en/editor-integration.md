# Editor Integration (LSP)

> *This page was translated from [Japanese](../ja/editor-integration.md) by AI.*

mds provides `mds-lsp`, a Language Server Protocol server for authoring-v2 Markdown packages.

## Main Features

| Feature | Description |
| --- | --- |
| Diagnostics | Section structure, canonical roots, legacy-table warnings, source/test mixing, unresolved wiki-style links |
| Hover and Definition | Prefer generated-file delegation, then remap results back to Markdown with the source map bridge |
| Completion and Snippets | Current source/test headings, code fences, and config snippets |
| Code Actions | Add missing source/test headings for tableless docs |
| Outline | Document symbols for Markdown headings |

## Installation

### VS Code

```bash
code --install-extension owo-x-project.mds
```

The Marketplace extension bundles the matching `mds-lsp` binary.

### Other editors

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh
```

This installs both `mds` and `mds-lsp`.

### Build from source

```bash
cargo build -p mds-lsp --release
```

When working inside this repository, build `mds-lsp` from the root Cargo workspace. No self-hosted sync step is involved.

## VS Code Settings

| Setting | Default | Meaning |
| --- | --- | --- |
| `mds.lsp.path` | `""` | Explicit path to `mds-lsp`; bundled server is preferred first |
| `mds.lsp.enabled` | `true` | Enables or disables the language server |
| `mds.lsp.logLevel` | `"info"` | Server log level |
| `mds.lsp.additionalLanguages` | `[]` | Extra `*.lang.md` suffixes for editor features |

## Activation Model

The extension activates when a workspace contains `mds.config.toml`. It watches the canonical `.mds/source` and `.mds/test` roots by default, and `mds.lsp.additionalLanguages` can extend editor-only file matching for extra suffixes.

## Generated-File Bridge

`mds-lsp` stores source maps for generated outputs. The VS Code extension uses bridge commands to:

- delegate hover and definition to generated files when that gives better language-service results
- remap the resulting ranges back to the owning Markdown code fence
- mirror generated diagnostics back onto indexed Markdown documents

Completion still falls back to shadow-document techniques when delegation is not the right fit.

## Other Editors

Use `mds-lsp` over stdio with `mds.config.toml` as the root marker.

Recommended file coverage:

- `.mds/source/**/*.md`
- `.mds/test/**/*.md`
- `mds.config.toml`
- `package.md`

## Local Development

```bash
cd editors/vscode
npm install
npm run compile
```

Then run the extension in VS Code's Extension Development Host.

## Troubleshooting

- If diagnostics do not appear, confirm that the file is under `.mds/source` or `.mds/test`.
- If bridge results look stale, rebuild the package or reopen the workspace so the index refreshes.
- If you override `mds.lsp.path`, confirm the selected binary matches the extension version.