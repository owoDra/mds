# Task 002: Replace Quality Slot And Capture Resolution

## 目的

quality slot command と diagnostic capture rule の解決を built-in tool profile 依存から config/schema 依存へ移す。

## 前提条件

- core が config/schema を読める

## 作業内容

- slot semantic ごとの command 解決経路を整理する
- capture rule の config/schema surface を実装へ反映する
- built-in tool profile 依存を削減する

## 完了条件

- quality slot と capture rule が config/schema から解決される

## 検証方法

- lint / typecheck / test の command 解決と diagnostics capture を確認する

## 依存関係

- `task-001-replace-language-and-file-rule-resolution.md`

## 成果物

- `mds/core/src/quality.rs`
- `mds/core/src/config.rs`
