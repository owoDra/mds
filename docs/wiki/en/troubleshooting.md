# Troubleshooting

> *This page was translated from [Japanese](../ja/troubleshooting.md) by AI.*

This page explains common problems and how to check them when using mds.

## No valid mds package found

If the target package is not found when running `mds lint`, `mds typecheck`, or similar commands, check the following:

- Does the target package have an `mds.config.toml`?
- Is `enabled = true` set in `[package]` of `mds.config.toml`?
- Is the correct path specified for `--package`?
- Does the target package have a `package.md`?

## Missing required sections

Implementation Markdown requires sections that mds expects.

Check that `Purpose`, `Contract`, `Types`, `Source`, `Cases`, and `Test` are present.

Heading hierarchy is also important. The main sections of implementation Markdown should be written as `##` headings.

## Errors in `Imports`, `Exports`, `Expose`, or `Uses` tables

Check column names, values, and duplicates in the table.

The dependency types in `Uses` must use the defined values.

| Value | Meaning |
| --- | --- |
| `builtin` | A dependency built into the language or runtime. |
| `package` | A dependency on an external package. |
| `workspace` | A dependency on another package in the same workspace. |
| `internal` | A dependency within the same package. |

Differences in letter case or duplicate rows with the same meaning will cause errors.

## Cannot overwrite generated files

mds will not overwrite existing files that do not have a managed header.

If there is a hand-written file at the generation target, choose one of the following:

- Change the generation target.
- Move the hand-written file to a different location.
- Reconsider the layout of the source Markdown.

Adding an mds managed header manually to a hand-written file to work around this is not recommended.

## Manifest is reported as corrupted

If `.mds/manifest.toml` cannot be read as TOML, or does not match the expected format, mds will not write generated output.

This is to prevent accidentally corrupting files by misjudging what is managed.

First, check the situation with `mds build --dry-run` and `mds lint`.

## Reported missing required tools

`mds lint`, `mds test`, and `mds doctor` require language-specific runtimes and selected inspection tools.

Running `mds doctor` will show you which tools are missing.

```bash
mds doctor --package path/to/package
```

If unused tools are reported as missing, check `[quality.ts]`, `[quality.py]`, and `[quality.rs]` in `mds.config.toml`. Set unnecessary `linter`, `fixer`, and `test_runner` to `false`, and also remove them from `required`.

## Want to preview generation plan only

Use `mds build --dry-run`.

```bash
mds build --package path/to/package --dry-run
```

This command does not write files. It only displays the generation plan and diffs.

## Want to check error types

Check the exit code.

| Exit Code | Meaning |
| --- | --- |
| `0` | Success. |
| `1` | Diagnostic or inspection error. |
| `2` | Command usage or configuration error. |
| `3` | Internal error occurred. |
| `4` | Runtime environment or required tools are missing. |
