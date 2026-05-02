---
id: SPEC-markdown-document-model
status: 採用
related:
  - docs/project/requirements/REQ-doc-model-markdown-document-types.md
  - docs/project/requirements/REQ-implementation-one-md-one-feature.md
---

# Markdown 文書モデル

## 概要

mds の Markdown 正本は `.mds/source` と `.mds/test` の fixed authoring root に分かれる。source md は生成元となる実装レベルのコードを含み、test md は `Covers` を介して source md を検証する。

## 関連要求

- `../../requirements/REQ-doc-model-markdown-document-types.md`
- `../../requirements/REQ-implementation-one-md-one-feature.md`

## 入力

- mds 対象 package 内の `.mds/source` と `.mds/test` 配下の Markdown ファイル
- package metadata

## 出力

- 文書種別
- 文書種別ごとの必須セクション検査結果
- navigation、package 情報、source/test 対応、implementation 単位の生成対象

## 挙動

- `.mds/source/overview.md` は source root の overview、architecture、managed dependency snapshot、navigation を担当する。
- `.mds/source/overview.md` は `Purpose`、`Architecture`、`Exposes`、`Rules` を必須セクションとする。
- `.mds/test/overview.md` は test root の overview と rule を担当し、`Purpose`、`Architecture`、`Rules` を必須セクションとする。
- `.mds/source/overview.md` の `Exposes` は `Kind`、`Name`、`Target`、`Summary` 列を持つ。
- package 情報は language package metadata を正とし、Markdown に直接複製しない。
- `.mds/source/**/*.{lang-ext}.md` は 1 機能 1 実装を担当する source md とする。
- `.mds/test/**/*.md` は `Covers` を持つ test md とする。
- source md は `Purpose`、`Contract`、`Expose`、`Uses`、`Types`、`Source`、`Cases` を扱う。
- test md は `Purpose`、`Covers`、`Uses`、`Cases`、`Test` を扱う。
- source md の `Types` と `Source` のコードブロック、test md の `Test` のコードブロックは派生ファイルの直接的な生成元とする。
- code block に import / use / require を直接書いてはならず、依存は `Uses` に記録する。
- 1 code block に複数の top-level function、class、type、impl などを混在させてはならない。
- `Purpose`、`Contract`、`Cases` は実コード生成の推測入力ではなく、人間と AI が意図と期待結果を確認するための説明情報とする。

## 状態遷移 / 不変条件

- `Structure` セクションは `overview.md` の必須構造にせず、公開面は `Exposes` に統合する。
- language package metadata は Markdown 文書種別として扱わない。
- authoring root は `.mds/source` と `.mds/test` に固定し、任意の markdown root は採用しない。
- source md は複数機能を混在させない。
- source md と test md は同一ファイルに同居させない。
- 1 code block に複数の top-level logical unit を混在させない。
- 設計説明だけの source md / test md を完成状態として扱わない。

## エラー / 例外

- 必須セクションが欠ける場合は構造エラーにする。
- source md / test md の必須セクションが H2 以外で書かれている場合は構造エラーにする。
- 任意ファイル名の Markdown を source md / test md とみなさない。
- `Contract` を独立 md として扱わない。
- `Purpose`、`Contract`、`Cases` だけから `Source`、`Types`、`Test` を暗黙生成しない。
- test md に `Covers` がない場合は構造エラーにする。

## 横断ルール

- 文書種別の検出結果は parser、generator、CLI、adapter、LSP で共有する。
- 表示名 override があっても canonical セクションとして解決する。

## 検証観点

- 各文書種別の正常 fixture と必須セクション不足 fixture を用意する。
- `.mds/source/overview.md` の `Exposes` から階層下の公開面を抽出できることを確認する。
- `.mds/test` の `Covers` から source への対応を解決できることを確認する。

## 関連資料

- `../../requirements/REQ-doc-model-markdown-document-types.md`
- `../../requirements/REQ-implementation-one-md-one-feature.md`
- `../../patterns/impl-one-feature-markdown.md`