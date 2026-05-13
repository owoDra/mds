---
id: ADR-007-self-hosted-src-md-build
status: 保管
related:
  - docs/plan/index.md
  - docs/plan/phase-00-record-new-direction.md
  - docs/project/proposals/archive/proposal-self-hosted-source-layout.md
  - docs/project/architecture.md
  - docs/project/validation.md
---

# mds 自身の正本を .mds/source / .mds/test に置き生成物を .build に集約する

## 現在位置づけ

この判断は、mds 自身の first-party package を `.mds/source` / `.mds/test` から self-hosting build する前提で採用された。`docs/plan/index.md` と `docs/plan/phase-00-record-new-direction.md` で定義した self-hosting removal / authoring-v2 plan により、その前提は repository の現行方針から外れたため、この ADR は superseded として archive に保管する。

後続 phase では `mds/**/src` と `mds/**/tests` を直接正本として扱い、残存する first-party `.mds` 資産や self-hosted build 導線は historical / cleanup target として扱う。

## 背景

mds は Markdown を設計書兼ソースの正本として扱う。一方で、mds 自身の Rust 実装は従来 `crates/` 配下の手書き Cargo workspace を編集入口としていた。この構成では、mds の主要な不変条件である「`.md` が正本で生成コードは派生物」という方針を mds 自身の開発で検証しにくい。

また、Rust の `target/`、VS Code extension の `out/`、release 用の `.release/` など、ビルド生成物が複数箇所に分散していた。正本と生成物の境界を明確にするため、生成物の配置を統一する必要がある。

## 判断

以下は self-hosting 前提で採用されていた当時の判断であり、現行の repository 方針ではない。

- mds 自身の編集入口を各 package 配下の `.mds/source/` と `.mds/test/` に移す。
- Rust 実装は `mds/core/.mds/source`、`mds/cli/.mds/source`、`mds/lsp/.mds/source` と対応する `.mds/test` を正本とする。
- Rust Cargo workspace は `.build/rust/` に生成する。
- VS Code extension など Node.js 系の生成物は `.build/node/` に置く。
- release artifact と supply-chain metadata は `.build/release/` に置く。
- `crates/` は開発ディレクトリとして廃止する。
- `src-md/project/specs/` は作らず、package 設計と source root 設計は package 配下の `.mds/source/overview.md` に置く。
- 検証事項の正本は引き続き `docs/project/validation.md` に置く。

## 代替案

- `crates/` を維持する: 既存 Cargo tooling との相性はよいが、mds 自身の正本が Markdown に移らず、dogfooding の効果が弱い。
- `src-md/project/specs/` を作る: 既存 specs の移行先として分かりやすいが、mds の `overview.md` / implementation md の責務と重複する。
- 生成コードを Git 管理する: bootstrap は容易になるが、`.build/` を生成物置き場として統一する方針と矛盾し、正本と派生物の境界が曖昧になる。

## 結果

以下は採用当時に想定していた結果であり、Phase 00 以降の repository 現行方針は `docs/plan/index.md` と `docs/plan/phase-00-record-new-direction.md` を参照する。

- 開発者は package 配下の `.mds/source/` と `.mds/test/` を編集し、`cargo run -p mds-cli -- build --verbose` で package 内の生成 `src/` / `tests/` と `.build/rust/` self-hosted workspace mirror をまとめて更新して Cargo commands を実行する。
- `.build/` は Git 管理しない。
- `crates/` を参照する新規資料や開発手順は追加しない。
- 既存の `docs/project/specs` は移行前資料として扱い、新しい実装設計は package 配下の `.mds/source/overview.md` または implementation md に置く。
- self-hosted Rust workspace の mirror 同期は repo 固有 path だけを対象に `mds build` へ統合する。一般利用 package では workspace root に `mds/core`、`mds/cli`、`mds/lsp` が存在しない限り追加処理は発火しない。

## 関連資料

- `../../../plan/index.md`
- `../../../plan/phase-00-record-new-direction.md`
- `../index.md`
- `../../architecture.md`
- `../../validation.md`
- `../../proposals/archive/proposal-self-hosted-source-layout.md`