---
id: SPEC-init-development-environment-setup
status: 採用
related:
  - docs/project/requirements/REQ-init-development-environment-setup.md
---

# 開発環境セットアップ初期化

## 概要

`mds init` は project 初期化、AI 初期化、開発環境セットアップを統合し、必要に応じて project dependencies、toolchains、global AI CLI を導入する。

## 関連要求

- `../../requirements/REQ-init-development-environment-setup.md`

## 入力

- `mds init`
- `mds init --ai`
- bootstrap entrypoint from `npx`
- bootstrap entrypoint from Cargo
- bootstrap entrypoint from `uvx`
- interactive answers for label language, detected toolchain commands, and AI kit generation items
- explicit noninteractive options
- `--ts-tools <list|default|none>`、`--py-tools <list|default|none>`、`--rs-tools <list|default|none>`

## 出力

- project skeleton
- `mds.config.toml`
- 選択された language quality tool 設定
- AI agent kit
- setup plan
- install results
- diagnostics

## 挙動

- `mds init` は project 初期化と開発環境セットアップを統合する。
- `mds setup` は独立コマンドとして要求しない。
- bootstrap 導線は `npx`、Cargo、`uvx` の 3 経路を正式対応にする。
- 既定動作は interactive default とし、外部コマンド実行前に確認する。
- 自動導入対象は project dependencies、toolchains、global AI CLI とする。
- interactive `mds init` は Label 言語、toolchain command、AI kit 生成項目の順に確認する。
- interactive toolchain command 入力は `package.json`、`pyproject.toml`、`Cargo.toml` を自動検知し、検知された package manager metadata ごとに type check、lint check、test check command の候補を提示して入力を受ける。
- 非対話 init の language quality tool は言語別に選択可能とし、TypeScript は `eslint`、`prettier`、`biome`、`vitest`、`jest`、Python は `ruff`、`black`、`pytest`、`unittest`、Rust は `rustfmt`、`clippy`、`cargo-test`、`nextest` を選択候補にする。
- `default` は代表 toolchain を選択し、`none` はその言語の lint / fix / test runner と required tool を生成 config 上で無効化する。
- `mds init` が生成する `mds.config.toml` は `[quality.ts]`、`[quality.py]`、`[quality.rs]` に選択結果または入力 command を明示し、未選択 tool を暗黙必須にしない。
- 非対話実行では `--yes`、`--ai`、`--install-toolchains`、`--install-ai-cli` などの明示 option がない限り変更しない。
- global toolchain / AI CLI の導入に失敗した場合、成功済み項目は保持し、失敗項目を診断する。
- 選択済み toolchain / AI CLI が不足する場合は、次に実行すべき install hint を診断または setup plan に含める。
- 再実行時は既存 mds project を検出し、生成計画と変更差分を提示する。

## 状態遷移 / 不変条件

- 外部影響が大きい操作はユーザー確認または明示 option を必要とする。
- `mds doctor` と同じ runtime / toolchain 判定を setup plan に使う。
- quality tool の必要 / 不要は生成 config と setup plan / doctor 診断で同じ意味を持つ。
- package manager、toolchain manager、AI CLI 固有差分は init/setup adapter または template 境界に閉じ込める。
- 部分成功と失敗を混ぜて曖昧な成功扱いにしない。

## エラー / 例外

- CLI usage / config error は exit code 2 にする。
- runtime / toolchain / installer 不足は environment 不足として exit code 4 にする。
- install command の失敗は診断として扱い、成功項目と失敗項目を出力する。
- 非対話実行で確認が必要な変更がある場合は変更せず exit code 2 にする。

## 横断ルール

- `mds init` 後に `mds doctor`、`mds check`、必要な toolchain 検証へ進める導線を出力する。
- `mds init` は package 境界、config、AI agent kit、release quality の各 spec と矛盾しない。
- bootstrap wrapper は core の意味体系を変更しない。

## 検証観点

- `npx`、Cargo、`uvx` の bootstrap smoke test を確認する。
- 対話実行、非対話実行、明示 option、部分失敗、再実行の fixture を確認する。
- project dependencies、toolchains、global AI CLI の導入 plan と診断を確認する。
- TypeScript / Python / Rust の quality tool 選択、interactive command 入力、代替 runner、`none` 指定を fixture で確認する。
- setup 後の `mds doctor` と `mds check` への導線を確認する。

## 関連資料

- `../../requirements/REQ-init-development-environment-setup.md`
- `SPEC-ai-agent-cli-initialization.md`
- `SPEC-doctor-command.md`
- `SPEC-cli-commands.md`
- `../../validation.md`
