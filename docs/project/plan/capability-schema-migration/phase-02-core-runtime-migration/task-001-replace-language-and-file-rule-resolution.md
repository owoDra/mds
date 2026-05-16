# Task 001: Replace Language And File Rule Resolution

## 目的

built-in descriptor 依存の language identity、output rule、special file rule、root module rule 解決を config/schema 依存へ移す。

## 前提条件

- schema surface が確定している

## 作業内容

- file suffix と fence に基づく language identity 解決へ寄せる
- output / special file / root module rule を config/schema から読む
- 旧 built-in 解決箇所を置換または fallback 化する

## 完了条件

- core の file / language ルール解決が config/schema 中心になる

## 検証方法

- examples で source/test/build/package sync が成立することを確認する

## 依存関係

- `../../phase-01-architecture-and-schema/task-001-freeze-capability-schema-surface.md`

## 成果物

- `mds/core/src/descriptor.rs` または後継 loader
- `mds/core/src/markdown.rs`
- `mds/core/src/generation.rs`
