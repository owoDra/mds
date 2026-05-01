# Alpha Release Runbook

## Version

`0.1.0-alpha.1`

## Prerequisites

- Rust toolchain (stable)
- `@vscode/vsce` (for VS Code extension)
- GitHub secrets configured: `CARGO_REGISTRY_TOKEN`, `VSCE_PAT`

## Local Verification (before publishing)

```bash
# 1. Full quality gate
cd crates && cargo fmt --check && cargo clippy -- -D warnings && cargo test

# 2. Examples smoke test
cargo run -p mds-cli -- check --package ../examples/minimal-ts --verbose

# 3. Release dry run
cargo build --release
./.github/scripts/generate-release-artifacts.sh
```

## Automated Release (recommended)

Use the GitHub Actions workflow:

1. Go to **Actions → Alpha Release**
2. Click **Run workflow**
3. Set `dry_run: true` for first verification
4. After verification, re-run with `dry_run: false`

## Manual Release (step-by-step)

### 1. Cargo Crates (dependency order)

```bash
cd crates

# Publish independent crates first
cargo publish -p mds-core
cargo publish -p mds-lang-rs

# Wait for crates.io index propagation (~30s)
sleep 30

# Publish dependent crates
cargo publish -p mds-cli
cargo publish -p mds-lsp
```

### 2. VS Code Extension

```bash
cd editors/vscode
npm install
npm run compile
npx @vscode/vsce package --pre-release --out ../../.build/node/vscode
npx @vscode/vsce publish --pre-release
```

### 3. GitHub Release

```bash
# Build platform binaries
./scripts/sync-build.sh
cargo --manifest-path .build/rust/Cargo.toml build --release

# Create GitHub Release with binaries
gh release create v0.1.0-alpha.1 \
  .build/rust/target/release/mds \
  .build/rust/target/release/mds-lsp \
  --title "v0.1.0-alpha.1" \
  --prerelease
```

### 4. Generate and Verify Release Artifacts

```bash
# Generate checksums, SBOM, provenance
./.github/scripts/generate-release-artifacts.sh

# Verify release gate
mds release check --manifest release.mds.toml --verbose
```
