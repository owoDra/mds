---
id: REQ-doc-model-markdown-document-types
status: 採用
related:
  - README.md
  - docs/project/architecture.md
---

# Markdown 文書種別

## 目標

mds は `.mds/source` と `.mds/test` の fixed authoring root 配下で、source overview、test overview、source md、test md を明確に区別して扱えること。

## 根拠

source rule、test rule、1 機能 1 実装、1 テスト責務を分離することで、正本の読み方と生成対象を安定させるため。

## 対象範囲

- `.mds/source/overview.md` が source root の overview、architecture、dependency snapshot、navigation を担当し、`Imports` / `Exports` を持たないこと
- `.mds/test/overview.md` が test root の overview と test rule を担当すること
- package / directory root の `Imports` / `Exports` は言語ごとの root module md が担当すること
- package 情報は language package metadata を正とすること
- root module md は `Source` section なしの metadata-only source md として扱えること
- `.mds/source/**/*.{lang-ext}.md` が 1 機能 1 実装を担当すること
- `.mds/source/**/*.{lang-ext}.md` はファイル名ではなく内容で spec state / impl state を判定し、`Source` / `Types` の生成対象コードがない間も設計・仕様正本として扱えること
- `.mds/test/**/*.md` が `Covers` を持つ test md を担当すること
- implementation md の doc comment / docstring はコードブロック外の Markdown テキストとして記述すること
- 各文書種別に必須セクションを定義すること
- `Exports` による公開定義は空または `-` の Summary を持たず、参照される定義は H5 shared definition と説明文を持つこと

## 対象外

- authoring root を package ごとに自由命名すること
- 任意ファイル名の Markdown を source md / test md とみなすこと
- language package metadata を Markdown 正本として扱うこと

## 成功指標

- 文書種別ごとの必須セクションを検査できる
- `.mds/source/lib.rs.md`、`.mds/source/mod.rs.md`、`.mds/source/index.ts.md` などの root module md から階層下の公開面を把握できる
- test md の `Covers` から source との対応を検査できる
- language package metadata と Markdown 正本の責務境界が明確である

## 制約 / 品質条件

- authoring root は `.mds/source` と `.mds/test` に固定する
- `overview.md` には `Imports` / `Exports` / `Exposes` を置かず、構造一覧と公開面は root module md の `Exports` に寄せる
- `overview.md` には `Imports` / `Exports` / `Exposes` も overview 専用 surface section も置かず、公開面は root module md または source md の `Exports` に寄せる
- package metadata は Markdown に手書きで複製せず、必要な snapshot は managed section として扱う
- implementation md の validator 既定値では top-level 実装を code fence ごとに分離する

## 関連資料

- `../../README.md`
- `../architecture.md`
