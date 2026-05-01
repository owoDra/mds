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
- `src-md/mds-core`: Rust core library source of truth
- `src-md/mds-cli`: native CLI source of truth
- `src-md/mds-lsp`: Language Server Protocol source of truth
- `src-md/vscode`: VS Code extension source context
- `editors/vscode`: VS Code extension

## Teams
- Rust implementation team
- TypeScript implementation team
- Python implementation team

## Integrations
- Claude Code
- Codex CLI
- Opencode
- GitHub Copilot CLI
