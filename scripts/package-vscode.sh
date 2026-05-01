#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
SOURCE_DIR="$ROOT/editors/vscode"
BUILD_DIR="$ROOT/.build/node/vscode"
PACKAGE_DIR="$BUILD_DIR/package"
PRE_RELEASE=false

for arg in "$@"; do
  case "$arg" in
    --pre-release) PRE_RELEASE=true ;;
    *) echo "unknown flag: $arg" >&2; exit 1 ;;
  esac
done

rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR" "$BUILD_DIR"

(cd "$SOURCE_DIR" && npm run compile)

cp "$SOURCE_DIR/package.json" "$PACKAGE_DIR/package.json"
node -e "const fs=require('fs'); const p='${PACKAGE_DIR}/package.json'; const pkg=require(p); if (pkg.scripts) delete pkg.scripts.vscode_prepublish; if (pkg.scripts) delete pkg.scripts['vscode:prepublish']; fs.writeFileSync(p, JSON.stringify(pkg, null, 2) + '\n');"
cp "$SOURCE_DIR/README.md" "$PACKAGE_DIR/README.md"
cp "$SOURCE_DIR/CHANGELOG.md" "$PACKAGE_DIR/CHANGELOG.md"
cp "$SOURCE_DIR/LICENSE" "$PACKAGE_DIR/LICENSE"
cp "$SOURCE_DIR/language-configuration.json" "$PACKAGE_DIR/language-configuration.json"
cp "$SOURCE_DIR/.vscodeignore" "$PACKAGE_DIR/.vscodeignore"
cp -R "$SOURCE_DIR/snippets" "$PACKAGE_DIR/snippets"
cp -R "$SOURCE_DIR/syntaxes" "$PACKAGE_DIR/syntaxes"
cp -R "$BUILD_DIR/out" "$PACKAGE_DIR/out"

(cd "$PACKAGE_DIR" && npm install --omit=dev --ignore-scripts --package-lock=false)

if [[ "$PRE_RELEASE" == "true" ]]; then
  (cd "$PACKAGE_DIR" && npx @vscode/vsce package --pre-release --out "$BUILD_DIR")
else
  (cd "$PACKAGE_DIR" && npx @vscode/vsce package --out "$BUILD_DIR")
fi
