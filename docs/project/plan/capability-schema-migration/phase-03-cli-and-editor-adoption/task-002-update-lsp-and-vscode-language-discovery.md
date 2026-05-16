# Task 002: Update LSP And VS Code Language Discovery

## 目的

`mds-lsp` と VS Code extension の language discovery / bridge 前提を config/schema runtime へ移す。

## 前提条件

- language identity 解決が新 runtime へ移行済みである

## 作業内容

- file suffix、fence、config/schema を使う language discovery を実装に反映する
- active language indicator と bridge の依存面を整理する
- remap command surface と整合を取る

## 完了条件

- editor 側の language discovery が built-in descriptor 前提から外れる

## 検証方法

- status bar、embedded bridge、diagnostic mirror を確認する

## 依存関係

- `../../phase-02-core-runtime-migration/task-003-preserve-diagnostic-remap-with-source-map.md`

## 成果物

- `mds/lsp/src/`
- `editors/vscode/src/extension.ts`
