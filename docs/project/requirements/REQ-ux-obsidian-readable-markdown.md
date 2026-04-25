---
id: REQ-ux-obsidian-readable-markdown
status: 採用
related:
  - README.md
  - docs/project/architecture.md
---

# Obsidian で読める Markdown

## 目標

mds の Markdown 正本は Obsidian でそのまま閲覧でき、標準寄り Markdown とリンクを活用できること。

## 根拠

人間が設計、仕様、実装、テストを読みやすく追跡できることが mds の前提価値であるため。

## 対象範囲

- 標準寄り Markdown を使うこと
- 標準 Markdown リンクを扱えること
- Obsidian `[[wikilink]]` を許容すること
- Graph view で関係を追えるリンク構造を維持すること

## 対象外

- Obsidian 専用パッケージを作ること
- Obsidian の独自機能に mds の正本構造を依存させること
- Obsidian 以外で読めない Markdown 方言を必須にすること

## 成功指標

- mds の代表 Markdown が Obsidian で大きく崩れず閲覧できる
- 標準リンクと wikilink の両方を検証対象として扱える
- Obsidian がなくても mds の parser と generator が動作する

## 制約 / 品質条件

- Obsidian は閲覧体験の対象であり、実行時依存ではない
- Markdown の可読性を損なうメタデータ表現を避ける

## 関連資料

- `../../README.md`
- `../architecture.md`
