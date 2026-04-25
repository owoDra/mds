---
status: 採用
related:
  - docs/project/requirements/REQ-metadata-expose-uses.md
  - docs/project/specs/shared/SPEC-expose-uses-tables.md
---

# テーブルメタデータ

## 目的

公開面と依存を Markdown テーブルで明示し、人間と AI エージェントの両方が読み取れるメタデータにする。

## 適用範囲

- `Expose`
- `Uses`
- `index.md` の `Exposes`
- package dependency table

## 適用しない範囲

- 長文説明が必要な設計判断
- 実行可能コードそのもの
- 一時的な作業メモ

## パターン

- 機械解釈したい関係は Markdown テーブルで表す。
- 表示名は override できても canonical column を維持する。
- summary column を持たせ、人間が意味を読めるようにする。

## 適用条件

- 行ごとに同じ列構造で表せる情報である。
- parser と人間の両方が同じ情報を参照する。

## 例外 / 逸脱条件

- 複雑な状態遷移や代替案比較は spec、proposal、ADR に書く。
- 任意の列追加は canonical schema と adapter の扱いを先に決める。

## 根拠

Markdown の可読性を維持しながら、構造化された公開面と依存を抽出するため。

## 関連資料

- `../requirements/REQ-metadata-expose-uses.md`
- `../specs/shared/SPEC-expose-uses-tables.md`
