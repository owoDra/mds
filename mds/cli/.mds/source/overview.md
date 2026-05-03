# mds-cli

## Purpose

Native CLI package for mds commands, including argument parsing, wizard flow, and command execution handoff to mds-core.

## Architecture

This package is authored under `mds/cli/.mds/source/` and synchronized into package `src/` / `tests/` by `mds build`. The repo-local helper `./.github/script/sync-self-hosted-rust.sh` mirrors those generated files into `.build/rust/mds/cli/` before Cargo commands. Package metadata is read from `../Cargo.toml`; mds does not use a package root `index.md`.

## Source Of Truth Scope

- CLI command behavior is described here and in the nearest command implementation markdown under `mds/cli/.mds/source/`.
- Legacy references to `docs/project/specs/shared/SPEC-cli-commands.md` and related command specs should migrate here.

## Command Surface

- `args.rs.md` defines command-line parsing and option validation.
- `main.rs.md` and `wizard.rs.md` define command dispatch, interactive setup flow, and localized guidance.
- The CLI remains a thin surface over `mds-core`; markdown model rules, descriptor behavior, package detection, and quality execution stay in core.

### Migrated CLI Command Rules

- The stable surface includes `check`, `build`, `lint`, `lint --fix`, `test`, `new`, `init`, `doctor`, `package sync`, `release check`, and the CLI-side update or wizard flows described by `args.rs.md` and `wizard.rs.md`.
- CLI-specific validation belongs here when it changes parsing, prompt flow, user-visible output, or exit code semantics instead of changing core package behavior.
- `mds init --ai` remains part of the CLI contract even though template generation and environment setup logic live in `mds-core`.

## Init, Release, And Operational Flows

- `mds init`, `mds doctor`, `mds package sync`, and `mds release check` command entrypoints are described here at the surface level and delegate detailed behavior to `mds-core` implementation markdown.
- User-visible output, exit-code expectations, and argument compatibility belong here when they are CLI-specific.

### Migrated User-Facing Behavior

- Successful command output should point users back to Markdown source-of-truth paths rather than generated artifact internals.
- Interactive flows must show plan-or-confirm steps before mutating project files, installing tools, or overwriting generated agent-kit files.
- Exit-code behavior remains thin over core execution: success for clean runs, non-zero for validation or environment failures, and a distinct internal-error code for unexpected CLI/runtime faults.

### Package Summary

| Name | Version |
| --- | --- |
| mds-cli | 0.1.0-alpha.1 |

### Dependencies

| Name | Version | Summary |
| --- | --- | --- |
| crossterm | 0.27 |  |
| mds-core | 0.1.0-alpha.1 |  |
| ratatui | 0.26 |  |

### Dev Dependencies

| Name | Version | Summary |
| --- | --- | --- |

## Exposes

| Kind | Name | Target | Summary |
| --- | --- | --- | --- |
| module | mds-cli | ../../.build/rust/mds-cli | Generated Cargo package. |

## Rules

- Keep package-level source design in this overview.
- Keep implementation code in `*.rs.md` files.
- Do not edit generated files under `../src`, `../tests`, or `.build/rust/mds-cli`.
- Do not add new docs/project spec documents for CLI behavior already described here.