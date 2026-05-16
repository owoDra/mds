# Getting Started

> *This page was translated from [Japanese](../ja/getting-started.md) by AI.*

This page shows the smallest current setup for trying mds.

## Installation

Install the latest platform-specific binary from GitHub Releases:

```bash
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh
```

This installs both `mds` and `mds-lsp` to `~/.local/bin` by default.

### VS Code extension

```bash
code --install-extension owo-x-project.mds
```

The Marketplace extension bundles the matching `mds-lsp` binary.

## Runtime Requirements

| Purpose | Requirements |
| --- | --- |
| Running `mds` itself | None when using the prebuilt binary |
| TypeScript checks | Node.js and the tools you select in `[quality.ts]` |
| Python checks | Python and the tools you select in `[quality.py]` |
| Rust checks | Rust/Cargo and the tools you select in `[quality.rs]` |

Unselected tools are not treated as missing.

## Minimal Package Layout

```text
my-package/
├── mds.config.toml
├── package.md
├── package.json
├── .mds/
│   ├── source/
│   │   ├── overview.md
│   │   └── greet.ts.md
│   └── test/
│       ├── overview.md
│       └── greet.ts.md
├── src/
└── tests/
```

`package.json`, `pyproject.toml`, `Cargo.toml`, and other recognized package metadata stay authoritative for package-manager details. `package.md` is the mds-facing document for package purpose and managed metadata snapshots.

## Minimal Config

```toml
[package]
enabled = true
allow_raw_source = false

[roots]
source_md = ".mds/source"
test_md = ".mds/test"
source_out = "src"
test_out = "tests"

[output]
source = "{source_out}/{module}.{ext}"
test = "{test_out}/{module}.test.{ext}"
```

If your package needs non-default output names, add `[[output.override]]` entries instead of changing the authoring roots.

## Authoring Model

- Source docs live in `.mds/source/**/*.lang.md`.
- Test docs live in `.mds/test/**/*.md`.
- Use `mds new greet.ts.md`, `mds new overview.md`, or `mds new index.ts.md` to scaffold the current tableless templates.
- Keep source behavior in source docs and executable verification in test docs.

## First Workflow

```bash
mds init --package ./path/to/package
mds lint --package ./path/to/package
mds build --package ./path/to/package --dry-run
mds build --package ./path/to/package
mds typecheck --package ./path/to/package
mds test --package ./path/to/package
```

## Default Output Mapping

| Markdown doc | Default output |
| --- | --- |
| `.mds/source/greet.ts.md` | `src/greet.ts` |
| `.mds/test/greet.ts.md` | `tests/greet.test.ts` |
| `.mds/source/lib.rs.md` | `src/lib.rs` |
| `.mds/test/lib.rs.md` | `tests/lib.test.rs` unless overridden |

The logical module id comes from the path inside `.mds/source` or `.mds/test` after removing `.md` and the trailing language suffix.

## Next Pages

- [Configuration](configuration.md)
- [Markdown Source](markdown-source.md)
- [Commands](commands.md)
- [Generation Mechanism](generation.md)