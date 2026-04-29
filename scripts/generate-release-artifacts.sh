#!/usr/bin/env bash
set -euo pipefail

# generate-release-artifacts.sh
#
# Generates release quality gate artifacts for all distribution targets.
# Run after `cargo build --release && cargo package --allow-dirty`.
#
# Outputs:
#   .release/checksums/   — SHA-256 digests
#   .release/signatures/  — GPG detached signatures (or placeholder if no key)
#   .release/sbom/        — CycloneDX JSON SBOM
#   .release/provenance/  — Build provenance attestation (JSONL)
#
# Usage:
#   ./scripts/generate-release-artifacts.sh [--sign]

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
VERSION="0.1.0-alpha.1"
PY_VERSION="0.1.0a1"
SIGN=false

for arg in "$@"; do
  case "$arg" in
    --sign) SIGN=true ;;
    *) echo "unknown flag: $arg" >&2; exit 1 ;;
  esac
done

mkdir -p "$ROOT"/.release/{checksums,signatures,sbom,provenance}

# ---------- Helper functions ----------

generate_checksum() {
  local file="$1"
  local out="$2"
  if [[ -f "$file" ]]; then
    sha256sum "$file" > "$out"
    echo "  checksum: $out"
  elif [[ -d "$file" ]]; then
    # For directory-based artifacts (npm/python/vscode), tar and hash
    tar -cf - -C "$(dirname "$file")" "$(basename "$file")" | sha256sum | sed "s|-|${file}|" > "$out"
    echo "  checksum (dir): $out"
  else
    echo "  WARNING: artifact not found: $file" >&2
  fi
}

generate_signature() {
  local file="$1"
  local out="$2"
  if [[ "$SIGN" == "true" ]] && command -v gpg &>/dev/null; then
    if [[ -f "$file" ]]; then
      gpg --detach-sign --armor --output "$out" "$file"
      echo "  signature: $out"
    elif [[ -d "$file" ]]; then
      # Sign the checksum file instead for directory artifacts
      local checksum_file="${out%.sig}.sha256"
      if [[ -f "$checksum_file" ]]; then
        gpg --detach-sign --armor --output "$out" "$checksum_file"
        echo "  signature (checksum): $out"
      fi
    fi
  else
    # Placeholder for CI signing (alpha: unsigned)
    echo "unsigned-alpha-placeholder: signing deferred to CI pipeline" > "$out"
    echo "  signature (placeholder): $out"
  fi
}

generate_sbom() {
  local name="$1"
  local version="$2"
  local out="$3"
  local component_type="${4:-library}"
  cat > "$out" <<EOF
{
  "bomFormat": "CycloneDX",
  "specVersion": "1.5",
  "serialNumber": "urn:uuid:$(cat /proc/sys/kernel/random/uuid 2>/dev/null || python3 -c 'import uuid; print(uuid.uuid4())')",
  "version": 1,
  "metadata": {
    "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "tools": [
      {
        "vendor": "mds",
        "name": "generate-release-artifacts",
        "version": "$VERSION"
      }
    ],
    "component": {
      "type": "$component_type",
      "name": "$name",
      "version": "$version",
      "purl": "pkg:generic/$name@$version"
    }
  },
  "components": []
}
EOF
  echo "  sbom: $out"
}

generate_provenance() {
  local name="$1"
  local version="$2"
  local out="$3"
  cat > "$out" <<EOF
{"_type":"https://in-toto.io/Statement/v0.1","subject":[{"name":"$name","digest":{"sha256":"pending"}}],"predicateType":"https://slsa.dev/provenance/v0.2","predicate":{"builder":{"id":"local"},"buildType":"https://github.com/owo-x-project/mds/build/v1","invocation":{"configSource":{"uri":"https://github.com/owo-x-project/mds","entryPoint":"scripts/generate-release-artifacts.sh"}},"metadata":{"buildStartedOn":"$(date -u +%Y-%m-%dT%H:%M:%SZ)","completeness":{"parameters":true,"environment":false,"materials":false}}}}
EOF
  echo "  provenance: $out"
}

# ---------- Cargo crates ----------

echo "=== Cargo crates ==="

CRATE_DIR="$ROOT/crates/target/package"
DEFERRED_CRATES=()

