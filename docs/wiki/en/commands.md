# Commands

> *This page was translated from [Japanese](../ja/commands.md) by AI.*

This page summarizes the current mds CLI.

## Common Form

```bash
mds <command> --package ./path/to/package
```

If `--package` is omitted, mds searches for enabled packages under the current directory.

## Command Summary

| Command | Purpose |
| --- | --- |
| `mds init` | Initialize a package, quality settings, and optional AI kit workflow |
| `mds new` | Scaffold current tableless Markdown templates |
| `mds build` | Plan and write generated outputs |
| `mds lint` | Validate Markdown structure and run selected linters |
| `mds typecheck` | Run selected typecheck commands |
| `mds test` | Run selected test commands |
| `mds doctor` | Check required tools and runtime environment |
| `mds package sync` | Sync managed package metadata into `package.md` |

## `mds init`

Use `mds init` to create or refresh package config.

```bash
mds init --package ./path/to/package
```

Current templates assume canonical `.mds/source` and `.mds/test` roots plus `[output]` patterns.

### `mds init --ai`

`mds init --ai` writes agent kit files for supported AI CLIs.

```bash
mds init --ai --target all --categories all --yes
```

Supported targets are Claude Code, Codex CLI, Opencode, and GitHub Copilot.

## `mds new`

`mds new` scaffolds current tableless documents.

```bash
mds new greet.ts.md
mds new overview.md
mds new index.ts.md
```

Use it for source docs, hierarchy overviews, and language root module docs. Pair source docs with test docs in `.mds/test` when behavior needs executable verification.

## `mds build`

`mds build` writes generated outputs.

```bash
mds build --package ./path/to/package
```

Use dry-run before the first write or after changing output patterns.

```bash
mds build --package ./path/to/package --dry-run
```

## `mds lint`

`mds lint` validates document structure, output planning, and selected toolchain commands.

```bash
mds lint --package ./path/to/package
```

`mds lint --fix` updates code fences using the configured fixer.

```bash
mds lint --package ./path/to/package --fix
```

## `mds typecheck`

```bash
mds typecheck --package ./path/to/package
```

This runs the typecheck command configured in the relevant `[quality.<lang>]` section.

## `mds test`

```bash
mds test --package ./path/to/package
```

This runs the test command configured in the relevant `[quality.<lang>]` section.

## `mds doctor`

```bash
mds doctor --package ./path/to/package
```

Add `--format json` when you need machine-readable output.

## `mds package sync`

```bash
mds package sync --package ./path/to/package
```

Use `--check` to report drift without rewriting `package.md`.

## Suggested Flow

1. `mds lint`
2. `mds build --dry-run`
3. `mds build`
4. `mds typecheck`
5. `mds test`
6. `mds doctor`

## Exit Codes

| Exit code | Meaning |
| --- | --- |
| `0` | Success |
| `1` | Diagnostic or check failure |
| `2` | Usage or configuration error |
| `3` | Internal error |
| `4` | Missing runtime or required tool |