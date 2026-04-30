# Development Guide

> *This page was translated from [Japanese](../ja/development.md) by AI.*

This page explains the environment setup, build, test, and debug procedures for participating in mds development.

If you are using mds to operate a project, refer to [Getting Started](getting-started.md). Installing via a package manager is the easiest (`cargo install mds-cli` / `npm install -g @owox-mds/cli` / `pip install mds-cli`).

The following are procedures for cloning the repository and developing.

## Prerequisites

| Tool | Version | Purpose |
| --- | --- | --- |
| Rust | 1.86 or later | Building and testing core processing |
| Node.js | 24 or later | Building and testing npm packages |
| Python | 3.13 or later | Building and testing Python packages |
| uv | Latest | Python dependency management |
| Git | Latest | Version control |

You do not need to set up all language environments at once. You can start with Rust only.

## Environment Setup

### Rust environment (required)

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add quality tools
rustup component add rustfmt clippy

# Verify build
cd crates
cargo build
```

### Node.js environment (when working with TypeScript)

```bash
# Install Node.js 24+ (example using nvm)
nvm install 24
nvm use 24

# Install npm package dependencies
cd packages
npm install
```

### Python environment (when working with Python)

```bash
# Install uv
python3 -m pip install --user uv

# Install Python package dependencies
cd python/mds_cli
uv sync
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
├── packages/                # npm package distribution
├── python/                  # Python package distribution
├── docs/
│   ├── project/             # Design source of truth (requirements, specs, ADRs)
│   └── wiki/ja/             # User-facing documentation
├── examples/                # Sample projects
├── result/                  # Output for operation verification
└── Makefile                 # Development task shortcuts
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

### Batch execution (Makefile)

```bash
make check             # Run fmt --check + clippy + test in batch
make fmt               # Auto-format
make lint              # clippy only
make test              # Tests only
```

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
