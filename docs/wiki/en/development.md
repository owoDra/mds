# Development Guide

> *This page was translated from [Japanese](../ja/development.md) by AI.*

This page explains the environment setup, build, test, and debug procedures for participating in mds development.

If you are using mds to operate a project, refer to [Getting Started](getting-started.md). The easiest installation is via `curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh`.

The following are procedures for cloning the repository and developing.

## Prerequisites

| Tool | Version | Purpose |
| --- | --- | --- |
| Rust | 1.86 or later | Building and testing core processing |
| Git | Latest | Version control |

## Environment Setup

### Rust environment (required)

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add quality tools
rustup component add rustfmt clippy

# Install mds for development
cargo install --path crates/mds-cli

# Verify
mds --version
```

## Repository Structure

```
mds/
├── crates/                  # Rust workspace
│   ├── mds-core/            # Core library (parsing, validation, generation, init)
│   │   ├── src/
│   │   │   ├── adapter/     # Language adapters
│   │   │   ├── config/      # mds.config.toml parsing
│   │   │   ├── diagnostics/ # Diagnostic messages
│   │   │   ├── generation/  # Code generation
│   │   │   ├── init/        # mds init implementation
│   │   │   ├── markdown/    # Markdown parsing
│   │   │   ├── model/       # Data models
│   │   │   └── ...
│   │   └── tests/           # Integration tests
│   ├── mds-cli/             # CLI entry point
│   │   └── src/
│   │       ├── main.rs      # Main function
│   │       ├── args/        # Argument parsing
│   │       └── wizard.rs    # Interactive init wizard
│   └── mds-lang-rs/         # Rust language adapter
├── editors/vscode/          # VS Code extension
├── docs/
│   ├── project/             # Design source of truth (requirements, specs, ADRs)
│   └── wiki/ja/             # User-facing documentation
├── examples/                # Sample projects
└── .vscode/tasks.json       # Development task definitions
```

## Build

### Rust build

```bash
cd crates
cargo build                # Debug build
cargo build --release      # Release build
```

### Build specific packages

```bash
cargo build -p mds-core    # Core only
cargo build -p mds-cli     # CLI only
```

## Testing

### Run all tests

```bash
cd crates
cargo test
```

### Run specific tests

```bash
cargo test -p mds-core                          # mds-core tests only
cargo test -p mds-core -- parser_generation      # Filter by name
cargo test -p mds-cli -- args                    # CLI argument tests only
```

### Writing tests

- Unit tests are placed in `#[cfg(test)]` within the target module
- Integration tests are placed in `crates/*/tests/`
- E2E tests verify through CLI binary execution

## Quality Checks

### Formatting

```bash
cd crates
cargo fmt              # Auto-format
cargo fmt --check      # Check diff only
```

### Static analysis

```bash
cargo clippy           # lint
cargo clippy -- -D warnings   # Treat warnings as errors
```

### Batch execution

```bash
cd crates
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

In VSCode, you can run the "mds: Check All" task for the same checks.

## Running mds Commands for Verification

How to run commands under development with sample packages.

```bash
cd crates

# Structure inspection
cargo run -p mds-cli -- check --package ../examples/minimal-ts

# Generation preview
cargo run -p mds-cli -- build --package ../examples/minimal-ts --dry-run

# Execute generation
cargo run -p mds-cli -- build --package ../examples/minimal-ts

# Interactive initialization
cargo run -p mds-cli -- init --package /tmp/test-project

# Environment diagnosis
cargo run -p mds-cli -- doctor --package ../examples/minimal-ts
```

## Debugging

### Log output

Use the `--verbose` flag for detailed output.

```bash
cargo run -p mds-cli -- check --package ../examples/minimal-ts --verbose
```

### Using a debugger

In VSCode, you can use the tasks defined in `.vscode/tasks.json`. You can also debug directly with `F5`.

Example `launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug mds check",
      "cargo": {
        "args": ["build", "-p", "mds-cli"],
        "filter": { "kind": "bin", "name": "mds" }
      },
      "args": ["check", "--package", "../examples/minimal-ts", "--verbose"]
    }
  ]
}
```

### Debugging tests

Run a specific test with detailed output:

```bash
cargo test -p mds-core -- --nocapture test_name
```

## Checklist for Code Changes

1. Format with `cargo fmt`
2. Confirm no warnings with `cargo clippy`
3. Confirm all tests pass with `cargo test`
4. Add tests for new features
5. Update documentation if needed
6. Verify with sample projects

## Related Documentation

- [CONTRIBUTING.md](../../../CONTRIBUTING.md) — Overall contribution guidelines
- [Architecture](../../project/architecture.md) — Design principles and invariants
- [Glossary](../../project/glossary/core.md) — Project-wide term definitions
- [Tech Stack](../../project/tech-stack.md) — Adopted technologies and version policies
- [Contributing](contributing.md) — Reporting and proposal guidelines
