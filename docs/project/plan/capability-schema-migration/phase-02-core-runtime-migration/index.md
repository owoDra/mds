# Phase 02: Core Runtime Migration

## 目的

`mds-core` の language / quality / remap runtime を built-in registry 依存から config/schema 依存へ移す。

## 前提条件

- schema surface が定義済みである
- migration compatibility 方針がある

## 完了条件

- core の主要 runtime が config/schema を読む
- diagnostic remap が新 runtime でも成立する

## 検証方法

- build / lint / typecheck / test / package sync の回帰確認

## task 一覧

- `task-001-replace-language-and-file-rule-resolution.md`: file rule と language identity 解決を置き換える
- `task-002-replace-quality-slot-and-capture-resolution.md`: quality slot と capture rule 解決を置き換える
- `task-003-preserve-diagnostic-remap-with-source-map.md`: remap を source map 前提で維持する

## 依存関係

- `../phase-01-architecture-and-schema/index.md`

## 参照

- `task-001-replace-language-and-file-rule-resolution.md`: file / language rule 解決の置換
- `task-002-replace-quality-slot-and-capture-resolution.md`: quality slot と capture rule の置換
- `task-003-preserve-diagnostic-remap-with-source-map.md`: source map remap の維持
