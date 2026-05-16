# Task 003: Preserve Diagnostic Remap With Source Map

## 目的

新 runtime へ移行しても、tool diagnostics の Markdown 正本 remap を維持する。

## 前提条件

- quality slot と capture rule が新 runtime で解決できる

## 作業内容

- remap 条件を明確化する
- path / line / column を持つ tool output の remap を確認する
- remap 不能時の fallback を整理する

## 完了条件

- CLI / LSP / editor が同じ Markdown 正本位置を見られる

## 検証方法

- representative tool 出力で remap の正否を確認する

## 依存関係

- `task-002-replace-quality-slot-and-capture-resolution.md`

## 成果物

- `mds/core/src/quality.rs`
- `docs/project/validation.md`
