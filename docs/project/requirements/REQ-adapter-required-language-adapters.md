---
id: REQ-adapter-required-language-adapters
status: 採用
related:
  - README.md
  - docs/project/architecture.md
---

# 必須 Language Adapter

## 目標

TypeScript、Python、Rust の language adapter を初期構成から必須要素として扱うこと。

## 根拠

mds の価値は Markdown 正本から複数言語のコード、型、テストを一貫して扱えることにあるため。

## 対象範囲

- `mds core`、`mds cli`、language adapter を必須概念として扱うこと
- npm 側で `@mds/core`、`@mds/cli`、`@mds/lang-ts`、`@mds/lang-py`、`@mds/lang-rs` を扱うこと
- Cargo / uv 側で Rust / Python 向け adapter 配布を扱うこと
- adapter が import 生成、lint、lint --fix、test runner 接続、ファイル名規約、出力規則を担うこと

## 対象外

- TypeScript、Python、Rust 以外の adapter を初期必須範囲に含めること
- core に言語固有の lint、lint --fix、test runner 実装を持たせること

## 成功指標

- 各 adapter の責務が仕様と実装で分離されている
- `Types` / `Source` / `Test` の出力規則を各 adapter が提供できる
- md 内コードブロックに対する仮想 lint / lint --fix を各 adapter が接続できる

## 制約 / 品質条件

- 言語差分は adapter に閉じ込める
- core の概念名と adapter の解釈が矛盾しない

## 関連資料

- `../../README.md`
- `../architecture.md`
