---
id: SPEC-ai-agent-cli-initialization
status: 採用
related:
  - docs/project/requirements/REQ-ai-agent-cli-initialization.md
---

# AI Agent CLI 初期化

## 概要

`mds init` は Claude Code、Codex CLI、Opencode、GitHub Copilot CLI 向けに、mds project の正本構造、作業規約、検証導線を伝える agent kit を生成する。

## 関連要求

- `../../requirements/REQ-ai-agent-cli-initialization.md`

## 入力

- `mds init`
- `mds init --ai`
- 対象 AI CLI
- 生成カテゴリ selection
- mds project metadata
- 同梱 template plugin

## 出力

- AI CLI ごとの instruction files (各 CLI のネイティブ参照パスに配置)
- AI CLI ごとの skills / commands
- 生成計画と diff
- 上書き確認結果
- 統合ガイド (stdout 表示)

## 挙動

- `mds init` は初期化フロー内で AI 初期化を選択できる。
- `mds init --ai` は AI 初期化だけを開始する。
- 初期対応対象は Claude Code、Codex CLI、Opencode、GitHub Copilot CLI とする。
- 標準 template plugin は mds 本体に同梱し、mds 本体 version と同期する。
- plugin は template、置換変数、対応 CLI metadata だけを持ち、任意コマンドを実行しない。
- agent kit は対象 CLI が対応する範囲で instructions、skills、commands を生成できる。
- ユーザーは instructions、skills、commands のカテゴリ単位で生成項目を選択できる。
- 各 AI CLI のメインファイル (CLAUDE.md, AGENTS.md, copilot-instructions.md) は生成しない。
- 各 AI CLI のネイティブ参照パスにファイルを生成する (例: .claude/rules/, .github/instructions/)。
- 生成後、メインファイルへの統合方法を stdout にガイド表示する。
- 既存ファイルを変更する場合は diff を提示し、確認後だけ非管理ファイルを上書きできる。

## 状態遷移 / 不変条件

- AI CLI 固有差分は include_str! テンプレートファイルに閉じ込める。
- 生成物は mds の正本構造、requirements、specs、validation への参照導線を保つ。
- AI CLI 向け生成物は core の Markdown model を変更しない。
- 生成物は YAML frontmatter に `mds-managed: true` を含み、再実行時に更新可能と判定する。
- 非管理領域の変更は確認後のみ許可する。
- 各 CLI のメインファイル (ユーザー所有) は一切変更しない。

## エラー / 例外

- 未対応 AI CLI が指定された場合は usage / config error として exit code 2 にする。
- template plugin が mds 本体 version と互換でない場合は診断にする。
- 非対話実行で必要な選択が不足する場合は変更せず exit code 2 にする。
- 上書き確認が得られない場合は対象ファイルを変更しない。

## 横断ルール

- `mds init` の stdout / stderr / exit code は `SPEC-cli-commands.md` と整合させる。
- AI CLI が読める形式と mds が管理する canonical な意味を分離する。
- template plugin の追加や変更は validation と release quality の検証対象にする。

## 検証観点

- 4 種の AI CLI それぞれで full agent kit 生成 fixture を確認する。
- カテゴリ単位 selection が期待ファイルだけを生成することを確認する。
- 非管理ファイルの上書きは diff と確認がない限り実行されないことを確認する。
- `mds init --ai` が project 初期化なしで AI 初期化だけを実行できることを確認する。

## 関連資料

- `../../requirements/REQ-ai-agent-cli-initialization.md`
- `SPEC-cli-commands.md`
- `../../validation.md`
