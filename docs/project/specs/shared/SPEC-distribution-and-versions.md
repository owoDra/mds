---
id: SPEC-distribution-and-versions
status: 採用
related:
  - docs/project/requirements/REQ-platform-multi-ecosystem-distribution.md
  - docs/project/requirements/REQ-adapter-required-language-adapters.md
---

# 配布と Version 方針

## 概要

mds は Cargo / native binary と VS Code 拡張を中心に配布し、Rust core の言語横断契約を共有する。

## 関連要求

- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`
- `../../requirements/REQ-adapter-required-language-adapters.md`

## 入力

- Cargo crate
- language adapter package
- runtime / toolchain version

## 出力

- native CLI `mds`
- adapter package

## 挙動

- Rust core は言語横断の中核契約を提供する。
- native CLI は Cargo から導入できる。
- Cargo crate 名は現在の workspace では `mds-cli`、`mds-core`、`mds-lsp` とする。
- wrapper は独自仕様を持たず、配布、起動、adapter 接続だけを担う。
- CLI binary name は原則 `mds` とする。
- 最低対応 version は Rust 1.86+、Node.js 24+、Python 3.13+ とする。
- language quality toolchain は利用者選択式とし、Prettier、ESLint、Biome、Vitest、Jest、Ruff、Black、Pytest、unittest、rustfmt、clippy、cargo test、cargo-nextest の最新安定系列を候補にする。
- bootstrap 導線は Cargo / native binary を対象にする。
- 公開前品質では現在の配布経路に checksum、署名、SBOM、provenance、install smoke test を要求する。

## 状態遷移 / 不変条件

- 配布形態が違っても、Markdown model、config、CLI、diagnostic、adapter 境界は同じ意味を持つ。
- wrapper は core の意味体系を変更しない。
- ecosystem 固有差分は adapter または wrapper に閉じ込める。
- release quality gate は配布形態ごとの artifact と wrapper 互換性を検証する。

## エラー / 例外

- runtime が最低対応 version を下回る場合は environment 不足として exit code 4 にする。
- adapter package が見つからない場合は adapter 診断にする。
- wrapper が同梱 binary を起動できない、または core と互換でない場合は environment 不足として扱う。
- 公開前品質の supply-chain 成果物が欠ける場合は release gate を失敗させる。

## 横断ルール

- `mds doctor` は runtime と toolchain version を診断する。
- `tech-stack.md` は最低対応 version の正本として更新する。
- package name はこの spec を正とし、変更する場合は spec または ADR を更新する。
- publish は明示承認された release flow でのみ実行する。

## 検証観点

- Cargo / native binary 経由で `mds --version` 相当が動くことを確認する。
- 最低対応 version 未満が doctor で exit code 4 になることを確認する。
- TypeScript / Python / Rust adapter が同じ core 概念を扱うことを確認する。
- 全配布経路で公開前品質 gate が成功することを確認する。

## 関連資料

- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`
- `../../requirements/REQ-adapter-required-language-adapters.md`
- `../../architecture.md`
- `../../tech-stack.md`
- `SPEC-release-prepublish-quality.md`
