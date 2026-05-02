# LSP Development Guide

> *This page was translated from [Japanese](../ja/lsp-development.md) by AI.*

This page explains how to develop, debug, test, and add features to mds-lsp (Language Server Protocol server).

For usage instructions of mds-lsp, see [Editor Integration (LSP)](editor-integration.md).

## Architecture Overview

mds-lsp is composed of the following components.

```
src-md/mds/lsp/
├── src/
│   ├── main.rs.md        # Entry point (stdio transport startup)
│   ├── lib.rs.md         # Library root
│   ├── server.rs.md      # LanguageServer trait implementation
│   ├── state.rs.md       # Workspace state management
│   ├── convert.rs.md     # Type conversion utilities
│   ├── labels.rs.md      # Section name and table header definitions
│   └── capabilities/
│       ├── mod.rs.md         # Capability module re-exports
│       ├── diagnostics.rs.md # Diagnostic (error/warning) generation
│       ├── completion.rs.md  # Completion candidate provision
│       ├── hover.rs.md       # Hover information provision
│       ├── navigation.rs.md  # Go to definition / Find references
│       ├── symbols.rs.md     # Document/workspace symbols
│       └── code_action.rs.md # Code actions (Quick Fix)
└── tests/
    ├── capabilities.rs.md # Capability unit tests
    └── diagnostics.rs.md  # Diagnostic integration tests
```

### Key dependency crates

| Crate | Purpose |
| --- | --- |
| `mds-core` | Core logic (Markdown parsing, models, configuration, diagnostics) |
| `tower-lsp` | LSP protocol implementation framework |
| `tokio` | Async runtime |
| `tracing` | Structured logging |

### State model

The workspace state defined in `state.rs` is the foundation for all capabilities.

| Struct | Role |
| --- | --- |
| `WorkspaceState` | Global state. Holds workspace folders, open files, package list |
| `SharedState` | Alias for `Arc<RwLock<WorkspaceState>>`. Shared across all handlers |
| `PackageState` | Per-package information and workspace index |
| `WorkspaceIndex` | Map of `ImplDoc`, reverse index of expose names |
| `OpenFile` | Tracks URI, text, version, and language of open files |

### Request flow

1. Editor sends a request (e.g., `textDocument/completion`)
2. `tower-lsp` deserializes the request and calls the `MdsLanguageServer` method
3. The handler in `server.rs` reads `state` and calls the corresponding `capabilities::*` function
4. The capability function generates results using `mds-core` models
5. `server.rs` returns the response

## Currently Supported Capabilities

| Capability | Summary |
| --- | --- |
| `textDocument/publishDiagnostics` | Missing sections, heading depth violations, code block language mismatch, config file errors |
| `textDocument/completion` | Section names, table column names, code block language labels, snippets |
| `textDocument/hover` | Section heading descriptions, target module information in Uses table |
| `textDocument/definition` | Jump from Uses table target to corresponding file |
| `textDocument/references` | List all files that Use an expose name |
| `textDocument/documentSymbol` | Return `##` / `###` headings as symbols |
| `workspace/symbol` | Search expose names |
| `textDocument/codeAction` | Quick Fix for adding missing sections |

## Build and Test

### Build

```bash
./.github/script/sync-build.sh
cd .build/rust
cargo build -p mds-lsp
```

Release build:

```bash
cargo build -p mds-lsp --release
```

### Run tests

```bash
# mds-lsp tests only
cargo test -p mds-lsp

# All crate tests
cargo test
```

### Code quality checks

```bash
# Format check + Clippy + tests
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

## Debugging

### Setting the log level

mds-lsp uses the `tracing` crate. Control the log level with the `RUST_LOG` environment variable.

```bash
# Launch directly with detailed logs
RUST_LOG=mds_lsp=debug mds-lsp

# Trace level (most detailed)
RUST_LOG=mds_lsp=trace mds-lsp
```

When launching from the VSCode extension, control it with the `mds.lsp.logLevel` setting. Logs are displayed in the "mds Language Server" channel in the Output panel.

### Manual testing via stdio

mds-lsp operates with stdio transport. You can manually send requests for testing:

```bash
# Start LSP server (waits for requests on stdin)
RUST_LOG=mds_lsp=debug mds-lsp
```

Send JSON-RPC requests in the following format:

```
Content-Length: {length}\r\n
\r\n
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{...}}
```

### Debugging the VSCode extension

1. Open the `editors/vscode` directory in VSCode
2. Run `npm install && npm run compile`
3. Press F5 to launch Extension Development Host
4. Open a `.ts.md` file in Extension Development Host
5. Check logs in Output panel → "mds Language Server" channel
6. Breakpoints can be set in the extension's TypeScript code

To debug the LSP server (Rust side):

1. Set `mds.lsp.path` to the debug build path (e.g., `.build/rust/target/debug/mds-lsp`)
2. Restart the VSCode extension
3. Debug the Rust side using `tracing` logs, or separately using `rust-lldb`/`rust-gdb`

## Adding a New Capability

### 1. Create the capability module

Create a new implementation md in `src-md/mds/lsp/src/capabilities/`.

```rust
// capabilities/my_feature.rs
use tower_lsp::lsp_types::*;
use crate::state::WorkspaceState;

