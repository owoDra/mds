# Phase 04: Cleanup And Validation

## 目的

旧 registry 前提を整理し、examples と validation を新 runtime に揃える。

## 前提条件

- core / CLI / editor の主要移行が完了している

## 完了条件

- obsolete built-in registry が整理される
- examples と validation が新 runtime を前提に通る

## 検証方法

- examples、build、lint、typecheck、test、doctor の回帰確認

## task 一覧

- `task-001-remove-obsolete-builtins-and-align-examples.md`: 旧 built-in と examples を整理する
- `task-002-run-regression-and-close-doc-gaps.md`: 回帰確認と文書差分解消を行う

## 依存関係

- `../phase-03-cli-and-editor-adoption/index.md`

## 参照

- `task-001-remove-obsolete-builtins-and-align-examples.md`: obsolete built-in と examples の整理
- `task-002-run-regression-and-close-doc-gaps.md`: 回帰確認と文書差分解消
