# Quality Inspection

> *This page was translated from [Japanese](../ja/quality.md) by AI.*

This page explains the quality inspections handled by mds.

## Quality Inspection Philosophy

In mds, not only generated files but also the state of the source Markdown is inspected.

By combining Markdown structure, dependency, generation target, code block, and target language inspections, discrepancies between the source of truth and derived code are reduced.

## Structural Inspection

`mds check` inspects the structure of Markdown.

It primarily verifies:

- Whether required sections exist
- Whether `Expose` and `Uses` tables are correct
- Whether the target language can be determined from the implementation Markdown filename
- Whether the generation target stays within the package boundary
- Whether there is a risk of overwriting hand-written files at the generation target
- Whether there are contradictions between `package.md` and package information

## Static Analysis

`mds lint` performs language-specific static analysis on code blocks within Markdown.

Code blocks are treated as temporary files, with dependency declarations generated from `Uses` attached for inspection.

Inspection tools can be selected per language. For TypeScript, ESLint or Biome; for Python, Ruff; for Rust, Cargo Clippy, among others. If unselected, static analysis for that language is not executed.

## Auto-fix

`mds lint --fix` applies auto-fixes to code blocks within Markdown.

This process does not freely rewrite Markdown descriptions or structure. The fix targets are code blocks in `Types`, `Source`, and `Test`.

To only confirm differences, use `--check`.

```bash
mds lint --package path/to/package --fix --check
```

Fix tools can also be selected per language. For TypeScript, Prettier or Biome; for Python, Ruff format or Black; for Rust, rustfmt, among others.

## Testing

`mds test` performs language-specific test execution targeting the `Test` section within Markdown.

By placing test code in implementation Markdown, the feature's purpose, contract, implementation, and tests can be tracked from the same document.

For test execution tools, TypeScript uses Vitest or Jest, Python uses Pytest or unittest, and Rust uses Cargo test or cargo-nextest.

## Environment Diagnostics

`mds doctor` diagnoses the execution environment and required tools.

```bash
mds doctor --package path/to/package
```

The diagnostics verify required execution environments and tools based on the language adapters enabled for the target package and the `[quality.*]` settings. Unselected tools are not treated as missing.

## Recommended Verification Order

During development, verifying in the following order makes it easier to isolate issues:

1. `mds check`
2. `mds build --dry-run`
3. `mds build`
4. `mds lint`
5. `mds test`
6. `mds doctor`

For this repository's own release flow, run `./.github/script/release-check.sh --manifest release.mds.toml` in addition to the above.
