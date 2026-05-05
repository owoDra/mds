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
```

Release assets are named by release tag and Rust target triple.

## Updating

Run the installer again to update to the latest release.

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
