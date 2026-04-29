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
- `crates/mds-core`: Rust core library
- `crates/mds-cli`: native CLI
- `crates/mds-lang-rs`: Rust language adapter
- `packages/core`: npm core package
- `packages/cli`: npm CLI package
- `packages/lang-ts`: TypeScript language adapter
- `packages/lang-py`: Python language adapter for npm distribution
- `packages/lang-rs`: Rust language adapter for npm distribution
- `python/mds_cli`: Python package distribution
- `python/mds_lang_py`: Python language adapter distribution

## Teams
- Rust implementation team
- TypeScript implementation team
- Python implementation team

## Integrations
- Claude Code
- Codex CLI
- Opencode
- GitHub Copilot CLI