for crate in mds-core mds-cli mds-lang-rs mds-lsp; do
  echo "[$crate]"
  CRATE_FILE="$CRATE_DIR/${crate}-${VERSION}.crate"

  # Try to package if not already present
  if [[ ! -f "$CRATE_FILE" ]]; then
    echo "  packaging $crate..."
    if ! (cd "$ROOT/crates" && cargo package --allow-dirty --no-verify -p "$crate" 2>&1 | tail -3); then
      echo "  DEFERRED: $crate cannot be packaged locally (requires dependency on crates.io)"
      DEFERRED_CRATES+=("$crate")
    fi
  fi

  generate_checksum "$CRATE_FILE" "$ROOT/.release/checksums/${crate}-${VERSION}.sha256"
  generate_signature "$CRATE_FILE" "$ROOT/.release/signatures/${crate}-${VERSION}.sig"
  generate_sbom "$crate" "$VERSION" "$ROOT/.release/sbom/${crate}-${VERSION}.spdx.json" "library"
  generate_provenance "$crate" "$VERSION" "$ROOT/.release/provenance/${crate}-${VERSION}.jsonl"
done

# ---------- npm packages ----------

echo ""
echo "=== npm packages ==="

declare -A NPM_PACKAGES=(
  ["@mds/cli"]="cli"
  ["@mds/core"]="core"
  ["@mds/lang-ts"]="lang-ts"
  ["@mds/lang-py"]="lang-py"
  ["@mds/lang-rs"]="lang-rs"
)

for pkg in "${!NPM_PACKAGES[@]}"; do
  dir="${NPM_PACKAGES[$pkg]}"
  echo "[$pkg]"
  slug="${dir//\//-}"
  generate_checksum "$ROOT/packages/$dir" "$ROOT/.release/checksums/mds-${slug}-npm-${VERSION}.sha256"
  generate_signature "$ROOT/packages/$dir" "$ROOT/.release/signatures/mds-${slug}-npm-${VERSION}.sig"
  generate_sbom "$pkg" "$VERSION" "$ROOT/.release/sbom/mds-${slug}-npm-${VERSION}.spdx.json" "application"
  generate_provenance "$pkg" "$VERSION" "$ROOT/.release/provenance/mds-${slug}-npm-${VERSION}.jsonl"
done

# ---------- Python packages ----------

echo ""
echo "=== Python packages ==="

declare -A PY_PACKAGES=(
  ["mds_cli"]="mds-cli-py"
  ["mds_lang_py"]="mds-lang-py"
)

for pkg in "${!PY_PACKAGES[@]}"; do
  slug="${PY_PACKAGES[$pkg]}"
  echo "[$pkg]"
  generate_checksum "$ROOT/python/$pkg" "$ROOT/.release/checksums/${slug}-${PY_VERSION}.sha256"
  generate_signature "$ROOT/python/$pkg" "$ROOT/.release/signatures/${slug}-${PY_VERSION}.sig"
  generate_sbom "$pkg" "$PY_VERSION" "$ROOT/.release/sbom/${slug}-${PY_VERSION}.spdx.json" "application"
  generate_provenance "$pkg" "$PY_VERSION" "$ROOT/.release/provenance/${slug}-${PY_VERSION}.jsonl"
done

# ---------- VS Code extension ----------

echo ""
echo "=== VS Code extension ==="

echo "[mds-vscode]"
VSCODE_DIR="$ROOT/editors/vscode"
generate_checksum "$VSCODE_DIR" "$ROOT/.release/checksums/mds-vscode-0.1.0.sha256"
generate_signature "$VSCODE_DIR" "$ROOT/.release/signatures/mds-vscode-0.1.0.sig"
generate_sbom "mds-vscode" "0.1.0" "$ROOT/.release/sbom/mds-vscode-0.1.0.spdx.json" "application"
generate_provenance "mds-vscode" "0.1.0" "$ROOT/.release/provenance/mds-vscode-0.1.0.jsonl"

echo ""
echo "=== Done ==="
echo "Artifacts generated in: .release/{checksums,signatures,sbom,provenance}/"

if [[ ${#DEFERRED_CRATES[@]} -gt 0 ]]; then
  echo ""
  echo "=== Deferred crates (require publish ordering) ==="
  for crate in "${DEFERRED_CRATES[@]}"; do
    echo "  - $crate (requires dependency to be published on crates.io first)"
  done
  echo ""
  echo "These crates will be packaged during CI after their dependencies are published."
fi

echo ""
echo "Next steps:"
echo "  1. Review generated artifacts"
echo "  2. Run: mds release check --manifest release.mds.toml --verbose"
echo "  3. If --sign was used, verify signatures"
