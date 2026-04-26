---
id: SPEC-doctor-command
status: 採用
related:
  - docs/project/requirements/REQ-cli-command-surface.md
  - docs/project/requirements/REQ-platform-multi-ecosystem-distribution.md
---

# Doctor コマンド

## 概要

`mds doctor` は mds の実行環境、設定、package 検出、adapter、外部 toolchain を診断する。

## 関連要求

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`

## 入力

- CLI option
- `mds.config.toml`
- package metadata
- 実行環境の PATH / runtime / toolchain

## 出力

- 環境診断結果
- adapter 有効性診断
- toolchain version 診断

## 挙動

- mds core / CLI version を表示する。
- 設定ファイルの検出と TOML parse 可否を診断する。
- mds 有効 package と非対象 package を検出する。
- 有効 adapter ごとに必要 runtime / toolchain を検出する。
- 必須 runtime / toolchain は有効 adapter 分だけを対象にする。
- toolchain 検出は package metadata を優先し、必要に応じて PATH 上の代表 command 実行で version を補完する。
- 必須 toolchain 不足は environment 不足として扱う。
- 任意 toolchain 不足は warning として扱う。
- runtime / toolchain が最低対応 version を下回る場合は environment 不足として扱う。
- 既定出力は人間向け text とし、`--format json` で機械処理向け JSON を stdout に出す。

## 状態遷移 / 不変条件

- doctor は読み取り専用であり、設定、Markdown、生成物、package metadata を変更しない。
- doctor は check / build と同じ設定解決と package 境界を使う。
- wrapper 配布環境でも同じ診断概念を使う。

## エラー / 例外

- 必須 toolchain または runtime 不足は exit code 4 にする。
- 最低対応 version 未満の必須 runtime / toolchain は exit code 4 にする。
- 設定 parse error は exit code 2 にする。
- 診断 warning のみの場合、exit code は 0 とする。
- 内部エラーは exit code 3 にする。

## 横断ルール

- stdout は環境サマリ、stderr は warning / error / diagnostic に使う。
- version mismatch は最低対応 version を下回る場合に environment 不足として扱う。
- toolchain 検出は adapter に閉じ込め、core は結果を共通診断へ集約する。

## 検証観点

- 必須 runtime / toolchain の存在と version を確認する fixture を用意する。
- toolchain 不足時の exit code 4 を確認する。
- 最低対応 version 未満の runtime / toolchain が exit code 4 になることを確認する。
- optional toolchain warning が exit code 0 になることを確認する。
- package metadata 優先検出、PATH 補完、`--format json` を確認する。
- wrapper 経由でも同じ診断を返すことを確認する。

## 関連資料

- `../../requirements/REQ-cli-command-surface.md`
- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`
- `SPEC-distribution-and-versions.md`
- `../../tech-stack.md`
