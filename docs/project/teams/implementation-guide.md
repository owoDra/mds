# Implementation Team Guide

## 役割

Implementation team は、この repository の first-party Rust / TypeScript 実装を checked-in source と tests で保守し、言語横断契約と adapter 境界を壊さずに product behavior を前進させる責任を持つ。

## 担当範囲

- `mds/core`、`mds/cli`、`mds/lsp` の checked-in Rust source、tests、workspace 設定。
- `editors/vscode` の checked-in TypeScript source、tests、build 設定。
- core の言語横断契約と language adapter 境界の整合。
- product fixture や adapter behavior の検証が必要なときの補助的な確認導線。

## ルール

- first-party implementation の通常変更は checked-in `src/` と `tests/` を直接編集し、`.mds/source`、`.mds/test`、生成 mirror を通常の編集入口にしない。
- Rust、TypeScript、Python の言語差分は adapter 境界に閉じ込め、core の document model、config、diagnostics の意味を言語ごとに変えない。
- public API は crate root や package entrypoint から明示し、内部 module や内部実装を暗黙に公開しない。
- test は production source から分離し、repository の標準 layout に合わせて `tests/` または既存の test convention に置く。
- repository の通常検証は `cargo fmt --all --check`、`cargo check --workspace`、`cargo test --workspace`、`cargo clippy --workspace --all-targets`、`editors/vscode` での `npm run compile` を使う。`mds` command は product fixture や CLI behavior の確認が必要な場合だけ補助的に使う。

## 固有知識

- Rust workspace の first-party crates は checked-in source を canonical source として扱い、module 境界、公開範囲、crate 間依存をコード上で明示する。
- VS Code extension の first-party TypeScript 実装は `editors/vscode` 配下で保守し、entrypoint、build 設定、関連 test を checked-in source として扱う。
- cross-language の output rule や import/use 生成差分は adapter の責務として残るが、この repository の通常開発導線は self-hosted Markdown editing や生成 mirror の同期を前提にしない。

## 関連資料

- `../architecture.md`
- `../tech-stack.md`
- `../patterns/impl-adapter-boundary.md`
- `../validation.md`
