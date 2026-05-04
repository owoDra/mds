# Distribution

This page explains how mds is distributed.

## Principle

mds is distributed as a single static binary built in Rust. No runtime dependencies required.

## Distribution Channels

| Channel | Method |
| --- | --- |
| GitHub Releases | Platform-specific binaries (recommended) |
| install.sh | One-liner install via `curl -fsSL .../install.sh \| sh` |

## Installation

```bash
# Recommended: install script
curl -fsSL https://raw.githubusercontent.com/owo-x-project/owox-mds/main/install.sh | sh

# Specific version
curl -fsSL .../install.sh | sh -s -- --version 0.3.0
```

## Self-Update

```bash
mds update              # Update to latest
mds update --version 0.4.0  # Update to specific version
```

## Version Pinning

Specify the mds version for a project in `mds.config.toml`:

```toml
[package]
enabled = true
mds_version = "0.3.0"
```

`mds doctor` detects version mismatches and warns.

## Included Binaries

| Binary | Purpose |
| --- | --- |
| `mds` | Main CLI command |
| `mds-lsp` | Language Server (for editor integration) |

## Pre-Release Quality Checks

Before release, the following are verified:

- Artifact existence and checksums
- Signatures
- Software Bill of Materials (SBOM)
- Provenance information
- Post-install smoke tests
