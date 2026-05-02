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
- `mds-core`: Rust core library package; Markdown source lives in `mds-core/src-md`
- `mds-cli`: native CLI package; Markdown source lives in `mds-cli/src-md`
- `mds-lsp`: Language Server Protocol package; Markdown source lives in `mds-lsp/src-md`
- `editors/vscode`: VS Code extension package; Markdown source context lives in `editors/vscode/src-md`

## Teams
- Rust implementation team
- TypeScript implementation team
- Python implementation team

## Integrations
- Claude Code
- Codex CLI
- Opencode
- GitHub Copilot CLI
