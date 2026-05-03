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
- 使わない cell や section は削除せず `-` で skip する。
- `From = internal` の row は target に definition 箇所への Markdown link を置き、必要なら section fragment まで指定する。

## 基本列

- `Imports`: `From`, `Target`, `Symbols`, `Via`, `Summary`, `Reference`
- `Exports`: `Name`, `Visibility`, `Summary`

## 補足

- import statement は descriptor-based renderer が `From`、`Target`、`Symbols`、`Via` から復元する。
- `Reference` は `internal` と参照可能な `external` dependency に付け、定義元の Markdown location を示す。
- 他モジュールや他 package から参照される主要な class / type / function は H5 shared definition 見出しと説明を置いて強調する。

## 根拠

人間が読みやすい Markdown を維持しながら、AI と parser が import / export / shared definition を安定して参照できるようにするため。