# Distribution

This page explains how mds is distributed.

## Principle

mds is distributed as pre-built native binaries built in Rust. The CLI has no runtime dependencies; editor integrations use the `mds-lsp` binary.

## Distribution Channels

| Channel | Method |
| --- | --- |
| GitHub Releases | Platform-specific archives containing `mds` and `mds-lsp` (recommended) |
| install.sh | One-liner install that downloads the matching GitHub Releases archive |
| VS Code Marketplace | Platform-specific extension packages with bundled `mds-lsp` |

## Installation

```bash
# Recommended: install the latest GitHub Releases archive
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh

# Specific version
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/latest/install.sh | sh -s -- --version 0.2.1-alpha
```

The release assets are named by tag and Rust target triple, for example:

- `mds-v0.2.1-alpha-x86_64-unknown-linux-gnu.tar.gz`
- `mds-v0.2.1-alpha-aarch64-apple-darwin.tar.gz`
- `mds-v0.2.1-alpha-x86_64-pc-windows-msvc.zip`

## Updating

Run the installer again. Use `--version` to pin a specific release.

## Version Pinning

Specify the mds version for a project in `mds.config.toml`:

```toml
[package]
enabled = true
mds_version = "0.2.1-alpha"
```

`mds doctor` detects version mismatches and warns.

## Included Binaries

| Binary | Purpose |
| --- | --- |
| `mds` | Main CLI command |
| `mds-lsp` | Language Server for non-VS Code editors or custom VS Code `mds.lsp.path` |

## VS Code Extension

```bash
code --install-extension owo-x-project.mds
```

The Marketplace extension is published as platform-specific packages and includes the matching `mds-lsp` binary under `server/<target>/`. VS Code users usually do not need a separate `mds-lsp` install.

## Pre-Release Quality Checks

Before release, the following are verified:

- Artifact existence and checksums
- Signatures
- Software Bill of Materials (SBOM)
- Provenance information
- Post-install smoke tests
