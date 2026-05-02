# Editor Integration (LSP)

> *This page was translated from [Japanese](../ja/editor-integration.md) by AI.*

mds provides `mds-lsp`, a server compliant with the Language Server Protocol (LSP). You can use real-time validation, code navigation, completion, and hover information for mds Markdown files in your editor.

**Key features:**

| Feature | Description |
| --- | --- |
| Real-time diagnostics | Section structure, table format, language matching, config validation, link validation |
| Go to Definition | Jump from Uses table Target to the referenced implementation Markdown |
| Find References | Search where Exposed names are Used |
| Document Symbols | Outline display of section headings |
| Workspace Symbols | Search module names across all `src-md/` |
| Completion | Section names, table column names, code block languages, snippets |
| Hover | Section descriptions, Purpose display of referenced modules |
| Code Action | Auto-add missing sections (Quick Fix) |

## Installation

### install.sh (recommended)

The install script installs both `mds` and `mds-lsp`:

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh
```

### Build from source (for developers)

```bash
./.github/script/sync-build.sh
cd .build/rust
cargo build -p mds-lsp --release
cp target/release/mds-lsp /usr/local/bin/
```

### Verify installation

```bash
mds-lsp --version   # Display version (if not supported, starts waiting on stdio)
```

`mds-lsp` operates with stdio transport. The editor automatically manages the process on startup.

## VSCode

### Installing the extension

```bash
cd editors/vscode
npm install
npm run compile
```

During development, launch via VSCode's "Extension Development Host":

1. Open `editors/vscode` in VSCode
2. Press F5 to launch Extension Development Host
3. Open `.ts.md`, `.py.md`, `.rs.md` files

### Configuration options

| Setting | Default | Description |
| --- | --- | --- |
| `mds.lsp.path` | `""` | Path to the mds-lsp binary. If empty, searches PATH |
| `mds.lsp.enabled` | `true` | Enable/disable the LSP server |
| `mds.lsp.logLevel` | `"info"` | Log level: error, warn, info, debug, trace |
| `mds.lsp.additionalLanguages` | `[]` | Additional language extensions (e.g., `[".go.md", ".java.md"]`) |

### Auto-activation

The extension is automatically activated when `mds.config.toml` exists in the workspace. It detects enabled language adapters from the `[adapters]` section of `mds.config.toml` and provides LSP features for files with the corresponding extensions.

When adding a new language, simply add the extension to `mds.lsp.additionalLanguages`.

### Syntax highlighting

mds-specific TextMate grammar is included:

- **Section headings**: `## Purpose`, `## Types`, etc. are highlighted
- **Uses table**: `builtin`, `package`, `workspace`, `internal` in the `From` column are highlighted as keywords
- **Code blocks**: Embedded syntax highlighting for TypeScript, Python, Rust

### Snippets

VSCode snippets are also included:

| Prefix | Description |
| --- | --- |
| `mds-doc` | Complete implementation document template |
| `mds-uses` | Uses table (with header) |
| `mds-use-row` | One row of Uses table |
| `mds-section` | Section heading |
| `mds-code-section` | Code section (Uses + code block) |
| `mds-code` | Code block |
| `mds-config` | Basic mds.config.toml |
| `mds-config-full` | mds.config.toml with all sections |

## Neovim

### Configuration example with nvim-lspconfig

```lua
local lspconfig = require('lspconfig')
local configs = require('lspconfig.configs')

if not configs.mds_lsp then
  configs.mds_lsp = {
    default_config = {
      cmd = { 'mds-lsp' },
      filetypes = { 'markdown' },
      root_dir = lspconfig.util.root_pattern('mds.config.toml'),
      settings = {},
    },
  }
end

lspconfig.mds_lsp.setup({
  on_attach = function(client, bufnr)
    -- Set up keymaps as needed
  end,
})
```

### Filetype configuration

```lua
vim.filetype.add({
  extension = {
    ['ts.md'] = 'markdown',
    ['py.md'] = 'markdown',
    ['rs.md'] = 'markdown',
  },
})
```

## Helix

Add the following to `languages.toml`:

```toml
[[language]]
name = "markdown"
language-servers = ["mds-lsp"]
file-types = ["md"]

[language-server.mds-lsp]
command = "mds-lsp"
```

## Other Editors

mds-lsp conforms to the standard LSP protocol (stdio transport) and can be used with any editor that supports LSP.

Required configuration:

1. Command: `mds-lsp`
2. Transport: stdio
3. Root pattern: `mds.config.toml`
4. File types: `*.ts.md`, `*.py.md`, `*.rs.md`, `mds.config.toml`, `package.md`

## LSP Feature List

### Phase 1: Real-time Diagnostics

Validation is automatically executed when files are opened or edited.

| Validation Target | Content |
| --- | --- |
| Section structure | Existence check for Purpose, Contract, Types, Source, Cases, Test |
| Table format | Validation of From, Target, Expose, Summary columns in Uses table |
| Language matching | Match between file extension (.ts.md, etc.) and code block language label |
| Code blocks | Check for prohibited import/use/require statements |
| Markdown links | Existence check for local link targets |
| mds.config.toml | TOML syntax, required fields, supported keys validation |
| package.md | Section structure, Package table validation |

### Phase 2: Code Navigation

| Feature | Description |
| --- | --- |
| Go to Definition | Jump from Uses table Target cell to the corresponding `.{lang}.md` file |
| Find References | Search which implementation Markdown's Uses references the current file |
| Document Symbols | Display `##` / `###` headings as symbols in outline |
| Workspace Symbols | Search all module names under `src-md/` |

### Phase 3: Code Assist

| Feature | Description |
| --- | --- |
| Section name completion | Show candidates like Purpose, Contract after `## ` |
| Table column completion | Show candidates like From, Target within table rows |
| Code block language completion | Show language candidates inferred from file after ` ``` ` |
| Snippets | Document templates, Uses rows, code blocks |
| Hover | Display section heading descriptions, Purpose of Uses targets |
| Code Action | Auto-add missing sections (Quick Fix) |

## Troubleshooting

### LSP server does not start

```bash
# Check if mds-lsp is in PATH
which mds-lsp

# Manual startup test (Ctrl+C to exit)
mds-lsp
```

### Checking logs

In VSCode, select the "mds Language Server" channel in the Output panel to check logs.

To change the log level:

```json
{
  "mds.lsp.logLevel": "debug"
}
```

### Diagnostics not showing

1. Check that `mds.config.toml` exists and has `enabled = true`
2. Check that the file is under the `src-md/` directory
3. Check that the file extension is one of `.ts.md`, `.py.md`, `.rs.md`
