#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null)"; then
  :
else
  ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
fi

MIRROR_ROOT="$ROOT/.build/rust"
PACKAGES=(
  "mds/core"
  "mds/cli"
  "mds/lsp"
)

rm -rf "$MIRROR_ROOT"
mkdir -p "$MIRROR_ROOT"

cp "$ROOT/Cargo.toml" "$MIRROR_ROOT/Cargo.toml"
if [[ -f "$ROOT/Cargo.lock" ]]; then
  cp "$ROOT/Cargo.lock" "$MIRROR_ROOT/Cargo.lock"
fi

for package in "${PACKAGES[@]}"; do
  package_root="$ROOT/$package"
  mirror_package_root="$MIRROR_ROOT/$package"
  mkdir -p "$mirror_package_root"

  cp "$package_root/Cargo.toml" "$mirror_package_root/Cargo.toml"
  if [[ -d "$package_root/src" ]]; then
    cp -a "$package_root/src" "$mirror_package_root/"
  fi
  if [[ -d "$package_root/tests" ]]; then
    cp -a "$package_root/tests" "$mirror_package_root/"
  fi
  if [[ -f "$package_root/build.rs" ]]; then
    cp "$package_root/build.rs" "$mirror_package_root/build.rs"
  fi
done

echo "workspace mirror ok: $MIRROR_ROOT"