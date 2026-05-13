# Project

## Name
mds

## Description
Markdown を設計書兼ソースの正本として扱い、多言語コード・型・テストを生成する強規約ツールチェーン。

## Language
Rust, TypeScript, Python

## Kind
monorepo

## Subprojects
- `mds/core`: edit checked-in Rust source and tests in `mds/core/src` and `mds/core/tests`
- `mds/cli`: edit checked-in Rust source and tests in `mds/cli/src` and `mds/cli/tests`
- `mds/lsp`: edit checked-in Rust source and tests in `mds/lsp/src` and `mds/lsp/tests`
- `editors/vscode`: edit checked-in extension source in `editors/vscode/src` and related checked-in tests or fixtures
- Remaining first-party `.mds` assets in this repository are historical, superseded, or cleanup targets under the self-hosting removal plan.

## Teams
- Rust implementation team
- TypeScript implementation team
- Python implementation team

## Integrations
- Claude Code
- Codex CLI
- Opencode
- GitHub Copilot CLI

## Validation Policy
- Standard repository validation is `cargo fmt --all --check`, `cargo check --workspace`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets`.
- For `editors/vscode`, run `npm run compile` in `editors/vscode`.
- Reserve `mds` commands for product behavior, fixture, or package authoring validation when a change specifically needs them.
