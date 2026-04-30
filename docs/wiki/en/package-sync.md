# Package Information Sync

> *This page was translated from [Japanese](../ja/package-sync.md) by AI.*

This page explains the purpose and usage of `mds package sync`.

## Purpose

`package.md` is a document that describes package name, version, dependencies, and development dependencies in a human-readable form.

Meanwhile, the actual package information resides in `package.json`, `pyproject.toml`, `Cargo.toml`, and similar files.

`mds package sync` synchronizes the managed sections of `package.md` based on these package information files.

## Targeted Information

The following information is primarily synchronized:

| Information | Description |
| --- | --- |
| Package name | The name of the target package. |
| Version | The version of the target package. |
| Dependencies | Dependencies required at runtime. |
| Development dependencies | Dependencies required for development and inspection. |

## Execution

To update `package.md`, run the following:

```bash
mds package sync --package path/to/package
```

To only confirm differences, use `--check`.

```bash
mds package sync --package path/to/package --check
```

## Handling of Hand-written Sections

`mds package sync` does not freely recreate the entire `package.md`.

It updates the managed sections while preserving other descriptions and rules.

However, if hand-written text is mixed into the managed sections, an error is raised to avoid unintended deletion.

## Sync Verification via Hooks

If you want to quickly detect sync gaps when package information changes, you can bind `mds package sync --check` to a hook.

In this workflow, the hook does not directly update `package.md`; instead, it checks whether synchronization is needed. If there are differences, it raises an error, and the user confirms the content before running `mds package sync`.

In `mds.config.toml`, you can enable the hook configuration as follows:

```toml
[package_sync]
hook_enabled = true
```

When `hook_enabled = true`, the default hook command is:

```bash
mds package sync --check
```

If you want a different command name or execution method, you can specify it explicitly:

```toml
[package_sync]
hook_enabled = true
hook = "mds package sync --check"
```

Hyphen-separated `[package-sync]` can also be used for the configuration name:

```toml
[package-sync]
hook-enabled = true
hook-command = "mds package sync --check"
```

This configuration is for recording the command used in hook workflows on the mds side. Registration with the hook mechanism itself should be done according to the package management tool or continuous integration system you are using.

## Hook Workflow Examples

In continuous integration, you can run the sync check as follows:

```bash
mds package sync --package path/to/package --check
```

When binding to a local pre-commit hook, it is also recommended to use `--check` first:

```bash
mds package sync --check
```

`--check` does not modify `package.md`. If synchronization is needed, it treats it as a failure, and after confirming the differences, you update with the following command:

```bash
mds package sync --package path/to/package
```

## Recommended Workflow

- Place the authoritative package information in each language's package information file.
- Place human-readable descriptions and synchronized dependencies in `package.md`.
- Use `mds package sync --check` for continuous verification.
- Confirm differences before updating with `mds package sync`.
- When binding to hooks, default to `--check` and use it for detecting sync gaps rather than automatic updates.

## Notes

- This is not a feature for generating arbitrary Markdown documents.
- Documents other than package information are not synchronization targets.
- External publishing processes or package publishing are not implicitly executed.
- Even when hooks are enabled, mds does not automatically register or modify external tool configuration files.
- Running `mds package sync` directly in a hook for automatic updates is not recommended because it tends to skip diff confirmation.
