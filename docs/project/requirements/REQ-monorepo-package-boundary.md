---
id: REQ-monorepo-package-boundary
status: 採用
related:
  - README.md
  - docs/project/architecture.md
  - docs/project/validation.md
---

# Monorepo Package 境界

## 目標

mds は monorepo で package 単位に有効化でき、mds 対象 package と非対象 package を安全に混在できること。

## 根拠

実プロジェクトでは複数言語や非 mds package が同じ repository に存在するため、repository 全体を一律に mds 対象にできないため。

## 対象範囲

- package 単位で mds の有効 / 無効を判定すること
- `allow_raw_source = true` による直書きソースとの混在を扱うこと
- `enabled = true`、`package.md`、実体の package 定義を mds package 判定に使うこと
- JS/TS、Python、Rust の package 定義を扱うこと

## 対象外

- repository 内のすべての package を強制的に mds 化すること
- package 定義がないディレクトリを暗黙に mds package とみなすこと
- 非対象 package のソースを mds が書き換えること

## 成功指標

- mds 対象 package と非対象 package が混在する fixture で対象範囲が安定する
- `package.json`、`pyproject.toml`、`Cargo.toml` を持つ package を判定できる
- 非対象 package に対して build、lint、lint --fix、test が破壊的に作用しない

## 制約 / 品質条件

- 対象範囲の誤検出を避ける
- 未対応構成は暗黙処理せず、明示的に未対応として扱う

## 関連資料

- `../../README.md`
- `../architecture.md`
- `../validation.md`
