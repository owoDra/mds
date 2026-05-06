# Descriptor Catalog Coverage

This example records the complete built-in descriptor surface that mds should keep covered by tests.

`coverage.toml` is intentionally a compact catalog, not a runnable package. The core test suite compares each list against `mds/core/src/descriptors/**` so newly added languages, framework overlays, lint tools, typecheck tools, test tools, or package managers must be represented here.
