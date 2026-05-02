---
id: SPEC-cli-commands
status: 採用
related:
  - docs/project/requirements/REQ-cli-command-surface.md
---

# CLI コマンド

## 概要

mds CLI は Markdown 正本からの派生コード生成、検査、同期、初期化を提供する。

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
- dependency snapshot 同期結果

## 挙動

- `mds build` は Markdown 内のコードブロックとメタ情報から派生コードを生成する。
- `mds check` は構造、参照、表、dependency snapshot drift を検証する。
- `mds check` は文書種別、必須セクション、`Expose` / `Uses` / `Covers` schema、コードブロック有無、fixed authoring root、dependency snapshot drift、path/root/pattern 解決、出力先衝突を検査し、実言語 lint/test は実行しない。
- `mds build --dry-run` は書き込みを行わず、生成計画に加えて git 互換寄りの unified diff を表示する。
- `mds build` は `mds check` と同じ dependency snapshot drift 診断を共有し、stale snapshot では成功しない。
- `mds lint`、`mds lint --fix`、`mds test` は Markdown 状態の code block を adapter 経由で処理する。
- `mds doctor` は実行環境、adapter、toolchain を診断する。
- `mds package sync` は package metadata を正として `.mds/source/overview.md` の managed dependency snapshot を更新または検査する。
- `mds init` は project 初期化、AI agent kit 生成、開発環境セットアップを実行する。
- `mds init --ai` は AI agent kit 生成だけを開始する。

## 状態遷移 / 不変条件

- 各コマンドは同じ package 境界と設定解決結果を使う。
- 失敗時に対象外 package や生成物を破壊的に変更しない。
- CLI の終了コードは 0 成功、1 診断あり、2 CLI usage/config error、3 internal error、4 environment / toolchain 不足とする。
- 成功時の要約と生成一覧を stdout、警告・エラー・診断を stderr に出す。
- 複数 package の一部が失敗した場合は、可能な package の処理を続け、最後に失敗 package をまとめて診断し exit code 1 にする。

## エラー / 例外

- 入力不備、対象なし、部分失敗、adapter 失敗を区別して報告する。
- ユーザーが次に取るべき対応を推測できる標準エラーを返す。
- 対象となる mds 有効 package が 0 件の場合は診断として stderr に出し、exit code 1 にする。

## 横断ルール

- 終了コードと標準出力 / 標準エラーの役割をコマンド間で一貫させる。
- `mds package sync` は code generation ではなく managed snapshot 同期を担う唯一の writer とする。
- 診断 code は `MD001_REQUIRED_SECTION`、`LINT001_TOOLCHAIN_FAILED` のようにカテゴリ prefix と短名で表す。
- `mds init` は interactive default を守り、非対話では明示 option がない限り変更しない。

## 検証観点

- 各コマンドの正常系、入力不備、対象なし、部分失敗を fixture で確認する。
- コマンド間で対象検出結果が矛盾しないことを確認する。
- `check` / `build --dry-run` / `build` / `package sync` の主要経路を必須 language adapter fixture で確認する。
- `lint`、`lint --fix`、`test`、`doctor` の正常系、入力不備、対象なし、部分失敗、environment 不足を確認する。
- `mds init` では project 初期化、AI 初期化、外部コマンド実行確認、非対話 option、部分失敗を確認する。

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `SPEC-md-state-quality-operations.md`
- `SPEC-doctor-command.md`
- `SPEC-package-sync.md`
- `SPEC-ai-agent-cli-initialization.md`
- `SPEC-init-development-environment-setup.md`
- `../../validation.md`