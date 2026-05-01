# Rust Implementation Team Guide

## 役割

Rust implementation team は、`src-md/mds-core`、`src-md/mds-cli`、`src-md/mds-lsp` の構造、module 境界、公開 API、検証導線を保つ責任を持つ。

## 担当範囲

- `src-md/mds-core`: 言語横断の document model、config、package boundary、Markdown parsing、generation planning、manifest、diagnostics。
- `src-md/mds-cli`: native CLI の argument parsing、stdout / stderr、exit code、core 呼び出し。
- `src-md/mds-lsp`: Language Server Protocol 実装。
- Rust 固有の use / module block / file pattern 生成は、現在は `src-md/mds-core` の adapter / generation 境界で扱う。
- Rust workspace は `.build/rust/` に生成し、`src-md/Cargo.toml` と `src-md/Cargo.lock` を同期元とする。

## ルール

- `src/lib.rs.md` は crate root として module 宣言、公開 API、再 export に寄せ、実処理を長く置かない。
- 1 module は 1 つの責務を持つ。複数の仕様領域をまたぐ場合は、上位の orchestration module から小さい module を呼び出す。
- mds の Rust crate では、module ごとの追加ファイルを見越して原則 `src/<name>/mod.rs` へ置く。単発の極小 module だけ `src/<name>.rs` を許容する。
- crate 外へ必要な型だけ `pub` にする。crate 内共有は `pub(crate)` を優先し、外部公開面を増やすときは spec / README / API 利用者への影響を確認する。
- tests は挙動単位で `tests/` に置く。production module 内の `#[cfg(test)]` は private helper の最小単体確認に限定し、fixture helper は production module に混ぜない。
- Cargo workspace では `.build/rust/Cargo.lock` と `.build/rust/target/` を共有する。crate 間依存は暗黙にせず、各 package の `Cargo.toml` に path dependency を明示する。
- `scripts/sync-build.sh`、`.build/rust` での `cargo fmt --check` と `cargo test` を Rust 実装変更の最小検証にする。

## 固有知識

- Rust compiler は crate root の `src/lib.rs` または `src/main.rs` から module tree を構築する。
- `mod foo;` は `src/foo.rs` または `src/foo/mod.rs` を読み込む。submodule は親 module の directory 配下に置ける。
- module 内の item は親から private が既定であり、公開範囲は `pub` / `pub(crate)` で明示する。
- Cargo の標準 layout は、library を `src/lib.rs`、binary を `src/main.rs`、integration test を `tests/` に置く。
- workspace は関連 crate が同じ `Cargo.lock` と `target/` を共有するための単位であり、crate を分けても依存関係は明示が必要である。
- mds では core の言語横断契約と adapter 固有処理を混ぜない。TypeScript / Python / Rust の file pattern や import / use 生成は adapter 境界で扱う。

## 関連資料

- `../architecture.md`
- `../tech-stack.md`
- `../patterns/impl-adapter-boundary.md`
- `../specs/shared/SPEC-parser-generation-mvp-phase.md`
- `../validation.md`
- Rust Book: Control Scope and Privacy with Modules
- Cargo Book: Package Layout
- Rust Book: Cargo Workspaces
