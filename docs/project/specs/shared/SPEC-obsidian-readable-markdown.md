---
id: SPEC-obsidian-readable-markdown
status: 採用
related:
  - docs/project/requirements/REQ-ux-obsidian-readable-markdown.md
---

# Obsidian で読める Markdown

## 概要

mds の Markdown 正本は標準寄り Markdown とし、Obsidian でもそのまま閲覧できる。

## 関連要求

- `../../requirements/REQ-ux-obsidian-readable-markdown.md`

## 入力

- mds の Markdown 正本
- 標準 Markdown link
- Obsidian wikilink

## 出力

- parser が解釈するリンク関係
- Obsidian で閲覧可能な Markdown

## 挙動

- 標準 Markdown link を扱う。
- Obsidian `[[wikilink]]` を許容する。
- Obsidian 専用パッケージは必須にしない。

## 状態遷移 / 不変条件

- Obsidian は閲覧体験の対象であり、mds の実行時依存ではない。
- Obsidian 以外で読めない Markdown 方言を必須にしない。

## エラー / 例外

- 解決できないリンクは check の参照診断として扱う。
- Obsidian 固有機能に依存する構造は mds 正本構造として扱わない。

## 横断ルール

- リンク解釈は check と documentation navigation で一貫させる。
- 可読性を損なうメタデータ表現を避ける。

## 検証観点

- 標準 link と wikilink の代表 fixture を確認する。
- Obsidian がなくても parser と generator が動作することを確認する。

## 関連資料

- `../../requirements/REQ-ux-obsidian-readable-markdown.md`
- `../../patterns/ux-readable-markdown-source.md`
