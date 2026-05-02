---
id: SPEC-package-sync
status: 採用
related:
  - docs/project/requirements/REQ-cli-command-surface.md
  - docs/project/requirements/REQ-monorepo-package-boundary.md
---

# Package Sync

## 概要

`mds package sync` は package metadata を正として扱い、`.mds/source/overview.md` の managed dependency snapshot を更新または検査する。

## 関連要求

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-monorepo-package-boundary.md`

## 入力

- `package.json`
- `pyproject.toml`
- `Cargo.toml`
- language package metadata
- `mds.config.toml`
- package manager post hook

## 出力

- `.mds/source/overview.md` の更新結果
- sync 差分
- check 診断

## 挙動

- `mds package sync` は `.mds/source/overview.md` の managed section として package summary、dependencies、dev dependencies snapshot を更新する。
- package metadata は package 名、version、dependency version の正とする。
- `Rules`、`Architecture`、`Exposes` など手書き補足領域は更新しない。
- `mds package sync --check` は書き込みを行わず、差分があれば診断する。
- package manager hook は任意機能とし、post 実行のみを対象にする。
- post hook の既定 command は `mds package sync` とする。
- post hook は依存変更後に `mds package sync` または利用者が明示した command を呼び出せる。
- hook は `[package_sync] hook_enabled = true` で利用者が明示的に有効化する。
- `hook_command` が未指定の場合、hook command は `mds package sync` とする。
- dependency table の最小列は `Name`、`Version`、`Summary` とする。
- managed section に table 以外の手書き内容が混在する場合は同期せず診断する。

## 状態遷移 / 不変条件

- package metadata を正とし、Markdown には hand-written package metadata を複製しない。
- mds は `.mds/source/overview.md` の hand-written 補足領域を破壊しない。
- managed dependency snapshot section の中に手書き補足を混在させない。
- hook は package manager 実行前には介入しない。
- 非対象 package の Markdown は更新しない。

## エラー / 例外

- package metadata が読めない場合は sync 診断にする。
- `.mds/source/overview.md` が存在しない、または managed section を解決できない場合は sync 診断にする。
- 管理 section に手書き補足が混在する場合は sync 診断にする。
- `--check` で差分がある場合は exit code 1 にする。
- hook 実行環境が不足する場合は exit code 4 にする。
- usage / config error は exit code 2、internal error は exit code 3 にする。

## 横断ルール

- `mds check` は package metadata 自体の parse error と dependency snapshot drift を診断する。
- `mds package sync` は check と同じ package 境界と設定解決を使う。
- package manager 固有差分は adapter または配布 wrapper に閉じ込める。

## 検証観点

- npm / Cargo / uv 代表 metadata が存在する package で `package sync` が managed section を更新できることを確認する。
- `Rules`、`Architecture`、`Exposes` が保持されることを確認する。
- `--check` が書き込みなしで差分診断を返すことを確認する。
- post hook 既定 command が `mds package sync` になることを確認する。
- dependency table の `Name`、`Version`、`Summary` を確認する。
- managed section 内の手書き混在を検出し、手書き補足領域は保持されることを確認する。

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-monorepo-package-boundary.md`
- `SPEC-package-boundary-detection.md`
- `../../validation.md`