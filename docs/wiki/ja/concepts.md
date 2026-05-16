# 基本概念

このページでは、current な mds model の概念を説明します。

## 正本

mds では、Markdown が package 設計、実装、検証の一次情報です。生成された source file は派生 output であり、authoritative な変更点ではありません。

## source doc

source doc は `.mds/source` の下にあり、通常は 1 機能または 1 つの root module を表します。

source doc には次をまとめます。

- `Purpose` に機能の意図
- `Contract` に stable behavior
- `API` に public surface の説明
- `Source` に executable code
- `Cases` に代表ケース

## test doc

test doc は `.mds/test` の下にあり、executable verification を持ちます。

test doc には次をまとめます。

- `Purpose` に検証の意図
- `Covers` に対象 source module
- `Cases` に期待結果
- `Test` に executable test code

## logical module id

各 source/test doc は canonical root 内の path から logical module id を持ちます。mds はこの id を output planning と wiki-style link の両方で使います。

## package output config

`mds.config.toml` は authoring root と output location を分離します。

- `[roots]` は `.mds/source` と `.mds/test` を固定しつつ output base directory を持つ
- `[output]` と `[[output.override]]` は実際の file path を決める

これにより authoring model を安定させたまま package ごとの output rule を表現できます。

## generated-file bridge

mds は generation planning 中に source map を記録します。`mds-lsp` はその source map を使い、generated file 経由の editor operation を元の Markdown range に remap できます。

## package boundary

`package.json`、`pyproject.toml`、`Cargo.toml` などの package manager metadata は package-manager behavior の正本です。`package.md` は mds 向けの package 文書で、feature-level authoring は `.mds/source` と `.mds/test` に置きます。