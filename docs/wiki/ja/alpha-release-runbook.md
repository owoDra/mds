# Alpha Release Runbook

## Version

`0.1.0-alpha.1`

## Prerequisites

- Rust toolchain (stable)
- Node.js 24+ and npm
- GitHub secret configured: `VSCE_PAT`
- GitHub Actions environment configured: `release`

## Local Verification (before tagging)

```bash
# Rust packages
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build -p mds-cli -p mds-lsp

# VS Code extension
cd editors/vscode
npm install
npm run compile
cd ../..

# Release helper syntax
bash -n install.sh
bash -n .github/script/package-vscode.sh
```

## Automated Release (recommended)

Create and push a `v*` tag. The `Release` GitHub Actions workflow builds all release artifacts and publishes them.

```bash
git tag v0.1.0-alpha.1
git push origin v0.1.0-alpha.1
```

The workflow performs the following:

- Builds `mds` and `mds-lsp` for `linux-x64`, `darwin-x64`, `darwin-arm64`, and `win32-x64`.
- Uploads GitHub Release assets such as `mds-v0.1.0-alpha.1-x86_64-unknown-linux-gnu.tar.gz` and `.sha256` files.
- Packages platform-specific VSIX files with bundled `mds-lsp` under `server/<target>/`.
- Publishes the VS Code extension with `VSCE_PAT`.

## Manual Dry Run

Use manual workflow dispatch with `dry_run: true` to build artifacts without publishing GitHub Release assets or Marketplace packages.

## Manual Release Fallback

Manual release should only be used when GitHub Actions is unavailable.

```bash
# Example for the current host only
cargo build --release -p mds-cli -p mds-lsp
mkdir -p dist
cp target/release/mds dist/
cp target/release/mds-lsp dist/
tar -czf mds-v0.1.0-alpha.1-<target>.tar.gz -C dist .

gh release create v0.1.0-alpha.1 \
  mds-v0.1.0-alpha.1-<target>.tar.gz \
  --title "v0.1.0-alpha.1" \
  --prerelease
```

Do not publish `mds-core` as a standalone crate for this release flow. It is linked into `mds` and `mds-lsp` as an internal workspace dependency.
