---
id: SPEC-package-sync
status: 採用
related:
  - docs/project/requirements/REQ-cli-command-surface.md
  - docs/project/requirements/REQ-monorepo-package-boundary.md
---

# Package Sync

## 概要

`mds package sync` は package metadata を正として、`package.md` の生成管理部分を同期する。

## 関連要求

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-monorepo-package-boundary.md`

## 入力

- `package.json`
- `pyproject.toml`
- `Cargo.toml`
- `package.md`
- `mds.config.toml`
- package manager post hook

## 出力

- 更新後の `package.md`
- sync 差分
- check 診断

## 挙動

- `mds package sync` は `Package`、`Dependencies`、`Dev Dependencies` セクション全体を生成管理対象として更新する。
- package metadata は package 名、version、dependency version の正とする。
- `Rules` など手書き補足領域は更新しない。
- `mds package sync --check` は書き込みを行わず、差分があれば診断する。
- package manager hook は任意機能とし、post 実行のみを対象にする。
- post hook の既定 command は `mds package sync --check` とする。
- post hook は依存変更後に `mds package sync --check` または利用者が明示した command を呼び出せる。
- hook は `[package_sync] hook_enabled = true` で利用者が明示的に有効化する。
- `hook_command` が未指定の場合、hook command は `mds package sync --check` とする。
- dependency table の最小列は `Name`、`Version`、`Summary` とする。
- `Package`、`Dependencies`、`Dev Dependencies` の管理 section に table 以外の手書き内容が混在する場合は同期せず診断する。

## 状態遷移 / 不変条件

- package metadata を正とし、`package.md` は同期対象の正本表示として扱う。
- mds は `package.md` の手書き補足領域を破壊しない。
- `Package`、`Dependencies`、`Dev Dependencies` の中に手書き補足を混在させない。
- hook は package manager 実行前には介入しない。
- 非対象 package の `package.md` は更新しない。

## エラー / 例外

- package metadata が読めない場合は sync 診断にする。
- `package.md` の必須セクションがない場合は sync 診断にする。
- 管理 section に手書き補足が混在する場合は sync 診断にする。
- `--check` で差分がある場合は exit code 1 にする。
- hook 実行環境が不足する場合は exit code 4 にする。
- usage / config error は exit code 2、internal error は exit code 3 にする。

## 横断ルール

- `mds check` は package metadata と `package.md` の不整合を診断する。
- `mds package sync` は check と同じ package 境界と設定解決を使う。
- package manager 固有差分は adapter または配布 wrapper に閉じ込める。

## 検証観点

- npm / Cargo / uv 代表 metadata から `package.md` の `Package`、`Dependencies`、`Dev Dependencies` セクション全体が更新されることを確認する。
- `Rules` が保持されることを確認する。
- `--check` が書き込みなしで差分診断を返すことを確認する。
- post hook 既定 command が `mds package sync --check` になることを確認する。
- dependency table の `Name`、`Version`、`Summary` を確認する。
- 管理 section 内の手書き混在を検出し、`Rules` などの手書き補足領域は保持されることを確認する。

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-monorepo-package-boundary.md`
- `SPEC-package-boundary-detection.md`
- `../../validation.md`
