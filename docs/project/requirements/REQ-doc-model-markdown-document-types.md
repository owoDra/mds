---
id: REQ-doc-model-markdown-document-types
status: 採用
related:
  - README.md
  - docs/project/architecture.md
---

# Markdown 文書種別

## 目標

mds は `index.md`、`package.md`、`*.{lang-ext}.md` の 3 種類を明確に区別して扱えること。

## 根拠

階層説明、package 説明、1 機能 1 実装の責務を分けることで、正本の読み方と生成対象を安定させるため。

## 対象範囲

- `index.md` が階層の overview、architecture、navigation を担当すること
- `package.md` が project または subproject 単位の package 情報を担当すること
- `*.{lang-ext}.md` が 1 機能 1 実装を担当すること
- 各文書種別に必須セクションを定義すること

## 対象外

- 任意ファイル名の Markdown を実装 md とみなすこと
- `Structure` セクションを `index.md` の必須構造として追加すること
- `package.md` をディレクトリ単位の自由文書として扱うこと

## 成功指標

- 文書種別ごとの必須セクションを検査できる
- `index.md` の `Exposes` から階層下の公開面を把握できる
- `package.md` が package metadata と package ルールを説明できる

## 制約 / 品質条件

- `index.md` の構造一覧は `Exposes` に統合する
- `package.md` は package metadata からの自動生成を基本にし、`Rules` の手書き補足を許す

## 関連資料

- `../../README.md`
- `../architecture.md`
