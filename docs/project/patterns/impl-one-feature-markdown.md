---
status: 採用
related:
  - docs/project/requirements/REQ-implementation-one-md-one-feature.md
  - docs/project/specs/shared/SPEC-markdown-document-model.md
---

# 1 機能 1 Markdown

## 目的

implementation md の責務を 1 機能に限定し、型、実装、テストの実コードと、期待結果、契約を追跡しやすくする。

## 適用範囲

- `*.{lang-ext}.md`
- Source / Types / Test 生成
- feature 単位の修正、レビュー、検証

## 適用しない範囲

- 階層説明用の `index.md`
- source overview 用の `overview.md`
- 複数 package を横断する設計文書

## パターン

- 1 つの implementation md は 1 つの機能だけを扱う。
- 同一機能の `Purpose`、`Contract`、`Types`、`Source`、`Cases`、`Test` は同じ md に置く。
- `Types`、`Source`、`Test` には派生ファイルの生成元となる実コードを書く。
- `Purpose`、`Contract`、`Cases` は実コードの意図と検証観点を説明するが、コードを暗黙生成する入力にはしない。
- 出力先は path と命名規約から決め、自由な `file=` 指定は使わない。

## 適用条件

- 機能の公開面、依存、期待結果、実装コード、テストコードを 1 単位で説明できる。
- 複数機能を分割しても利用者の理解や生成結果が破綻しない。

## 例外 / 逸脱条件

- 横断仕様や package 構成は implementation md に押し込めない。
- 共通型が複数機能で再利用される場合でも、配置は spec と adapter 規則に従って判断する。

## 根拠

複数機能の混在や設計説明だけの正本化は、生成対象、依存、テストの追跡を曖昧にするため。

## 関連資料

- `../requirements/REQ-implementation-one-md-one-feature.md`
- `../specs/shared/SPEC-markdown-document-model.md`
- `../specs/shared/SPEC-code-generation-output.md`