pub fn provide_my_feature(
    text: &str,
    position: Position,
    // Required arguments...
) -> Vec<MyResult> {
    // Implementation
    vec![]
}
```

### 2. Register the module

Add the module to `capabilities/mod.rs`:

```rust
pub mod my_feature;
```

### 3. Add the server handler

Implement the `LanguageServer` trait method in `server.rs`:

```rust
async fn my_feature(&self, params: MyFeatureParams) -> Result<Vec<MyResult>> {
    let state = self.state.read().await;
    // Get data from state and call the capability function
    Ok(capabilities::my_feature::provide_my_feature(...))
}
```

### 4. Update ServerCapabilities

Declare the corresponding capability in the `initialize` method of `server.rs`:

```rust
Ok(InitializeResult {
    capabilities: ServerCapabilities {
        my_feature_provider: Some(...),
        ..Default::default()
    },
    ..
})
```

### 5. Add tests

Add tests to `tests/capabilities.rs` or a new test file:

```rust
#[test]
fn my_feature_basic_case() {
    let text = "## Purpose\n\nSample document\n";
    let position = Position { line: 0, character: 5 };
    let result = provide_my_feature(text, position);
    assert!(!result.is_empty());
}
```

## VSCode Extension Development

### Structure

```
editors/vscode/
├── package.json              # Extension manifest
├── src/
│   └── extension.ts          # Extension entry point
├── syntaxes/
│   └── mds-markdown.tmLanguage.json  # TextMate grammar
├── snippets/
│   └── mds.json              # Snippet definitions
└── language-configuration.json # Language configuration
```

### Dynamic language detection

`extension.ts` reads `mds.config.toml` files in the workspace and dynamically detects enabled languages from `[quality.*]` and `[adapters.*]` sections. This allows support for new language adapters without code changes.

Based on detected languages:
- The LSP client's document selector is dynamically constructed
- File watch targets are dynamically configured
- Embedded language support within code blocks is enabled

Simply adding a new language to `LANGUAGE_REGISTRY` completes the extension-side support.

### Embedded language support

A mechanism provides IDE features (completion, hover, go to definition) for the corresponding language within code blocks in mds Markdown.

How it works:

1. Parse the document to identify code block positions and languages
2. If the cursor is inside a code block, create a shadow document with the code content
3. Delegate to the corresponding language provider via VS Code's `executeCommand` API
4. Return results to the user

Current limitations:
- Shadow documents lack file context (imports, etc.), so type inference is limited
- Only works if the language server can process untitled documents
- Jump to definitions outside code blocks is not supported

### Updating TextMate grammar

To add syntax highlighting for a new language, add a new code block pattern to the `repository` in `syntaxes/mds-markdown.tmLanguage.json`:

```json
{
  "mds-code-block-newlang": {
    "begin": "^(\\\\s*```)(newlang|nl)\\\\s*$",
    "end": "^(\\\\s*```\\\\s*)$",
    "contentName": "meta.embedded.block.newlang",
    "patterns": [{ "include": "source.newlang" }]
  }
}
```

Then add the mapping to `embeddedLanguages` in `package.json`.

## Troubleshooting

### LSP server does not start

```bash
# Check the binary
which mds-lsp
mds-lsp --version

# Try launching manually
RUST_LOG=mds_lsp=debug mds-lsp
```

### Diagnostics not showing

1. Check that the target file has the correct extension (`.ts.md`, `.py.md`, `.rs.md`)
2. Check that `mds.config.toml` exists in the workspace
3. Check the mds-lsp logs in VSCode's Output panel
4. Check that `mds.lsp.enabled` is `true`

### Changes not reflected

```bash
# If you changed the Rust side
cd crates
cargo build -p mds-lsp

# If you changed the VSCode extension side
cd editors/vscode
npm run compile
```

In VSCode, use `Ctrl+Shift+P` → "Developer: Reload Window" to reload the extension.

### Tests are failing

```bash
# Check if there are changes in dependency crates
cargo test -p mds-core
cargo test -p mds-lsp

# Run specific tests only
cargo test -p mds-lsp -- test_name --nocapture
```
