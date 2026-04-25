---
id: ADR-003-multi-ecosystem-rust-core
status: 採用
related:
  - docs/project/requirements/REQ-platform-multi-ecosystem-distribution.md
  - docs/project/requirements/REQ-adapter-required-language-adapters.md
---

# Rust core とマルチエコシステム配布を採用する

## 背景

mds は npm 専用 CLI ではなく、Markdown Source toolchain として Node、Rust、Python の利用者に提供する必要がある。

## 判断

Rust core と native CLI を中核にし、npm、Cargo、uv から導入できる配布構成を採用する。TypeScript、Python、Rust の language adapter は初期必須構成とする。

## 代替案

- npm 専用実装: TypeScript 利用者には導入しやすいが、Rust / Python 利用者の一次導線にならない。
- 各言語で完全別実装: 利用者には自然だが、Markdown model と生成規則の一貫性を保ちにくい。

## 結果

言語横断の中核契約は Rust core に集約し、言語差分は adapter と配布パッケージに閉じ込める。

## 関連資料

- `../../requirements/REQ-platform-multi-ecosystem-distribution.md`
- `../../requirements/REQ-adapter-required-language-adapters.md`
- `../../patterns/impl-adapter-boundary.md`
- `../../tech-stack.md`
