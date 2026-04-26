---
id: SPEC-cli-commands
status: 採用
related:
  - docs/project/requirements/REQ-cli-command-surface.md
---

# CLI コマンド

## 概要

mds CLI は Markdown 正本からの派生コード生成と検査を提供する。

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

## 挙動

- `mds build` は Markdown 内のコードブロックとメタ情報から派生コードを生成する。
- `mds check` は構造、参照、表を検証する。
- Parser + 生成 MVP の CLI は `mds check` と `mds build` を対象にし、Post-MVP では `mds lint`、`mds lint --fix`、`mds test`、`mds doctor`、`mds package sync` を採用対象にする。
- Parser + 生成 MVP は TypeScript、Python、Rust の3言語すべてを対象にする。
- `mds check` は文書種別、必須セクション、`Expose` / `Uses` schema、コードブロック有無、path/root/pattern 解決、出力先衝突を検査し、実言語 lint/test は実行しない。
- `mds build --dry-run` は書き込みを行わず、生成計画に加えて git 互換寄りの unified diff を表示する。
- MVP CLI option は `--package <path>`、`--dry-run`（build のみ）、`--verbose` とする。
- `--package` 未指定時、`mds check/build` は cwd 配下の mds 有効 package を探索して対象にする。
- `mds lint`、`mds lint --fix`、`mds test` は Markdown 状態の `Types` / `Source` / `Test` コードブロックを adapter 経由で処理する。
- `mds doctor` は実行環境、adapter、toolchain を診断する。
- `mds package sync` は package metadata を正として `package.md` の生成管理部分を同期する。

## 状態遷移 / 不変条件

- 各コマンドは同じ package 境界と設定解決結果を使う。
- 失敗時に対象外 package や生成物を破壊的に変更しない。
- CLI の終了コードは 0 成功、1 診断あり、2 CLI usage/config error、3 internal error、4 environment / toolchain 不足とする。
- MVP CLI は成功時の要約と生成一覧を stdout、警告・エラー・診断を stderr に出す。
- 複数 package の一部が失敗した場合は、可能な package の処理を続け、最後に失敗 package をまとめて診断し exit code 1 にする。

## エラー / 例外

- 入力不備、対象なし、部分失敗、adapter 失敗を区別して報告する。
- ユーザーが次に取るべき対応を推測できる標準エラーを返す。
- 対象となる mds 有効 package が 0 件の場合は診断として stderr に出し、exit code 1 にする。

## 横断ルール

- 終了コードと標準出力 / 標準エラーの役割をコマンド間で一貫させる。
- Parser + 生成 MVP の完了条件は、必須 language adapter の fixture で `mds check`、`mds build --dry-run`、`mds build` が通り、期待ファイル、manifest、header、adapter 固有の追加生成物を検証できることとする。
- Post-MVP コマンドは同じ package 境界、設定解決、stdout / stderr 役割、exit code 体系を共有する。
- 診断 code は `MD001_REQUIRED_SECTION`、`LINT001_TOOLCHAIN_FAILED` のようにカテゴリ prefix と短名で表す。

## 検証観点

- 各コマンドの正常系、入力不備、対象なし、部分失敗を fixture で確認する。
- コマンド間で対象検出結果が矛盾しないことを確認する。
- MVP では `check` / `build --dry-run` / `build` の3経路を必須 language adapter fixture で確認する。
- Post-MVP では `lint`、`lint --fix`、`test`、`doctor`、`package sync` の正常系、入力不備、対象なし、部分失敗、environment 不足を確認する。

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `SPEC-md-state-quality-operations.md`
- `SPEC-doctor-command.md`
- `SPEC-package-sync.md`
- `../../validation.md`
