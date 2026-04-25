---
status: 採用
related:
  - docs/project/requirements/REQ-core-markdown-source-of-truth.md
  - docs/project/requirements/REQ-ux-obsidian-readable-markdown.md
---

# 読める Markdown 正本

## 目的

Markdown を機械入力だけでなく、人間が設計、仕様、実装レベルのコード、テストを追える正本として保つ。

## 適用範囲

- mds の Markdown 正本全般
- Obsidian での閲覧
- AI エージェントによる実装、修正、検証

## 適用しない範囲

- 生成コードの整形規則そのもの
- Obsidian 専用拡張への依存

## パターン

- 説明文、見出し、テーブル、コードブロックを標準寄り Markdown で書く。
- 実装そのものは `Types`、`Source`、`Test` のコードブロックに書き、説明文だけで代替しない。
- 機械解釈が必要な情報は表にし、自由文だけに埋め込まない。
- Obsidian wikilink は許容するが、標準 Markdown と併存できる範囲に留める。

## 適用条件

- 人間がレビューする正本である。
- AI エージェントが同じ文書を根拠に作業する。

## 例外 / 逸脱条件

- 実行時に必要な生成コードは Markdown から派生させる。
- AI が設計説明から推測したコードを正本なしに生成する運用は採用しない。
- 可読性を損なう機械専用メタデータは採用前に spec 化する。

## 根拠

mds の価値は、同じ Markdown に説明と実コードがあり、人間と AI が同じ正本を読めることにあるため。

## 関連資料

- `../requirements/REQ-core-markdown-source-of-truth.md`
- `../requirements/REQ-ux-obsidian-readable-markdown.md`
- `../specs/shared/SPEC-obsidian-readable-markdown.md`
