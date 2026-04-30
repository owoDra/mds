# Alpha Release Runbook

## Version

`0.1.0-alpha.1`

## Prerequisites

- Rust toolchain (stable)
- Node.js >= 24
- Python >= 3.13 + uv
- `@vscode/vsce` (for VS Code extension)
- GitHub secrets configured: `CARGO_REGISTRY_TOKEN`, `NPM_TOKEN`, `PYPI_TOKEN`, `VSCE_PAT`

## Local Verification (before publishing)

```bash
# 1. Full quality gate
make check

# 2. Examples smoke test
make run-check

# 3. Release dry run (package + artifacts + gate)
make release-dry-run
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

### 2. npm Packages

```bash
# Build release binary
cd crates && cargo build --release && cd ..

# Vendor binary into npm wrapper
node packages/scripts/vendor-binary.js

# Publish all workspaces
cd packages && npm publish --workspaces --tag alpha --access public
```

### 3. Python Packages

```bash
# Vendor binary into Python wrapper
python3 python/scripts/vendor_binary.py

# Build and publish
cd python/mds_cli && uv build && uv publish
cd ../mds_lang_py && uv build && uv publish
```

### 4. VS Code Extension

```bash
cd editors/vscode
npm install
npm run compile
npx @vscode/vsce package --pre-release
npx @vscode/vsce publish --pre-release
```

### 5. Generate and Verify Release Artifacts

```bash
# Generate checksums, SBOM, provenance
./scripts/generate-release-artifacts.sh

# Verify release gate
mds release check --manifest release.mds.toml --verbose
```

## Known Limitations (Alpha)

| Item | Status | Note |
|------|--------|------|
| `mds-cli` / `mds-lsp` cargo package | Requires ordered publish | mds-core must be on crates.io first |
| npm/Python wrappers | Require native binary | Use `make vendor` before packaging |
| Signatures | Placeholder | GPG/sigstore signing deferred to stable |
| SBOM components | Scaffold only | Full dependency enumeration in beta |
| Provenance | Local builder | CI provenance attestations in beta |

## Post-Release Checks

```bash
# Cargo install smoke test
cargo install mds-cli --version 0.1.0-alpha.1
mds --version

# npm install smoke test
npm install -g mds-cli@alpha
mds --version

# Python install smoke test
pip install mds-cli==0.1.0a1
mds --version
```

## Rollback

```bash
# Cargo: yank
cargo yank mds-core --version 0.1.0-alpha.1
cargo yank mds-lang-rs --version 0.1.0-alpha.1
cargo yank mds-cli --version 0.1.0-alpha.1
cargo yank mds-lsp --version 0.1.0-alpha.1

# npm: unpublish (within 72h)
cd packages && npm unpublish --workspaces

# VS Code: unpublish
npx @vscode/vsce unpublish owo-x-project.mds
```
