# Quality Checks

> *This page was translated from [Japanese](../ja/quality.md) by AI.*

This page explains how mds validates authoring-v2 packages.

## Structural Validation

`mds lint` validates the Markdown model before it invokes any external tool.

It checks:

- required sections for the current doc kind
- canonical `.mds/source` and `.mds/test` roots
- output planning stays inside the package
- managed-file safety before overwriting outputs
- package-local wiki-style links and symbol references

## Check Policy

`[check]` controls several authoring-v2 diagnostics.

| Key | Default | Meaning |
| --- | --- | --- |
| `legacy_tables` | `warn` | Warn or error on old metadata-table patterns |
| `unresolved_module_symbols` | `warn` | Policy for unresolved `[[module#symbol]]` |
| `implementation_section_only` | `true` | Only executable sections are generation sources |
| `split_source_and_test` | `true` | Reject mixing source behavior and test behavior in the wrong doc kind |

Unresolved `[[module]]` is always an error.

## Tool-Driven Checks

After structural validation, mds can invoke the commands configured in `[quality.<lang>]`.

Examples:

- `mds lint` for linters
- `mds lint --fix` for fixers
- `mds typecheck` for typecheck commands
- `mds test` for test runners
- `mds doctor` for runtime and tool availability

## Auto-Fix Scope
## Recommended Verification Order

During development, verifying in the following order makes it easier to isolate issues:

1. `mds lint`
2. `mds build --dry-run`
3. `mds build`
4. `mds typecheck`
5. `mds test`
6. `mds doctor`

For this repository's own release flow, run `./.github/script/release-check.sh --manifest release.mds.toml` in addition to the above.
