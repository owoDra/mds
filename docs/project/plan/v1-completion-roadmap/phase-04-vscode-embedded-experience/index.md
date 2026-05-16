# Phase 04: VS Code Embedded Experience

## 目的

VS Code extension の active language UX、embedded language bridge、diagnostic mirror を v1 spec に揃える。

## 前提条件

- `phase-03-lsp-workspace-and-navigation` の bridge command surface が利用できる
- core / LSP の language identity と remap 契約が安定している

## 完了条件

- status bar で `mds <active-language> | <doc-kind>` 相当を常時視認できる
- provider がある言語で references / rename / code action / formatting まで橋渡しできる
- generated / embedded diagnostics が Markdown 正本へ正しく戻る

## 検証方法

- `editors/vscode` compile と extension-side regression で status bar / provider delegation / diagnostic mirror を確認する
- `examples/minimal-ts` で active language と embedded provider 動作を確認する

## task 一覧

1. 以下は `phase-03` 完了後に並行着手できる
   - `task-001-add-status-bar-and-active-context.md`: active language / doc kind indicator を追加する
   - `task-002-expand-embedded-provider-delegation.md`: provider bridge を references / rename / code action / formatting まで広げる
2. `task-003-remap-diagnostics-and-add-extension-coverage.md`: diagnostic mirror と extension 回帰確認を固める

## 依存関係

- `../phase-03-lsp-workspace-and-navigation/index.md`
- `../../../specs/vscode-extension/SPEC-vscode-embedded-editor-experience.md`
- `../../../specs/shared/SPEC-ux-embedded-language-bridge.md`

## 参照

1. 以下は `phase-03` 完了後に並行着手できる
   - `task-001-add-status-bar-and-active-context.md`: status bar と active context の完成
   - `task-002-expand-embedded-provider-delegation.md`: embedded provider bridge の完成
2. `task-003-remap-diagnostics-and-add-extension-coverage.md`: diagnostic mirror と extension 回帰確認の完成
