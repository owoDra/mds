---
id: REQ-implementation-one-md-one-feature
status: 採用
related:
  - README.md
  - docs/project/architecture.md
  - docs/project/validation.md
---

# 1 Markdown 1 実装

## 目標

implementation md は 1 つの機能だけを扱い、目的、契約、型、実装、期待結果、テストを同じ正本に持ち、`Types`、`Source`、`Test` に実装レベルのコードを書けること。

## 根拠

機能単位で責務と生成物を追跡し、複数機能の混在や自由な出力先指定による複雑化を避けるため。

## 対象範囲

- implementation md の命名を `*.{lang-ext}.md` とすること
- `Purpose`、`Contract`、`Types`、`Source`、`Cases`、`Test` を扱うこと
- `Types`、`Source`、`Test` のコードブロックを派生ファイルの直接的な生成元として扱うこと
- 同一機能の型、実装、テスト、期待結果、契約を同じ md に置けること
- コードブロックをセクション内で出現順に連結できること

## 対象外

- 複数機能を 1 つの implementation md に混在させること
- `Contract` を独立 md として扱うこと
- 自由な `file=` 指定で別ファイルへ出力すること
- `Purpose` や `Contract` だけから実装コードを推測生成すること

## 成功指標

- implementation md から Source、Types、Test の生成対象を一意に決められる
- `Cases` と実行可能な `Test` の両方を保持できる
- 複数機能混在や無関係な責務の同居を検査で検出できる

## 制約 / 品質条件

- `Contract` は外から見た振る舞いの約束を表す
- `Types` はデータ構造を表す
- `Source` は実装の説明ではなく、生成元となる実装コードを含む
- テストは期待結果だけからの全自動生成に依存せず、実テストコードを保持する

## 関連資料

- `../../README.md`
- `../architecture.md`
- `../validation.md`
