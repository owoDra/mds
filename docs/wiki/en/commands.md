# Commands

> *This page was translated from [Japanese](../ja/commands.md) by AI.*

This page explains the purpose and usage of mds commands.

## Basic Form

mds commands can be run by specifying the target package.

```bash
mds check --package path/to/package
```

If `--package` is omitted, mds searches for enabled packages under the current directory.

## `mds check`

`mds check` inspects Markdown structure, tables, configuration, and generation targets.

It mainly verifies the following.

- Whether required sections exist
- Whether `Expose` and `Uses` tables are correct
- Whether the target language can be determined
- Whether generation targets escape outside the package
- Whether it would overwrite existing files that are not managed

Example execution.

```bash
mds check --package path/to/package
```

## `mds build`

`mds build` generates derived code from Markdown.

```bash
mds build --package path/to/package
```

Generation targets are code blocks written in the `Types`, `Source`, and `Test` sections of implementation Markdown.

## `mds build --dry-run`

`mds build --dry-run` displays the generation plan and differences without writing files.

```bash
mds build --package path/to/package --dry-run
```

When generating for the first time or after changing generation rules, use this command to verify differences first.

## `mds lint`

`mds lint` runs language-specific checking tools on code blocks within Markdown.

```bash
mds lint --package path/to/package
```

The checking tools differ for TypeScript, Python, and Rust. The specific connections are handled by language adapters.

## `mds lint --fix`

`mds lint --fix` applies auto-fixes to code blocks within Markdown.

```bash
mds lint --package path/to/package --fix
```

If you only want to verify without applying fixes, add `--check`.

```bash
mds lint --package path/to/package --fix --check
```

Auto-fix updates the contents of targeted code blocks rather than rewriting the entire Markdown.

## `mds test`

`mds test` runs language-specific tests on test code in `Test` sections within Markdown.

```bash
mds test --package path/to/package
```

## `mds doctor`

`mds doctor` diagnoses the runtime environment and required tools.

```bash
mds doctor --package path/to/package
```

To output in JSON format, run as follows.

```bash
mds doctor --package path/to/package --format json
```

## `mds package sync`

`mds package sync` synchronizes the managed portion of `package.md` based on the target language's package information.

```bash
mds package sync --package path/to/package
```

To only check for differences, add `--check`.

```bash
mds package sync --package path/to/package --check
```

## `mds init`

`mds init` performs initialization for using mds.

```bash
mds init --package path/to/package
```

Initialization handles project configuration, settings for support tools, and development environment preparation. External command execution and environment changes are performed only when explicitly requested by the user.

### AI Agent Initialization

`mds init --ai` generates configuration files (agent kit) for AI coding agents.

```bash
# Generate all categories for all AI CLIs
mds init --ai --target all --categories all --yes

# Generate only for Claude Code
mds init --ai --target claude-code --yes

# Generate only specific categories
mds init --ai --target all --categories instructions,skills --yes
```

The supported AI CLIs are as follows.

| AI CLI | Identifier | Output Location |
| --- | --- | --- |
| Claude Code | `claude-code`, `claude` | `.claude/rules/`, `.claude/skills/`, `.claude/commands/` |
| Codex CLI | `codex-cli`, `codex` | `.codex/instructions.md`, `.codex/skills/` |
| Opencode | `opencode` | `.opencode/agents/`, `.opencode/skills/` |
| GitHub Copilot | `github-copilot-cli`, `copilot` | `.github/instructions/`, `.github/prompts/` |

Generation categories are `instructions`, `skills`, and `commands`.

Main files such as CLAUDE.md, AGENTS.md, and copilot-instructions.md are not generated. Files are placed in each CLI's native reference paths, and an integration guide is displayed after generation.

Generated files include `mds-managed: true` frontmatter, allowing safe updates on re-execution. Overwriting non-managed files is rejected unless `--force` is specified.

### Quality Tool Selection

Tools used for quality checks can be selected per language.

```bash
mds init --package path/to/package --ts-tools biome,jest --py-tools ruff,black,pytest --rs-tools rustfmt,cargo-test
```

Languages or quality checks you don't use can be disabled with `none`.

```bash
mds init --package path/to/package --ts-tools none --py-tools pytest --rs-tools clippy,nextest
```

Specifying `default` uses mds's representative combinations. Selections are written to `[quality.ts]`, `[quality.py]`, and `[quality.rs]` in `mds.config.toml`.

### Interactive Mode

Running `mds init` without arguments (or with only `--package`) launches the interactive wizard.

```bash
mds init
mds init --package path/to/package
```

The wizard guides you through selecting initialization mode, language tools, AI targets, and setup options, then executes after plan confirmation.

The traditional flag-based approach remains available.

## `mds new`

`mds new` generates scaffolding for a new implementation Markdown.

```bash
mds new greet.ts.md
mds new utils/helper.py.md --package path/to/package
mds new parser.rs.md --force
```

The language is determined from the file name suffix.

| Suffix | Language |
| --- | --- |
| `.ts.md` | TypeScript |
| `.py.md` | Python |
| `.rs.md` | Rust |

The generated template includes all sections: Purpose, Expose, Uses, Types, Source, and Test. The output destination is under `src-md/`.

Existing files are not overwritten. Use `--force` to force overwrite.

## `mds release check`

`mds release check` performs pre-publication artifact inspection.

```bash
mds release check --manifest release.mds.toml
```

Pre-publication inspection covers artifacts, checksums, signatures, software bill of materials, provenance information, and post-installation verification.

## Exit Codes

mds uses distinct exit codes to differentiate failure types.

| Exit Code | Meaning |
| --- | --- |
| `0` | Success. |
| `1` | Diagnostic or check error. |
| `2` | Command usage or configuration error. |
| `3` | Internal error occurred. |
| `4` | Runtime environment or required tools are missing. |
