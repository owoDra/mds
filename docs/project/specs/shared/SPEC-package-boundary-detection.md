---
id: SPEC-package-boundary-detection
status: 採用
related:
  - docs/project/requirements/REQ-monorepo-package-boundary.md
---

# Package 境界検出

## 概要

mds は monorepo 内で package 単位に対象範囲を決め、mds 対象 package と非対象 package を混在させる。

## 関連要求

- `../../requirements/REQ-monorepo-package-boundary.md`

## 入力

- `mds.config.toml` の package 設定
- language package metadata
- 実体の package 定義
- `allow_raw_source`

## 出力

- mds 対象 package の一覧
- mds 非対象 package の一覧
- package ごとの言語、runner、出力ルート

## 挙動

- `[package] enabled = true` と実体の package 定義が揃う package を mds 対象とみなす。
- JS / TS は `package.json`、Python は `pyproject.toml`、Rust は `Cargo.toml` を実体の package 定義とする。
- `enabled = false` の package は mds 対象外とする。
- `allow_raw_source = true` の場合、mds 非対象 package や直書きソースとの混在を許可する。
- package 情報は実体 package metadata を正とし、Markdown へ package metadata を複製しない。

## 状態遷移 / 不変条件

- 対象 package の判定は build と check で一貫する。
- 非対象 package の source は mds が破壊的に書き換えない。

## エラー / 例外

- `enabled = true` だが package 定義がない場合は対象 package として扱わず、設定または構造の不備として報告する。
- package 定義がない任意ディレクトリは暗黙に mds package とみなさない。
- 実体 package metadata を parse できない場合は診断する。

## 横断ルール

- package 境界は subproject 固有仕様より前に解決する。
- language adapter は package 判定結果に従って呼び出す。

## 検証観点

- mds 対象 package、非対象 package、複数言語 package が混在する fixture を使う。
- 誤検出で非対象 package が変更されないことを確認する。

## 関連資料

- `../../requirements/REQ-monorepo-package-boundary.md`
- `../../validation.md`
