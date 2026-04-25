---
id: REQ-metadata-expose-uses
status: 採用
related:
  - README.md
  - docs/project/architecture.md
  - docs/project/validation.md
---

# Expose と Uses

## 目標

mds は公開面を `Expose`、依存を `Uses` として Markdown 内の表で明示できること。

## 根拠

import / use / require をコードブロック外で管理し、人間と AI が公開面と依存関係を読み取りやすくするため。

## 対象範囲

- `Expose` テーブルで公開される型、関数、クラスなどを表すこと
- `Uses` テーブルで `internal`、`package`、`builtin` などの依存元と参照先を表すこと
- `Types`、`Source`、`Test` ごとに `Uses` を分けること
- import / use / require を language adapter が生成すること

## 対象外

- import / use / require をコードブロック内に手書きする前提
- `symbol` フィールドによる公開名管理
- 型依存、実装依存、テスト依存を同じ依存表に混在させること

## 成功指標

- `Expose` から公開面を機械的に抽出できる
- `Uses` から adapter が仮想 import を生成できる
- 型依存、実装依存、テスト依存を個別に検査できる

## 制約 / 品質条件

- `Expose` と `Uses` の意味は設定で変更できない
- 言語固有の kind は adapter が扱えるようにするが、共通概念を壊さない

## 関連資料

- `../../README.md`
- `../architecture.md`
- `../validation.md`
