---
id: SPEC-core-overview-and-package-sync
status: 提案中
related:
  - ../shared/SPEC-model-package-layout.md
  - ../shared/SPEC-authoring-markdown-format.md
  - ../../requirements/v1/REQ-quality-safe-package-bounded-generation.md
  - ../../requirements/v1/REQ-product-markdown-source-of-truth.md
subproject: mds-core
---

# mds-core Overview And Package Sync

## 概要

`mds-core` における `.mds/source/overview.md` special file と `mds package sync` の契約を定義する。

## 関連要求

- `REQ-quality-safe-package-bounded-generation`
- `REQ-product-markdown-source-of-truth`

## 入力

- `.mds/source/overview.md`
- package manager metadata
- package sync config
- `mds package sync` command mode

## 出力

- synchronized source overview
- package metadata diff
- package sync diagnostics

## 挙動

- `mds-core` は `.mds/source/overview.md` を package 単位の必須 special file として扱う。
- source overview には package summary、dependencies、dev dependencies の managed region を持てる。
- `mds package sync` は package metadata を読み、managed region だけを同期する。
- `--check` では差分を報告し、通常実行では managed region を更新する。
- manual prose と managed region 外の記述は保持する。
- package sync hook を使う場合、package sync 後に hook command を案内または実行対象として扱える。

## 状態遷移 / 不変条件

- source overview は package ごとに 1 つ存在する。
- managed region 更新は package metadata に対して決定的である。
- manual prose は package sync で壊さない。
- test overview は任意の補助文書であり、package sync の必須対象ではない。

## エラー / 例外

- source overview 欠落は package error とする。
- 必須 managed region 欠落は package sync error とする。
- metadata 読み取り不能時は package sync を失敗させる。
- `--check` で差分がある場合は non-zero result を返せる。

## 横断ルール

- special file 契約は一般 `mds file` の section 規則とは分離する。
- overview は package metadata と authoring docs を結ぶ trace point として扱う。

## 検証観点

- source overview 欠落を検出できる。
- managed region のみが更新される。
- `--check` と write mode の差分が説明可能である。
- test overview が package sync に必須でないことが保たれる。

## 関連資料

- `../shared/SPEC-model-package-layout.md`
- `../shared/SPEC-authoring-markdown-format.md`
- `../../requirements/v1/REQ-quality-safe-package-bounded-generation.md`
- `../../requirements/v1/REQ-product-markdown-source-of-truth.md`
