---
id: ADR-004-expose-uses-metadata
status: 採用
related:
  - docs/project/requirements/REQ-metadata-expose-uses.md
  - docs/project/specs/shared/SPEC-expose-uses-tables.md
---

# Expose と Uses を Markdown 表にする

## 背景

公開面と依存は、人間が読めるだけでなく generator と adapter が解釈できる必要がある。

## 判断

公開面は `Expose`、依存は `Uses` の Markdown テーブルで表す。import / use / require はコードブロック内に置かず、language adapter が生成する。

## 代替案

- コードブロック内に import を手書きする: 言語としては自然だが、文書上の依存一覧が散らばる。
- front matter に依存を書く: 機械処理しやすいが、人間が Markdown 本文として読みづらい。
- `symbol` フィールドで公開名を管理する: 1 md 1 実装と `Expose` があれば重複する。

## 結果

依存と公開面は Markdown 表を正本とし、adapter が言語固有の import へ変換する。

## 関連資料

- `../../requirements/REQ-metadata-expose-uses.md`
- `../../specs/shared/SPEC-expose-uses-tables.md`
- `../../patterns/data-table-metadata.md`
