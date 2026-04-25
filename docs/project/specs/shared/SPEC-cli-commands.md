---
id: SPEC-cli-commands
status: 採用
related:
  - docs/project/requirements/REQ-cli-command-surface.md
---

# CLI コマンド

## 概要

mds CLI は Markdown 正本からの派生コード生成、検査、品質確認、環境確認、package 同期を提供する。

## 関連要求

- `../../requirements/REQ-cli-command-surface.md`

## 入力

- CLI arguments
- `mds.config.toml`
- Markdown 正本
- package metadata

## 出力

- 生成コード
- 検査結果
- 依存グラフ
- lint / format / test 結果
- 環境診断
- 更新された `package.md`

## 挙動

- `mds build` は Markdown 内のコードブロックとメタ情報から派生コードを生成する。
- `mds check` は構造、参照、表を検証する。
- `mds graph` は Markdown 依存グラフを表示する。
- `mds lint` は Markdown 状態で lint を実行する。
- `mds format` は Markdown 状態で format を実行する。
- `mds test` は生成または仮想コードで test を実行する。
- `mds doctor` は実行環境と adapter toolchain を確認する。
- `mds package sync` は package metadata から `package.md` を更新する。

## 状態遷移 / 不変条件

- 各コマンドは同じ package 境界と設定解決結果を使う。
- 失敗時に対象外 package や生成物を破壊的に変更しない。

## エラー / 例外

- 入力不備、対象なし、部分失敗、adapter 失敗を区別して報告する。
- ユーザーが次に取るべき対応を推測できる標準エラーを返す。

## 横断ルール

- 終了コードと標準出力 / 標準エラーの役割をコマンド間で一貫させる。
- package sync は package metadata 以外の任意文書生成に使わない。

## 検証観点

- 各コマンドの正常系、入力不備、対象なし、部分失敗を fixture で確認する。
- コマンド間で対象検出結果が矛盾しないことを確認する。

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `../../validation.md`
