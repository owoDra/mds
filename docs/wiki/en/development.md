# Development Guide

> *This page was translated from [Japanese](../ja/development.md) by AI.*

This page explains the environment setup, build, test, and debug procedures for participating in mds development.

If you are using mds to operate a project, refer to [Getting Started](getting-started.md). The easiest installation is the GitHub Releases installer: `curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh`.

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

# Build the checked-in workspace
cargo build --workspace

# Optional: install the CLI from the checked-in source tree
cargo install --path mds/cli

# Verify
cargo run -p mds-cli -- --version
```

## Repository Structure

```
owox-mds/
├── mds/
│   ├── core/
│   │   ├── src/             # mds-core checked-in Rust source
│   │   └── tests/           # mds-core checked-in tests
│   ├── cli/
│   │   ├── src/             # mds CLI checked-in Rust source
│   │   └── tests/           # mds CLI checked-in tests
│   └── lsp/
│       ├── src/             # mds-lsp checked-in Rust source
│       └── tests/           # mds-lsp checked-in tests
├── editors/vscode/
│   ├── src/                 # VS Code extension source
│   └── package.json         # Extension manifest and scripts
├── docs/
│   ├── project/             # Design source of truth (requirements, specs, ADRs)
│   └── wiki/                # User-facing documentation
├── examples/                # Sample projects
└── target/                  # Cargo build artifacts
```

## Build

### Build the first-party Rust workspace

```bash
cargo build --workspace
```

### Build specific packages

```bash
cargo build -p mds-core
cargo build -p mds-cli
cargo build -p mds-lsp
```

### Build the VSCode extension

```bash
cd editors/vscode
npm install
npm run compile
```

For this repository's first-party packages, edit the checked-in Rust and TypeScript source trees directly. Reserve `mds` commands for smoke-testing example packages or validating product behavior.

## Testing

### Run all tests

```bash
cargo test --workspace
```

### Run specific tests

```bash
cargo test -p mds-core                           # mds-core tests only
cargo test -p mds-cli                            # mds-cli tests only
cargo test -p mds-lsp                            # mds-lsp tests only
```

### Writing tests

- Unit tests are placed in `#[cfg(test)]` within the target module
- Integration tests are placed in `mds/*/tests/*.rs`
- E2E tests verify through CLI binary execution or sample packages

## Quality Checks

### Formatting

```bash
cargo fmt --all --check
```

### Static analysis

```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

### Batch execution

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

If you changed `editors/vscode`, also run `cd editors/vscode && npm run compile`.

## Running mds Commands for Verification

How to run commands under development with sample packages. These commands validate product behavior against example packages; they do not regenerate the repository's first-party sources.

```bash
# Structure inspection
cargo run -p mds-cli -- check --package examples/minimal-ts

# Generation preview
cargo run -p mds-cli -- build --package examples/minimal-ts --dry-run

# Execute generation
cargo run -p mds-cli -- build --package examples/minimal-ts

# Interactive initialization
cargo run -p mds-cli -- init --package /tmp/test-project

# Environment diagnosis
cargo run -p mds-cli -- doctor --package examples/minimal-ts
```

## Debugging

### Log output

Use the `--verbose` flag for detailed output.

```bash
cargo run -p mds-cli -- check --package examples/minimal-ts --verbose
```

### Using a debugger

In VSCode, you can use the tasks defined in `.vscode/tasks.json`. You can also debug directly with `F5`.

Example `launch.json`:

```json
{
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug mds lint",
      "cargo": {
        "args": ["build", "-p", "mds-cli"],
        "filter": { "kind": "bin", "name": "mds" }
      },
      "args": ["check", "--package", "examples/minimal-ts", "--verbose"]
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

1. Edit the checked-in Rust or TypeScript source and test files directly under `mds/` and `editors/vscode/`
2. Run `cargo fmt --all --check`
3. Run `cargo check --workspace`
4. Run `cargo test --workspace`
5. Run `cargo clippy --workspace --all-targets -- -D warnings`
6. If you changed `editors/vscode`, run `cd editors/vscode && npm run compile`
7. Add tests for new features
8. Update documentation if needed
9. Verify with sample projects using `cargo run -p mds-cli -- ...`

## Related Documentation

- [CONTRIBUTING.md](../../../CONTRIBUTING.md) — Overall contribution guidelines
- [Architecture](../../project/architecture.md) — Design principles and invariants
- [Glossary](../../project/glossary/core.md) — Project-wide term definitions
- [Tech Stack](../../project/tech-stack.md) — Adopted technologies and version policies
- [Contributing](contributing.md) — Reporting and proposal guidelines
