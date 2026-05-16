# Task 001: Remove Obsolete Builtins And Align Examples

## 目的

新 runtime へ移行済みの built-in registry や tool profile を整理し、examples を新前提へ揃える。

## 前提条件

- 新 runtime で主要機能が動作する

## 作業内容

- obsolete built-in を削除または非推奨化する
- examples の config / schema / docs を更新する

## 完了条件

- examples が新 runtime で説明できる
- 旧 built-in 依存が縮小している

## 検証方法

- examples で representative flow を実行確認する

## 依存関係

- `../../phase-03-cli-and-editor-adoption/task-001-update-init-and-new-for-schema-runtime.md`
- `../../phase-03-cli-and-editor-adoption/task-002-update-lsp-and-vscode-language-discovery.md`

## 成果物

- `examples/`
- built-in registry 関連実装
