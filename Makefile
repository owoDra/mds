.PHONY: build build-release test lint fmt check clean doc run-check run-build

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
	@echo "Docs:"
	@echo "  make doc            Generate and open API docs"
