---
status: 採用
related:
  - docs/project/patterns/data-table-metadata.md
  - docs/project/requirements/REQ-doc-model-markdown-document-types.md
---

# Imports / Exports セクション

## 目的

import / export と shared definition を code block の暗黙構造から切り離し、Markdown section と table で明示する。

## 適用範囲

- implementation md の `Imports`
- implementation md の `Exports`
- H5 shared definition 見出し

## パターン

- `## Imports` と `## Exports` を canonical H2 section とする。
- 行単位の関係は Markdown table で表す。
- shared definition は H5 見出しで表し、同一ファイル内外からリンク可能にする。
- import-only code block は正規形にしない。

## 基本列

- `Imports`: `Kind`, `From`, `Target`, `Symbols`, `Via`, `Summary`, `Code`
- `Exports`: `Kind`, `Name`, `Visibility`, `Summary`

## 補足

- `Code` は descriptor-based renderer がすべての言語を吸収しきるまでの literal fallback とする。
- 人間は `From`、`Target`、`Symbols`、`Via` を読み、generator は必要に応じて `Code` を使う。

## 根拠

人間が読みやすい Markdown を維持しながら、AI と parser が import / export / shared definition を安定して参照できるようにするため。