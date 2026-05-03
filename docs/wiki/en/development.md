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
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cargo install --path .build/rust/mds-cli

# Verify
mds --version
```

## Repository Structure

```
mds/
├── src-md/                  # Markdown source of truth for mds itself
│   ├── index.md             # Source root design
│   ├── mds/core/            # Core library source of truth
│   ├── mds/cli/             # CLI source of truth
│   └── mds/lsp/             # LSP source of truth
├── .build/                  # Generated artifacts (not tracked)
│   └── rust/                # Generated Cargo workspace
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
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust
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
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust
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
- Integration tests are placed in `src-md/*/tests/*.rs.md` and synchronized to `.build/rust/*/tests/`
- E2E tests verify through CLI binary execution

## Quality Checks

### Formatting

```bash
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust
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
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```

In VSCode, you can run the "mds: Check All" task for the same checks.

## Running mds Commands for Verification

How to run commands under development with sample packages.

```bash
cargo run -p mds-cli -- build --verbose
./.github/script/sync-self-hosted-rust.sh
cd .build/rust

# Structure inspection
cargo run -p mds-cli -- check --package ../../examples/minimal-ts

# Generation preview
cargo run -p mds-cli -- build --package ../../examples/minimal-ts --dry-run

# Execute generation
cargo run -p mds-cli -- build --package ../../examples/minimal-ts

# Interactive initialization
cargo run -p mds-cli -- init --package /tmp/test-project

# Environment diagnosis
cargo run -p mds-cli -- doctor --package ../../examples/minimal-ts
```

## Debugging

### Log output

Use the `--verbose` flag for detailed output.

```bash
cargo run -p mds-cli -- check --package ../../examples/minimal-ts --verbose
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
      "args": ["check", "--package", "../../examples/minimal-ts", "--verbose"]
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

1. Run `cargo run -p mds-cli -- build --verbose` to update package-local generated `src/` / `tests/`
2. Run `./.github/script/sync-self-hosted-rust.sh` to rebuild `.build/rust/` for this repository
3. Format with `cargo fmt` in `.build/rust`
4. Confirm no warnings with `cargo clippy` in `.build/rust`
5. Confirm all tests pass with `cargo test` in `.build/rust`
6. Add tests for new features
7. Update documentation if needed
8. Verify with sample projects

## Related Documentation

- [CONTRIBUTING.md](../../../CONTRIBUTING.md) — Overall contribution guidelines
- [Architecture](../../project/architecture.md) — Design principles and invariants
- [Glossary](../../project/glossary/core.md) — Project-wide term definitions
- [Tech Stack](../../project/tech-stack.md) — Adopted technologies and version policies
- [Contributing](contributing.md) — Reporting and proposal guidelines
