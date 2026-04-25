---
id: SPEC-markdown-document-model
status: 採用
related:
  - docs/project/requirements/REQ-doc-model-markdown-document-types.md
  - docs/project/requirements/REQ-implementation-one-md-one-feature.md
---

# Markdown 文書モデル

## 概要

mds の Markdown 正本は `index.md`、`package.md`、`*.{lang-ext}.md` の 3 種類に分かれる。implementation md は設計説明だけでなく、生成元となる実装レベルのコードを含む。

## 関連要求

- `../../requirements/REQ-doc-model-markdown-document-types.md`
- `../../requirements/REQ-implementation-one-md-one-feature.md`

## 入力

- mds 対象 package 内の Markdown ファイル
- package metadata

## 出力

- 文書種別
- 文書種別ごとの必須セクション検査結果
- navigation、package 情報、implementation 単位の生成対象

## 挙動

- `index.md` は階層の overview、architecture、navigation を担当する。
- `index.md` は `Purpose`、`Architecture`、`Exposes`、`Rules` を必須セクションとする。
- `package.md` は package 単位の package 情報を担当する。
- `package.md` は `Package`、`Dependencies`、`Dev Dependencies`、`Rules` を必須セクションとする。
- `*.{lang-ext}.md` は 1 機能 1 実装を担当する implementation md とする。
- implementation md は `Purpose`、`Contract`、`Types`、`Source`、`Cases`、`Test` を扱う。
- `Types`、`Source`、`Test` のコードブロックは派生ファイルの直接的な生成元とする。
- `Purpose`、`Contract`、`Cases` は実コード生成の推測入力ではなく、人間と AI が意図と期待結果を確認するための説明情報とする。

## 状態遷移 / 不変条件

- `Structure` セクションは `index.md` の必須構造にせず、公開面は `Exposes` に統合する。
- `package.md` は directory 単位ではなく package 単位で扱う。
- implementation md は複数機能を混在させない。
- 設計説明だけの implementation md を完成状態として扱わない。

## エラー / 例外

- 必須セクションが欠ける場合は構造エラーにする。
- 任意ファイル名の Markdown を implementation md とみなさない。
- `Contract` を独立 md として扱わない。
- `Purpose`、`Contract`、`Cases` だけから `Source`、`Types`、`Test` を暗黙生成しない。

## 横断ルール

- 文書種別の検出結果は parser、generator、CLI、adapter で共有する。
- 表示名 override があっても canonical セクションとして解決する。

## 検証観点

- 各文書種別の正常 fixture と必須セクション不足 fixture を用意する。
- `index.md` の `Exposes` から階層下の公開面を抽出できることを確認する。

## 関連資料

- `../../requirements/REQ-doc-model-markdown-document-types.md`
- `../../requirements/REQ-implementation-one-md-one-feature.md`
- `../../patterns/impl-one-feature-markdown.md`
