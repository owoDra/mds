---
id: REQ-generation-code-output-rules
status: 採用
related:
  - README.md
  - docs/project/architecture.md
  - docs/project/validation.md
---

# 生成コード出力規則

## 目標

mds は implementation md 内の `Source`、`Types`、`Test` の実コードブロックから生成物を規約に従って出力できること。

## 根拠

出力先を自由指定にせず、path と命名規約で決めることで、生成結果を予測可能にするため。

## 対象範囲

- Source は `Source` のコードブロックを `*.{lang-ext}.md` から末尾 `.md` を外したファイルへ生成すること
- Types は `Types` のコードブロックを language adapter の pattern に従って別ファイルへ生成すること
- Test は `Test` のコードブロックを 1 implementation md につき 1 テストファイルへ生成すること
- 生成物の配置は設定された出力ルートと adapter 規則に従うこと

## 対象外

- 任意の `file=` 指定による出力先変更
- 1 つの implementation md から複数機能の Source を生成すること
- 生成コードを手修正して正本との差分を維持する運用
- 設計説明や `Cases` だけを入力にして実装コードを新規合成する運用

## 成功指標

- 代表的な `.ts.md`、`.py.md`、`.rs.md` から期待される Source、Types、Test ファイル名を導出できる
- adapter の test file pattern と types file pattern が反映される
- 生成結果と Markdown 正本内のコードブロックの整合性を fixture で比較できる

## 制約 / 品質条件

- 出力規則は adapter によって言語差分を吸収する
- 自由な出力先指定より規約の一貫性を優先する

## 関連資料

- `../../README.md`
- `../architecture.md`
- `../validation.md`
