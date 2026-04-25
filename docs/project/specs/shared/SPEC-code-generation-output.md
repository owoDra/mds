---
id: SPEC-code-generation-output
status: 採用
related:
  - docs/project/requirements/REQ-generation-code-output-rules.md
---

# 生成コード出力

## 概要

mds は implementation md 内の `Source`、`Types`、`Test` に書かれた実コードブロックを規約に従って派生ファイルへ出力する。

## 関連要求

- `../../requirements/REQ-generation-code-output-rules.md`

## 入力

- `*.{lang-ext}.md`
- `mds.config.toml` の output root
- language adapter の file pattern

## 出力

- Source ファイル
- Types ファイル
- Test ファイル

## 挙動

- Source は `Source` のコードブロックを implementation md の末尾 `.md` を外したファイル名へ生成する。
- Types は `Types` のコードブロックを language adapter の pattern に従い、原則として別ファイルへ生成する。
- Test は `Test` のコードブロックを 1 implementation md につき 1 テストファイルへ生成する。
- `Types`、`Source`、`Test` 内のコードブロックは原則として出現順に連結する。
- コードブロックの間には説明文や補足見出しを置ける。

## 状態遷移 / 不変条件

- 自由な `file=` 指定は採用しない。
- 生成コードは派生物であり、正本は Markdown とする。
- 1 implementation md から複数機能の Source を生成しない。
- generator は設計説明から実装コードを合成せず、Markdown 内のコードブロックを生成元とする。

## エラー / 例外

- adapter が対象言語の file pattern を解決できない場合は生成エラーにする。
- 出力先が対象 package の範囲外になる場合は reject する。

## 横断ルール

- output root は package 設定解決後の値に従う。
- 生成後の import / use / require は `Uses` から adapter が生成する。

## 検証観点

- `.ts.md`、`.py.md`、`.rs.md` の代表 fixture から期待ファイル名を導出できることを確認する。
- 出力コードと Markdown 内コードブロック由来の期待出力を比較する。
- 自由な `file=` 指定が使えないことを確認する。

## 関連資料

- `../../requirements/REQ-generation-code-output-rules.md`
- `../../patterns/impl-one-feature-markdown.md`
- `../../validation.md`
