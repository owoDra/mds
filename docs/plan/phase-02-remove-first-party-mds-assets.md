# Phase 02: first-party mds 管理資産の削除

## 前提

- Phase 01 により、first-party Rust / TypeScript source が canonical source になっている。
- first-party `.mds/manifest.toml` を読まなくても build / test できる。
- product として mds が user package を build / lint する機能は残す。

## やること

- `mds/core/.mds/` を削除する。
- `mds/cli/.mds/` を削除する。
- `mds/lsp/.mds/` を削除する。
- first-party `.mds/manifest.toml` と generated output manifests を削除する。
- `.build/rust` self-hosted mirror generation の前提を削除する。
- first-party self-hosted synchronization だけを検証する tests を削除する。
- product behavior の検証に必要な fixture は、小さな明示 fixture として残すか作り直す。
- docs / scripts / tests から first-party `.mds` path 参照を削除する。

## 完了条件

- `mds/core/.mds`、`mds/cli/.mds`、`mds/lsp/.mds` が存在しない。
- `.build/rust` が通常 development / validation に不要である。
- Cargo と VS Code extension の検証が first-party mds manifests に依存しない。
- mds product の build / lint / source map などの検証は dedicated fixture で行われる。

## 注意事項

- user package 向けの `.mds/source` / `.mds/test` support を削除する phase ではない。
- self-hosted-only tests と product behavior tests を混同しない。前者は削除候補、後者は fixture 化して残す。
- `.build/` 配下に他用途の release / cache がある場合、`.build/rust` self-hosted mirror だけを対象にする。
- 削除前に `git status` と関連 path の参照検索を行い、同時作業者の変更を巻き込まない。
