---
id: SPEC-expose-uses-tables
status: 採用
related:
  - docs/project/requirements/REQ-metadata-expose-uses.md
  - docs/project/adr/active/ADR-004-expose-uses-metadata.md
---

# Expose / Uses テーブル

## 概要

mds は公開面を `Expose`、依存を `Uses` の Markdown テーブルで表す。

## 関連要求

- `../../requirements/REQ-metadata-expose-uses.md`

## 入力

- `Expose` テーブル
- `Uses` テーブル
- label override 後の canonical column

## 出力

- 公開 symbol の一覧
- セクション単位の依存一覧
- adapter に渡す仮想 import / use / require の材料

## 挙動

- `Expose` は `Kind`、`Name`、`Summary` 列を持つ。
- `Uses` は `From`、`Target`、`Expose`、`Summary` 列を持つ。
- `Uses` は `Types`、`Source`、`Test` ごとに別々に持つ。
- `From` は少なくとも `internal`、`package`、`builtin` を扱う。
- import / use / require はコードブロック内に書かず、language adapter が `Uses` から生成する。

## 状態遷移 / 不変条件

- `Expose` と `Uses` の意味は設定で変更できない。
- 型依存、実装依存、テスト依存は混在させない。
- 公開名は `Expose` で表し、`symbol` フィールドは採用しない。

## エラー / 例外

- 必須列がないテーブルは構造エラーにする。
- unknown な `From` や `Kind` は adapter または仕様で扱える場合を除き検査エラーにする。
- コードブロック内の import / use / require は規約違反として扱う。

## 横断ルール

- テーブル列の表示名は override できるが、canonical column は維持する。
- 言語固有 kind は adapter に閉じ込める。

## 検証観点

- `Expose` から公開面を抽出できることを確認する。
- `Uses` から仮想 import が作れることを確認する。
- `Types` / `Source` / `Test` ごとの依存分離を確認する。

## 関連資料

- `../../requirements/REQ-metadata-expose-uses.md`
- `../../patterns/data-table-metadata.md`
- `../../adr/active/ADR-004-expose-uses-metadata.md`
