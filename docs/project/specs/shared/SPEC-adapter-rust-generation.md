---
id: SPEC-adapter-rust-generation
status: 採用
related:
  - docs/project/requirements/REQ-adapter-required-language-adapters.md
  - docs/project/requirements/REQ-generation-code-output-rules.md
  - docs/project/specs/shared/SPEC-code-generation-output.md
---

# Rust Adapter 生成

## 概要

Rust adapter 境界は Rust の生成 file pattern、`mod` 管理、`use` 生成を担う。

## 関連要求

- `../../requirements/REQ-adapter-required-language-adapters.md`
- `../../requirements/REQ-generation-code-output-rules.md`

## 入力

- `.rs.md` implementation md
- `Types`、`Source`、`Test` のコードブロック
- `Uses` テーブル
- `index.md` / `Exposes`
- 解決済みの `markdown_root`、`source_root`、`types_root`、`test_root`

## 出力

- Rust Source ファイル
- Rust Types ファイル
- Rust Test ファイル
- Rust `use` 文
- Rust の mds 管理 module block

## 挙動

- 既定 pattern は、`src-md/foo/bar.rs.md` から Source `src/foo/bar.rs`、Types `src/foo/bar_types.rs`、Test `tests/foo_bar_test.rs` を生成する。
- Rust の `mod` 宣言は mds が管理し、`index.md` / `Exposes` と生成対象から `mod.rs` または `lib.rs` の mds 管理領域を更新する。
- Rust の `mod` 管理領域は `// mds:begin generated modules` から `// mds:end generated modules` の間とする。
- Rust の `lib.rs` / `mod.rs` に mds 管理 marker がない場合、既存ファイルでは末尾に管理 block を追加し、ファイルがなければ header 付きで新規作成する。
- `Uses` の `Types`、`Source`、`Test` 依存は Rust `use` として生成する。
- `Uses.Expose` の alias は Rust `as` use へ変換し、Rust use に対応しない default / namespace 表現は adapter 診断にする。
- Markdown 状態の quality 操作では rustfmt、clippy、cargo test へ一時 Rust code を渡す。

## 状態遷移 / 不変条件

- Rust 固有の module tree 管理は adapter の責務とし、core の Markdown model を変更しない。
- mds は管理 marker の外側の手書き Rust code を更新しない。

## エラー / 例外

- `.rs.md` 以外の implementation md を Rust adapter の生成対象にしない。
- Rust `use` に変換できない `Uses` は adapter 診断にする。
- `mod` 管理 block の BEGIN / END が不整合な場合は生成エラーにする。
- Rust toolchain が不足する場合は environment 不足診断にする。

## 横断ルール

- shared spec の生成 lifecycle、manifest、header、上書き規則に従う。
- shared spec の `Expose` / `Uses` canonical schema を変更しない。

## 検証観点

- `src-md/foo/bar.rs.md` から `src/foo/bar.rs`、`src/foo/bar_types.rs`、`tests/foo_bar_test.rs` が導出できることを確認する。
- `lib.rs` / `mod.rs` の mds 管理 block が追加または更新されることを fixture で確認する。
- `Uses` から Rust `use` が生成できることを fixture で確認する。
- alias use と rustfmt / clippy / cargo test 接続を fixture で確認する。

## 関連資料

- `SPEC-code-generation-output.md`
- `SPEC-expose-uses-tables.md`
- `SPEC-md-state-quality-operations.md`
- `../../patterns/impl-adapter-boundary.md`
