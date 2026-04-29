.PHONY: build build-release test lint fmt check clean doc run-check run-build vendor-npm vendor-python

# --- Build ---
build:
	cd crates && cargo build

build-release:
	cd crates && cargo build --release

# --- Quality ---
test:
	cd crates && cargo test

lint:
	cd crates && cargo clippy -- -D warnings

fmt:
	cd crates && cargo fmt

fmt-check:
	cd crates && cargo fmt --check

check: fmt-check lint test
	@echo "All checks passed."

# --- Run mds commands against examples ---
run-check:
	cd crates && cargo run -p mds-cli -- check --package ../examples/minimal-ts --verbose

run-build-dry:
	cd crates && cargo run -p mds-cli -- build --package ../examples/minimal-ts --dry-run --verbose

run-build:
	cd crates && cargo run -p mds-cli -- build --package ../examples/minimal-ts --verbose

run-init:
	cd crates && cargo run -p mds-cli -- init --package /tmp/mds-test-project

run-doctor:
	cd crates && cargo run -p mds-cli -- doctor --verbose

# --- Clean ---
clean:
	cd crates && cargo clean

# --- Vendor native binary ---
vendor-npm: build-release
	node packages/scripts/vendor-binary.js

vendor-python: build-release
	python3 python/scripts/vendor_binary.py

vendor: vendor-npm vendor-python
	@echo "Vendored native binary into npm and Python packages."

# --- Release ---
release-artifacts: build-release
	./scripts/generate-release-artifacts.sh

release-check: release-artifacts
	crates/target/debug/mds release check --manifest release.mds.toml --verbose

release-dry-run: check release-artifacts
	@echo ""
	@echo "=== Cargo package ==="
	cd crates && cargo package --allow-dirty -p mds-core && cargo package --allow-dirty -p mds-lang-rs
	@echo ""
	@echo "=== npm pack ==="
	cd packages && npm pack --workspaces --dry-run
	@echo ""
	@echo "=== Python build ==="
	cd python/mds_cli && python3 -m py_compile mds_cli/__init__.py && python3 -m py_compile mds_cli/__main__.py
	cd python/mds_lang_py && python3 -m py_compile mds_lang_py/__init__.py
	@echo ""
	@echo "=== VS Code extension ==="
	cd editors/vscode && npx @vscode/vsce package --pre-release 2>/dev/null || echo "vsce not available (install: npm i -g @vscode/vsce)"
	@echo ""
	@echo "=== Release gate ==="
	crates/target/debug/mds release check --manifest release.mds.toml --verbose
	@echo ""
	@echo "Alpha release dry run complete."

# --- Documentation ---
doc:
	cd crates && cargo doc --no-deps --open

# --- Help ---
help:
	@echo "mds development tasks"
	@echo ""
	@echo "Build:"
	@echo "  make build          Debug build"
	@echo "  make build-release  Release build"
	@echo "  make clean          Remove build artifacts"
	@echo ""
	@echo "Quality:"
	@echo "  make test           Run all tests"
	@echo "  make lint           Run clippy"
	@echo "  make fmt            Auto-format code"
	@echo "  make fmt-check      Check formatting"
	@echo "  make check          Run fmt-check + lint + test"
	@echo ""
	@echo "Run (against examples/):"
	@echo "  make run-check      mds check"
	@echo "  make run-build-dry  mds build --dry-run"
	@echo "  make run-build      mds build"
	@echo "  make run-init       mds init (interactive)"
	@echo "  make run-doctor     mds doctor"
	@echo ""
	@echo "Release:"
	@echo "  make vendor             Vendor native binary"
	@echo "  make release-artifacts  Generate checksums/SBOM/provenance"
	@echo "  make release-check      Run release quality gate"
	@echo "  make release-dry-run    Full dry-run (check + package + gate)"
	@echo ""
	@echo "Docs:"
	@echo "  make doc            Generate and open API docs"
