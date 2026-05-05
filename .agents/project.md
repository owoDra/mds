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
- `mds-core`: Rust core library package; Markdown source lives in `mds/core/src-md`
- `mds-cli`: native CLI package; Markdown source lives in `mds/cli/src-md`
- `mds-lsp`: Language Server Protocol package; Markdown source lives in `mds/lsp/src-md`
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

## Validation Policy
- `mds/core`、`mds/cli`、`mds/lsp` など mds 管理 package の build / test / lint は mds command を入口にする。
- 通常は `mds package sync`、`mds build`、`mds lint --package <package>`、`mds test --package <package>` を使う。
- Cargo 直実行は mds CLI 起動不能時の bootstrap、release binary 作成、mds 管理外 Rust workspace 検証に限る。
