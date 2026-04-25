---
id: SPEC-md-state-quality-operations
status: 採用
related:
  - docs/project/requirements/REQ-quality-md-state-validation.md
---

# Markdown 状態の品質操作

## 概要

mds は生成後コードだけでなく、Markdown 内に書かれた実コードの状態に対して lint、format、test を適用できる。

## 関連要求

- `../../requirements/REQ-quality-md-state-validation.md`

## 入力

- implementation md
- `Types`、`Source`、`Test` のコードブロック
- `Uses` テーブル
- language adapter 設定

## 出力

- lint 結果
- format 済み Markdown
- test 結果
- adapter toolchain の診断

## 挙動

- Markdown を読み、対象セクションのコードブロックを抽出する。
- `Uses` から仮想 import / use / require を作る。
- 仮想コードを formatter、linter、test runner に渡す。
- format 結果は Markdown のコードブロックへ戻す。
- 説明文は診断の文脈として扱い、実行対象コードとして扱わない。

## 状態遷移 / 不変条件

- Markdown 正本の構造を壊さずに format 結果を反映する。
- core は各言語 toolchain の詳細を持たず、adapter が接続する。

## エラー / 例外

- 構造エラーと language toolchain の失敗を区別する。
- 仮想 import が生成できない `Uses` は adapter 診断として報告する。
- format 結果を安全に戻せない場合は Markdown を更新しない。

## 横断ルール

- adapter は対象言語の formatter、linter、test runner への接続を提供する。
- adapter ごとの代表 toolchain と呼び出し契約は、各 language adapter の subproject 固有 spec で定義する。

## 検証観点

- 各言語 fixture で仮想 import を含む lint / format / test を確認する。
- format 後に Markdown 構造が維持されることを確認する。

## 関連資料

- `../../requirements/REQ-quality-md-state-validation.md`
- `../../patterns/impl-adapter-boundary.md`
- `../../validation.md`
- `../packages-lang-ts/SPEC-adapter-typescript-generation.md`
- `../packages-lang-py/SPEC-adapter-python-generation.md`
- `../crates-mds-lang-rs/SPEC-adapter-rust-generation.md`
