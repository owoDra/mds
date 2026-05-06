# Descriptor Samples

This package is a runnable example for every built-in language descriptor currently supported by mds.

Run from the repository root:

```bash
cargo run -p mds-cli -- package sync --package examples/descriptor-samples
cargo run -p mds-cli -- lint --package examples/descriptor-samples
cargo run -p mds-cli -- build --package examples/descriptor-samples --dry-run
```

Each `.mds/source/sample.<descriptor>.md` file should render one source file through the matching descriptor.
