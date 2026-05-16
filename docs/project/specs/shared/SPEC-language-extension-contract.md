---
id: SPEC-language-extension-contract
status: 提案中
related:
  - ../../requirements/v1/REQ-quality-language-and-toolchain-independence.md
  - ../../requirements/v1/REQ-ux-language-aware-embedded-lsp-bridge.md
  - ../../architecture.md
  - ../../tech-stack.md
---

# Language And Capability Schema Contract

## 概要

複数言語を共通 authoring model へ載せるための package config、capability schema、package manager 連携の責務境界を定義する共有仕様。

## 関連要求

- `REQ-quality-language-and-toolchain-independence`
- `REQ-ux-language-aware-embedded-lsp-bridge`

## 入力

- `*.lang.md` file naming
- code fence language label
- package config
- optional capability schema
- package manager metadata / scripts

## 出力

- language identity
- output naming rule
- package metadata reader
- quality command resolution
- diagnostic capture rule

## 挙動

- 言語 identity は impl md の file suffix と code fence language label を基準に決定する。
- output rule、special file rule、root module rule、optional import/render hint は package config または参照される capability schema で定義できる。
- quality integration は `typecheck` `lint` `fix` `test` の slot semantic を中核にし、command 解決は package config、package manager scripts、optional capability schema から行う。
- diagnostic capture rule は package config または capability schema で定義できる。
- 新しい言語追加は、既存 authoring model を壊さず config / schema 追加で拡張できることを目標にする。
- editor integration は language identity を file suffix、fence、config / schema から発見できること。

## 状態遷移 / 不変条件

- 同一 package 内の language resolution は決定的であること。
- 言語追加のために core の共通 authoring ルールを書き換えることを原則要求しない。
- quality command 解決は slot semantic と package policy の責務境界を保つ。

## エラー / 例外

- 不明な language key は validation warning または error とする。
- 必要 config / schema 欠落で package 処理継続不能なら error とする。
- language identity が曖昧な file は editor / build の両方で不安定動作を起こさないよう拒否または診断する。

## 横断ルール

- v1 は複数言語を support するが、全言語即時対応は要求しない。
- v2 へ拡張しても、言語追加は config / schema 中心で進める。

## 検証観点

- 既存 TS / Python / Rust examples が同一 model で処理できる。
- 新 language 追加時に共通 core 変更が最小で済む。
- quality command 解決が言語ごとに一貫する。
- diagnostic capture と remap が source map 前提で成立する。

## 関連資料

- `../../requirements/v1/REQ-quality-language-and-toolchain-independence.md`
- `../../requirements/v1/REQ-ux-language-aware-embedded-lsp-bridge.md`
- `../../architecture.md`
- `../../tech-stack.md`
